use signal::{ByteViewable, Restorable, Signal, Signalizable};
use signal_lojix::{
    ConfigurationReceipt, Input, LojixNexusConfiguration, NodeSelector, Output, Selection,
    TestDefaultsChoice,
};

fn node_selection() -> Input {
    Input::Query(Selection::ByNode(NodeSelector {
        cluster_name: signal_lojix::ClusterName("production.eu".into()),
        node_name: signal_lojix::NodeName(".state/cache".into()),
        generation_artifact_option: None,
    }))
}

fn configuration() -> LojixNexusConfiguration {
    LojixNexusConfiguration {
        ordinary_socket_path: signal_lojix::OrdinarySocketPath("/run/lojix/ordinary.sock".into()),
        ordinary_socket_mode: signal_lojix::OrdinarySocketMode(0o660),
        owner_socket_path: signal_lojix::OwnerSocketPath("/run/lojix/meta.sock".into()),
        owner_socket_mode: signal_lojix::OwnerSocketMode(0o600),
        state_directory_path: signal_lojix::StateDirectoryPath("/var/lib/lojix".into()),
        daemon_host: signal_lojix::DaemonHost("deployment.host".into()),
        test_defaults_choice: TestDefaultsChoice::NoTestDefaults,
    }
}

#[cfg(feature = "datom")]
fn gold_horizon_definition() -> horizon_lib::HorizonDefinition {
    use datom_codec::Decimal;

    horizon_lib::HorizonDefinition {
        horizon_configuration: horizon_lib::HorizonConfiguration {
            generic_nodes: horizon_lib::GenericNodes(vec![horizon_lib::NodeDefinition {
                node_name: horizon_lib::NodeName("opencode-test".into()),
                node_variant: horizon_lib::NodeVariant::Live(horizon_lib::LiveDefinition {}),
                first_magnitude: horizon_lib::Magnitude::Min,
                second_magnitude: horizon_lib::Magnitude::Min,
                machine_definition: horizon_lib::MachineDefinition::Metal(
                    horizon_lib::Metal_Data {
                        architecture: horizon_lib::Architecture::X86_64,
                        hardware: horizon_lib::Hardware {
                            integer: 1,
                            model_name_option: None,
                            mother_board_option: None,
                            first_integer_option: None,
                            second_integer_option: None,
                            location_option: None,
                        },
                    },
                ),
                node_environment: horizon_lib::NodeEnvironment {
                    keyboard: horizon_lib::Keyboard::Qwerty,
                    compressed_swap_option: None,
                },
                node_network: horizon_lib::NodeNetwork {
                    link_local_ip_vector: vec![],
                    node_ip_option: None,
                    wireguard_pub_key_option: None,
                    wireguard_proxy_vector: vec![],
                    router_interfaces_option: Some(horizon_lib::RouterInterfaces {
                        first_interface: horizon_lib::Interface("enp1s0".into()),
                        second_interface: horizon_lib::Interface("wlp2s0".into()),
                        wlan_band: horizon_lib::WlanBand::FiveG,
                        integer: 36,
                        wlan_standard: horizon_lib::WlanStandard::Wifi6,
                        secret_reference_option: Some(horizon_lib::SecretReference {
                            secret_name: horizon_lib::SecretName("router-wifi-password".into()),
                        }),
                        backup_wireless_option: None,
                        country_code: horizon_lib::CountryCode("MX".into()),
                    }),
                },
                node_keys: horizon_lib::NodeKeys {
                    ssh_pub_key: horizon_lib::SshPubKey("ssh-ed25519 AAAAfixture".into()),
                    nix_pub_key_option: None,
                    yggdrasil_key_option: None,
                },
                boolean_option: None,
                capabilities: horizon_lib::Capabilities(vec![
                    horizon_lib::NodeCapability::OpenCodeTesting(horizon_lib::UserName(
                        "li".into(),
                    )),
                    horizon_lib::NodeCapability::Router(horizon_lib::NoSettings {}),
                    horizon_lib::NodeCapability::TailnetController(
                        horizon_lib::TailnetController_Data {
                            certificate_authority_option: Some(horizon_lib::CertificateAuthority(
                                "-----BEGIN CERTIFICATE-----fixture".into(),
                            )),
                            tls_certificate_reference: horizon_lib::TlsCertificateReference {
                                secret_name: horizon_lib::SecretName(
                                    "headscale-tls-certificate".into(),
                                ),
                            },
                            tls_key_reference: horizon_lib::TlsKeyReference {
                                secret_name: horizon_lib::SecretName("headscale-tls-key".into()),
                            },
                        },
                    ),
                    horizon_lib::NodeCapability::TailnetClient(horizon_lib::SecretReference {
                        secret_name: horizon_lib::SecretName("tailnet-preauth-key".into()),
                    }),
                    horizon_lib::NodeCapability::UsbDownlink(horizon_lib::UsbDownlink_Data {
                        ipv4_cidr: horizon_lib::Ipv4Cidr("10.47.0.1/24".into()),
                    }),
                ]),
                fixed_location_option: Some(horizon_lib::FixedLocation {
                    first_decimal: Decimal::try_from(19.4326).expect("finite latitude"),
                    second_decimal: Decimal::try_from(-99.1332).expect("finite longitude"),
                    third_decimal: Decimal::try_from(2_240.0).expect("finite altitude"),
                    fourth_decimal: Decimal::try_from(3.5).expect("finite accuracy"),
                }),
            }]),
            domain_configuration: horizon_lib::DomainConfiguration {
                string: "internal.invalid".into(),
                domain_name_vector: vec![],
            },
        },
        cluster_definition: horizon_lib::ClusterDefinition {
            cluster_name: horizon_lib::ClusterName("production.eu".into()),
            cluster_nodes: horizon_lib::ClusterNodes(vec![]),
            generic_node_names: horizon_lib::GenericNodeNames(vec![horizon_lib::NodeName(
                "opencode-test".into(),
            )]),
            users: horizon_lib::Users(vec![]),
            domains: horizon_lib::Domains(vec![]),
            cluster_trust: horizon_lib::ClusterTrust {
                magnitude: horizon_lib::Magnitude::Zero,
                cluster_trust_entry_vector: vec![],
                node_trust_entry_vector: vec![],
                user_trust_entry_vector: vec![],
            },
        },
    }
}

