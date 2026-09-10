#[cfg(feature = "datom")]
use signal_lojix::DatabaseMarker;
use signal_lojix::{
    ByteViewable, ConfigurationReceipt, KeyMaterialQuery, LojixNexusConfiguration, Query, Response,
    Restorable, Signal, Signalizable, TestDefaultsChoice,
};

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
    let query = Query::CheckHostKeyMaterial(KeyMaterialQuery {
        cluster_name: "production.eu".into(),
        node_name: ".state/cache".into(),
        proposal_source: "https://proposal.example/flake.nix".into(),
    });
    let sent = query.signalize().expect("signalize query");
    assert!(!sent.bytes().is_empty());
    let received = Signal::<Query>::from(sent.bytes().to_vec());
    assert_eq!(received.restore().expect("restore query"), query);

    let response = Response::Unwatched(signal_lojix::SubscriptionClosed {
        subscription_token: 41,
    });
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

    let query = Query::CheckHostKeyMaterial(KeyMaterialQuery {
        cluster_name: "production.eu".into(),
        node_name: ".state/cache".into(),
        proposal_source: "https://proposal.example/flake.nix".into(),
    });
    let rendered = query.clone().datomize(vec![]).protosize().textualize();
    assert!(rendered.contains("production.eu"));
    assert!(rendered.contains(".state/cache"));
    assert!(rendered.contains("https://proposal.example/flake.nix"));

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

    let response = Response::KeyMaterialChecked(signal_lojix::KeyMaterialReport {
        node_name: "host.example".into(),
        key_material_mismatch_vector: vec![],
        database_marker: DatabaseMarker {
            commit_sequence: 9,
            state_digest: 17,
        },
    });
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
