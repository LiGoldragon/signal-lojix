use signal_lojix::schema::lib::{
    ActivationEffect, DatabaseMarker, Generation, GenerationArtifact, GenerationListing,
    GenerationSlot, NodeSelector, Selection,
};
use signal_lojix::{Input, Output};

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
fn canonical_package_roots_round_trip_ordinary_reply_without_nota_text() {
    let output = Output::Queried(
        GenerationListing {
            generation_vector: vec![Generation {
                generation_identifier: 1.into(),
                deployment_identifier: 2.into(),
                cluster_name: "goldragon".to_string().into(),
                node_name: "zeus".to_string().into(),
                generation_artifact: GenerationArtifact::UserEnvironment,
                optional_user_name: Some("bird".to_string().into()),
                activation_effect: ActivationEffect::LiveActivation,
                generation_slot: GenerationSlot::Current,
                closure_path: "/nix/store/bird-home".to_string().into(),
                optional_source_revision_record: None,
            }],
            database_marker: marker(),
        }
        .into(),
    );
    let frame = output.encode_signal_frame().expect("encode reply");
    let (_route, decoded) = Output::decode_signal_frame(&frame).expect("decode reply");

    assert_eq!(decoded, output);
}
