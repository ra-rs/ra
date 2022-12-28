#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Transfer Mode Control Register"]
    pub sfmsmd: SFMSMD,
    #[doc = "0x04 - Chip Selection Control Register"]
    pub sfmssc: SFMSSC,
    #[doc = "0x08 - Clock Control Register"]
    pub sfmskc: SFMSKC,
    #[doc = "0x0c - Status Register"]
    pub sfmsst: SFMSST,
    #[doc = "0x10 - Communication Port Register"]
    pub sfmcom: SFMCOM,
    #[doc = "0x14 - Communication Mode Control Register"]
    pub sfmcmd: SFMCMD,
    #[doc = "0x18 - Communication Status Register"]
    pub sfmcst: SFMCST,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - Instruction Code Register"]
    pub sfmsic: SFMSIC,
    #[doc = "0x24 - Address Mode Control Register"]
    pub sfmsac: SFMSAC,
    #[doc = "0x28 - Dummy Cycle Control Register"]
    pub sfmsdc: SFMSDC,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - SPI Protocol Control Register"]
    pub sfmspc: SFMSPC,
    #[doc = "0x34 - Port Control Register"]
    pub sfmpmd: SFMPMD,
    _reserved12: [u8; 0x07cc],
    #[doc = "0x804 - External QSPI Address Register"]
    pub sfmcnt1: SFMCNT1,
}
#[doc = "SFMSMD (rw) register accessor: an alias for `Reg<SFMSMD_SPEC>`"]
pub type SFMSMD = crate::Reg<sfmsmd::SFMSMD_SPEC>;
#[doc = "Transfer Mode Control Register"]
pub mod sfmsmd;
#[doc = "SFMSSC (rw) register accessor: an alias for `Reg<SFMSSC_SPEC>`"]
pub type SFMSSC = crate::Reg<sfmssc::SFMSSC_SPEC>;
#[doc = "Chip Selection Control Register"]
pub mod sfmssc;
#[doc = "SFMSKC (rw) register accessor: an alias for `Reg<SFMSKC_SPEC>`"]
pub type SFMSKC = crate::Reg<sfmskc::SFMSKC_SPEC>;
#[doc = "Clock Control Register"]
pub mod sfmskc;
#[doc = "SFMSST (r) register accessor: an alias for `Reg<SFMSST_SPEC>`"]
pub type SFMSST = crate::Reg<sfmsst::SFMSST_SPEC>;
#[doc = "Status Register"]
pub mod sfmsst;
#[doc = "SFMCOM (rw) register accessor: an alias for `Reg<SFMCOM_SPEC>`"]
pub type SFMCOM = crate::Reg<sfmcom::SFMCOM_SPEC>;
#[doc = "Communication Port Register"]
pub mod sfmcom;
#[doc = "SFMCMD (rw) register accessor: an alias for `Reg<SFMCMD_SPEC>`"]
pub type SFMCMD = crate::Reg<sfmcmd::SFMCMD_SPEC>;
#[doc = "Communication Mode Control Register"]
pub mod sfmcmd;
#[doc = "SFMCST (rw) register accessor: an alias for `Reg<SFMCST_SPEC>`"]
pub type SFMCST = crate::Reg<sfmcst::SFMCST_SPEC>;
#[doc = "Communication Status Register"]
pub mod sfmcst;
#[doc = "SFMSIC (rw) register accessor: an alias for `Reg<SFMSIC_SPEC>`"]
pub type SFMSIC = crate::Reg<sfmsic::SFMSIC_SPEC>;
#[doc = "Instruction Code Register"]
pub mod sfmsic;
#[doc = "SFMSAC (rw) register accessor: an alias for `Reg<SFMSAC_SPEC>`"]
pub type SFMSAC = crate::Reg<sfmsac::SFMSAC_SPEC>;
#[doc = "Address Mode Control Register"]
pub mod sfmsac;
#[doc = "SFMSDC (rw) register accessor: an alias for `Reg<SFMSDC_SPEC>`"]
pub type SFMSDC = crate::Reg<sfmsdc::SFMSDC_SPEC>;
#[doc = "Dummy Cycle Control Register"]
pub mod sfmsdc;
#[doc = "SFMSPC (rw) register accessor: an alias for `Reg<SFMSPC_SPEC>`"]
pub type SFMSPC = crate::Reg<sfmspc::SFMSPC_SPEC>;
#[doc = "SPI Protocol Control Register"]
pub mod sfmspc;
#[doc = "SFMPMD (rw) register accessor: an alias for `Reg<SFMPMD_SPEC>`"]
pub type SFMPMD = crate::Reg<sfmpmd::SFMPMD_SPEC>;
#[doc = "Port Control Register"]
pub mod sfmpmd;
#[doc = "SFMCNT1 (rw) register accessor: an alias for `Reg<SFMCNT1_SPEC>`"]
pub type SFMCNT1 = crate::Reg<sfmcnt1::SFMCNT1_SPEC>;
#[doc = "External QSPI Address Register"]
pub mod sfmcnt1;