#[test]
fn peer_bytes_restore_typed_input_and_output() {
    let query = node_selection();
    let sent = query.signalize().expect("signalize query");
    assert!(!sent.bytes().is_empty());
    let received = Signal::<Input>::from(sent.bytes().to_vec());
    assert_eq!(received.restore().expect("restore query"), query);

    let response = failed_deployment_response();
    let sent = response.signalize().expect("signalize response");
    let received = Signal::<Output>::from(sent.bytes().to_vec());
    assert_eq!(received.restore().expect("restore response"), response);
}

#[test]
fn configure_query_and_desired_state_receipt_cross_fresh_peer_bytes() {
    let query = Input::Configure(configuration());
    let sent = query.signalize().expect("signalize Configure query");
    let received = Signal::<Input>::from(sent.bytes().to_vec());
    assert_eq!(received.restore().expect("restore Configure query"), query);

    let response = Output::Configured(ConfigurationReceipt {
        lojix_nexus_configuration: configuration(),
        meta_configure_occurred: signal_lojix::MetaConfigureOccurred(false),
    });
    let sent = response.signalize().expect("signalize receipt");
    let received = Signal::<Output>::from(sent.bytes().to_vec());
    assert_eq!(received.restore().expect("restore receipt"), response);
}

#[cfg(feature = "datom")]
fn assert_horizon_definition_crosses_archive_and_datom(definition: horizon_lib::HorizonDefinition) {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    let configuration = LojixNexusConfiguration {
        test_defaults_choice: TestDefaultsChoice::TestDefaults(signal_lojix::TestDefaults {
            cluster_name: signal_lojix::ClusterName("production.eu".into()),
            node_name: signal_lojix::NodeName("opencode-test".into()),
            test_mode: signal_lojix::TestMode::Hermetic,
            flake_reference: signal_lojix::FlakeReference("github:LiGoldragon/CriomOS".into()),
            nix_system: signal_lojix::NixSystem("x86_64-linux".into()),
            deployment_output_selector: signal_lojix::DeploymentOutputSelector {
                flake_attribute: signal_lojix::FlakeAttribute(
                    "checks.x86_64-linux.contract".into(),
                ),
            },
            horizon_definition_option: Some(definition),
        }),
        ..configuration()
    };
    let sent = configuration
        .signalize()
        .expect("signalize Gold definition");
    let received = Signal::<LojixNexusConfiguration>::from(sent.bytes().to_vec());
    assert_eq!(
        received.restore().expect("restore Gold definition"),
        configuration
    );
    let rendered = configuration
        .clone()
        .datomize(vec![])
        .protosize()
        .textualize();
    let restored = Potential::<LojixNexusConfiguration>::from(rendered)
        .actualize(&mut Budget {
            remaining: 4_096,
            reader: ReaderBudget { remaining: 4_096 },
            depth: 0,
            maximum_depth: 256,
        })
        .expect("restore Gold definition from Datom");
    assert_eq!(restored, configuration);
}

