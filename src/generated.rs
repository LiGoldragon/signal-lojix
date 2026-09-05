#![allow(dead_code)]
#![allow(clippy::redundant_closure)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserEnvironmentAction {
    ActivateNow,
    Realize,
    SetProfile,
}
impl datom_codec::Datomic for UserEnvironmentAction {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "ActivateNow" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::ActivateNow)
            }
            "Realize" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Realize)
            }
            "SetProfile" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::SetProfile)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for UserEnvironmentAction {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::ActivateNow => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("ActivateNow")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Realize => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Realize").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::SetProfile => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("SetProfile")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
pub type WatchCacheRetentionPayload = CacheRetentionWatch;
pub type GenerationIdentifier = protos::Integer;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CacheRetentionTransition {
    Demoted,
    Retired,
    Pinned,
    Promoted,
    Unpinned,
    Evicted,
}
impl datom_codec::Datomic for CacheRetentionTransition {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Demoted" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Demoted)
            }
            "Retired" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Retired)
            }
            "Pinned" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Pinned)
            }
            "Promoted" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Promoted)
            }
            "Unpinned" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Unpinned)
            }
            "Evicted" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Evicted)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for CacheRetentionTransition {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Demoted => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Demoted").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Retired => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Retired").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Pinned => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Pinned").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Promoted => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Promoted").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Unpinned => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Unpinned").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Evicted => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Evicted").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
pub type CommitSequence = protos::Integer;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeploymentPhaseEvent(
    pub DeploymentIdentifier,
    pub GenerationIdentifier,
    pub ClusterName,
    pub NodeName,
    pub DeploymentPhase,
    pub EventLogPosition,
    pub TransitionMarker,
    pub std::option::Option<ImmutableRevision>,
    pub std::option::Option<DeploymentTerminal>,
);
impl datom_codec::Datomic for DeploymentPhaseEvent {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 9)?;
        let p0: DeploymentIdentifier = datom_codec::Positional::position(&mut p)?;
        let p1: GenerationIdentifier = datom_codec::Positional::position(&mut p)?;
        let p2: ClusterName = datom_codec::Positional::position(&mut p)?;
        let p3: NodeName = datom_codec::Positional::position(&mut p)?;
        let p4: DeploymentPhase = datom_codec::Positional::position(&mut p)?;
        let p5: EventLogPosition = datom_codec::Positional::position(&mut p)?;
        let p6: TransitionMarker = datom_codec::Positional::position(&mut p)?;
        let p7: std::option::Option<ImmutableRevision> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p8: std::option::Option<DeploymentTerminal> = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1, p2, p3, p4, p5, p6, p7, p8))
    }
}
impl protos::Conceivable<datom_codec::Datom> for DeploymentPhaseEvent {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.3)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.4)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.5)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.6)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.7)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.8)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeploymentWatch(
    pub std::option::Option<DeploymentIdentifier>,
    pub std::option::Option<ClusterName>,
    pub std::option::Option<NodeName>,
);
impl datom_codec::Datomic for DeploymentWatch {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 3)?;
        let p0: std::option::Option<DeploymentIdentifier> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p1: std::option::Option<ClusterName> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p2: std::option::Option<NodeName> = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1, p2))
    }
}
impl protos::Conceivable<datom_codec::Datom> for DeploymentWatch {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type WatchRejectedPayload = RejectedWatch;
pub type ProposalSource = protos::Text;
pub type WatchingPayload = SubscriptionOpened;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RequestedDeploymentAction {
    Host(HostDeployAction),
    UserEnvironment(UserEnvironmentAction),
}
impl datom_codec::Datomic for RequestedDeploymentAction {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Host" => {
                std::result::Result::Ok(Self::Host(datom_codec::Carrying::body(v)?))
            }
            "UserEnvironment" => {
                std::result::Result::Ok(
                    Self::UserEnvironment(datom_codec::Carrying::body(v)?),
                )
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for RequestedDeploymentAction {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Host(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Host").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::UserEnvironment(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("UserEnvironment")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                },
            ),
        )
    }
}
pub type NodeName = protos::Text;
pub type QueryRejectedPayload = RejectedQuery;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RejectedQuery(pub QueryRejectionReason, pub DatabaseMarker);
impl datom_codec::Datomic for RejectedQuery {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: QueryRejectionReason = datom_codec::Positional::position(&mut p)?;
        let p1: DatabaseMarker = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for RejectedQuery {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GenerationLookup(pub GenerationIdentifier);
impl datom_codec::Datomic for GenerationLookup {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 1)?;
        let p0: GenerationIdentifier = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0))
    }
}
impl protos::Conceivable<datom_codec::Datom> for GenerationLookup {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TestOutcome {
    Failed(FailureStage),
    Pending,
    Passed,
}
impl datom_codec::Datomic for TestOutcome {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Failed" => {
                std::result::Result::Ok(Self::Failed(datom_codec::Carrying::body(v)?))
            }
            "Pending" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Pending)
            }
            "Passed" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Passed)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for TestOutcome {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Failed(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Failed").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Pending => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Pending").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Passed => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Passed").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyMaterialConcern {
    SecureShellPublicKey,
    YggdrasilPublicKey,
    YggdrasilAddress,
}
impl datom_codec::Datomic for KeyMaterialConcern {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "SecureShellPublicKey" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::SecureShellPublicKey)
            }
            "YggdrasilPublicKey" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::YggdrasilPublicKey)
            }
            "YggdrasilAddress" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::YggdrasilAddress)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for KeyMaterialConcern {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::SecureShellPublicKey => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("SecureShellPublicKey")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::YggdrasilPublicKey => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("YggdrasilPublicKey")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::YggdrasilAddress => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("YggdrasilAddress")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyMaterialMismatch(
    pub KeyMaterialConcern,
    pub MismatchValue,
    pub MismatchValue,
    pub OperatorHint,
);
impl datom_codec::Datomic for KeyMaterialMismatch {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 4)?;
        let p0: KeyMaterialConcern = datom_codec::Positional::position(&mut p)?;
        let p1: MismatchValue = datom_codec::Positional::position(&mut p)?;
        let p2: MismatchValue = datom_codec::Positional::position(&mut p)?;
        let p3: OperatorHint = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1, p2, p3))
    }
}
impl protos::Conceivable<datom_codec::Datom> for KeyMaterialMismatch {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.3)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyMaterialCheckRejectionReason {
    ProposalSourceUnreachable,
    HostUnreachable,
    PublicationMalformed,
    NodeUnknown,
}
impl datom_codec::Datomic for KeyMaterialCheckRejectionReason {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "ProposalSourceUnreachable" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::ProposalSourceUnreachable)
            }
            "HostUnreachable" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::HostUnreachable)
            }
            "PublicationMalformed" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::PublicationMalformed)
            }
            "NodeUnknown" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::NodeUnknown)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for KeyMaterialCheckRejectionReason {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::ProposalSourceUnreachable => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("ProposalSourceUnreachable")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::HostUnreachable => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("HostUnreachable")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::PublicationMalformed => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("PublicationMalformed")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::NodeUnknown => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("NodeUnknown")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TestRunLookup(
    pub ClusterName,
    pub NodeName,
    pub std::option::Option<TestRunIdentifier>,
);
impl datom_codec::Datomic for TestRunLookup {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 3)?;
        let p0: ClusterName = datom_codec::Positional::position(&mut p)?;
        let p1: NodeName = datom_codec::Positional::position(&mut p)?;
        let p2: std::option::Option<TestRunIdentifier> = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1, p2))
    }
}
impl protos::Conceivable<datom_codec::Datom> for TestRunLookup {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type SubscriptionToken = protos::Integer;
pub type NixSystem = protos::Text;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeploymentRecord(
    pub DeploymentIdentifier,
    pub GenerationIdentifier,
    pub DeploymentRequestIdentity,
    pub std::option::Option<AdmissionMarker>,
    pub DeploymentLifecycle,
    pub std::option::Option<TerminalMarker>,
    pub std::option::Option<DeploymentTerminal>,
);
impl datom_codec::Datomic for DeploymentRecord {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 7)?;
        let p0: DeploymentIdentifier = datom_codec::Positional::position(&mut p)?;
        let p1: GenerationIdentifier = datom_codec::Positional::position(&mut p)?;
        let p2: DeploymentRequestIdentity = datom_codec::Positional::position(&mut p)?;
        let p3: std::option::Option<AdmissionMarker> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p4: DeploymentLifecycle = datom_codec::Positional::position(&mut p)?;
        let p5: std::option::Option<TerminalMarker> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p6: std::option::Option<DeploymentTerminal> = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1, p2, p3, p4, p5, p6))
    }
}
impl protos::Conceivable<datom_codec::Datom> for DeploymentRecord {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.3)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.4)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.5)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.6)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type UnwatchedPayload = SubscriptionClosed;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FailureStage {
    HermeticCheck,
    BringUp,
    Assert,
    Deploy,
    TearDown,
}
impl datom_codec::Datomic for FailureStage {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "HermeticCheck" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::HermeticCheck)
            }
            "BringUp" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::BringUp)
            }
            "Assert" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Assert)
            }
            "Deploy" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Deploy)
            }
            "TearDown" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::TearDown)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for FailureStage {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::HermeticCheck => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("HermeticCheck")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::BringUp => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("BringUp").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Assert => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Assert").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Deploy => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Deploy").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::TearDown => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("TearDown").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TestRunRecord(
    pub TestRunIdentifier,
    pub ClusterName,
    pub NodeName,
    pub NodeName,
    pub TestMode,
    pub TestRunPhase,
    pub TestOutcome,
    pub std::option::Option<ClosurePath>,
);
impl datom_codec::Datomic for TestRunRecord {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 8)?;
        let p0: TestRunIdentifier = datom_codec::Positional::position(&mut p)?;
        let p1: ClusterName = datom_codec::Positional::position(&mut p)?;
        let p2: NodeName = datom_codec::Positional::position(&mut p)?;
        let p3: NodeName = datom_codec::Positional::position(&mut p)?;
        let p4: TestMode = datom_codec::Positional::position(&mut p)?;
        let p5: TestRunPhase = datom_codec::Positional::position(&mut p)?;
        let p6: TestOutcome = datom_codec::Positional::position(&mut p)?;
        let p7: std::option::Option<ClosurePath> = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1, p2, p3, p4, p5, p6, p7))
    }
}
impl protos::Conceivable<datom_codec::Datom> for TestRunRecord {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.3)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.4)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.5)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.6)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.7)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostDeployAction {
    TestActivation,
    ScheduleBootOnce,
    Realize,
    SetBootProfile,
    Evaluate,
    ActivateNow,
}
impl datom_codec::Datomic for HostDeployAction {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "TestActivation" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::TestActivation)
            }
            "ScheduleBootOnce" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::ScheduleBootOnce)
            }
            "Realize" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Realize)
            }
            "SetBootProfile" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::SetBootProfile)
            }
            "Evaluate" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Evaluate)
            }
            "ActivateNow" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::ActivateNow)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for HostDeployAction {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::TestActivation => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("TestActivation")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::ScheduleBootOnce => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("ScheduleBootOnce")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Realize => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Realize").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::SetBootProfile => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("SetBootProfile")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Evaluate => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Evaluate").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::ActivateNow => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("ActivateNow")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
pub type PinLabel = protos::Text;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GenerationListing(
    pub std::vec::Vec<Generation>,
    pub std::vec::Vec<DeploymentRecord>,
    pub DatabaseMarker,
);
impl datom_codec::Datomic for GenerationListing {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 3)?;
        let p0: std::vec::Vec<Generation> = datom_codec::Positional::position(&mut p)?;
        let p1: std::vec::Vec<DeploymentRecord> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p2: DatabaseMarker = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1, p2))
    }
}
impl protos::Conceivable<datom_codec::Datom> for GenerationListing {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeploymentLookup(pub DeploymentIdentifier);
impl datom_codec::Datomic for DeploymentLookup {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 1)?;
        let p0: DeploymentIdentifier = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0))
    }
}
impl protos::Conceivable<datom_codec::Datom> for DeploymentLookup {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type WatchDeploymentsPayload = DeploymentWatch;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnwatchRejectionReason {
    SubscriptionTokenUnknown,
    SubscriptionAlreadyClosed,
}
impl datom_codec::Datomic for UnwatchRejectionReason {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "SubscriptionTokenUnknown" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::SubscriptionTokenUnknown)
            }
            "SubscriptionAlreadyClosed" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::SubscriptionAlreadyClosed)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for UnwatchRejectionReason {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::SubscriptionTokenUnknown => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("SubscriptionTokenUnknown")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::SubscriptionAlreadyClosed => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("SubscriptionAlreadyClosed")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GenerationSlot {
    Pinned,
    Recent,
    Rollback,
    BootPending,
    Current,
}
impl datom_codec::Datomic for GenerationSlot {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Pinned" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Pinned)
            }
            "Recent" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Recent)
            }
            "Rollback" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Rollback)
            }
            "BootPending" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::BootPending)
            }
            "Current" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Current)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for GenerationSlot {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Pinned => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Pinned").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Recent => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Recent").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Rollback => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Rollback").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::BootPending => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("BootPending")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Current => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Current").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
pub type TestRunIdentifier = protos::Integer;
pub type MismatchValue = protos::Text;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostComposition {
    CompleteHost,
    BaseHost,
}
impl datom_codec::Datomic for HostComposition {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "CompleteHost" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::CompleteHost)
            }
            "BaseHost" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::BaseHost)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for HostComposition {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::CompleteHost => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("CompleteHost")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::BaseHost => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("BaseHost").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CacheRetentionWatch(
    pub std::option::Option<ClusterName>,
    pub std::option::Option<NodeName>,
);
impl datom_codec::Datomic for CacheRetentionWatch {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: std::option::Option<ClusterName> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p1: std::option::Option<NodeName> = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for CacheRetentionWatch {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type FlakeAttribute = protos::Text;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl datom_codec::Datomic for DeploymentPhase {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Built" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Built)
            }
            "Completed" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Completed)
            }
            "Failed" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Failed)
            }
            "Copying" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Copying)
            }
            "Rejected" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Rejected)
            }
            "Activated" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Activated)
            }
            "Submitted" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Submitted)
            }
            "Building" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Building)
            }
            "Activating" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Activating)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for DeploymentPhase {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Built => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Built").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Completed => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Completed").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Failed => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Failed").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Copying => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Copying").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Rejected => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Rejected").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Activated => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Activated").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Submitted => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Submitted").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Building => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Building").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Activating => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Activating")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DatabaseMarker(pub CommitSequence, pub StateDigest);
