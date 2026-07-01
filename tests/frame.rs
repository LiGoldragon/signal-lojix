use signal_lojix::schema::lib::{
    DatabaseMarker, GenerationListing, Input, NodeSelector, Output, Selection,
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
            artifact: None,
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
            generations: Vec::new(),
            database_marker: marker(),
        }
        .into(),
    );
    let frame = output.encode_signal_frame().expect("encode reply");
    let (_route, decoded) = Output::decode_signal_frame(&frame).expect("decode reply");

    assert_eq!(decoded, output);
}
