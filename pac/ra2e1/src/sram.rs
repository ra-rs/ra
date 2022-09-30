#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM Parity Error Operation After Detection Register"]
    pub parioad: PARIOAD,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - SRAM Protection Register"]
    pub sramprcr: SRAMPRCR,
}
#[doc = "PARIOAD (rw) register accessor: an alias for `Reg<PARIOAD_SPEC>`"]
pub type PARIOAD = crate::Reg<parioad::PARIOAD_SPEC>;
#[doc = "SRAM Parity Error Operation After Detection Register"]
pub mod parioad;
#[doc = "SRAMPRCR (rw) register accessor: an alias for `Reg<SRAMPRCR_SPEC>`"]
pub type SRAMPRCR = crate::Reg<sramprcr::SRAMPRCR_SPEC>;
#[doc = "SRAM Protection Register"]
pub mod sramprcr;
