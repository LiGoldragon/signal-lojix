use horizon_current::proposal::{ClusterProposal, NodeService};
use signal_lojix::{
    ClusterProposalWire, horizon_wire::HorizonWireConversionError, horizon_wire_types::*,
};

fn proposal() -> ClusterProposalWire {
    ClusterProposalWire {
        nodes: vec![NodeProposalEntryWire {
            name: NodeNameWire("ouranos".into()),
            proposal: NodeProposalWire {
                species: NodeSpeciesWire::Center,
                size: MagnitudeWire::Min,
                trust: MagnitudeWire::Min,
                machine: MachineWire {
                    species: MachineSpeciesWire::Metal,
                    arch: Some(ArchWire::X86_64),
                    cores: 4,
                    model: None,
                    mother_board: None,
                    super_node: None,
                    super_user: None,
                    chip_gen: None,
                    ram_gb: None,
                    disk_gb: None,
                    location: None,
                    super_nodes: vec![],
                },
                io: IoWire {
                    keyboard: KeyboardWire::Qwerty,
                    bootloader: BootloaderWire::Uefi,
                    disks: vec![],
                    swap_devices: vec![],
                    compressed_swap: None,
                },
                pub_keys: NodePubKeysWire {
                    ssh: SshPubKeyWire("AAAAfixture".into()),
                    nix: None,
                    yggdrasil: None,
                },
                link_local_ips: vec![],
                node_ip: None,
                wireguard_pub_key: None,
                nordvpn: false,
                wifi_cert: false,
                wireguard_untrusted_proxies: vec![],
                wants_printing: false,
                wants_hw_video_accel: false,
                router_interfaces: None,
                online: None,
                services: vec![NodeServiceWire::UsbIpv4Gateway {
                    downstream: InterfaceWire("enp0s20f0u1c2".into()),
                    downstream_mac: MacAddressWire("00:0e:c6:33:4f:97".into()),
                    gateway: Ipv4CidrWire("10.44.0.1/24".into()),
                    uplink: InterfaceWire("enp0s31f6".into()),
                }],
            },
        }],
        users: vec![],
        domains: vec![],
        trust: ClusterTrustWire {
            cluster: MagnitudeWire::Zero,
            clusters: vec![],
            nodes: vec![],
            users: vec![],
        },
        domain_configuration: DomainConfigurationWire {
            internal_suffix: InternalDomainSuffixWire("criome".into()),
            public_cluster_domains: vec![],
        },
    }
}

#[test]
fn converts_typed_usb_gateway_without_projection_context() {
    let proposal = ClusterProposal::try_from(proposal()).expect("convert proposal");
    let node = proposal
        .nodes
        .get(&"ouranos".parse().expect("node name"))
        .unwrap();
    assert!(matches!(
        node.services.as_slice(),
        [NodeService::UsbIpv4Gateway { .. }]
    ));
}

#[test]
fn rejects_duplicate_node_names() {
    let mut input = proposal();
    input.nodes.push(input.nodes[0].clone());
    assert!(matches!(
        ClusterProposal::try_from(input),
        Err(HorizonWireConversionError::DuplicateKey { map: "nodes", .. })
    ));
}

#[test]
fn rejects_out_of_range_and_invalid_usb_values() {
    let mut input = proposal();
    input.nodes[0].proposal.machine.cores = -1;
    assert!(matches!(
        ClusterProposal::try_from(input),
        Err(HorizonWireConversionError::InvalidValue {
            field: "machine cores",
            ..
        })
    ));

    let mut input = proposal();
    input.nodes[0].proposal.services = vec![NodeServiceWire::UsbIpv4Gateway {
        downstream: InterfaceWire("enp0s20f0u1c2".into()),
        downstream_mac: MacAddressWire("not-a-mac".into()),
        gateway: Ipv4CidrWire("10.44.0.1/24".into()),
        uplink: InterfaceWire("enp0s20f0u1c2".into()),
    }];
    assert!(matches!(
        ClusterProposal::try_from(input),
        Err(HorizonWireConversionError::InvalidValue {
            field: "USB gateway interfaces",
            ..
        })
    ));
}
