use signal_lojix::schema::lib::{
    ContractMarker, FrameBody, InputRoute, OutputRoute, z2VTvQ, z2VYRj, z2VZ95, z2VZWT, z2VcE8,
    z2VcR1, z2Ve15,
};

fn exchange() -> signal_frame::ExchangeIdentifier {
    signal_frame::ExchangeIdentifier::new(
        signal_frame::SessionEpoch::new(9),
        signal_frame::ExchangeLane::Connector,
        signal_frame::LaneSequence::new(3),
    )
}

fn unwatch_request() -> z2VTvQ {
    z2VTvQ::z2VUL5(z2VcE8::new(z2VZ95 {
        field_0: z2VYRj::new(7),
    }))
}

fn unwatched_reply() -> z2VcR1 {
    z2VcR1::z2VNFq(z2Ve15::new(z2VZWT {
        field_0: z2VYRj::new(7),
    }))
}

#[test]
fn handwritten_input_role_round_trips_the_encoded_request() {
    let input = unwatch_request();
    assert_eq!(input.route(), InputRoute::Unwatch);
    let frame = input
        .clone()
        .encode_request_frame(exchange())
        .expect("encode request");
    let (decoded_exchange, decoded) =
        ContractMarker::decode_single_request(&frame).expect("decode request");
    assert_eq!(decoded_exchange, exchange());
    assert_eq!(decoded, input);
}

#[test]
fn handwritten_output_role_round_trips_the_encoded_reply() {
    let output = unwatched_reply();
    assert_eq!(output.route(), OutputRoute::Unwatched);
    let frame = output
        .clone()
        .encode_reply_frame(exchange())
        .expect("encode reply");
    let decoded = ContractMarker::decode_frame(&frame).expect("decode reply");
    assert_eq!(
        decoded.into_body(),
        FrameBody::Reply {
            exchange: exchange(),
            reply: signal_frame::Reply::committed(signal_frame::NonEmpty::single(
                signal_frame::SubReply::Ok(output),
            )),
        }
    );
}
