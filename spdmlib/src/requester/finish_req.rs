// Copyright (c) 2020 Intel Corporation
//
// SPDX-License-Identifier: BSD-2-Clause-Patent

use crate::error::SpdmResult;
use crate::requester::*;

use crate::common::ManagedBuffer;

impl<'a> RequesterContext<'a> {
    pub fn send_receive_spdm_finish(&mut self, session_id: u32) -> SpdmResult {
        info!("send spdm finish\n");
        let mut send_buffer = [0u8; config::MAX_SPDM_TRANSPORT_SIZE];
        let (send_used, base_hash_size, message_f) =
            self.encode_spdm_finish(session_id, &mut send_buffer)?;
        self.send_secured_message(session_id, &send_buffer[..send_used], false)?;

        let mut receive_buffer = [0u8; config::MAX_SPDM_TRANSPORT_SIZE];
        let receive_used = self.receive_secured_message(session_id, &mut receive_buffer)?;
        self.handle_spdm_finish_response(
            session_id,
            base_hash_size,
            message_f,
            &receive_buffer[..receive_used],
        )
    }

    pub fn encode_spdm_finish(
        &mut self,
        session_id: u32,
        buf: &mut [u8],
    ) -> SpdmResult<(usize, usize, ManagedBuffer)> {
        let mut writer = Writer::init(buf);

        let request = SpdmMessage {
            header: SpdmMessageHeader {
                version: SpdmVersion::SpdmVersion11,
                request_response_code: SpdmResponseResponseCode::SpdmRequestFinish,
            },
            payload: SpdmMessagePayload::SpdmFinishRequest(SpdmFinishRequestPayload {
                finish_request_attributes: SpdmFinishRequestAttributes::empty(),
                req_slot_id: 0,
                signature: SpdmSignatureStruct::default(),
                verify_data: SpdmDigestStruct {
                    data_size: self.common.negotiate_info.base_hash_sel.get_size(),
                    data: [0xcc; SPDM_MAX_HASH_SIZE],
                },
            }),
        };
        request.spdm_encode(&mut self.common, &mut writer);
        let send_used = writer.used();

        // generate HMAC with finished_key
        let base_hash_size = self.common.negotiate_info.base_hash_sel.get_size() as usize;
        let temp_used = send_used - base_hash_size;

        let mut message_f = ManagedBuffer::default();
        message_f
            .append_message(&buf[..temp_used])
            .ok_or(spdm_err!(ENOMEM))?;

        let session = self
            .common
            .get_immutable_session_via_id(session_id)
            .unwrap();
        let message_k = &session.runtime_info.message_k;

        let transcript_data =
            self.common
                .calc_req_transcript_data(false, message_k, Some(&message_f))?;
        let session = self.common.get_session_via_id(session_id).unwrap();
        let hmac = session.generate_hmac_with_request_finished_key(transcript_data.as_ref())?;
        message_f
            .append_message(hmac.as_ref())
            .ok_or(spdm_err!(ENOMEM))?;

        // patch the message before send
        buf[(send_used - base_hash_size)..send_used].copy_from_slice(hmac.as_ref());
        Ok((send_used, base_hash_size, message_f))
    }