#[cfg(feature = "datom")]
#[test]
fn peer_bytes_restore_gold_opencode_testing_horizon_definition() {
    assert_horizon_definition_crosses_archive_and_datom(gold_horizon_definition());
}

#[cfg(feature = "datom")]
#[test]
fn a_user_key_carrying_agent_stack_and_stable_codex_crosses_archive_and_datom() {
    let mut definition = gold_horizon_definition();
    definition.cluster_definition.users = horizon_lib::Users(vec![horizon_lib::UserDefinition {
        user_name: horizon_lib::UserName("li".into()),
        user_role: horizon_lib::UserRole::Code,
        magnitude: horizon_lib::Magnitude::Max,
        keyboard: horizon_lib::Keyboard::Colemak,
        style: horizon_lib::Style::Emacs,
        github_id_option: Some(horizon_lib::GithubId("li".into())),
        boolean_option: Some(true),
        user_pub_key_vector: vec![horizon_lib::UserPubKey {
            node_name: horizon_lib::NodeName("opencode-test".into()),
            ssh_pub_key: horizon_lib::SshPubKey("AAAAuser".into()),
            keygrip: horizon_lib::Keygrip("keygrip".into()),
            user_capability_vector: vec![
                horizon_lib::UserCapability::AgentStack,
                horizon_lib::UserCapability::StableCodex,
            ],
        }],
        editor_option: Some(horizon_lib::Editor::Emacs),
        text_size_option: Some(horizon_lib::TextSize::Large),
    }]);
    assert_horizon_definition_crosses_archive_and_datom(definition);
}