impl datom_codec::Datomic for DatabaseMarker {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: CommitSequence = datom_codec::Positional::position(&mut p)?;
        let p1: StateDigest = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for DatabaseMarker {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type AdmissionMarker = DatabaseMarker;
pub type SshDestination = protos::Text;
pub type UserName = protos::Text;
pub type KeyMaterialCheckRejectedPayload = RejectedKeyMaterialCheck;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeploymentRequestIdentity(
    pub DeploymentEnvironment,
    pub ClusterName,
    pub NodeName,
    pub GenerationArtifact,
    pub RequestedDeploymentAction,
    pub ActivationEffect,
    pub SourceRevisionPolicy,
    pub std::option::Option<ImmutableRevision>,
);
impl datom_codec::Datomic for DeploymentRequestIdentity {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 8)?;
        let p0: DeploymentEnvironment = datom_codec::Positional::position(&mut p)?;
        let p1: ClusterName = datom_codec::Positional::position(&mut p)?;
        let p2: NodeName = datom_codec::Positional::position(&mut p)?;
        let p3: GenerationArtifact = datom_codec::Positional::position(&mut p)?;
        let p4: RequestedDeploymentAction = datom_codec::Positional::position(&mut p)?;
        let p5: ActivationEffect = datom_codec::Positional::position(&mut p)?;
        let p6: SourceRevisionPolicy = datom_codec::Positional::position(&mut p)?;
        let p7: std::option::Option<ImmutableRevision> = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1, p2, p3, p4, p5, p6, p7))
    }
}
impl protos::Conceivable<datom_codec::Datom> for DeploymentRequestIdentity {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.3)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.4)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.5)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.6)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.7)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type QueryPayload = Selection;
pub type TransitionMarker = DatabaseMarker;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyMaterialReport(
    pub NodeName,
    pub std::vec::Vec<KeyMaterialMismatch>,
    pub DatabaseMarker,
);
impl datom_codec::Datomic for KeyMaterialReport {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 3)?;
        let p0: NodeName = datom_codec::Positional::position(&mut p)?;
        let p1: std::vec::Vec<KeyMaterialMismatch> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p2: DatabaseMarker = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1, p2))
    }
}
impl protos::Conceivable<datom_codec::Datom> for KeyMaterialReport {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeSelector(
    pub ClusterName,
    pub NodeName,
    pub std::option::Option<RequestedGenerationArtifact>,
);
impl datom_codec::Datomic for NodeSelector {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 3)?;
        let p0: ClusterName = datom_codec::Positional::position(&mut p)?;
        let p1: NodeName = datom_codec::Positional::position(&mut p)?;
        let p2: std::option::Option<RequestedGenerationArtifact> = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1, p2))
    }
}
impl protos::Conceivable<datom_codec::Datom> for NodeSelector {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type EventLogPosition = protos::Integer;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RejectedWatch(pub WatchRejectionReason);
impl datom_codec::Datomic for RejectedWatch {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 1)?;
        let p0: WatchRejectionReason = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0))
    }
}
impl protos::Conceivable<datom_codec::Datom> for RejectedWatch {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CacheRetentionTransitionEvent(
    pub GenerationIdentifier,
    pub ClusterName,
    pub NodeName,
    pub CacheRetentionTransition,
    pub GenerationSlot,
    pub std::option::Option<GenerationSlot>,
    pub std::option::Option<PinLabel>,
    pub EventLogPosition,
);
impl datom_codec::Datomic for CacheRetentionTransitionEvent {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 8)?;
        let p0: GenerationIdentifier = datom_codec::Positional::position(&mut p)?;
        let p1: ClusterName = datom_codec::Positional::position(&mut p)?;
        let p2: NodeName = datom_codec::Positional::position(&mut p)?;
        let p3: CacheRetentionTransition = datom_codec::Positional::position(&mut p)?;
        let p4: GenerationSlot = datom_codec::Positional::position(&mut p)?;
        let p5: std::option::Option<GenerationSlot> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p6: std::option::Option<PinLabel> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p7: EventLogPosition = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1, p2, p3, p4, p5, p6, p7))
    }
}
impl protos::Conceivable<datom_codec::Datom> for CacheRetentionTransitionEvent {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.3)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.4)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.5)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.6)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.7)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type NixBuilderSpec = protos::Text;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeploymentInputMode {
    Horizon,
    Direct,
}
impl datom_codec::Datomic for DeploymentInputMode {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Horizon" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Horizon)
            }
            "Direct" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Direct)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for DeploymentInputMode {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Horizon => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Horizon").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Direct => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Direct").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TestRunPhase {
    Submitted,
    BringingUp,
    TearingDown,
    Completed,
    Deploying,
    Asserting,
    Failed,
}
impl datom_codec::Datomic for TestRunPhase {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Submitted" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Submitted)
            }
            "BringingUp" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::BringingUp)
            }
            "TearingDown" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::TearingDown)
            }
            "Completed" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Completed)
            }
            "Deploying" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Deploying)
            }
            "Asserting" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Asserting)
            }
            "Failed" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Failed)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for TestRunPhase {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Submitted => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Submitted").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::BringingUp => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("BringingUp")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::TearingDown => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("TearingDown")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Completed => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Completed").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Deploying => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Deploying").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Asserting => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Asserting").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Failed => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Failed").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
pub type TestRunsQueriedPayload = TestRunListing;
pub type UnwatchRejectedPayload = RejectedUnwatch;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeploymentTransport(pub NixStoreUri, pub SshDestination);
impl datom_codec::Datomic for DeploymentTransport {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: NixStoreUri = datom_codec::Positional::position(&mut p)?;
        let p1: SshDestination = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for DeploymentTransport {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TestExecutionProfile(
    pub TestMode,
    pub NixSystem,
    pub DeploymentOutputSelector,
    pub std::option::Option<DeploymentTransport>,
);
impl datom_codec::Datomic for TestExecutionProfile {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 4)?;
        let p0: TestMode = datom_codec::Positional::position(&mut p)?;
        let p1: NixSystem = datom_codec::Positional::position(&mut p)?;
        let p2: DeploymentOutputSelector = datom_codec::Positional::position(&mut p)?;
        let p3: std::option::Option<DeploymentTransport> = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1, p2, p3))
    }
}
impl protos::Conceivable<datom_codec::Datom> for TestExecutionProfile {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.3)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubscriptionOpened(pub SubscriptionToken, pub CommitSequence);
impl datom_codec::Datomic for SubscriptionOpened {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: SubscriptionToken = datom_codec::Positional::position(&mut p)?;
        let p1: CommitSequence = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for SubscriptionOpened {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WatchRejectionReason {
    MalformedWatch,
    SubscriptionLimitReached,
    StreamUnavailable,
}
impl datom_codec::Datomic for WatchRejectionReason {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "MalformedWatch" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::MalformedWatch)
            }
            "SubscriptionLimitReached" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::SubscriptionLimitReached)
            }
            "StreamUnavailable" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::StreamUnavailable)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for WatchRejectionReason {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::MalformedWatch => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("MalformedWatch")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::SubscriptionLimitReached => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("SubscriptionLimitReached")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::StreamUnavailable => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("StreamUnavailable")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActivationBackend {
    HomeManagerNixProfileV1,
    NixosSystemdBootV1,
}
impl datom_codec::Datomic for ActivationBackend {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "HomeManagerNixProfileV1" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::HomeManagerNixProfileV1)
            }
            "NixosSystemdBootV1" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::NixosSystemdBootV1)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for ActivationBackend {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::HomeManagerNixProfileV1 => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("HomeManagerNixProfileV1")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::NixosSystemdBootV1 => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("NixosSystemdBootV1")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GenerationArtifact {
    BaseHost,
    CompleteHost,
    UserEnvironment,
}
impl datom_codec::Datomic for GenerationArtifact {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "BaseHost" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::BaseHost)
            }
            "CompleteHost" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::CompleteHost)
            }
            "UserEnvironment" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::UserEnvironment)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for GenerationArtifact {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::BaseHost => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("BaseHost").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::CompleteHost => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("CompleteHost")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::UserEnvironment => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("UserEnvironment")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
pub type KeyMaterialCheckedPayload = KeyMaterialReport;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RejectedUnwatch(pub UnwatchRejectionReason, pub SubscriptionToken);
impl datom_codec::Datomic for RejectedUnwatch {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: UnwatchRejectionReason = datom_codec::Positional::position(&mut p)?;
        let p1: SubscriptionToken = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for RejectedUnwatch {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventLogPage(
    pub std::vec::Vec<DeploymentPhaseEvent>,
    pub std::vec::Vec<CacheRetentionTransitionEvent>,
    pub DatabaseMarker,
);
impl datom_codec::Datomic for EventLogPage {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 3)?;
        let p0: std::vec::Vec<DeploymentPhaseEvent> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p1: std::vec::Vec<CacheRetentionTransitionEvent> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p2: DatabaseMarker = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1, p2))
    }
}
impl protos::Conceivable<datom_codec::Datom> for EventLogPage {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubscriptionClose(pub SubscriptionToken);
impl datom_codec::Datomic for SubscriptionClose {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 1)?;
        let p0: SubscriptionToken = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0))
    }
}
impl protos::Conceivable<datom_codec::Datom> for SubscriptionClose {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type TerminalMarker = DatabaseMarker;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeploymentEnvironment {
    HostEnvironment,
    UserEnvironment(UserName),
}
impl datom_codec::Datomic for DeploymentEnvironment {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "HostEnvironment" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::HostEnvironment)
            }
            "UserEnvironment" => {
                std::result::Result::Ok(
                    Self::UserEnvironment(datom_codec::Carrying::body(v)?),
                )
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for DeploymentEnvironment {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::HostEnvironment => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("HostEnvironment")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::UserEnvironment(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("UserEnvironment")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                },
            ),
        )
    }
}
pub type ImmutableRevision = protos::Text;
pub type UnwatchPayload = SubscriptionClose;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HostSelection {
    OnHost(NodeName),
    DefaultHost,
}
impl datom_codec::Datomic for HostSelection {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "OnHost" => {
                std::result::Result::Ok(Self::OnHost(datom_codec::Carrying::body(v)?))
            }
            "DefaultHost" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::DefaultHost)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for HostSelection {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::OnHost(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("OnHost").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::DefaultHost => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("DefaultHost")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventLogRange(pub EventLogPosition, pub EventLogPosition);
impl datom_codec::Datomic for EventLogRange {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: EventLogPosition = datom_codec::Positional::position(&mut p)?;
        let p1: EventLogPosition = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for EventLogRange {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl datom_codec::Datomic for DeploymentLifecycle {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Failed" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Failed)
            }
            "Rejected" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Rejected)
            }
            "Completed" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Completed)
            }
            "Building" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Building)
            }
            "Activating" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Activating)
            }
            "Submitted" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Submitted)
            }
            "Copying" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Copying)
            }
            "Activated" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Activated)
            }
            "Built" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Built)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for DeploymentLifecycle {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Failed => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Failed").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Rejected => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Rejected").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Completed => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Completed").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Building => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Building").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Activating => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Activating")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Submitted => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Submitted").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Copying => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Copying").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Activated => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Activated").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Built => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Built").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeploymentOutputSelector(pub FlakeAttribute);
impl datom_codec::Datomic for DeploymentOutputSelector {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 1)?;
        let p0: FlakeAttribute = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0))
    }
}
impl protos::Conceivable<datom_codec::Datom> for DeploymentOutputSelector {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type ClosurePath = protos::Text;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActivationEffect {
    ProfileOnly,
    BootOnceProfile,
    TestActivation,
    LiveActivation,
    BootProfile,
}
impl datom_codec::Datomic for ActivationEffect {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "ProfileOnly" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::ProfileOnly)
            }
            "BootOnceProfile" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::BootOnceProfile)
            }
            "TestActivation" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::TestActivation)
            }
            "LiveActivation" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::LiveActivation)
            }
            "BootProfile" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::BootProfile)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for ActivationEffect {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::ProfileOnly => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("ProfileOnly")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::BootOnceProfile => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("BootOnceProfile")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::TestActivation => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("TestActivation")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::LiveActivation => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("LiveActivation")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::BootProfile => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("BootProfile")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubscriptionClosed(pub SubscriptionToken);
impl datom_codec::Datomic for SubscriptionClosed {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 1)?;
        let p0: SubscriptionToken = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0))
    }
}
impl protos::Conceivable<datom_codec::Datom> for SubscriptionClosed {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type CheckHostKeyMaterialPayload = KeyMaterialQuery;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TestMode {
    Hermetic,
    Live,
}
impl datom_codec::Datomic for TestMode {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Hermetic" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Hermetic)
            }
            "Live" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Live)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for TestMode {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Hermetic => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Hermetic").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Live => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Live").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