    pub fn handle_spdm_finish_response(
        &mut self,
        session_id: u32,
        base_hash_size: usize,
        mut message_f: ManagedBuffer,
        receive_buffer: &[u8],
    ) -> SpdmResult {
        let in_clear_text = self
            .common
            .negotiate_info
            .req_capabilities_sel
            .contains(SpdmRequestCapabilityFlags::HANDSHAKE_IN_THE_CLEAR_CAP)
            && self
                .common
                .negotiate_info
                .rsp_capabilities_sel
                .contains(SpdmResponseCapabilityFlags::HANDSHAKE_IN_THE_CLEAR_CAP);

        let mut reader = Reader::init(receive_buffer);
        match SpdmMessageHeader::read(&mut reader) {
            Some(message_header) => match message_header.request_response_code {
                SpdmResponseResponseCode::SpdmResponseFinishRsp => {
                    let finish_rsp =
                        SpdmFinishResponsePayload::spdm_read(&mut self.common, &mut reader);
                    let receive_used = reader.used();
                    if let Some(finish_rsp) = finish_rsp {
                        debug!("!!! finish rsp : {:02x?}\n", finish_rsp);

                        if in_clear_text {
                            let session = self
                                .common
                                .get_immutable_session_via_id(session_id)
                                .unwrap();
                            let message_k = &session.runtime_info.message_k;

                            // verify HMAC with finished_key
                            let temp_used = receive_used - base_hash_size;
                            message_f
                                .append_message(&receive_buffer[..temp_used])
                                .ok_or(spdm_err!(ENOMEM))?;

                            let transcript_data = self.common.calc_req_transcript_data(
                                false,
                                message_k,
                                Some(&message_f),
                            )?;

                            let session = self.common.get_session_via_id(session_id).unwrap();
                            if session
                                .verify_hmac_with_response_finished_key(
                                    transcript_data.as_ref(),
                                    &finish_rsp.verify_data,
                                )
                                .is_err()
                            {
                                error!("verify_hmac_with_response_finished_key fail");
                                let _ = session.teardown(session_id);
                                return spdm_result_err!(EFAULT);
                            } else {
                                info!("verify_hmac_with_response_finished_key pass");
                            }
                            message_f
                                .append_message(finish_rsp.verify_data.as_ref())
                                .ok_or(spdm_err!(ENOMEM))?;
                            session.runtime_info.message_f = message_f;
                        } else {
                            let session = self.common.get_session_via_id(session_id).unwrap();
                            message_f
                                .append_message(&receive_buffer[..receive_used])
                                .ok_or(spdm_err!(ENOMEM))?;
                            session.runtime_info.message_f = message_f;
                        }

                        let session = self
                            .common
                            .get_immutable_session_via_id(session_id)
                            .unwrap();
                        let message_k = &session.runtime_info.message_k;
                        // generate the data secret
                        let th2 = self.common.calc_req_transcript_hash(
                            false,
                            message_k,
                            Some(&message_f),
                        )?;
                        debug!("!!! th2 : {:02x?}\n", th2.as_ref());
                        let session = self.common.get_session_via_id(session_id).unwrap();
                        session.generate_data_secret(&th2).unwrap();
                        session.set_session_state(
                            crate::session::SpdmSessionState::SpdmSessionEstablished,
                        );

                        Ok(())
                    } else {
                        error!("!!! finish : fail !!!\n");
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
    use crate::session::SpdmSession;
    use crate::testlib::*;
    use crate::{crypto, responder};

    #[test]
    fn test_case0_send_receive_spdm_finish() {
        let (rsp_config_info, rsp_provision_info) = create_info();
        let (req_config_info, req_provision_info) = create_info();

        let shared_buffer = SharedBuffer::new();
        let mut device_io_responder = FakeSpdmDeviceIoReceve::new(&shared_buffer);

        let pcidoe_transport_encap = &mut PciDoeTransportEncap {};

        crypto::asym_sign::register(ASYM_SIGN_IMPL);

        let mut responder = responder::ResponderContext::new(
            &mut device_io_responder,
            pcidoe_transport_encap,
            rsp_config_info,
            rsp_provision_info,
        );

        responder.common.negotiate_info.req_ct_exponent_sel = 0;
        responder.common.negotiate_info.req_capabilities_sel =
            SpdmRequestCapabilityFlags::HANDSHAKE_IN_THE_CLEAR_CAP;

        responder.common.negotiate_info.rsp_ct_exponent_sel = 0;
        responder.common.negotiate_info.rsp_capabilities_sel =
            SpdmResponseCapabilityFlags::HANDSHAKE_IN_THE_CLEAR_CAP;

        responder.common.negotiate_info.base_asym_sel =
            SpdmBaseAsymAlgo::TPM_ALG_ECDSA_ECC_NIST_P384;
        responder.common.negotiate_info.base_hash_sel = SpdmBaseHashAlgo::TPM_ALG_SHA_384;

        responder.common.provision_info.my_cert_chain = Some(REQ_CERT_CHAIN_DATA);

        responder.common.reset_runtime_info();

        responder.common.session = [SpdmSession::new(); 4];
        responder.common.session[0].setup(4294901758).unwrap();
        responder.common.session[0].set_crypto_param(
            SpdmBaseHashAlgo::TPM_ALG_SHA_384,
            SpdmDheAlgo::SECP_384_R1,
            SpdmAeadAlgo::AES_256_GCM,
            SpdmKeyScheduleAlgo::SPDM_KEY_SCHEDULE,
        );
        responder.common.session[0]
            .set_session_state(crate::session::SpdmSessionState::SpdmSessionHandshaking);

        let pcidoe_transport_encap2 = &mut PciDoeTransportEncap {};
        let mut device_io_requester = FakeSpdmDeviceIo::new(&shared_buffer, &mut responder);

        let mut requester = RequesterContext::new(
            &mut device_io_requester,
            pcidoe_transport_encap2,
            req_config_info,
            req_provision_info,
        );

        requester.common.negotiate_info.req_ct_exponent_sel = 0;
        requester.common.negotiate_info.req_capabilities_sel =
            SpdmRequestCapabilityFlags::HANDSHAKE_IN_THE_CLEAR_CAP;

        requester.common.negotiate_info.rsp_ct_exponent_sel = 0;
        requester.common.negotiate_info.rsp_capabilities_sel =
            SpdmResponseCapabilityFlags::HANDSHAKE_IN_THE_CLEAR_CAP;

        requester.common.negotiate_info.base_asym_sel =
            SpdmBaseAsymAlgo::TPM_ALG_ECDSA_ECC_NIST_P384;
        requester.common.negotiate_info.base_hash_sel = SpdmBaseHashAlgo::TPM_ALG_SHA_384;

        requester.common.peer_info.peer_cert_chain.cert_chain = REQ_CERT_CHAIN_DATA;

        requester.common.reset_runtime_info();

        requester.common.session = [SpdmSession::new(); 4];
        requester.common.session[0].setup(4294901758).unwrap();
        requester.common.session[0].set_crypto_param(
            SpdmBaseHashAlgo::TPM_ALG_SHA_384,
            SpdmDheAlgo::SECP_384_R1,
            SpdmAeadAlgo::AES_256_GCM,
            SpdmKeyScheduleAlgo::SPDM_KEY_SCHEDULE,
        );
        requester.common.session[0]
            .set_session_state(crate::session::SpdmSessionState::SpdmSessionHandshaking);

        // let _ = requester.send_receive_spdm_finish(4294901758);
        let status = requester.send_receive_spdm_finish(4294901758).is_ok();
        assert!(status);
    }
}
