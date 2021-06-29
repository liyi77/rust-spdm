use fuzzlib::*;

fn fuzz_handle_spdm_capability(data: &[u8]) {
    
    let (config_info, provision_info) = create_info();
    let pcidoe_transport_encap = &mut PciDoeTransportEncap {};
    let mctp_transport_encap = &mut MctpTransportEncap {};

    spdmlib::crypto::asym_sign::register(ASYM_SIGN_IMPL);

    let shared_buffer = SharedBuffer::new();
    let mut socket_io_transport = FakeSpdmDeviceIoReceve::new(&shared_buffer);

    let mut context = responder::ResponderContext::new(
        &mut socket_io_transport,
        if USE_PCIDOE {
            pcidoe_transport_encap
        } else {
            mctp_transport_encap
        },
        config_info,
        provision_info,
    );

    // context.handle_spdm_capability(&[0x10, 0x84, 00,00, 0x11, 0xE1, 00, 00, 00, 00, 00, 00, 00,00,00,0x0C]);
    context.handle_spdm_capability(data);
    let mut req_buf = [0u8; 512];
    socket_io_transport.receive(&mut req_buf).unwrap();
    println!("Received: {:?}", req_buf);
}

fn main() {
    afl::fuzz!(|data: &[u8]| {
        fuzz_handle_spdm_capability(data);
    });
}
