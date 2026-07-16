#![cfg(feature = "nota-text")]

use nota::{NotaDecode, NotaEncode, NotaSource};
use signal_lojix::schema::lib::{
    CacheRetentionWatch, DatabaseMarker, GenerationListing, Input, NodeSelector, Output, Selection,
    SubscriptionOpened,
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

fn watch_input() -> Input {
    Input::WatchCacheRetention(
        CacheRetentionWatch {
            optional_cluster_name: Some("goldragon".to_string().into()),
            optional_node_name: None,
        }
        .into(),
    )
}

fn queried_output() -> Output {
    Output::Queried(
        GenerationListing {
            generation_vector: Vec::new(),
            database_marker: marker(),
        }
        .into(),
    )
}

fn watching_output() -> Output {
    Output::Watching(
        SubscriptionOpened {
            subscription_token: 7.into(),
            commit_sequence: 1.into(),
        }
        .into(),
    )
}

fn round_trip_nota<Value>(value: Value)
where
    Value: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let encoded = value.to_nota();
    let recovered = NotaSource::new(&encoded)
        .parse::<Value>()
        .expect("decode nota text");
    assert_eq!(recovered, value);
}

#[test]
fn ordinary_requests_round_trip_through_rkyv_frames() {
    for request in [query_input(), watch_input()] {
        let frame = request.encode_signal_frame().expect("encode request");
        let (_route, decoded) = Input::decode_signal_frame(&frame).expect("decode request");
        assert_eq!(decoded, request);
    }
}

#[test]
fn ordinary_replies_round_trip_through_rkyv_frames() {
    for reply in [queried_output(), watching_output()] {
        let frame = reply.encode_signal_frame().expect("encode reply");
        let (_route, decoded) = Output::decode_signal_frame(&frame).expect("decode reply");
        assert_eq!(decoded, reply);
    }
}

#[test]
fn ordinary_roots_round_trip_through_nota_text() {
    round_trip_nota(query_input());
    round_trip_nota(watch_input());
    round_trip_nota(queried_output());
    round_trip_nota(watching_output());
}

#[test]
fn ordinary_nota_heads_are_contract_local_verbs() {
    assert!(query_input().to_nota().starts_with("(Query "));
    assert!(watch_input().to_nota().starts_with("(WatchCacheRetention "));
    assert!(queried_output().to_nota().starts_with("(Queried "));
    assert!(watching_output().to_nota().starts_with("(Watching "));
}
