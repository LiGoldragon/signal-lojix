use signal_lojix::schema::lib::{
    AdmissionMarker, DatabaseMarker, DeploymentEventsQueriedPayload,
    DeploymentPhase, DeploymentPhaseEvent, DeploymentTerminal, EventLogPage,
    GenerationListing, Input, NodeSelector, Output, Selection, TerminalMarker,
    TransitionMarker,
};

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
fn default_build_round_trips_ordinary_request_without_nota_text() {
    let input = query_input();
    let frame = input.encode_signal_frame().expect("encode request");
    let (_route, decoded) = Input::decode_signal_frame(&frame).expect("decode request");

    assert_eq!(decoded, input);
}

#[test]
fn default_build_round_trips_ordinary_reply_without_nota_text() {
    let output = Output::Queried(
        GenerationListing {
            generation_vector: Vec::new(),
            deployment_record_vector: Vec::new(),
            database_marker: marker(),
        }
        .into(),
    );
    let frame = output.encode_signal_frame().expect("encode reply");
    let (_route, decoded) = Output::decode_signal_frame(&frame).expect("decode reply");

    assert_eq!(decoded, output);
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
    let output = Output::DeploymentEventsQueried(
        DeploymentEventsQueriedPayload::from(EventLogPage {
            deployment_phase_event_vector: vec![event.clone()],
            cache_retention_transition_event_vector: Vec::new(),
            database_marker: marker(),
        }),
    );

    let frame = output.encode_signal_frame().expect("encode event frame");
    let (_route, decoded) = Output::decode_signal_frame(&frame).expect("decode event frame");
    assert_eq!(decoded, output);

    let Output::DeploymentEventsQueried(page) = decoded else {
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