#[cfg(feature = "datom")]
#[test]
fn a_git_server_capability_crosses_archive_and_datom() {
    let mut definition = gold_horizon_definition();
    definition.horizon_configuration.generic_nodes.0[0]
        .capabilities
        .0
        .push(horizon_lib::NodeCapability::GitServer(
            horizon_lib::NoSettings {},
        ));
    assert_horizon_definition_crosses_archive_and_datom(definition);
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

    let mut pending = Potential::<Input>::from(rendered);
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
    let mut pending = Potential::<Output>::from(rendered);
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
fn failed_deployment_response() -> Output {
    use signal_lojix::{
        ActivationEffect, AdmissionMarker, DatabaseMarker, DeploymentEnvironment,
        DeploymentFailure, DeploymentFailureStage, DeploymentPhase, DeploymentRecord,
        DeploymentRequestIdentity, DeploymentTerminal, DeploymentTerminalReason, FailedCommand,
        FailureEvidence, GenerationArtifact, GenerationListing, RequestedDeploymentAction,
        SourceRevisionPolicy, TerminalMarker, UserEnvironmentAction,
    };
    let marker = DatabaseMarker {
        commit_sequence: signal_lojix::CommitSequence(9),
        state_digest: signal_lojix::StateDigest(17),
    };
    Output::Queried(GenerationListing {
        generation_vector: vec![],
        deployment_record_vector: vec![DeploymentRecord {
            deployment_identifier: signal_lojix::DeploymentIdentifier(190),
            generation_identifier: signal_lojix::GenerationIdentifier(41),
            deployment_request_identity: DeploymentRequestIdentity {
                deployment_environment: DeploymentEnvironment::UserEnvironment(signal_lojix::UserName("li".into())),
                cluster_name: signal_lojix::ClusterName("goldragon".into()),
                node_name: signal_lojix::NodeName("ouranos".into()),
                generation_artifact: GenerationArtifact::UserEnvironment,
                requested_deployment_action: RequestedDeploymentAction::UserEnvironment(
                    UserEnvironmentAction::ActivateNow,
                ),
                activation_effect: ActivationEffect::LiveActivation,
                source_revision_policy: SourceRevisionPolicy::RequireImmutable,
                immutable_revision_option: None,
            },
            admission_marker_option: Some(AdmissionMarker {
                database_marker: marker.clone(),
            }),
            deployment_phase: DeploymentPhase::Failed,
            terminal_marker_option: Some(TerminalMarker {
                database_marker: marker.clone(),
            }),
            deployment_terminal_option: Some(DeploymentTerminal::Failed(DeploymentFailure {
                deployment_failure_stage: DeploymentFailureStage::Activate,
                deployment_terminal_reason: DeploymentTerminalReason::ActivationFailed,
                failure_evidence_option: Some(FailureEvidence {
                    failed_command_option: Some(FailedCommand {
                        command_program: signal_lojix::CommandProgram("ssh".into()),
                        command_argument_vector: vec![
                            signal_lojix::CommandArgument("li@ouranos".into()),
                            signal_lojix::CommandArgument("nix-env --profile /nix/var/nix/profiles/per-user/li/home-manager --set /nix/store/x-home".into()),
                        ],
                        exit_code_option: Some(signal_lojix::ExitCode(1)),
                    }),
                    failure_detail: signal_lojix::FailureDetail("Activating vscodium: managed extensions inconsistent".into()),
                    detail_truncated: signal_lojix::DetailTruncated(false),
                }),
            })),
        }],
        database_marker: marker,
    })
}

/// A closure copy that failed. `nix copy` engages no builder, so the copy
/// stage names itself rather than borrowing `BuilderUnreachable`. This example
/// is the falsifiable specification of that reason's place on the wire.
fn failed_closure_copy_response() -> Output {
    use signal_lojix::{
        ActivationEffect, AdmissionMarker, DatabaseMarker, DeploymentEnvironment,
        DeploymentFailure, DeploymentFailureStage, DeploymentPhase, DeploymentRecord,
        DeploymentRequestIdentity, DeploymentTerminal, DeploymentTerminalReason, FailedCommand,
        FailureEvidence, GenerationArtifact, GenerationListing, HostDeployAction,
        RequestedDeploymentAction, SourceRevisionPolicy, TerminalMarker,
    };
    let marker = DatabaseMarker {
        commit_sequence: signal_lojix::CommitSequence(31),
        state_digest: signal_lojix::StateDigest(44),
    };
    Output::Queried(GenerationListing {
        generation_vector: vec![],
        deployment_record_vector: vec![DeploymentRecord {
            deployment_identifier: signal_lojix::DeploymentIdentifier(212),
            generation_identifier: signal_lojix::GenerationIdentifier(57),
            deployment_request_identity: DeploymentRequestIdentity {
                deployment_environment: DeploymentEnvironment::HostEnvironment,
                cluster_name: signal_lojix::ClusterName("goldragon".into()),
                node_name: signal_lojix::NodeName("ouranos".into()),
                generation_artifact: GenerationArtifact::CompleteHost,
                requested_deployment_action: RequestedDeploymentAction::Host(
                    HostDeployAction::ActivateNow,
                ),
                activation_effect: ActivationEffect::LiveActivation,
                source_revision_policy: SourceRevisionPolicy::RequireImmutable,
                immutable_revision_option: None,
            },
            admission_marker_option: Some(AdmissionMarker {
                database_marker: marker.clone(),
            }),
            deployment_phase: DeploymentPhase::Failed,
            terminal_marker_option: Some(TerminalMarker {
                database_marker: marker.clone(),
            }),
            deployment_terminal_option: Some(DeploymentTerminal::Failed(DeploymentFailure {
                deployment_failure_stage: DeploymentFailureStage::CopyClosure,
                deployment_terminal_reason: DeploymentTerminalReason::ClosureCopyFailed,
                failure_evidence_option: Some(FailureEvidence {
                    failed_command_option: Some(FailedCommand {
                        command_program: signal_lojix::CommandProgram("nix".into()),
                        command_argument_vector: vec![
                            signal_lojix::CommandArgument("copy".into()),
                            signal_lojix::CommandArgument("--substitute-on-destination".into()),
                            signal_lojix::CommandArgument("--to".into()),
                            signal_lojix::CommandArgument("ssh-ng://root@ouranos".into()),
                            signal_lojix::CommandArgument("/nix/store/x-nixos-system".into()),
                        ],
                        exit_code_option: Some(signal_lojix::ExitCode(1)),
                    }),
                    failure_detail: signal_lojix::FailureDetail(
                        "error: cannot open connection to remote store".into(),
                    ),
                    detail_truncated: signal_lojix::DetailTruncated(false),
                }),
            })),
        }],
        database_marker: marker,
    })
}

#[test]
fn a_failed_closure_copy_crosses_peer_bytes_naming_its_own_stage() {
    let response = failed_closure_copy_response();
    let sent = response.signalize().expect("signalize copy failure");
    let received = Signal::<Output>::from(sent.bytes().to_vec());
    assert_eq!(received.restore().expect("restore copy failure"), response);
}

#[cfg(feature = "datom")]
#[test]
fn a_failed_closure_copy_renders_its_own_reason_in_datom() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    let response = failed_closure_copy_response();
    let rendered = response.clone().datomize(vec![]).protosize().textualize();
    assert!(
        rendered.contains("ClosureCopyFailed"),
        "a copy failure names the copy stage, not a builder: {rendered}"
    );
    assert!(
        !rendered.contains("BuilderUnreachable"),
        "no builder is engaged by a copy: {rendered}"
    );
    let mut pending = Potential::<Output>::from(rendered);
    let restored = pending
        .actualize(&mut Budget {
            remaining: 4_096,
            reader: ReaderBudget { remaining: 4_096 },
            depth: 0,
            maximum_depth: 256,
        })
        .expect("restore datom copy failure");
    assert_eq!(restored, response);
}

