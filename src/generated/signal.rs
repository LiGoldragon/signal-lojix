#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type OrdinarySocketPath = String;
#[rustfmt::skip]
pub type OrdinarySocketMode = i64;
#[rustfmt::skip]
pub type OwnerSocketPath = String;
#[rustfmt::skip]
pub type OwnerSocketMode = i64;
#[rustfmt::skip]
pub type StateDirectoryPath = String;
#[rustfmt::skip]
pub type DaemonHost = String;
#[rustfmt::skip]
pub type MetaConfigureOccurred = bool;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum TestDefaultsChoice {
    NoTestDefaults,
    TestDefaults(TestDefaults),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TestDefaults {
    pub cluster_name: ClusterName,
    pub node_name: NodeName,
    pub test_mode: TestMode,
    pub flake_reference: FlakeReference,
    pub nix_system: NixSystem,
    pub deployment_output_selector: DeploymentOutputSelector,
    pub horizon_definition_option: Option<horizon_lib::HorizonDefinition>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct LojixNexusConfiguration {
    pub ordinary_socket_path: OrdinarySocketPath,
    pub ordinary_socket_mode: OrdinarySocketMode,
    pub owner_socket_path: OwnerSocketPath,
    pub owner_socket_mode: OwnerSocketMode,
    pub state_directory_path: StateDirectoryPath,
    pub daemon_host: DaemonHost,
    pub test_defaults_choice: TestDefaultsChoice,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ConfigurationReceipt {
    pub lojix_nexus_configuration: LojixNexusConfiguration,
    pub meta_configure_occurred: MetaConfigureOccurred,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ConfigurationRejectionReason {
    OrdinaryConfigureClosed,
    InvalidConfiguration,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ConfigurationRejection {
    pub configuration_rejection_reason: ConfigurationRejectionReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum UserEnvironmentAction {
    ActivateNow,
    Realize,
    SetProfile,
}
#[rustfmt::skip]
pub type GenerationIdentifier = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum CacheRetentionTransition {
    Demoted,
    Retired,
    Pinned,
    Promoted,
    Unpinned,
    Evicted,
}
#[rustfmt::skip]
pub type CommitSequence = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct DeploymentPhaseEvent {
    pub deployment_identifier: DeploymentIdentifier,
    pub generation_identifier: GenerationIdentifier,
    pub cluster_name: ClusterName,
    pub node_name: NodeName,
    pub deployment_phase: DeploymentPhase,
    pub event_log_position: EventLogPosition,
    pub transition_marker: TransitionMarker,
    pub immutable_revision_option: Option<ImmutableRevision>,
    pub deployment_terminal_option: Option<DeploymentTerminal>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct DeploymentWatch {
    pub deployment_identifier_option: Option<DeploymentIdentifier>,
    pub cluster_name_option: Option<ClusterName>,
    pub node_name_option: Option<NodeName>,
}
#[rustfmt::skip]
pub type ProposalSource = String;
#[rustfmt::skip]
pub type SecretsDirectory = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum SecretsInput {
    NoSecrets,
    SecretsDirectory(SecretsDirectory),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum RequestedDeploymentAction {
    Host(HostDeployAction),
    UserEnvironment(UserEnvironmentAction),
}
#[rustfmt::skip]
pub type NodeName = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RejectedQuery {
    pub query_rejection_reason: QueryRejectionReason,
    pub database_marker: DatabaseMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct GenerationLookup {
    pub generation_identifier: GenerationIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum TestOutcome {
    Pending,
    Passed,
    Failed(FailureStage),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TestRunLookup {
    pub cluster_name: ClusterName,
    pub node_name: NodeName,
    pub test_run_identifier_option: Option<TestRunIdentifier>,
}
#[rustfmt::skip]
pub type SubscriptionToken = i64;
#[rustfmt::skip]
pub type NixSystem = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct DeploymentRecord {
    pub deployment_identifier: DeploymentIdentifier,
    pub generation_identifier: GenerationIdentifier,
    pub deployment_request_identity: DeploymentRequestIdentity,
    pub admission_marker_option: Option<AdmissionMarker>,
    pub deployment_lifecycle: DeploymentLifecycle,
    pub terminal_marker_option: Option<TerminalMarker>,
    pub deployment_terminal_option: Option<DeploymentTerminal>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum FailureStage {
    HermeticCheck,
    BringUp,
    Assert,
    Deploy,
    TearDown,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum HostDeployAction {
    TestActivation,
    ScheduleBootOnce,
    Realize,
    SetBootProfile,
    Evaluate,
    ActivateNow,
}
#[rustfmt::skip]
pub type PinLabel = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct GenerationListing {
    pub generation_vector: std::vec::Vec<Generation>,
    pub deployment_record_vector: std::vec::Vec<DeploymentRecord>,
    pub database_marker: DatabaseMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct DeploymentLookup {
    pub deployment_identifier: DeploymentIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum UnwatchRejectionReason {
    SubscriptionTokenUnknown,
    SubscriptionAlreadyClosed,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum GenerationSlot {
    Pinned,
    Recent,
    Rollback,
    BootPending,
    Current,
}
#[rustfmt::skip]
pub type TestRunIdentifier = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum HostComposition {
    CompleteHost,
    BaseHost,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct CacheRetentionWatch {
    pub cluster_name_option: Option<ClusterName>,
    pub node_name_option: Option<NodeName>,
}
#[rustfmt::skip]
pub type FlakeAttribute = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum DeploymentPhase {
    Built,
    Completed,
    Failed,
    Copying,
    Rejected,
    Activated,
    Submitted,
    Building,
    Activating,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct DatabaseMarker {
    pub commit_sequence: CommitSequence,
    pub state_digest: StateDigest,
}
#[rustfmt::skip]
pub type AdmissionMarker = DatabaseMarker;
#[rustfmt::skip]
pub type SshDestination = String;
#[rustfmt::skip]
pub type UserName = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct DeploymentRequestIdentity {
    pub deployment_environment: DeploymentEnvironment,
    pub cluster_name: ClusterName,
    pub node_name: NodeName,
    pub generation_artifact: GenerationArtifact,
    pub requested_deployment_action: RequestedDeploymentAction,
    pub activation_effect: ActivationEffect,
    pub source_revision_policy: SourceRevisionPolicy,
    pub immutable_revision_option: Option<ImmutableRevision>,
}
#[rustfmt::skip]
pub type TransitionMarker = DatabaseMarker;
#[rustfmt::skip]
pub type EventLogPosition = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RejectedWatch {
    pub watch_rejection_reason: WatchRejectionReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct CacheRetentionTransitionEvent {
    pub generation_identifier: GenerationIdentifier,
    pub cluster_name: ClusterName,
    pub node_name: NodeName,
    pub cache_retention_transition: CacheRetentionTransition,
    pub generation_slot: GenerationSlot,
    pub generation_slot_option: Option<GenerationSlot>,
    pub pin_label_option: Option<PinLabel>,
    pub event_log_position: EventLogPosition,
}
#[rustfmt::skip]
pub type NixBuilderSpec = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum DeploymentInputMode {
    Horizon,
    Direct,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum TestRunPhase {
    Submitted,
    BringingUp,
    TearingDown,
    Completed,
    Deploying,
    Asserting,
    Failed,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct DeploymentTransport {
    pub nix_store_uri: NixStoreUri,
    pub ssh_destination: SshDestination,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TestExecutionProfile {
    pub test_mode: TestMode,
    pub nix_system: NixSystem,
    pub deployment_output_selector: DeploymentOutputSelector,
    pub deployment_transport_option: Option<DeploymentTransport>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SubscriptionOpened {
    pub subscription_token: SubscriptionToken,
    pub commit_sequence: CommitSequence,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum WatchRejectionReason {
    MalformedWatch,
    SubscriptionLimitReached,
    StreamUnavailable,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ActivationBackend {
    HomeManagerNixProfileV1,
    NixosSystemdBootV1,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum GenerationArtifact {
    BaseHost,
    CompleteHost,
    UserEnvironment,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RejectedUnwatch {
    pub unwatch_rejection_reason: UnwatchRejectionReason,
    pub subscription_token: SubscriptionToken,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct EventLogPage {
    pub deployment_phase_event_vector: std::vec::Vec<DeploymentPhaseEvent>,
    pub cache_retention_transition_event_vector: std::vec::Vec<
        CacheRetentionTransitionEvent,
    >,
    pub database_marker: DatabaseMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SubscriptionClose {
    pub subscription_token: SubscriptionToken,
}
#[rustfmt::skip]
pub type TerminalMarker = DatabaseMarker;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum DeploymentEnvironment {
    HostEnvironment,
    UserEnvironment(UserName),
}
#[rustfmt::skip]
pub type ImmutableRevision = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum HostSelection {
    DefaultHost,
    OnHost(NodeName),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct EventLogRange {
    pub first_event_log_position: EventLogPosition,
    pub second_event_log_position: EventLogPosition,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum DeploymentLifecycle {
    Failed,
    Rejected,
    Completed,
    Building,
    Activating,
    Submitted,
    Copying,
    Activated,
    Built,
}
#[rustfmt::skip]
pub type ClosurePath = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ActivationEffect {
    ProfileOnly,
    BootOnceProfile,
    TestActivation,
    LiveActivation,
    BootProfile,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SubscriptionClosed {
    pub subscription_token: SubscriptionToken,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum TestMode {
    Hermetic,
    Live,
}
#[rustfmt::skip]
pub type StateDigest = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum QueryRejectionReason {
    MalformedSelector,
    EventLogPositionOutOfRange,
    GenerationUnknown,
    NodeUnknown,
}
#[rustfmt::skip]
pub type NixStoreUri = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TestRunListing {
    pub test_run_record_vector: std::vec::Vec<TestRunRecord>,
    pub database_marker: DatabaseMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum DeploymentTerminal {
    Succeeded,
    Failed(DeploymentFailure),
    Rejected(DeploymentTerminalReason),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum SourceRevisionPolicy {
    ResolveAndRecord,
    RequireImmutable,
}
#[rustfmt::skip]
pub type ClusterName = String;
#[rustfmt::skip]
pub type DeploymentIdentifier = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct DeploymentFailure {
    pub deployment_failure_stage: DeploymentFailureStage,
    pub deployment_terminal_reason: DeploymentTerminalReason,
    pub failure_evidence_option: Option<FailureEvidence>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct FailureEvidence {
    pub failed_command_option: Option<FailedCommand>,
    pub failure_detail: FailureDetail,
    pub detail_truncated: DetailTruncated,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct FailedCommand {
    pub command_program: CommandProgram,
    pub command_argument_vector: std::vec::Vec<CommandArgument>,
    pub exit_code_option: Option<ExitCode>,
}
#[rustfmt::skip]
pub type CommandProgram = String;
#[rustfmt::skip]
pub type CommandArgument = String;
#[rustfmt::skip]
pub type ExitCode = i64;
#[rustfmt::skip]
pub type FailureDetail = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum DeploymentFailureStage {
    Build,
    Eval,
    MaterializeHorizon,
    Daemon,
    Activate,
    CopyClosure,
    Admission,
    FlakeAuth,
}
#[rustfmt::skip]
pub type FlakeReference = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum DeploymentTerminalReason {
    NodeUnknown,
    FlakeReferenceMalformed,
    ProposalSourceUnreachable,
    DeploymentInFlight,
    InvalidDeploymentRouting,
    UnsupportedDeployAction,
    InternalError,
    ClusterUnknown,
    ActivationFailed,
    BuilderUnreachable,
    SubstituterUnreachable,
    EvaluationFailed,
    BuildFailed,
    ClosureCopyFailed,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Selection {
    ByNode(NodeSelector),
    ByTestRun(TestRunLookup),
    ByDeployment(DeploymentLookup),
    ByGeneration(GenerationLookup),
    ByEventLog(EventLogRange),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum RequestedGenerationArtifact {
    UserEnvironment,
    CompleteHost,
    BaseHost,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct NodeSelector {
    pub cluster_name: ClusterName,
    pub node_name: NodeName,
    pub requested_generation_artifact_option: Option<RequestedGenerationArtifact>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct DeploymentOutputSelector {
    pub flake_attribute: FlakeAttribute,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct Generation {
    pub generation_identifier: GenerationIdentifier,
    pub deployment_identifier: DeploymentIdentifier,
    pub cluster_name: ClusterName,
    pub node_name: NodeName,
    pub generation_artifact: GenerationArtifact,
    pub activation_effect: ActivationEffect,
    pub generation_slot: GenerationSlot,
    pub closure_path_option: Option<ClosurePath>,
    pub immutable_revision_option: Option<ImmutableRevision>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TestRunRecord {
    pub test_run_identifier: TestRunIdentifier,
    pub cluster_name: ClusterName,
    pub first_node_name: NodeName,
    pub second_node_name: NodeName,
    pub test_mode: TestMode,
    pub test_run_phase: TestRunPhase,
    pub test_outcome: TestOutcome,
    pub closure_path_option: Option<ClosurePath>,
}
#[rustfmt::skip]
pub type DetailTruncated = bool;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Query {
    Configure(LojixNexusConfiguration),
    WatchDeployments(DeploymentWatch),
    Query(Selection),
    WatchCacheRetention(CacheRetentionWatch),
    Unwatch(SubscriptionClose),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Response {
    Configured(ConfigurationReceipt),
    ConfigurationRejected(ConfigurationRejection),
    TestRunsQueried(TestRunListing),
    UnwatchRejected(RejectedUnwatch),
    QueryRejected(RejectedQuery),
    Watching(SubscriptionOpened),
    Queried(GenerationListing),
    DeploymentEventsQueried(EventLogPage),
    Unwatched(SubscriptionClosed),
    WatchRejected(RejectedWatch),
}
