#![cfg(feature = "dotos-text")]

use dotos::{DotosDecode, DotosEncode, DotosSource};
use signal_lojix::schema::lib::{z2VTvQ, z2VYRj, z2VZ95, z2VZWT, z2VcE8, z2VcR1, z2Ve15};

fn unwatch_request() -> z2VTvQ {
    z2VTvQ::z2VUL5(z2VcE8::new(z2VZ95 {
        field_0: z2VYRj::new(7),
    }))
}

fn unwatched_reply() -> z2VcR1 {
    z2VcR1::z2VNFq(z2Ve15::new(z2VZWT {
        field_0: z2VYRj::new(7),
    }))
}

fn round_trip<Value>(value: Value) -> String
where
    Value: DotosDecode + DotosEncode + PartialEq + std::fmt::Debug,
{
    let encoded = value.to_dotos();
    let recovered = DotosSource::new(&encoded)
        .parse::<Value>()
        .expect("decode Dotos text");
    assert_eq!(recovered, value);
    encoded
}

#[test]
fn encoded_roots_round_trip_through_readable_dotos_roles() {
    assert!(round_trip(unwatch_request()).contains("Unwatch"));
    assert!(round_trip(unwatched_reply()).contains("Unwatched"));
}