/// One committed phase event as a deployment watch writes it: the terminal
/// event of a deployment that succeeded, carrying its revision and terminal.
fn observed_phase_event_response() -> Output {
    use signal_lojix::{
        DatabaseMarker, DeploymentPhase, DeploymentPhaseEvent, DeploymentTerminal, TransitionMarker,
    };
    Output::DeploymentPhaseObserved(DeploymentPhaseEvent {
        deployment_identifier: signal_lojix::DeploymentIdentifier(190),
        generation_identifier: signal_lojix::GenerationIdentifier(41),
        cluster_name: signal_lojix::ClusterName("goldragon".into()),
        node_name: signal_lojix::NodeName("ouranos".into()),
        deployment_phase: DeploymentPhase::Completed,
        event_log_position: signal_lojix::EventLogPosition(7),
        transition_marker: TransitionMarker {
            database_marker: DatabaseMarker {
                commit_sequence: signal_lojix::CommitSequence(12),
                state_digest: signal_lojix::StateDigest(33),
            },
        },
        immutable_revision_option: Some(signal_lojix::ImmutableRevision(
            "0123456789abcdef0123456789abcdef01234567".into(),
        )),
        deployment_terminal_option: Some(DeploymentTerminal::Succeeded),
    })
}

#[test]
fn an_observed_phase_event_crosses_peer_bytes() {
    let response = observed_phase_event_response();
    let sent = response
        .signalize()
        .expect("signalize observed phase event");
    let received = Signal::<Output>::from(sent.bytes().to_vec());
    assert_eq!(
        received.restore().expect("restore observed phase event"),
        response
    );
}

