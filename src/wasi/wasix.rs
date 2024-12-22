

pub type sighandler_t = crate::size_t;
pub type pthread_t = core::ffi::c_ulong;
pub type pthread_key_t = core::ffi::c_uint;
pub type socklen_t = u32;
pub type in_addr_t = u32;
pub type in_port_t = u16;
pub type sa_family_t = u16;
pub type sa_type_t = u16;

s! {
    #[repr(C)]
    pub struct in_addr {
        pub s_addr: crate::in_addr_t,
    }

    #[repr(C)]
    #[repr(align(4))]
    pub struct in6_addr {
        pub s6_addr: [u8; 16],
    }

    #[repr(C)]
    pub struct sockaddr_un {
        pub sun_family: sa_family_t,
        pub sun_path: [core::ffi::c_char; 108],
    }

    #[repr(C)]
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [core::ffi::c_char; 14],
    }

    #[repr(C)]
    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: crate::in_port_t,
        pub sin_addr: crate::in_addr,
        pub sin_zero: [u8; 8],
    }

    #[repr(C)]
    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: crate::in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: crate::in6_addr,
        pub sin6_scope_id: u32,
    }

    #[repr(C)]
    pub struct addrinfo {
        pub ai_flags: core::ffi::c_int,
        pub ai_family: core::ffi::c_int,
        pub ai_socktype: core::ffi::c_int,
        pub ai_protocol: core::ffi::c_int,
        pub ai_addrlen: socklen_t,
        pub ai_addr: *mut crate::sockaddr,
        pub ai_canonname: *mut core::ffi::c_char,
        pub ai_next: *mut crate::addrinfo,
    }

    #[repr(C)]
    pub struct sockaddr_ll {
        pub sll_family: crate::c_ushort,
        pub sll_protocol: crate::c_ushort,
        pub sll_ifindex: core::ffi::c_int,
        pub sll_hatype: crate::c_ushort,
        pub sll_pkttype: crate::c_uchar,
        pub sll_halen: crate::c_uchar,
        pub sll_addr: [crate::c_uchar; 8]
    }

    #[repr(C)]
    pub struct sockaddr_storage {
        pub ss_family: sa_family_t,
        __ss_align: crate::size_t,
        #[cfg(target_pointer_width = "32")]
        __ss_pad2: [u8; 128 - 2 * 4],
        #[cfg(target_pointer_width = "64")]
        __ss_pad2: [u8; 128 - 2 * 8],
    }

    #[repr(C)]
    #[repr(align(4))]
    pub struct ifaddrs {
        pub ifa_next: *mut ifaddrs,
        pub ifa_name: *mut core::ffi::c_char,
        pub ifa_flags: core::ffi::c_int,
        pub ifa_addr: *mut crate::sockaddr,
        pub ifa_netmask: *mut crate::sockaddr,
        pub ifu_broadaddr_or_dstaddr: *mut crate::sockaddr,
        pub ifa_data: *mut crate::c_uchar,
    }

    #[repr(C)]
    #[repr(align(4))]
    pub struct msghdr {
        pub msg_name: *mut crate::c_uchar,
        pub msg_namelen: crate::socklen_t,
        pub msg_iov: *mut crate::iovec,
        pub msg_iovlen: crate::size_t,
        pub msg_control: *mut crate::c_uchar,
        pub msg_controllen: crate::socklen_t,
        pub msg_flags: core::ffi::c_int,
    }

    #[repr(C)]
    pub struct group_filter {
        pub gf_interface: u32,
        pub gf_group: crate::sockaddr_storage,
        pub gf_fmode: u32,
        pub gf_numsrc: u32,
        pub gf_slist: [crate::sockaddr_storage; 1],
    }

    #[repr(C)]
    pub struct group_req {
        pub gr_interface: u32,
        pub gr_group: crate::sockaddr_storage,
    }

    #[repr(C)]
    pub struct group_source_req {
        pub gsr_interface: u32,
        pub gsr_group: crate::sockaddr_storage,
        pub gsr_source: crate::sockaddr_storage,
    }

    #[repr(C)]
    pub struct in_pktinfo {
        pub ipi_ifindex: core::ffi::c_int,
        pub ipi_spec_dst: crate::in_addr,
        pub ipi_addr: crate::in_addr,
    }

    #[repr(C)]
    pub struct ip6_mtuinfo {
        pub ip6m_addr: crate::sockaddr_in6,
        pub ip6m_mtu: u32,
    }

    #[repr(C)]
    pub struct in6_pktinfo {
        pub ipi6_addr: crate::in6_addr,
        pub ipi6_ifindex: core::ffi::c_uint,
    }

    #[repr(C)]
    pub struct ip_mreq_source {
        pub imr_multiaddr: crate::in_addr,
        pub imr_interface: crate::in_addr,
        pub imr_sourceaddr: crate::in_addr,
    }

    #[repr(C)]
    pub struct ip_mreq {
        pub imr_multiaddr: crate::in_addr,
        pub imr_interface: crate::in_addr,
    }

    #[repr(C)]
    pub struct ip_mreqn {
        pub imr_multiaddr: crate::in_addr,
        pub imr_address: crate::in_addr,
        pub imr_ifindex: core::ffi::c_int,
    }

    #[repr(C)]
    pub struct ip_msfilter {
        pub imsf_multiaddr: crate::in_addr,
        pub imsf_interface: crate::in_addr,
        pub imsf_fmode: u32,
        pub imsf_numsrc: u32,
        pub imsf_slist: [crate::in_addr; 1],
    }

    #[repr(C)]
    pub struct ip_opts {
        pub ip_dst: crate::in_addr,
        pub ip_opts: [core::ffi::c_char; 40],
    }

    #[repr(C)]
    pub struct ipv6_mreq {
        pub ipv6mr_multiaddr: crate::in6_addr,
        pub ipv6mr_interface: core::ffi::c_uint,
    }

    #[repr(C)]
    pub struct sock_filter {
        pub code: u16,
        pub jt: u8,
        pub jf: u8,
        pub k: u32,
    }

    #[repr(C)]
    pub struct sock_fprog {
        pub len: crate::c_ushort,
        pub filter: *mut sock_filter,
    }

    #[repr(C)]
    pub struct pthread_attr_t {
        #[cfg(target_pointer_width = "32")]
        __size: [u32; 8],
        #[cfg(target_pointer_width = "64")]
        __size: [u64; 7],
    }

    #[repr(C)]
    pub struct sigset_t {
        #[cfg(target_pointer_width = "32")]
        __val: [u32; 2],
        #[cfg(target_pointer_width = "64")]
        __val: [u64; 1],
    }

    #[repr(C)]
    pub struct siginfo_t {
        pub si_signo: core::ffi::c_int,
        pub si_errno: core::ffi::c_int,
        pub si_code: core::ffi::c_int,
        _pad: [core::ffi::c_int; 31],
        _align: [u64; 0],
    }

    #[repr(C)]
    pub struct sigaction {
        pub sa_sigaction: crate::sighandler_t,
        pub sa_mask: crate::sigset_t,
        pub sa_flags: core::ffi::c_int,
        pub sa_restorer: Option<extern fn()>,
    }

    pub struct linger {
        pub l_onoff: core::ffi::c_int,
        pub l_linger: core::ffi::c_int,
    }

    pub struct sched_param {
        pub sched_priority: core::ffi::c_int,
        pub sched_ss_low_priority: core::ffi::c_int,
        pub sched_ss_repl_period: crate::timespec,
        pub sched_ss_init_budget: crate::timespec,
        pub sched_ss_max_repl: core::ffi::c_int,
    }

    pub struct posix_spawn_file_actions_t {
        __allocated: core::ffi::c_int,
        __used: core::ffi::c_int,
        __actions: *mut core::ffi::c_int,
        __pad: [core::ffi::c_int; 16],
    }

    pub struct posix_spawnattr_t {
        __flags: crate::c_short,
        __pgrp: crate::pid_t,
        __sd: crate::sigset_t,
        __ss: crate::sigset_t,
        __prio: core::ffi::c_int,
        __policy: core::ffi::c_int,
        __pad: [core::ffi::c_int; 16],
    }
}

