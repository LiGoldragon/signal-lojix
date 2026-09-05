use core::num::{NonZeroU16, NonZeroU32};

use datom_codec::{Actualizable, IncorporationBudget, Potential, Textualizable};
use signal_frame::{
    BoundExchangeFrame, ContractBinding, ContractId, ExchangeFrameBody, ExchangeIdentifier,
    ExchangeLane, LaneSequence, RootCode, SessionEpoch, VariantCode, WireContract, WireRevision,
    WireRoute,
};
use signal_lojix::{
    ExchangeDecodeFault, LojixWire, Request, RequestWire, SubscriptionClose,
    WireConversion, decode_request, encode_request,
};

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(9),
        ExchangeLane::Connector,
        LaneSequence::new(3),
    )
}

fn request() -> Request {
    Request::Unwatch(SubscriptionClose(7))
}

#[test]
fn typed_datom_request_round_trips_and_rejects_malformed_text() {
    let expected = request();
    let text = <Request as Textualizable<datom_codec::Datom>>::textualize(&expected);
    let recovered = Potential::<Request>::from(text.as_str())
        .actualize(IncorporationBudget::try_from(128).unwrap())
        .expect("typed Datom request decodes");
    assert_eq!(recovered, expected);
    assert!(Potential::<Request>::from("Unwatch.{ nope }")
        .actualize(IncorporationBudget::try_from(128).unwrap())
        .is_err());
}

#[test]
fn bound_structural_request_round_trips_with_its_contract_route() {
    let expected = request();
    let bytes = encode_request(exchange(), expected.clone()).expect("encode bound frame");
    let (found_exchange, found) = decode_request(&bytes).expect("decode bound frame");
    assert_eq!(found_exchange, exchange());
    assert_eq!(found, expected);
}

struct WrongContract;
impl WireContract for WrongContract {
    const BINDING: ContractBinding = ContractBinding::new(
        ContractId::new(NonZeroU32::new(1).unwrap()),
        WireRevision::new(NonZeroU16::new(4).unwrap()),
    );
}

struct WrongRevision;
impl WireContract for WrongRevision {
    const BINDING: ContractBinding = ContractBinding::new(
        ContractId::new(NonZeroU32::new(5).unwrap()),
        WireRevision::new(NonZeroU16::new(5).unwrap()),
    );
}

fn forged<Contract: WireContract>(route: WireRoute) -> Vec<u8> {
    let payload = request().into_wire();
    BoundExchangeFrame::<Contract, RequestWire, signal_lojix::ResponseWire>::new(
        route,
        ExchangeFrameBody::Request {
            exchange: exchange(),
            request: signal_frame::Request::from_payload(payload),
        },
    )
    .encode_length_prefixed()
    .unwrap()
}

#[test]
fn binding_revision_route_and_malformed_archives_fail_closed() {
    let expected_route = WireRoute::new(RootCode::new(0), VariantCode::new(4));
    assert!(matches!(
        decode_request(&forged::<WrongContract>(expected_route)),
        Err(ExchangeDecodeFault::Frame(signal_frame::FrameError::ContractMismatch { .. }))
    ));
    assert!(matches!(
        decode_request(&forged::<WrongRevision>(expected_route)),
        Err(ExchangeDecodeFault::Frame(signal_frame::FrameError::UnsupportedWireRevision { .. }))
    ));
    assert!(matches!(
        decode_request(&forged::<LojixWire>(WireRoute::new(RootCode::new(1), VariantCode::new(4)))),
        Err(ExchangeDecodeFault::RouteMismatch { .. })
    ));
    assert!(matches!(decode_request(&[0, 0, 0, 1]), Err(ExchangeDecodeFault::Frame(_))));
}