#[cfg(feature = "datom")]
#[test]
fn an_observed_phase_event_crosses_datom() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    let response = observed_phase_event_response();
    let rendered = response.clone().datomize(vec![]).protosize().textualize();
    assert!(
        rendered.starts_with("DeploymentPhaseObserved."),
        "an observed event is headed by its output: {rendered}"
    );
    let mut pending = Potential::<Output>::from(rendered);
    let restored = pending
        .actualize(&mut Budget {
            remaining: 4_096,
            reader: ReaderBudget { remaining: 4_096 },
            depth: 0,
            maximum_depth: 256,
        })
        .expect("restore datom observed phase event");
    assert_eq!(restored, response);
}

fn marker() -> signal_lojix::DatabaseMarker {
    signal_lojix::DatabaseMarker {
        commit_sequence: signal_lojix::CommitSequence(3),
        state_digest: signal_lojix::StateDigest(5),
    }
}

/// A Nexus whose every capability succeeds with a fixed value, so the
/// generated dispatch shows which output each input is answered by.
struct AnsweringNexus;

impl signal_lojix::Answering for AnsweringNexus {
    fn configure(
        &self,
        configuration: &LojixNexusConfiguration,
    ) -> Result<ConfigurationReceipt, Output> {
        Ok(ConfigurationReceipt {
            lojix_nexus_configuration: configuration.clone(),
            meta_configure_occurred: signal_lojix::MetaConfigureOccurred(false),
        })
    }

    fn watch_deployments(
        &self,
        _watch: &signal_lojix::DeploymentWatch,
    ) -> Result<signal_lojix::SubscriptionOpened, Output> {
        Ok(signal_lojix::SubscriptionOpened {
            subscription_token: signal_lojix::SubscriptionToken(1),
            commit_sequence: signal_lojix::CommitSequence(3),
        })
    }

    fn query(&self, _selection: &Selection) -> Result<signal_lojix::GenerationListing, Output> {
        Ok(signal_lojix::GenerationListing {
            generation_vector: vec![],
            deployment_record_vector: vec![],
            database_marker: marker(),
        })
    }

    fn query_deployment_events(
        &self,
        _range: &signal_lojix::EventLogRange,
    ) -> Result<signal_lojix::EventLogPage, Output> {
        Ok(signal_lojix::EventLogPage {
            deployment_phase_event_vector: vec![],
            cache_retention_transition_event_vector: vec![],
            database_marker: marker(),
        })
    }

    fn query_test_runs(
        &self,
        _lookup: &signal_lojix::TestRunLookup,
    ) -> Result<signal_lojix::TestRunListing, Output> {
        Ok(signal_lojix::TestRunListing {
            test_run_record_vector: vec![],
            database_marker: marker(),
        })
    }

    fn unwatch(
        &self,
        close: &signal_lojix::SubscriptionClose,
    ) -> Result<signal_lojix::SubscriptionClosed, Output> {
        Ok(signal_lojix::SubscriptionClosed {
            subscription_token: close.subscription_token,
        })
    }
}