pub const PTHREAD_STACK_MIN: crate::size_t = 2048;

pub const __WASI_SDFLAGS_RD: core::ffi::c_int = 1;
pub const __WASI_SDFLAGS_WR: core::ffi::c_int = 2;

pub const SHUT_RD: core::ffi::c_int = __WASI_SDFLAGS_RD;
pub const SHUT_WR: core::ffi::c_int = __WASI_SDFLAGS_WR;
pub const SHUT_RDWR: core::ffi::c_int = SHUT_RD | SHUT_WR;

pub const __WASI_RIFLAGS_RECV_PEEK: core::ffi::c_int = 1;
pub const __WASI_RIFLAGS_RECV_WAITALL: core::ffi::c_int = 2;
pub const __WASI_RIFLAGS_RECV_DATA_TRUNCATED: core::ffi::c_int = 4;

pub const MSG_OOB: core::ffi::c_int = 0x0001;
pub const MSG_PEEK: core::ffi::c_int = 0x0002;
pub const MSG_DONTROUTE: core::ffi::c_int = 0x0004;
pub const MSG_CTRUNC: core::ffi::c_int = 0x0008;
pub const MSG_PROXY: core::ffi::c_int = 0x0010;
pub const MSG_TRUNC: core::ffi::c_int = 0x0020;
pub const MSG_DONTWAIT: core::ffi::c_int = 0x0040;
pub const MSG_EOR: core::ffi::c_int = 0x0080;
pub const MSG_WAITALL: core::ffi::c_int = 0x0100;
pub const MSG_FIN: core::ffi::c_int = 0x0200;
pub const MSG_SYN: core::ffi::c_int = 0x0400;
pub const MSG_CONFIRM: core::ffi::c_int = 0x0800;
pub const MSG_RST: core::ffi::c_int = 0x1000;
pub const MSG_ERRQUEUE: core::ffi::c_int = 0x2000;
pub const MSG_NOSIGNAL: core::ffi::c_int = 0x4000;
pub const MSG_MORE: core::ffi::c_int = 0x8000;
pub const MSG_WAITFORONE: core::ffi::c_int = 0x10000;
pub const MSG_BATCH: core::ffi::c_int = 0x40000;
pub const MSG_ZEROCOPY: core::ffi::c_int = 0x4000000;
pub const MSG_FASTOPEN: core::ffi::c_int = 0x20000000;
pub const MSG_CMSG_CLOEXEC: core::ffi::c_int = 0x40000000;

pub const __WASI_FILETYPE_UNKNOWN: core::ffi::c_int = 0;
pub const __WASI_FILETYPE_BLOCK_DEVICE: core::ffi::c_int = 1;
pub const __WASI_FILETYPE_CHARACTER_DEVICE: core::ffi::c_int = 2;
pub const __WASI_FILETYPE_DIRECTORY: core::ffi::c_int = 3;
pub const __WASI_FILETYPE_REGULAR_FILE: core::ffi::c_int = 4;
pub const __WASI_FILETYPE_SOCKET_DGRAM: core::ffi::c_int = 5;
pub const __WASI_FILETYPE_SOCKET_STREAM: core::ffi::c_int = 6;
pub const __WASI_FILETYPE_SYMBOLIC_LINK: core::ffi::c_int = 7;
pub const __WASI_FILETYPE_SOCKET_RAW: core::ffi::c_int = 8;
pub const __WASI_FILETYPE_SOCKET_SEQPACKET: core::ffi::c_int = 9;

pub const __WASI_SOCK_TYPE_SOCKET_UNUSED: core::ffi::c_int = 0;
pub const __WASI_SOCK_TYPE_SOCKET_STREAM: core::ffi::c_int = 1;
pub const __WASI_SOCK_TYPE_SOCKET_DGRAM: core::ffi::c_int = 2;
pub const __WASI_SOCK_TYPE_SOCKET_RAW: core::ffi::c_int = 3;
pub const __WASI_SOCK_TYPE_SOCKET_SEQPACKET: core::ffi::c_int = 4;

pub const SOCK_DGRAM: core::ffi::c_int = __WASI_SOCK_TYPE_SOCKET_DGRAM;
pub const SOCK_STREAM: core::ffi::c_int = __WASI_SOCK_TYPE_SOCKET_STREAM;
pub const SOCK_RAW: core::ffi::c_int = __WASI_SOCK_TYPE_SOCKET_RAW;
pub const SOCK_SEQPACKET: core::ffi::c_int = __WASI_SOCK_TYPE_SOCKET_SEQPACKET;

pub const SOCK_NONBLOCK: core::ffi::c_int = 0x00004000;
pub const SOCK_CLOEXEC: core::ffi::c_int = 0x00002000;

pub const SOL_SOCKET: core::ffi::c_int = 0x7fffffff;
pub const SOL_IP: core::ffi::c_int = IPPROTO_IP;
pub const SOL_ICMP: core::ffi::c_int = IPPROTO_ICMP;
pub const SOL_IGMP: core::ffi::c_int = IPPROTO_IGMP;
pub const SOL_IPIP: core::ffi::c_int = IPPROTO_IPIP;
pub const SOL_TCP: core::ffi::c_int = IPPROTO_TCP;
pub const SOL_EGP: core::ffi::c_int = IPPROTO_EGP;
pub const SOL_PUP: core::ffi::c_int = IPPROTO_PUP;
pub const SOL_UDP: core::ffi::c_int = IPPROTO_UDP;
pub const SOL_IDP: core::ffi::c_int = IPPROTO_IDP;
pub const SOL_TP: core::ffi::c_int = IPPROTO_TP;
pub const SOL_DCCP: core::ffi::c_int = IPPROTO_DCCP;
pub const SOL_IPV6: core::ffi::c_int = IPPROTO_IPV6;
pub const SOL_ROUTING: core::ffi::c_int = IPPROTO_ROUTING;
pub const SOL_FRAGMENT: core::ffi::c_int = IPPROTO_FRAGMENT;
pub const SOL_RSVP: core::ffi::c_int = IPPROTO_RSVP;
pub const SOL_GRE: core::ffi::c_int = IPPROTO_GRE;
pub const SOL_ESP: core::ffi::c_int = IPPROTO_ESP;
pub const SOL_AH: core::ffi::c_int = IPPROTO_AH;
pub const SOL_ICMPV6: core::ffi::c_int = IPPROTO_ICMPV6;
pub const SOL_NONE: core::ffi::c_int = IPPROTO_NONE;
pub const SOL_DSTOPTS: core::ffi::c_int = IPPROTO_DSTOPTS;
pub const SOL_MTP: core::ffi::c_int = IPPROTO_MTP;
pub const SOL_BEETPH: core::ffi::c_int = IPPROTO_BEETPH;
pub const SOL_ENCAP: core::ffi::c_int = IPPROTO_ENCAP;
pub const SOL_PIM: core::ffi::c_int = IPPROTO_PIM;
pub const SOL_COMP: core::ffi::c_int = IPPROTO_COMP;
pub const SOL_SCTP: core::ffi::c_int = IPPROTO_SCTP;
pub const SOL_MH: core::ffi::c_int = IPPROTO_MH;
pub const SOL_UDPLITE: core::ffi::c_int = IPPROTO_UDPLITE;
pub const SOL_MPLS: core::ffi::c_int = IPPROTO_MPLS;
pub const SOL_ETHERNET: core::ffi::c_int = IPPROTO_ETHERNET;
pub const SOL_MPTCP: core::ffi::c_int = IPPROTO_MPTCP;

