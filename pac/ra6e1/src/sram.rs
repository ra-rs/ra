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
