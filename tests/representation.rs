//! Every enumeration Lojix stores lists its variants in the order Lojix's
//! Memory holds them (lojix `src/runtime_model.rs` at da0ad8a), so a stored
//! value keeps its archived tag: the n-th variant archives with tag n.
//! DeploymentPhase is also the stored DeploymentLifecycle, which had the same
//! nine variants in the same order.

use signal_lojix::*;

/// The archived tag of an enumeration value: the first byte of the root
/// object, which rkyv places at the end of the buffer.
fn tag<T>(value: &T) -> u8
where
    T: rkyv::Archive
        + for<'a> rkyv::Serialize<
            rkyv::api::high::HighSerializer<
                rkyv::util::AlignedVec,
                rkyv::ser::allocator::ArenaHandle<'a>,
                rkyv::rancor::Error,
            >,
        >,
{
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(value).expect("archive");
    bytes[bytes.len() - core::mem::size_of::<rkyv::Archived<T>>()]
}

fn assert_tags<T>(name: &str, values: &[T])
where
    T: rkyv::Archive
        + for<'a> rkyv::Serialize<
            rkyv::api::high::HighSerializer<
                rkyv::util::AlignedVec,
                rkyv::ser::allocator::ArenaHandle<'a>,
                rkyv::rancor::Error,
            >,
        >,
{
    let tags = values.iter().map(tag).collect::<Vec<u8>>();
    let expected = (0..values.len() as u8).collect::<Vec<u8>>();
    assert_eq!(tags, expected, "{name} archives in the stored order");
}