pub const __WASI_SOCK_OPTION_NOOP: core::ffi::c_int = 0;
pub const __WASI_SOCK_OPTION_REUSE_PORT: core::ffi::c_int = 1;
pub const __WASI_SOCK_OPTION_REUSE_ADDR: core::ffi::c_int = 2;
pub const __WASI_SOCK_OPTION_NO_DELAY: core::ffi::c_int = 3;
pub const __WASI_SOCK_OPTION_DONT_ROUTE: core::ffi::c_int = 4;
pub const __WASI_SOCK_OPTION_ONLY_V6: core::ffi::c_int = 5;
pub const __WASI_SOCK_OPTION_BROADCAST: core::ffi::c_int = 6;
pub const __WASI_SOCK_OPTION_MULTICAST_LOOP_V4: core::ffi::c_int = 7;
pub const __WASI_SOCK_OPTION_MULTICAST_LOOP_V6: core::ffi::c_int = 8;
pub const __WASI_SOCK_OPTION_PROMISCUOUS: core::ffi::c_int = 9;
pub const __WASI_SOCK_OPTION_LISTENING: core::ffi::c_int = 10;
pub const __WASI_SOCK_OPTION_LAST_ERROR: core::ffi::c_int = 11;
pub const __WASI_SOCK_OPTION_KEEP_ALIVE: core::ffi::c_int = 12;
pub const __WASI_SOCK_OPTION_LINGER: core::ffi::c_int = 13;
pub const __WASI_SOCK_OPTION_OOB_INLINE: core::ffi::c_int = 14;
pub const __WASI_SOCK_OPTION_RECV_BUF_SIZE: core::ffi::c_int = 15;
pub const __WASI_SOCK_OPTION_SEND_BUF_SIZE: core::ffi::c_int = 16;
pub const __WASI_SOCK_OPTION_RECV_LOWAT: core::ffi::c_int = 17;
pub const __WASI_SOCK_OPTION_SEND_LOWAT: core::ffi::c_int = 18;
pub const __WASI_SOCK_OPTION_RECV_TIMEOUT: core::ffi::c_int = 19;
pub const __WASI_SOCK_OPTION_SEND_TIMEOUT: core::ffi::c_int = 20;
pub const __WASI_SOCK_OPTION_CONNECT_TIMEOUT: core::ffi::c_int = 21;
pub const __WASI_SOCK_OPTION_ACCEPT_TIMEOUT: core::ffi::c_int = 22;
pub const __WASI_SOCK_OPTION_TTL: core::ffi::c_int = 23;
pub const __WASI_SOCK_OPTION_MULTICAST_TTL_V4: core::ffi::c_int = 24;
pub const __WASI_SOCK_OPTION_TYPE: core::ffi::c_int = 25;
pub const __WASI_SOCK_OPTION_PROTO: core::ffi::c_int = 26;

pub const SO_ACCEPTCONN: core::ffi::c_int = __WASI_SOCK_OPTION_LISTENING;
pub const SO_BROADCAST: core::ffi::c_int = __WASI_SOCK_OPTION_BROADCAST;
pub const SO_DONTROUTE: core::ffi::c_int = __WASI_SOCK_OPTION_DONT_ROUTE;
pub const SO_NODELAY: core::ffi::c_int = __WASI_SOCK_OPTION_NO_DELAY;
pub const SO_ERROR: core::ffi::c_int = __WASI_SOCK_OPTION_LAST_ERROR;
pub const SO_KEEPALIVE: core::ffi::c_int = __WASI_SOCK_OPTION_KEEP_ALIVE;
pub const SO_LINGER: core::ffi::c_int = __WASI_SOCK_OPTION_LINGER;
pub const SO_OOBINLINE: core::ffi::c_int = __WASI_SOCK_OPTION_OOB_INLINE;
pub const SO_ONLYV6: core::ffi::c_int = __WASI_SOCK_OPTION_ONLY_V6;
pub const SO_RCVBUF: core::ffi::c_int = __WASI_SOCK_OPTION_RECV_BUF_SIZE;
pub const SO_RCVLOWAT: core::ffi::c_int = __WASI_SOCK_OPTION_RECV_LOWAT;
pub const SO_RCVTIMEO: core::ffi::c_int = __WASI_SOCK_OPTION_RECV_TIMEOUT;
pub const SO_REUSEPORT: core::ffi::c_int = __WASI_SOCK_OPTION_REUSE_PORT;
pub const SO_REUSEADDR: core::ffi::c_int = __WASI_SOCK_OPTION_REUSE_ADDR;
pub const SO_SNDBUF: core::ffi::c_int = __WASI_SOCK_OPTION_SEND_BUF_SIZE;
pub const SO_SNDLOWAT: core::ffi::c_int = __WASI_SOCK_OPTION_SEND_LOWAT;
pub const SO_SNDTIMEO: core::ffi::c_int = __WASI_SOCK_OPTION_SEND_TIMEOUT;
pub const SO_MCASTLOOPV4: core::ffi::c_int = __WASI_SOCK_OPTION_MULTICAST_LOOP_V4;
pub const SO_MCASTLOOPV6: core::ffi::c_int = __WASI_SOCK_OPTION_MULTICAST_LOOP_V6;
pub const SO_CONNTIMEO: core::ffi::c_int = __WASI_SOCK_OPTION_CONNECT_TIMEOUT;
pub const SO_ACCPTIMEO: core::ffi::c_int = __WASI_SOCK_OPTION_ACCEPT_TIMEOUT;
pub const SO_TTL: core::ffi::c_int = __WASI_SOCK_OPTION_TTL;
pub const SO_MCASTTTLV4: core::ffi::c_int = __WASI_SOCK_OPTION_MULTICAST_TTL_V4;
pub const SO_TYPE: core::ffi::c_int = __WASI_SOCK_OPTION_TYPE;
pub const SO_PROTOCOL: core::ffi::c_int = __WASI_SOCK_OPTION_PROTO;
pub const SO_MARK: core::ffi::c_int = __WASI_SOCK_OPTION_NOOP;
pub const SO_BINDTODEVICE: core::ffi::c_int = __WASI_SOCK_OPTION_NOOP;
pub const SO_INCOMING_CPU: core::ffi::c_int = __WASI_SOCK_OPTION_NOOP;
pub const SO_ATTACH_FILTER: core::ffi::c_int = __WASI_SOCK_OPTION_NOOP;
pub const SO_DETACH_FILTER: core::ffi::c_int = __WASI_SOCK_OPTION_NOOP;

