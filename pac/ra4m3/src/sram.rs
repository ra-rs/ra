#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM Parity Error Operation After Detection Register"]
    pub parioad: PARIOAD,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - SRAM Protection Register"]
    pub sramprcr: SRAMPRCR,
    _reserved2: [u8; 0x03],
    #[doc = "0x08 - SRAM Wait State Control Register"]
    pub sramwtsc: SRAMWTSC,
    _reserved3: [u8; 0x03],
    #[doc = "0x0c - SRAM Protection Register 2"]
    pub sramprcr2: SRAMPRCR2,
    _reserved4: [u8; 0xb3],
    #[doc = "0xc0 - ECC Operating Mode Control Register"]
    pub eccmode: ECCMODE,
    #[doc = "0xc1 - ECC 2-Bit Error Status Register"]
    pub ecc2sts: ECC2STS,
    #[doc = "0xc2 - ECC 1-Bit Error Information Update Enable Register"]
    pub ecc1stsen: ECC1STSEN,
    #[doc = "0xc3 - ECC 1-Bit Error Status Register"]
    pub ecc1sts: ECC1STS,
    #[doc = "0xc4 - ECC Protection Register"]
    pub eccprcr: ECCPRCR,
    _reserved9: [u8; 0x0b],
    #[doc = "0xd0 - ECC Protection Register 2"]
    pub eccprcr2: ECCPRCR2,
    _reserved10: [u8; 0x03],
    #[doc = "0xd4 - ECC Test Control Register"]
    pub eccetst: ECCETST,
    _reserved11: [u8; 0x03],
    #[doc = "0xd8 - SRAM ECC Error Operation After Detection Register"]
    pub eccoad: ECCOAD,
}
#[doc = "PARIOAD (rw) register accessor: an alias for `Reg<PARIOAD_SPEC>`"]
pub type PARIOAD = crate::Reg<parioad::PARIOAD_SPEC>;
#[doc = "SRAM Parity Error Operation After Detection Register"]
pub mod parioad;
#[doc = "SRAMPRCR (rw) register accessor: an alias for `Reg<SRAMPRCR_SPEC>`"]
pub type SRAMPRCR = crate::Reg<sramprcr::SRAMPRCR_SPEC>;
#[doc = "SRAM Protection Register"]
pub mod sramprcr;
#[doc = "SRAMWTSC (rw) register accessor: an alias for `Reg<SRAMWTSC_SPEC>`"]
pub type SRAMWTSC = crate::Reg<sramwtsc::SRAMWTSC_SPEC>;
#[doc = "SRAM Wait State Control Register"]
pub mod sramwtsc;
#[doc = "SRAMPRCR2 (rw) register accessor: an alias for `Reg<SRAMPRCR2_SPEC>`"]
pub type SRAMPRCR2 = crate::Reg<sramprcr2::SRAMPRCR2_SPEC>;
#[doc = "SRAM Protection Register 2"]
pub mod sramprcr2;
#[doc = "ECCMODE (rw) register accessor: an alias for `Reg<ECCMODE_SPEC>`"]
pub type ECCMODE = crate::Reg<eccmode::ECCMODE_SPEC>;
#[doc = "ECC Operating Mode Control Register"]
pub mod eccmode;
#[doc = "ECC2STS (rw) register accessor: an alias for `Reg<ECC2STS_SPEC>`"]
pub type ECC2STS = crate::Reg<ecc2sts::ECC2STS_SPEC>;
#[doc = "ECC 2-Bit Error Status Register"]
pub mod ecc2sts;
#[doc = "ECC1STSEN (rw) register accessor: an alias for `Reg<ECC1STSEN_SPEC>`"]
pub type ECC1STSEN = crate::Reg<ecc1stsen::ECC1STSEN_SPEC>;
#[doc = "ECC 1-Bit Error Information Update Enable Register"]
pub mod ecc1stsen;
#[doc = "ECC1STS (rw) register accessor: an alias for `Reg<ECC1STS_SPEC>`"]
pub type ECC1STS = crate::Reg<ecc1sts::ECC1STS_SPEC>;
#[doc = "ECC 1-Bit Error Status Register"]
pub mod ecc1sts;
#[doc = "ECCPRCR (rw) register accessor: an alias for `Reg<ECCPRCR_SPEC>`"]
pub type ECCPRCR = crate::Reg<eccprcr::ECCPRCR_SPEC>;
#[doc = "ECC Protection Register"]
pub mod eccprcr;
#[doc = "ECCPRCR2 (rw) register accessor: an alias for `Reg<ECCPRCR2_SPEC>`"]
pub type ECCPRCR2 = crate::Reg<eccprcr2::ECCPRCR2_SPEC>;
#[doc = "ECC Protection Register 2"]
pub mod eccprcr2;
#[doc = "ECCETST (rw) register accessor: an alias for `Reg<ECCETST_SPEC>`"]
pub type ECCETST = crate::Reg<eccetst::ECCETST_SPEC>;
#[doc = "ECC Test Control Register"]
pub mod eccetst;
#[doc = "ECCOAD (rw) register accessor: an alias for `Reg<ECCOAD_SPEC>`"]
pub type ECCOAD = crate::Reg<eccoad::ECCOAD_SPEC>;
#[doc = "SRAM ECC Error Operation After Detection Register"]
pub mod eccoad;