#[test]
fn each_input_is_answered_by_its_own_output() {
    use signal_lojix::Answerable;

    let inputs = [
        Input::Configure(configuration()),
        Input::WatchDeployments(signal_lojix::DeploymentWatch {
            deployment_identifier_option: None,
            cluster_name_option: None,
            node_name_option: None,
        }),
        node_selection(),
        Input::QueryDeploymentEvents(signal_lojix::EventLogRange {
            first_event_log_position: signal_lojix::EventLogPosition(0),
            second_event_log_position: signal_lojix::EventLogPosition(10),
        }),
        Input::QueryTestRuns(signal_lojix::TestRunLookup {
            cluster_name: signal_lojix::ClusterName("goldragon".into()),
            node_name: signal_lojix::NodeName("ouranos".into()),
            test_run_identifier_option: None,
        }),
        Input::Unwatch(signal_lojix::SubscriptionClose {
            subscription_token: signal_lojix::SubscriptionToken(1),
        }),
    ];
    let answered = inputs
        .iter()
        .map(|input| match input.answer(&AnsweringNexus) {
            Output::Configured(_) => "Configured",
            Output::DeploymentsWatched(_) => "DeploymentsWatched",
            Output::Queried(_) => "Queried",
            Output::DeploymentEventsQueried(_) => "DeploymentEventsQueried",
            Output::TestRunsQueried(_) => "TestRunsQueried",
            Output::Unwatched(_) => "Unwatched",
            _ => "a refusal",
        })
        .collect::<Vec<_>>();
    assert_eq!(
        answered,
        [
            "Configured",
            "DeploymentsWatched",
            "Queried",
            "DeploymentEventsQueried",
            "TestRunsQueried",
            "Unwatched",
        ]
    );
}

#[test]
fn a_store_refusal_is_an_output_of_the_contract() {
    let refusal = Output::Refused(signal_lojix::Refusal::Store("the commit failed".into()));
    let received = Signal::<Output>::from(
        refusal
            .signalize()
            .expect("signalize refusal")
            .bytes()
            .to_vec(),
    );
    assert_eq!(received.restore().expect("restore refusal"), refusal);
}

#[cfg(feature = "datom")]
#[test]
fn a_store_refusal_is_written_by_its_name() {
    use datom_codec::Datomizable;
    use protos::{Protosizable, Textualizable};

    let refusal = Output::Refused(signal_lojix::Refusal::Store("the commit failed".into()));
    assert_eq!(
        refusal.datomize(vec![]).protosize().textualize(),
        "Refused.Store.«the commit failed»"
    );
}

/// The query texts CriomOS's checks send read as they did before the scalar
/// declarations became newtypes, and write back unchanged.
#[cfg(feature = "datom")]
#[test]
fn the_query_texts_criomos_sends_read_and_write_unchanged() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Compactable, Protosizable, ReaderBudget};

    let read = |text: &str| {
        Potential::<Input>::from(text)
            .actualize(&mut Budget {
                remaining: 4_096,
                reader: ReaderBudget { remaining: 4_096 },
                depth: 0,
                maximum_depth: 256,
            })
            .expect("read the query text")
    };
    let by_node = "Query.ByNode.{ fixture-cluster fixture-node None }";
    let query = read(by_node);
    assert_eq!(
        query,
        Input::Query(Selection::ByNode(NodeSelector {
            cluster_name: signal_lojix::ClusterName("fixture-cluster".into()),
            node_name: signal_lojix::NodeName("fixture-node".into()),
            generation_artifact_option: None,
        }))
    );
    assert_eq!(query.datomize(vec![]).protosize().compact(), by_node);
    let by_deployment = "Query.ByDeployment.{ 1 }";
    let query = read(by_deployment);
    assert_eq!(
        query,
        Input::Query(Selection::ByDeployment(signal_lojix::DeploymentLookup {
            deployment_identifier: signal_lojix::DeploymentIdentifier(1),
        }))
    );
    assert_eq!(query.datomize(vec![]).protosize().compact(), by_deployment);
}
