// Handwritten operational behavior for the authority-verified ordinary Lojix Interface.
//
// The strict bootstrap projection owns every structural type below. This file
// owns only behavior the current bootstrap language cannot yet express:
// structural runtime traits, the ordinary Input/Output role seating, and the
// allocated Signal frame boundary.

use rkyv::{
    Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize,
    rancor::Source as _,
};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
#[rkyv(serialize_bounds(
    __S: rkyv::ser::Writer + rkyv::ser::Allocator,
    __S::Error: rkyv::rancor::Source,
))]
#[rkyv(deserialize_bounds(__D::Error: rkyv::rancor::Source))]
#[rkyv(bytecheck(bounds(__C: rkyv::validation::ArchiveContext)))]
#[doc(hidden)]
pub enum WireValue {
    Text(std::string::String), Integer(u64), Boolean(bool),
    Sequence(#[rkyv(omit_bounds)] Vec<WireValue>),
    Absent, Present(#[rkyv(omit_bounds)] Box<WireValue>),
    Product(#[rkyv(omit_bounds)] Vec<WireValue>),
    Variant { ordinal: u16, #[rkyv(omit_bounds)] fields: Vec<WireValue> },
}

#[derive(Debug, thiserror::Error)]
#[error("structural wire value does not match the authority-verified Interface")]
#[doc(hidden)]
pub struct WireShapeError;

/// Current-stage structural behavior shared by Interfaces that import these
/// producer-owned types.
#[doc(hidden)]
pub trait WireShape: Sized {
    fn to_wire(&self) -> WireValue;
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError>;
}

impl WireShape for std::string::String {
    fn to_wire(&self) -> WireValue { WireValue::Text(self.clone()) }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { match value { WireValue::Text(value) => Ok(value), _ => Err(WireShapeError) } }
}
impl WireShape for u64 {
    fn to_wire(&self) -> WireValue { WireValue::Integer(*self) }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { match value { WireValue::Integer(value) => Ok(value), _ => Err(WireShapeError) } }
}
impl WireShape for bool {
    fn to_wire(&self) -> WireValue { WireValue::Boolean(*self) }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { match value { WireValue::Boolean(value) => Ok(value), _ => Err(WireShapeError) } }
}
impl<Value: WireShape> WireShape for Vec<Value> {
    fn to_wire(&self) -> WireValue { WireValue::Sequence(self.iter().map(WireShape::to_wire).collect()) }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
        let WireValue::Sequence(values) = value else { return Err(WireShapeError) };
        values.into_iter().map(Value::from_wire).collect()
    }
}
impl<Value: WireShape> WireShape for Option<Value> {
    fn to_wire(&self) -> WireValue { match self { Some(value) => WireValue::Present(Box::new(value.to_wire())), None => WireValue::Absent } }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
        match value { WireValue::Present(value) => Ok(Some(Value::from_wire(*value)?)), WireValue::Absent => Ok(None), _ => Err(WireShapeError) }
    }
}
fn one_field(mut fields: Vec<WireValue>) -> Result<WireValue, WireShapeError> {
    if fields.len() != 1 { return Err(WireShapeError); }
    Ok(fields.pop().expect("one field checked"))
}

macro_rules! wire_traits {
    ($name:ident) => {
        impl Clone for $name { fn clone(&self) -> Self { Self::from_wire(self.to_wire()).expect("a projected value revalidates") } }
        impl std::fmt::Debug for $name { fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.to_wire().fmt(formatter) } }
        impl PartialEq for $name { fn eq(&self, other: &Self) -> bool { self.to_wire() == other.to_wire() } }
        impl Eq for $name {}
    };
}
macro_rules! wire_external_newtype {
    ($name:ident, $inner:ty) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { self.payload().to_wire() }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { Ok(Self::new(<$inner as WireShape>::from_wire(value)?)) }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::DotosEncode::to_dotos(self.payload())
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self::new)
            }
        }
    };
}
macro_rules! wire_newtype {
    ($name:ident, $inner:ty) => {
        impl $name {
            pub fn new(payload: $inner) -> Self { Self(payload) }
            pub fn payload(&self) -> &$inner { &self.0 }
            pub fn into_payload(self) -> $inner { self.0 }
        }
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { self.0.to_wire() }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { Ok(Self(<$inner as WireShape>::from_wire(value)?)) }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::DotosEncode::to_dotos(&self.0)
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self)
            }
        }
    };
}
macro_rules! wire_struct {
    ($name:ident { $($field:ident: $field_type:ty),* $(,)? }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { WireValue::Product(vec![$(self.$field.to_wire()),*]) }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Product(fields) = value else { return Err(WireShapeError) };
                let mut fields = fields.into_iter();
                let result = Self { $($field: <$field_type as WireShape>::from_wire(fields.next().ok_or(WireShapeError)?)?),* };
                if fields.next().is_some() { return Err(WireShapeError); }
                Ok(result)
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::Delimiter::Parenthesis.wrap([
                    $(dotos::DotosEncode::to_dotos(&self.$field)),*
                ])
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                let body = dotos::DotosBody::from_delimited(
                    block,
                    dotos::Delimiter::Parenthesis,
                    stringify!($name),
                )?;
                let expected = [$(stringify!($field)),*].len();
                let mut fields = body.expect_fields(stringify!($name), expected)?.iter();
                Ok(Self {
                    $($field: <$field_type as dotos::DotosDecode>::from_dotos_block(
                        fields.next().expect("field count checked"),
                    )?),*
                })
            }
        }
    };
}
macro_rules! wire_enum {
    ($name:ident {
        unit { $($unit_ordinal:literal => $unit:ident : $unit_visible:literal),* $(,)? }
        unary { $($unary_ordinal:literal => $unary:ident($payload:ty) : $unary_visible:literal),* $(,)? }
    }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue {
                match self {
                    $(Self::$unit => WireValue::Variant { ordinal: $unit_ordinal, fields: Vec::new() },)*
                    $(Self::$unary(payload) => WireValue::Variant { ordinal: $unary_ordinal, fields: vec![payload.to_wire()] },)*
                }
            }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Variant { ordinal, fields } = value else { return Err(WireShapeError) };
                match ordinal {
                    $($unit_ordinal if fields.is_empty() => Ok(Self::$unit),)*
                    $($unary_ordinal => Ok(Self::$unary(<$payload as WireShape>::from_wire(one_field(fields)?)?)),)*
                    _ => Err(WireShapeError),
                }
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                match self {
                    $(Self::$unit => $unit_visible.to_owned(),)*
                    $(Self::$unary(payload) => format!(
                        "{}.{}",
                        $unary_visible,
                        dotos::DotosEncode::to_dotos(payload),
                    ),)*
                }
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                if let Some(variant) = block.demote_to_string() {
                    return match variant {
                        $($unit_visible => Ok(Self::$unit),)*
                        _ => Err(dotos::DotosDecodeError::UnknownVariant {
                            enum_name: stringify!($name),
                            variant: variant.to_owned(),
                        }),
                    };
                }
                let (head, payload) = block.as_application().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                let _ = &payload;
                let variant = head.demote_to_string().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                match variant {
                    $($unary_visible => Ok(Self::$unary(
                        <$payload as dotos::DotosDecode>::from_dotos_block(payload)?,
                    )),)*
                    _ => Err(dotos::DotosDecodeError::UnknownVariant {
                        enum_name: stringify!($name),
                        variant: variant.to_owned(),
                    }),
                }
            }
        }
    };
}
wire_enum!(z2VbPz { unit { 0 => z2VRmx : "ActivateNow", 1 => z2VLoi : "Realize", 2 => z2VUoE : "SetProfile" } unary {  } });
wire_newtype!(z2VLkY, z2VKvY);
wire_enum!(z2VcR1 { unit {  } unary { 0 => z2VS1B(z2VcB9) : "TestRunsQueried", 1 => z2VQom(z2VUqu) : "UnwatchRejected", 2 => z2VaL5(z2VRxu) : "QueryRejected", 3 => z2VZdT(z2VNmE) : "Watching", 4 => z2Vby6(z2Vb9C) : "KeyMaterialCheckRejected", 5 => z2VZXM(z2VW5R) : "Queried", 6 => z2VYcX(z2VRhr) : "DeploymentEventsQueried", 7 => z2VNFq(z2Ve15) : "Unwatched", 8 => z2VVBd(z2VWof) : "KeyMaterialChecked", 9 => z2VNoL(z2VUAE) : "WatchRejected" } });
wire_external_newtype!(z2VU8F, u64);
wire_enum!(z2VTki { unit { 0 => z2VLTU : "Demoted", 1 => z2VVvC : "Retired", 2 => z2Vbgy : "Pinned", 3 => z2Va9j : "Promoted", 4 => z2VdwX : "Unpinned", 5 => z2VUZc : "Evicted" } unary {  } });
wire_external_newtype!(z2VR89, u64);
wire_struct!(z2VRZj { field_0: z2Vdkm, field_1: z2VU8F, field_2: z2VXtV, field_3: z2VXGN, field_4: z2VVG2, field_5: z2VPWn, field_6: z2Vchw, field_7: Option<z2VQbS>, field_8: Option<z2VZQW> });
wire_struct!(z2VY5C { field_0: Option<z2Vdkm>, field_1: Option<z2VXtV>, field_2: Option<z2VXGN> });
wire_newtype!(z2VUAE, z2VdhM);
wire_external_newtype!(z2VYbZ, std::string::String);
wire_newtype!(z2VNmE, z2VeXy);
wire_enum!(z2VYJj { unit {  } unary { 0 => z2VdtZ(z2VZH3) : "Host", 1 => z2VbKj(z2VbPz) : "UserEnvironment" } });
wire_external_newtype!(z2VXGN, std::string::String);
wire_newtype!(z2VRxu, z2VSEh);
wire_struct!(z2VSEh { field_0: z2VcoS, field_1: z2VaUx });
wire_struct!(z2VZQZ { field_0: z2VU8F });
wire_enum!(z2VNtH { unit { 1 => z2VaJP : "Pending", 2 => z2VQCt : "Passed" } unary { 0 => z2VTqM(z2VWkd) : "Failed" } });
wire_enum!(z2VZ35 { unit { 0 => z2Vbes : "SecureShellPublicKey", 1 => z2VcDV : "YggdrasilPublicKey", 2 => z2VNZE : "YggdrasilAddress" } unary {  } });
wire_struct!(z2VYYM { field_0: z2VZ35, field_1: z2VQsT, field_2: z2VQsT, field_3: z2VMfj });
wire_enum!(z2VSLz { unit { 0 => z2VYQz : "ProposalSourceUnreachable", 1 => z2VaPL : "HostUnreachable", 2 => z2VQZj : "PublicationMalformed", 3 => z2VPBA : "NodeUnknown" } unary {  } });
wire_struct!(z2VTjL { field_0: z2VXtV, field_1: z2VXGN, field_2: Option<z2VNzL> });
wire_external_newtype!(z2VYRj, u64);
wire_external_newtype!(z2VaCk, std::string::String);
wire_struct!(z2Vbti { field_0: z2Vdkm, field_1: z2VU8F, field_2: z2VaQR, field_3: Option<z2VSfk>, field_4: z2VSVX, field_5: Option<z2VW3Q>, field_6: Option<z2VZQW> });
wire_newtype!(z2Ve15, z2VZWT);
wire_enum!(z2VWkd { unit { 0 => z2VNRB : "HermeticCheck", 1 => z2VZrd : "BringUp", 2 => z2VWp7 : "Assert", 3 => z2VUtp : "Deploy", 4 => z2VdmZ : "TearDown" } unary {  } });
wire_struct!(z2VYpD { field_0: z2VNzL, field_1: z2VXtV, field_2: z2VXGN, field_3: z2VXGN, field_4: z2VKsi, field_5: z2VPhf, field_6: z2VNtH, field_7: Option<z2VTGR> });
wire_enum!(z2VZH3 { unit { 0 => z2VUJK : "TestActivation", 1 => z2Ve1w : "ScheduleBootOnce", 2 => z2VZeG : "Realize", 3 => z2VNyd : "SetBootProfile", 4 => z2VRtT : "Evaluate", 5 => z2Vapv : "ActivateNow" } unary {  } });
wire_external_newtype!(z2VMFV, std::string::String);
wire_struct!(z2VTQ9 { field_0: Vec<z2VZx1>, field_1: Vec<z2Vbti>, field_2: z2VaUx });
wire_struct!(z2VLsn { field_0: z2Vdkm });
wire_newtype!(z2VX97, z2VY5C);
wire_enum!(z2VRgR { unit { 0 => z2Vdj3 : "SubscriptionTokenUnknown", 1 => z2VYqF : "SubscriptionAlreadyClosed" } unary {  } });
wire_enum!(z2VVKC { unit { 0 => z2VLpz : "Pinned", 1 => z2VTwD : "Recent", 2 => z2VNs9 : "Rollback", 3 => z2VeUT : "BootPending", 4 => z2VUaC : "Current" } unary {  } });
wire_external_newtype!(z2VNzL, u64);
wire_external_newtype!(z2VQsT, std::string::String);
wire_enum!(z2VU8v { unit { 0 => z2VYNa : "CompleteHost", 1 => z2VQ5p : "BaseHost" } unary {  } });
wire_struct!(z2VKvY { field_0: Option<z2VXtV>, field_1: Option<z2VXGN> });
wire_external_newtype!(z2VVJv, std::string::String);
wire_enum!(z2VVG2 { unit { 0 => z2VYWx : "Built", 1 => z2VPKd : "Completed", 2 => z2VTfm : "Failed", 3 => z2VXsR : "Copying", 4 => z2VRQP : "Rejected", 5 => z2VPgL : "Activated", 6 => z2VbX9 : "Submitted", 7 => z2VQYT : "Building", 8 => z2VP98 : "Activating" } unary {  } });
wire_struct!(z2VaUx { field_0: z2VR89, field_1: z2VebC });
wire_newtype!(z2VSfk, z2VaUx);
wire_external_newtype!(z2VSgN, std::string::String);
wire_external_newtype!(z2VXcT, std::string::String);
wire_newtype!(z2Vb9C, z2Vdhd);
wire_struct!(z2VaQR { field_0: z2VWUZ, field_1: z2VXtV, field_2: z2VXGN, field_3: z2Vccx, field_4: z2VYJj, field_5: z2VSJr, field_6: z2VLic, field_7: Option<z2VQbS> });
wire_newtype!(z2VXQc, z2Vcmr);
wire_newtype!(z2Vchw, z2VaUx);
wire_struct!(z2VbSz { field_0: z2VXGN, field_1: Vec<z2VYYM>, field_2: z2VaUx });
wire_struct!(z2VaWM { field_0: z2VXtV, field_1: z2VXGN, field_2: Option<z2VSqw> });
wire_external_newtype!(z2VPWn, u64);
wire_struct!(z2VdhM { field_0: z2VSSW });
wire_struct!(z2VUnY { field_0: z2VU8F, field_1: z2VXtV, field_2: z2VXGN, field_3: z2VTki, field_4: z2VVKC, field_5: Option<z2VVKC>, field_6: Option<z2VMFV>, field_7: z2VPWn });
wire_external_newtype!(z2VatQ, std::string::String);
wire_enum!(z2VQgC { unit { 0 => z2VNUJ : "Horizon", 1 => z2VXMh : "Direct" } unary {  } });
wire_enum!(z2VPhf { unit { 0 => z2VZSx : "Submitted", 1 => z2VQnr : "BringingUp", 2 => z2VPeJ : "TearingDown", 3 => z2Vc2g : "Completed", 4 => z2VZRE : "Deploying", 5 => z2VP1f : "Asserting", 6 => z2Vd9o : "Failed" } unary {  } });
wire_newtype!(z2VcB9, z2VWhZ);
wire_newtype!(z2VUqu, z2VWjn);
wire_struct!(z2VSn8 { field_0: z2VSDa, field_1: z2VSgN });
wire_struct!(z2VL3w { field_0: z2VKsi, field_1: z2VaCk, field_2: z2VXma, field_3: Option<z2VSn8> });
wire_struct!(z2VeXy { field_0: z2VYRj, field_1: z2VR89 });
wire_enum!(z2VSSW { unit { 0 => z2VdBQ : "MalformedWatch", 1 => z2Vdtn : "SubscriptionLimitReached", 2 => z2VNsZ : "StreamUnavailable" } unary {  } });
wire_enum!(z2VWva { unit { 0 => z2VN5x : "HomeManagerNixProfileV1", 1 => z2VWX7 : "NixosSystemdBootV1" } unary {  } });
wire_enum!(z2Vccx { unit { 0 => z2VKxg : "BaseHost", 1 => z2VSDK : "CompleteHost", 2 => z2Vdt1 : "UserEnvironment" } unary {  } });
wire_newtype!(z2VWof, z2VbSz);
wire_struct!(z2VWjn { field_0: z2VRgR, field_1: z2VYRj });
wire_struct!(z2VNia { field_0: Vec<z2VRZj>, field_1: Vec<z2VUnY>, field_2: z2VaUx });
wire_struct!(z2VZ95 { field_0: z2VYRj });
wire_newtype!(z2VW3Q, z2VaUx);
wire_enum!(z2VWUZ { unit { 0 => z2VR5a : "HostEnvironment" } unary { 1 => z2VWiX(z2VXcT) : "UserEnvironment" } });
wire_external_newtype!(z2VQbS, std::string::String);
wire_newtype!(z2VcE8, z2VZ95);
wire_enum!(z2Vbmn { unit { 1 => z2VPV4 : "DefaultHost" } unary { 0 => z2VZES(z2VXGN) : "OnHost" } });
wire_struct!(z2VYDb { field_0: z2VPWn, field_1: z2VPWn });
wire_enum!(z2VSVX { unit { 0 => z2VPDe : "Failed", 1 => z2VWHo : "Rejected", 2 => z2VZDJ : "Completed", 3 => z2VaaD : "Building", 4 => z2VeS7 : "Activating", 5 => z2VdRe : "Submitted", 6 => z2Vecz : "Copying", 7 => z2VToU : "Activated", 8 => z2VZH5 : "Built" } unary {  } });
wire_struct!(z2VXma { field_0: z2VVJv });
wire_external_newtype!(z2VTGR, std::string::String);
wire_enum!(z2VSJr { unit { 0 => z2Vbim : "ProfileOnly", 1 => z2VM1U : "BootOnceProfile", 2 => z2VWyK : "TestActivation", 3 => z2VP6k : "LiveActivation", 4 => z2VeUe : "BootProfile" } unary {  } });
wire_struct!(z2VZWT { field_0: z2VYRj });
wire_newtype!(z2VeF3, z2VZdM);
wire_enum!(z2VTvQ { unit {  } unary { 0 => z2VbbV(z2VeF3) : "CheckHostKeyMaterial", 1 => z2VdkL(z2VX97) : "WatchDeployments", 2 => z2VTeG(z2VXQc) : "Query", 3 => z2VZpJ(z2VLkY) : "WatchCacheRetention", 4 => z2VUL5(z2VcE8) : "Unwatch" } });
wire_enum!(z2VKsi { unit { 0 => z2VdnZ : "Hermetic", 1 => z2VP1x : "Live" } unary {  } });
wire_external_newtype!(z2VebC, u64);
wire_enum!(z2VcoS { unit { 0 => z2VbqN : "MalformedSelector", 1 => z2VXYu : "EventLogPositionOutOfRange", 2 => z2VYjx : "GenerationUnknown", 3 => z2VV4Q : "NodeUnknown" } unary {  } });
wire_external_newtype!(z2VSDa, std::string::String);
wire_struct!(z2VWhZ { field_0: Vec<z2VYpD>, field_1: z2VaUx });
wire_enum!(z2VZQW { unit { 2 => z2VciR : "Succeeded" } unary { 0 => z2VRVE(z2Vdph) : "Failed", 1 => z2VTCC(z2VYo6) : "Rejected" } });
wire_enum!(z2VLic { unit { 0 => z2Ve6d : "ResolveAndRecord", 1 => z2VRE9 : "RequireImmutable" } unary {  } });
wire_struct!(z2VZdM { field_0: z2VXtV, field_1: z2VXGN, field_2: z2VYbZ });
wire_external_newtype!(z2VXtV, std::string::String);
wire_external_newtype!(z2VMfj, std::string::String);
wire_external_newtype!(z2Vdkm, u64);
wire_enum!(z2VSqw { unit { 0 => z2VdBP : "UserEnvironment", 1 => z2VafE : "CompleteHost", 2 => z2VNtW : "BaseHost" } unary {  } });
wire_newtype!(z2VRhr, z2VNia);
wire_struct!(z2Vdph { field_0: z2VQSr, field_1: z2VYo6 });
wire_enum!(z2VQSr { unit { 0 => z2VezU : "Build", 1 => z2VUaL : "Eval", 2 => z2VMxu : "MaterializeHorizon", 3 => z2VW1w : "Daemon", 4 => z2VUAR : "Activate", 5 => z2VTcL : "CopyClosure", 6 => z2VWuE : "Admission", 7 => z2VWKC : "FlakeAuth" } unary {  } });
wire_struct!(z2VZx1 { field_0: z2VU8F, field_1: z2Vdkm, field_2: z2VXtV, field_3: z2VXGN, field_4: z2Vccx, field_5: z2VSJr, field_6: z2VVKC, field_7: Option<z2VTGR>, field_8: Option<z2VQbS> });
wire_newtype!(z2VW5R, z2VTQ9);
wire_struct!(z2Vdhd { field_0: z2VSLz, field_1: z2VaUx });
wire_external_newtype!(z2VXKF, std::string::String);
wire_enum!(z2VYo6 { unit { 0 => z2VW8n : "NodeUnknown", 1 => z2Vb5u : "FlakeReferenceMalformed", 2 => z2VQVZ : "ProposalSourceUnreachable", 3 => z2VXt6 : "DeploymentInFlight", 4 => z2VMwr : "InvalidDeploymentRouting", 5 => z2VYUu : "UnsupportedDeployAction", 6 => z2VdC4 : "InternalError", 7 => z2VYvX : "ClusterUnknown", 8 => z2VTKv : "ActivationFailed", 9 => z2VcLy : "BuilderUnreachable", 10 => z2VbxC : "SubstituterUnreachable" } unary {  } });
wire_enum!(z2Vcmr { unit {  } unary { 0 => z2VSAw(z2VaWM) : "ByNode", 1 => z2VXA6(z2VTjL) : "ByTestRun", 2 => z2VS4H(z2VLsn) : "ByDeployment", 3 => z2VWXq(z2VZQZ) : "ByGeneration", 4 => z2VUAB(z2VYDb) : "ByEventLog" } });

