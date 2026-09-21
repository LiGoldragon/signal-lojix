//! Producer-owned, archiveable representation of Horizon 37416 authored input.
//!
//! Horizon's `ClusterProposal` is an input document.  Its projection is a
//! different concern and deliberately has no place in this module.  The source
//! model's ordered maps are represented by named entry vectors: `datom-codec`
//! composes vectors but has no `BTreeMap` implementation.  Conversion verifies
//! key uniqueness and the original unsigned bounds before constructing Horizon.

macro_rules! wire_string {
    ($name:ident) => {
        #[derive(
            rkyv::Archive,
            rkyv::Serialize,
            rkyv::Deserialize,
            Clone,
            Debug,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
        )]
        #[cfg_attr(
            feature = "datom",
            derive(datom_codec::Datomizable, datom_codec::Composing)
        )]
        pub struct $name(pub String);
    };
}

macro_rules! wire_type {
    ($item:item) => {
        #[derive(
            rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash,
        )]
        #[cfg_attr(
            feature = "datom",
            derive(datom_codec::Datomizable, datom_codec::Composing)
        )]
        $item
    };
}

wire_string!(ClusterNameWire);
wire_string!(NodeNameWire);
wire_string!(UserNameWire);
wire_string!(DomainNameWire);
wire_string!(ModelNameWire);
wire_string!(GithubIdWire);
wire_string!(SecretNameWire);
wire_string!(WirelessNetworkNameWire);
wire_string!(KeygripWire);
wire_string!(SshPubKeyWire);
wire_string!(NixPubKeyWire);
wire_string!(YggPubKeyWire);
wire_string!(WireguardPubKeyWire);
wire_string!(YggAddressWire);
wire_string!(YggSubnetWire);
wire_string!(NodeIpWire);
wire_string!(TapSubnetWire);
wire_string!(InterfaceWire);
wire_string!(DevicePathWire);
wire_string!(MountPathWire);
wire_string!(LocationWire);
wire_string!(InternalDomainSuffixWire);
wire_string!(PublicClusterDomainWire);
wire_string!(ServedDomainWire);
wire_string!(SiteSourceWire);
wire_string!(MacAddressWire);
wire_string!(Ipv4CidrWire);

wire_type! {
    pub struct ClusterProposalWire {
        pub nodes: Vec<NodeProposalEntryWire>,
        pub users: Vec<UserProposalEntryWire>,
        pub domains: Vec<DomainProposalEntryWire>,
        pub trust: ClusterTrustWire,
        pub domain_configuration: DomainConfigurationWire,
    }
}

wire_type! {
    pub struct NodeProposalEntryWire {
        pub name: NodeNameWire,
        pub proposal: NodeProposalWire,
    }
}

wire_type! {
    pub struct UserProposalEntryWire {
        pub name: UserNameWire,
        pub proposal: UserProposalWire,
    }
}

wire_type! {
    pub struct DomainProposalEntryWire {
        pub name: DomainNameWire,
        pub proposal: DomainProposalWire,
    }
}

wire_type! {
    pub struct NodeProposalWire {
        pub species: NodeSpeciesWire,
        pub size: MagnitudeWire,
        pub trust: MagnitudeWire,
        pub machine: MachineWire,
        pub io: IoWire,
        pub pub_keys: NodePubKeysWire,
        pub link_local_ips: Vec<LinkLocalIpWire>,
        pub node_ip: Option<NodeIpWire>,
        pub wireguard_pub_key: Option<WireguardPubKeyWire>,
        pub nordvpn: bool,
        pub wifi_cert: bool,
        pub wireguard_untrusted_proxies: Vec<WireguardProxyWire>,
        pub wants_printing: bool,
        pub wants_hw_video_accel: bool,
        pub router_interfaces: Option<RouterInterfacesWire>,
        pub online: Option<bool>,
        pub services: Vec<NodeServiceWire>,
    }
}

wire_type! {
    pub enum NodeServiceWire {
        TailnetClient,
        TailnetController,
        NixBuilder { maximum_jobs: Option<i64> },
        NixCache,
        OpenCodeTesting,
        PersonaDevelopment { capabilities: Vec<PersonaDevelopmentCapabilityWire> },
        VmHost {
            guest_subnet: TapSubnetWire,
            kvm: KvmAvailabilityWire,
            maximum_guests: Option<i64>,
        },
        WebHost { sites: Vec<HostedSiteWire> },
        UsbIpv4Gateway {
            downstream: InterfaceWire,
            downstream_mac: MacAddressWire,
            gateway: Ipv4CidrWire,
            uplink: InterfaceWire,
        },
    }
}

wire_type! { pub enum PersonaDevelopmentCapabilityWire { GitoliteServer } }
wire_type! { pub enum KvmAvailabilityWire { Available, Absent } }
wire_type! { pub enum SiteRendererWire { MarkdownStatic } }
wire_type! { pub enum WlanBandWire { TwoG, FiveG, SixG } }
wire_type! { pub enum WlanStandardWire { Wifi4, Wifi6, Wifi7 } }
wire_type! { pub enum FsTypeWire { Ext2, Ext3, Ext4, Btrfs, Xfs, Zfs, F2fs, Bcachefs, Vfat, Exfat, Ntfs, Tmpfs } }
wire_type! { pub enum NodeSpeciesWire { Center, LargeAi, LargeAiRouter, Hybrid, Edge, EdgeTesting, MediaBroadcast, Router, RouterTesting, TestVm, CloudNode } }
wire_type! { pub enum UserSpeciesWire { Code, Multimedia, Unlimited } }
wire_type! { pub enum MachineSpeciesWire { Metal, Pod } }
wire_type! { pub enum KeyboardWire { Qwerty, Colemak } }
wire_type! { pub enum StyleWire { Vim, Emacs } }
wire_type! { pub enum EditorWire { Codium, Emacs } }
wire_type! { pub enum TextSizeWire { ExtraSmall, Small, Medium, Large, ExtraLarge } }
wire_type! { pub enum BootloaderWire { Uefi, Mbr, Uboot } }
wire_type! { pub enum ArchWire { X86_64, Arm64 } }
wire_type! { pub enum MotherBoardWire { Ondyfaind } }
wire_type! { pub enum DomainSpeciesWire { Cloudflare } }
wire_type! { pub enum MagnitudeWire { Zero, Min, Medium, Large, Max } }

