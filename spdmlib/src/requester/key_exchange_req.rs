// Copyright (c) 2020 Intel Corporation
//
// SPDX-License-Identifier: BSD-2-Clause-Patent

extern crate alloc;
use alloc::boxed::Box;

use crate::error::SpdmResult;
use crate::requester::*;

use crate::common::ManagedBuffer;

use crate::crypto;

const INITIAL_SESSION_ID: u16 = 0xFFFE;

impl<'a> RequesterContext<'a> {
    pub fn send_receive_spdm_key_exchange(
        &mut self,
        slot_id: u8,
        measurement_summary_hash_type: SpdmMeasurementSummaryHashType,
    ) -> SpdmResult<u32> {
        info!("send spdm key exchange\n");

        let mut send_buffer = [0u8; config::MAX_SPDM_TRANSPORT_SIZE];
        let (key_exchange_context, send_used) = self.encode_spdm_key_exchange(
            &mut send_buffer,
            slot_id,
            measurement_summary_hash_type,
        )?;
        self.send_message(&send_buffer[..send_used])?;

        // Receive
        let mut receive_buffer = [0u8; config::MAX_SPDM_TRANSPORT_SIZE];
        let receive_used = self.receive_message(&mut receive_buffer)?;
        self.handle_spdm_key_exhcange_response(
            &send_buffer[..send_used],
            &receive_buffer[..receive_used],
            measurement_summary_hash_type,
            key_exchange_context,
        )
    }

    pub fn encode_spdm_key_exchange(
        &mut self,
        buf: &mut [u8],
        slot_id: u8,
        measurement_summary_hash_type: SpdmMeasurementSummaryHashType,
    ) -> SpdmResult<(Box<dyn crypto::SpdmDheKeyExchange>, usize)> {
        let mut writer = Writer::init(buf);

        let req_session_id = INITIAL_SESSION_ID;

        let mut random = [0u8; SPDM_RANDOM_SIZE];
        crypto::rand::get_random(&mut random)?;

        let (exchange, key_exchange_context) =
            crypto::dhe::generate_key_pair(self.common.negotiate_info.dhe_sel)
                .ok_or(spdm_err!(EFAULT))?;

        debug!("!!! exchange data : {:02x?}\n", exchange);
        let mut opaque = SpdmOpaqueStruct {
            data_size: crate::common::OPAQUE_DATA_SUPPORT_VERSION.len() as u16,
            ..Default::default()
        };
        opaque.data[..(opaque.data_size as usize)]
            .copy_from_slice(crate::common::OPAQUE_DATA_SUPPORT_VERSION.as_ref());
        let request = SpdmMessage {
            header: SpdmMessageHeader {
                version: SpdmVersion::SpdmVersion11,
                request_response_code: SpdmResponseResponseCode::SpdmRequestKeyExchange,
            },
            payload: SpdmMessagePayload::SpdmKeyExchangeRequest(SpdmKeyExchangeRequestPayload {
                slot_id,
                measurement_summary_hash_type,
                req_session_id,
                random: SpdmRandomStruct { data: random },
                exchange,
                opaque,
            }),
        };
        request.spdm_encode(&mut self.common, &mut writer);
        Ok((key_exchange_context, writer.used()))
    }