pub const AF_UNSPEC: core::ffi::c_int = 0;
pub const AF_INET: core::ffi::c_int = 1;
pub const AF_INET6: core::ffi::c_int = 2;
pub const AF_UNIX: core::ffi::c_int = 3;

pub const IF_NAMESIZE: crate::size_t = 16;
pub const IFNAMSIZ: crate::size_t = IF_NAMESIZE;

pub const IPPROTO_IP: core::ffi::c_int = 0;
pub const IPPROTO_HOPOPTS: core::ffi::c_int = 0;
pub const IPPROTO_ICMP: core::ffi::c_int = 1;
pub const IPPROTO_IGMP: core::ffi::c_int = 2;
pub const IPPROTO_IPIP: core::ffi::c_int = 4;
pub const IPPROTO_TCP: core::ffi::c_int = 6;
pub const IPPROTO_EGP: core::ffi::c_int = 8;
pub const IPPROTO_PUP: core::ffi::c_int = 12;
pub const IPPROTO_UDP: core::ffi::c_int = 17;
pub const IPPROTO_IDP: core::ffi::c_int = 22;
pub const IPPROTO_TP: core::ffi::c_int = 29;
pub const IPPROTO_DCCP: core::ffi::c_int = 33;
pub const IPPROTO_IPV6: core::ffi::c_int = 41;
pub const IPPROTO_ROUTING: core::ffi::c_int = 43;
pub const IPPROTO_FRAGMENT: core::ffi::c_int = 44;
pub const IPPROTO_RSVP: core::ffi::c_int = 46;
pub const IPPROTO_GRE: core::ffi::c_int = 47;
pub const IPPROTO_ESP: core::ffi::c_int = 50;
pub const IPPROTO_AH: core::ffi::c_int = 51;
pub const IPPROTO_ICMPV6: core::ffi::c_int = 58;
pub const IPPROTO_NONE: core::ffi::c_int = 59;
pub const IPPROTO_DSTOPTS: core::ffi::c_int = 60;
pub const IPPROTO_MTP: core::ffi::c_int = 92;
pub const IPPROTO_BEETPH: core::ffi::c_int = 94;
pub const IPPROTO_ENCAP: core::ffi::c_int = 98;
pub const IPPROTO_PIM: core::ffi::c_int = 103;
pub const IPPROTO_COMP: core::ffi::c_int = 108;
pub const IPPROTO_SCTP: core::ffi::c_int = 132;
pub const IPPROTO_MH: core::ffi::c_int = 135;
pub const IPPROTO_UDPLITE: core::ffi::c_int = 136;
pub const IPPROTO_MPLS: core::ffi::c_int = 137;
pub const IPPROTO_ETHERNET: core::ffi::c_int = 143;
pub const IPPROTO_RAW: core::ffi::c_int = 255;
pub const IPPROTO_MPTCP: core::ffi::c_int = 262;
pub const IPPROTO_MAX: core::ffi::c_int = 263;

pub const IP_TOS: core::ffi::c_int = 1;
pub const IP_TTL: core::ffi::c_int = 2;
pub const IP_HDRINCL: core::ffi::c_int = 3;
pub const IP_OPTIONS: core::ffi::c_int = 4;
pub const IP_ROUTER_ALERT: core::ffi::c_int = 5;
pub const IP_RECVOPTS: core::ffi::c_int = 6;
pub const IP_RETOPTS: core::ffi::c_int = 7;
pub const IP_PKTINFO: core::ffi::c_int = 8;
pub const IP_PKTOPTIONS: core::ffi::c_int = 9;
pub const IP_PMTUDISC: core::ffi::c_int = 10;
pub const IP_MTU_DISCOVER: core::ffi::c_int = 10;
pub const IP_RECVERR: core::ffi::c_int = 11;
pub const IP_RECVTTL: core::ffi::c_int = 12;
pub const IP_RECVTOS: core::ffi::c_int = 13;
pub const IP_MTU: core::ffi::c_int = 14;
pub const IP_FREEBIND: core::ffi::c_int = 15;
pub const IP_IPSEC_POLICY: core::ffi::c_int = 16;
pub const IP_XFRM_POLICY: core::ffi::c_int = 17;
pub const IP_PASSSEC: core::ffi::c_int = 18;
pub const IP_TRANSPARENT: core::ffi::c_int = 19;
pub const IP_ORIGDSTADDR: core::ffi::c_int = 20;
pub const IP_RECVORIGDSTADDR: core::ffi::c_int = IP_ORIGDSTADDR;
pub const IP_MINTTL: core::ffi::c_int = 21;
pub const IP_NODEFRAG: core::ffi::c_int = 22;
pub const IP_CHECKSUM: core::ffi::c_int = 23;
pub const IP_BIND_ADDRESS_NO_PORT: core::ffi::c_int = 24;
pub const IP_RECVFRAGSIZE: core::ffi::c_int = 25;
pub const IP_RECVERR_RFC4884: core::ffi::c_int = 26;
pub const IP_MULTICAST_IF: core::ffi::c_int = 32;
pub const IP_MULTICAST_TTL: core::ffi::c_int = 33;
pub const IP_MULTICAST_LOOP: core::ffi::c_int = 34;
pub const IP_ADD_MEMBERSHIP: core::ffi::c_int = 35;
pub const IP_DROP_MEMBERSHIP: core::ffi::c_int = 36;
pub const IP_UNBLOCK_SOURCE: core::ffi::c_int = 37;
pub const IP_BLOCK_SOURCE: core::ffi::c_int = 38;
pub const IP_ADD_SOURCE_MEMBERSHIP: core::ffi::c_int = 39;
pub const IP_DROP_SOURCE_MEMBERSHIP: core::ffi::c_int = 40;
pub const IP_MSFILTER: core::ffi::c_int = 41;
pub const IP_MULTICAST_ALL: core::ffi::c_int = 49;
pub const IP_UNICAST_IF: core::ffi::c_int = 50;

pub const IP_RECVRETOPTS: core::ffi::c_int = IP_RETOPTS;

pub const IP_PMTUDISC_DONT: core::ffi::c_int = 0;
pub const IP_PMTUDISC_WANT: core::ffi::c_int = 1;
pub const IP_PMTUDISC_DO: core::ffi::c_int = 2;
pub const IP_PMTUDISC_PROBE: core::ffi::c_int = 3;
pub const IP_PMTUDISC_INTERFACE: core::ffi::c_int = 4;
pub const IP_PMTUDISC_OMIT: core::ffi::c_int = 5;

pub const IP_DEFAULT_MULTICAST_TTL: core::ffi::c_int = 1;
pub const IP_DEFAULT_MULTICAST_LOOP: core::ffi::c_int = 1;
pub const IP_MAX_MEMBERSHIPS: core::ffi::c_int = 20;

pub const MCAST_JOIN_GROUP: core::ffi::c_int = 42;
pub const MCAST_BLOCK_SOURCE: core::ffi::c_int = 43;
pub const MCAST_UNBLOCK_SOURCE: core::ffi::c_int = 44;
pub const MCAST_LEAVE_GROUP: core::ffi::c_int = 45;
pub const MCAST_JOIN_SOURCE_GROUP: core::ffi::c_int = 46;
pub const MCAST_LEAVE_SOURCE_GROUP: core::ffi::c_int = 47;
pub const MCAST_MSFILTER: core::ffi::c_int = 48;