wire_type! {
    pub struct HostedSiteWire {
        pub domain: ServedDomainWire,
        pub source: SiteSourceWire,
        pub renderer: SiteRendererWire,
    }
}

wire_type! {
    pub struct RouterInterfacesWire {
        pub wan: InterfaceWire,
        pub wlan: InterfaceWire,
        pub wlan_band: WlanBandWire,
        pub wlan_channel: i64,
        pub wlan_standard: WlanStandardWire,
        pub wpa3_sae_password: Option<SecretReferenceWire>,
        pub backup_wireless: Option<BackupWirelessWire>,
    }
}

wire_type! {
    pub struct BackupWirelessWire {
        pub interface: InterfaceWire,
        pub network_name: WirelessNetworkNameWire,
        pub band: WlanBandWire,
        pub channel: i64,
        pub standard: WlanStandardWire,
        pub password: SecretReferenceWire,
    }
}

wire_type! { pub struct SecretReferenceWire { pub name: SecretNameWire } }

wire_type! {
    pub struct NodePubKeysWire {
        pub ssh: SshPubKeyWire,
        pub nix: Option<NixPubKeyWire>,
        pub yggdrasil: Option<YggPubKeyEntryWire>,
    }
}

wire_type! {
    pub struct YggPubKeyEntryWire {
        pub pub_key: YggPubKeyWire,
        pub address: YggAddressWire,
        pub subnet: YggSubnetWire,
    }
}

wire_type! {
    pub struct UserProposalWire {
        pub species: UserSpeciesWire,
        pub size: MagnitudeWire,
        pub keyboard: KeyboardWire,
        pub style: StyleWire,
        pub github_id: Option<GithubIdWire>,
        pub fast_repeat: Option<bool>,
        pub pub_keys: Vec<UserPubKeyEntryByNodeWire>,
        pub editor: Option<EditorWire>,
        pub text_size: Option<TextSizeWire>,
    }
}

wire_type! {
    pub struct UserPubKeyEntryByNodeWire {
        pub node: NodeNameWire,
        pub key: UserPubKeyEntryWire,
    }
}

wire_type! {
    pub struct UserPubKeyEntryWire {
        pub ssh: SshPubKeyWire,
        pub keygrip: KeygripWire,
    }
}

wire_type! { pub struct DomainProposalWire { pub species: DomainSpeciesWire } }

wire_type! {
    pub struct ClusterTrustWire {
        pub cluster: MagnitudeWire,
        pub clusters: Vec<ClusterTrustByClusterWire>,
        pub nodes: Vec<ClusterTrustByNodeWire>,
        pub users: Vec<ClusterTrustByUserWire>,
    }
}

wire_type! { pub struct ClusterTrustByClusterWire { pub cluster: ClusterNameWire, pub trust: MagnitudeWire } }
wire_type! { pub struct ClusterTrustByNodeWire { pub node: NodeNameWire, pub trust: MagnitudeWire } }
wire_type! { pub struct ClusterTrustByUserWire { pub user: UserNameWire, pub trust: MagnitudeWire } }

wire_type! {
    pub struct WireguardProxyWire {
        pub pub_key: WireguardPubKeyWire,
        pub endpoint: String,
        pub interface_ip: NodeIpWire,
    }
}

wire_type! {
    pub struct DomainConfigurationWire {
        pub internal_suffix: InternalDomainSuffixWire,
        pub public_cluster_domains: Vec<PublicClusterDomainWire>,
    }
}

wire_type! {
    pub struct MachineWire {
        pub species: MachineSpeciesWire,
        pub arch: Option<ArchWire>,
        pub cores: i64,
        pub model: Option<ModelNameWire>,
        pub mother_board: Option<MotherBoardWire>,
        pub super_node: Option<NodeNameWire>,
        pub super_user: Option<UserNameWire>,
        pub chip_gen: Option<i64>,
        pub ram_gb: Option<i64>,
        pub disk_gb: Option<i64>,
        pub location: Option<LocationWire>,
        pub super_nodes: Vec<NodeNameWire>,
    }
}

wire_type! {
    pub struct IoWire {
        pub keyboard: KeyboardWire,
        pub bootloader: BootloaderWire,
        pub disks: Vec<DiskByMountPathWire>,
        pub swap_devices: Vec<SwapDeviceWire>,
        pub compressed_swap: Option<CompressedSwapWire>,
    }
}

wire_type! {
    pub struct DiskByMountPathWire {
        pub mount_path: MountPathWire,
        pub disk: DiskWire,
    }
}

wire_type! {
    pub struct DiskWire {
        pub device: DevicePathWire,
        pub fs_type: FsTypeWire,
        pub options: Vec<String>,
    }
}

wire_type! { pub struct SwapDeviceWire { pub device: DevicePathWire, pub size_mebibytes: Option<i64> } }
wire_type! { pub struct CompressedSwapWire { pub memory_percent: i64 } }
wire_type! { pub struct LinkLocalIpWire { pub iface: InterfaceWire, pub suffix: String } }
