#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x90],
    #[doc = "0x90 - Data Flash Control Register"]
    pub dflctl: DFLCTL,
    _reserved1: [u8; 0x6f],
    #[doc = "0x100 - Flash P/E Mode Control Register"]
    pub fpmcr: FPMCR,
    _reserved2: [u8; 0x03],
    #[doc = "0x104 - Flash Area Select Register"]
    pub fasr: FASR,
    _reserved3: [u8; 0x03],
    #[doc = "0x108 - Flash Processing Start Address Register L"]
    pub fsarl: FSARL,
    _reserved4: [u8; 0x06],
    #[doc = "0x110 - Flash Processing Start Address Register H"]
    pub fsarh: FSARH,
    _reserved5: [u8; 0x02],
    #[doc = "0x114 - Flash Control Register"]
    pub fcr: FCR,
    _reserved6: [u8; 0x03],
    #[doc = "0x118 - Flash Processing End Address Register L"]
    pub fearl: FEARL,
    _reserved7: [u8; 0x06],
    #[doc = "0x120 - Flash Processing End Address Register H"]
    pub fearh: FEARH,
    _reserved8: [u8; 0x02],
    #[doc = "0x124 - Flash Reset Register"]
    pub fresetr: FRESETR,
    _reserved9: [u8; 0x07],
    #[doc = "0x12c - Flash Status Register 1"]
    pub fstatr1: FSTATR1,
    _reserved10: [u8; 0x03],
    #[doc = "0x130 - Flash Write Buffer Register L0"]
    pub fwbl0: FWBL0,
    _reserved11: [u8; 0x06],
    #[doc = "0x138 - Flash Write Buffer Register H0"]
    pub fwbh0: FWBH0,
    _reserved12: [u8; 0x46],
    #[doc = "0x180 - Protection Unlock Register"]
    pub fpr: FPR,
    _reserved13: [u8; 0x03],
    #[doc = "0x184 - Protection Unlock Status Register"]
    pub fpsr: FPSR,
    _reserved14: [u8; 0x03],
    #[doc = "0x188 - Flash Read Buffer Register L0"]
    pub frbl0: FRBL0,
    _reserved15: [u8; 0x06],
    #[doc = "0x190 - Flash Read Buffer Register H0"]
    pub frbh0: FRBH0,
    _reserved16: [u8; 0x2e],
    #[doc = "0x1c0 - Flash Start-Up Setting Monitor Register"]
    pub fscmr: FSCMR,
    _reserved17: [u8; 0x06],
    #[doc = "0x1c8 - Flash Access Window Start Address Monitor Register"]
    pub fawsmr: FAWSMR,
    _reserved18: [u8; 0x06],
    #[doc = "0x1d0 - Flash Access Window End Address Monitor Register"]
    pub fawemr: FAWEMR,
    _reserved19: [u8; 0x06],
    #[doc = "0x1d8 - Flash Initial Setting Register"]
    pub fisr: FISR,
    _reserved20: [u8; 0x03],
    #[doc = "0x1dc - Flash Extra Area Control Register"]
    pub fexcr: FEXCR,
    _reserved21: [u8; 0x03],
    #[doc = "0x1e0 - Flash Error Address Monitor Register L"]
    pub feaml: FEAML,
    _reserved22: [u8; 0x06],
    #[doc = "0x1e8 - Flash Error Address Monitor Register H"]
    pub feamh: FEAMH,
    _reserved23: [u8; 0x06],
    #[doc = "0x1f0 - Flash Status Register 2"]
    pub fstatr2: FSTATR2,
    _reserved24: [u8; 0x36],
    #[doc = "0x228 - Temperature Sensor Calibration Data Register"]
    pub tscdr: TSCDR,
    _reserved25: [u8; 0x3d84],
    #[doc = "0x3fb0 - Flash P/E Mode Entry Register"]
    pub fentryr: FENTRYR,
    _reserved26: [u8; 0x12],
    #[doc = "0x3fc4 - Memory Wait Cycle Control Register for Data Flash"]
    pub fldwaitr: FLDWAITR,
    _reserved27: [u8; 0x03],
    #[doc = "0x3fc8 - Prefetch Buffer Enable Register"]
    pub pfber: PFBER,
}
#[doc = "DFLCTL (rw) register accessor: an alias for `Reg<DFLCTL_SPEC>`"]
pub type DFLCTL = crate::Reg<dflctl::DFLCTL_SPEC>;
#[doc = "Data Flash Control Register"]
pub mod dflctl;
#[doc = "FPMCR (rw) register accessor: an alias for `Reg<FPMCR_SPEC>`"]
pub type FPMCR = crate::Reg<fpmcr::FPMCR_SPEC>;
#[doc = "Flash P/E Mode Control Register"]
pub mod fpmcr;
#[doc = "FASR (rw) register accessor: an alias for `Reg<FASR_SPEC>`"]
pub type FASR = crate::Reg<fasr::FASR_SPEC>;
#[doc = "Flash Area Select Register"]
pub mod fasr;
#[doc = "FSARL (rw) register accessor: an alias for `Reg<FSARL_SPEC>`"]
pub type FSARL = crate::Reg<fsarl::FSARL_SPEC>;
#[doc = "Flash Processing Start Address Register L"]
pub mod fsarl;
#[doc = "FSARH (rw) register accessor: an alias for `Reg<FSARH_SPEC>`"]
pub type FSARH = crate::Reg<fsarh::FSARH_SPEC>;
#[doc = "Flash Processing Start Address Register H"]
pub mod fsarh;
#[doc = "FCR (rw) register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "Flash Control Register"]
pub mod fcr;
#[doc = "FEARL (rw) register accessor: an alias for `Reg<FEARL_SPEC>`"]
pub type FEARL = crate::Reg<fearl::FEARL_SPEC>;
#[doc = "Flash Processing End Address Register L"]
pub mod fearl;
#[doc = "FEARH (rw) register accessor: an alias for `Reg<FEARH_SPEC>`"]
pub type FEARH = crate::Reg<fearh::FEARH_SPEC>;
#[doc = "Flash Processing End Address Register H"]
pub mod fearh;
#[doc = "FRESETR (rw) register accessor: an alias for `Reg<FRESETR_SPEC>`"]
pub type FRESETR = crate::Reg<fresetr::FRESETR_SPEC>;
#[doc = "Flash Reset Register"]
pub mod fresetr;
#[doc = "FSTATR1 (r) register accessor: an alias for `Reg<FSTATR1_SPEC>`"]
pub type FSTATR1 = crate::Reg<fstatr1::FSTATR1_SPEC>;
#[doc = "Flash Status Register 1"]
pub mod fstatr1;
#[doc = "FWBL0 (rw) register accessor: an alias for `Reg<FWBL0_SPEC>`"]
pub type FWBL0 = crate::Reg<fwbl0::FWBL0_SPEC>;
#[doc = "Flash Write Buffer Register L0"]
pub mod fwbl0;
#[doc = "FWBH0 (rw) register accessor: an alias for `Reg<FWBH0_SPEC>`"]
pub type FWBH0 = crate::Reg<fwbh0::FWBH0_SPEC>;
#[doc = "Flash Write Buffer Register H0"]
pub mod fwbh0;
#[doc = "FPR (rw) register accessor: an alias for `Reg<FPR_SPEC>`"]
pub type FPR = crate::Reg<fpr::FPR_SPEC>;
#[doc = "Protection Unlock Register"]
pub mod fpr;
#[doc = "FPSR (r) register accessor: an alias for `Reg<FPSR_SPEC>`"]
pub type FPSR = crate::Reg<fpsr::FPSR_SPEC>;
#[doc = "Protection Unlock Status Register"]
pub mod fpsr;
#[doc = "FRBL0 (r) register accessor: an alias for `Reg<FRBL0_SPEC>`"]
pub type FRBL0 = crate::Reg<frbl0::FRBL0_SPEC>;
#[doc = "Flash Read Buffer Register L0"]
pub mod frbl0;
#[doc = "FRBH0 (r) register accessor: an alias for `Reg<FRBH0_SPEC>`"]
pub type FRBH0 = crate::Reg<frbh0::FRBH0_SPEC>;
#[doc = "Flash Read Buffer Register H0"]
pub mod frbh0;
#[doc = "FSCMR (r) register accessor: an alias for `Reg<FSCMR_SPEC>`"]
pub type FSCMR = crate::Reg<fscmr::FSCMR_SPEC>;
#[doc = "Flash Start-Up Setting Monitor Register"]
pub mod fscmr;
#[doc = "FAWSMR (r) register accessor: an alias for `Reg<FAWSMR_SPEC>`"]
pub type FAWSMR = crate::Reg<fawsmr::FAWSMR_SPEC>;
#[doc = "Flash Access Window Start Address Monitor Register"]
pub mod fawsmr;
#[doc = "FAWEMR (r) register accessor: an alias for `Reg<FAWEMR_SPEC>`"]
pub type FAWEMR = crate::Reg<fawemr::FAWEMR_SPEC>;
#[doc = "Flash Access Window End Address Monitor Register"]
pub mod fawemr;
#[doc = "FISR (rw) register accessor: an alias for `Reg<FISR_SPEC>`"]
pub type FISR = crate::Reg<fisr::FISR_SPEC>;
#[doc = "Flash Initial Setting Register"]
pub mod fisr;
#[doc = "FEXCR (rw) register accessor: an alias for `Reg<FEXCR_SPEC>`"]
pub type FEXCR = crate::Reg<fexcr::FEXCR_SPEC>;
#[doc = "Flash Extra Area Control Register"]
pub mod fexcr;
#[doc = "FEAML (rw) register accessor: an alias for `Reg<FEAML_SPEC>`"]
pub type FEAML = crate::Reg<feaml::FEAML_SPEC>;
#[doc = "Flash Error Address Monitor Register L"]
pub mod feaml;
#[doc = "FEAMH (rw) register accessor: an alias for `Reg<FEAMH_SPEC>`"]
pub type FEAMH = crate::Reg<feamh::FEAMH_SPEC>;
#[doc = "Flash Error Address Monitor Register H"]
pub mod feamh;
#[doc = "FSTATR2 (r) register accessor: an alias for `Reg<FSTATR2_SPEC>`"]
pub type FSTATR2 = crate::Reg<fstatr2::FSTATR2_SPEC>;
#[doc = "Flash Status Register 2"]
pub mod fstatr2;
#[doc = "TSCDR (r) register accessor: an alias for `Reg<TSCDR_SPEC>`"]
pub type TSCDR = crate::Reg<tscdr::TSCDR_SPEC>;
#[doc = "Temperature Sensor Calibration Data Register"]
pub mod tscdr;
#[doc = "FENTRYR (rw) register accessor: an alias for `Reg<FENTRYR_SPEC>`"]
pub type FENTRYR = crate::Reg<fentryr::FENTRYR_SPEC>;
#[doc = "Flash P/E Mode Entry Register"]
pub mod fentryr;
#[doc = "FLDWAITR (rw) register accessor: an alias for `Reg<FLDWAITR_SPEC>`"]
pub type FLDWAITR = crate::Reg<fldwaitr::FLDWAITR_SPEC>;
#[doc = "Memory Wait Cycle Control Register for Data Flash"]
pub mod fldwaitr;
#[doc = "PFBER (rw) register accessor: an alias for `Reg<PFBER_SPEC>`"]
pub type PFBER = crate::Reg<pfber::PFBER_SPEC>;
#[doc = "Prefetch Buffer Enable Register"]
pub mod pfber;