macro_rules! archive_root {
    ($root:ident) => {
        impl Archive for $root {
            type Archived = <WireValue as Archive>::Archived;
            type Resolver = <WireValue as Archive>::Resolver;
            fn resolve(&self, resolver: Self::Resolver, out: rkyv::Place<Self::Archived>) { self.to_wire().resolve(resolver, out); }
        }
        impl<Serializer> RkyvSerialize<Serializer> for $root where Serializer: rkyv::rancor::Fallible + ?Sized, WireValue: RkyvSerialize<Serializer> {
            fn serialize(&self, serializer: &mut Serializer) -> Result<Self::Resolver, Serializer::Error> { self.to_wire().serialize(serializer) }
        }
        impl<Deserializer> RkyvDeserialize<$root, Deserializer> for ArchivedWireValue
        where Deserializer: rkyv::rancor::Fallible + ?Sized, Deserializer::Error: rkyv::rancor::Source, ArchivedWireValue: RkyvDeserialize<WireValue, Deserializer> {
            fn deserialize(&self, deserializer: &mut Deserializer) -> Result<$root, Deserializer::Error> {
                let wire = <ArchivedWireValue as RkyvDeserialize<WireValue, Deserializer>>::deserialize(self, deserializer)?;
                <$root as WireShape>::from_wire(wire).map_err(Deserializer::Error::new)
            }
        }
    };
}
archive_root!(z2VTvQ);
archive_root!(z2VcR1);