    pub fn handle_spdm_key_exhcange_response(
        &mut self,
        send_buffer: &[u8],
        receive_buffer: &[u8],
        measurement_summary_hash_type: SpdmMeasurementSummaryHashType,
        key_exchange_context: Box<dyn crypto::SpdmDheKeyExchange>,
    ) -> SpdmResult<u32> {
        if (measurement_summary_hash_type
            == SpdmMeasurementSummaryHashType::SpdmMeasurementSummaryHashTypeTcb)
            || (measurement_summary_hash_type
                == SpdmMeasurementSummaryHashType::SpdmMeasurementSummaryHashTypeAll)
        {
            self.common.runtime_info.need_measurement_summary_hash = true;
        } else {
            self.common.runtime_info.need_measurement_summary_hash = false;
        }

        let mut reader = Reader::init(receive_buffer);
        match SpdmMessageHeader::read(&mut reader) {
            Some(message_header) => match message_header.request_response_code {
                SpdmResponseResponseCode::SpdmResponseKeyExchangeRsp => {
                    let key_exchange_rsp =
                        SpdmKeyExchangeResponsePayload::spdm_read(&mut self.common, &mut reader);
                    let receive_used = reader.used();
                    if let Some(key_exchange_rsp) = key_exchange_rsp {
                        debug!("!!! key_exchange rsp : {:02x?}\n", key_exchange_rsp);
                        debug!(
                            "!!! exchange data (peer) : {:02x?}\n",
                            &key_exchange_rsp.exchange
                        );

                        let final_key = key_exchange_context
                            .compute_final_key(&key_exchange_rsp.exchange)
                            .ok_or(spdm_err!(EFAULT))?;

                        debug!("!!! final_key : {:02x?}\n", final_key.as_ref());

                        // verify signature
                        let base_asym_size =
                            self.common.negotiate_info.base_asym_sel.get_size() as usize;
                        let base_hash_size =
                            self.common.negotiate_info.base_hash_sel.get_size() as usize;

                        let mut message_k = ManagedBuffer::default();
                        message_k
                            .append_message(send_buffer)
                            .ok_or(spdm_err!(ENOMEM))?;
                        let temp_receive_used = receive_used - base_asym_size - base_hash_size;
                        message_k
                            .append_message(&receive_buffer[..temp_receive_used])
                            .ok_or(spdm_err!(ENOMEM))?;

                        if self
                            .common
                            .verify_key_exchange_rsp_signature(
                                &message_k,
                                &key_exchange_rsp.signature,
                            )
                            .is_err()
                        {
                            error!("verify_key_exchange_rsp_signature fail");
                            return spdm_result_err!(EFAULT);
                        } else {
                            info!("verify_key_exchange_rsp_signature pass");
                        }
                        message_k
                            .append_message(key_exchange_rsp.signature.as_ref())
                            .ok_or(spdm_err!(ENOMEM))?;

                        // create session - generate the handshake secret (including finished_key)
                        let th1 = self
                            .common
                            .calc_req_transcript_hash(false, &message_k, None)?;
                        debug!("!!! th1 : {:02x?}\n", th1.as_ref());
                        let base_hash_algo = self.common.negotiate_info.base_hash_sel;
                        let dhe_algo = self.common.negotiate_info.dhe_sel;
                        let aead_algo = self.common.negotiate_info.aead_sel;
                        let key_schedule_algo = self.common.negotiate_info.key_schedule_sel;
                        let sequence_number_count =
                            self.common.transport_encap.get_sequence_number_count();
                        let max_random_count = self.common.transport_encap.get_max_random_count();

                        let session_id = ((INITIAL_SESSION_ID as u32) << 16)
                            + key_exchange_rsp.rsp_session_id as u32;
                        let session = self
                            .common
                            .get_next_avaiable_session()
                            .ok_or(spdm_err!(EINVAL))?;

                        session.setup(session_id).unwrap();
                        session.set_use_psk(false);

                        session.set_crypto_param(
                            base_hash_algo,
                            dhe_algo,
                            aead_algo,
                            key_schedule_algo,
                        );
                        session.set_transport_param(sequence_number_count, max_random_count);
                        session.set_dhe_secret(&final_key);
                        session.generate_handshake_secret(&th1).unwrap();

                        // verify HMAC with finished_key
                        let transcript_data = self
                            .common
                            .calc_req_transcript_data(false, &message_k, None)?;
                        let session = self
                            .common
                            .get_session_via_id(session_id)
                            .ok_or(spdm_err!(EINVAL))?;
                        if session
                            .verify_hmac_with_response_finished_key(
                                transcript_data.as_ref(),
                                &key_exchange_rsp.verify_data,
                            )
                            .is_err()
                        {
                            error!("verify_hmac_with_response_finished_key fail");
                            let _ = session.teardown(session_id);
                            return spdm_result_err!(EFAULT);
                        } else {
                            info!("verify_hmac_with_response_finished_key pass");
                        }
                        message_k
                            .append_message(key_exchange_rsp.verify_data.as_ref())
                            .ok_or(spdm_err!(ENOMEM))?;
                        session.runtime_info.message_k = message_k;

                        session.set_session_state(
                            crate::session::SpdmSessionState::SpdmSessionHandshaking,
                        );

                        Ok(session_id)
                    } else {
                        error!("!!! key_exchange : fail !!!\n");
                        spdm_result_err!(EFAULT)
                    }
                }
                _ => spdm_result_err!(EINVAL),
            },
            None => spdm_result_err!(EIO),
        }
    }
}

