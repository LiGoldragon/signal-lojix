use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use signal_core::{FrameBody, Request as CoreRequest};
use signal_lojix::*;

fn cluster() -> ClusterName {
    ClusterName::from_text("goldragon").expect("cluster name")
}

fn node() -> NodeName {
    NodeName::from_text("ouranos").expect("node name")
}

fn user() -> UserName {
    UserName::from_text("li").expect("user name")
}

fn deployment() -> DeploymentId {
    DeploymentId::from_text("deploy_aab").expect("deployment id")
}

fn mutation() -> CacheRetentionMutationId {
    CacheRetentionMutationId::from_text("mutation_aab").expect("mutation id")
}

fn generation_id() -> GenerationId {
    GenerationId::from_text("generation_aab").expect("generation id")
}

fn store_path() -> StorePath {
    StorePath::from_text("/nix/store/00000000000000000000000000000000-criomos").expect("store path")
}

fn derivation_path() -> DerivationPath {
    DerivationPath::from_text("/nix/store/00000000000000000000000000000000-criomos.drv")
        .expect("derivation path")
}

fn proposal_source() -> ProposalSource {
    ProposalSource::from_text("/git/github.com/LiGoldragon/goldragon/datom.nota")
        .expect("proposal source")
}

fn flake_reference() -> FlakeReference {
    FlakeReference::from_text("github:LiGoldragon/CriomOS/horizon-re-engineering")
        .expect("flake reference")
}

fn builder_selection() -> BuilderSelection {
    BuilderSelection::NamedBuilder(NamedBuilder { node: node() })
}

fn deployment_plan() -> DeploymentPlan {
    DeploymentPlan::HomeOnlyDeployment(HomeOnlyDeployment {
        user: user(),
        mode: HomeMode::Activate,
    })
}

fn generation() -> Generation {
    Generation {
        generation: generation_id(),
        cluster: cluster(),
        node: node(),
        kind: GenerationKind::HomeOnly,
        store_path: store_path(),
        state: GenerationState::Activated,
    }
}

fn round_trip_request(request: Request) -> Request {
    let expected_verb = request.signal_verb();
    // signal-core's macro emits `into_signal_request` on the Request enum;
    // it auto-derives the verb from the variant via `signal_verb()`.
    let frame = Frame::new(FrameBody::Request(request.into_signal_request()));
    let bytes = frame.encode_length_prefixed().expect("encode frame");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode frame");

    match decoded.into_body() {
        FrameBody::Request(CoreRequest::Operation { verb, payload }) => {
            assert_eq!(verb, expected_verb);
            payload
        }
        other => panic!("expected request operation, got {other:?}"),
    }
}

fn round_trip_reply(reply: Reply) -> Reply {
    let frame = Frame::new(FrameBody::Reply(signal_core::Reply::operation(
        reply.clone(),
    )));
    let bytes = frame.encode_length_prefixed().expect("encode frame");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode frame");

    match decoded.into_body() {
        FrameBody::Reply(signal_core::Reply::Operation(decoded_reply)) => decoded_reply,
        other => panic!("expected reply operation, got {other:?}"),
    }
}

fn round_trip_nota<T>(value: T, expected: &str)
where
    T: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let mut encoder = Encoder::new();
    value.encode(&mut encoder).expect("encode nota");
    let encoded = encoder.into_string();
    assert_eq!(encoded, expected);

    let mut decoder = Decoder::new(&encoded);
    let recovered = T::decode(&mut decoder).expect("decode nota");
    assert_eq!(recovered, value);
}

#[test]
fn deployment_submission_round_trips_through_length_prefixed_frame() {
    let request = Request::DeploymentSubmission(DeploymentSubmission {
        cluster: cluster(),
        node: node(),
        source: proposal_source(),
        flake: flake_reference(),
        plan: deployment_plan(),
        builder: builder_selection(),
        substituters: vec![NodeName::from_text("prometheus").expect("node name")],
    });

    assert_eq!(round_trip_request(request.clone()), request);
}

#[test]
fn cache_retention_request_round_trips_through_length_prefixed_frame() {
    let request = Request::CacheRetentionRequest(CacheRetentionRequest {
        generation: generation_id(),
        action: CacheRetentionAction::PinGeneration(PinGeneration {}),
    });

    assert_eq!(round_trip_request(request.clone()), request);
}

#[test]
fn generation_query_round_trips_through_length_prefixed_frame() {
    let request = Request::GenerationQuery(GenerationQuery {
        cluster: Some(cluster()),
        node: None,
        kind: Some(GenerationKind::HomeOnly),
    });

    assert_eq!(round_trip_request(request.clone()), request);
}