pub type StateDigest = protos::Integer;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QueryRejectionReason {
    MalformedSelector,
    EventLogPositionOutOfRange,
    GenerationUnknown,
    NodeUnknown,
}
impl datom_codec::Datomic for QueryRejectionReason {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "MalformedSelector" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::MalformedSelector)
            }
            "EventLogPositionOutOfRange" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::EventLogPositionOutOfRange)
            }
            "GenerationUnknown" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::GenerationUnknown)
            }
            "NodeUnknown" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::NodeUnknown)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for QueryRejectionReason {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::MalformedSelector => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("MalformedSelector")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::EventLogPositionOutOfRange => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("EventLogPositionOutOfRange")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::GenerationUnknown => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("GenerationUnknown")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::NodeUnknown => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("NodeUnknown")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
pub type NixStoreUri = protos::Text;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TestRunListing(pub std::vec::Vec<TestRunRecord>, pub DatabaseMarker);
impl datom_codec::Datomic for TestRunListing {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: std::vec::Vec<TestRunRecord> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p1: DatabaseMarker = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for TestRunListing {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeploymentTerminal {
    Failed(DeploymentFailure),
    Rejected(DeploymentTerminalReason),
    Succeeded,
}
impl datom_codec::Datomic for DeploymentTerminal {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Failed" => {
                std::result::Result::Ok(Self::Failed(datom_codec::Carrying::body(v)?))
            }
            "Rejected" => {
                std::result::Result::Ok(Self::Rejected(datom_codec::Carrying::body(v)?))
            }
            "Succeeded" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Succeeded)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for DeploymentTerminal {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Failed(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Failed").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Rejected(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Rejected")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Succeeded => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Succeeded").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceRevisionPolicy {
    ResolveAndRecord,
    RequireImmutable,
}
impl datom_codec::Datomic for SourceRevisionPolicy {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "ResolveAndRecord" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::ResolveAndRecord)
            }
            "RequireImmutable" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::RequireImmutable)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for SourceRevisionPolicy {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::ResolveAndRecord => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("ResolveAndRecord")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::RequireImmutable => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("RequireImmutable")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyMaterialQuery(pub ClusterName, pub NodeName, pub ProposalSource);
impl datom_codec::Datomic for KeyMaterialQuery {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 3)?;
        let p0: ClusterName = datom_codec::Positional::position(&mut p)?;
        let p1: NodeName = datom_codec::Positional::position(&mut p)?;
        let p2: ProposalSource = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1, p2))
    }
}
impl protos::Conceivable<datom_codec::Datom> for KeyMaterialQuery {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type ClusterName = protos::Text;
pub type OperatorHint = protos::Text;
pub type DeploymentIdentifier = protos::Integer;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RequestedGenerationArtifact {
    UserEnvironment,
    CompleteHost,
    BaseHost,
}
impl datom_codec::Datomic for RequestedGenerationArtifact {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "UserEnvironment" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::UserEnvironment)
            }
            "CompleteHost" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::CompleteHost)
            }
            "BaseHost" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::BaseHost)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for RequestedGenerationArtifact {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::UserEnvironment => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("UserEnvironment")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::CompleteHost => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("CompleteHost")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::BaseHost => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("BaseHost").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
pub type DeploymentEventsQueriedPayload = EventLogPage;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeploymentFailure(pub DeploymentFailureStage, pub DeploymentTerminalReason);
impl datom_codec::Datomic for DeploymentFailure {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: DeploymentFailureStage = datom_codec::Positional::position(&mut p)?;
        let p1: DeploymentTerminalReason = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for DeploymentFailure {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl datom_codec::Datomic for DeploymentFailureStage {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Build" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Build)
            }
            "Eval" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Eval)
            }
            "MaterializeHorizon" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::MaterializeHorizon)
            }
            "Daemon" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Daemon)
            }
            "Activate" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Activate)
            }
            "CopyClosure" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::CopyClosure)
            }
            "Admission" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Admission)
            }
            "FlakeAuth" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::FlakeAuth)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for DeploymentFailureStage {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Build => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Build").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Eval => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Eval").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::MaterializeHorizon => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("MaterializeHorizon")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Daemon => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Daemon").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Activate => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Activate").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::CopyClosure => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("CopyClosure")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Admission => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Admission").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::FlakeAuth => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("FlakeAuth").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Generation(
    pub GenerationIdentifier,
    pub DeploymentIdentifier,
    pub ClusterName,
    pub NodeName,
    pub GenerationArtifact,
    pub ActivationEffect,
    pub GenerationSlot,
    pub std::option::Option<ClosurePath>,
    pub std::option::Option<ImmutableRevision>,
);
impl datom_codec::Datomic for Generation {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 9)?;
        let p0: GenerationIdentifier = datom_codec::Positional::position(&mut p)?;
        let p1: DeploymentIdentifier = datom_codec::Positional::position(&mut p)?;
        let p2: ClusterName = datom_codec::Positional::position(&mut p)?;
        let p3: NodeName = datom_codec::Positional::position(&mut p)?;
        let p4: GenerationArtifact = datom_codec::Positional::position(&mut p)?;
        let p5: ActivationEffect = datom_codec::Positional::position(&mut p)?;
        let p6: GenerationSlot = datom_codec::Positional::position(&mut p)?;
        let p7: std::option::Option<ClosurePath> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p8: std::option::Option<ImmutableRevision> = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1, p2, p3, p4, p5, p6, p7, p8))
    }
}
impl protos::Conceivable<datom_codec::Datom> for Generation {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.3)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.4)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.5)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.6)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.7)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.8)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type QueriedPayload = GenerationListing;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RejectedKeyMaterialCheck(
    pub KeyMaterialCheckRejectionReason,
    pub DatabaseMarker,
);
impl datom_codec::Datomic for RejectedKeyMaterialCheck {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: KeyMaterialCheckRejectionReason = datom_codec::Positional::position(
            &mut p,
        )?;
        let p1: DatabaseMarker = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for RejectedKeyMaterialCheck {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type FlakeReference = protos::Text;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
}
impl datom_codec::Datomic for DeploymentTerminalReason {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "NodeUnknown" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::NodeUnknown)
            }
            "FlakeReferenceMalformed" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::FlakeReferenceMalformed)
            }
            "ProposalSourceUnreachable" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::ProposalSourceUnreachable)
            }
            "DeploymentInFlight" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::DeploymentInFlight)
            }
            "InvalidDeploymentRouting" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::InvalidDeploymentRouting)
            }
            "UnsupportedDeployAction" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::UnsupportedDeployAction)
            }
            "InternalError" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::InternalError)
            }
            "ClusterUnknown" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::ClusterUnknown)
            }
            "ActivationFailed" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::ActivationFailed)
            }
            "BuilderUnreachable" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::BuilderUnreachable)
            }
            "SubstituterUnreachable" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::SubstituterUnreachable)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for DeploymentTerminalReason {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::NodeUnknown => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("NodeUnknown")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::FlakeReferenceMalformed => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("FlakeReferenceMalformed")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::ProposalSourceUnreachable => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("ProposalSourceUnreachable")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::DeploymentInFlight => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("DeploymentInFlight")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::InvalidDeploymentRouting => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("InvalidDeploymentRouting")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::UnsupportedDeployAction => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("UnsupportedDeployAction")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::InternalError => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("InternalError")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::ClusterUnknown => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("ClusterUnknown")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::ActivationFailed => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("ActivationFailed")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::BuilderUnreachable => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("BuilderUnreachable")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::SubstituterUnreachable => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("SubstituterUnreachable")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Selection {
    ByNode(NodeSelector),
    ByTestRun(TestRunLookup),
    ByDeployment(DeploymentLookup),
    ByGeneration(GenerationLookup),
    ByEventLog(EventLogRange),
}
impl datom_codec::Datomic for Selection {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "ByNode" => {
                std::result::Result::Ok(Self::ByNode(datom_codec::Carrying::body(v)?))
            }
            "ByTestRun" => {
                std::result::Result::Ok(Self::ByTestRun(datom_codec::Carrying::body(v)?))
            }
            "ByDeployment" => {
                std::result::Result::Ok(
                    Self::ByDeployment(datom_codec::Carrying::body(v)?),
                )
            }
            "ByGeneration" => {
                std::result::Result::Ok(
                    Self::ByGeneration(datom_codec::Carrying::body(v)?),
                )
            }
            "ByEventLog" => {
                std::result::Result::Ok(
                    Self::ByEventLog(datom_codec::Carrying::body(v)?),
                )
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for Selection {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::ByNode(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("ByNode").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::ByTestRun(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("ByTestRun")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::ByDeployment(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("ByDeployment")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::ByGeneration(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("ByGeneration")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::ByEventLog(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("ByEventLog")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Request {
    CheckHostKeyMaterial(CheckHostKeyMaterialPayload),
    WatchDeployments(WatchDeploymentsPayload),
    Query(QueryPayload),
    WatchCacheRetention(WatchCacheRetentionPayload),
    Unwatch(UnwatchPayload),
}
impl datom_codec::Datomic for Request {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "CheckHostKeyMaterial" => {
                std::result::Result::Ok(
                    Self::CheckHostKeyMaterial(datom_codec::Carrying::body(v)?),
                )
            }
            "WatchDeployments" => {
                std::result::Result::Ok(
                    Self::WatchDeployments(datom_codec::Carrying::body(v)?),
                )
            }
            "Query" => {
                std::result::Result::Ok(Self::Query(datom_codec::Carrying::body(v)?))
            }
            "WatchCacheRetention" => {
                std::result::Result::Ok(
                    Self::WatchCacheRetention(datom_codec::Carrying::body(v)?),
                )
            }
            "Unwatch" => {
                std::result::Result::Ok(Self::Unwatch(datom_codec::Carrying::body(v)?))
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for Request {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::CheckHostKeyMaterial(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("CheckHostKeyMaterial")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::WatchDeployments(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("WatchDeployments")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Query(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Query").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::WatchCacheRetention(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("WatchCacheRetention")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Unwatch(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Unwatch").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Response {
    TestRunsQueried(TestRunsQueriedPayload),
    UnwatchRejected(UnwatchRejectedPayload),
    QueryRejected(QueryRejectedPayload),
    Watching(WatchingPayload),
    KeyMaterialCheckRejected(KeyMaterialCheckRejectedPayload),
    Queried(QueriedPayload),
    DeploymentEventsQueried(DeploymentEventsQueriedPayload),
    Unwatched(UnwatchedPayload),
    KeyMaterialChecked(KeyMaterialCheckedPayload),
    WatchRejected(WatchRejectedPayload),
}
impl datom_codec::Datomic for Response {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "TestRunsQueried" => {
                std::result::Result::Ok(
                    Self::TestRunsQueried(datom_codec::Carrying::body(v)?),
                )
            }
            "UnwatchRejected" => {
                std::result::Result::Ok(
                    Self::UnwatchRejected(datom_codec::Carrying::body(v)?),
                )
            }
            "QueryRejected" => {
                std::result::Result::Ok(
                    Self::QueryRejected(datom_codec::Carrying::body(v)?),
                )
            }
            "Watching" => {
                std::result::Result::Ok(Self::Watching(datom_codec::Carrying::body(v)?))
            }
            "KeyMaterialCheckRejected" => {
                std::result::Result::Ok(
                    Self::KeyMaterialCheckRejected(datom_codec::Carrying::body(v)?),
                )
            }
            "Queried" => {
                std::result::Result::Ok(Self::Queried(datom_codec::Carrying::body(v)?))
            }
            "DeploymentEventsQueried" => {
                std::result::Result::Ok(
                    Self::DeploymentEventsQueried(datom_codec::Carrying::body(v)?),
                )
            }
            "Unwatched" => {
                std::result::Result::Ok(Self::Unwatched(datom_codec::Carrying::body(v)?))
            }
            "KeyMaterialChecked" => {
                std::result::Result::Ok(
                    Self::KeyMaterialChecked(datom_codec::Carrying::body(v)?),
                )
            }
            "WatchRejected" => {
                std::result::Result::Ok(
                    Self::WatchRejected(datom_codec::Carrying::body(v)?),
                )
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for Response {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::TestRunsQueried(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("TestRunsQueried")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::UnwatchRejected(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("UnwatchRejected")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::QueryRejected(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("QueryRejected")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Watching(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Watching")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::KeyMaterialCheckRejected(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("KeyMaterialCheckRejected")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Queried(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Queried").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::DeploymentEventsQueried(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("DeploymentEventsQueried")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Unwatched(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Unwatched")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::KeyMaterialChecked(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("KeyMaterialChecked")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::WatchRejected(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("WatchRejected")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                },
            ),
        )
    }
}
pub trait WireConversion: Sized {
    type Wire;
    fn into_wire(self) -> Self::Wire;
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault>;
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WireFault {
    Text,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum UserEnvironmentActionWire {
    ActivateNow,
    Realize,
    SetProfile,
}
pub type WatchCacheRetentionPayloadWire = CacheRetentionWatchWire;
pub type GenerationIdentifierWire = i64;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum CacheRetentionTransitionWire {
    Demoted,
    Retired,
    Pinned,
    Promoted,
    Unpinned,
    Evicted,
}
pub type CommitSequenceWire = i64;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct DeploymentPhaseEventWire(
    pub DeploymentIdentifierWire,
    pub GenerationIdentifierWire,
    pub ClusterNameWire,
    pub NodeNameWire,
    pub DeploymentPhaseWire,
    pub EventLogPositionWire,
    pub TransitionMarkerWire,
    pub std::option::Option<ImmutableRevisionWire>,
    pub std::option::Option<DeploymentTerminalWire>,
);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct DeploymentWatchWire(
    pub std::option::Option<DeploymentIdentifierWire>,
    pub std::option::Option<ClusterNameWire>,
    pub std::option::Option<NodeNameWire>,
);
pub type WatchRejectedPayloadWire = RejectedWatchWire;
pub type ProposalSourceWire = std::string::String;
pub type WatchingPayloadWire = SubscriptionOpenedWire;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum RequestedDeploymentActionWire {
    Host(HostDeployActionWire),
    UserEnvironment(UserEnvironmentActionWire),
}
pub type NodeNameWire = std::string::String;
pub type QueryRejectedPayloadWire = RejectedQueryWire;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RejectedQueryWire(pub QueryRejectionReasonWire, pub DatabaseMarkerWire);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GenerationLookupWire(pub GenerationIdentifierWire);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TestOutcomeWire {
    Failed(FailureStageWire),
    Pending,
    Passed,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum KeyMaterialConcernWire {
    SecureShellPublicKey,
    YggdrasilPublicKey,
    YggdrasilAddress,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct KeyMaterialMismatchWire(
    pub KeyMaterialConcernWire,
    pub MismatchValueWire,
    pub MismatchValueWire,
    pub OperatorHintWire,
);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum KeyMaterialCheckRejectionReasonWire {
    ProposalSourceUnreachable,
    HostUnreachable,
    PublicationMalformed,
    NodeUnknown,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TestRunLookupWire(
    pub ClusterNameWire,
    pub NodeNameWire,
    pub std::option::Option<TestRunIdentifierWire>,
);
pub type SubscriptionTokenWire = i64;
pub type NixSystemWire = std::string::String;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct DeploymentRecordWire(
    pub DeploymentIdentifierWire,
    pub GenerationIdentifierWire,
    pub DeploymentRequestIdentityWire,
    pub std::option::Option<AdmissionMarkerWire>,
    pub DeploymentLifecycleWire,
    pub std::option::Option<TerminalMarkerWire>,
    pub std::option::Option<DeploymentTerminalWire>,
);
pub type UnwatchedPayloadWire = SubscriptionClosedWire;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum FailureStageWire {
    HermeticCheck,
    BringUp,
    Assert,
    Deploy,
    TearDown,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TestRunRecordWire(
    pub TestRunIdentifierWire,
    pub ClusterNameWire,
    pub NodeNameWire,
    pub NodeNameWire,
    pub TestModeWire,
    pub TestRunPhaseWire,
    pub TestOutcomeWire,
    pub std::option::Option<ClosurePathWire>,
);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum HostDeployActionWire {
    TestActivation,
    ScheduleBootOnce,
    Realize,
    SetBootProfile,
    Evaluate,
    ActivateNow,
}
pub type PinLabelWire = std::string::String;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GenerationListingWire(
    pub std::vec::Vec<GenerationWire>,
    pub std::vec::Vec<DeploymentRecordWire>,
    pub DatabaseMarkerWire,
);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct DeploymentLookupWire(pub DeploymentIdentifierWire);
pub type WatchDeploymentsPayloadWire = DeploymentWatchWire;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum UnwatchRejectionReasonWire {
    SubscriptionTokenUnknown,
    SubscriptionAlreadyClosed,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum GenerationSlotWire {
    Pinned,
    Recent,
    Rollback,
    BootPending,
    Current,
}
pub type TestRunIdentifierWire = i64;
pub type MismatchValueWire = std::string::String;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum HostCompositionWire {
    CompleteHost,
    BaseHost,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CacheRetentionWatchWire(
    pub std::option::Option<ClusterNameWire>,
    pub std::option::Option<NodeNameWire>,
);
pub type FlakeAttributeWire = std::string::String;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum DeploymentPhaseWire {
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
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct DatabaseMarkerWire(pub CommitSequenceWire, pub StateDigestWire);
pub type AdmissionMarkerWire = DatabaseMarkerWire;
pub type SshDestinationWire = std::string::String;
pub type UserNameWire = std::string::String;
pub type KeyMaterialCheckRejectedPayloadWire = RejectedKeyMaterialCheckWire;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct DeploymentRequestIdentityWire(
    pub DeploymentEnvironmentWire,
    pub ClusterNameWire,
    pub NodeNameWire,
    pub GenerationArtifactWire,
    pub RequestedDeploymentActionWire,
    pub ActivationEffectWire,
    pub SourceRevisionPolicyWire,
    pub std::option::Option<ImmutableRevisionWire>,
);
pub type QueryPayloadWire = SelectionWire;
pub type TransitionMarkerWire = DatabaseMarkerWire;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct KeyMaterialReportWire(
    pub NodeNameWire,
    pub std::vec::Vec<KeyMaterialMismatchWire>,
    pub DatabaseMarkerWire,
);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct NodeSelectorWire(
    pub ClusterNameWire,
    pub NodeNameWire,
    pub std::option::Option<RequestedGenerationArtifactWire>,
);
pub type EventLogPositionWire = i64;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RejectedWatchWire(pub WatchRejectionReasonWire);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CacheRetentionTransitionEventWire(
    pub GenerationIdentifierWire,
    pub ClusterNameWire,
    pub NodeNameWire,
    pub CacheRetentionTransitionWire,
    pub GenerationSlotWire,
    pub std::option::Option<GenerationSlotWire>,
    pub std::option::Option<PinLabelWire>,
    pub EventLogPositionWire,
);
pub type NixBuilderSpecWire = std::string::String;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum DeploymentInputModeWire {
    Horizon,
    Direct,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TestRunPhaseWire {
    Submitted,
    BringingUp,
    TearingDown,
    Completed,
    Deploying,
    Asserting,
    Failed,
}
pub type TestRunsQueriedPayloadWire = TestRunListingWire;
pub type UnwatchRejectedPayloadWire = RejectedUnwatchWire;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct DeploymentTransportWire(pub NixStoreUriWire, pub SshDestinationWire);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TestExecutionProfileWire(
    pub TestModeWire,
    pub NixSystemWire,
    pub DeploymentOutputSelectorWire,
    pub std::option::Option<DeploymentTransportWire>,
);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SubscriptionOpenedWire(pub SubscriptionTokenWire, pub CommitSequenceWire);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum WatchRejectionReasonWire {
    MalformedWatch,
    SubscriptionLimitReached,
    StreamUnavailable,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ActivationBackendWire {
    HomeManagerNixProfileV1,
    NixosSystemdBootV1,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum GenerationArtifactWire {
    BaseHost,
    CompleteHost,
    UserEnvironment,
}
pub type KeyMaterialCheckedPayloadWire = KeyMaterialReportWire;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RejectedUnwatchWire(
    pub UnwatchRejectionReasonWire,
    pub SubscriptionTokenWire,
);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct EventLogPageWire(
    pub std::vec::Vec<DeploymentPhaseEventWire>,
    pub std::vec::Vec<CacheRetentionTransitionEventWire>,
    pub DatabaseMarkerWire,
);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SubscriptionCloseWire(pub SubscriptionTokenWire);
pub type TerminalMarkerWire = DatabaseMarkerWire;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum DeploymentEnvironmentWire {
    HostEnvironment,
    UserEnvironment(UserNameWire),
}
pub type ImmutableRevisionWire = std::string::String;
pub type UnwatchPayloadWire = SubscriptionCloseWire;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum HostSelectionWire {
    OnHost(NodeNameWire),
    DefaultHost,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct EventLogRangeWire(pub EventLogPositionWire, pub EventLogPositionWire);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum DeploymentLifecycleWire {
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
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct DeploymentOutputSelectorWire(pub FlakeAttributeWire);
pub type ClosurePathWire = std::string::String;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ActivationEffectWire {
    ProfileOnly,
    BootOnceProfile,
    TestActivation,
    LiveActivation,
    BootProfile,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SubscriptionClosedWire(pub SubscriptionTokenWire);
pub type CheckHostKeyMaterialPayloadWire = KeyMaterialQueryWire;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TestModeWire {
    Hermetic,
    Live,
}
pub type StateDigestWire = i64;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum QueryRejectionReasonWire {
    MalformedSelector,
    EventLogPositionOutOfRange,
    GenerationUnknown,
    NodeUnknown,
}
pub type NixStoreUriWire = std::string::String;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TestRunListingWire(
    pub std::vec::Vec<TestRunRecordWire>,
    pub DatabaseMarkerWire,
);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum DeploymentTerminalWire {
    Failed(DeploymentFailureWire),
    Rejected(DeploymentTerminalReasonWire),
    Succeeded,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum SourceRevisionPolicyWire {
    ResolveAndRecord,
    RequireImmutable,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct KeyMaterialQueryWire(
    pub ClusterNameWire,
    pub NodeNameWire,
    pub ProposalSourceWire,
);
pub type ClusterNameWire = std::string::String;
pub type OperatorHintWire = std::string::String;
pub type DeploymentIdentifierWire = i64;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum RequestedGenerationArtifactWire {
    UserEnvironment,
    CompleteHost,
    BaseHost,
}
pub type DeploymentEventsQueriedPayloadWire = EventLogPageWire;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct DeploymentFailureWire(
    pub DeploymentFailureStageWire,
    pub DeploymentTerminalReasonWire,
);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum DeploymentFailureStageWire {
    Build,
    Eval,
    MaterializeHorizon,
    Daemon,
    Activate,
    CopyClosure,
    Admission,
    FlakeAuth,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GenerationWire(
    pub GenerationIdentifierWire,
    pub DeploymentIdentifierWire,
    pub ClusterNameWire,
    pub NodeNameWire,
    pub GenerationArtifactWire,
    pub ActivationEffectWire,
    pub GenerationSlotWire,
    pub std::option::Option<ClosurePathWire>,
    pub std::option::Option<ImmutableRevisionWire>,
);
pub type QueriedPayloadWire = GenerationListingWire;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RejectedKeyMaterialCheckWire(
    pub KeyMaterialCheckRejectionReasonWire,
    pub DatabaseMarkerWire,
);
pub type FlakeReferenceWire = std::string::String;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum DeploymentTerminalReasonWire {
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
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum SelectionWire {
    ByNode(NodeSelectorWire),
    ByTestRun(TestRunLookupWire),
    ByDeployment(DeploymentLookupWire),
    ByGeneration(GenerationLookupWire),
    ByEventLog(EventLogRangeWire),
}
impl WireConversion for UserEnvironmentAction {
    type Wire = UserEnvironmentActionWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            UserEnvironmentAction::ActivateNow => UserEnvironmentActionWire::ActivateNow,
            UserEnvironmentAction::Realize => UserEnvironmentActionWire::Realize,
            UserEnvironmentAction::SetProfile => UserEnvironmentActionWire::SetProfile,
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            UserEnvironmentActionWire::ActivateNow => {
                Ok(UserEnvironmentAction::ActivateNow)
            }
            UserEnvironmentActionWire::Realize => Ok(UserEnvironmentAction::Realize),
            UserEnvironmentActionWire::SetProfile => {
                Ok(UserEnvironmentAction::SetProfile)
            }
        }
    }
}
impl WireConversion for CacheRetentionTransition {
    type Wire = CacheRetentionTransitionWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            CacheRetentionTransition::Demoted => CacheRetentionTransitionWire::Demoted,
            CacheRetentionTransition::Retired => CacheRetentionTransitionWire::Retired,
            CacheRetentionTransition::Pinned => CacheRetentionTransitionWire::Pinned,
            CacheRetentionTransition::Promoted => CacheRetentionTransitionWire::Promoted,
            CacheRetentionTransition::Unpinned => CacheRetentionTransitionWire::Unpinned,
            CacheRetentionTransition::Evicted => CacheRetentionTransitionWire::Evicted,
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            CacheRetentionTransitionWire::Demoted => {
                Ok(CacheRetentionTransition::Demoted)
            }
            CacheRetentionTransitionWire::Retired => {
                Ok(CacheRetentionTransition::Retired)
            }
            CacheRetentionTransitionWire::Pinned => Ok(CacheRetentionTransition::Pinned),
            CacheRetentionTransitionWire::Promoted => {
                Ok(CacheRetentionTransition::Promoted)
            }
            CacheRetentionTransitionWire::Unpinned => {
                Ok(CacheRetentionTransition::Unpinned)
            }
            CacheRetentionTransitionWire::Evicted => {
                Ok(CacheRetentionTransition::Evicted)
            }
        }
    }
}
impl WireConversion for DeploymentPhaseEvent {
    type Wire = DeploymentPhaseEventWire;
    fn into_wire(self) -> Self::Wire {
        let DeploymentPhaseEvent(p0, p1, p2, p3, p4, p5, p6, p7, p8) = self;
        DeploymentPhaseEventWire(
            p0,
            p1,
            p2.to_string(),
            p3.to_string(),
            <DeploymentPhase as WireConversion>::into_wire(p4),
            p5,
            <DatabaseMarker as WireConversion>::into_wire(p6),
            p7.map(|value| value.to_string()),
            p8.map(|value| <DeploymentTerminal as WireConversion>::into_wire(value)),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let DeploymentPhaseEventWire(p0, p1, p2, p3, p4, p5, p6, p7, p8) = wire;
        Ok(
            DeploymentPhaseEvent(
                Ok(p0)?,
                Ok(p1)?,
                protos::Text::try_from(p2).map_err(|_| WireFault::Text)?,
                protos::Text::try_from(p3).map_err(|_| WireFault::Text)?,
                <DeploymentPhase as WireConversion>::try_from_wire(p4)?,
                Ok(p5)?,
                <DatabaseMarker as WireConversion>::try_from_wire(p6)?,
                p7
                    .map(|value| {
                        protos::Text::try_from(value).map_err(|_| WireFault::Text)
                    })
                    .transpose()?,
                p8
                    .map(|value| <DeploymentTerminal as WireConversion>::try_from_wire(
                        value,
                    ))
                    .transpose()?,
            ),
        )
    }
}
impl WireConversion for DeploymentWatch {
    type Wire = DeploymentWatchWire;
    fn into_wire(self) -> Self::Wire {
        let DeploymentWatch(p0, p1, p2) = self;
        DeploymentWatchWire(
            p0.map(|value| value),
            p1.map(|value| value.to_string()),
            p2.map(|value| value.to_string()),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let DeploymentWatchWire(p0, p1, p2) = wire;
        Ok(
            DeploymentWatch(
                p0.map(|value| Ok(value)).transpose()?,
                p1
                    .map(|value| {
                        protos::Text::try_from(value).map_err(|_| WireFault::Text)
                    })
                    .transpose()?,
                p2
                    .map(|value| {
                        protos::Text::try_from(value).map_err(|_| WireFault::Text)
                    })
                    .transpose()?,
            ),
        )
    }
}
impl WireConversion for RequestedDeploymentAction {
    type Wire = RequestedDeploymentActionWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            RequestedDeploymentAction::Host(value) => {
                RequestedDeploymentActionWire::Host(
                    <HostDeployAction as WireConversion>::into_wire(value),
                )
            }
            RequestedDeploymentAction::UserEnvironment(value) => {
                RequestedDeploymentActionWire::UserEnvironment(
                    <UserEnvironmentAction as WireConversion>::into_wire(value),
                )
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            RequestedDeploymentActionWire::Host(value) => {
                Ok(
                    RequestedDeploymentAction::Host(
                        <HostDeployAction as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            RequestedDeploymentActionWire::UserEnvironment(value) => {
                Ok(
                    RequestedDeploymentAction::UserEnvironment(
                        <UserEnvironmentAction as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
        }
    }
}
impl WireConversion for RejectedQuery {
    type Wire = RejectedQueryWire;
    fn into_wire(self) -> Self::Wire {
        let RejectedQuery(p0, p1) = self;
        RejectedQueryWire(
            <QueryRejectionReason as WireConversion>::into_wire(p0),
            <DatabaseMarker as WireConversion>::into_wire(p1),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let RejectedQueryWire(p0, p1) = wire;
        Ok(
            RejectedQuery(
                <QueryRejectionReason as WireConversion>::try_from_wire(p0)?,
                <DatabaseMarker as WireConversion>::try_from_wire(p1)?,
            ),
        )
    }
}
impl WireConversion for GenerationLookup {
    type Wire = GenerationLookupWire;
    fn into_wire(self) -> Self::Wire {
        let GenerationLookup(p0) = self;
        GenerationLookupWire(p0)
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let GenerationLookupWire(p0) = wire;
        Ok(GenerationLookup(Ok(p0)?))
    }
}
impl WireConversion for TestOutcome {
    type Wire = TestOutcomeWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            TestOutcome::Failed(value) => {
                TestOutcomeWire::Failed(
                    <FailureStage as WireConversion>::into_wire(value),
                )
            }
            TestOutcome::Pending => TestOutcomeWire::Pending,
            TestOutcome::Passed => TestOutcomeWire::Passed,
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            TestOutcomeWire::Failed(value) => {
                Ok(
                    TestOutcome::Failed(
                        <FailureStage as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            TestOutcomeWire::Pending => Ok(TestOutcome::Pending),
            TestOutcomeWire::Passed => Ok(TestOutcome::Passed),
        }
    }
}
impl WireConversion for KeyMaterialConcern {
    type Wire = KeyMaterialConcernWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            KeyMaterialConcern::SecureShellPublicKey => {
                KeyMaterialConcernWire::SecureShellPublicKey
            }
            KeyMaterialConcern::YggdrasilPublicKey => {
                KeyMaterialConcernWire::YggdrasilPublicKey
            }
            KeyMaterialConcern::YggdrasilAddress => {
                KeyMaterialConcernWire::YggdrasilAddress
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            KeyMaterialConcernWire::SecureShellPublicKey => {
                Ok(KeyMaterialConcern::SecureShellPublicKey)
            }
            KeyMaterialConcernWire::YggdrasilPublicKey => {
                Ok(KeyMaterialConcern::YggdrasilPublicKey)
            }
            KeyMaterialConcernWire::YggdrasilAddress => {
                Ok(KeyMaterialConcern::YggdrasilAddress)
            }
        }
    }
}
impl WireConversion for KeyMaterialMismatch {
    type Wire = KeyMaterialMismatchWire;
    fn into_wire(self) -> Self::Wire {
        let KeyMaterialMismatch(p0, p1, p2, p3) = self;
        KeyMaterialMismatchWire(
            <KeyMaterialConcern as WireConversion>::into_wire(p0),
            p1.to_string(),
            p2.to_string(),
            p3.to_string(),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let KeyMaterialMismatchWire(p0, p1, p2, p3) = wire;
        Ok(
            KeyMaterialMismatch(
                <KeyMaterialConcern as WireConversion>::try_from_wire(p0)?,
                protos::Text::try_from(p1).map_err(|_| WireFault::Text)?,
                protos::Text::try_from(p2).map_err(|_| WireFault::Text)?,
                protos::Text::try_from(p3).map_err(|_| WireFault::Text)?,
            ),
        )
    }
}
impl WireConversion for KeyMaterialCheckRejectionReason {
    type Wire = KeyMaterialCheckRejectionReasonWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            KeyMaterialCheckRejectionReason::ProposalSourceUnreachable => {
                KeyMaterialCheckRejectionReasonWire::ProposalSourceUnreachable
            }
            KeyMaterialCheckRejectionReason::HostUnreachable => {
                KeyMaterialCheckRejectionReasonWire::HostUnreachable
            }
            KeyMaterialCheckRejectionReason::PublicationMalformed => {
                KeyMaterialCheckRejectionReasonWire::PublicationMalformed
            }
            KeyMaterialCheckRejectionReason::NodeUnknown => {
                KeyMaterialCheckRejectionReasonWire::NodeUnknown
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            KeyMaterialCheckRejectionReasonWire::ProposalSourceUnreachable => {
                Ok(KeyMaterialCheckRejectionReason::ProposalSourceUnreachable)
            }
            KeyMaterialCheckRejectionReasonWire::HostUnreachable => {
                Ok(KeyMaterialCheckRejectionReason::HostUnreachable)
            }
            KeyMaterialCheckRejectionReasonWire::PublicationMalformed => {
                Ok(KeyMaterialCheckRejectionReason::PublicationMalformed)
            }
            KeyMaterialCheckRejectionReasonWire::NodeUnknown => {
                Ok(KeyMaterialCheckRejectionReason::NodeUnknown)
            }
        }
    }
}
impl WireConversion for TestRunLookup {
    type Wire = TestRunLookupWire;
    fn into_wire(self) -> Self::Wire {
        let TestRunLookup(p0, p1, p2) = self;
        TestRunLookupWire(p0.to_string(), p1.to_string(), p2.map(|value| value))
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let TestRunLookupWire(p0, p1, p2) = wire;
        Ok(
            TestRunLookup(
                protos::Text::try_from(p0).map_err(|_| WireFault::Text)?,
                protos::Text::try_from(p1).map_err(|_| WireFault::Text)?,
                p2.map(|value| Ok(value)).transpose()?,
            ),
        )
    }
}
impl WireConversion for DeploymentRecord {
    type Wire = DeploymentRecordWire;
    fn into_wire(self) -> Self::Wire {
        let DeploymentRecord(p0, p1, p2, p3, p4, p5, p6) = self;
        DeploymentRecordWire(
            p0,
            p1,
            <DeploymentRequestIdentity as WireConversion>::into_wire(p2),
            p3.map(|value| <DatabaseMarker as WireConversion>::into_wire(value)),
            <DeploymentLifecycle as WireConversion>::into_wire(p4),
            p5.map(|value| <DatabaseMarker as WireConversion>::into_wire(value)),
            p6.map(|value| <DeploymentTerminal as WireConversion>::into_wire(value)),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let DeploymentRecordWire(p0, p1, p2, p3, p4, p5, p6) = wire;
        Ok(
            DeploymentRecord(
                Ok(p0)?,
                Ok(p1)?,
                <DeploymentRequestIdentity as WireConversion>::try_from_wire(p2)?,
                p3
                    .map(|value| <DatabaseMarker as WireConversion>::try_from_wire(
                        value,
                    ))
                    .transpose()?,
                <DeploymentLifecycle as WireConversion>::try_from_wire(p4)?,
                p5
                    .map(|value| <DatabaseMarker as WireConversion>::try_from_wire(
                        value,
                    ))
                    .transpose()?,
                p6
                    .map(|value| <DeploymentTerminal as WireConversion>::try_from_wire(
                        value,
                    ))
                    .transpose()?,
            ),
        )
    }
}
impl WireConversion for FailureStage {
    type Wire = FailureStageWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            FailureStage::HermeticCheck => FailureStageWire::HermeticCheck,
            FailureStage::BringUp => FailureStageWire::BringUp,
            FailureStage::Assert => FailureStageWire::Assert,
            FailureStage::Deploy => FailureStageWire::Deploy,
            FailureStage::TearDown => FailureStageWire::TearDown,
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            FailureStageWire::HermeticCheck => Ok(FailureStage::HermeticCheck),
            FailureStageWire::BringUp => Ok(FailureStage::BringUp),
            FailureStageWire::Assert => Ok(FailureStage::Assert),
            FailureStageWire::Deploy => Ok(FailureStage::Deploy),
            FailureStageWire::TearDown => Ok(FailureStage::TearDown),
        }
    }
}
impl WireConversion for TestRunRecord {
    type Wire = TestRunRecordWire;
    fn into_wire(self) -> Self::Wire {
        let TestRunRecord(p0, p1, p2, p3, p4, p5, p6, p7) = self;
        TestRunRecordWire(
            p0,
            p1.to_string(),
            p2.to_string(),
            p3.to_string(),
            <TestMode as WireConversion>::into_wire(p4),
            <TestRunPhase as WireConversion>::into_wire(p5),
            <TestOutcome as WireConversion>::into_wire(p6),
            p7.map(|value| value.to_string()),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let TestRunRecordWire(p0, p1, p2, p3, p4, p5, p6, p7) = wire;
        Ok(
            TestRunRecord(
                Ok(p0)?,
                protos::Text::try_from(p1).map_err(|_| WireFault::Text)?,
                protos::Text::try_from(p2).map_err(|_| WireFault::Text)?,
                protos::Text::try_from(p3).map_err(|_| WireFault::Text)?,
                <TestMode as WireConversion>::try_from_wire(p4)?,
                <TestRunPhase as WireConversion>::try_from_wire(p5)?,
                <TestOutcome as WireConversion>::try_from_wire(p6)?,
                p7
                    .map(|value| {
                        protos::Text::try_from(value).map_err(|_| WireFault::Text)
                    })
                    .transpose()?,
            ),
        )
    }
}
impl WireConversion for HostDeployAction {
    type Wire = HostDeployActionWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            HostDeployAction::TestActivation => HostDeployActionWire::TestActivation,
            HostDeployAction::ScheduleBootOnce => HostDeployActionWire::ScheduleBootOnce,
            HostDeployAction::Realize => HostDeployActionWire::Realize,
            HostDeployAction::SetBootProfile => HostDeployActionWire::SetBootProfile,
            HostDeployAction::Evaluate => HostDeployActionWire::Evaluate,
            HostDeployAction::ActivateNow => HostDeployActionWire::ActivateNow,
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            HostDeployActionWire::TestActivation => Ok(HostDeployAction::TestActivation),
            HostDeployActionWire::ScheduleBootOnce => {
                Ok(HostDeployAction::ScheduleBootOnce)
            }
            HostDeployActionWire::Realize => Ok(HostDeployAction::Realize),
            HostDeployActionWire::SetBootProfile => Ok(HostDeployAction::SetBootProfile),
            HostDeployActionWire::Evaluate => Ok(HostDeployAction::Evaluate),
            HostDeployActionWire::ActivateNow => Ok(HostDeployAction::ActivateNow),
        }
    }
}
impl WireConversion for GenerationListing {
    type Wire = GenerationListingWire;
    fn into_wire(self) -> Self::Wire {
        let GenerationListing(p0, p1, p2) = self;
        GenerationListingWire(
            p0
                .into_iter()
                .map(|value| <Generation as WireConversion>::into_wire(value))
                .collect(),
            p1
                .into_iter()
                .map(|value| <DeploymentRecord as WireConversion>::into_wire(value))
                .collect(),
            <DatabaseMarker as WireConversion>::into_wire(p2),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let GenerationListingWire(p0, p1, p2) = wire;
        Ok(
            GenerationListing(
                p0
                    .into_iter()
                    .map(|value| <Generation as WireConversion>::try_from_wire(value))
                    .collect::<std::result::Result<std::vec::Vec<_>, WireFault>>()?,
                p1
                    .into_iter()
                    .map(|value| <DeploymentRecord as WireConversion>::try_from_wire(
                        value,
                    ))
                    .collect::<std::result::Result<std::vec::Vec<_>, WireFault>>()?,
                <DatabaseMarker as WireConversion>::try_from_wire(p2)?,
            ),
        )
    }
}
impl WireConversion for DeploymentLookup {
    type Wire = DeploymentLookupWire;
    fn into_wire(self) -> Self::Wire {
        let DeploymentLookup(p0) = self;
        DeploymentLookupWire(p0)
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let DeploymentLookupWire(p0) = wire;
        Ok(DeploymentLookup(Ok(p0)?))
    }
}
impl WireConversion for UnwatchRejectionReason {
    type Wire = UnwatchRejectionReasonWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            UnwatchRejectionReason::SubscriptionTokenUnknown => {
                UnwatchRejectionReasonWire::SubscriptionTokenUnknown
            }
            UnwatchRejectionReason::SubscriptionAlreadyClosed => {
                UnwatchRejectionReasonWire::SubscriptionAlreadyClosed
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            UnwatchRejectionReasonWire::SubscriptionTokenUnknown => {
                Ok(UnwatchRejectionReason::SubscriptionTokenUnknown)
            }
            UnwatchRejectionReasonWire::SubscriptionAlreadyClosed => {
                Ok(UnwatchRejectionReason::SubscriptionAlreadyClosed)
            }
        }
    }
}
impl WireConversion for GenerationSlot {
    type Wire = GenerationSlotWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            GenerationSlot::Pinned => GenerationSlotWire::Pinned,
            GenerationSlot::Recent => GenerationSlotWire::Recent,
            GenerationSlot::Rollback => GenerationSlotWire::Rollback,
            GenerationSlot::BootPending => GenerationSlotWire::BootPending,
            GenerationSlot::Current => GenerationSlotWire::Current,
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            GenerationSlotWire::Pinned => Ok(GenerationSlot::Pinned),
            GenerationSlotWire::Recent => Ok(GenerationSlot::Recent),
            GenerationSlotWire::Rollback => Ok(GenerationSlot::Rollback),
            GenerationSlotWire::BootPending => Ok(GenerationSlot::BootPending),
            GenerationSlotWire::Current => Ok(GenerationSlot::Current),
        }
    }
}
impl WireConversion for HostComposition {
    type Wire = HostCompositionWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            HostComposition::CompleteHost => HostCompositionWire::CompleteHost,
            HostComposition::BaseHost => HostCompositionWire::BaseHost,
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            HostCompositionWire::CompleteHost => Ok(HostComposition::CompleteHost),
            HostCompositionWire::BaseHost => Ok(HostComposition::BaseHost),
        }
    }
}
impl WireConversion for CacheRetentionWatch {
    type Wire = CacheRetentionWatchWire;
    fn into_wire(self) -> Self::Wire {
        let CacheRetentionWatch(p0, p1) = self;
        CacheRetentionWatchWire(
            p0.map(|value| value.to_string()),
            p1.map(|value| value.to_string()),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let CacheRetentionWatchWire(p0, p1) = wire;
        Ok(
            CacheRetentionWatch(
                p0
                    .map(|value| {
                        protos::Text::try_from(value).map_err(|_| WireFault::Text)
                    })
                    .transpose()?,
                p1
                    .map(|value| {
                        protos::Text::try_from(value).map_err(|_| WireFault::Text)
                    })
                    .transpose()?,
            ),
        )
    }
}
impl WireConversion for DeploymentPhase {
    type Wire = DeploymentPhaseWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            DeploymentPhase::Built => DeploymentPhaseWire::Built,
            DeploymentPhase::Completed => DeploymentPhaseWire::Completed,
            DeploymentPhase::Failed => DeploymentPhaseWire::Failed,
            DeploymentPhase::Copying => DeploymentPhaseWire::Copying,
            DeploymentPhase::Rejected => DeploymentPhaseWire::Rejected,
            DeploymentPhase::Activated => DeploymentPhaseWire::Activated,
            DeploymentPhase::Submitted => DeploymentPhaseWire::Submitted,
            DeploymentPhase::Building => DeploymentPhaseWire::Building,
            DeploymentPhase::Activating => DeploymentPhaseWire::Activating,
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            DeploymentPhaseWire::Built => Ok(DeploymentPhase::Built),
            DeploymentPhaseWire::Completed => Ok(DeploymentPhase::Completed),
            DeploymentPhaseWire::Failed => Ok(DeploymentPhase::Failed),
            DeploymentPhaseWire::Copying => Ok(DeploymentPhase::Copying),
            DeploymentPhaseWire::Rejected => Ok(DeploymentPhase::Rejected),
            DeploymentPhaseWire::Activated => Ok(DeploymentPhase::Activated),
            DeploymentPhaseWire::Submitted => Ok(DeploymentPhase::Submitted),
            DeploymentPhaseWire::Building => Ok(DeploymentPhase::Building),
            DeploymentPhaseWire::Activating => Ok(DeploymentPhase::Activating),
        }
    }
}
impl WireConversion for DatabaseMarker {
    type Wire = DatabaseMarkerWire;
    fn into_wire(self) -> Self::Wire {
        let DatabaseMarker(p0, p1) = self;
        DatabaseMarkerWire(p0, p1)
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let DatabaseMarkerWire(p0, p1) = wire;
        Ok(DatabaseMarker(Ok(p0)?, Ok(p1)?))
    }
}
impl WireConversion for DeploymentRequestIdentity {
    type Wire = DeploymentRequestIdentityWire;
    fn into_wire(self) -> Self::Wire {
        let DeploymentRequestIdentity(p0, p1, p2, p3, p4, p5, p6, p7) = self;
        DeploymentRequestIdentityWire(
            <DeploymentEnvironment as WireConversion>::into_wire(p0),
            p1.to_string(),
            p2.to_string(),
            <GenerationArtifact as WireConversion>::into_wire(p3),
            <RequestedDeploymentAction as WireConversion>::into_wire(p4),
            <ActivationEffect as WireConversion>::into_wire(p5),
            <SourceRevisionPolicy as WireConversion>::into_wire(p6),
            p7.map(|value| value.to_string()),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let DeploymentRequestIdentityWire(p0, p1, p2, p3, p4, p5, p6, p7) = wire;
        Ok(
            DeploymentRequestIdentity(
                <DeploymentEnvironment as WireConversion>::try_from_wire(p0)?,
                protos::Text::try_from(p1).map_err(|_| WireFault::Text)?,
                protos::Text::try_from(p2).map_err(|_| WireFault::Text)?,
                <GenerationArtifact as WireConversion>::try_from_wire(p3)?,
                <RequestedDeploymentAction as WireConversion>::try_from_wire(p4)?,
                <ActivationEffect as WireConversion>::try_from_wire(p5)?,
                <SourceRevisionPolicy as WireConversion>::try_from_wire(p6)?,
                p7
                    .map(|value| {
                        protos::Text::try_from(value).map_err(|_| WireFault::Text)
                    })
                    .transpose()?,
            ),
        )
    }
}
impl WireConversion for KeyMaterialReport {
    type Wire = KeyMaterialReportWire;
    fn into_wire(self) -> Self::Wire {
        let KeyMaterialReport(p0, p1, p2) = self;
        KeyMaterialReportWire(
            p0.to_string(),
            p1
                .into_iter()
                .map(|value| <KeyMaterialMismatch as WireConversion>::into_wire(value))
                .collect(),
            <DatabaseMarker as WireConversion>::into_wire(p2),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let KeyMaterialReportWire(p0, p1, p2) = wire;
        Ok(
            KeyMaterialReport(
                protos::Text::try_from(p0).map_err(|_| WireFault::Text)?,
                p1
                    .into_iter()
                    .map(|value| <KeyMaterialMismatch as WireConversion>::try_from_wire(
                        value,
                    ))
                    .collect::<std::result::Result<std::vec::Vec<_>, WireFault>>()?,
                <DatabaseMarker as WireConversion>::try_from_wire(p2)?,
            ),
        )
    }
}
impl WireConversion for NodeSelector {
    type Wire = NodeSelectorWire;
    fn into_wire(self) -> Self::Wire {
        let NodeSelector(p0, p1, p2) = self;
        NodeSelectorWire(
            p0.to_string(),
            p1.to_string(),
            p2
                .map(|value| <RequestedGenerationArtifact as WireConversion>::into_wire(
                    value,
                )),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let NodeSelectorWire(p0, p1, p2) = wire;
        Ok(
            NodeSelector(
                protos::Text::try_from(p0).map_err(|_| WireFault::Text)?,
                protos::Text::try_from(p1).map_err(|_| WireFault::Text)?,
                p2
                    .map(|value| <RequestedGenerationArtifact as WireConversion>::try_from_wire(
                        value,
                    ))
                    .transpose()?,
            ),
        )
    }
}
impl WireConversion for RejectedWatch {
    type Wire = RejectedWatchWire;
    fn into_wire(self) -> Self::Wire {
        let RejectedWatch(p0) = self;
        RejectedWatchWire(<WatchRejectionReason as WireConversion>::into_wire(p0))
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let RejectedWatchWire(p0) = wire;
        Ok(RejectedWatch(<WatchRejectionReason as WireConversion>::try_from_wire(p0)?))
    }
}
impl WireConversion for CacheRetentionTransitionEvent {
    type Wire = CacheRetentionTransitionEventWire;
    fn into_wire(self) -> Self::Wire {
        let CacheRetentionTransitionEvent(p0, p1, p2, p3, p4, p5, p6, p7) = self;
        CacheRetentionTransitionEventWire(
            p0,
            p1.to_string(),
            p2.to_string(),
            <CacheRetentionTransition as WireConversion>::into_wire(p3),
            <GenerationSlot as WireConversion>::into_wire(p4),
            p5.map(|value| <GenerationSlot as WireConversion>::into_wire(value)),
            p6.map(|value| value.to_string()),
            p7,
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let CacheRetentionTransitionEventWire(p0, p1, p2, p3, p4, p5, p6, p7) = wire;
        Ok(
            CacheRetentionTransitionEvent(
                Ok(p0)?,
                protos::Text::try_from(p1).map_err(|_| WireFault::Text)?,
                protos::Text::try_from(p2).map_err(|_| WireFault::Text)?,
                <CacheRetentionTransition as WireConversion>::try_from_wire(p3)?,
                <GenerationSlot as WireConversion>::try_from_wire(p4)?,
                p5
                    .map(|value| <GenerationSlot as WireConversion>::try_from_wire(
                        value,
                    ))
                    .transpose()?,
                p6
                    .map(|value| {
                        protos::Text::try_from(value).map_err(|_| WireFault::Text)
                    })
                    .transpose()?,
                Ok(p7)?,
            ),
        )
    }
}
impl WireConversion for DeploymentInputMode {
    type Wire = DeploymentInputModeWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            DeploymentInputMode::Horizon => DeploymentInputModeWire::Horizon,
            DeploymentInputMode::Direct => DeploymentInputModeWire::Direct,
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            DeploymentInputModeWire::Horizon => Ok(DeploymentInputMode::Horizon),
            DeploymentInputModeWire::Direct => Ok(DeploymentInputMode::Direct),
        }
    }
}
impl WireConversion for TestRunPhase {
    type Wire = TestRunPhaseWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            TestRunPhase::Submitted => TestRunPhaseWire::Submitted,
            TestRunPhase::BringingUp => TestRunPhaseWire::BringingUp,
            TestRunPhase::TearingDown => TestRunPhaseWire::TearingDown,
            TestRunPhase::Completed => TestRunPhaseWire::Completed,
            TestRunPhase::Deploying => TestRunPhaseWire::Deploying,
            TestRunPhase::Asserting => TestRunPhaseWire::Asserting,
            TestRunPhase::Failed => TestRunPhaseWire::Failed,
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            TestRunPhaseWire::Submitted => Ok(TestRunPhase::Submitted),
            TestRunPhaseWire::BringingUp => Ok(TestRunPhase::BringingUp),
            TestRunPhaseWire::TearingDown => Ok(TestRunPhase::TearingDown),
            TestRunPhaseWire::Completed => Ok(TestRunPhase::Completed),
            TestRunPhaseWire::Deploying => Ok(TestRunPhase::Deploying),
            TestRunPhaseWire::Asserting => Ok(TestRunPhase::Asserting),
            TestRunPhaseWire::Failed => Ok(TestRunPhase::Failed),
        }
    }
}
impl WireConversion for DeploymentTransport {
    type Wire = DeploymentTransportWire;
    fn into_wire(self) -> Self::Wire {
        let DeploymentTransport(p0, p1) = self;
        DeploymentTransportWire(p0.to_string(), p1.to_string())
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let DeploymentTransportWire(p0, p1) = wire;
        Ok(
            DeploymentTransport(
                protos::Text::try_from(p0).map_err(|_| WireFault::Text)?,
                protos::Text::try_from(p1).map_err(|_| WireFault::Text)?,
            ),
        )
    }
}
impl WireConversion for TestExecutionProfile {
    type Wire = TestExecutionProfileWire;
    fn into_wire(self) -> Self::Wire {
        let TestExecutionProfile(p0, p1, p2, p3) = self;
        TestExecutionProfileWire(
            <TestMode as WireConversion>::into_wire(p0),
            p1.to_string(),
            <DeploymentOutputSelector as WireConversion>::into_wire(p2),
            p3.map(|value| <DeploymentTransport as WireConversion>::into_wire(value)),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let TestExecutionProfileWire(p0, p1, p2, p3) = wire;
        Ok(
            TestExecutionProfile(
                <TestMode as WireConversion>::try_from_wire(p0)?,
                protos::Text::try_from(p1).map_err(|_| WireFault::Text)?,
                <DeploymentOutputSelector as WireConversion>::try_from_wire(p2)?,
                p3
                    .map(|value| <DeploymentTransport as WireConversion>::try_from_wire(
                        value,
                    ))
                    .transpose()?,
            ),
        )
    }
}
impl WireConversion for SubscriptionOpened {
    type Wire = SubscriptionOpenedWire;
    fn into_wire(self) -> Self::Wire {
        let SubscriptionOpened(p0, p1) = self;
        SubscriptionOpenedWire(p0, p1)
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let SubscriptionOpenedWire(p0, p1) = wire;
        Ok(SubscriptionOpened(Ok(p0)?, Ok(p1)?))
    }
}
impl WireConversion for WatchRejectionReason {
    type Wire = WatchRejectionReasonWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            WatchRejectionReason::MalformedWatch => {
                WatchRejectionReasonWire::MalformedWatch
            }
            WatchRejectionReason::SubscriptionLimitReached => {
                WatchRejectionReasonWire::SubscriptionLimitReached
            }
            WatchRejectionReason::StreamUnavailable => {
                WatchRejectionReasonWire::StreamUnavailable
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            WatchRejectionReasonWire::MalformedWatch => {
                Ok(WatchRejectionReason::MalformedWatch)
            }
            WatchRejectionReasonWire::SubscriptionLimitReached => {
                Ok(WatchRejectionReason::SubscriptionLimitReached)
            }
            WatchRejectionReasonWire::StreamUnavailable => {
                Ok(WatchRejectionReason::StreamUnavailable)
            }
        }
    }
}
impl WireConversion for ActivationBackend {
    type Wire = ActivationBackendWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            ActivationBackend::HomeManagerNixProfileV1 => {
                ActivationBackendWire::HomeManagerNixProfileV1
            }
            ActivationBackend::NixosSystemdBootV1 => {
                ActivationBackendWire::NixosSystemdBootV1
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            ActivationBackendWire::HomeManagerNixProfileV1 => {
                Ok(ActivationBackend::HomeManagerNixProfileV1)
            }
            ActivationBackendWire::NixosSystemdBootV1 => {
                Ok(ActivationBackend::NixosSystemdBootV1)
            }
        }
    }
}
impl WireConversion for GenerationArtifact {
    type Wire = GenerationArtifactWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            GenerationArtifact::BaseHost => GenerationArtifactWire::BaseHost,
            GenerationArtifact::CompleteHost => GenerationArtifactWire::CompleteHost,
            GenerationArtifact::UserEnvironment => {
                GenerationArtifactWire::UserEnvironment
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            GenerationArtifactWire::BaseHost => Ok(GenerationArtifact::BaseHost),
            GenerationArtifactWire::CompleteHost => Ok(GenerationArtifact::CompleteHost),
            GenerationArtifactWire::UserEnvironment => {
                Ok(GenerationArtifact::UserEnvironment)
            }
        }
    }
}
impl WireConversion for RejectedUnwatch {
    type Wire = RejectedUnwatchWire;
    fn into_wire(self) -> Self::Wire {
        let RejectedUnwatch(p0, p1) = self;
        RejectedUnwatchWire(
            <UnwatchRejectionReason as WireConversion>::into_wire(p0),
            p1,
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let RejectedUnwatchWire(p0, p1) = wire;
        Ok(
            RejectedUnwatch(
                <UnwatchRejectionReason as WireConversion>::try_from_wire(p0)?,
                Ok(p1)?,
            ),
        )
    }
}
impl WireConversion for EventLogPage {
    type Wire = EventLogPageWire;
    fn into_wire(self) -> Self::Wire {
        let EventLogPage(p0, p1, p2) = self;
        EventLogPageWire(
            p0
                .into_iter()
                .map(|value| <DeploymentPhaseEvent as WireConversion>::into_wire(value))
                .collect(),
            p1
                .into_iter()
                .map(|value| <CacheRetentionTransitionEvent as WireConversion>::into_wire(
                    value,
                ))
                .collect(),
            <DatabaseMarker as WireConversion>::into_wire(p2),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let EventLogPageWire(p0, p1, p2) = wire;
        Ok(
            EventLogPage(
                p0
                    .into_iter()
                    .map(|value| <DeploymentPhaseEvent as WireConversion>::try_from_wire(
                        value,
                    ))
                    .collect::<std::result::Result<std::vec::Vec<_>, WireFault>>()?,
                p1
                    .into_iter()
                    .map(|value| <CacheRetentionTransitionEvent as WireConversion>::try_from_wire(
                        value,
                    ))
                    .collect::<std::result::Result<std::vec::Vec<_>, WireFault>>()?,
                <DatabaseMarker as WireConversion>::try_from_wire(p2)?,
            ),
        )
    }
}
impl WireConversion for SubscriptionClose {
    type Wire = SubscriptionCloseWire;
    fn into_wire(self) -> Self::Wire {
        let SubscriptionClose(p0) = self;
        SubscriptionCloseWire(p0)
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let SubscriptionCloseWire(p0) = wire;
        Ok(SubscriptionClose(Ok(p0)?))
    }
}
impl WireConversion for DeploymentEnvironment {
    type Wire = DeploymentEnvironmentWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            DeploymentEnvironment::HostEnvironment => {
                DeploymentEnvironmentWire::HostEnvironment
            }
            DeploymentEnvironment::UserEnvironment(value) => {
                DeploymentEnvironmentWire::UserEnvironment(value.to_string())
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            DeploymentEnvironmentWire::HostEnvironment => {
                Ok(DeploymentEnvironment::HostEnvironment)
            }
            DeploymentEnvironmentWire::UserEnvironment(value) => {
                Ok(
                    DeploymentEnvironment::UserEnvironment(
                        protos::Text::try_from(value).map_err(|_| WireFault::Text)?,
                    ),
                )
            }
        }
    }
}
impl WireConversion for HostSelection {
    type Wire = HostSelectionWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            HostSelection::OnHost(value) => HostSelectionWire::OnHost(value.to_string()),
            HostSelection::DefaultHost => HostSelectionWire::DefaultHost,
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            HostSelectionWire::OnHost(value) => {
                Ok(
                    HostSelection::OnHost(
                        protos::Text::try_from(value).map_err(|_| WireFault::Text)?,
                    ),
                )
            }
            HostSelectionWire::DefaultHost => Ok(HostSelection::DefaultHost),
        }
    }
}
impl WireConversion for EventLogRange {
    type Wire = EventLogRangeWire;
    fn into_wire(self) -> Self::Wire {
        let EventLogRange(p0, p1) = self;
        EventLogRangeWire(p0, p1)
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let EventLogRangeWire(p0, p1) = wire;
        Ok(EventLogRange(Ok(p0)?, Ok(p1)?))
    }
}
impl WireConversion for DeploymentLifecycle {
    type Wire = DeploymentLifecycleWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            DeploymentLifecycle::Failed => DeploymentLifecycleWire::Failed,
            DeploymentLifecycle::Rejected => DeploymentLifecycleWire::Rejected,
            DeploymentLifecycle::Completed => DeploymentLifecycleWire::Completed,
            DeploymentLifecycle::Building => DeploymentLifecycleWire::Building,
            DeploymentLifecycle::Activating => DeploymentLifecycleWire::Activating,
            DeploymentLifecycle::Submitted => DeploymentLifecycleWire::Submitted,
            DeploymentLifecycle::Copying => DeploymentLifecycleWire::Copying,
            DeploymentLifecycle::Activated => DeploymentLifecycleWire::Activated,
            DeploymentLifecycle::Built => DeploymentLifecycleWire::Built,
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            DeploymentLifecycleWire::Failed => Ok(DeploymentLifecycle::Failed),
            DeploymentLifecycleWire::Rejected => Ok(DeploymentLifecycle::Rejected),
            DeploymentLifecycleWire::Completed => Ok(DeploymentLifecycle::Completed),
            DeploymentLifecycleWire::Building => Ok(DeploymentLifecycle::Building),
            DeploymentLifecycleWire::Activating => Ok(DeploymentLifecycle::Activating),
            DeploymentLifecycleWire::Submitted => Ok(DeploymentLifecycle::Submitted),
            DeploymentLifecycleWire::Copying => Ok(DeploymentLifecycle::Copying),
            DeploymentLifecycleWire::Activated => Ok(DeploymentLifecycle::Activated),
            DeploymentLifecycleWire::Built => Ok(DeploymentLifecycle::Built),
        }
    }
}
impl WireConversion for DeploymentOutputSelector {
    type Wire = DeploymentOutputSelectorWire;
    fn into_wire(self) -> Self::Wire {
        let DeploymentOutputSelector(p0) = self;
        DeploymentOutputSelectorWire(p0.to_string())
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let DeploymentOutputSelectorWire(p0) = wire;
        Ok(
            DeploymentOutputSelector(
                protos::Text::try_from(p0).map_err(|_| WireFault::Text)?,
            ),
        )
    }
}
impl WireConversion for ActivationEffect {
    type Wire = ActivationEffectWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            ActivationEffect::ProfileOnly => ActivationEffectWire::ProfileOnly,
            ActivationEffect::BootOnceProfile => ActivationEffectWire::BootOnceProfile,
            ActivationEffect::TestActivation => ActivationEffectWire::TestActivation,
            ActivationEffect::LiveActivation => ActivationEffectWire::LiveActivation,
            ActivationEffect::BootProfile => ActivationEffectWire::BootProfile,
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            ActivationEffectWire::ProfileOnly => Ok(ActivationEffect::ProfileOnly),
            ActivationEffectWire::BootOnceProfile => {
                Ok(ActivationEffect::BootOnceProfile)
            }
            ActivationEffectWire::TestActivation => Ok(ActivationEffect::TestActivation),
            ActivationEffectWire::LiveActivation => Ok(ActivationEffect::LiveActivation),
            ActivationEffectWire::BootProfile => Ok(ActivationEffect::BootProfile),
        }
    }
}
impl WireConversion for SubscriptionClosed {
    type Wire = SubscriptionClosedWire;
    fn into_wire(self) -> Self::Wire {
        let SubscriptionClosed(p0) = self;
        SubscriptionClosedWire(p0)
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let SubscriptionClosedWire(p0) = wire;
        Ok(SubscriptionClosed(Ok(p0)?))
    }
}
impl WireConversion for TestMode {
    type Wire = TestModeWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            TestMode::Hermetic => TestModeWire::Hermetic,
            TestMode::Live => TestModeWire::Live,
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            TestModeWire::Hermetic => Ok(TestMode::Hermetic),
            TestModeWire::Live => Ok(TestMode::Live),
        }
    }
}
impl WireConversion for QueryRejectionReason {
    type Wire = QueryRejectionReasonWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            QueryRejectionReason::MalformedSelector => {
                QueryRejectionReasonWire::MalformedSelector
            }
            QueryRejectionReason::EventLogPositionOutOfRange => {
                QueryRejectionReasonWire::EventLogPositionOutOfRange
            }
            QueryRejectionReason::GenerationUnknown => {
                QueryRejectionReasonWire::GenerationUnknown
            }
            QueryRejectionReason::NodeUnknown => QueryRejectionReasonWire::NodeUnknown,
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            QueryRejectionReasonWire::MalformedSelector => {
                Ok(QueryRejectionReason::MalformedSelector)
            }
            QueryRejectionReasonWire::EventLogPositionOutOfRange => {
                Ok(QueryRejectionReason::EventLogPositionOutOfRange)
            }
            QueryRejectionReasonWire::GenerationUnknown => {
                Ok(QueryRejectionReason::GenerationUnknown)
            }
            QueryRejectionReasonWire::NodeUnknown => {
                Ok(QueryRejectionReason::NodeUnknown)
            }
        }
    }
}
impl WireConversion for TestRunListing {
    type Wire = TestRunListingWire;
    fn into_wire(self) -> Self::Wire {
        let TestRunListing(p0, p1) = self;
        TestRunListingWire(
            p0
                .into_iter()
                .map(|value| <TestRunRecord as WireConversion>::into_wire(value))
                .collect(),
            <DatabaseMarker as WireConversion>::into_wire(p1),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let TestRunListingWire(p0, p1) = wire;
        Ok(
            TestRunListing(
                p0
                    .into_iter()
                    .map(|value| <TestRunRecord as WireConversion>::try_from_wire(value))
                    .collect::<std::result::Result<std::vec::Vec<_>, WireFault>>()?,
                <DatabaseMarker as WireConversion>::try_from_wire(p1)?,
            ),
        )
    }
}
impl WireConversion for DeploymentTerminal {
    type Wire = DeploymentTerminalWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            DeploymentTerminal::Failed(value) => {
                DeploymentTerminalWire::Failed(
                    <DeploymentFailure as WireConversion>::into_wire(value),
                )
            }
            DeploymentTerminal::Rejected(value) => {
                DeploymentTerminalWire::Rejected(
                    <DeploymentTerminalReason as WireConversion>::into_wire(value),
                )
            }
            DeploymentTerminal::Succeeded => DeploymentTerminalWire::Succeeded,
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            DeploymentTerminalWire::Failed(value) => {
                Ok(
                    DeploymentTerminal::Failed(
                        <DeploymentFailure as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            DeploymentTerminalWire::Rejected(value) => {
                Ok(
                    DeploymentTerminal::Rejected(
                        <DeploymentTerminalReason as WireConversion>::try_from_wire(
                            value,
                        )?,
                    ),
                )
            }
            DeploymentTerminalWire::Succeeded => Ok(DeploymentTerminal::Succeeded),
        }
    }
}
impl WireConversion for SourceRevisionPolicy {
    type Wire = SourceRevisionPolicyWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            SourceRevisionPolicy::ResolveAndRecord => {
                SourceRevisionPolicyWire::ResolveAndRecord
            }
            SourceRevisionPolicy::RequireImmutable => {
                SourceRevisionPolicyWire::RequireImmutable
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            SourceRevisionPolicyWire::ResolveAndRecord => {
                Ok(SourceRevisionPolicy::ResolveAndRecord)
            }
            SourceRevisionPolicyWire::RequireImmutable => {
                Ok(SourceRevisionPolicy::RequireImmutable)
            }
        }
    }
}
impl WireConversion for KeyMaterialQuery {
    type Wire = KeyMaterialQueryWire;
    fn into_wire(self) -> Self::Wire {
        let KeyMaterialQuery(p0, p1, p2) = self;
        KeyMaterialQueryWire(p0.to_string(), p1.to_string(), p2.to_string())
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let KeyMaterialQueryWire(p0, p1, p2) = wire;
        Ok(
            KeyMaterialQuery(
                protos::Text::try_from(p0).map_err(|_| WireFault::Text)?,
                protos::Text::try_from(p1).map_err(|_| WireFault::Text)?,
                protos::Text::try_from(p2).map_err(|_| WireFault::Text)?,
            ),
        )
    }
}
impl WireConversion for RequestedGenerationArtifact {
    type Wire = RequestedGenerationArtifactWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            RequestedGenerationArtifact::UserEnvironment => {
                RequestedGenerationArtifactWire::UserEnvironment
            }
            RequestedGenerationArtifact::CompleteHost => {
                RequestedGenerationArtifactWire::CompleteHost
            }
            RequestedGenerationArtifact::BaseHost => {
                RequestedGenerationArtifactWire::BaseHost
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            RequestedGenerationArtifactWire::UserEnvironment => {
                Ok(RequestedGenerationArtifact::UserEnvironment)
            }
            RequestedGenerationArtifactWire::CompleteHost => {
                Ok(RequestedGenerationArtifact::CompleteHost)
            }
            RequestedGenerationArtifactWire::BaseHost => {
                Ok(RequestedGenerationArtifact::BaseHost)
            }
        }
    }
}
impl WireConversion for DeploymentFailure {
    type Wire = DeploymentFailureWire;
    fn into_wire(self) -> Self::Wire {
        let DeploymentFailure(p0, p1) = self;
        DeploymentFailureWire(
            <DeploymentFailureStage as WireConversion>::into_wire(p0),
            <DeploymentTerminalReason as WireConversion>::into_wire(p1),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let DeploymentFailureWire(p0, p1) = wire;
        Ok(
            DeploymentFailure(
                <DeploymentFailureStage as WireConversion>::try_from_wire(p0)?,
                <DeploymentTerminalReason as WireConversion>::try_from_wire(p1)?,
            ),
        )
    }
}
impl WireConversion for DeploymentFailureStage {
    type Wire = DeploymentFailureStageWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            DeploymentFailureStage::Build => DeploymentFailureStageWire::Build,
            DeploymentFailureStage::Eval => DeploymentFailureStageWire::Eval,
            DeploymentFailureStage::MaterializeHorizon => {
                DeploymentFailureStageWire::MaterializeHorizon
            }
            DeploymentFailureStage::Daemon => DeploymentFailureStageWire::Daemon,
            DeploymentFailureStage::Activate => DeploymentFailureStageWire::Activate,
            DeploymentFailureStage::CopyClosure => {
                DeploymentFailureStageWire::CopyClosure
            }
            DeploymentFailureStage::Admission => DeploymentFailureStageWire::Admission,
            DeploymentFailureStage::FlakeAuth => DeploymentFailureStageWire::FlakeAuth,
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            DeploymentFailureStageWire::Build => Ok(DeploymentFailureStage::Build),
            DeploymentFailureStageWire::Eval => Ok(DeploymentFailureStage::Eval),
            DeploymentFailureStageWire::MaterializeHorizon => {
                Ok(DeploymentFailureStage::MaterializeHorizon)
            }
            DeploymentFailureStageWire::Daemon => Ok(DeploymentFailureStage::Daemon),
            DeploymentFailureStageWire::Activate => Ok(DeploymentFailureStage::Activate),
            DeploymentFailureStageWire::CopyClosure => {
                Ok(DeploymentFailureStage::CopyClosure)
            }
            DeploymentFailureStageWire::Admission => {
                Ok(DeploymentFailureStage::Admission)
            }
            DeploymentFailureStageWire::FlakeAuth => {
                Ok(DeploymentFailureStage::FlakeAuth)
            }
        }
    }
}
impl WireConversion for Generation {
    type Wire = GenerationWire;
    fn into_wire(self) -> Self::Wire {
        let Generation(p0, p1, p2, p3, p4, p5, p6, p7, p8) = self;
        GenerationWire(
            p0,
            p1,
            p2.to_string(),
            p3.to_string(),
            <GenerationArtifact as WireConversion>::into_wire(p4),
            <ActivationEffect as WireConversion>::into_wire(p5),
            <GenerationSlot as WireConversion>::into_wire(p6),
            p7.map(|value| value.to_string()),
            p8.map(|value| value.to_string()),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let GenerationWire(p0, p1, p2, p3, p4, p5, p6, p7, p8) = wire;
        Ok(
            Generation(
                Ok(p0)?,
                Ok(p1)?,
                protos::Text::try_from(p2).map_err(|_| WireFault::Text)?,
                protos::Text::try_from(p3).map_err(|_| WireFault::Text)?,
                <GenerationArtifact as WireConversion>::try_from_wire(p4)?,
                <ActivationEffect as WireConversion>::try_from_wire(p5)?,
                <GenerationSlot as WireConversion>::try_from_wire(p6)?,
                p7
                    .map(|value| {
                        protos::Text::try_from(value).map_err(|_| WireFault::Text)
                    })
                    .transpose()?,
                p8
                    .map(|value| {
                        protos::Text::try_from(value).map_err(|_| WireFault::Text)
                    })
                    .transpose()?,
            ),
        )
    }
}
impl WireConversion for RejectedKeyMaterialCheck {
    type Wire = RejectedKeyMaterialCheckWire;
    fn into_wire(self) -> Self::Wire {
        let RejectedKeyMaterialCheck(p0, p1) = self;
        RejectedKeyMaterialCheckWire(
            <KeyMaterialCheckRejectionReason as WireConversion>::into_wire(p0),
            <DatabaseMarker as WireConversion>::into_wire(p1),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let RejectedKeyMaterialCheckWire(p0, p1) = wire;
        Ok(
            RejectedKeyMaterialCheck(
                <KeyMaterialCheckRejectionReason as WireConversion>::try_from_wire(p0)?,
                <DatabaseMarker as WireConversion>::try_from_wire(p1)?,
            ),
        )
    }
}
impl WireConversion for DeploymentTerminalReason {
    type Wire = DeploymentTerminalReasonWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            DeploymentTerminalReason::NodeUnknown => {
                DeploymentTerminalReasonWire::NodeUnknown
            }
            DeploymentTerminalReason::FlakeReferenceMalformed => {
                DeploymentTerminalReasonWire::FlakeReferenceMalformed
            }
            DeploymentTerminalReason::ProposalSourceUnreachable => {
                DeploymentTerminalReasonWire::ProposalSourceUnreachable
            }
            DeploymentTerminalReason::DeploymentInFlight => {
                DeploymentTerminalReasonWire::DeploymentInFlight
            }
            DeploymentTerminalReason::InvalidDeploymentRouting => {
                DeploymentTerminalReasonWire::InvalidDeploymentRouting
            }
            DeploymentTerminalReason::UnsupportedDeployAction => {
                DeploymentTerminalReasonWire::UnsupportedDeployAction
            }
            DeploymentTerminalReason::InternalError => {
                DeploymentTerminalReasonWire::InternalError
            }
            DeploymentTerminalReason::ClusterUnknown => {
                DeploymentTerminalReasonWire::ClusterUnknown
            }
            DeploymentTerminalReason::ActivationFailed => {
                DeploymentTerminalReasonWire::ActivationFailed
            }
            DeploymentTerminalReason::BuilderUnreachable => {
                DeploymentTerminalReasonWire::BuilderUnreachable
            }
            DeploymentTerminalReason::SubstituterUnreachable => {
                DeploymentTerminalReasonWire::SubstituterUnreachable
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            DeploymentTerminalReasonWire::NodeUnknown => {
                Ok(DeploymentTerminalReason::NodeUnknown)
            }
            DeploymentTerminalReasonWire::FlakeReferenceMalformed => {
                Ok(DeploymentTerminalReason::FlakeReferenceMalformed)
            }
            DeploymentTerminalReasonWire::ProposalSourceUnreachable => {
                Ok(DeploymentTerminalReason::ProposalSourceUnreachable)
            }
            DeploymentTerminalReasonWire::DeploymentInFlight => {
                Ok(DeploymentTerminalReason::DeploymentInFlight)
            }
            DeploymentTerminalReasonWire::InvalidDeploymentRouting => {
                Ok(DeploymentTerminalReason::InvalidDeploymentRouting)
            }
            DeploymentTerminalReasonWire::UnsupportedDeployAction => {
                Ok(DeploymentTerminalReason::UnsupportedDeployAction)
            }
            DeploymentTerminalReasonWire::InternalError => {
                Ok(DeploymentTerminalReason::InternalError)
            }
            DeploymentTerminalReasonWire::ClusterUnknown => {
                Ok(DeploymentTerminalReason::ClusterUnknown)
            }
            DeploymentTerminalReasonWire::ActivationFailed => {
                Ok(DeploymentTerminalReason::ActivationFailed)
            }
            DeploymentTerminalReasonWire::BuilderUnreachable => {
                Ok(DeploymentTerminalReason::BuilderUnreachable)
            }
            DeploymentTerminalReasonWire::SubstituterUnreachable => {
                Ok(DeploymentTerminalReason::SubstituterUnreachable)
            }
        }
    }
}
impl WireConversion for Selection {
    type Wire = SelectionWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            Selection::ByNode(value) => {
                SelectionWire::ByNode(<NodeSelector as WireConversion>::into_wire(value))
            }
            Selection::ByTestRun(value) => {
                SelectionWire::ByTestRun(
                    <TestRunLookup as WireConversion>::into_wire(value),
                )
            }
            Selection::ByDeployment(value) => {
                SelectionWire::ByDeployment(
                    <DeploymentLookup as WireConversion>::into_wire(value),
                )
            }
            Selection::ByGeneration(value) => {
                SelectionWire::ByGeneration(
                    <GenerationLookup as WireConversion>::into_wire(value),
                )
            }
            Selection::ByEventLog(value) => {
                SelectionWire::ByEventLog(
                    <EventLogRange as WireConversion>::into_wire(value),
                )
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            SelectionWire::ByNode(value) => {
                Ok(
                    Selection::ByNode(
                        <NodeSelector as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            SelectionWire::ByTestRun(value) => {
                Ok(
                    Selection::ByTestRun(
                        <TestRunLookup as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            SelectionWire::ByDeployment(value) => {
                Ok(
                    Selection::ByDeployment(
                        <DeploymentLookup as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            SelectionWire::ByGeneration(value) => {
                Ok(
                    Selection::ByGeneration(
                        <GenerationLookup as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            SelectionWire::ByEventLog(value) => {
                Ok(
                    Selection::ByEventLog(
                        <EventLogRange as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
        }
    }
}
impl WireConversion for Request {
    type Wire = RequestWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            Request::CheckHostKeyMaterial(value) => {
                RequestWire::CheckHostKeyMaterial(
                    <KeyMaterialQuery as WireConversion>::into_wire(value),
                )
            }
            Request::WatchDeployments(value) => {
                RequestWire::WatchDeployments(
                    <DeploymentWatch as WireConversion>::into_wire(value),
                )
            }
            Request::Query(value) => {
                RequestWire::Query(<Selection as WireConversion>::into_wire(value))
            }
            Request::WatchCacheRetention(value) => {
                RequestWire::WatchCacheRetention(
                    <CacheRetentionWatch as WireConversion>::into_wire(value),
                )
            }
            Request::Unwatch(value) => {
                RequestWire::Unwatch(
                    <SubscriptionClose as WireConversion>::into_wire(value),
                )
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            RequestWire::CheckHostKeyMaterial(value) => {
                Ok(
                    Request::CheckHostKeyMaterial(
                        <KeyMaterialQuery as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            RequestWire::WatchDeployments(value) => {
                Ok(
                    Request::WatchDeployments(
                        <DeploymentWatch as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            RequestWire::Query(value) => {
                Ok(Request::Query(<Selection as WireConversion>::try_from_wire(value)?))
            }
            RequestWire::WatchCacheRetention(value) => {
                Ok(
                    Request::WatchCacheRetention(
                        <CacheRetentionWatch as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            RequestWire::Unwatch(value) => {
                Ok(
                    Request::Unwatch(
                        <SubscriptionClose as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
        }
    }
}
impl WireConversion for Response {
    type Wire = ResponseWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            Response::TestRunsQueried(value) => {
                ResponseWire::TestRunsQueried(
                    <TestRunListing as WireConversion>::into_wire(value),
                )
            }
            Response::UnwatchRejected(value) => {
                ResponseWire::UnwatchRejected(
                    <RejectedUnwatch as WireConversion>::into_wire(value),
                )
            }
            Response::QueryRejected(value) => {
                ResponseWire::QueryRejected(
                    <RejectedQuery as WireConversion>::into_wire(value),
                )
            }
            Response::Watching(value) => {
                ResponseWire::Watching(
                    <SubscriptionOpened as WireConversion>::into_wire(value),
                )
            }
            Response::KeyMaterialCheckRejected(value) => {
                ResponseWire::KeyMaterialCheckRejected(
                    <RejectedKeyMaterialCheck as WireConversion>::into_wire(value),
                )
            }
            Response::Queried(value) => {
                ResponseWire::Queried(
                    <GenerationListing as WireConversion>::into_wire(value),
                )
            }
            Response::DeploymentEventsQueried(value) => {
                ResponseWire::DeploymentEventsQueried(
                    <EventLogPage as WireConversion>::into_wire(value),
                )
            }
            Response::Unwatched(value) => {
                ResponseWire::Unwatched(
                    <SubscriptionClosed as WireConversion>::into_wire(value),
                )
            }
            Response::KeyMaterialChecked(value) => {
                ResponseWire::KeyMaterialChecked(
                    <KeyMaterialReport as WireConversion>::into_wire(value),
                )
            }
            Response::WatchRejected(value) => {
                ResponseWire::WatchRejected(
                    <RejectedWatch as WireConversion>::into_wire(value),
                )
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            ResponseWire::TestRunsQueried(value) => {
                Ok(
                    Response::TestRunsQueried(
                        <TestRunListing as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            ResponseWire::UnwatchRejected(value) => {
                Ok(
                    Response::UnwatchRejected(
                        <RejectedUnwatch as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            ResponseWire::QueryRejected(value) => {
                Ok(
                    Response::QueryRejected(
                        <RejectedQuery as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            ResponseWire::Watching(value) => {
                Ok(
                    Response::Watching(
                        <SubscriptionOpened as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            ResponseWire::KeyMaterialCheckRejected(value) => {
                Ok(
                    Response::KeyMaterialCheckRejected(
                        <RejectedKeyMaterialCheck as WireConversion>::try_from_wire(
                            value,
                        )?,
                    ),
                )
            }
            ResponseWire::Queried(value) => {
                Ok(
                    Response::Queried(
                        <GenerationListing as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            ResponseWire::DeploymentEventsQueried(value) => {
                Ok(
                    Response::DeploymentEventsQueried(
                        <EventLogPage as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            ResponseWire::Unwatched(value) => {
                Ok(
                    Response::Unwatched(
                        <SubscriptionClosed as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            ResponseWire::KeyMaterialChecked(value) => {
                Ok(
                    Response::KeyMaterialChecked(
                        <KeyMaterialReport as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            ResponseWire::WatchRejected(value) => {
                Ok(
                    Response::WatchRejected(
                        <RejectedWatch as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
        }
    }
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum RequestWire {
    CheckHostKeyMaterial(CheckHostKeyMaterialPayloadWire),
    WatchDeployments(WatchDeploymentsPayloadWire),
    Query(QueryPayloadWire),
    WatchCacheRetention(WatchCacheRetentionPayloadWire),
    Unwatch(UnwatchPayloadWire),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ResponseWire {
    TestRunsQueried(TestRunsQueriedPayloadWire),
    UnwatchRejected(UnwatchRejectedPayloadWire),
    QueryRejected(QueryRejectedPayloadWire),
    Watching(WatchingPayloadWire),
    KeyMaterialCheckRejected(KeyMaterialCheckRejectedPayloadWire),
    Queried(QueriedPayloadWire),
    DeploymentEventsQueried(DeploymentEventsQueriedPayloadWire),
    Unwatched(UnwatchedPayloadWire),
    KeyMaterialChecked(KeyMaterialCheckedPayloadWire),
    WatchRejected(WatchRejectedPayloadWire),
}