pub const MCAST_EXCLUDE: core::ffi::c_int = 0;
pub const MCAST_INCLUDE: core::ffi::c_int = 1;

pub const IPV6_ADDRFORM: core::ffi::c_int = 1;
pub const IPV6_2292PKTINFO: core::ffi::c_int = 2;
pub const IPV6_2292HOPOPTS: core::ffi::c_int = 3;
pub const IPV6_2292DSTOPTS: core::ffi::c_int = 4;
pub const IPV6_2292RTHDR: core::ffi::c_int = 5;
pub const IPV6_2292PKTOPTIONS: core::ffi::c_int = 6;
pub const IPV6_CHECKSUM: core::ffi::c_int = 7;
pub const IPV6_2292HOPLIMIT: core::ffi::c_int = 8;
pub const IPV6_NEXTHOP: core::ffi::c_int = 9;
pub const IPV6_AUTHHDR: core::ffi::c_int = 10;
pub const IPV6_UNICAST_HOPS: core::ffi::c_int = 16;
pub const IPV6_MULTICAST_IF: core::ffi::c_int = 17;
pub const IPV6_MULTICAST_HOPS: core::ffi::c_int = 18;
pub const IPV6_MULTICAST_LOOP: core::ffi::c_int = 19;
pub const IPV6_JOIN_GROUP: core::ffi::c_int = 20;
pub const IPV6_LEAVE_GROUP: core::ffi::c_int = 21;
pub const IPV6_ROUTER_ALERT: core::ffi::c_int = 22;
pub const IPV6_MTU_DISCOVER: core::ffi::c_int = 23;
pub const IPV6_MTU: core::ffi::c_int = 24;
pub const IPV6_RECVERR: core::ffi::c_int = 25;
pub const IPV6_V6ONLY: core::ffi::c_int = 26;
pub const IPV6_JOIN_ANYCAST: core::ffi::c_int = 27;
pub const IPV6_LEAVE_ANYCAST: core::ffi::c_int = 28;
pub const IPV6_MULTICAST_ALL: core::ffi::c_int = 29;
pub const IPV6_ROUTER_ALERT_ISOLATE: core::ffi::c_int = 30;
pub const IPV6_IPSEC_POLICY: core::ffi::c_int = 34;
pub const IPV6_XFRM_POLICY: core::ffi::c_int = 35;
pub const IPV6_HDRINCL: core::ffi::c_int = 36;

pub const IPV6_RECVPKTINFO: core::ffi::c_int = 49;
pub const IPV6_PKTINFO: core::ffi::c_int = 50;
pub const IPV6_RECVHOPLIMIT: core::ffi::c_int = 51;
pub const IPV6_HOPLIMIT: core::ffi::c_int = 52;
pub const IPV6_RECVHOPOPTS: core::ffi::c_int = 53;
pub const IPV6_HOPOPTS: core::ffi::c_int = 54;
pub const IPV6_RTHDRDSTOPTS: core::ffi::c_int = 55;
pub const IPV6_RECVRTHDR: core::ffi::c_int = 56;
pub const IPV6_RTHDR: core::ffi::c_int = 57;
pub const IPV6_RECVDSTOPTS: core::ffi::c_int = 58;
pub const IPV6_DSTOPTS: core::ffi::c_int = 59;
pub const IPV6_RECVPATHMTU: core::ffi::c_int = 60;
pub const IPV6_PATHMTU: core::ffi::c_int = 61;
pub const IPV6_DONTFRAG: core::ffi::c_int = 62;
pub const IPV6_RECVTCLASS: core::ffi::c_int = 66;
pub const IPV6_TCLASS: core::ffi::c_int = 67;
pub const IPV6_AUTOFLOWLABEL: core::ffi::c_int = 70;
pub const IPV6_ADDR_PREFERENCES: core::ffi::c_int = 72;
pub const IPV6_MINHOPCOUNT: core::ffi::c_int = 73;
pub const IPV6_ORIGDSTADDR: core::ffi::c_int = 74;
pub const IPV6_RECVORIGDSTADDR: core::ffi::c_int = IPV6_ORIGDSTADDR;
pub const IPV6_TRANSPARENT: core::ffi::c_int = 75;
pub const IPV6_UNICAST_IF: core::ffi::c_int = 76;
pub const IPV6_RECVFRAGSIZE: core::ffi::c_int = 77;
pub const IPV6_FREEBIND: core::ffi::c_int = 78;

pub const IPV6_ADD_MEMBERSHIP: core::ffi::c_int = IPV6_JOIN_GROUP;
pub const IPV6_DROP_MEMBERSHIP: core::ffi::c_int = IPV6_LEAVE_GROUP;
pub const IPV6_RXHOPOPTS: core::ffi::c_int = IPV6_HOPOPTS;
pub const IPV6_RXDSTOPTS: core::ffi::c_int = IPV6_DSTOPTS;

pub const IPV6_PMTUDISC_DONT: core::ffi::c_int = 0;
pub const IPV6_PMTUDISC_WANT: core::ffi::c_int = 1;
pub const IPV6_PMTUDISC_DO: core::ffi::c_int = 2;
pub const IPV6_PMTUDISC_PROBE: core::ffi::c_int = 3;
pub const IPV6_PMTUDISC_INTERFACE: core::ffi::c_int = 4;
pub const IPV6_PMTUDISC_OMIT: core::ffi::c_int = 5;

pub const IPV6_PREFER_SRC_TMP: core::ffi::c_int = 0x0001;
pub const IPV6_PREFER_SRC_PUBLIC: core::ffi::c_int = 0x0002;
pub const IPV6_PREFER_SRC_PUBTMP_DEFAULT: core::ffi::c_int = 0x0100;
pub const IPV6_PREFER_SRC_COA: core::ffi::c_int = 0x0004;
pub const IPV6_PREFER_SRC_HOME: core::ffi::c_int = 0x0400;
pub const IPV6_PREFER_SRC_CGA: core::ffi::c_int = 0x0008;
pub const IPV6_PREFER_SRC_NONCGA: core::ffi::c_int = 0x0800;

pub const IPV6_RTHDR_LOOSE: core::ffi::c_int = 0;
pub const IPV6_RTHDR_STRICT: core::ffi::c_int = 1;

pub const IPV6_RTHDR_TYPE_0: core::ffi::c_int = 0;

