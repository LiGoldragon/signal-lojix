use signal_lojix::schema::lib::{
    AdmissionMarker, DatabaseMarker, DeploymentEventsQueriedPayload, DeploymentPhase,
    DeploymentPhaseEvent, DeploymentTerminal, EventLogPage, FrameBody, GenerationListing, Input,
    NodeSelector, Output, Selection, TerminalMarker, TransitionMarker,
};

fn exchange() -> signal_frame::ExchangeIdentifier {
    signal_frame::ExchangeIdentifier::new(
        signal_frame::SessionEpoch::new(9),
        signal_frame::ExchangeLane::Connector,
        signal_frame::LaneSequence::new(3),
    )
}

fn marker() -> DatabaseMarker {
    DatabaseMarker {
        commit_sequence: 1.into(),
        state_digest: 1.into(),
    }
}

fn query_input() -> Input {
    Input::Query(
        Selection::ByNode(NodeSelector {
            cluster_name: "goldragon".to_string().into(),
            node_name: "ouranos".to_string().into(),
            optional_generation_artifact: None,
        })
        .into(),
    )
}

#[test]
fn default_build_round_trips_ordinary_request_without_dotos_text() {
    let input = query_input();
    let frame = input
        .clone()
        .encode_request_frame(exchange())
        .expect("encode request");
    let (decoded_exchange, decoded) =
        signal_lojix::schema::lib::ContractMarker::decode_single_request(&frame)
            .expect("decode request");

    assert_eq!(decoded_exchange, exchange());
    assert_eq!(decoded, input);
}

#[test]
fn default_build_round_trips_ordinary_reply_without_dotos_text() {
    let output = Output::Queried(
        GenerationListing {
            generation_vector: Vec::new(),
            deployment_record_vector: Vec::new(),
            database_marker: marker(),
        }
        .into(),
    );
    let frame = output
        .clone()
        .encode_reply_frame(exchange())
        .expect("encode reply");
    let decoded =
        signal_lojix::schema::lib::ContractMarker::decode_frame(&frame).expect("decode reply");

    assert_eq!(
        decoded.into_body(),
        FrameBody::Reply {
            exchange: exchange(),
            reply: signal_frame::Reply::committed(signal_frame::NonEmpty::single(
                signal_frame::SubReply::Ok(output),
            )),
        },
    );
}

#[test]
fn deployment_phase_event_completed_round_trips_through_the_binary_frame() {
    // These are intentionally separate nominal wrappers.  A transition event
    // must not accidentally carry the admission or terminal commit marker.
    let admission_marker: AdmissionMarker = marker().into();
    let terminal_marker: TerminalMarker = marker().into();
    let transition_marker: TransitionMarker = marker().into();
    assert_eq!(admission_marker.payload(), terminal_marker.payload());
    assert_eq!(terminal_marker.payload(), transition_marker.payload());

    let event = DeploymentPhaseEvent {
        deployment_identifier: 41.into(),
        generation_identifier: 17.into(),
        cluster_name: "goldragon".to_string().into(),
        node_name: "ouranos".to_string().into(),
        deployment_phase: DeploymentPhase::Completed,
        event_log_position: 9.into(),
        transition_marker,
        optional_immutable_revision: Some("a".repeat(40).into()),
        optional_deployment_terminal: Some(DeploymentTerminal::Succeeded),
    };
    let output =
        Output::DeploymentEventsQueried(DeploymentEventsQueriedPayload::from(EventLogPage {
            deployment_phase_event_vector: vec![event.clone()],
            cache_retention_transition_event_vector: Vec::new(),
            database_marker: marker(),
        }));

    let frame = output
        .clone()
        .encode_reply_frame(exchange())
        .expect("encode event frame");
    let decoded = signal_lojix::schema::lib::ContractMarker::decode_frame(&frame)
        .expect("decode event frame");
    let FrameBody::Reply {
        exchange: decoded_exchange,
        reply,
    } = decoded.into_body()
    else {
        panic!("decoded frame must retain a reply body");
    };
    assert_eq!(decoded_exchange, exchange());
    let signal_frame::Reply::Accepted { per_operation, .. } = reply else {
        panic!("decoded event reply must be accepted");
    };
    let signal_frame::SubReply::Ok(Output::DeploymentEventsQueried(page)) =
        per_operation.into_head()
    else {
        panic!("decoded output must retain the deployment-event route");
    };
    let recovered = &page.payload().deployment_phase_event_vector[0];
    assert_eq!(recovered, &event);
    assert_eq!(recovered.deployment_phase, DeploymentPhase::Completed);
    assert_eq!(
        recovered.optional_deployment_terminal,
        Some(DeploymentTerminal::Succeeded),
    );
}
