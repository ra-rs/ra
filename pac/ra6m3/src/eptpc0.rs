#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SYNFP Status Register"]
    pub sysr: SYSR,
    #[doc = "0x04 - SYNFP Status Notification Permission Register"]
    pub syipr: SYIPR,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - SYNFP MAC Address Registers"]
    pub symacru: SYMACRU,
    #[doc = "0x14 - SYNFP MAC Address Registers"]
    pub symacrl: SYMACRL,
    #[doc = "0x18 - SYNFP LLC-CTL Value Register"]
    pub syllcctlr: SYLLCCTLR,
    #[doc = "0x1c - SYNFP Local IP Address Register"]
    pub syipaddrr: SYIPADDRR,
    _reserved6: [u8; 0x20],
    #[doc = "0x40 - SYNFP Specification Version Setting Register"]
    pub syspvrr: SYSPVRR,
    #[doc = "0x44 - SYNFP Domain Number Setting Register"]
    pub sydomr: SYDOMR,
    _reserved8: [u8; 0x08],
    #[doc = "0x50 - Announce Message Flag Field Setting Register"]
    pub anfr: ANFR,
    #[doc = "0x54 - Sync Message Flag Field Setting Register"]
    pub synfr: SYNFR,
    #[doc = "0x58 - Delay_Req Message Flag Field Setting Register"]
    pub dyrqfr: DYRQFR,
    #[doc = "0x5c - Delay_Resp Message Flag Field Setting Register"]
    pub dyrpfr: DYRPFR,
    #[doc = "0x60 - SYNFP Local Clock ID Registers"]
    pub sycidru: SYCIDRU,
    #[doc = "0x64 - SYNFP Local Clock ID Registers"]
    pub sycidrl: SYCIDRL,
    #[doc = "0x68 - SYNFP Local Port Number Register"]
    pub sypnumr: SYPNUMR,
    _reserved15: [u8; 0x14],
    #[doc = "0x80 - SYNFP Register Value Load Directive Register"]
    pub syrvldr: SYRVLDR,
    _reserved16: [u8; 0x0c],
    #[doc = "0x90 - SYNFP Reception Filter Register 1"]
    pub syrfl1r: SYRFL1R,
    #[doc = "0x94 - SYNFP Reception Filter Register 2"]
    pub syrfl2r: SYRFL2R,
    #[doc = "0x98 - SYNFP Transmission Enable Register"]
    pub sytrenr: SYTRENR,
    _reserved19: [u8; 0x04],
    #[doc = "0xa0 - Master Clock ID Registers"]
    pub mtcidu: MTCIDU,
    #[doc = "0xa4 - Master Clock ID Registers"]
    pub mtcidl: MTCIDL,
    #[doc = "0xa8 - Master clock port number register"]
    pub mtpid: MTPID,
    _reserved22: [u8; 0x14],
    #[doc = "0xc0 - SYNFP Transmission Interval Setting Register"]
    pub sytlir: SYTLIR,
    #[doc = "0xc4 - SYNFP Received logMessageInterval Value Indication Register"]
    pub syrlir: SYRLIR,
    #[doc = "0xc8 - offsetFromMaster Value Registers"]
    pub ofmru: OFMRU,
    #[doc = "0xcc - offsetFromMaster Value Registers"]
    pub ofmrl: OFMRL,
    #[doc = "0xd0 - meanPathDelay Value Registers"]
    pub mpdru: MPDRU,
    #[doc = "0xd4 - meanPathDelay Value Registers"]
    pub mpdrl: MPDRL,
    _reserved28: [u8; 0x08],
    #[doc = "0xe0 - grandmasterPriority Field Setting Register"]
    pub gmpr: GMPR,
    #[doc = "0xe4 - grandmasterClockQuality Field Setting Register"]
    pub gmcqr: GMCQR,
    #[doc = "0xe8 - grandmasterIdentity Field Setting Registers"]
    pub gmidru: GMIDRU,
    #[doc = "0xec - grandmasterIdentity Field Setting Registers"]
    pub gmidrl: GMIDRL,
    #[doc = "0xf0 - currentUtcOffset/timeSource Field Setting Register"]
    pub cuotsr: CUOTSR,
    #[doc = "0xf4 - stepsRemoved Field Setting Register"]
    pub srr: SRR,
    _reserved34: [u8; 0x08],
    #[doc = "0x100 - PTP-primary Message Destination MAC Address Setting Registers"]
    pub ppmacru: PPMACRU,
    #[doc = "0x104 - PTP-primary Message Destination MAC Address Setting Registers"]
    pub ppmacrl: PPMACRL,
    #[doc = "0x108 - PTP-pdelay Message MAC Address Setting Registers"]
    pub pdmacru: PDMACRU,
    #[doc = "0x10c - PTP-pdelay Message MAC Address Setting Registers"]
    pub pdmacrl: PDMACRL,
    #[doc = "0x110 - PTP Message EtherType Setting Register"]
    pub petyper: PETYPER,
    _reserved39: [u8; 0x0c],
    #[doc = "0x120 - PTP-primary Message Destination IP Address Setting Register"]
    pub ppipr: PPIPR,
    #[doc = "0x124 - PTP-pdelay Message Destination IP Address Setting Register"]
    pub pdipr: PDIPR,
    #[doc = "0x128 - PTP Event Message TOS Setting Register"]
    pub petosr: PETOSR,
    #[doc = "0x12c - PTP general Message TOS Setting Register"]
    pub pgtosr: PGTOSR,
    #[doc = "0x130 - PTP-primary Message TTL Setting Register"]
    pub ppttlr: PPTTLR,
    #[doc = "0x134 - PTP-pdelay Message TTL Setting Register"]
    pub pdttlr: PDTTLR,
    #[doc = "0x138 - PTP Event Message UDP Destination Port Number Setting Register"]
    pub peudpr: PEUDPR,
    #[doc = "0x13c - PTP general Message UDP Destination Port Number Setting Register"]
    pub pgudpr: PGUDPR,
    #[doc = "0x140 - Frame Reception Filter Setting Register"]
    pub ffltr: FFLTR,
    _reserved48: [u8; 0x1c],
    #[doc = "0x160 - Frame Reception Filter MAC Address %s Setting Registers"]
    pub fmac0ru: FMACRU,
    #[doc = "0x164 - Frame Reception Filter MAC Address %s Setting Registers"]
    pub fmac0rl: FMACRL,
    #[doc = "0x168 - Frame Reception Filter MAC Address %s Setting Registers"]
    pub fmac1ru: FMACRU,
    #[doc = "0x16c - Frame Reception Filter MAC Address %s Setting Registers"]
    pub fmac1rl: FMACRL,
    _reserved52: [u8; 0x50],
    #[doc = "0x1c0 - Asymmetric Delay Setting Registers"]
    pub dasymru: DASYMRU,
    #[doc = "0x1c4 - Asymmetric Delay Setting Registers"]
    pub dasymrl: DASYMRL,
    #[doc = "0x1c8 - Timestamp Latency Setting Register"]
    pub tslatr: TSLATR,
    #[doc = "0x1cc - SYNFP Operation Setting Register"]
    pub syconfr: SYCONFR,
    #[doc = "0x1d0 - SYNFP Frame Format Setting Register"]
    pub syformr: SYFORMR,
    #[doc = "0x1d4 - Response Message Reception Timeout Register"]
    pub rstoutr: RSTOUTR,
}
#[doc = "SYSR (rw) register accessor: an alias for `Reg<SYSR_SPEC>`"]
pub type SYSR = crate::Reg<sysr::SYSR_SPEC>;
#[doc = "SYNFP Status Register"]
pub mod sysr;
#[doc = "SYIPR (rw) register accessor: an alias for `Reg<SYIPR_SPEC>`"]
pub type SYIPR = crate::Reg<syipr::SYIPR_SPEC>;
#[doc = "SYNFP Status Notification Permission Register"]
pub mod syipr;
#[doc = "SYMACRU (rw) register accessor: an alias for `Reg<SYMACRU_SPEC>`"]
pub type SYMACRU = crate::Reg<symacru::SYMACRU_SPEC>;
#[doc = "SYNFP MAC Address Registers"]
pub mod symacru;
#[doc = "SYMACRL (rw) register accessor: an alias for `Reg<SYMACRL_SPEC>`"]
pub type SYMACRL = crate::Reg<symacrl::SYMACRL_SPEC>;
#[doc = "SYNFP MAC Address Registers"]
pub mod symacrl;
#[doc = "SYLLCCTLR (rw) register accessor: an alias for `Reg<SYLLCCTLR_SPEC>`"]
pub type SYLLCCTLR = crate::Reg<syllcctlr::SYLLCCTLR_SPEC>;
#[doc = "SYNFP LLC-CTL Value Register"]
pub mod syllcctlr;
#[doc = "SYIPADDRR (rw) register accessor: an alias for `Reg<SYIPADDRR_SPEC>`"]
pub type SYIPADDRR = crate::Reg<syipaddrr::SYIPADDRR_SPEC>;
#[doc = "SYNFP Local IP Address Register"]
pub mod syipaddrr;
#[doc = "SYSPVRR (rw) register accessor: an alias for `Reg<SYSPVRR_SPEC>`"]
pub type SYSPVRR = crate::Reg<syspvrr::SYSPVRR_SPEC>;
#[doc = "SYNFP Specification Version Setting Register"]
pub mod syspvrr;
#[doc = "SYDOMR (rw) register accessor: an alias for `Reg<SYDOMR_SPEC>`"]
pub type SYDOMR = crate::Reg<sydomr::SYDOMR_SPEC>;
#[doc = "SYNFP Domain Number Setting Register"]
pub mod sydomr;
#[doc = "ANFR (rw) register accessor: an alias for `Reg<ANFR_SPEC>`"]
pub type ANFR = crate::Reg<anfr::ANFR_SPEC>;
#[doc = "Announce Message Flag Field Setting Register"]
pub mod anfr;
#[doc = "SYNFR (rw) register accessor: an alias for `Reg<SYNFR_SPEC>`"]
pub type SYNFR = crate::Reg<synfr::SYNFR_SPEC>;
#[doc = "Sync Message Flag Field Setting Register"]
pub mod synfr;
#[doc = "DYRQFR (rw) register accessor: an alias for `Reg<DYRQFR_SPEC>`"]
pub type DYRQFR = crate::Reg<dyrqfr::DYRQFR_SPEC>;
#[doc = "Delay_Req Message Flag Field Setting Register"]
pub mod dyrqfr;
#[doc = "DYRPFR (rw) register accessor: an alias for `Reg<DYRPFR_SPEC>`"]
pub type DYRPFR = crate::Reg<dyrpfr::DYRPFR_SPEC>;
#[doc = "Delay_Resp Message Flag Field Setting Register"]
pub mod dyrpfr;
#[doc = "SYCIDRU (rw) register accessor: an alias for `Reg<SYCIDRU_SPEC>`"]
pub type SYCIDRU = crate::Reg<sycidru::SYCIDRU_SPEC>;
#[doc = "SYNFP Local Clock ID Registers"]
pub mod sycidru;
#[doc = "SYCIDRL (rw) register accessor: an alias for `Reg<SYCIDRL_SPEC>`"]
pub type SYCIDRL = crate::Reg<sycidrl::SYCIDRL_SPEC>;
#[doc = "SYNFP Local Clock ID Registers"]
pub mod sycidrl;
#[doc = "SYPNUMR (rw) register accessor: an alias for `Reg<SYPNUMR_SPEC>`"]
pub type SYPNUMR = crate::Reg<sypnumr::SYPNUMR_SPEC>;
#[doc = "SYNFP Local Port Number Register"]
pub mod sypnumr;
#[doc = "SYRVLDR (w) register accessor: an alias for `Reg<SYRVLDR_SPEC>`"]
pub type SYRVLDR = crate::Reg<syrvldr::SYRVLDR_SPEC>;
#[doc = "SYNFP Register Value Load Directive Register"]
pub mod syrvldr;
#[doc = "SYRFL1R (rw) register accessor: an alias for `Reg<SYRFL1R_SPEC>`"]
pub type SYRFL1R = crate::Reg<syrfl1r::SYRFL1R_SPEC>;
#[doc = "SYNFP Reception Filter Register 1"]
pub mod syrfl1r;
#[doc = "SYRFL2R (rw) register accessor: an alias for `Reg<SYRFL2R_SPEC>`"]
pub type SYRFL2R = crate::Reg<syrfl2r::SYRFL2R_SPEC>;
#[doc = "SYNFP Reception Filter Register 2"]
pub mod syrfl2r;
#[doc = "SYTRENR (rw) register accessor: an alias for `Reg<SYTRENR_SPEC>`"]
pub type SYTRENR = crate::Reg<sytrenr::SYTRENR_SPEC>;
#[doc = "SYNFP Transmission Enable Register"]
pub mod sytrenr;
#[doc = "MTCIDU (rw) register accessor: an alias for `Reg<MTCIDU_SPEC>`"]
pub type MTCIDU = crate::Reg<mtcidu::MTCIDU_SPEC>;
#[doc = "Master Clock ID Registers"]
pub mod mtcidu;
#[doc = "MTCIDL (rw) register accessor: an alias for `Reg<MTCIDL_SPEC>`"]
pub type MTCIDL = crate::Reg<mtcidl::MTCIDL_SPEC>;
#[doc = "Master Clock ID Registers"]
pub mod mtcidl;
#[doc = "MTPID (rw) register accessor: an alias for `Reg<MTPID_SPEC>`"]
pub type MTPID = crate::Reg<mtpid::MTPID_SPEC>;
#[doc = "Master clock port number register"]
pub mod mtpid;
#[doc = "SYTLIR (rw) register accessor: an alias for `Reg<SYTLIR_SPEC>`"]
pub type SYTLIR = crate::Reg<sytlir::SYTLIR_SPEC>;
#[doc = "SYNFP Transmission Interval Setting Register"]
pub mod sytlir;
#[doc = "SYRLIR (r) register accessor: an alias for `Reg<SYRLIR_SPEC>`"]
pub type SYRLIR = crate::Reg<syrlir::SYRLIR_SPEC>;
#[doc = "SYNFP Received logMessageInterval Value Indication Register"]
pub mod syrlir;
#[doc = "OFMRU (r) register accessor: an alias for `Reg<OFMRU_SPEC>`"]
pub type OFMRU = crate::Reg<ofmru::OFMRU_SPEC>;
#[doc = "offsetFromMaster Value Registers"]
pub mod ofmru;
#[doc = "OFMRL (r) register accessor: an alias for `Reg<OFMRL_SPEC>`"]
pub type OFMRL = crate::Reg<ofmrl::OFMRL_SPEC>;
#[doc = "offsetFromMaster Value Registers"]
pub mod ofmrl;
#[doc = "MPDRU (r) register accessor: an alias for `Reg<MPDRU_SPEC>`"]
pub type MPDRU = crate::Reg<mpdru::MPDRU_SPEC>;
#[doc = "meanPathDelay Value Registers"]
pub mod mpdru;
#[doc = "MPDRL (r) register accessor: an alias for `Reg<MPDRL_SPEC>`"]
pub type MPDRL = crate::Reg<mpdrl::MPDRL_SPEC>;
#[doc = "meanPathDelay Value Registers"]
pub mod mpdrl;
#[doc = "GMPR (rw) register accessor: an alias for `Reg<GMPR_SPEC>`"]
pub type GMPR = crate::Reg<gmpr::GMPR_SPEC>;
#[doc = "grandmasterPriority Field Setting Register"]
pub mod gmpr;
#[doc = "GMCQR (rw) register accessor: an alias for `Reg<GMCQR_SPEC>`"]
pub type GMCQR = crate::Reg<gmcqr::GMCQR_SPEC>;
#[doc = "grandmasterClockQuality Field Setting Register"]
pub mod gmcqr;
#[doc = "GMIDRU (rw) register accessor: an alias for `Reg<GMIDRU_SPEC>`"]
pub type GMIDRU = crate::Reg<gmidru::GMIDRU_SPEC>;
#[doc = "grandmasterIdentity Field Setting Registers"]
pub mod gmidru;
#[doc = "GMIDRL (rw) register accessor: an alias for `Reg<GMIDRL_SPEC>`"]
pub type GMIDRL = crate::Reg<gmidrl::GMIDRL_SPEC>;
#[doc = "grandmasterIdentity Field Setting Registers"]
pub mod gmidrl;
#[doc = "CUOTSR (rw) register accessor: an alias for `Reg<CUOTSR_SPEC>`"]
pub type CUOTSR = crate::Reg<cuotsr::CUOTSR_SPEC>;
#[doc = "currentUtcOffset/timeSource Field Setting Register"]
pub mod cuotsr;
#[doc = "SRR (rw) register accessor: an alias for `Reg<SRR_SPEC>`"]
pub type SRR = crate::Reg<srr::SRR_SPEC>;
#[doc = "stepsRemoved Field Setting Register"]
pub mod srr;
#[doc = "PPMACRU (rw) register accessor: an alias for `Reg<PPMACRU_SPEC>`"]
pub type PPMACRU = crate::Reg<ppmacru::PPMACRU_SPEC>;
#[doc = "PTP-primary Message Destination MAC Address Setting Registers"]
pub mod ppmacru;
#[doc = "PPMACRL (rw) register accessor: an alias for `Reg<PPMACRL_SPEC>`"]
pub type PPMACRL = crate::Reg<ppmacrl::PPMACRL_SPEC>;
#[doc = "PTP-primary Message Destination MAC Address Setting Registers"]
pub mod ppmacrl;
#[doc = "PDMACRU (rw) register accessor: an alias for `Reg<PDMACRU_SPEC>`"]
pub type PDMACRU = crate::Reg<pdmacru::PDMACRU_SPEC>;
#[doc = "PTP-pdelay Message MAC Address Setting Registers"]
pub mod pdmacru;
#[doc = "PDMACRL (rw) register accessor: an alias for `Reg<PDMACRL_SPEC>`"]
pub type PDMACRL = crate::Reg<pdmacrl::PDMACRL_SPEC>;
#[doc = "PTP-pdelay Message MAC Address Setting Registers"]
pub mod pdmacrl;
#[doc = "PETYPER (rw) register accessor: an alias for `Reg<PETYPER_SPEC>`"]
pub type PETYPER = crate::Reg<petyper::PETYPER_SPEC>;
#[doc = "PTP Message EtherType Setting Register"]
pub mod petyper;
#[doc = "PPIPR (rw) register accessor: an alias for `Reg<PPIPR_SPEC>`"]
pub type PPIPR = crate::Reg<ppipr::PPIPR_SPEC>;
#[doc = "PTP-primary Message Destination IP Address Setting Register"]
pub mod ppipr;
#[doc = "PDIPR (rw) register accessor: an alias for `Reg<PDIPR_SPEC>`"]
pub type PDIPR = crate::Reg<pdipr::PDIPR_SPEC>;
#[doc = "PTP-pdelay Message Destination IP Address Setting Register"]
pub mod pdipr;
#[doc = "PETOSR (rw) register accessor: an alias for `Reg<PETOSR_SPEC>`"]
pub type PETOSR = crate::Reg<petosr::PETOSR_SPEC>;
#[doc = "PTP Event Message TOS Setting Register"]
pub mod petosr;
#[doc = "PGTOSR (rw) register accessor: an alias for `Reg<PGTOSR_SPEC>`"]
pub type PGTOSR = crate::Reg<pgtosr::PGTOSR_SPEC>;
#[doc = "PTP general Message TOS Setting Register"]
pub mod pgtosr;
#[doc = "PPTTLR (rw) register accessor: an alias for `Reg<PPTTLR_SPEC>`"]
pub type PPTTLR = crate::Reg<ppttlr::PPTTLR_SPEC>;
#[doc = "PTP-primary Message TTL Setting Register"]
pub mod ppttlr;
#[doc = "PDTTLR (rw) register accessor: an alias for `Reg<PDTTLR_SPEC>`"]
pub type PDTTLR = crate::Reg<pdttlr::PDTTLR_SPEC>;
#[doc = "PTP-pdelay Message TTL Setting Register"]
pub mod pdttlr;
#[doc = "PEUDPR (rw) register accessor: an alias for `Reg<PEUDPR_SPEC>`"]
pub type PEUDPR = crate::Reg<peudpr::PEUDPR_SPEC>;
#[doc = "PTP Event Message UDP Destination Port Number Setting Register"]
pub mod peudpr;
#[doc = "PGUDPR (rw) register accessor: an alias for `Reg<PGUDPR_SPEC>`"]
pub type PGUDPR = crate::Reg<pgudpr::PGUDPR_SPEC>;
#[doc = "PTP general Message UDP Destination Port Number Setting Register"]
pub mod pgudpr;
#[doc = "FFLTR (rw) register accessor: an alias for `Reg<FFLTR_SPEC>`"]
pub type FFLTR = crate::Reg<ffltr::FFLTR_SPEC>;
#[doc = "Frame Reception Filter Setting Register"]
pub mod ffltr;
#[doc = "FMACRU (rw) register accessor: an alias for `Reg<FMACRU_SPEC>`"]
pub type FMACRU = crate::Reg<fmacru::FMACRU_SPEC>;
#[doc = "Frame Reception Filter MAC Address %s Setting Registers"]
pub mod fmacru;
#[doc = "FMACRL (rw) register accessor: an alias for `Reg<FMACRL_SPEC>`"]
pub type FMACRL = crate::Reg<fmacrl::FMACRL_SPEC>;
#[doc = "Frame Reception Filter MAC Address %s Setting Registers"]
pub mod fmacrl;
#[doc = "DASYMRU (rw) register accessor: an alias for `Reg<DASYMRU_SPEC>`"]
pub type DASYMRU = crate::Reg<dasymru::DASYMRU_SPEC>;
#[doc = "Asymmetric Delay Setting Registers"]
pub mod dasymru;
#[doc = "DASYMRL (rw) register accessor: an alias for `Reg<DASYMRL_SPEC>`"]
pub type DASYMRL = crate::Reg<dasymrl::DASYMRL_SPEC>;
#[doc = "Asymmetric Delay Setting Registers"]
pub mod dasymrl;
#[doc = "TSLATR (rw) register accessor: an alias for `Reg<TSLATR_SPEC>`"]
pub type TSLATR = crate::Reg<tslatr::TSLATR_SPEC>;
#[doc = "Timestamp Latency Setting Register"]
pub mod tslatr;
#[doc = "SYCONFR (rw) register accessor: an alias for `Reg<SYCONFR_SPEC>`"]
pub type SYCONFR = crate::Reg<syconfr::SYCONFR_SPEC>;
#[doc = "SYNFP Operation Setting Register"]
pub mod syconfr;
#[doc = "SYFORMR (rw) register accessor: an alias for `Reg<SYFORMR_SPEC>`"]
pub type SYFORMR = crate::Reg<syformr::SYFORMR_SPEC>;
#[doc = "SYNFP Frame Format Setting Register"]
pub mod syformr;
#[doc = "RSTOUTR (rw) register accessor: an alias for `Reg<RSTOUTR_SPEC>`"]
pub type RSTOUTR = crate::Reg<rstoutr::RSTOUTR_SPEC>;
#[doc = "Response Message Reception Timeout Register"]
pub mod rstoutr;
