// Copyright (c) 2020 Intel Corporation
//
// SPDX-License-Identifier: BSD-2-Clause-Patent

use fuzzlib::*;

fn fuzz_send_receive_spdm_certificate(fuzzdata: &[u8]) {
    let (rsp_config_info, rsp_provision_info) = rsp_create_info();
    let (req_config_info, req_provision_info) = req_create_info();

    let shared_buffer = SharedBuffer::new();
    let mut device_io_responder = FuzzSpdmDeviceIoReceve::new(&shared_buffer, fuzzdata);

    let pcidoe_transport_encap = &mut PciDoeTransportEncap {};

    spdmlib::crypto::asym_sign::register(ASYM_SIGN_IMPL);

    let mut responder = responder::ResponderContext::new(
        &mut device_io_responder,
        pcidoe_transport_encap,
        rsp_config_info,
        rsp_provision_info,
    );

    // version_rsp
    responder.common.reset_runtime_info();
    responder.common.runtime_info.message_a.append_message(&[
        16, 132, 0, 0, 17, 4, 0, 0, 0, 2, 0, 16, 0, 17, 17, 225, 0, 0, 0, 0, 0, 0, 198, 118, 0, 0,
        17, 97, 0, 0, 0, 0, 0, 0, 246, 122, 0, 0, 17, 227, 4, 0, 48, 0, 1, 0, 128, 0, 0, 0, 2, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 32, 16, 0, 3, 32, 2, 0, 4, 32, 2,
        0, 5, 32, 1, 0, 17, 99, 4, 0, 52, 0, 1, 0, 4, 0, 0, 0, 128, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 32, 16, 0, 3, 32, 2, 0, 4, 32, 2, 0, 5, 32, 1, 0
    ]);

    // capability_rsp
    responder.common.negotiate_info.req_ct_exponent_sel = 0;
    responder.common.negotiate_info.req_capabilities_sel = SpdmRequestCapabilityFlags::CERT_CAP
    | SpdmRequestCapabilityFlags::CHAL_CAP
    | SpdmRequestCapabilityFlags::ENCRYPT_CAP
    | SpdmRequestCapabilityFlags::MAC_CAP
    //| SpdmRequestCapabilityFlags::MUT_AUTH_CAP
    | SpdmRequestCapabilityFlags::KEY_EX_CAP
    | SpdmRequestCapabilityFlags::PSK_CAP
    | SpdmRequestCapabilityFlags::ENCAP_CAP
    | SpdmRequestCapabilityFlags::HBEAT_CAP
    | SpdmRequestCapabilityFlags::KEY_UPD_CAP;
    responder.common.negotiate_info.rsp_ct_exponent_sel = 0;
    responder.common.negotiate_info.rsp_capabilities_sel = SpdmResponseCapabilityFlags::CERT_CAP
    | SpdmResponseCapabilityFlags::CHAL_CAP
    | SpdmResponseCapabilityFlags::MEAS_CAP_SIG
    | SpdmResponseCapabilityFlags::MEAS_FRESH_CAP
    | SpdmResponseCapabilityFlags::ENCRYPT_CAP
    | SpdmResponseCapabilityFlags::MAC_CAP
    //| SpdmResponseCapabilityFlags::MUT_AUTH_CAP
    | SpdmResponseCapabilityFlags::KEY_EX_CAP
    | SpdmResponseCapabilityFlags::PSK_CAP_WITH_CONTEXT
    | SpdmResponseCapabilityFlags::ENCAP_CAP
    | SpdmResponseCapabilityFlags::HBEAT_CAP
    | SpdmResponseCapabilityFlags::KEY_UPD_CAP;

    // algorithm_rsp
    responder
        .common
        .negotiate_info
        .measurement_specification_sel = SpdmMeasurementSpecification::DMTF;
    responder.common.negotiate_info.measurement_hash_sel = SpdmMeasurementHashAlgo::TPM_ALG_SHA_384;
    responder.common.negotiate_info.base_hash_sel = SpdmBaseHashAlgo::TPM_ALG_SHA_384;
    responder.common.negotiate_info.base_asym_sel = SpdmBaseAsymAlgo::TPM_ALG_ECDSA_ECC_NIST_P384;
    responder.common.negotiate_info.dhe_sel = SpdmDheAlgo::SECP_384_R1;
    responder.common.negotiate_info.aead_sel = SpdmAeadAlgo::AES_256_GCM;
    responder.common.negotiate_info.req_asym_sel = SpdmReqAsymAlgo::TPM_ALG_RSAPSS_2048;
    responder.common.negotiate_info.key_schedule_sel = SpdmKeyScheduleAlgo::SPDM_KEY_SCHEDULE;
    responder.common.provision_info.my_cert_chain = Some(SpdmCertChainData {
        data_size: 1544,
        data: [
            8, 6, 0, 0, 90, 100, 179, 139, 93, 95, 77, 179, 95, 178, 170, 29, 70, 159, 106, 220,
            202, 127, 172, 133, 190, 240, 132, 16, 156, 205, 84, 9, 240, 171, 56, 58, 170, 247,
            166, 46, 59, 215, 129, 44, 234, 36, 126, 20, 169, 86, 157, 40, 48, 130, 1, 207, 48,
            130, 1, 86, 160, 3, 2, 1, 2, 2, 20, 32, 58, 194, 89, 204, 218, 203, 246, 114, 241, 192,
            26, 98, 26, 69, 130, 144, 36, 184, 175, 48, 10, 6, 8, 42, 134, 72, 206, 61, 4, 3, 3,
            48, 31, 49, 29, 48, 27, 6, 3, 85, 4, 3, 12, 20, 105, 110, 116, 101, 108, 32, 116, 101,
            115, 116, 32, 69, 67, 80, 50, 53, 54, 32, 67, 65, 48, 30, 23, 13, 50, 49, 48, 50, 48,
            57, 48, 48, 53, 48, 53, 56, 90, 23, 13, 51, 49, 48, 50, 48, 55, 48, 48, 53, 48, 53, 56,
            90, 48, 31, 49, 29, 48, 27, 6, 3, 85, 4, 3, 12, 20, 105, 110, 116, 101, 108, 32, 116,
            101, 115, 116, 32, 69, 67, 80, 50, 53, 54, 32, 67, 65, 48, 118, 48, 16, 6, 7, 42, 134,
            72, 206, 61, 2, 1, 6, 5, 43, 129, 4, 0, 34, 3, 98, 0, 4, 153, 143, 129, 104, 154, 131,
            155, 131, 57, 173, 14, 50, 141, 185, 66, 13, 174, 204, 145, 169, 188, 74, 225, 187,
            121, 76, 34, 250, 63, 12, 157, 147, 60, 26, 2, 92, 194, 115, 5, 236, 67, 93, 4, 2, 177,
            104, 179, 244, 216, 222, 12, 141, 83, 183, 4, 142, 161, 67, 154, 235, 49, 13, 170, 206,
            137, 45, 186, 115, 218, 79, 30, 57, 93, 146, 17, 33, 56, 180, 0, 212, 245, 85, 140,
            232, 113, 48, 61, 70, 131, 244, 196, 82, 80, 218, 18, 91, 163, 83, 48, 81, 48, 29, 6,
            3, 85, 29, 14, 4, 22, 4, 20, 207, 9, 212, 122, 238, 8, 144, 98, 191, 230, 156, 180,
            185, 223, 225, 65, 51, 28, 3, 165, 48, 31, 6, 3, 85, 29, 35, 4, 24, 48, 22, 128, 20,
            207, 9, 212, 122, 238, 8, 144, 98, 191, 230, 156, 180, 185, 223, 225, 65, 51, 28, 3,
            165, 48, 15, 6, 3, 85, 29, 19, 1, 1, 255, 4, 5, 48, 3, 1, 1, 255, 48, 10, 6, 8, 42,
            134, 72, 206, 61, 4, 3, 3, 3, 103, 0, 48, 100, 2, 48, 90, 180, 245, 149, 37, 130, 246,
            104, 62, 73, 199, 180, 187, 66, 129, 145, 126, 56, 208, 45, 172, 83, 174, 142, 176, 81,
            80, 170, 248, 126, 255, 192, 48, 171, 213, 8, 91, 6, 247, 225, 191, 57, 210, 62, 174,
            191, 142, 72, 2, 48, 9, 117, 168, 192, 111, 79, 60, 173, 93, 78, 79, 248, 44, 59, 57,
            70, 160, 223, 131, 142, 181, 211, 97, 97, 89, 188, 57, 215, 173, 104, 94, 13, 79, 63,
            226, 202, 193, 116, 143, 71, 55, 17, 200, 34, 89, 111, 100, 82, 48, 130, 1, 215, 48,
            130, 1, 93, 160, 3, 2, 1, 2, 2, 1, 1, 48, 10, 6, 8, 42, 134, 72, 206, 61, 4, 3, 3, 48,
            31, 49, 29, 48, 27, 6, 3, 85, 4, 3, 12, 20, 105, 110, 116, 101, 108, 32, 116, 101, 115,
            116, 32, 69, 67, 80, 50, 53, 54, 32, 67, 65, 48, 30, 23, 13, 50, 49, 48, 50, 48, 57,
            48, 48, 53, 48, 53, 57, 90, 23, 13, 51, 49, 48, 50, 48, 55, 48, 48, 53, 48, 53, 57, 90,
            48, 46, 49, 44, 48, 42, 6, 3, 85, 4, 3, 12, 35, 105, 110, 116, 101, 108, 32, 116, 101,
            115, 116, 32, 69, 67, 80, 50, 53, 54, 32, 105, 110, 116, 101, 114, 109, 101, 100, 105,
            97, 116, 101, 32, 99, 101, 114, 116, 48, 118, 48, 16, 6, 7, 42, 134, 72, 206, 61, 2, 1,
            6, 5, 43, 129, 4, 0, 34, 3, 98, 0, 4, 119, 27, 36, 246, 198, 118, 31, 184, 48, 7, 139,
            184, 163, 158, 192, 38, 193, 234, 125, 252, 41, 125, 224, 89, 178, 100, 50, 117, 74,
            227, 2, 100, 60, 188, 133, 142, 198, 236, 239, 176, 121, 244, 193, 164, 185, 187, 41,
            107, 174, 173, 240, 125, 99, 198, 175, 179, 115, 94, 79, 63, 254, 137, 138, 187, 125,
            43, 96, 62, 22, 186, 130, 207, 164, 112, 4, 133, 195, 163, 60, 94, 106, 160, 239, 218,
            213, 32, 48, 25, 186, 121, 149, 176, 194, 127, 76, 221, 163, 94, 48, 92, 48, 12, 6, 3,
            85, 29, 19, 4, 5, 48, 3, 1, 1, 255, 48, 11, 6, 3, 85, 29, 15, 4, 4, 3, 2, 1, 254, 48,
            29, 6, 3, 85, 29, 14, 4, 22, 4, 20, 18, 224, 26, 35, 198, 35, 228, 2, 88, 11, 6, 172,
            144, 250, 75, 128, 61, 201, 241, 29, 48, 32, 6, 3, 85, 29, 37, 1, 1, 255, 4, 22, 48,
            20, 6, 8, 43, 6, 1, 5, 5, 7, 3, 1, 6, 8, 43, 6, 1, 5, 5, 7, 3, 2, 48, 10, 6, 8, 42,
            134, 72, 206, 61, 4, 3, 3, 3, 104, 0, 48, 101, 2, 48, 3, 50, 177, 139, 32, 244, 118,
            218, 140, 131, 150, 135, 85, 217, 18, 114, 189, 88, 77, 10, 55, 175, 41, 149, 29, 54,
            196, 158, 165, 205, 226, 59, 245, 224, 122, 100, 54, 30, 212, 241, 225, 187, 20, 87,
            158, 134, 130, 114, 2, 49, 0, 192, 214, 2, 153, 80, 118, 52, 22, 214, 81, 156, 196,
            134, 8, 104, 148, 191, 60, 9, 126, 16, 229, 98, 138, 186, 72, 10, 165, 237, 26, 106,
            246, 60, 47, 77, 56, 93, 125, 92, 96, 99, 136, 132, 93, 73, 51, 226, 167, 48, 130, 2,
            34, 48, 130, 1, 168, 160, 3, 2, 1, 2, 2, 1, 3, 48, 10, 6, 8, 42, 134, 72, 206, 61, 4,
            3, 3, 48, 46, 49, 44, 48, 42, 6, 3, 85, 4, 3, 12, 35, 105, 110, 116, 101, 108, 32, 116,
            101, 115, 116, 32, 69, 67, 80, 50, 53, 54, 32, 105, 110, 116, 101, 114, 109, 101, 100,
            105, 97, 116, 101, 32, 99, 101, 114, 116, 48, 30, 23, 13, 50, 49, 48, 50, 48, 57, 48,
            48, 53, 48, 53, 57, 90, 23, 13, 50, 50, 48, 50, 48, 57, 48, 48, 53, 48, 53, 57, 90, 48,
            43, 49, 41, 48, 39, 6, 3, 85, 4, 3, 12, 32, 105, 110, 116, 101, 108, 32, 116, 101, 115,
            116, 32, 69, 67, 80, 50, 53, 54, 32, 114, 101, 115, 112, 111, 110, 100, 101, 114, 32,
            99, 101, 114, 116, 48, 118, 48, 16, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 5, 43, 129, 4,
            0, 34, 3, 98, 0, 4, 108, 34, 65, 223, 183, 228, 214, 141, 83, 114, 78, 74, 27, 153,
            130, 230, 86, 210, 45, 151, 75, 152, 64, 169, 153, 214, 13, 216, 233, 166, 252, 116,
            185, 206, 137, 72, 167, 181, 9, 182, 36, 73, 214, 35, 179, 95, 58, 240, 153, 176, 202,
            99, 125, 36, 254, 233, 18, 25, 15, 194, 115, 28, 227, 118, 145, 236, 87, 108, 205, 123,
            171, 50, 253, 109, 110, 146, 125, 55, 96, 1, 219, 19, 146, 59, 119, 247, 18, 151, 29,
            94, 227, 185, 21, 131, 175, 137, 163, 129, 156, 48, 129, 153, 48, 12, 6, 3, 85, 29, 19,
            1, 1, 255, 4, 2, 48, 0, 48, 11, 6, 3, 85, 29, 15, 4, 4, 3, 2, 5, 224, 48, 29, 6, 3, 85,
            29, 14, 4, 22, 4, 20, 72, 31, 93, 149, 206, 137, 212, 125, 164, 76, 33, 143, 91, 213,
            80, 150, 255, 186, 226, 238, 48, 49, 6, 3, 85, 29, 17, 4, 42, 48, 40, 160, 38, 6, 10,
            43, 6, 1, 4, 1, 131, 28, 130, 18, 1, 160, 24, 12, 22, 65, 67, 77, 69, 58, 87, 73, 68,
            71, 69, 84, 58, 49, 50, 51, 52, 53, 54, 55, 56, 57, 48, 48, 42, 6, 3, 85, 29, 37, 1, 1,
            255, 4, 32, 48, 30, 6, 8, 43, 6, 1, 5, 5, 7, 3, 1, 6, 8, 43, 6, 1, 5, 5, 7, 3, 2, 6, 8,
            43, 6, 1, 5, 5, 7, 3, 9, 48, 10, 6, 8, 42, 134, 72, 206, 61, 4, 3, 3, 3, 104, 0, 48,
            101, 2, 48, 8, 230, 31, 13, 223, 24, 211, 47, 80, 73, 153, 176, 226, 100, 149, 48, 169,
            90, 191, 131, 118, 174, 74, 57, 216, 226, 81, 18, 132, 156, 190, 17, 29, 59, 119, 32,
            111, 5, 108, 199, 152, 178, 186, 184, 150, 117, 37, 207, 2, 49, 0, 147, 18, 91, 102,
            147, 192, 231, 86, 27, 104, 40, 39, 216, 142, 105, 170, 48, 118, 5, 111, 75, 208, 206,
            16, 15, 248, 223, 74, 171, 155, 77, 177, 71, 228, 205, 206, 206, 72, 13, 248, 53, 61,
            188, 37, 206, 236, 185, 202, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
    });

    // digest_rsp
    responder
        .common
        .runtime_info
        .message_b
        .append_message(&[17, 129, 0, 0]);
    responder.common.runtime_info.message_b.append_message(&[
        17, 1, 0, 1, 40, 175, 112, 39, 188, 45, 149, 181, 160, 228, 38, 4, 197, 140, 92, 60, 191,
        162, 200, 36, 166, 48, 202, 47, 15, 74, 121, 53, 87, 251, 57, 59, 221, 138, 200, 138, 146,
        216, 163, 112, 23, 18, 131, 155, 102, 225, 58, 58,
    ]);

    let pcidoe_transport_encap2 = &mut PciDoeTransportEncap {};
    let mut device_io_requester =
        fake_device_io::FakeSpdmDeviceIo::new(&shared_buffer, &mut responder);

    let mut requester = requester::RequesterContext::new(
        &mut device_io_requester,
        pcidoe_transport_encap2,
        req_config_info,
        req_provision_info,
    );

    // version_req
    requester.common.reset_runtime_info();
    requester
        .common
        .runtime_info
        .message_a
        .append_message(&[
            16, 132, 0, 0, 17, 4, 0, 0, 0, 2, 0, 16, 0, 17, 17, 225, 0, 0, 0, 0, 0, 0, 198, 118, 0, 0,
            17, 97, 0, 0, 0, 0, 0, 0, 246, 122, 0, 0, 17, 227, 4, 0, 48, 0, 1, 0, 128, 0, 0, 0, 2, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 32, 16, 0, 3, 32, 2, 0, 4, 32, 2,
            0, 5, 32, 1, 0, 17, 99, 4, 0, 52, 0, 1, 0, 4, 0, 0, 0, 128, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 32, 16, 0, 3, 32, 2, 0, 4, 32, 2, 0, 5, 32, 1, 0
        ]);

    // capability_req
    requester.common.negotiate_info.req_ct_exponent_sel = 0;
    requester.common.negotiate_info.req_capabilities_sel =
        requester.common.config_info.req_capabilities;
    requester.common.negotiate_info.req_capabilities_sel = SpdmRequestCapabilityFlags::CERT_CAP
    | SpdmRequestCapabilityFlags::CHAL_CAP
    | SpdmRequestCapabilityFlags::ENCRYPT_CAP
    | SpdmRequestCapabilityFlags::MAC_CAP
    //| SpdmRequestCapabilityFlags::MUT_AUTH_CAP
    | SpdmRequestCapabilityFlags::KEY_EX_CAP
    | SpdmRequestCapabilityFlags::PSK_CAP
    | SpdmRequestCapabilityFlags::ENCAP_CAP
    | SpdmRequestCapabilityFlags::HBEAT_CAP
    | SpdmRequestCapabilityFlags::KEY_UPD_CAP;
    requester.common.negotiate_info.rsp_ct_exponent_sel = 0;
    requester.common.negotiate_info.rsp_capabilities_sel = SpdmResponseCapabilityFlags::CERT_CAP
    | SpdmResponseCapabilityFlags::CHAL_CAP
    | SpdmResponseCapabilityFlags::MEAS_CAP_SIG
    | SpdmResponseCapabilityFlags::MEAS_FRESH_CAP
    | SpdmResponseCapabilityFlags::ENCRYPT_CAP
    | SpdmResponseCapabilityFlags::MAC_CAP
    //| SpdmResponseCapabilityFlags::MUT_AUTH_CAP
    | SpdmResponseCapabilityFlags::KEY_EX_CAP
    | SpdmResponseCapabilityFlags::PSK_CAP_WITH_CONTEXT
    | SpdmResponseCapabilityFlags::ENCAP_CAP
    | SpdmResponseCapabilityFlags::HBEAT_CAP
    | SpdmResponseCapabilityFlags::KEY_UPD_CAP;

    //algorithm_req
    requester
        .common
        .negotiate_info
        .measurement_specification_sel = SpdmMeasurementSpecification::DMTF;
    requester.common.negotiate_info.measurement_hash_sel = SpdmMeasurementHashAlgo::TPM_ALG_SHA_384;
    requester.common.negotiate_info.base_hash_sel = SpdmBaseHashAlgo::TPM_ALG_SHA_384;
    requester.common.negotiate_info.base_asym_sel = SpdmBaseAsymAlgo::TPM_ALG_ECDSA_ECC_NIST_P384;
    requester.common.negotiate_info.dhe_sel = SpdmDheAlgo::SECP_384_R1;
    requester.common.negotiate_info.aead_sel = SpdmAeadAlgo::AES_256_GCM;
    requester.common.negotiate_info.req_asym_sel = SpdmReqAsymAlgo::TPM_ALG_RSAPSS_2048;
    requester.common.negotiate_info.key_schedule_sel = SpdmKeyScheduleAlgo::SPDM_KEY_SCHEDULE;

    // digest_req
    requester
        .common
        .runtime_info
        .message_b
        .append_message(&[17, 129, 0, 0]);
    requester.common.runtime_info.message_b.append_message(&[
        17, 1, 0, 1, 40, 175, 112, 39, 188, 45, 149, 181, 160, 228, 38, 4, 197, 140, 92, 60, 191,
        162, 200, 36, 166, 48, 202, 47, 15, 74, 121, 53, 87, 251, 57, 59, 221, 138, 200, 138, 146,
        216, 163, 112, 23, 18, 131, 155, 102, 225, 58, 58,
    ]);

    let _ = requester.send_receive_spdm_certificate(0).is_err();
}

fn main() {
    //     afl::fuzz!(|data: &[u8]| {
    //     fuzz_send_receive_spdm_certificate(data, 5);
    // });
    fuzz_send_receive_spdm_certificate(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
}