#[test]
fn stored_enumerations_archive_in_the_stored_order() {
    use ActivationEffect as Effect;
    use DeploymentFailureStage as Stage;
    use DeploymentTerminalReason as Reason;

    assert_tags(
        "DeploymentInputMode",
        &[DeploymentInputMode::Direct, DeploymentInputMode::Horizon],
    );
    assert_tags(
        "ActivationBackend",
        &[
            ActivationBackend::NixosSystemdBootV1,
            ActivationBackend::HomeManagerNixProfileV1,
        ],
    );
    assert_tags(
        "SecretsInput",
        &[
            SecretsInput::NoSecrets,
            SecretsInput::SecretsDirectory(SecretsDirectory("secrets".into())),
        ],
    );
    assert_tags(
        "SourceRevisionPolicy",
        &[
            SourceRevisionPolicy::RequireImmutable,
            SourceRevisionPolicy::ResolveAndRecord,
        ],
    );
    assert_tags(
        "GenerationArtifact",
        &[
            GenerationArtifact::CompleteHost,
            GenerationArtifact::BaseHost,
            GenerationArtifact::UserEnvironment,
        ],
    );
    assert_tags(
        "ActivationEffect",
        &[
            Effect::LiveActivation,
            Effect::BootProfile,
            Effect::TestActivation,
            Effect::BootOnceProfile,
            Effect::ProfileOnly,
        ],
    );
    assert_tags(
        "GenerationSlot",
        &[
            GenerationSlot::Current,
            GenerationSlot::BootPending,
            GenerationSlot::Rollback,
            GenerationSlot::Pinned,
            GenerationSlot::Recent,
        ],
    );
    assert_tags("TestMode", &[TestMode::Hermetic, TestMode::Live]);
    assert_tags(
        "TestRunPhase",
        &[
            TestRunPhase::Submitted,
            TestRunPhase::BringingUp,
            TestRunPhase::Deploying,
            TestRunPhase::Asserting,
            TestRunPhase::TearingDown,
            TestRunPhase::Completed,
            TestRunPhase::Failed,
        ],
    );
    assert_tags(
        "FailureStage",
        &[
            FailureStage::BringUp,
            FailureStage::Deploy,
            FailureStage::Assert,
            FailureStage::TearDown,
            FailureStage::HermeticCheck,
        ],
    );
    assert_tags(
        "TestOutcome",
        &[
            TestOutcome::Pending,
            TestOutcome::Passed,
            TestOutcome::Failed(FailureStage::BringUp),
        ],
    );
    assert_tags(
        "DeploymentEnvironment",
        &[
            DeploymentEnvironment::HostEnvironment,
            DeploymentEnvironment::UserEnvironment(UserName("li".into())),
        ],
    );
    assert_tags(
        "RequestedDeploymentAction",
        &[
            RequestedDeploymentAction::Host(HostDeployAction::Evaluate),
            RequestedDeploymentAction::UserEnvironment(UserEnvironmentAction::Realize),
        ],
    );
    assert_tags(
        "DeploymentPhase",
        &[
            DeploymentPhase::Submitted,
            DeploymentPhase::Building,
            DeploymentPhase::Built,
            DeploymentPhase::Copying,
            DeploymentPhase::Activating,
            DeploymentPhase::Activated,
            DeploymentPhase::Completed,
            DeploymentPhase::Rejected,
            DeploymentPhase::Failed,
        ],
    );
    assert_tags(
        "DeploymentFailureStage",
        &[
            Stage::Admission,
            Stage::FlakeAuth,
            Stage::MaterializeHorizon,
            Stage::Eval,
            Stage::Build,
            Stage::CopyClosure,
            Stage::Activate,
            Stage::Daemon,
        ],
    );
    assert_tags(
        "DeploymentTerminalReason",
        &[
            Reason::ClusterUnknown,
            Reason::NodeUnknown,
            Reason::ProposalSourceUnreachable,
            Reason::FlakeReferenceMalformed,
            Reason::InvalidDeploymentRouting,
            Reason::BuilderUnreachable,
            Reason::SubstituterUnreachable,
            Reason::DeploymentInFlight,
            Reason::UnsupportedDeployAction,
            Reason::InternalError,
            Reason::ActivationFailed,
            Reason::EvaluationFailed,
            Reason::BuildFailed,
            Reason::ClosureCopyFailed,
        ],
    );
    assert_tags(
        "DeploymentTerminal",
        &[
            DeploymentTerminal::Succeeded,
            DeploymentTerminal::Rejected(Reason::ClusterUnknown),
            DeploymentTerminal::Failed(DeploymentFailure {
                deployment_failure_stage: Stage::Build,
                deployment_terminal_reason: Reason::BuildFailed,
                failure_evidence_option: None,
            }),
        ],
    );
    assert_tags(
        "CacheRetentionTransition",
        &[
            CacheRetentionTransition::Pinned,
            CacheRetentionTransition::Unpinned,
            CacheRetentionTransition::Promoted,
            CacheRetentionTransition::Demoted,
            CacheRetentionTransition::Retired,
            CacheRetentionTransition::Evicted,
        ],
    );
    assert_tags(
        "HostDeployAction",
        &[
            HostDeployAction::Evaluate,
            HostDeployAction::Realize,
            HostDeployAction::SetBootProfile,
            HostDeployAction::ActivateNow,
            HostDeployAction::TestActivation,
            HostDeployAction::ScheduleBootOnce,
        ],
    );
    assert_tags(
        "HostComposition",
        &[HostComposition::CompleteHost, HostComposition::BaseHost],
    );
    assert_tags(
        "UserEnvironmentAction",
        &[
            UserEnvironmentAction::Realize,
            UserEnvironmentAction::SetProfile,
            UserEnvironmentAction::ActivateNow,
        ],
    );
}

/// A marker is now a one-field struct over DatabaseMarker; it archives to the
/// bytes of the DatabaseMarker it holds, as Lojix's stored marker newtypes do.
#[test]
fn a_marker_archives_as_the_database_marker_it_holds() {
    let database_marker = DatabaseMarker {
        commit_sequence: CommitSequence(9),
        state_digest: StateDigest(17),
    };
    let plain = rkyv::to_bytes::<rkyv::rancor::Error>(&database_marker).expect("archive");
    let admission = AdmissionMarker {
        database_marker: database_marker.clone(),
    };
    let transition = TransitionMarker {
        database_marker: database_marker.clone(),
    };
    let terminal = TerminalMarker { database_marker };
    for bytes in [
        rkyv::to_bytes::<rkyv::rancor::Error>(&admission).expect("archive"),
        rkyv::to_bytes::<rkyv::rancor::Error>(&transition).expect("archive"),
        rkyv::to_bytes::<rkyv::rancor::Error>(&terminal).expect("archive"),
    ] {
        assert_eq!(bytes.as_slice(), plain.as_slice());
    }
}
