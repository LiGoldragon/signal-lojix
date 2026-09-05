//! Ordinary Signal contract for the lojix deploy orchestrator component.
//!
//! `ethos/lib.ethos` is the canonical current Ethos Signal declaration. The
//! generated public Datom roots and structural binary projections are the
//! sole contract surface.

#[path = "generated.rs"]
mod generated;
pub use generated::*;

pub const LOJIX_SIGNAL_SOURCE: &str = include_str!("../ethos/lib.ethos");

/// The allocated ordinary Lojix wire contract: seat 1, structural revision 4.
pub enum LojixWire {}

impl signal_frame::WireContract for LojixWire {
    const BINDING: signal_frame::ContractBinding = signal_frame::ContractBinding::new(
        signal_frame::ContractId::new(core::num::NonZeroU32::MIN),
        signal_frame::WireRevision::new(
            core::num::NonZeroU16::new(4).expect("the Lojix structural revision is nonzero"),
        ),
    );
}

pub type LojixFrame = signal_frame::BoundExchangeFrame<LojixWire, RequestWire, ResponseWire>;

#[derive(Debug)]
pub enum ExchangeDecodeFault {
    Frame(signal_frame::FrameError),
    UnexpectedFrameBody,
    MultiplePayloads { count: usize },
    RouteMismatch {
        expected: signal_frame::WireRoute,
        actual: signal_frame::WireRoute,
    },
    Wire(WireFault),
}

impl From<signal_frame::FrameError> for ExchangeDecodeFault {
    fn from(fault: signal_frame::FrameError) -> Self {
        Self::Frame(fault)
    }
}

fn request_route(request: &Request) -> signal_frame::WireRoute {
    let variant = match request {
        Request::CheckHostKeyMaterial(_) => 0,
        Request::WatchDeployments(_) => 1,
        Request::Query(_) => 2,
        Request::WatchCacheRetention(_) => 3,
        Request::Unwatch(_) => 4,
    };
    signal_frame::WireRoute::new(signal_frame::RootCode::new(0), signal_frame::VariantCode::new(variant))
}

fn response_route(response: &Response) -> signal_frame::WireRoute {
    let variant = match response {
        Response::TestRunsQueried(_) => 0,
        Response::UnwatchRejected(_) => 1,
        Response::QueryRejected(_) => 2,
        Response::Watching(_) => 3,
        Response::KeyMaterialCheckRejected(_) => 4,
        Response::Queried(_) => 5,
        Response::DeploymentEventsQueried(_) => 6,
        Response::Unwatched(_) => 7,
        Response::KeyMaterialChecked(_) => 8,
        Response::WatchRejected(_) => 9,
    };
    signal_frame::WireRoute::new(signal_frame::RootCode::new(1), signal_frame::VariantCode::new(variant))
}

pub fn encode_request(
    exchange: signal_frame::ExchangeIdentifier,
    request: Request,
) -> Result<Vec<u8>, signal_frame::FrameError> {
    let route = request_route(&request);
    let wire = request.into_wire();
    LojixFrame::new(
        route,
        signal_frame::ExchangeFrameBody::Request {
            exchange,
            request: signal_frame::Request::from_payload(wire),
        },
    )
    .encode_length_prefixed()
}

pub fn encode_response(
    exchange: signal_frame::ExchangeIdentifier,
    response: Response,
) -> Result<Vec<u8>, signal_frame::FrameError> {
    let route = response_route(&response);
    let wire = response.into_wire();
    LojixFrame::new(
        route,
        signal_frame::ExchangeFrameBody::Reply {
            exchange,
            reply: signal_frame::Reply::committed(signal_frame::NonEmpty::single(
                signal_frame::SubReply::Ok(wire),
            )),
        },
    )
    .encode_length_prefixed()
}

pub fn decode_request(
    bytes: &[u8],
) -> Result<(signal_frame::ExchangeIdentifier, Request), ExchangeDecodeFault> {
    let frame = LojixFrame::decode_length_prefixed(bytes)?;
    let actual = frame.short_header().route();
    let signal_frame::ExchangeFrameBody::Request { exchange, request } = frame.into_body() else {
        return Err(ExchangeDecodeFault::UnexpectedFrameBody);
    };
    if request.payloads().len() != 1 {
        return Err(ExchangeDecodeFault::MultiplePayloads {
            count: request.payloads().len(),
        });
    }
    let request = Request::try_from_wire(request.payloads().clone().into_head())
        .map_err(ExchangeDecodeFault::Wire)?;
    let expected = request_route(&request);
    if actual != expected {
        return Err(ExchangeDecodeFault::RouteMismatch { expected, actual });
    }
    Ok((exchange, request))
}