pub enum ContractMarker {}
impl signal_frame::WireContract for ContractMarker {
    const BINDING: signal_frame::ContractBinding = signal_frame::ContractBinding::new(
        match signal_frame::ContractId::try_new(5) { Ok(value) => value, Err(_) => panic!("contract ID is allocated") },
        match signal_frame::WireRevision::try_new(2) { Ok(value) => value, Err(_) => panic!("wire revision is allocated") },
    );
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngineRefusalReason { Rejected, Unavailable }
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct EngineRefusal { pub reason: EngineRefusalReason, pub detail: std::string::String }
impl EngineRefusal {
    pub fn rejected(detail: std::string::String) -> Self { Self { reason: EngineRefusalReason::Rejected, detail } }
    pub fn unavailable(detail: std::string::String) -> Self { Self { reason: EngineRefusalReason::Unavailable, detail } }
}
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SignalFrameError {
    #[error("failed to encode bound signal frame")] FrameEncode,
    #[error("failed to decode bound signal frame")] ArchiveDecode,
    #[error("unexpected signal frame body")] UnexpectedFrameBody,
    #[error("expected one request operation, found {found}")] OperationCount { found: usize },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum InputRoute { Query, WatchDeployments, WatchCacheRetention, Unwatch, CheckHostKeyMaterial }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OutputRoute { Queried, DeploymentEventsQueried, TestRunsQueried, Watching, Unwatched, KeyMaterialChecked, QueryRejected, WatchRejected, UnwatchRejected, KeyMaterialCheckRejected }
impl z2VTvQ {
    pub fn route(&self) -> InputRoute { match self {
        Self::z2VTeG(_) => InputRoute::Query,
        Self::z2VdkL(_) => InputRoute::WatchDeployments,
        Self::z2VZpJ(_) => InputRoute::WatchCacheRetention,
        Self::z2VUL5(_) => InputRoute::Unwatch,
        Self::z2VbbV(_) => InputRoute::CheckHostKeyMaterial,
    } }
    pub fn wire_route(&self) -> signal_frame::WireRoute { signal_frame::WireRoute::new(signal_frame::RootCode::new(0), signal_frame::VariantCode::new(self.route() as u8)) }
    pub fn into_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Frame {
        let route = self.wire_route(); Frame::new(route, FrameBody::Request { exchange, request: signal_frame::Request::from_payload(self) })
    }
    pub fn encode_request_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Result<Vec<u8>, SignalFrameError> { self.into_frame(exchange).encode().map_err(|_| SignalFrameError::FrameEncode) }
}
impl z2VcR1 {
    pub fn route(&self) -> OutputRoute { match self {
        Self::z2VZXM(_) => OutputRoute::Queried,
        Self::z2VYcX(_) => OutputRoute::DeploymentEventsQueried,
        Self::z2VS1B(_) => OutputRoute::TestRunsQueried,
        Self::z2VZdT(_) => OutputRoute::Watching,
        Self::z2VNFq(_) => OutputRoute::Unwatched,
        Self::z2VVBd(_) => OutputRoute::KeyMaterialChecked,
        Self::z2VaL5(_) => OutputRoute::QueryRejected,
        Self::z2VNoL(_) => OutputRoute::WatchRejected,
        Self::z2VQom(_) => OutputRoute::UnwatchRejected,
        Self::z2Vby6(_) => OutputRoute::KeyMaterialCheckRejected,
    } }
    pub fn wire_route(&self) -> signal_frame::WireRoute { signal_frame::WireRoute::new(signal_frame::RootCode::new(1), signal_frame::VariantCode::new(self.route() as u8)) }
    pub fn into_reply_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Frame {
        let route = self.wire_route(); let reply = signal_frame::Reply::committed(signal_frame::NonEmpty::single(signal_frame::SubReply::Ok(self)));
        Frame::new(route, FrameBody::Reply { exchange, reply })
    }
    pub fn encode_reply_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Result<Vec<u8>, SignalFrameError> { self.into_reply_frame(exchange).encode().map_err(|_| SignalFrameError::FrameEncode) }
}
impl signal_frame::RequestPayload for z2VTvQ {}
impl signal_frame::SignalOperationHeads for z2VTvQ { const HEADS: &'static [&'static str] = &["Query", "WatchDeployments", "WatchCacheRetention", "Unwatch", "CheckHostKeyMaterial"]; }
impl signal_frame::LogVariant for z2VTvQ {
    fn log_variant(&self) -> u64 { let route = self.wire_route(); u64::from(route.root().value()) | (u64::from(route.variant().value()) << 8) }
}
pub type Frame = signal_frame::BoundExchangeFrame<ContractMarker, z2VTvQ, z2VcR1>;
pub type FrameBody = signal_frame::ExchangeFrameBody<z2VTvQ, z2VcR1>;
pub type Request = signal_frame::Request<z2VTvQ>;
pub type ReplyEnvelope = signal_frame::Reply<z2VcR1>;
pub type RequestBuilder = signal_frame::RequestBuilder<z2VTvQ>;
impl ContractMarker {
    pub fn decode_frame(bytes: &[u8]) -> Result<Frame, SignalFrameError> { Frame::decode(bytes).map_err(|_| SignalFrameError::ArchiveDecode) }
    pub fn decode_single_request(bytes: &[u8]) -> Result<(signal_frame::ExchangeIdentifier, z2VTvQ), SignalFrameError> {
        match Self::decode_frame(bytes)?.into_body() {
            FrameBody::Request { exchange, request } => { let found=request.payloads().len(); if found!=1 { return Err(SignalFrameError::OperationCount { found }); } Ok((exchange, request.payloads.into_head())) },
            _ => Err(SignalFrameError::UnexpectedFrameBody),
        }
    }
}
