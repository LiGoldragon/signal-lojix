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

#[cfg(feature = "datom")]
fn gold_horizon_definition() -> horizon_lib::HorizonDefinition {
    use datom_codec::Decimal;

    horizon_lib::HorizonDefinition {
        horizon_configuration: horizon_lib::HorizonConfiguration {
            generic_nodes: vec![horizon_lib::NodeDefinition {
                node_name: "opencode-test".into(),
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
                        first_interface: "enp1s0".into(),
                        second_interface: "wlp2s0".into(),
                        wlan_band: horizon_lib::WlanBand::FiveG,
                        integer: 36,
                        wlan_standard: horizon_lib::WlanStandard::Wifi6,
                        secret_reference_option: Some(horizon_lib::SecretReference {
                            secret_name: "router-wifi-password".into(),
                        }),
                        backup_wireless_option: None,
                        country_code: "MX".into(),
                    }),
                },
                node_keys: horizon_lib::NodeKeys {
                    ssh_pub_key: "ssh-ed25519 AAAAfixture".into(),
                    nix_pub_key_option: None,
                    yggdrasil_key_option: None,
                },
                boolean_option: None,
                capabilities: vec![
                    horizon_lib::NodeCapability::OpenCodeTesting(horizon_lib::NoSettings {}),
                    horizon_lib::NodeCapability::Router(horizon_lib::NoSettings {}),
                    horizon_lib::NodeCapability::TailnetController(
                        horizon_lib::TailnetController_Data {
                            certificate_authority_option: Some(
                                "-----BEGIN CERTIFICATE-----fixture".into(),
                            ),
                            tls_certificate_reference: horizon_lib::TlsCertificateReference {
                                secret_name: "headscale-tls-certificate".into(),
                            },
                            tls_key_reference: horizon_lib::TlsKeyReference {
                                secret_name: "headscale-tls-key".into(),
                            },
                        },
                    ),
                    horizon_lib::NodeCapability::TailnetClient(horizon_lib::SecretReference {
                        secret_name: "tailnet-preauth-key".into(),
                    }),
                    horizon_lib::NodeCapability::UsbDownlink(horizon_lib::UsbDownlink_Data {
                        ipv4_cidr: "10.47.0.1/24".into(),
                    }),
                ],
                fixed_location_option: Some(horizon_lib::FixedLocation {
                    first_decimal: Decimal::try_from(19.4326).expect("finite latitude"),
                    second_decimal: Decimal::try_from(-99.1332).expect("finite longitude"),
                    third_decimal: Decimal::try_from(2_240.0).expect("finite altitude"),
                    fourth_decimal: Decimal::try_from(3.5).expect("finite accuracy"),
                }),
            }],
            domain_configuration: horizon_lib::DomainConfiguration {
                string: "internal.invalid".into(),
                domain_name_vector: vec![],
            },
        },
        cluster_definition: horizon_lib::ClusterDefinition {
            cluster_name: "production.eu".into(),
            cluster_nodes: vec![],
            generic_node_names: vec!["opencode-test".into()],
            users: vec![],
            domains: vec![],
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
fn peer_bytes_restore_gold_opencode_testing_horizon_definition() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    let configuration = LojixNexusConfiguration {
        test_defaults_choice: TestDefaultsChoice::TestDefaults(signal_lojix::TestDefaults {
            cluster_name: "production.eu".into(),
            node_name: "opencode-test".into(),
            test_mode: signal_lojix::TestMode::Hermetic,
            flake_reference: "github:LiGoldragon/CriomOS".into(),
            nix_system: "x86_64-linux".into(),
            deployment_output_selector: signal_lojix::DeploymentOutputSelector {
                flake_attribute: "checks.x86_64-linux.contract".into(),
            },
            horizon_definition_option: Some(gold_horizon_definition()),
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

/// A closure copy that failed. `nix copy` engages no builder, so the copy
/// stage names itself rather than borrowing `BuilderUnreachable`. This example
/// is the falsifiable specification of that reason's place on the wire.
fn failed_closure_copy_response() -> Response {
    use signal_lojix::{
        ActivationEffect, DatabaseMarker, DeploymentEnvironment, DeploymentFailure,
        DeploymentFailureStage, DeploymentLifecycle, DeploymentRecord, DeploymentRequestIdentity,
        DeploymentTerminal, DeploymentTerminalReason, FailedCommand, FailureEvidence,
        GenerationArtifact, GenerationListing, HostDeployAction, RequestedDeploymentAction,
        SourceRevisionPolicy,
    };
    let marker = DatabaseMarker {
        commit_sequence: 31,
        state_digest: 44,
    };
    Response::Queried(GenerationListing {
        generation_vector: vec![],
        deployment_record_vector: vec![DeploymentRecord {
            deployment_identifier: 212,
            generation_identifier: 57,
            deployment_request_identity: DeploymentRequestIdentity {
                deployment_environment: DeploymentEnvironment::HostEnvironment,
                cluster_name: "goldragon".into(),
                node_name: "ouranos".into(),
                generation_artifact: GenerationArtifact::CompleteHost,
                requested_deployment_action: RequestedDeploymentAction::Host(
                    HostDeployAction::ActivateNow,
                ),
                activation_effect: ActivationEffect::LiveActivation,
                source_revision_policy: SourceRevisionPolicy::RequireImmutable,
                immutable_revision_option: None,
            },
            admission_marker_option: Some(marker.clone()),
            deployment_lifecycle: DeploymentLifecycle::Failed,
            terminal_marker_option: Some(marker.clone()),
            deployment_terminal_option: Some(DeploymentTerminal::Failed(DeploymentFailure {
                deployment_failure_stage: DeploymentFailureStage::CopyClosure,
                deployment_terminal_reason: DeploymentTerminalReason::ClosureCopyFailed,
                failure_evidence_option: Some(FailureEvidence {
                    failed_command_option: Some(FailedCommand {
                        command_program: "nix".into(),
                        command_argument_vector: vec![
                            "copy".into(),
                            "--substitute-on-destination".into(),
                            "--to".into(),
                            "ssh-ng://root@ouranos".into(),
                            "/nix/store/x-nixos-system".into(),
                        ],
                        exit_code_option: Some(1),
                    }),
                    failure_detail: "error: cannot open connection to remote store".into(),
                    detail_truncated: false,
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
    let received = Signal::<Response>::from(sent.bytes().to_vec());
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
    let mut pending = Potential::<Response>::from(rendered);
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
