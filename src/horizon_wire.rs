//! Fallible boundary from the archiveable authored Signal input to Horizon.
//! It deliberately has no projected `Horizon` or `Viewpoint` parameter.

use crate::horizon_wire_types::*;
use horizon_current::{
    address::*, domain::*, io::*, machine::Machine, magnitude::Magnitude, name::*, proposal::*,
    species::*,
};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HorizonWireConversionError {
    DuplicateKey { map: &'static str, key: String },
    InvalidValue { field: &'static str, value: String },
}
type Result<T> = std::result::Result<T, HorizonWireConversionError>;
fn bad(field: &'static str, value: impl ToString) -> HorizonWireConversionError {
    HorizonWireConversionError::InvalidValue {
        field,
        value: value.to_string(),
    }
}
fn u32v(field: &'static str, value: i64) -> Result<u32> {
    u32::try_from(value).map_err(|_| bad(field, value))
}
fn u16v(field: &'static str, value: i64) -> Result<u16> {
    u16::try_from(value).map_err(|_| bad(field, value))
}
fn parse<T: TryFrom<String>>(field: &'static str, value: String) -> Result<T> {
    T::try_from(value.clone()).map_err(|_| bad(field, value))
}
fn named<T>(
    field: &'static str,
    value: String,
    f: impl FnOnce(String) -> horizon_current::Result<T>,
) -> Result<T> {
    f(value.clone()).map_err(|_| bad(field, value))
}
fn put<K: Ord + Clone + std::fmt::Display, V>(
    map: &mut BTreeMap<K, V>,
    key: K,
    value: V,
    kind: &'static str,
) -> Result<()> {
    if map.contains_key(&key) {
        return Err(HorizonWireConversionError::DuplicateKey {
            map: kind,
            key: key.to_string(),
        });
    }
    map.insert(key, value);
    Ok(())
}

macro_rules! e { ($v:expr,$t:ident,$($w:path=>$x:ident),+)=>{match $v { $($w=>$t::$x,)+ }}; }
fn mag(v: MagnitudeWire) -> Magnitude {
    e!(v,Magnitude,MagnitudeWire::Zero=>Zero,MagnitudeWire::Min=>Min,MagnitudeWire::Medium=>Medium,MagnitudeWire::Large=>Large,MagnitudeWire::Max=>Max)
}
fn ns(v: NodeSpeciesWire) -> NodeSpecies {
    e!(v,NodeSpecies,NodeSpeciesWire::Center=>Center,NodeSpeciesWire::LargeAi=>LargeAi,NodeSpeciesWire::LargeAiRouter=>LargeAiRouter,NodeSpeciesWire::Hybrid=>Hybrid,NodeSpeciesWire::Edge=>Edge,NodeSpeciesWire::EdgeTesting=>EdgeTesting,NodeSpeciesWire::MediaBroadcast=>MediaBroadcast,NodeSpeciesWire::Router=>Router,NodeSpeciesWire::RouterTesting=>RouterTesting,NodeSpeciesWire::TestVm=>TestVm,NodeSpeciesWire::CloudNode=>CloudNode)
}
fn us(v: UserSpeciesWire) -> UserSpecies {
    e!(v,UserSpecies,UserSpeciesWire::Code=>Code,UserSpeciesWire::Multimedia=>Multimedia,UserSpeciesWire::Unlimited=>Unlimited)
}
fn ms(v: MachineSpeciesWire) -> MachineSpecies {
    e!(v,MachineSpecies,MachineSpeciesWire::Metal=>Metal,MachineSpeciesWire::Pod=>Pod)
}
fn kb(v: KeyboardWire) -> Keyboard {
    e!(v,Keyboard,KeyboardWire::Qwerty=>Qwerty,KeyboardWire::Colemak=>Colemak)
}
fn st(v: StyleWire) -> Style {
    e!(v,Style,StyleWire::Vim=>Vim,StyleWire::Emacs=>Emacs)
}
fn ed(v: EditorWire) -> Editor {
    e!(v,Editor,EditorWire::Codium=>Codium,EditorWire::Emacs=>Emacs)
}
fn ts(v: TextSizeWire) -> TextSize {
    e!(v,TextSize,TextSizeWire::ExtraSmall=>ExtraSmall,TextSizeWire::Small=>Small,TextSizeWire::Medium=>Medium,TextSizeWire::Large=>Large,TextSizeWire::ExtraLarge=>ExtraLarge)
}
fn bl(v: BootloaderWire) -> Bootloader {
    e!(v,Bootloader,BootloaderWire::Uefi=>Uefi,BootloaderWire::Mbr=>Mbr,BootloaderWire::Uboot=>Uboot)
}
fn ar(v: ArchWire) -> Arch {
    e!(v,Arch,ArchWire::X86_64=>X86_64,ArchWire::Arm64=>Arm64)
}
fn fs(v: FsTypeWire) -> FsType {
    e!(v,FsType,FsTypeWire::Ext2=>Ext2,FsTypeWire::Ext3=>Ext3,FsTypeWire::Ext4=>Ext4,FsTypeWire::Btrfs=>Btrfs,FsTypeWire::Xfs=>Xfs,FsTypeWire::Zfs=>Zfs,FsTypeWire::F2fs=>F2fs,FsTypeWire::Bcachefs=>Bcachefs,FsTypeWire::Vfat=>Vfat,FsTypeWire::Exfat=>Exfat,FsTypeWire::Ntfs=>Ntfs,FsTypeWire::Tmpfs=>Tmpfs)
}
fn wb(v: WlanBandWire) -> WlanBand {
    e!(v,WlanBand,WlanBandWire::TwoG=>TwoG,WlanBandWire::FiveG=>FiveG,WlanBandWire::SixG=>SixG)
}
fn ws(v: WlanStandardWire) -> WlanStandard {
    e!(v,WlanStandard,WlanStandardWire::Wifi4=>Wifi4,WlanStandardWire::Wifi6=>Wifi6,WlanStandardWire::Wifi7=>Wifi7)
}
fn nn(v: NodeNameWire) -> Result<NodeName> {
    named("node name", v.0, NodeName::try_new)
}
fn un(v: UserNameWire) -> Result<UserName> {
    named("user name", v.0, UserName::try_new)
}
fn cn(v: ClusterNameWire) -> Result<ClusterName> {
    named("cluster name", v.0, ClusterName::try_new)
}
fn dn(v: DomainNameWire) -> Result<DomainName> {
    named("domain name", v.0, DomainName::try_new)
}

impl TryFrom<ClusterProposalWire> for ClusterProposal {
    type Error = HorizonWireConversionError;
    fn try_from(v: ClusterProposalWire) -> Result<Self> {
        let mut nodes = BTreeMap::new();
        for x in v.nodes {
            let k = nn(x.name)?;
            put(&mut nodes, k, node(x.proposal)?, "nodes")?;
        }
        let mut users = BTreeMap::new();
        for x in v.users {
            let k = un(x.name)?;
            put(&mut users, k, user(x.proposal)?, "users")?;
        }
        let mut domains = BTreeMap::new();
        for x in v.domains {
            let k = dn(x.name)?;
            put(
                &mut domains,
                k,
                DomainProposal {
                    species: match x.proposal.species {
                        DomainSpeciesWire::Cloudflare => DomainSpecies::Cloudflare,
                    },
                },
                "domains",
            )?;
        }
        Ok(ClusterProposal {
            nodes,
            users,
            domains,
            trust: trust(v.trust)?,
            domain_configuration: DomainConfiguration {
                internal_suffix: InternalDomainSuffix::new(
                    v.domain_configuration.internal_suffix.0,
                ),
                public_cluster_domains: v
                    .domain_configuration
                    .public_cluster_domains
                    .into_iter()
                    .map(|x| PublicClusterDomain::new(x.0))
                    .collect(),
            },
        })
    }
}
fn node(v: NodeProposalWire) -> Result<NodeProposal> {
    Ok(NodeProposal {
        species: ns(v.species),
        size: mag(v.size),
        trust: mag(v.trust),
        machine: machine(v.machine)?,
        io: io(v.io)?,
        pub_keys: keys(v.pub_keys)?,
        link_local_ips: v
            .link_local_ips
            .into_iter()
            .map(|x| LinkLocalIp {
                iface: Interface::new(x.iface.0),
                suffix: x.suffix,
            })
            .collect(),
        node_ip: v.node_ip.map(|x| parse("node IP", x.0)).transpose()?,
        wireguard_pub_key: v
            .wireguard_pub_key
            .map(|x| parse("WireGuard public key", x.0))
            .transpose()?,
        nordvpn: v.nordvpn,
        wifi_cert: v.wifi_cert,
        wireguard_untrusted_proxies: v
            .wireguard_untrusted_proxies
            .into_iter()
            .map(proxy)
            .collect::<Result<_>>()?,
        wants_printing: v.wants_printing,
        wants_hw_video_accel: v.wants_hw_video_accel,
        router_interfaces: v.router_interfaces.map(router).transpose()?,
        online: v.online,
        services: v.services.into_iter().map(service).collect::<Result<_>>()?,
    })
}
fn machine(v: MachineWire) -> Result<Machine> {
    Ok(Machine {
        species: ms(v.species),
        arch: v.arch.map(ar),
        cores: u32v("machine cores", v.cores)?,
        model: v
            .model
            .map(|x| named("model", x.0, ModelName::try_new))
            .transpose()?,
        mother_board: v.mother_board.map(|x| match x {
            MotherBoardWire::Ondyfaind => MotherBoard::Ondyfaind,
        }),
        super_node: v.super_node.map(nn).transpose()?,
        super_user: v.super_user.map(un).transpose()?,
        chip_gen: v.chip_gen.map(|x| u32v("chip generation", x)).transpose()?,
        ram_gb: v.ram_gb.map(|x| u32v("RAM GiB", x)).transpose()?,
        disk_gb: v.disk_gb.map(|x| u32v("disk GiB", x)).transpose()?,
        location: v
            .location
            .map(|x| horizon_current::machine::Location::new(x.0)),
        super_nodes: v.super_nodes.into_iter().map(nn).collect::<Result<_>>()?,
    })
}
fn io(v: IoWire) -> Result<Io> {
    let mut disks = BTreeMap::new();
    for x in v.disks {
        put(
            &mut disks,
            MountPath::new(x.mount_path.0),
            Disk {
                device: DevicePath::new(x.disk.device.0),
                fs_type: fs(x.disk.fs_type),
                options: x.disk.options,
            },
            "disks",
        )?;
    }
    Ok(Io {
        keyboard: kb(v.keyboard),
        bootloader: bl(v.bootloader),
        disks,
        swap_devices: v
            .swap_devices
            .into_iter()
            .map(|x| {
                Ok(SwapDevice {
                    device: DevicePath::new(x.device.0),
                    size_mebibytes: x.size_mebibytes.map(|n| u32v("swap MiB", n)).transpose()?,
                })
            })
            .collect::<Result<_>>()?,
        compressed_swap: v
            .compressed_swap
            .map(|x| {
                Ok(CompressedSwap {
                    memory_percent: u32v("compressed swap percent", x.memory_percent)?,
                })
            })
            .transpose()?,
    })
}
fn keys(v: NodePubKeysWire) -> Result<NodePubKeys> {
    Ok(NodePubKeys {
        ssh: parse("SSH public key", v.ssh.0)?,
        nix: v.nix.map(|x| parse("Nix public key", x.0)).transpose()?,
        yggdrasil: v
            .yggdrasil
            .map(|x| {
                Ok(YggPubKeyEntry {
                    pub_key: parse("Yggdrasil public key", x.pub_key.0)?,
                    address: parse("Yggdrasil address", x.address.0)?,
                    subnet: named("Yggdrasil subnet", x.subnet.0, YggSubnet::try_new)?,
                })
            })
            .transpose()?,
    })
}
fn proxy(v: WireguardProxyWire) -> Result<WireguardProxy> {
    Ok(WireguardProxy {
        pub_key: parse("WireGuard public key", v.pub_key.0)?,
        endpoint: v.endpoint,
        interface_ip: parse("WireGuard interface IP", v.interface_ip.0)?,
    })
}
fn router(v: RouterInterfacesWire) -> Result<RouterInterfaces> {
    Ok(RouterInterfaces {
        wan: Interface::new(v.wan.0),
        wlan: Interface::new(v.wlan.0),
        wlan_band: wb(v.wlan_band),
        wlan_channel: u16v("WLAN channel", v.wlan_channel)?,
        wlan_standard: ws(v.wlan_standard),
        wpa3_sae_password: v
            .wpa3_sae_password
            .map(|x| {
                Ok(SecretReference {
                    name: named("secret name", x.name.0, SecretName::try_new)?,
                })
            })
            .transpose()?,
        backup_wireless: v
            .backup_wireless
            .map(|x| {
                Ok(BackupWireless {
                    interface: Interface::new(x.interface.0),
                    network_name: named(
                        "wireless network name",
                        x.network_name.0,
                        WirelessNetworkName::try_new,
                    )?,
                    band: wb(x.band),
                    channel: u16v("backup WLAN channel", x.channel)?,
                    standard: ws(x.standard),
                    password: SecretReference {
                        name: named("secret name", x.password.name.0, SecretName::try_new)?,
                    },
                })
            })
            .transpose()?,
    })
}
fn service(v: NodeServiceWire) -> Result<NodeService> {
    Ok(match v {
        NodeServiceWire::TailnetClient => NodeService::TailnetClient {},
        NodeServiceWire::TailnetController => NodeService::TailnetController {},
        NodeServiceWire::NixBuilder { maximum_jobs } => NodeService::NixBuilder {
            maximum_jobs: maximum_jobs
                .map(|x| u32v("Nix builder maximum jobs", x))
                .transpose()?,
        },
        NodeServiceWire::NixCache => NodeService::NixCache {},
        NodeServiceWire::OpenCodeTesting => NodeService::OpenCodeTesting {},
        NodeServiceWire::PersonaDevelopment { capabilities } => NodeService::PersonaDevelopment {
            capabilities: capabilities
                .into_iter()
                .map(|x| match x {
                    PersonaDevelopmentCapabilityWire::GitoliteServer => {
                        PersonaDevelopmentCapability::GitoliteServer
                    }
                })
                .collect(),
        },
        NodeServiceWire::VmHost {
            guest_subnet,
            kvm,
            maximum_guests,
        } => NodeService::VmHost {
            guest_subnet: parse("VM guest subnet", guest_subnet.0)?,
            kvm: match kvm {
                KvmAvailabilityWire::Available => KvmAvailability::Available,
                KvmAvailabilityWire::Absent => KvmAvailability::Absent,
            },
            maximum_guests: maximum_guests
                .map(|x| Ok(MaximumGuests::new(u32v("VM maximum guests", x)?)))
                .transpose()?,
        },
        NodeServiceWire::WebHost { sites } => NodeService::WebHost {
            sites: sites
                .into_iter()
                .map(|x| HostedSite {
                    domain: ServedDomain::new(x.domain.0),
                    source: SiteSource::new(x.source.0),
                    renderer: match x.renderer {
                        SiteRendererWire::MarkdownStatic => SiteRenderer::MarkdownStatic,
                    },
                })
                .collect(),
        },
        NodeServiceWire::UsbIpv4Gateway {
            downstream,
            downstream_mac,
            gateway,
            uplink,
        } => {
            let downstream = Interface::new(downstream.0);
            let uplink = Interface::new(uplink.0);
            if downstream == uplink {
                return Err(bad("USB gateway interfaces", format!("{downstream}")));
            }
            NodeService::UsbIpv4Gateway {
                downstream,
                downstream_mac: parse("USB gateway MAC", downstream_mac.0)?,
                gateway: parse("USB gateway IPv4 CIDR", gateway.0)?,
                uplink,
            }
        }
    })
}
fn user(v: UserProposalWire) -> Result<UserProposal> {
    let mut pub_keys = BTreeMap::new();
    for x in v.pub_keys {
        let k = nn(x.node)?;
        put(
            &mut pub_keys,
            k,
            UserPubKeyEntry {
                ssh: parse("user SSH public key", x.key.ssh.0)?,
                keygrip: parse("user keygrip", x.key.keygrip.0)?,
            },
            "user public keys",
        )?;
    }
    Ok(UserProposal {
        species: us(v.species),
        size: mag(v.size),
        keyboard: kb(v.keyboard),
        style: st(v.style),
        github_id: v
            .github_id
            .map(|x| named("GitHub id", x.0, GithubId::try_new))
            .transpose()?,
        fast_repeat: v.fast_repeat,
        pub_keys,
        editor: v.editor.map(ed),
        text_size: v.text_size.map(ts),
    })
}
fn trust(v: ClusterTrustWire) -> Result<ClusterTrust> {
    let mut clusters = BTreeMap::new();
    for x in v.clusters {
        let k = cn(x.cluster)?;
        put(&mut clusters, k, mag(x.trust), "cluster trust")?;
    }
    let mut nodes = BTreeMap::new();
    for x in v.nodes {
        let k = nn(x.node)?;
        put(&mut nodes, k, mag(x.trust), "node trust")?;
    }
    let mut users = BTreeMap::new();
    for x in v.users {
        let k = un(x.user)?;
        put(&mut users, k, mag(x.trust), "user trust")?;
    }
    Ok(ClusterTrust {
        cluster: mag(v.cluster),
        clusters,
        nodes,
        users,
    })
}

// Goldragon authors Horizon's domain Datomic record. Convert that record at
// the producer boundary before sending this archiveable, typed Signal DTO.
// Keep every authored field here: a projection is not an input document.
macro_rules! reverse_enum {
    ($value:expr, $domain:ident => $wire:ident, $($variant:ident),+ $(,)?) => {
        match $value { $($domain::$variant => $wire::$variant,)+ }
    };
}

impl TryFrom<ClusterProposal> for ClusterProposalWire {
    type Error = HorizonWireConversionError;

    fn try_from(value: ClusterProposal) -> Result<Self> {
        Ok(Self {
            nodes: value.nodes.into_iter().map(|(name, proposal)| {
                Ok(NodeProposalEntryWire {
                    name: NodeNameWire(name.as_str().to_owned()),
                    proposal: wire_node(proposal)?,
                })
            }).collect::<Result<_>>()?,
            users: value.users.into_iter().map(|(name, proposal)| UserProposalEntryWire {
                name: UserNameWire(name.as_str().to_owned()),
                proposal: wire_user(proposal),
            }).collect(),
            domains: value.domains.into_iter().map(|(name, proposal)| DomainProposalEntryWire {
                name: DomainNameWire(name.as_str().to_owned()),
                proposal: DomainProposalWire {
                    species: reverse_enum!(proposal.species, DomainSpecies => DomainSpeciesWire, Cloudflare),
                },
            }).collect(),
            trust: wire_trust(value.trust),
            domain_configuration: DomainConfigurationWire {
                internal_suffix: InternalDomainSuffixWire(value.domain_configuration.internal_suffix.as_str().to_owned()),
                public_cluster_domains: value.domain_configuration.public_cluster_domains.into_iter()
                    .map(|domain| PublicClusterDomainWire(domain.as_str().to_owned())).collect(),
            },
        })
    }
}

fn wire_node(value: NodeProposal) -> Result<NodeProposalWire> {
    Ok(NodeProposalWire {
        species: reverse_enum!(value.species, NodeSpecies => NodeSpeciesWire,
            Center, LargeAi, LargeAiRouter, Hybrid, Edge, EdgeTesting,
            MediaBroadcast, Router, RouterTesting, TestVm, CloudNode),
        size: wire_magnitude(value.size),
        trust: wire_magnitude(value.trust),
        machine: wire_machine(value.machine),
        io: wire_io(value.io),
        pub_keys: NodePubKeysWire {
            ssh: SshPubKeyWire(value.pub_keys.ssh.as_str().to_owned()),
            nix: value
                .pub_keys
                .nix
                .map(|key| NixPubKeyWire(key.as_str().to_owned())),
            yggdrasil: value.pub_keys.yggdrasil.map(|entry| YggPubKeyEntryWire {
                pub_key: YggPubKeyWire(entry.pub_key.as_str().to_owned()),
                address: YggAddressWire(entry.address.to_string()),
                subnet: YggSubnetWire(entry.subnet.as_str().to_owned()),
            }),
        },
        link_local_ips: value
            .link_local_ips
            .into_iter()
            .map(|ip| LinkLocalIpWire {
                iface: InterfaceWire(ip.iface.as_str().to_owned()),
                suffix: ip.suffix,
            })
            .collect(),
        node_ip: value.node_ip.map(|ip| NodeIpWire(String::from(ip))),
        wireguard_pub_key: value
            .wireguard_pub_key
            .map(|key| WireguardPubKeyWire(key.as_str().to_owned())),
        nordvpn: value.nordvpn,
        wifi_cert: value.wifi_cert,
        wireguard_untrusted_proxies: value
            .wireguard_untrusted_proxies
            .into_iter()
            .map(|proxy| WireguardProxyWire {
                pub_key: WireguardPubKeyWire(proxy.pub_key.as_str().to_owned()),
                endpoint: proxy.endpoint,
                interface_ip: NodeIpWire(String::from(proxy.interface_ip)),
            })
            .collect(),
        wants_printing: value.wants_printing,
        wants_hw_video_accel: value.wants_hw_video_accel,
        router_interfaces: value.router_interfaces.map(wire_router),
        online: value.online,
        services: value
            .services
            .into_iter()
            .map(wire_service)
            .collect::<Result<_>>()?,
    })
}

fn wire_magnitude(value: Magnitude) -> MagnitudeWire {
    reverse_enum!(value, Magnitude => MagnitudeWire, Zero, Min, Medium, Large, Max)
}

fn wire_machine(value: Machine) -> MachineWire {
    MachineWire {
        species: reverse_enum!(value.species, MachineSpecies => MachineSpeciesWire, Metal, Pod),
        arch: value
            .arch
            .map(|arch| reverse_enum!(arch, Arch => ArchWire, X86_64, Arm64)),
        cores: i64::from(value.cores),
        model: value
            .model
            .map(|name| ModelNameWire(name.as_str().to_owned())),
        mother_board: value
            .mother_board
            .map(|board| reverse_enum!(board, MotherBoard => MotherBoardWire, Ondyfaind)),
        super_node: value
            .super_node
            .map(|name| NodeNameWire(name.as_str().to_owned())),
        super_user: value
            .super_user
            .map(|name| UserNameWire(name.as_str().to_owned())),
        chip_gen: value.chip_gen.map(i64::from),
        ram_gb: value.ram_gb.map(i64::from),
        disk_gb: value.disk_gb.map(i64::from),
        location: value
            .location
            .map(|location| LocationWire(location.as_str().to_owned())),
        super_nodes: value
            .super_nodes
            .into_iter()
            .map(|name| NodeNameWire(name.as_str().to_owned()))
            .collect(),
    }
}

fn wire_io(value: Io) -> IoWire {
    IoWire {
        keyboard: reverse_enum!(value.keyboard, Keyboard => KeyboardWire, Qwerty, Colemak),
        bootloader: reverse_enum!(value.bootloader, Bootloader => BootloaderWire, Uefi, Mbr, Uboot),
        disks: value
            .disks
            .into_iter()
            .map(|(mount_path, disk)| DiskByMountPathWire {
                mount_path: MountPathWire(mount_path.as_str().to_owned()),
                disk: DiskWire {
                    device: DevicePathWire(disk.device.as_str().to_owned()),
                    fs_type: reverse_enum!(disk.fs_type, FsType => FsTypeWire,
                    Ext2, Ext3, Ext4, Btrfs, Xfs, Zfs, F2fs, Bcachefs, Vfat, Exfat, Ntfs, Tmpfs),
                    options: disk.options,
                },
            })
            .collect(),
        swap_devices: value
            .swap_devices
            .into_iter()
            .map(|swap| SwapDeviceWire {
                device: DevicePathWire(swap.device.as_str().to_owned()),
                size_mebibytes: swap.size_mebibytes.map(i64::from),
            })
            .collect(),
        compressed_swap: value.compressed_swap.map(|swap| CompressedSwapWire {
            memory_percent: i64::from(swap.memory_percent),
        }),
    }
}

fn wire_router(value: RouterInterfaces) -> RouterInterfacesWire {
    RouterInterfacesWire {
        wan: InterfaceWire(value.wan.as_str().to_owned()),
        wlan: InterfaceWire(value.wlan.as_str().to_owned()),
        wlan_band: reverse_enum!(value.wlan_band, WlanBand => WlanBandWire, TwoG, FiveG, SixG),
        wlan_channel: i64::from(value.wlan_channel),
        wlan_standard: reverse_enum!(value.wlan_standard, WlanStandard => WlanStandardWire, Wifi4, Wifi6, Wifi7),
        wpa3_sae_password: value.wpa3_sae_password.map(|secret| SecretReferenceWire {
            name: SecretNameWire(secret.name.as_str().to_owned()),
        }),
        backup_wireless: value.backup_wireless.map(|wireless| BackupWirelessWire {
            interface: InterfaceWire(wireless.interface.as_str().to_owned()),
            network_name: WirelessNetworkNameWire(wireless.network_name.as_str().to_owned()),
            band: reverse_enum!(wireless.band, WlanBand => WlanBandWire, TwoG, FiveG, SixG),
            channel: i64::from(wireless.channel),
            standard: reverse_enum!(wireless.standard, WlanStandard => WlanStandardWire, Wifi4, Wifi6, Wifi7),
            password: SecretReferenceWire { name: SecretNameWire(wireless.password.name.as_str().to_owned()) },
        }),
    }
}

fn wire_service(value: NodeService) -> Result<NodeServiceWire> {
    Ok(match value {
        NodeService::TailnetClient {} => NodeServiceWire::TailnetClient,
        NodeService::TailnetController {} => NodeServiceWire::TailnetController,
        NodeService::NixBuilder { maximum_jobs } => NodeServiceWire::NixBuilder {
            maximum_jobs: maximum_jobs.map(i64::from),
        },
        NodeService::NixCache {} => NodeServiceWire::NixCache,
        NodeService::OpenCodeTesting {} => NodeServiceWire::OpenCodeTesting,
        NodeService::PersonaDevelopment { capabilities } => NodeServiceWire::PersonaDevelopment {
            capabilities: capabilities.into_iter().map(|capability| match capability {
                PersonaDevelopmentCapability::GitoliteServer => PersonaDevelopmentCapabilityWire::GitoliteServer,
            }).collect(),
        },
        NodeService::VmHost { guest_subnet, kvm, maximum_guests } => NodeServiceWire::VmHost {
            guest_subnet: TapSubnetWire(guest_subnet.to_string()),
            kvm: reverse_enum!(kvm, KvmAvailability => KvmAvailabilityWire, Available, Absent),
            maximum_guests: maximum_guests.map(|count| i64::from(count.count())),
        },
        NodeService::WebHost { sites } => NodeServiceWire::WebHost {
            sites: sites.into_iter().map(|site| HostedSiteWire {
                domain: ServedDomainWire(site.domain.as_str().to_owned()),
                source: SiteSourceWire(site.source.as_str().to_owned()),
                renderer: reverse_enum!(site.renderer, SiteRenderer => SiteRendererWire, MarkdownStatic),
            }).collect(),
        },
        NodeService::UsbIpv4Gateway { downstream, downstream_mac, gateway, uplink } => {
            if downstream == uplink {
                return Err(bad("USB gateway interfaces", downstream));
            }
            NodeServiceWire::UsbIpv4Gateway {
                downstream: InterfaceWire(downstream.as_str().to_owned()),
                downstream_mac: MacAddressWire(downstream_mac.to_string()),
                gateway: Ipv4CidrWire(gateway.to_string()),
                uplink: InterfaceWire(uplink.as_str().to_owned()),
            }
        }
    })
}

fn wire_user(value: UserProposal) -> UserProposalWire {
    UserProposalWire {
        species: reverse_enum!(value.species, UserSpecies => UserSpeciesWire, Code, Multimedia, Unlimited),
        size: wire_magnitude(value.size),
        keyboard: reverse_enum!(value.keyboard, Keyboard => KeyboardWire, Qwerty, Colemak),
        style: reverse_enum!(value.style, Style => StyleWire, Vim, Emacs),
        github_id: value
            .github_id
            .map(|id| GithubIdWire(id.as_str().to_owned())),
        fast_repeat: value.fast_repeat,
        pub_keys: value
            .pub_keys
            .into_iter()
            .map(|(node, key)| UserPubKeyEntryByNodeWire {
                node: NodeNameWire(node.as_str().to_owned()),
                key: UserPubKeyEntryWire {
                    ssh: SshPubKeyWire(key.ssh.as_str().to_owned()),
                    keygrip: KeygripWire(key.keygrip.as_str().to_owned()),
                },
            })
            .collect(),
        editor: value
            .editor
            .map(|editor| reverse_enum!(editor, Editor => EditorWire, Codium, Emacs)),
        text_size: value.text_size.map(|size| {
            reverse_enum!(size, TextSize => TextSizeWire,
            ExtraSmall, Small, Medium, Large, ExtraLarge)
        }),
    }
}

fn wire_trust(value: ClusterTrust) -> ClusterTrustWire {
    ClusterTrustWire {
        cluster: wire_magnitude(value.cluster),
        clusters: value
            .clusters
            .into_iter()
            .map(|(cluster, trust)| ClusterTrustByClusterWire {
                cluster: ClusterNameWire(cluster.as_str().to_owned()),
                trust: wire_magnitude(trust),
            })
            .collect(),
        nodes: value
            .nodes
            .into_iter()
            .map(|(node, trust)| ClusterTrustByNodeWire {
                node: NodeNameWire(node.as_str().to_owned()),
                trust: wire_magnitude(trust),
            })
            .collect(),
        users: value
            .users
            .into_iter()
            .map(|(user, trust)| ClusterTrustByUserWire {
                user: UserNameWire(user.as_str().to_owned()),
                trust: wire_magnitude(trust),
            })
            .collect(),
    }
}