#[cfg(test)]
mod tests_requester {
    use super::*;
    use crate::testlib::*;
    use crate::{crypto, responder};

    #[test]
    fn test_case0_send_receive_spdm_key_exchange() {
        let (rsp_config_info, rsp_provision_info) = create_info();
        let (req_config_info, req_provision_info) = create_info();

        let shared_buffer = SharedBuffer::new();
        let mut device_io_responder = FakeSpdmDeviceIoReceve::new(&shared_buffer);
        let pcidoe_transport_encap = &mut PciDoeTransportEncap {};

        crypto::asym_sign::register(ASYM_SIGN_IMPL);

        let message_m = &[
            0x11, 0xe0, 0x00, 0x00, 0x11, 0x60, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let mut responder = responder::ResponderContext::new(
            &mut device_io_responder,
            pcidoe_transport_encap,
            rsp_config_info,
            rsp_provision_info,
        );

        responder.common.provision_info.my_cert_chain = Some(REQ_CERT_CHAIN_DATA);

        responder.common.negotiate_info.base_hash_sel = SpdmBaseHashAlgo::TPM_ALG_SHA_384;
        responder.common.negotiate_info.aead_sel = SpdmAeadAlgo::AES_128_GCM;
        responder.common.negotiate_info.dhe_sel = SpdmDheAlgo::SECP_384_R1;
        responder.common.negotiate_info.base_asym_sel =
            SpdmBaseAsymAlgo::TPM_ALG_ECDSA_ECC_NIST_P384;

        responder.common.reset_runtime_info();

        responder
            .common
            .runtime_info
            .message_m
            .append_message(message_m);
        responder.common.peer_info.peer_cert_chain.cert_chain = REQ_CERT_CHAIN_DATA;

        let pcidoe_transport_encap2 = &mut PciDoeTransportEncap {};
        let mut device_io_requester = FakeSpdmDeviceIo::new(&shared_buffer, &mut responder);

        let mut requester = RequesterContext::new(
            &mut device_io_requester,
            pcidoe_transport_encap2,
            req_config_info,
            req_provision_info,
        );

        requester.common.negotiate_info.base_hash_sel = SpdmBaseHashAlgo::TPM_ALG_SHA_384;
        requester.common.negotiate_info.aead_sel = SpdmAeadAlgo::AES_128_GCM;
        requester.common.negotiate_info.dhe_sel = SpdmDheAlgo::SECP_384_R1;
        requester.common.negotiate_info.base_asym_sel =
            SpdmBaseAsymAlgo::TPM_ALG_ECDSA_ECC_NIST_P384;

        requester.common.reset_runtime_info();

        requester
            .common
            .runtime_info
            .message_m
            .append_message(message_m);
        requester.common.peer_info.peer_cert_chain.cert_chain = REQ_CERT_CHAIN_DATA;

        let measurement_summary_hash_type =
            SpdmMeasurementSummaryHashType::SpdmMeasurementSummaryHashTypeAll;
        let status = requester
            .send_receive_spdm_key_exchange(0, measurement_summary_hash_type)
            .is_ok();
        assert!(status);
    }
}
