#![cfg(feature = "dotos-text")]

use dotos::{DotosDecode, DotosEncode, DotosSource};
use signal_lojix::schema::lib::{
    ActivationEffect, CacheRetentionWatch, DatabaseMarker, Generation, GenerationArtifact,
    GenerationListing, GenerationSlot, Input, NodeSelector, Output, RequestedGenerationArtifact,
    Selection, SubscriptionOpened, TestMode, TestOutcome, TestRunListing, TestRunPhase,
    TestRunRecord,
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
            optional_requested_generation_artifact: None,
        })
        .into(),
    )
}

fn selected_query_input(artifact: RequestedGenerationArtifact) -> Input {
    Input::Query(
        Selection::ByNode(NodeSelector {
            cluster_name: "goldragon".to_string().into(),
            node_name: "ouranos".to_string().into(),
            optional_requested_generation_artifact: Some(artifact),
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
            deployment_record_vector: Vec::new(),
            database_marker: marker(),
        }
        .into(),
    )
}

fn current_generation_output() -> Output {
    Output::Queried(
        GenerationListing {
            generation_vector: vec![Generation {
                generation_identifier: 8.into(),
                deployment_identifier: 12.into(),
                cluster_name: "fixture-cluster".to_string().into(),
                node_name: "fixture-node".to_string().into(),
                generation_artifact: GenerationArtifact::CompleteHost,
                activation_effect: ActivationEffect::LiveActivation,
                generation_slot: GenerationSlot::Current,
                optional_closure_path: Some(
                    "/nix/store/0123456789abcdfghijklmnpqrsvwxyz-fixture-system"
                        .to_string()
                        .into(),
                ),
                optional_immutable_revision: Some("a".repeat(40).into()),
            }],
            deployment_record_vector: Vec::new(),
            database_marker: marker(),
        }
        .into(),
    )
}

fn completed_test_run_output() -> Output {
    Output::TestRunsQueried(
        TestRunListing {
            test_run_record_vector: vec![TestRunRecord {
                test_run_identifier: 13.into(),
                cluster_name: "fixture-cluster".to_string().into(),
                node: "fixture-node".to_string().into(),
                host: "fixture-host".to_string().into(),
                test_mode: TestMode::Hermetic,
                test_run_phase: TestRunPhase::Completed,
                test_outcome: TestOutcome::Passed,
                optional_closure_path: Some(
                    "/nix/store/0123456789abcdfghijklmnpqrsvwxyz-goldragon-test"
                        .to_string()
                        .into(),
                ),
            }],
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

fn round_trip_dotos<Value>(value: Value)
where
    Value: DotosEncode + DotosDecode + PartialEq + std::fmt::Debug,
{
    let encoded = value.to_dotos();
    let recovered = DotosSource::new(&encoded)
        .parse::<Value>()
        .expect("decode dotos text");
    assert_eq!(recovered, value);
}

fn assert_canonical_store_root(path: &str) {
    let store_path = path
        .strip_prefix("/nix/store/")
        .expect("closure path must use the canonical Nix store root");
    let (hash, name) = store_path
        .split_once('-')
        .expect("canonical Nix store root must include its name");
    assert_eq!(hash.len(), 32, "canonical Nix store hash width");
    assert!(
        hash.chars()
            .all(|character| "0123456789abcdfghijklmnpqrsvwxyz".contains(character)),
        "canonical Nix store hash alphabet",
    );
    assert!(!name.is_empty(), "canonical Nix store root name");
}

#[test]
fn ordinary_requests_round_trip_through_rkyv_frames() {
    for request in [query_input(), watch_input()] {
        let frame = request
            .clone()
            .encode_request_frame(exchange())
            .expect("encode request");
        let (decoded_exchange, decoded) =
            signal_lojix::schema::lib::ContractMarker::decode_single_request(&frame)
                .expect("decode request");
        assert_eq!(decoded_exchange, exchange());
        assert_eq!(decoded, request);
    }
}

#[test]
fn ordinary_replies_round_trip_through_rkyv_frames() {
    for reply in [
        queried_output(),
        current_generation_output(),
        completed_test_run_output(),
        watching_output(),
    ] {
        let frame = reply
            .clone()
            .encode_reply_frame(exchange())
            .expect("encode reply");
        let decoded =
            signal_lojix::schema::lib::ContractMarker::decode_frame(&frame).expect("decode reply");
        let signal_lojix::schema::lib::FrameBody::Reply {
            exchange: decoded_exchange,
            reply: decoded_reply,
        } = decoded.into_body()
        else {
            panic!("decoded frame must retain a reply body");
        };
        assert_eq!(decoded_exchange, exchange());
        assert_eq!(
            decoded_reply,
            signal_frame::Reply::committed(signal_frame::NonEmpty::single(
                signal_frame::SubReply::Ok(reply),
            )),
        );
    }
}

#[test]
fn ordinary_roots_round_trip_through_dotos_text() {
    round_trip_dotos(query_input());
    round_trip_dotos(selected_query_input(RequestedGenerationArtifact::BaseHost));
    round_trip_dotos(watch_input());
    round_trip_dotos(queried_output());
    round_trip_dotos(current_generation_output());
    round_trip_dotos(completed_test_run_output());
    round_trip_dotos(watching_output());
}

#[test]
fn current_closure_paths_are_canonical_when_supplied() {
    let Output::Queried(current) = current_generation_output() else {
        panic!("current fixture must be a generation listing");
    };
    let Some(current_path) = &current.payload().generation_vector[0].optional_closure_path else {
        panic!("current generation must carry its canonical closure root");
    };
    assert_canonical_store_root(current_path.payload());

    let Output::TestRunsQueried(test_runs) = completed_test_run_output() else {
        panic!("test fixture must be a test-run listing");
    };
    let Some(test_path) = &test_runs.payload().test_run_record_vector[0].optional_closure_path
    else {
        panic!("completed test run must carry its canonical closure root");
    };
    assert_canonical_store_root(test_path.payload());
}

#[test]
fn retired_legacy_vocabulary_is_rejected_at_the_wire_boundary() {
    let selected = selected_query_input(RequestedGenerationArtifact::BaseHost).to_dotos();
    let attempted_legacy_selection = selected.replace("BaseHost", "LegacyUnknownArtifact");
    assert!(
        DotosSource::new(&attempted_legacy_selection)
            .parse::<Input>()
            .is_err()
    );
}

#[test]
fn ordinary_dotos_heads_are_contract_local_verbs() {
    assert!(query_input().to_dotos().contains("Query"));
    assert!(watch_input().to_dotos().contains("WatchCacheRetention"));
    assert!(queried_output().to_dotos().contains("Queried"));
    assert!(current_generation_output().to_dotos().contains("Queried"));
    assert!(
        completed_test_run_output()
            .to_dotos()
            .contains("TestRunsQueried")
    );
    assert!(watching_output().to_dotos().contains("Watching"));
}