pub const TCP_NODELAY: core::ffi::c_int = 1;
pub const TCP_MAXSEG: core::ffi::c_int = 2;
pub const TCP_CORK: core::ffi::c_int = 3;
pub const TCP_KEEPIDLE: core::ffi::c_int = 4;
pub const TCP_KEEPINTVL: core::ffi::c_int = 5;
pub const TCP_KEEPCNT: core::ffi::c_int = 6;
pub const TCP_SYNCNT: core::ffi::c_int = 7;
pub const TCP_LINGER2: core::ffi::c_int = 8;
pub const TCP_DEFER_ACCEPT: core::ffi::c_int = 9;
pub const TCP_WINDOW_CLAMP: core::ffi::c_int = 10;
pub const TCP_INFO: core::ffi::c_int = 11;
pub const TCP_QUICKACK: core::ffi::c_int = 12;
pub const TCP_CONGESTION: core::ffi::c_int = 13;
pub const TCP_MD5SIG: core::ffi::c_int = 14;
pub const TCP_THIN_LINEAR_TIMEOUTS: core::ffi::c_int = 16;
pub const TCP_THIN_DUPACK: core::ffi::c_int = 17;
pub const TCP_USER_TIMEOUT: core::ffi::c_int = 18;
pub const TCP_REPAIR: core::ffi::c_int = 19;
pub const TCP_REPAIR_QUEUE: core::ffi::c_int = 20;
pub const TCP_QUEUE_SEQ: core::ffi::c_int = 21;
pub const TCP_REPAIR_OPTIONS: core::ffi::c_int = 22;
pub const TCP_FASTOPEN: core::ffi::c_int = 23;
pub const TCP_TIMESTAMP: core::ffi::c_int = 24;
pub const TCP_NOTSENT_LOWAT: core::ffi::c_int = 25;
pub const TCP_CC_INFO: core::ffi::c_int = 26;
pub const TCP_SAVE_SYN: core::ffi::c_int = 27;
pub const TCP_SAVED_SYN: core::ffi::c_int = 28;
pub const TCP_REPAIR_WINDOW: core::ffi::c_int = 29;
pub const TCP_FASTOPEN_CONNECT: core::ffi::c_int = 30;
pub const TCP_ULP: core::ffi::c_int = 31;
pub const TCP_MD5SIG_EXT: core::ffi::c_int = 32;
pub const TCP_FASTOPEN_KEY: core::ffi::c_int = 33;
pub const TCP_FASTOPEN_NO_COOKIE: core::ffi::c_int = 34;
pub const TCP_ZEROCOPY_RECEIVE: core::ffi::c_int = 35;
pub const TCP_INQ: core::ffi::c_int = 36;
pub const TCP_TX_DELAY: core::ffi::c_int = 37;

pub const TCP_CM_INQ: core::ffi::c_int = TCP_INQ;

pub const TCP_ESTABLISHED: core::ffi::c_int = 1;
pub const TCP_SYN_SENT: core::ffi::c_int = 2;
pub const TCP_SYN_RECV: core::ffi::c_int = 3;
pub const TCP_FIN_WAIT1: core::ffi::c_int = 4;
pub const TCP_FIN_WAIT2: core::ffi::c_int = 5;
pub const TCP_TIME_WAIT: core::ffi::c_int = 6;
pub const TCP_CLOSE: core::ffi::c_int = 7;
pub const TCP_CLOSE_WAIT: core::ffi::c_int = 8;
pub const TCP_LAST_ACK: core::ffi::c_int = 9;
pub const TCP_LISTEN: core::ffi::c_int = 10;
pub const TCP_CLOSING: core::ffi::c_int = 11;

pub const PRIO_MIN: core::ffi::c_int = -20;
pub const PRIO_MAX: core::ffi::c_int = 20;

pub const INADDR_LOOPBACK: in_addr_t = 2130706433;
pub const INADDR_ANY: in_addr_t = 0;
pub const INADDR_BROADCAST: in_addr_t = 4294967295;
pub const INADDR_NONE: in_addr_t = 4294967295;

pub const INADDR_UNSPEC_GROUP: in_addr_t = 0xe0000000;
pub const INADDR_ALLHOSTS_GROUP: in_addr_t = 0xe0000001;
pub const INADDR_ALLRTRS_GROUP: in_addr_t = 0xe0000002;
pub const INADDR_ALLSNOOPERS_GROUP: in_addr_t = 0xe000006a;
pub const INADDR_MAX_LOCAL_GROUP: in_addr_t = 0xe00000ff;

pub const ARPOP_REQUEST: u16 = 1;
pub const ARPOP_REPLY: u16 = 2;

pub const POSIX_SPAWN_RESETIDS: core::ffi::c_int = 0x01;
pub const POSIX_SPAWN_SETPGROUP: core::ffi::c_int = 0x02;
pub const POSIX_SPAWN_SETSIGDEF: core::ffi::c_int = 0x04;
pub const POSIX_SPAWN_SETSIGMASK: core::ffi::c_int = 0x08;
pub const POSIX_SPAWN_SETSCHEDPARAM: core::ffi::c_int = 0x10;
pub const POSIX_SPAWN_SETSCHEDULER: core::ffi::c_int = 0x20;

pub const WNOHANG: core::ffi::c_int = 0x00000001;
pub const WUNTRACED: core::ffi::c_int = 0x00000002;
pub const WSTOPPED: core::ffi::c_int = WUNTRACED;
pub const WEXITED: core::ffi::c_int = 0x00000004;
pub const WCONTINUED: core::ffi::c_int = 0x00000008;
pub const WNOWAIT: core::ffi::c_int = 0x01000000;

pub const SIG_DFL: sighandler_t = 0 as sighandler_t;
pub const SIG_IGN: sighandler_t = 1 as sighandler_t;
pub const SIG_ERR: sighandler_t = !0 as sighandler_t;

pub const SIGHUP: core::ffi::c_int = 1;
pub const SIGINT: core::ffi::c_int = 2;
pub const SIGQUIT: core::ffi::c_int = 3;
pub const SIGILL: core::ffi::c_int = 4;
pub const SIGTRAP: core::ffi::c_int = 5;
pub const SIGABRT: core::ffi::c_int = 6;
pub const SIGBUS: core::ffi::c_int = 7;
pub const SIGFPE: core::ffi::c_int = 8;
pub const SIGKILL: core::ffi::c_int = 9;
pub const SIGUSR1: core::ffi::c_int = 10;
pub const SIGSEGV: core::ffi::c_int = 11;
pub const SIGUSR2: core::ffi::c_int = 12;
pub const SIGPIPE: core::ffi::c_int = 13;
pub const SIGALRM: core::ffi::c_int = 14;
pub const SIGTERM: core::ffi::c_int = 15;
pub const SIGSTKFLT: core::ffi::c_int = 16;
pub const SIGCHLD: core::ffi::c_int = 17;
pub const SIGCONT: core::ffi::c_int = 18;
pub const SIGSTOP: core::ffi::c_int = 19;
pub const SIGTSTP: core::ffi::c_int = 20;
pub const SIGTTIN: core::ffi::c_int = 21;
pub const SIGTTOU: core::ffi::c_int = 22;
pub const SIGURG: core::ffi::c_int = 23;
pub const SIGXCPU: core::ffi::c_int = 24;
pub const SIGXFSZ: core::ffi::c_int = 25;
pub const SIGVTALRM: core::ffi::c_int = 26;
pub const SIGPROF: core::ffi::c_int = 27;
pub const SIGWINCH: core::ffi::c_int = 28;
pub const SIGIO: core::ffi::c_int = 29;
pub const SIGPOLL: core::ffi::c_int = 29;
pub const SIGPWR: core::ffi::c_int = 30;
pub const SIGSYS: core::ffi::c_int = 31;

pub const SIG_BLOCK: core::ffi::c_int = 0;
pub const SIG_UNBLOCK: core::ffi::c_int = 1;
pub const SIG_SETMASK: core::ffi::c_int = 2;

