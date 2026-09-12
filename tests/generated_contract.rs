use signal::{ByteViewable, Restorable, Signal, Signalizable};
use signal_lojix::{
    ConfigurationReceipt, LojixNexusConfiguration, NodeSelector, Query, Response, Selection,
    TestDefaultsChoice,
};

fn node_selection() -> Query {
    Query::Query(Selection::ByNode(NodeSelector {
        cluster_name: "production.eu".into(),
        node_name: ".state/cache".into(),
        requested_generation_artifact_option: None,
    }))
}

fn configuration() -> LojixNexusConfiguration {
    LojixNexusConfiguration {
        ordinary_socket_path: "/run/lojix/ordinary.sock".into(),
        ordinary_socket_mode: 0o660,
        owner_socket_path: "/run/lojix/meta.sock".into(),
        owner_socket_mode: 0o600,
        state_directory_path: "/var/lib/lojix".into(),
        daemon_host: "deployment.host".into(),
        test_defaults_choice: TestDefaultsChoice::NoTestDefaults,
    }
}

#[test]
fn peer_bytes_restore_typed_query_and_response() {
    let query = node_selection();
    let sent = query.signalize().expect("signalize query");
    assert!(!sent.bytes().is_empty());
    let received = Signal::<Query>::from(sent.bytes().to_vec());
    assert_eq!(received.restore().expect("restore query"), query);

    let response = failed_deployment_response();
    let sent = response.signalize().expect("signalize response");
    let received = Signal::<Response>::from(sent.bytes().to_vec());
    assert_eq!(received.restore().expect("restore response"), response);
}

#[test]
fn configure_query_and_desired_state_receipt_cross_fresh_peer_bytes() {
    let query = Query::Configure(configuration());
    let sent = query.signalize().expect("signalize Configure query");
    let received = Signal::<Query>::from(sent.bytes().to_vec());
    assert_eq!(received.restore().expect("restore Configure query"), query);

    let response = Response::Configured(ConfigurationReceipt {
        lojix_nexus_configuration: configuration(),
        meta_configure_occurred: false,
    });
    let sent = response.signalize().expect("signalize receipt");
    let received = Signal::<Response>::from(sent.bytes().to_vec());
    assert_eq!(received.restore().expect("restore receipt"), response);
}

#[cfg(feature = "datom")]
#[test]
fn datom_round_trip_preserves_bare_punctuation_strings() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    let query = node_selection();
    let rendered = query.clone().datomize(vec![]).protosize().textualize();
    assert!(rendered.contains("production.eu"));
    assert!(rendered.contains(".state/cache"));

    let mut pending = Potential::<Query>::from(rendered);
    let restored = pending
        .actualize(&mut Budget {
            remaining: 4_096,
            reader: ReaderBudget { remaining: 4_096 },
            depth: 0,
            maximum_depth: 256,
        })
        .expect("restore datom query");
    assert_eq!(restored, query);
}

#[cfg(feature = "datom")]
#[test]
fn datom_round_trip_preserves_named_response_payload() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    let response = failed_deployment_response();
    let rendered = response.clone().datomize(vec![]).protosize().textualize();
    let mut pending = Potential::<Response>::from(rendered);
    let restored = pending
        .actualize(&mut Budget {
            remaining: 4_096,
            reader: ReaderBudget { remaining: 4_096 },
            depth: 0,
            maximum_depth: 256,
        })
        .expect("restore datom response");
    assert_eq!(restored, response);
}

/// A terminal deployment record whose failure carries the evidence a retry
/// needs: the command that failed, its exit code, and the bounded detail it
/// printed. This is the falsifiable specification of the failure surface.
fn failed_deployment_response() -> Response {
    use signal_lojix::{
        ActivationEffect, DatabaseMarker, DeploymentEnvironment, DeploymentFailure,
        DeploymentFailureStage, DeploymentLifecycle, DeploymentRecord, DeploymentRequestIdentity,
        DeploymentTerminal, DeploymentTerminalReason, FailedCommand, FailureEvidence,
        GenerationArtifact, GenerationListing, RequestedDeploymentAction, SourceRevisionPolicy,
        UserEnvironmentAction,
    };
    let marker = DatabaseMarker {
        commit_sequence: 9,
        state_digest: 17,
    };
    Response::Queried(GenerationListing {
        generation_vector: vec![],
        deployment_record_vector: vec![DeploymentRecord {
            deployment_identifier: 190,
            generation_identifier: 41,
            deployment_request_identity: DeploymentRequestIdentity {
                deployment_environment: DeploymentEnvironment::UserEnvironment("li".into()),
                cluster_name: "goldragon".into(),
                node_name: "ouranos".into(),
                generation_artifact: GenerationArtifact::UserEnvironment,
                requested_deployment_action: RequestedDeploymentAction::UserEnvironment(
                    UserEnvironmentAction::ActivateNow,
                ),
                activation_effect: ActivationEffect::LiveActivation,
                source_revision_policy: SourceRevisionPolicy::RequireImmutable,
                immutable_revision_option: None,
            },
            admission_marker_option: Some(marker.clone()),
            deployment_lifecycle: DeploymentLifecycle::Failed,
            terminal_marker_option: Some(marker.clone()),
            deployment_terminal_option: Some(DeploymentTerminal::Failed(DeploymentFailure {
                deployment_failure_stage: DeploymentFailureStage::Activate,
                deployment_terminal_reason: DeploymentTerminalReason::ActivationFailed,
                failure_evidence_option: Some(FailureEvidence {
                    failed_command_option: Some(FailedCommand {
                        command_program: "ssh".into(),
                        command_argument_vector: vec![
                            "li@ouranos".into(),
                            "nix-env --profile /nix/var/nix/profiles/per-user/li/home-manager --set /nix/store/x-home".into(),
                        ],
                        exit_code_option: Some(1),
                    }),
                    failure_detail: "Activating vscodium: managed extensions inconsistent".into(),
                    detail_truncated: false,
                }),
            })),
        }],
        database_marker: marker,
    })
}
