#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - C-Cache Control Register"]
    pub ccactl: CCACTL,
    #[doc = "0x04 - C-Cache Flush Control Register"]
    pub ccafct: CCAFCT,
    #[doc = "0x08 - C-Cache Line Configuration Register"]
    pub ccalcf: CCALCF,
    _reserved3: [u8; 0x34],
    #[doc = "0x40 - S-Cache Control Register"]
    pub scactl: SCACTL,
    #[doc = "0x44 - S-Cache Flush Control Register"]
    pub scafct: SCAFCT,
    #[doc = "0x48 - S-Cache Line Configuration Register"]
    pub scalcf: SCALCF,
    _reserved6: [u8; 0x01b4],
    #[doc = "0x200 - Cache Parity Error Operation After Detection Register"]
    pub capoad: CAPOAD,
    #[doc = "0x204 - Cache Protection Register"]
    pub caprcr: CAPRCR,
}
#[doc = "CCACTL (rw) register accessor: an alias for `Reg<CCACTL_SPEC>`"]
pub type CCACTL = crate::Reg<ccactl::CCACTL_SPEC>;
#[doc = "C-Cache Control Register"]
pub mod ccactl;
#[doc = "CCAFCT (rw) register accessor: an alias for `Reg<CCAFCT_SPEC>`"]
pub type CCAFCT = crate::Reg<ccafct::CCAFCT_SPEC>;
#[doc = "C-Cache Flush Control Register"]
pub mod ccafct;
#[doc = "CCALCF (rw) register accessor: an alias for `Reg<CCALCF_SPEC>`"]
pub type CCALCF = crate::Reg<ccalcf::CCALCF_SPEC>;
#[doc = "C-Cache Line Configuration Register"]
pub mod ccalcf;
#[doc = "SCACTL (rw) register accessor: an alias for `Reg<SCACTL_SPEC>`"]
pub type SCACTL = crate::Reg<scactl::SCACTL_SPEC>;
#[doc = "S-Cache Control Register"]
pub mod scactl;
#[doc = "SCAFCT (rw) register accessor: an alias for `Reg<SCAFCT_SPEC>`"]
pub type SCAFCT = crate::Reg<scafct::SCAFCT_SPEC>;
#[doc = "S-Cache Flush Control Register"]
pub mod scafct;
#[doc = "SCALCF (rw) register accessor: an alias for `Reg<SCALCF_SPEC>`"]
pub type SCALCF = crate::Reg<scalcf::SCALCF_SPEC>;
#[doc = "S-Cache Line Configuration Register"]
pub mod scalcf;
#[doc = "CAPOAD (rw) register accessor: an alias for `Reg<CAPOAD_SPEC>`"]
pub type CAPOAD = crate::Reg<capoad::CAPOAD_SPEC>;
#[doc = "Cache Parity Error Operation After Detection Register"]
pub mod capoad;
#[doc = "CAPRCR (rw) register accessor: an alias for `Reg<CAPRCR_SPEC>`"]
pub type CAPRCR = crate::Reg<caprcr::CAPRCR_SPEC>;
#[doc = "Cache Protection Register"]
pub mod caprcr;
