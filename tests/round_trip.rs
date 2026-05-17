use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use signal_core::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply as CoreReply, RequestPayload,
    SessionEpoch, SignalVerb, StreamEventIdentifier, SubReply, SubscriptionTokenInner,
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
    FlakeReference::from_text("github:LiGoldragon/CriomOS/horizon-re-engineering")
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

fn round_trip_request(request: Request) -> Request {
    let expected_verb = request.signal_verb();
    let frame = LojixFrame::new(LojixFrameBody::Request {
        exchange: exchange(),
        request: request.into_request(),
    });
    let bytes = frame.encode_length_prefixed().expect("encode frame");
    let decoded = LojixFrame::decode_length_prefixed(&bytes).expect("decode frame");

    match decoded.into_body() {
        LojixFrameBody::Request { request, .. } => {
            let operation = request.operations().head();
            assert_eq!(operation.verb, expected_verb);
            operation.payload.clone()
        }
        other => panic!("expected request frame, got {other:?}"),
    }
}

fn round_trip_reply(reply: Reply) -> Reply {
    let frame = LojixFrame::new(LojixFrameBody::Reply {
        exchange: exchange(),
        reply: CoreReply::completed(NonEmpty::single(SubReply::Ok {
            verb: SignalVerb::Assert,
            payload: reply.clone(),
        })),
    });
    let bytes = frame.encode_length_prefixed().expect("encode frame");
    let decoded = LojixFrame::decode_length_prefixed(&bytes).expect("decode frame");

    match decoded.into_body() {
        LojixFrameBody::Reply { reply, .. } => match reply {
            CoreReply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok { payload, .. } => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            other => panic!("expected accepted reply, got {other:?}"),
        },
        other => panic!("expected reply frame, got {other:?}"),
    }
}

