#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x14],
    #[doc = "0x14 - Bus Control Register"]
    pub bctl: BCTL,
    _reserved1: [u8; 0x08],
    #[doc = "0x20 - Reset Control Register"]
    pub rstctl: RSTCTL,
    #[doc = "0x24 - Present State Register"]
    pub prsst: PRSST,
    _reserved3: [u8; 0x38],
    #[doc = "0x60 - Bus Function Control Register"]
    pub bfctl: BFCTL,
    #[doc = "0x64 - Slave Control Register"]
    pub svctl: SVCTL,
    _reserved5: [u8; 0x08],
    #[doc = "0x70 - Reference Clock Control Register"]
    pub refckctl: REFCKCTL,
    #[doc = "0x74 - Standard Bit Rate Register"]
    pub stdbr: STDBR,
    #[doc = "0x78 - Extended Bit Rate Register"]
    pub extbr: EXTBR,
    #[doc = "0x7c - Bus Free Condition Detection Time Register"]
    pub bfrecdt: BFRECDT,
    _reserved9: [u8; 0x08],
    #[doc = "0x88 - Output Control Register"]
    pub outctl: OUTCTL,
    #[doc = "0x8c - Input Control Register"]
    pub inctl: INCTL,
    #[doc = "0x90 - Timeout Control Register"]
    pub tmoctl: TMOCTL,
    _reserved12: [u8; 0x0c],
    #[doc = "0xa0 - Acknowledge Control Register"]
    pub ackctl: ACKCTL,
    #[doc = "0xa4 - SCL Stretch Control Register"]
    pub scstrctl: SCSTRCTL,
    _reserved14: [u8; 0x98],
    #[doc = "0x140 - Condition Control Register"]
    pub cndctl: CNDCTL,
    _reserved15: [u8; 0x14],
    _reserved_15_ntdtbp: [u8; 0x04],
    _reserved16: [u8; 0x74],
    #[doc = "0x1d0 - Bus Status Register"]
    pub bst: BST,
    #[doc = "0x1d4 - Bus Status Enable Register"]
    pub bste: BSTE,
    #[doc = "0x1d8 - Bus Interrupt Enable Register"]
    pub bie: BIE,
    #[doc = "0x1dc - Bus Status Force Register"]
    pub bstfc: BSTFC,
    #[doc = "0x1e0 - Normal Transfer Status Register"]
    pub ntst: NTST,
    #[doc = "0x1e4 - Normal Transfer Status Enable Register"]
    pub ntste: NTSTE,
    #[doc = "0x1e8 - Normal Transfer Interrupt Enable Register"]
    pub ntie: NTIE,
    #[doc = "0x1ec - Normal Transfer Status Force Register"]
    pub ntstfc: NTSTFC,
    _reserved24: [u8; 0x20],
    #[doc = "0x210 - Bus Condition Status Register"]
    pub bcst: BCST,
    #[doc = "0x214 - Slave Status Register"]
    pub svst: SVST,
    _reserved26: [u8; 0x98],
    #[doc = "0x2b0..0x2bc - Slave Device Address Table Basic Register %s"]
    pub sdatbas: [SDATBAS; 3],
    _reserved27: [u8; 0x74],
    #[doc = "0x330..0x33c - Slave Device Address Register %s"]
    pub svdvad: [SVDVAD; 3],
    _reserved28: [u8; 0x44],
    #[doc = "0x380 - Bit Count Register"]
    pub bitcnt: BITCNT,
    _reserved29: [u8; 0x48],
    #[doc = "0x3cc - Present State Debug Register"]
    pub prstdbg: PRSTDBG,
}
impl RegisterBlock {
    #[doc = "0x158 - Normal Transfer Data Buffer Port Register 0"]
    #[inline(always)]
    pub const fn ntdtbp0_by(&self) -> &NTDTBP0_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(344usize).cast() }
    }
    #[doc = "0x158 - Normal Transfer Data Buffer Port Register 0"]
    #[inline(always)]
    pub const fn ntdtbp0(&self) -> &NTDTBP0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(344usize).cast() }
    }
}
#[doc = "BCTL (rw) register accessor: an alias for `Reg<BCTL_SPEC>`"]
pub type BCTL = crate::Reg<bctl::BCTL_SPEC>;
#[doc = "Bus Control Register"]
pub mod bctl;
#[doc = "RSTCTL (rw) register accessor: an alias for `Reg<RSTCTL_SPEC>`"]
pub type RSTCTL = crate::Reg<rstctl::RSTCTL_SPEC>;
#[doc = "Reset Control Register"]
pub mod rstctl;
#[doc = "PRSST (rw) register accessor: an alias for `Reg<PRSST_SPEC>`"]
pub type PRSST = crate::Reg<prsst::PRSST_SPEC>;
#[doc = "Present State Register"]
pub mod prsst;
#[doc = "BFCTL (rw) register accessor: an alias for `Reg<BFCTL_SPEC>`"]
pub type BFCTL = crate::Reg<bfctl::BFCTL_SPEC>;
#[doc = "Bus Function Control Register"]
pub mod bfctl;
#[doc = "SVCTL (rw) register accessor: an alias for `Reg<SVCTL_SPEC>`"]
pub type SVCTL = crate::Reg<svctl::SVCTL_SPEC>;
#[doc = "Slave Control Register"]
pub mod svctl;
#[doc = "REFCKCTL (rw) register accessor: an alias for `Reg<REFCKCTL_SPEC>`"]
pub type REFCKCTL = crate::Reg<refckctl::REFCKCTL_SPEC>;
#[doc = "Reference Clock Control Register"]
pub mod refckctl;
#[doc = "STDBR (rw) register accessor: an alias for `Reg<STDBR_SPEC>`"]
pub type STDBR = crate::Reg<stdbr::STDBR_SPEC>;
#[doc = "Standard Bit Rate Register"]
pub mod stdbr;
#[doc = "EXTBR (rw) register accessor: an alias for `Reg<EXTBR_SPEC>`"]
pub type EXTBR = crate::Reg<extbr::EXTBR_SPEC>;
#[doc = "Extended Bit Rate Register"]
pub mod extbr;
#[doc = "BFRECDT (rw) register accessor: an alias for `Reg<BFRECDT_SPEC>`"]
pub type BFRECDT = crate::Reg<bfrecdt::BFRECDT_SPEC>;
#[doc = "Bus Free Condition Detection Time Register"]
pub mod bfrecdt;
#[doc = "OUTCTL (rw) register accessor: an alias for `Reg<OUTCTL_SPEC>`"]
pub type OUTCTL = crate::Reg<outctl::OUTCTL_SPEC>;
#[doc = "Output Control Register"]
pub mod outctl;
#[doc = "INCTL (rw) register accessor: an alias for `Reg<INCTL_SPEC>`"]
pub type INCTL = crate::Reg<inctl::INCTL_SPEC>;
#[doc = "Input Control Register"]
pub mod inctl;
#[doc = "TMOCTL (rw) register accessor: an alias for `Reg<TMOCTL_SPEC>`"]
pub type TMOCTL = crate::Reg<tmoctl::TMOCTL_SPEC>;
#[doc = "Timeout Control Register"]
pub mod tmoctl;
#[doc = "ACKCTL (rw) register accessor: an alias for `Reg<ACKCTL_SPEC>`"]
pub type ACKCTL = crate::Reg<ackctl::ACKCTL_SPEC>;
#[doc = "Acknowledge Control Register"]
pub mod ackctl;
#[doc = "SCSTRCTL (rw) register accessor: an alias for `Reg<SCSTRCTL_SPEC>`"]
pub type SCSTRCTL = crate::Reg<scstrctl::SCSTRCTL_SPEC>;
#[doc = "SCL Stretch Control Register"]
pub mod scstrctl;
#[doc = "CNDCTL (rw) register accessor: an alias for `Reg<CNDCTL_SPEC>`"]
pub type CNDCTL = crate::Reg<cndctl::CNDCTL_SPEC>;
#[doc = "Condition Control Register"]
pub mod cndctl;
#[doc = "NTDTBP0 (rw) register accessor: an alias for `Reg<NTDTBP0_SPEC>`"]
pub type NTDTBP0 = crate::Reg<ntdtbp0::NTDTBP0_SPEC>;
#[doc = "Normal Transfer Data Buffer Port Register 0"]
pub mod ntdtbp0;
#[doc = "NTDTBP0_BY (rw) register accessor: an alias for `Reg<NTDTBP0_BY_SPEC>`"]
pub type NTDTBP0_BY = crate::Reg<ntdtbp0_by::NTDTBP0_BY_SPEC>;
#[doc = "Normal Transfer Data Buffer Port Register 0"]
pub mod ntdtbp0_by;
#[doc = "BST (rw) register accessor: an alias for `Reg<BST_SPEC>`"]
pub type BST = crate::Reg<bst::BST_SPEC>;
#[doc = "Bus Status Register"]
pub mod bst;
#[doc = "BSTE (rw) register accessor: an alias for `Reg<BSTE_SPEC>`"]
pub type BSTE = crate::Reg<bste::BSTE_SPEC>;
#[doc = "Bus Status Enable Register"]
pub mod bste;
#[doc = "BIE (rw) register accessor: an alias for `Reg<BIE_SPEC>`"]
pub type BIE = crate::Reg<bie::BIE_SPEC>;
#[doc = "Bus Interrupt Enable Register"]
pub mod bie;
#[doc = "BSTFC (rw) register accessor: an alias for `Reg<BSTFC_SPEC>`"]
pub type BSTFC = crate::Reg<bstfc::BSTFC_SPEC>;
#[doc = "Bus Status Force Register"]
pub mod bstfc;
#[doc = "NTST (rw) register accessor: an alias for `Reg<NTST_SPEC>`"]
pub type NTST = crate::Reg<ntst::NTST_SPEC>;
#[doc = "Normal Transfer Status Register"]
pub mod ntst;
#[doc = "NTSTE (rw) register accessor: an alias for `Reg<NTSTE_SPEC>`"]
pub type NTSTE = crate::Reg<ntste::NTSTE_SPEC>;
#[doc = "Normal Transfer Status Enable Register"]
pub mod ntste;
#[doc = "NTIE (rw) register accessor: an alias for `Reg<NTIE_SPEC>`"]
pub type NTIE = crate::Reg<ntie::NTIE_SPEC>;
#[doc = "Normal Transfer Interrupt Enable Register"]
pub mod ntie;
#[doc = "NTSTFC (w) register accessor: an alias for `Reg<NTSTFC_SPEC>`"]
pub type NTSTFC = crate::Reg<ntstfc::NTSTFC_SPEC>;
#[doc = "Normal Transfer Status Force Register"]
pub mod ntstfc;
#[doc = "BCST (r) register accessor: an alias for `Reg<BCST_SPEC>`"]
pub type BCST = crate::Reg<bcst::BCST_SPEC>;
#[doc = "Bus Condition Status Register"]
pub mod bcst;
#[doc = "SVST (rw) register accessor: an alias for `Reg<SVST_SPEC>`"]
pub type SVST = crate::Reg<svst::SVST_SPEC>;
#[doc = "Slave Status Register"]
pub mod svst;
#[doc = "SDATBAS (rw) register accessor: an alias for `Reg<SDATBAS_SPEC>`"]
pub type SDATBAS = crate::Reg<sdatbas::SDATBAS_SPEC>;
#[doc = "Slave Device Address Table Basic Register %s"]
pub mod sdatbas;
#[doc = "SVDVAD (r) register accessor: an alias for `Reg<SVDVAD_SPEC>`"]
pub type SVDVAD = crate::Reg<svdvad::SVDVAD_SPEC>;
#[doc = "Slave Device Address Register %s"]
pub mod svdvad;
#[doc = "BITCNT (r) register accessor: an alias for `Reg<BITCNT_SPEC>`"]
pub type BITCNT = crate::Reg<bitcnt::BITCNT_SPEC>;
#[doc = "Bit Count Register"]
pub mod bitcnt;
#[doc = "PRSTDBG (r) register accessor: an alias for `Reg<PRSTDBG_SPEC>`"]
pub type PRSTDBG = crate::Reg<prstdbg::PRSTDBG_SPEC>;
#[doc = "Present State Debug Register"]
pub mod prstdbg;
