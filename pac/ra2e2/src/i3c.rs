#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Protocol Selection Register"]
    pub prts: PRTS,
    _reserved1: [u8; 0x10],
    #[doc = "0x14 - Bus Control Register"]
    pub bctl: BCTL,
    #[doc = "0x18 - Master Device Address Register"]
    pub msdvad: MSDVAD,
    _reserved3: [u8; 0x04],
    #[doc = "0x20 - Reset Control Register"]
    pub rstctl: RSTCTL,
    #[doc = "0x24 - Present State Register"]
    pub prsst: PRSST,
    _reserved5: [u8; 0x08],
    #[doc = "0x30 - Internal Status Register"]
    pub inst: INST,
    #[doc = "0x34 - Internal Status Enable Register"]
    pub inste: INSTE,
    #[doc = "0x38 - Internal Interrupt Enable Register"]
    pub inie: INIE,
    #[doc = "0x3c - Internal Status Force Register"]
    pub instfc: INSTFC,
    _reserved9: [u8; 0x04],
    #[doc = "0x44 - Device Characteristic Table Register"]
    pub dvct: DVCT,
    _reserved10: [u8; 0x10],
    #[doc = "0x58 - IBI Notify Control Register"]
    pub ibinctl: IBINCTL,
    _reserved11: [u8; 0x04],
    #[doc = "0x60 - Bus Function Control Register"]
    pub bfctl: BFCTL,
    #[doc = "0x64 - Slave Control Register"]
    pub svctl: SVCTL,
    _reserved13: [u8; 0x08],
    #[doc = "0x70 - Reference Clock Control Register"]
    pub refckctl: REFCKCTL,
    #[doc = "0x74 - Standard Bit Rate Register"]
    pub stdbr: STDBR,
    #[doc = "0x78 - Extended Bit Rate Register"]
    pub extbr: EXTBR,
    #[doc = "0x7c - Bus Free Condition Detection Time Register"]
    pub bfrecdt: BFRECDT,
    #[doc = "0x80 - Bus Available Condition Detection Time Register"]
    pub bavlcdt: BAVLCDT,
    #[doc = "0x84 - Bus Idle Condition Detection Time Register"]
    pub bidlcdt: BIDLCDT,
    #[doc = "0x88 - Output Control Register"]
    pub outctl: OUTCTL,
    #[doc = "0x8c - Input Control Register"]
    pub inctl: INCTL,
    #[doc = "0x90 - Timeout Control Register"]
    pub tmoctl: TMOCTL,
    _reserved22: [u8; 0x0c],
    #[doc = "0xa0 - Acknowledge Control Register"]
    pub ackctl: ACKCTL,
    #[doc = "0xa4 - SCL Stretch Control Register"]
    pub scstrctl: SCSTRCTL,
    _reserved24: [u8; 0x08],
    #[doc = "0xb0 - SCL Stalling Control Register"]
    pub scstlctl: SCSTLCTL,
    _reserved25: [u8; 0x0c],
    #[doc = "0xc0 - Slave Transfer Data Length Register 0"]
    pub svtdlg0: SVTDLG0,
    _reserved26: [u8; 0x7c],
    #[doc = "0x140 - Condition Control Register"]
    pub cndctl: CNDCTL,
    _reserved27: [u8; 0x0c],
    #[doc = "0x150 - Normal Command Queue Port Register"]
    pub ncmdqp: NCMDQP,
    #[doc = "0x154 - Normal Response Queue Port Register"]
    pub nrspqp: NRSPQP,
    _reserved_29_ntdtbp: [u8; 0x04],
    _reserved30: [u8; 0x20],
    #[doc = "0x17c - Normal IBI Queue Port Register"]
    pub nibiqp: NIBIQP,
    #[doc = "0x180 - Normal Receive Status Queue Port Register"]
    pub nrsqp: NRSQP,
    _reserved32: [u8; 0x0c],
    #[doc = "0x190 - Normal Queue Threshold Control Register"]
    pub nqthctl: NQTHCTL,
    #[doc = "0x194 - Normal Transfer Data Buffer Threshold Control Register 0"]
    pub ntbthctl0: NTBTHCTL0,
    _reserved34: [u8; 0x28],
    #[doc = "0x1c0 - Normal Receive Status Queue Threshold Control Register"]
    pub nrqthctl: NRQTHCTL,
    _reserved35: [u8; 0x0c],
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
    _reserved43: [u8; 0x20],
    #[doc = "0x210 - Bus Condition Status Register"]
    pub bcst: BCST,
    #[doc = "0x214 - Slave Status Register"]
    pub svst: SVST,
    _reserved45: [u8; 0x0c],
    #[doc = "0x224 - Device Address Table Basic Register %s"]
    pub datbas0: DATBAS,
    _reserved46: [u8; 0x04],
    #[doc = "0x22c - Device Address Table Basic Register %s"]
    pub datbas1: DATBAS,
    _reserved47: [u8; 0x04],
    #[doc = "0x234 - Device Address Table Basic Register %s"]
    pub datbas2: DATBAS,
    _reserved48: [u8; 0x04],
    #[doc = "0x23c - Device Address Table Basic Register %s"]
    pub datbas3: DATBAS,
    _reserved49: [u8; 0x60],
    #[doc = "0x2a0 - Extended Device Address Table Basic Register"]
    pub exdatbas: EXDATBAS,
    _reserved50: [u8; 0x0c],
    #[doc = "0x2b0 - Slave Device Address Table Basic Register 0"]
    pub sdatbas0: SDATBAS0,
    _reserved51: [u8; 0x1c],
    #[doc = "0x2d0..0x2e0 - Master Device Characteristic Table Register %s"]
    pub msdct: [MSDCT; 4],
    _reserved52: [u8; 0x40],
    #[doc = "0x320 - Slave Device Characteristic Table Register"]
    pub svdct: SVDCT,
    #[doc = "0x324 - Slave Device Characteristic Table Provisional ID Low Register"]
    pub sdctpidl: SDCTPIDL,
    #[doc = "0x328 - Slave Device Characteristic Table Provisional ID High Register"]
    pub sdctpidh: SDCTPIDH,
    _reserved55: [u8; 0x04],
    #[doc = "0x330 - Slave Device Address Register 0"]
    pub svdvad0: SVDVAD0,
    _reserved56: [u8; 0x1c],
    #[doc = "0x350 - CCC Slave Events Command Register"]
    pub csecmd: CSECMD,
    #[doc = "0x354 - CCC Enter Activity State Register"]
    pub ceactst: CEACTST,
    #[doc = "0x358 - CCC Max Write Length Register"]
    pub cmwlg: CMWLG,
    #[doc = "0x35c - CCC Max Read Length Register"]
    pub cmrlg: CMRLG,
    #[doc = "0x360 - CCC Enter Test Mode Register"]
    pub cetstmd: CETSTMD,
    #[doc = "0x364 - CCC Get Device Status Register"]
    pub cgdvst: CGDVST,
    #[doc = "0x368 - CCC Max Data Speed W (Write) Register"]
    pub cmdspw: CMDSPW,
    #[doc = "0x36c - CCC Max Data Speed R (Read) Register"]
    pub cmdspr: CMDSPR,
    #[doc = "0x370 - CCC Max Data Speed T (Turnaround) Register"]
    pub cmdspt: CMDSPT,
    #[doc = "0x374 - CCC Exchange Timing Support Information M (Mode) Register"]
    pub cetsm: CETSM,
    _reserved66: [u8; 0x08],
    #[doc = "0x380 - Bit Count Register"]
    pub bitcnt: BITCNT,
    _reserved67: [u8; 0x10],
    #[doc = "0x394 - Normal Queue Status Level Register"]
    pub nqstlv: NQSTLV,
    #[doc = "0x398 - Normal Data Buffer Status Level Register 0"]
    pub ndbstlv0: NDBSTLV0,
    _reserved69: [u8; 0x24],
    #[doc = "0x3c0 - Normal Receive Status Queue Status Level Register"]
    pub nrsqstlv: NRSQSTLV,
    _reserved70: [u8; 0x08],
    #[doc = "0x3cc - Present State Debug Register"]
    pub prstdbg: PRSTDBG,
    #[doc = "0x3d0 - Master Error Counters Register"]
    pub mserrcnt: MSERRCNT,
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
#[doc = "PRTS (rw) register accessor: an alias for `Reg<PRTS_SPEC>`"]
pub type PRTS = crate::Reg<prts::PRTS_SPEC>;
#[doc = "Protocol Selection Register"]
pub mod prts;
#[doc = "BCTL (rw) register accessor: an alias for `Reg<BCTL_SPEC>`"]
pub type BCTL = crate::Reg<bctl::BCTL_SPEC>;
#[doc = "Bus Control Register"]
pub mod bctl;
#[doc = "MSDVAD (rw) register accessor: an alias for `Reg<MSDVAD_SPEC>`"]
pub type MSDVAD = crate::Reg<msdvad::MSDVAD_SPEC>;
#[doc = "Master Device Address Register"]
pub mod msdvad;
#[doc = "RSTCTL (rw) register accessor: an alias for `Reg<RSTCTL_SPEC>`"]
pub type RSTCTL = crate::Reg<rstctl::RSTCTL_SPEC>;
#[doc = "Reset Control Register"]
pub mod rstctl;
#[doc = "PRSST (rw) register accessor: an alias for `Reg<PRSST_SPEC>`"]
pub type PRSST = crate::Reg<prsst::PRSST_SPEC>;
#[doc = "Present State Register"]
pub mod prsst;
#[doc = "INST (rw) register accessor: an alias for `Reg<INST_SPEC>`"]
pub type INST = crate::Reg<inst::INST_SPEC>;
#[doc = "Internal Status Register"]
pub mod inst;
#[doc = "INSTE (rw) register accessor: an alias for `Reg<INSTE_SPEC>`"]
pub type INSTE = crate::Reg<inste::INSTE_SPEC>;
#[doc = "Internal Status Enable Register"]
pub mod inste;
#[doc = "INIE (rw) register accessor: an alias for `Reg<INIE_SPEC>`"]
pub type INIE = crate::Reg<inie::INIE_SPEC>;
#[doc = "Internal Interrupt Enable Register"]
pub mod inie;
#[doc = "INSTFC (w) register accessor: an alias for `Reg<INSTFC_SPEC>`"]
pub type INSTFC = crate::Reg<instfc::INSTFC_SPEC>;
#[doc = "Internal Status Force Register"]
pub mod instfc;
#[doc = "DVCT (r) register accessor: an alias for `Reg<DVCT_SPEC>`"]
pub type DVCT = crate::Reg<dvct::DVCT_SPEC>;
#[doc = "Device Characteristic Table Register"]
pub mod dvct;
#[doc = "IBINCTL (rw) register accessor: an alias for `Reg<IBINCTL_SPEC>`"]
pub type IBINCTL = crate::Reg<ibinctl::IBINCTL_SPEC>;
#[doc = "IBI Notify Control Register"]
pub mod ibinctl;
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
#[doc = "BAVLCDT (rw) register accessor: an alias for `Reg<BAVLCDT_SPEC>`"]
pub type BAVLCDT = crate::Reg<bavlcdt::BAVLCDT_SPEC>;
#[doc = "Bus Available Condition Detection Time Register"]
pub mod bavlcdt;
#[doc = "BIDLCDT (rw) register accessor: an alias for `Reg<BIDLCDT_SPEC>`"]
pub type BIDLCDT = crate::Reg<bidlcdt::BIDLCDT_SPEC>;
#[doc = "Bus Idle Condition Detection Time Register"]
pub mod bidlcdt;
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
#[doc = "SCSTLCTL (rw) register accessor: an alias for `Reg<SCSTLCTL_SPEC>`"]
pub type SCSTLCTL = crate::Reg<scstlctl::SCSTLCTL_SPEC>;
#[doc = "SCL Stalling Control Register"]
pub mod scstlctl;
#[doc = "SVTDLG0 (rw) register accessor: an alias for `Reg<SVTDLG0_SPEC>`"]
pub type SVTDLG0 = crate::Reg<svtdlg0::SVTDLG0_SPEC>;
#[doc = "Slave Transfer Data Length Register 0"]
pub mod svtdlg0;
#[doc = "CNDCTL (rw) register accessor: an alias for `Reg<CNDCTL_SPEC>`"]
pub type CNDCTL = crate::Reg<cndctl::CNDCTL_SPEC>;
#[doc = "Condition Control Register"]
pub mod cndctl;
#[doc = "NCMDQP (w) register accessor: an alias for `Reg<NCMDQP_SPEC>`"]
pub type NCMDQP = crate::Reg<ncmdqp::NCMDQP_SPEC>;
#[doc = "Normal Command Queue Port Register"]
pub mod ncmdqp;
#[doc = "NRSPQP (r) register accessor: an alias for `Reg<NRSPQP_SPEC>`"]
pub type NRSPQP = crate::Reg<nrspqp::NRSPQP_SPEC>;
#[doc = "Normal Response Queue Port Register"]
pub mod nrspqp;
#[doc = "NTDTBP0 (rw) register accessor: an alias for `Reg<NTDTBP0_SPEC>`"]
pub type NTDTBP0 = crate::Reg<ntdtbp0::NTDTBP0_SPEC>;
#[doc = "Normal Transfer Data Buffer Port Register 0"]
pub mod ntdtbp0;
#[doc = "NTDTBP0_BY (rw) register accessor: an alias for `Reg<NTDTBP0_BY_SPEC>`"]
pub type NTDTBP0_BY = crate::Reg<ntdtbp0_by::NTDTBP0_BY_SPEC>;
#[doc = "Normal Transfer Data Buffer Port Register 0"]
pub mod ntdtbp0_by;
#[doc = "NIBIQP (rw) register accessor: an alias for `Reg<NIBIQP_SPEC>`"]
pub type NIBIQP = crate::Reg<nibiqp::NIBIQP_SPEC>;
#[doc = "Normal IBI Queue Port Register"]
pub mod nibiqp;
#[doc = "NRSQP (r) register accessor: an alias for `Reg<NRSQP_SPEC>`"]
pub type NRSQP = crate::Reg<nrsqp::NRSQP_SPEC>;
#[doc = "Normal Receive Status Queue Port Register"]
pub mod nrsqp;
#[doc = "NQTHCTL (rw) register accessor: an alias for `Reg<NQTHCTL_SPEC>`"]
pub type NQTHCTL = crate::Reg<nqthctl::NQTHCTL_SPEC>;
#[doc = "Normal Queue Threshold Control Register"]
pub mod nqthctl;
#[doc = "NTBTHCTL0 (rw) register accessor: an alias for `Reg<NTBTHCTL0_SPEC>`"]
pub type NTBTHCTL0 = crate::Reg<ntbthctl0::NTBTHCTL0_SPEC>;
#[doc = "Normal Transfer Data Buffer Threshold Control Register 0"]
pub mod ntbthctl0;
#[doc = "NRQTHCTL (rw) register accessor: an alias for `Reg<NRQTHCTL_SPEC>`"]
pub type NRQTHCTL = crate::Reg<nrqthctl::NRQTHCTL_SPEC>;
#[doc = "Normal Receive Status Queue Threshold Control Register"]
pub mod nrqthctl;
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
#[doc = "DATBAS (rw) register accessor: an alias for `Reg<DATBAS_SPEC>`"]
pub type DATBAS = crate::Reg<datbas::DATBAS_SPEC>;
#[doc = "Device Address Table Basic Register %s"]
pub mod datbas;
#[doc = "EXDATBAS (rw) register accessor: an alias for `Reg<EXDATBAS_SPEC>`"]
pub type EXDATBAS = crate::Reg<exdatbas::EXDATBAS_SPEC>;
#[doc = "Extended Device Address Table Basic Register"]
pub mod exdatbas;
#[doc = "SDATBAS0 (rw) register accessor: an alias for `Reg<SDATBAS0_SPEC>`"]
pub type SDATBAS0 = crate::Reg<sdatbas0::SDATBAS0_SPEC>;
#[doc = "Slave Device Address Table Basic Register 0"]
pub mod sdatbas0;
#[doc = "MSDCT (rw) register accessor: an alias for `Reg<MSDCT_SPEC>`"]
pub type MSDCT = crate::Reg<msdct::MSDCT_SPEC>;
#[doc = "Master Device Characteristic Table Register %s"]
pub mod msdct;
#[doc = "SVDCT (rw) register accessor: an alias for `Reg<SVDCT_SPEC>`"]
pub type SVDCT = crate::Reg<svdct::SVDCT_SPEC>;
#[doc = "Slave Device Characteristic Table Register"]
pub mod svdct;
#[doc = "SDCTPIDL (rw) register accessor: an alias for `Reg<SDCTPIDL_SPEC>`"]
pub type SDCTPIDL = crate::Reg<sdctpidl::SDCTPIDL_SPEC>;
#[doc = "Slave Device Characteristic Table Provisional ID Low Register"]
pub mod sdctpidl;
#[doc = "SDCTPIDH (rw) register accessor: an alias for `Reg<SDCTPIDH_SPEC>`"]
pub type SDCTPIDH = crate::Reg<sdctpidh::SDCTPIDH_SPEC>;
#[doc = "Slave Device Characteristic Table Provisional ID High Register"]
pub mod sdctpidh;
#[doc = "SVDVAD0 (r) register accessor: an alias for `Reg<SVDVAD0_SPEC>`"]
pub type SVDVAD0 = crate::Reg<svdvad0::SVDVAD0_SPEC>;
#[doc = "Slave Device Address Register 0"]
pub mod svdvad0;
#[doc = "CSECMD (rw) register accessor: an alias for `Reg<CSECMD_SPEC>`"]
pub type CSECMD = crate::Reg<csecmd::CSECMD_SPEC>;
#[doc = "CCC Slave Events Command Register"]
pub mod csecmd;
#[doc = "CEACTST (rw) register accessor: an alias for `Reg<CEACTST_SPEC>`"]
pub type CEACTST = crate::Reg<ceactst::CEACTST_SPEC>;
#[doc = "CCC Enter Activity State Register"]
pub mod ceactst;
#[doc = "CMWLG (rw) register accessor: an alias for `Reg<CMWLG_SPEC>`"]
pub type CMWLG = crate::Reg<cmwlg::CMWLG_SPEC>;
#[doc = "CCC Max Write Length Register"]
pub mod cmwlg;
#[doc = "CMRLG (rw) register accessor: an alias for `Reg<CMRLG_SPEC>`"]
pub type CMRLG = crate::Reg<cmrlg::CMRLG_SPEC>;
#[doc = "CCC Max Read Length Register"]
pub mod cmrlg;
#[doc = "CETSTMD (r) register accessor: an alias for `Reg<CETSTMD_SPEC>`"]
pub type CETSTMD = crate::Reg<cetstmd::CETSTMD_SPEC>;
#[doc = "CCC Enter Test Mode Register"]
pub mod cetstmd;
#[doc = "CGDVST (rw) register accessor: an alias for `Reg<CGDVST_SPEC>`"]
pub type CGDVST = crate::Reg<cgdvst::CGDVST_SPEC>;
#[doc = "CCC Get Device Status Register"]
pub mod cgdvst;
#[doc = "CMDSPW (rw) register accessor: an alias for `Reg<CMDSPW_SPEC>`"]
pub type CMDSPW = crate::Reg<cmdspw::CMDSPW_SPEC>;
#[doc = "CCC Max Data Speed W (Write) Register"]
pub mod cmdspw;
#[doc = "CMDSPR (rw) register accessor: an alias for `Reg<CMDSPR_SPEC>`"]
pub type CMDSPR = crate::Reg<cmdspr::CMDSPR_SPEC>;
#[doc = "CCC Max Data Speed R (Read) Register"]
pub mod cmdspr;
#[doc = "CMDSPT (rw) register accessor: an alias for `Reg<CMDSPT_SPEC>`"]
pub type CMDSPT = crate::Reg<cmdspt::CMDSPT_SPEC>;
#[doc = "CCC Max Data Speed T (Turnaround) Register"]
pub mod cmdspt;
#[doc = "CETSM (rw) register accessor: an alias for `Reg<CETSM_SPEC>`"]
pub type CETSM = crate::Reg<cetsm::CETSM_SPEC>;
#[doc = "CCC Exchange Timing Support Information M (Mode) Register"]
pub mod cetsm;
#[doc = "BITCNT (r) register accessor: an alias for `Reg<BITCNT_SPEC>`"]
pub type BITCNT = crate::Reg<bitcnt::BITCNT_SPEC>;
#[doc = "Bit Count Register"]
pub mod bitcnt;
#[doc = "NQSTLV (r) register accessor: an alias for `Reg<NQSTLV_SPEC>`"]
pub type NQSTLV = crate::Reg<nqstlv::NQSTLV_SPEC>;
#[doc = "Normal Queue Status Level Register"]
pub mod nqstlv;
#[doc = "NDBSTLV0 (r) register accessor: an alias for `Reg<NDBSTLV0_SPEC>`"]
pub type NDBSTLV0 = crate::Reg<ndbstlv0::NDBSTLV0_SPEC>;
#[doc = "Normal Data Buffer Status Level Register 0"]
pub mod ndbstlv0;
#[doc = "NRSQSTLV (r) register accessor: an alias for `Reg<NRSQSTLV_SPEC>`"]
pub type NRSQSTLV = crate::Reg<nrsqstlv::NRSQSTLV_SPEC>;
#[doc = "Normal Receive Status Queue Status Level Register"]
pub mod nrsqstlv;
#[doc = "PRSTDBG (r) register accessor: an alias for `Reg<PRSTDBG_SPEC>`"]
pub type PRSTDBG = crate::Reg<prstdbg::PRSTDBG_SPEC>;
#[doc = "Present State Debug Register"]
pub mod prstdbg;
#[doc = "MSERRCNT (r) register accessor: an alias for `Reg<MSERRCNT_SPEC>`"]
pub type MSERRCNT = crate::Reg<mserrcnt::MSERRCNT_SPEC>;
#[doc = "Master Error Counters Register"]
pub mod mserrcnt;