fn round_trip_event(event: Event) -> Event {
    let frame = LojixFrame::new(LojixFrameBody::SubscriptionEvent {
        event_identifier: stream_event(),
        token: SubscriptionTokenInner::new(1),
        event,
    });
    let bytes = frame.encode_length_prefixed().expect("encode frame");
    let decoded = LojixFrame::decode_length_prefixed(&bytes).expect("decode frame");

    match decoded.into_body() {
        LojixFrameBody::SubscriptionEvent { event, .. } => event,
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
fn observation_subscription_requests_round_trip_through_length_prefixed_frame() {
    let deployment_subscription =
        Request::DeploymentObservationSubscription(DeploymentObservationSubscription {
            cluster: Some(cluster()),
            node: Some(node()),
            deployment: Some(deployment()),
        });
    let cache_subscription =
        Request::CacheRetentionObservationSubscription(CacheRetentionObservationSubscription {
            generation: Some(generation_id()),
        });
    let deployment_retraction =
        Request::DeploymentObservationRetraction(DeploymentObservationToken::new(1));
    let cache_retraction =
        Request::CacheRetentionObservationRetraction(CacheRetentionObservationToken::new(2));

    assert_eq!(
        round_trip_request(deployment_subscription.clone()),
        deployment_subscription
    );
    assert_eq!(
        round_trip_request(cache_subscription.clone()),
        cache_subscription
    );
    assert_eq!(
        round_trip_request(deployment_retraction.clone()),
        deployment_retraction
    );
    assert_eq!(
        round_trip_request(cache_retraction.clone()),
        cache_retraction
    );
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
    let deployment_subscription =
        Request::DeploymentObservationSubscription(DeploymentObservationSubscription {
            cluster: None,
            node: None,
            deployment: None,
        });
    let cache_subscription =
        Request::CacheRetentionObservationSubscription(CacheRetentionObservationSubscription {
            generation: None,
        });
    let deployment_retraction =
        Request::DeploymentObservationRetraction(DeploymentObservationToken::new(1));
    let cache_retraction =
        Request::CacheRetentionObservationRetraction(CacheRetentionObservationToken::new(2));

    assert_eq!(
        deployment_request.signal_verb(),
        signal_core::SignalVerb::Assert
    );
    assert_eq!(cache_request.signal_verb(), signal_core::SignalVerb::Mutate);
    assert_eq!(query_request.signal_verb(), signal_core::SignalVerb::Match);
    assert_eq!(
        deployment_subscription.signal_verb(),
        signal_core::SignalVerb::Subscribe
    );
    assert_eq!(
        cache_subscription.signal_verb(),
        signal_core::SignalVerb::Subscribe
    );
    assert_eq!(
        deployment_retraction.signal_verb(),
        signal_core::SignalVerb::Retract
    );
    assert_eq!(
        cache_retraction.signal_verb(),
        signal_core::SignalVerb::Retract
    );
}

#[test]
fn stream_relation_witnesses_are_generated_by_the_channel_macro() {
    let deployment_subscription =
        Request::DeploymentObservationSubscription(DeploymentObservationSubscription {
            cluster: None,
            node: None,
            deployment: None,
        });
    let cache_subscription =
        Request::CacheRetentionObservationSubscription(CacheRetentionObservationSubscription {
            generation: None,
        });
    let deployment_retraction =
        Request::DeploymentObservationRetraction(DeploymentObservationToken::new(1));
    let cache_retraction =
        Request::CacheRetentionObservationRetraction(CacheRetentionObservationToken::new(2));

    assert_eq!(
        deployment_subscription.opened_stream(),
        Some(LojixStreamKind::DeploymentObservationStream)
    );
    assert_eq!(
        cache_subscription.opened_stream(),
        Some(LojixStreamKind::CacheRetentionObservationStream)
    );
    assert_eq!(
        deployment_retraction.closed_stream(),
        Some(LojixStreamKind::DeploymentObservationStream)
    );
    assert_eq!(
        cache_retraction.closed_stream(),
        Some(LojixStreamKind::CacheRetentionObservationStream)
    );

    assert_eq!(
        Event::DeploymentObservation(deployment_observation()).stream_kind(),
        LojixStreamKind::DeploymentObservationStream
    );
    assert_eq!(
        Event::CacheRetentionObservation(cache_retention_observation()).stream_kind(),
        LojixStreamKind::CacheRetentionObservationStream
    );
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
    let opened =
        Reply::DeploymentObservationSubscriptionOpened(DeploymentObservationSubscriptionOpened {
            token: DeploymentObservationToken::new(1),
            observations: vec![deployment_observation()],
        });
    let closed =
        Reply::DeploymentObservationSubscriptionClosed(DeploymentObservationSubscriptionClosed {
            token: DeploymentObservationToken::new(1),
        });

    assert_eq!(round_trip_reply(accepted.clone()), accepted);
    assert_eq!(round_trip_reply(rejected.clone()), rejected);
    assert_eq!(round_trip_reply(opened.clone()), opened);
    assert_eq!(round_trip_reply(closed.clone()), closed);
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
    let opened = Reply::CacheRetentionObservationSubscriptionOpened(
        CacheRetentionObservationSubscriptionOpened {
            token: CacheRetentionObservationToken::new(2),
            observations: vec![cache_retention_observation()],
        },
    );
    let closed = Reply::CacheRetentionObservationSubscriptionClosed(
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
    let deployment_event = Event::DeploymentObservation(deployment_observation());
    let cache_event = Event::CacheRetentionObservation(cache_retention_observation());

    assert_eq!(round_trip_event(deployment_event.clone()), deployment_event);
    assert_eq!(round_trip_event(cache_event.clone()), cache_event);
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
fn subscription_records_round_trip_through_nota_text() {
    round_trip_nota(
        Request::DeploymentObservationSubscription(DeploymentObservationSubscription {
            cluster: Some(cluster()),
            node: Some(node()),
            deployment: None,
        }),
        "(DeploymentObservationSubscription goldragon ouranos None)",
    );
    round_trip_nota(
        Reply::DeploymentObservationSubscriptionOpened(DeploymentObservationSubscriptionOpened {
            token: DeploymentObservationToken::new(1),
            observations: vec![deployment_observation()],
        }),
        "(DeploymentObservationSubscriptionOpened (DeploymentObservationToken 1) [(DeploymentObservation (DeploymentBuilt deploy_aab (EvaluatedDerivation \"/nix/store/00000000000000000000000000000000-criomos.drv\")))])",
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
        "(LojixDaemonConfiguration \"/tmp/lojix-daemon.sock\" 432 None \"/tmp/horizon.nota\" \"/tmp/lojix-state\" \"/tmp/lojix-gcroots\" [] operator goldragon)",
    );
}

#[test]
fn cli_configuration_round_trips_through_nota_text() {
    round_trip_nota(
        LojixCliConfiguration {
            daemon_socket_path: wire_path("/tmp/lojix-daemon.sock"),
            reply_rendering: ReplyRendering::Compact,
        },
        "(LojixCliConfiguration \"/tmp/lojix-daemon.sock\" Compact)",
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
