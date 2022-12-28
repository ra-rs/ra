#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM Parity Error Operation After Detection Register"]
    pub parioad: PARIOAD,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - SRAM Protection Register"]
    pub sramprcr: SRAMPRCR,
    _reserved2: [u8; 0x03],
    #[doc = "0x08 - RAM Wait State Control Register"]
    pub sramwtsc: SRAMWTSC,
    _reserved3: [u8; 0xb7],
    #[doc = "0xc0 - ECCRAM Operating Mode Control Register"]
    pub eccmode: ECCMODE,
    #[doc = "0xc1 - ECCRAM 2-Bit Error Status Register"]
    pub ecc2sts: ECC2STS,
    #[doc = "0xc2 - ECCRAM 1-Bit Error Information Update Enable Register"]
    pub ecc1stsen: ECC1STSEN,
    #[doc = "0xc3 - ECCRAM 1-Bit Error Status Register"]
    pub ecc1sts: ECC1STS,
    #[doc = "0xc4 - ECCRAM Protection Register"]
    pub eccprcr: ECCPRCR,
    _reserved8: [u8; 0x0f],
    #[doc = "0xd4 - ECC Test Control Register"]
    pub eccetst: ECCETST,
    _reserved9: [u8; 0x03],
    #[doc = "0xd8 - RAM ECC Error Operation After Detection Register"]
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
#[doc = "RAM Wait State Control Register"]
pub mod sramwtsc;
#[doc = "ECCMODE (rw) register accessor: an alias for `Reg<ECCMODE_SPEC>`"]
pub type ECCMODE = crate::Reg<eccmode::ECCMODE_SPEC>;
#[doc = "ECCRAM Operating Mode Control Register"]
pub mod eccmode;
#[doc = "ECC2STS (rw) register accessor: an alias for `Reg<ECC2STS_SPEC>`"]
pub type ECC2STS = crate::Reg<ecc2sts::ECC2STS_SPEC>;
#[doc = "ECCRAM 2-Bit Error Status Register"]
pub mod ecc2sts;
#[doc = "ECC1STSEN (rw) register accessor: an alias for `Reg<ECC1STSEN_SPEC>`"]
pub type ECC1STSEN = crate::Reg<ecc1stsen::ECC1STSEN_SPEC>;
#[doc = "ECCRAM 1-Bit Error Information Update Enable Register"]
pub mod ecc1stsen;
#[doc = "ECC1STS (rw) register accessor: an alias for `Reg<ECC1STS_SPEC>`"]
pub type ECC1STS = crate::Reg<ecc1sts::ECC1STS_SPEC>;
#[doc = "ECCRAM 1-Bit Error Status Register"]
pub mod ecc1sts;
#[doc = "ECCPRCR (rw) register accessor: an alias for `Reg<ECCPRCR_SPEC>`"]
pub type ECCPRCR = crate::Reg<eccprcr::ECCPRCR_SPEC>;
#[doc = "ECCRAM Protection Register"]
pub mod eccprcr;
#[doc = "ECCETST (rw) register accessor: an alias for `Reg<ECCETST_SPEC>`"]
pub type ECCETST = crate::Reg<eccetst::ECCETST_SPEC>;
#[doc = "ECC Test Control Register"]
pub mod eccetst;
#[doc = "ECCOAD (rw) register accessor: an alias for `Reg<ECCOAD_SPEC>`"]
pub type ECCOAD = crate::Reg<eccoad::ECCOAD_SPEC>;
#[doc = "RAM ECC Error Operation After Detection Register"]
pub mod eccoad;
