use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply as FrameReply, RequestPayload,
    SessionEpoch, StreamEventIdentifier, StreamingFrame, StreamingFrameBody, SubReply,
    SubscriptionTokenInner,
};
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
    FlakeReference::from_text("github:LiGoldragon/CriomOS/horizon-leaner-shape")
        .expect("flake reference")
}

fn wire_path(value: &str) -> WirePath {
    WirePath::from_text(value).expect("wire path")
}

fn operator_identity() -> OperatorIdentity {
    OperatorIdentity::from_text("operator").expect("operator identity")
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

fn deployment_request() -> DeploymentRequest {
    DeploymentRequest {
        cluster: cluster(),
        node: node(),
        source: proposal_source(),
        flake: flake_reference(),
        plan: deployment_plan(),
        builder: builder_selection(),
        substituters: vec![NodeName::from_text("prometheus").expect("node name")],
    }
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

fn deployment_observation() -> DeploymentObservation {
    DeploymentObservation {
        phase: DeploymentPhase::DeploymentBuilt(DeploymentBuilt {
            deployment: deployment(),
            result: BuildResult::EvaluatedDerivation(EvaluatedDerivation {
                derivation_path: derivation_path(),
            }),
        }),
    }
}

fn cache_retention_observation() -> CacheRetentionObservation {
    CacheRetentionObservation {
        mutation: mutation(),
        generation: generation_id(),
        state: CacheRetentionState::Pinned,
    }
}

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn stream_event() -> StreamEventIdentifier {
    StreamEventIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Acceptor,
        LaneSequence::first(),
    )
}

fn round_trip_operation(operation: LojixOperation) -> LojixOperation {
    let request = operation.clone().into_request();
    let frame = LojixFrame::new(StreamingFrameBody::Request {
        exchange: exchange(),
        request,
    });
    let bytes = frame.encode_length_prefixed().expect("encode frame");
    let decoded =
        StreamingFrame::<LojixOperation, LojixReply, LojixEvent>::decode_length_prefixed(&bytes)
            .expect("decode frame");

    match decoded.into_body() {
        StreamingFrameBody::Request { request, .. } => request.payloads().head().clone(),
        other => panic!("expected request frame, got {other:?}"),
    }
}

fn round_trip_reply(reply: LojixReply) -> LojixReply {
    let frame = LojixFrame::new(StreamingFrameBody::Reply {
        exchange: exchange(),
        reply: FrameReply::completed(NonEmpty::single(SubReply::Ok {
            payload: reply.clone(),
        })),
    });
    let bytes = frame.encode_length_prefixed().expect("encode frame");
    let decoded =
        StreamingFrame::<LojixOperation, LojixReply, LojixEvent>::decode_length_prefixed(&bytes)
            .expect("decode frame");

    match decoded.into_body() {
        StreamingFrameBody::Reply { reply, .. } => match reply {
            FrameReply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok { payload, .. } => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            other => panic!("expected accepted reply, got {other:?}"),
        },
        other => panic!("expected reply frame, got {other:?}"),
    }
}

fn round_trip_event(event: LojixEvent) -> LojixEvent {
    let frame = LojixFrame::new(StreamingFrameBody::SubscriptionEvent {
        event_identifier: stream_event(),
        token: SubscriptionTokenInner::new(1),
        event,
    });
    let bytes = frame.encode_length_prefixed().expect("encode frame");
    let decoded =
        StreamingFrame::<LojixOperation, LojixReply, LojixEvent>::decode_length_prefixed(&bytes)
            .expect("decode frame");

    match decoded.into_body() {
        StreamingFrameBody::SubscriptionEvent { event, .. } => event,
        other => panic!("expected subscription event frame, got {other:?}"),
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
fn deploy_operation_round_trips_through_length_prefixed_frame() {
    let operation = LojixOperation::Deploy(deployment_request());

    assert_eq!(round_trip_operation(operation.clone()), operation);
}

#[test]
fn deployment_request_digest_is_stable_over_canonical_bytes() {
    let request = deployment_request();
    let first = request
        .canonical_digest()
        .expect("deployment request digest");
    let second = request
        .canonical_digest()
        .expect("deployment request digest");
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&request).expect("canonical bytes");

    assert_eq!(first, second);
    assert_eq!(first, DeploymentRequestDigest::from_canonical_bytes(&bytes));
    assert_eq!(first.as_str().len(), 64);
}

#[test]
fn deployment_request_digest_changes_when_request_content_changes() {
    let request = deployment_request();
    let changed = DeploymentRequest {
        node: NodeName::from_text("zeus").expect("node name"),
        ..request.clone()
    };

    assert_ne!(
        request
            .canonical_digest()
            .expect("deployment request digest"),
        changed
            .canonical_digest()
            .expect("changed deployment request digest")
    );
}

#[test]
fn cache_retention_operations_round_trip_through_length_prefixed_frame() {
    let pin = LojixOperation::Pin(Pin {
        generation: generation_id(),
    });
    let unpin = LojixOperation::Unpin(Unpin {
        generation: generation_id(),
    });
    let retire = LojixOperation::Retire(Retire {
        generation: generation_id(),
    });

    assert_eq!(round_trip_operation(pin.clone()), pin);
    assert_eq!(round_trip_operation(unpin.clone()), unpin);
    assert_eq!(round_trip_operation(retire.clone()), retire);
}

#[test]
fn query_operation_round_trips_through_length_prefixed_frame() {
    let operation = LojixOperation::Query(GenerationQuery {
        cluster: Some(cluster()),
        node: None,
        kind: Some(GenerationKind::HomeOnly),
    });

    assert_eq!(round_trip_operation(operation.clone()), operation);
}

#[test]
fn watch_and_unwatch_operations_round_trip_through_length_prefixed_frame() {
    let watch_deployments = LojixOperation::WatchDeployments(WatchDeployments {
        cluster: Some(cluster()),
        node: Some(node()),
        deployment: Some(deployment()),
    });
    let watch_cache = LojixOperation::WatchCacheRetention(WatchCacheRetention {
        generation: Some(generation_id()),
    });
    let unwatch_deployments =
        LojixOperation::UnwatchDeployments(DeploymentObservationToken::new(1));
    let unwatch_cache =
        LojixOperation::UnwatchCacheRetention(CacheRetentionObservationToken::new(2));

    assert_eq!(
        round_trip_operation(watch_deployments.clone()),
        watch_deployments
    );
    assert_eq!(round_trip_operation(watch_cache.clone()), watch_cache);
    assert_eq!(
        round_trip_operation(unwatch_deployments.clone()),
        unwatch_deployments
    );
    assert_eq!(round_trip_operation(unwatch_cache.clone()), unwatch_cache);
}

#[test]
fn stream_relation_witnesses_are_generated_by_the_channel_macro() {
    let watch_deployments = LojixOperation::WatchDeployments(WatchDeployments {
        cluster: None,
        node: None,
        deployment: None,
    });
    let watch_cache = LojixOperation::WatchCacheRetention(WatchCacheRetention { generation: None });
    let unwatch_deployments =
        LojixOperation::UnwatchDeployments(DeploymentObservationToken::new(1));
    let unwatch_cache =
        LojixOperation::UnwatchCacheRetention(CacheRetentionObservationToken::new(2));

    assert_eq!(
        watch_deployments.opened_stream(),
        Some(LojixStreamKind::DeploymentObservationStream)
    );
    assert_eq!(
        watch_cache.opened_stream(),
        Some(LojixStreamKind::CacheRetentionObservationStream)
    );
    assert_eq!(
        unwatch_deployments.closed_stream(),
        Some(LojixStreamKind::DeploymentObservationStream)
    );
    assert_eq!(
        unwatch_cache.closed_stream(),
        Some(LojixStreamKind::CacheRetentionObservationStream)
    );

    assert_eq!(
        LojixEvent::DeploymentObservation(deployment_observation()).stream_kind(),
        LojixStreamKind::DeploymentObservationStream
    );
    assert_eq!(
        LojixEvent::CacheRetentionObservation(cache_retention_observation()).stream_kind(),
        LojixStreamKind::CacheRetentionObservationStream
    );
}

#[test]
fn deployment_replies_round_trip_through_length_prefixed_frame() {
    let accepted = LojixReply::DeploymentAccepted(DeploymentAccepted {
        deployment: deployment(),
    });
    let rejected = LojixReply::DeploymentRejected(DeploymentRejected {
        reason: DeploymentRejectionReason::BuilderUnavailable,
        detail: Some(FailureText::from_text("builder is not reachable").expect("failure text")),
    });
    let opened = LojixReply::DeploymentObservationSubscriptionOpened(
        DeploymentObservationSubscriptionOpened {
            token: DeploymentObservationToken::new(1),
            observations: vec![deployment_observation()],
        },
    );
    let closed = LojixReply::DeploymentObservationSubscriptionClosed(
        DeploymentObservationSubscriptionClosed {
            token: DeploymentObservationToken::new(1),
        },
    );

    assert_eq!(round_trip_reply(accepted.clone()), accepted);
    assert_eq!(round_trip_reply(rejected.clone()), rejected);
    assert_eq!(round_trip_reply(opened.clone()), opened);
    assert_eq!(round_trip_reply(closed.clone()), closed);
}

#[test]
fn cache_retention_replies_round_trip_through_length_prefixed_frame() {
    let accepted = LojixReply::CacheRetentionAccepted(CacheRetentionAccepted {
        mutation: mutation(),
    });
    let rejected = LojixReply::CacheRetentionRejected(CacheRetentionRejected {
        reason: CacheRetentionRejectionReason::PolicyConflict,
        detail: None,
    });
    let opened = LojixReply::CacheRetentionObservationSubscriptionOpened(
        CacheRetentionObservationSubscriptionOpened {
            token: CacheRetentionObservationToken::new(2),
            observations: vec![cache_retention_observation()],
        },
    );
    let closed = LojixReply::CacheRetentionObservationSubscriptionClosed(
        CacheRetentionObservationSubscriptionClosed {
            token: CacheRetentionObservationToken::new(2),
        },
    );

    assert_eq!(round_trip_reply(accepted.clone()), accepted);
    assert_eq!(round_trip_reply(rejected.clone()), rejected);
    assert_eq!(round_trip_reply(opened.clone()), opened);
    assert_eq!(round_trip_reply(closed.clone()), closed);
}

#[test]
fn observation_events_round_trip_through_subscription_event_frame() {
    let deployment_event = LojixEvent::DeploymentObservation(deployment_observation());
    let cache_event = LojixEvent::CacheRetentionObservation(cache_retention_observation());

    assert_eq!(round_trip_event(deployment_event.clone()), deployment_event);
    assert_eq!(round_trip_event(cache_event.clone()), cache_event);
}

#[test]
fn generation_listing_round_trips_through_length_prefixed_frame() {
    let reply = LojixReply::GenerationListing(GenerationListing {
        generations: vec![generation()],
    });

    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn sum_records_round_trip_through_nota_text() {
    round_trip_nota(deployment_plan(), "(HomeOnlyDeployment (li Activate))");
    round_trip_nota(builder_selection(), "(NamedBuilder (ouranos))");
    round_trip_nota(
        DeploymentPhase::DeploymentFailed(DeploymentFailed {
            deployment: deployment(),
            reason: FailureText::from_text("activation failed").expect("failure text"),
        }),
        "(DeploymentFailed (deploy_aab \"activation failed\"))",
    );
}

#[test]
fn watch_operation_round_trips_through_nota_text() {
    round_trip_nota(
        LojixOperation::WatchDeployments(WatchDeployments {
            cluster: Some(cluster()),
            node: Some(node()),
            deployment: None,
        }),
        "(WatchDeployments ((Some goldragon) (Some ouranos) None))",
    );
    round_trip_nota(
        LojixReply::DeploymentObservationSubscriptionOpened(
            DeploymentObservationSubscriptionOpened {
                token: DeploymentObservationToken::new(1),
                observations: vec![deployment_observation()],
            },
        ),
        "(DeploymentObservationSubscriptionOpened ((1) [((DeploymentBuilt (deploy_aab (EvaluatedDerivation (\"/nix/store/00000000000000000000000000000000-criomos.drv\")))))]))",
    );
}

#[test]
fn daemon_configuration_round_trips_through_nota_text() {
    round_trip_nota(
        LojixDaemonConfiguration {
            daemon_socket_path: wire_path("/tmp/lojix-daemon.sock"),
            daemon_socket_mode: SocketMode::new(0o660),
            daemon_socket_group: None,
            horizon_configuration_source: wire_path("/tmp/horizon.nota"),
            state_directory: wire_path("/tmp/lojix-state"),
            gc_root_directory: wire_path("/tmp/lojix-gcroots"),
            peer_daemons: Vec::new(),
            operator_identity: operator_identity(),
            owned_cluster: cluster(),
        },
        "(\"/tmp/lojix-daemon.sock\" 432 None \"/tmp/horizon.nota\" \"/tmp/lojix-state\" \"/tmp/lojix-gcroots\" [] operator goldragon)",
    );
}

#[test]
fn cli_configuration_round_trips_through_nota_text() {
    round_trip_nota(
        LojixCliConfiguration {
            daemon_socket_path: wire_path("/tmp/lojix-daemon.sock"),
            reply_rendering: ReplyRendering::Compact,
        },
        "(\"/tmp/lojix-daemon.sock\" Compact)",
    );
}

#[test]
fn daemon_configuration_decodes_from_rkyv_bytes() {
    let configuration = LojixDaemonConfiguration {
        daemon_socket_path: wire_path("/tmp/lojix-daemon.sock"),
        daemon_socket_mode: SocketMode::new(0o600),
        daemon_socket_group: None,
        horizon_configuration_source: wire_path("/tmp/horizon.nota"),
        state_directory: wire_path("/tmp/lojix-state"),
        gc_root_directory: wire_path("/tmp/lojix-gcroots"),
        peer_daemons: Vec::new(),
        operator_identity: operator_identity(),
        owned_cluster: cluster(),
    };
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&configuration).expect("rkyv encode");
    let decoded =
        <LojixDaemonConfiguration as nota_config::ConfigurationRecord>::from_rkyv_bytes(&bytes)
            .expect("rkyv decode");
    assert_eq!(decoded, configuration);
}

#[test]
fn cli_configuration_rejects_rkyv_bytes() {
    let err =
        <LojixCliConfiguration as nota_config::ConfigurationRecord>::from_rkyv_bytes(b"not-rkyv")
            .expect_err("cli configuration is nota-only");
    assert!(matches!(err, nota_config::Error::RkyvNotSupported(_)));
}

#[test]
fn validation_newtypes_reject_invalid_boundary_text() {
    assert!(ClusterName::from_text("").is_err());
    assert!(NodeName::from_text("not a node").is_err());
    assert!(FailureText::from_text("two\nlines").is_err());
    assert!(StorePath::from_text("/tmp/not-store").is_err());
    assert!(WirePath::from_text("two\nlines").is_err());
    assert!(UnixGroup::from_text("not a group").is_err());
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