pub const SA_NOCLDSTOP: core::ffi::c_int = 1;
pub const SA_NOCLDWAIT: core::ffi::c_int = 2;
pub const SA_SIGINFO: core::ffi::c_int = 4;
pub const SA_ONSTACK: core::ffi::c_int = 0x08000000;
pub const SA_RESTART: core::ffi::c_int = 0x10000000;
pub const SA_NODEFER: core::ffi::c_int = 0x40000000;
pub const SA_RESETHAND: core::ffi::c_int = 0x80000000;
pub const SA_RESTORER: core::ffi::c_int = 0x04000000;

#[cfg_attr(
    feature = "rustc-dep-of-std",
    link(
        name = "c",
        kind = "static",
        modifiers = "-bundle",
        cfg(target_feature = "crt-static")
    )
)]
#[cfg_attr(
    feature = "rustc-dep-of-std",
    link(name = "c", cfg(not(target_feature = "crt-static")))
)]
extern "C" {
    pub fn dup(fd: core::ffi::c_int) -> core::ffi::c_int;
    pub fn dup2(src: core::ffi::c_int, dst: core::ffi::c_int) -> core::ffi::c_int;

    pub fn accept(socket: core::ffi::c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> core::ffi::c_int;
    pub fn accept4(
        socket: core::ffi::c_int,
        addr: *mut sockaddr,
        addrlen: *mut socklen_t,
        flags: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn bind(socket: core::ffi::c_int, addr: *const crate::sockaddr, addrlen: crate::socklen_t) -> core::ffi::c_int;
    pub fn connect(socket: core::ffi::c_int, addr: *const sockaddr, addrlen: socklen_t) -> core::ffi::c_int;
    pub fn freeifaddrs(ifa: *mut crate::ifaddrs);
    pub fn getifaddrs(ifap: *mut *mut crate::ifaddrs) -> core::ffi::c_int;
    pub fn getpeername(socket: core::ffi::c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> core::ffi::c_int;
    pub fn getsockname(socket: core::ffi::c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> core::ffi::c_int;
    pub fn getsockopt(
        sockfd: core::ffi::c_int,
        level: core::ffi::c_int,
        option_name: core::ffi::c_int,
        option_value: *mut core::ffi::c_void,
        option_len: *mut crate::socklen_t,
    ) -> core::ffi::c_int;
    pub fn listen(socket: core::ffi::c_int, backlog: core::ffi::c_int) -> core::ffi::c_int;
    pub fn recvfrom(
        socket: core::ffi::c_int,
        buffer: *mut core::ffi::c_void,
        length: crate::size_t,
        flags: core::ffi::c_int,
        addr: *mut crate::sockaddr,
        addrlen: *mut crate::socklen_t,
    ) -> crate::ssize_t;
    pub fn recvmsg(socket: core::ffi::c_int, msg: *mut crate::msghdr, flags: core::ffi::c_int) -> crate::ssize_t;
    pub fn sendfile(
        socket: core::ffi::c_int,
        in_fd: core::ffi::c_int,
        ofs: *const crate::off_t,
        count: crate::size_t,
    ) -> crate::ssize_t;
    pub fn sendmsg(socket: core::ffi::c_int, msg: *const crate::msghdr, flags: core::ffi::c_int) -> crate::ssize_t;
    pub fn sendto(
        socket: core::ffi::c_int,
        buffer: *const core::ffi::c_void,
        length: crate::size_t,
        flags: core::ffi::c_int,
        addr: *const sockaddr,
        addrlen: socklen_t,
    ) -> crate::ssize_t;
    pub fn setsockopt(
        socket: core::ffi::c_int,
        level: core::ffi::c_int,
        option_name: core::ffi::c_int,
        option_value: *const core::ffi::c_void,
        option_len: socklen_t,
    ) -> core::ffi::c_int;
    pub fn socket(domain: core::ffi::c_int, ty: core::ffi::c_int, protocol: core::ffi::c_int) -> core::ffi::c_int;
    pub fn socketpair(
        domain: core::ffi::c_int,
        ty: core::ffi::c_int,
        protocol: core::ffi::c_int,
        socket_vector: *mut core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub fn __wasilibc_initialize_environ();
    pub fn __wasilibc_get_stack_pointer() -> *mut core::ffi::c_void;
    pub fn __wasilibc_set_stack_pointer(val: *mut core::ffi::c_void);
    pub fn __wasilibc_get_pthread_self() -> *mut core::ffi::c_void;
    pub fn __wasilibc_set_pthread_self(val: *mut core::ffi::c_void);
    pub fn __wasilibc_init_tls(val: *mut core::ffi::c_void);
    pub fn __wasilibc_tls_size() -> u64;
    pub fn __wasilibc_tls_align() -> u64;
    pub fn __wasilibc_get_tls_base() -> *mut core::ffi::c_void;
    pub fn __wasilibc_set_tls_base(val: *mut core::ffi::c_void);

    pub fn getpid() -> crate::pid_t;

    pub fn execl(path: *const core::ffi::c_char, arg0: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    pub fn execle(path: *const core::ffi::c_char, arg0: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    pub fn execlp(file: *const core::ffi::c_char, arg0: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    pub fn execv(prog: *const core::ffi::c_char, argv: *const *const core::ffi::c_char) -> core::ffi::c_int;
    pub fn execve(
        prog: *const core::ffi::c_char,
        argv: *const *const core::ffi::c_char,
        envp: *const *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    pub fn execvp(c: *const core::ffi::c_char, argv: *const *const core::ffi::c_char) -> core::ffi::c_int;
    pub fn fork() -> crate::pid_t;

    pub fn posix_spawn(
        pid: *mut crate::pid_t,
        path: *const core::ffi::c_char,
        file_actions: *const crate::posix_spawn_file_actions_t,
        attrp: *const crate::posix_spawnattr_t,
        argv: *const *mut core::ffi::c_char,
        envp: *const *mut core::ffi::c_char,
    ) -> core::ffi::c_int;
    pub fn posix_spawnp(
        pid: *mut crate::pid_t,
        file: *const core::ffi::c_char,
        file_actions: *const crate::posix_spawn_file_actions_t,
        attrp: *const crate::posix_spawnattr_t,
        argv: *const *mut core::ffi::c_char,
        envp: *const *mut core::ffi::c_char,
    ) -> core::ffi::c_int;
    pub fn posix_spawnattr_init(attr: *mut posix_spawnattr_t) -> core::ffi::c_int;
    pub fn posix_spawnattr_destroy(attr: *mut posix_spawnattr_t) -> core::ffi::c_int;
    pub fn posix_spawnattr_getsigdefault(
        attr: *const posix_spawnattr_t,
        default: *mut crate::sigset_t,
    ) -> core::ffi::c_int;
    pub fn posix_spawnattr_setsigdefault(
        attr: *mut posix_spawnattr_t,
        default: *const crate::sigset_t,
    ) -> core::ffi::c_int;
    pub fn posix_spawnattr_getsigmask(
        attr: *const posix_spawnattr_t,
        default: *mut crate::sigset_t,
    ) -> core::ffi::c_int;
    pub fn posix_spawnattr_setsigmask(
        attr: *mut posix_spawnattr_t,
        default: *const crate::sigset_t,
    ) -> core::ffi::c_int;
    pub fn posix_spawnattr_getflags(
        attr: *const posix_spawnattr_t,
        flags: *mut crate::c_short,
    ) -> core::ffi::c_int;
    pub fn posix_spawnattr_setflags(attr: *mut posix_spawnattr_t, flags: crate::c_short) -> core::ffi::c_int;
    pub fn posix_spawnattr_getpgroup(
        attr: *const posix_spawnattr_t,
        flags: *mut crate::pid_t,
    ) -> core::ffi::c_int;
    pub fn posix_spawnattr_setpgroup(attr: *mut posix_spawnattr_t, flags: crate::pid_t) -> core::ffi::c_int;
    pub fn posix_spawnattr_getschedpolicy(
        attr: *const posix_spawnattr_t,
        flags: *mut core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn posix_spawnattr_setschedpolicy(attr: *mut posix_spawnattr_t, flags: core::ffi::c_int) -> core::ffi::c_int;
    pub fn posix_spawnattr_getschedparam(
        attr: *const posix_spawnattr_t,
        param: *mut crate::sched_param,
    ) -> core::ffi::c_int;
    pub fn posix_spawnattr_setschedparam(
        attr: *mut posix_spawnattr_t,
        param: *const crate::sched_param,
    ) -> core::ffi::c_int;

    pub fn posix_spawn_file_actions_init(actions: *mut posix_spawn_file_actions_t) -> core::ffi::c_int;
    pub fn posix_spawn_file_actions_destroy(actions: *mut posix_spawn_file_actions_t) -> core::ffi::c_int;
    pub fn posix_spawn_file_actions_addopen(
        actions: *mut posix_spawn_file_actions_t,
        fd: core::ffi::c_int,
        path: *const core::ffi::c_char,
        oflag: core::ffi::c_int,
        mode: crate::mode_t,
    ) -> core::ffi::c_int;
    pub fn posix_spawn_file_actions_addclose(
        actions: *mut posix_spawn_file_actions_t,
        fd: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn posix_spawn_file_actions_adddup2(
        actions: *mut posix_spawn_file_actions_t,
        fd: core::ffi::c_int,
        newfd: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub fn wait(status: *mut core::ffi::c_int) -> crate::pid_t;
    pub fn waitpid(pid: crate::pid_t, status: *mut core::ffi::c_int, options: core::ffi::c_int) -> crate::pid_t;
    pub fn kill(pid: crate::pid_t, sig: core::ffi::c_int) -> core::ffi::c_int;

    pub fn sigemptyset(set: *mut sigset_t) -> core::ffi::c_int;
    pub fn sigaddset(set: *mut sigset_t, signum: core::ffi::c_int) -> core::ffi::c_int;
    pub fn sigfillset(set: *mut sigset_t) -> core::ffi::c_int;
    pub fn sigdelset(set: *mut sigset_t, signum: core::ffi::c_int) -> core::ffi::c_int;
    pub fn sigismember(set: *const sigset_t, signum: core::ffi::c_int) -> core::ffi::c_int;

    pub fn sigprocmask(how: core::ffi::c_int, set: *const sigset_t, oldset: *mut sigset_t) -> core::ffi::c_int;
    pub fn sigpending(set: *mut sigset_t) -> core::ffi::c_int;

    pub fn pthread_self() -> crate::pthread_t;
    pub fn pthread_join(native: crate::pthread_t, value: *mut *mut core::ffi::c_void) -> core::ffi::c_int;
    pub fn pthread_exit(value: *mut core::ffi::c_void) -> !;
    pub fn pthread_attr_init(attr: *mut crate::pthread_attr_t) -> core::ffi::c_int;
    pub fn pthread_attr_destroy(attr: *mut crate::pthread_attr_t) -> core::ffi::c_int;
    pub fn pthread_attr_setstacksize(attr: *mut crate::pthread_attr_t, stack_size: crate::size_t) -> core::ffi::c_int;
    pub fn pthread_attr_setdetachstate(attr: *mut crate::pthread_attr_t, state: core::ffi::c_int) -> core::ffi::c_int;
    pub fn pthread_detach(thread: crate::pthread_t) -> core::ffi::c_int;
    pub fn pthread_create(
        native: *mut crate::pthread_t,
        attr: *const crate::pthread_attr_t,
        f: extern "C" fn(*mut core::ffi::c_void) -> *mut core::ffi::c_void,
        value: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    pub fn pthread_key_create(
        key: *mut pthread_key_t,
        dtor: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    ) -> core::ffi::c_int;
    pub fn pthread_key_delete(key: pthread_key_t) -> core::ffi::c_int;
    pub fn pthread_getspecific(key: pthread_key_t) -> *mut core::ffi::c_void;
    pub fn pthread_setspecific(key: pthread_key_t, value: *const core::ffi::c_void) -> core::ffi::c_int;
    pub fn raise(signum: core::ffi::c_int) -> core::ffi::c_int;

    fn sigaction_external_default(
        sig: core::ffi::c_int,
        sa: *const sigaction,
        old: *mut sigaction,
        _external_handler: Option<unsafe extern "C" fn(core::ffi::c_int)>,
    ) -> core::ffi::c_int;
    fn __wasm_signal(signum: core::ffi::c_int);
}

pub unsafe fn sigaction(sig: core::ffi::c_int, sa: *const sigaction, old: *mut sigaction) -> core::ffi::c_int {
    sigaction_external_default(sig, sa, old, Some(default_handler))
}

extern "C" fn default_handler(sig: core::ffi::c_int) {
    if sig == SIGCHLD || sig == SIGURG || sig == SIGWINCH || sig == SIGCONT {
        return;
    } else {
        unsafe { crate::abort() };
    }
}

/// mocked functions that dont do anything in WASI land
pub fn mlock(_addr: *const core::ffi::c_void, _len: crate::size_t) -> core::ffi::c_int {
    0
}
pub fn munlock(_addr: *const core::ffi::c_void, _len: crate::size_t) -> core::ffi::c_int {
    0
}
pub fn mlockall(_flags: core::ffi::c_int) -> core::ffi::c_int {
    0
}
pub fn munlockall() -> core::ffi::c_int {
    0
}

pub fn WIFSTOPPED(status: core::ffi::c_int) -> bool {
    (status & 0xff) == 0x7f
}

pub fn WSTOPSIG(status: core::ffi::c_int) -> core::ffi::c_int {
    (status >> 8) & 0xff
}

pub fn WIFCONTINUED(status: core::ffi::c_int) -> bool {
    status == 0xffff
}

pub fn WIFSIGNALED(status: core::ffi::c_int) -> bool {
    ((status & 0x7f) + 1) as i8 >= 2
}

pub fn WTERMSIG(status: core::ffi::c_int) -> core::ffi::c_int {
    status & 0x7f
}

pub fn WIFEXITED(status: core::ffi::c_int) -> bool {
    (status & 0x7f) == 0
}

pub fn WEXITSTATUS(status: core::ffi::c_int) -> core::ffi::c_int {
    (status >> 8) & 0xff
}

pub fn WCOREDUMP(status: core::ffi::c_int) -> bool {
    (status & 0x80) != 0
}

pub fn W_EXITCODE(ret: core::ffi::c_int, sig: core::ffi::c_int) -> core::ffi::c_int {
    (ret << 8) | sig
}

pub fn W_STOPCODE(sig: core::ffi::c_int) -> core::ffi::c_int {
    (sig << 8) | 0x7f
}

pub fn QCMD(cmd: core::ffi::c_int, type_: core::ffi::c_int) -> core::ffi::c_int {
    (cmd << 8) | (type_ & 0x00ff)
}
