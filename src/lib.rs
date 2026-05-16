//! Signal contract for the `lojix` deploy orchestrator.
//!
//! This crate owns the typed records that cross the Unix socket
//! between the thin `lojix` CLI and the long-lived
//! `lojix-daemon`. It is a pure contract crate: records,
//! validation newtypes, NOTA projection, and rkyv archive shape.
//! Runtime actors, storage tables, subprocess execution, and socket
//! lifecycle live in the `lojix` implementation crate.

use nota_codec::{NotaEnum, NotaRecord, NotaSum, NotaTransparent, NotaTryTransparent};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::signal_channel;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("{kind} must be non-empty ASCII letters, digits, dash, or underscore: {value}")]
    InvalidIdentifier { kind: &'static str, value: String },
    #[error("{kind} must be non-empty single-line text: {value}")]
    InvalidSingleLineText { kind: &'static str, value: String },
    #[error("{kind} must be a single /nix/store path: {value}")]
    InvalidStorePath { kind: &'static str, value: String },
}

macro_rules! validated_identifier {
    ($type_name:ident, $kind:literal) => {
        #[derive(
            Archive,
            RkyvSerialize,
            RkyvDeserialize,
            NotaTryTransparent,
            Debug,
            Clone,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
        )]
        pub struct $type_name(String);

        impl $type_name {
            pub fn try_new(value: String) -> Result<Self> {
                Self::from_text(value)
            }

            pub fn from_text(value: impl Into<String>) -> Result<Self> {
                let value = value.into();
                let valid = !value.is_empty()
                    && value.chars().all(|character| {
                        character.is_ascii_alphanumeric() || character == '-' || character == '_'
                    });
                if valid {
                    Ok(Self(value))
                } else {
                    Err(Error::InvalidIdentifier { kind: $kind, value })
                }
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $type_name {
            type Error = Error;

            fn try_from(value: String) -> Result<Self> {
                Self::from_text(value)
            }
        }

        impl TryFrom<&str> for $type_name {
            type Error = Error;

            fn try_from(value: &str) -> Result<Self> {
                Self::from_text(value)
            }
        }

        impl AsRef<str> for $type_name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl fmt::Display for $type_name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

macro_rules! validated_single_line_text {
    ($type_name:ident, $kind:literal) => {
        #[derive(
            Archive,
            RkyvSerialize,
            RkyvDeserialize,
            NotaTryTransparent,
            Debug,
            Clone,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
        )]
        pub struct $type_name(String);

        impl $type_name {
            pub fn try_new(value: String) -> Result<Self> {
                Self::from_text(value)
            }

            pub fn from_text(value: impl Into<String>) -> Result<Self> {
                let value = value.into();
                if value.trim().is_empty() || value.contains('\n') || value.contains('\r') {
                    Err(Error::InvalidSingleLineText { kind: $kind, value })
                } else {
                    Ok(Self(value))
                }
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $type_name {
            type Error = Error;

            fn try_from(value: String) -> Result<Self> {
                Self::from_text(value)
            }
        }

        impl TryFrom<&str> for $type_name {
            type Error = Error;

            fn try_from(value: &str) -> Result<Self> {
                Self::from_text(value)
            }
        }

        impl AsRef<str> for $type_name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl fmt::Display for $type_name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

macro_rules! validated_store_path {
    ($type_name:ident, $kind:literal, $must_end_with:literal) => {
        #[derive(
            Archive,
            RkyvSerialize,
            RkyvDeserialize,
            NotaTryTransparent,
            Debug,
            Clone,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
        )]
        pub struct $type_name(String);

        impl $type_name {
            pub fn try_new(value: String) -> Result<Self> {
                Self::from_text(value)
            }

            pub fn from_text(value: impl Into<String>) -> Result<Self> {
                let value = value.into();
                let trimmed = value.trim();
                let has_required_suffix =
                    $must_end_with.is_empty() || trimmed.ends_with($must_end_with);
                if trimmed.starts_with("/nix/store/")
                    && has_required_suffix
                    && !trimmed.contains('\n')
                    && !trimmed.contains('\r')
                {
                    Ok(Self(trimmed.to_string()))
                } else {
                    Err(Error::InvalidStorePath { kind: $kind, value })
                }
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $type_name {
            type Error = Error;

            fn try_from(value: String) -> Result<Self> {
                Self::from_text(value)
            }
        }

        impl TryFrom<&str> for $type_name {
            type Error = Error;

            fn try_from(value: &str) -> Result<Self> {
                Self::from_text(value)
            }
        }

        impl AsRef<str> for $type_name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl fmt::Display for $type_name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

validated_identifier!(ClusterName, "cluster name");
validated_identifier!(NodeName, "node name");
validated_identifier!(UserName, "user name");
validated_identifier!(DeploymentId, "deployment id");
validated_identifier!(CacheRetentionMutationId, "cache retention mutation id");
validated_identifier!(GenerationId, "generation id");

validated_single_line_text!(ProposalSource, "proposal source");
validated_single_line_text!(FlakeReference, "flake reference");
validated_single_line_text!(FailureText, "failure text");
validated_single_line_text!(WirePath, "wire path");
validated_single_line_text!(OperatorIdentity, "operator identity");
validated_identifier!(UnixGroup, "unix group");

validated_store_path!(StorePath, "store path", "");
validated_store_path!(DerivationPath, "derivation path", ".drv");

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaTransparent,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct SocketMode(u32);

impl SocketMode {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn into_u32(self) -> u32 {
        self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplyRendering {
    Compact,
    Pretty,
    EventStreamLines,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct PeerDaemonBinding {
    pub cluster: ClusterName,
    pub node: NodeName,
    pub daemon_socket_path: WirePath,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct LojixDaemonConfiguration {
    pub daemon_socket_path: WirePath,
    pub daemon_socket_mode: SocketMode,
    pub daemon_socket_group: Option<UnixGroup>,
    pub state_directory: WirePath,
    pub gc_root_directory: WirePath,
    pub peer_daemons: Vec<PeerDaemonBinding>,
    pub operator_identity: OperatorIdentity,
    pub owned_cluster: ClusterName,
}

nota_config::impl_rkyv_configuration!(LojixDaemonConfiguration);

#[derive(NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct LojixCliConfiguration {
    pub daemon_socket_path: WirePath,
    pub reply_rendering: ReplyRendering,
}

nota_config::impl_nota_only_configuration!(LojixCliConfiguration);

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemAction {
    Eval,
    Build,
    Boot,
    Switch,
    Test,
    BootOnce,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum HomeMode {
    Build,
    Profile,
    Activate,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerationKind {
    FullOs,
    OsOnly,
    HomeOnly,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerationState {
    Built,
    Activated,
    Retired,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct FullOsDeployment {
    pub action: SystemAction,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct OsOnlyDeployment {
    pub action: SystemAction,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct HomeOnlyDeployment {
    pub user: UserName,
    pub mode: HomeMode,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaSum, Debug, Clone, PartialEq, Eq)]
pub enum DeploymentPlan {
    FullOsDeployment(FullOsDeployment),
    OsOnlyDeployment(OsOnlyDeployment),
    HomeOnlyDeployment(HomeOnlyDeployment),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct BuildLocally {}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DispatcherChoosesBuilder {}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct NamedBuilder {
    pub node: NodeName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaSum, Debug, Clone, PartialEq, Eq)]
pub enum BuilderSelection {
    BuildLocally(BuildLocally),
    DispatcherChoosesBuilder(DispatcherChoosesBuilder),
    NamedBuilder(NamedBuilder),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DeploymentSubmission {
    pub cluster: ClusterName,
    pub node: NodeName,
    pub source: ProposalSource,
    pub flake: FlakeReference,
    pub plan: DeploymentPlan,
    pub builder: BuilderSelection,
    pub substituters: Vec<NodeName>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DeploymentAccepted {
    pub deployment: DeploymentId,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum DeploymentRejectionReason {
    InvalidRequest,
    UnknownNode,
    BuilderUnavailable,
    SecretUnavailable,
    BuildFailed,
    ActivationFailed,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DeploymentRejected {
    pub reason: DeploymentRejectionReason,
    pub detail: Option<FailureText>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct EvaluatedDerivation {
    pub derivation_path: DerivationPath,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RealizedStorePath {
    pub store_path: StorePath,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaSum, Debug, Clone, PartialEq, Eq)]
pub enum BuildResult {
    EvaluatedDerivation(EvaluatedDerivation),
    RealizedStorePath(RealizedStorePath),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Generation {
    pub generation: GenerationId,
    pub cluster: ClusterName,
    pub node: NodeName,
    pub kind: GenerationKind,
    pub store_path: StorePath,
    pub state: GenerationState,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DeploymentSubmitted {
    pub deployment: DeploymentId,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DeploymentBuilding {
    pub deployment: DeploymentId,
    pub builder: BuilderSelection,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DeploymentBuilt {
    pub deployment: DeploymentId,
    pub result: BuildResult,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ClosureCopying {
    pub deployment: DeploymentId,
    pub store_path: StorePath,
    pub target: NodeName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ActivationRunning {
    pub deployment: DeploymentId,
    pub target: NodeName,
    pub plan: DeploymentPlan,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ActivationSucceeded {
    pub deployment: DeploymentId,
    pub generation: Generation,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DeploymentFailed {
    pub deployment: DeploymentId,
    pub reason: FailureText,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaSum, Debug, Clone, PartialEq, Eq)]
pub enum DeploymentPhase {
    DeploymentSubmitted(DeploymentSubmitted),
    DeploymentBuilding(DeploymentBuilding),
    DeploymentBuilt(DeploymentBuilt),
    ClosureCopying(ClosureCopying),
    ActivationRunning(ActivationRunning),
    ActivationSucceeded(ActivationSucceeded),
    DeploymentFailed(DeploymentFailed),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DeploymentObservation {
    pub phase: DeploymentPhase,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DeploymentObservationSubscription {
    pub cluster: Option<ClusterName>,
    pub node: Option<NodeName>,
    pub deployment: Option<DeploymentId>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DeploymentObservationToken {
    pub value: u64,
}

impl DeploymentObservationToken {
    pub const fn new(value: u64) -> Self {
        Self { value }
    }

    pub const fn value(&self) -> u64 {
        self.value
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DeploymentObservationSubscriptionOpened {
    pub token: DeploymentObservationToken,
    pub observations: Vec<DeploymentObservation>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DeploymentObservationSubscriptionClosed {
    pub token: DeploymentObservationToken,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct PinGeneration {}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct UnpinGeneration {}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RetireGeneration {}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaSum, Debug, Clone, PartialEq, Eq)]
pub enum CacheRetentionAction {
    PinGeneration(PinGeneration),
    UnpinGeneration(UnpinGeneration),
    RetireGeneration(RetireGeneration),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct CacheRetentionRequest {
    pub generation: GenerationId,
    pub action: CacheRetentionAction,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct CacheRetentionAccepted {
    pub mutation: CacheRetentionMutationId,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum CacheRetentionRejectionReason {
    UnknownGeneration,
    PolicyConflict,
    StoreUnavailable,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct CacheRetentionRejected {
    pub reason: CacheRetentionRejectionReason,
    pub detail: Option<FailureText>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheRetentionState {
    Pinned,
    Unpinned,
    Retired,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct CacheRetentionObservation {
    pub mutation: CacheRetentionMutationId,
    pub generation: GenerationId,
    pub state: CacheRetentionState,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct CacheRetentionObservationSubscription {
    pub generation: Option<GenerationId>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct CacheRetentionObservationToken {
    pub value: u64,
}

impl CacheRetentionObservationToken {
    pub const fn new(value: u64) -> Self {
        Self { value }
    }

    pub const fn value(&self) -> u64 {
        self.value
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct CacheRetentionObservationSubscriptionOpened {
    pub token: CacheRetentionObservationToken,
    pub observations: Vec<CacheRetentionObservation>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct CacheRetentionObservationSubscriptionClosed {
    pub token: CacheRetentionObservationToken,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct GenerationQuery {
    pub cluster: Option<ClusterName>,
    pub node: Option<NodeName>,
    pub kind: Option<GenerationKind>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct GenerationListing {
    pub generations: Vec<Generation>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationKind {
    DeploymentSubmission,
    CacheRetentionRequest,
    GenerationQuery,
    DeploymentObservationSubscription,
    CacheRetentionObservationSubscription,
    DeploymentObservationRetraction,
    CacheRetentionObservationRetraction,
}

// Per signal-core's `signal_channel!` macro: each request variant is
// preceded by the SignalVerb it instantiates, and the macro auto-emits
// the `Request::signal_verb()` mapping witness. Observations are push
// events, so they live in the event block rather than in ordinary
// request/reply exchange payloads.
signal_channel! {
    channel Lojix {
        request Request {
            Assert DeploymentSubmission(DeploymentSubmission),
            Mutate CacheRetentionRequest(CacheRetentionRequest),
            Match GenerationQuery(GenerationQuery),
            Subscribe DeploymentObservationSubscription(DeploymentObservationSubscription)
                opens DeploymentObservationStream,
            Subscribe CacheRetentionObservationSubscription(CacheRetentionObservationSubscription)
                opens CacheRetentionObservationStream,
            Retract DeploymentObservationRetraction(DeploymentObservationToken),
            Retract CacheRetentionObservationRetraction(CacheRetentionObservationToken),
        }
        reply Reply {
            DeploymentAccepted(DeploymentAccepted),
            DeploymentRejected(DeploymentRejected),
            CacheRetentionAccepted(CacheRetentionAccepted),
            CacheRetentionRejected(CacheRetentionRejected),
            GenerationListing(GenerationListing),
            DeploymentObservationSubscriptionOpened(DeploymentObservationSubscriptionOpened),
            DeploymentObservationSubscriptionClosed(DeploymentObservationSubscriptionClosed),
            CacheRetentionObservationSubscriptionOpened(CacheRetentionObservationSubscriptionOpened),
            CacheRetentionObservationSubscriptionClosed(CacheRetentionObservationSubscriptionClosed),
        }
        event Event {
            DeploymentObservation(DeploymentObservation) belongs DeploymentObservationStream,
            CacheRetentionObservation(CacheRetentionObservation)
                belongs CacheRetentionObservationStream,
        }
        stream DeploymentObservationStream {
            token DeploymentObservationToken;
            opened DeploymentObservationSubscriptionOpened;
            event DeploymentObservation;
            close DeploymentObservationRetraction;
        }
        stream CacheRetentionObservationStream {
            token CacheRetentionObservationToken;
            opened CacheRetentionObservationSubscriptionOpened;
            event CacheRetentionObservation;
            close CacheRetentionObservationRetraction;
        }
    }
}

impl Request {
    pub fn operation_kind(&self) -> OperationKind {
        match self {
            Self::DeploymentSubmission(_) => OperationKind::DeploymentSubmission,
            Self::CacheRetentionRequest(_) => OperationKind::CacheRetentionRequest,
            Self::GenerationQuery(_) => OperationKind::GenerationQuery,
            Self::DeploymentObservationSubscription(_) => {
                OperationKind::DeploymentObservationSubscription
            }
            Self::CacheRetentionObservationSubscription(_) => {
                OperationKind::CacheRetentionObservationSubscription
            }
            Self::DeploymentObservationRetraction(_) => {
                OperationKind::DeploymentObservationRetraction
            }
            Self::CacheRetentionObservationRetraction(_) => {
                OperationKind::CacheRetentionObservationRetraction
            }
        }
    }
}