#[test]
fn request_variants_have_expected_signal_verbs() {
    let deployment_request = Request::DeploymentSubmission(DeploymentSubmission {
        cluster: cluster(),
        node: node(),
        source: proposal_source(),
        flake: flake_reference(),
        plan: deployment_plan(),
        builder: builder_selection(),
        substituters: Vec::new(),
    });
    let cache_request = Request::CacheRetentionRequest(CacheRetentionRequest {
        generation: generation_id(),
        action: CacheRetentionAction::RetireGeneration(RetireGeneration {}),
    });
    let query_request = Request::GenerationQuery(GenerationQuery {
        cluster: None,
        node: None,
        kind: None,
    });

    assert_eq!(
        deployment_request.signal_verb(),
        signal_core::SignalVerb::Assert
    );
    assert_eq!(cache_request.signal_verb(), signal_core::SignalVerb::Mutate);
    assert_eq!(query_request.signal_verb(), signal_core::SignalVerb::Match);
}

#[test]
fn deployment_replies_round_trip_through_length_prefixed_frame() {
    let accepted = Reply::DeploymentAccepted(DeploymentAccepted {
        deployment: deployment(),
    });
    let rejected = Reply::DeploymentRejected(DeploymentRejected {
        reason: DeploymentRejectionReason::BuilderUnavailable,
        detail: Some(FailureText::from_text("builder is not reachable").expect("failure text")),
    });
    let observed = Reply::DeploymentObservation(DeploymentObservation {
        phase: DeploymentPhase::DeploymentBuilt(DeploymentBuilt {
            deployment: deployment(),
            result: BuildResult::EvaluatedDerivation(EvaluatedDerivation {
                derivation_path: derivation_path(),
            }),
        }),
    });

    assert_eq!(round_trip_reply(accepted.clone()), accepted);
    assert_eq!(round_trip_reply(rejected.clone()), rejected);
    assert_eq!(round_trip_reply(observed.clone()), observed);
}

#[test]
fn cache_retention_replies_round_trip_through_length_prefixed_frame() {
    let accepted = Reply::CacheRetentionAccepted(CacheRetentionAccepted {
        mutation: mutation(),
    });
    let rejected = Reply::CacheRetentionRejected(CacheRetentionRejected {
        reason: CacheRetentionRejectionReason::PolicyConflict,
        detail: None,
    });
    let observed = Reply::CacheRetentionObservation(CacheRetentionObservation {
        mutation: mutation(),
        generation: generation_id(),
        state: CacheRetentionState::Pinned,
    });

    assert_eq!(round_trip_reply(accepted.clone()), accepted);
    assert_eq!(round_trip_reply(rejected.clone()), rejected);
    assert_eq!(round_trip_reply(observed.clone()), observed);
}

#[test]
fn generation_listing_round_trips_through_length_prefixed_frame() {
    let reply = Reply::GenerationListing(GenerationListing {
        generations: vec![generation()],
    });

    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn sum_records_round_trip_through_nota_text() {
    round_trip_nota(deployment_plan(), "(HomeOnlyDeployment li Activate)");
    round_trip_nota(builder_selection(), "(NamedBuilder ouranos)");
    round_trip_nota(
        CacheRetentionAction::RetireGeneration(RetireGeneration {}),
        "(RetireGeneration)",
    );
    round_trip_nota(
        DeploymentPhase::DeploymentFailed(DeploymentFailed {
            deployment: deployment(),
            reason: FailureText::from_text("activation failed").expect("failure text"),
        }),
        "(DeploymentFailed deploy_aab \"activation failed\")",
    );
}

#[test]
fn validation_newtypes_reject_invalid_boundary_text() {
    assert!(ClusterName::from_text("").is_err());
    assert!(NodeName::from_text("not a node").is_err());
    assert!(FailureText::from_text("two\nlines").is_err());
    assert!(StorePath::from_text("/tmp/not-store").is_err());
    assert!(
        DerivationPath::from_text("/nix/store/00000000000000000000000000000000-criomos").is_err()
    );
}

#[test]
fn contract_crate_has_no_runtime_dependencies() {
    let manifest = include_str!("../Cargo.toml");
    for forbidden in [
        "kameo",
        "tokio",
        "redb",
        "sema",
        "sema-engine",
        "zbus",
        "dbus",
    ] {
        assert!(
            !manifest.contains(forbidden),
            "contract crate must not depend on runtime or storage crate {forbidden}"
        );
    }
}
