#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAC Control Register 0"]
    pub cacr0: CACR0,
    #[doc = "0x01 - CAC Control Register 1"]
    pub cacr1: CACR1,
    #[doc = "0x02 - CAC Control Register 2"]
    pub cacr2: CACR2,
    #[doc = "0x03 - CAC Interrupt Control Register"]
    pub caicr: CAICR,
    #[doc = "0x04 - CAC Status Register"]
    pub castr: CASTR,
    _reserved5: [u8; 0x01],
    #[doc = "0x06 - CAC Upper-Limit Value Setting Register"]
    pub caulvr: CAULVR,
    #[doc = "0x08 - CAC Lower-Limit Value Setting Register"]
    pub callvr: CALLVR,
    #[doc = "0x0a - CAC Counter Buffer Register"]
    pub cacntbr: CACNTBR,
}
#[doc = "CACR0 (rw) register accessor: an alias for `Reg<CACR0_SPEC>`"]
pub type CACR0 = crate::Reg<cacr0::CACR0_SPEC>;
#[doc = "CAC Control Register 0"]
pub mod cacr0;
#[doc = "CACR1 (rw) register accessor: an alias for `Reg<CACR1_SPEC>`"]
pub type CACR1 = crate::Reg<cacr1::CACR1_SPEC>;
#[doc = "CAC Control Register 1"]
pub mod cacr1;
#[doc = "CACR2 (rw) register accessor: an alias for `Reg<CACR2_SPEC>`"]
pub type CACR2 = crate::Reg<cacr2::CACR2_SPEC>;
#[doc = "CAC Control Register 2"]
pub mod cacr2;
#[doc = "CAICR (rw) register accessor: an alias for `Reg<CAICR_SPEC>`"]
pub type CAICR = crate::Reg<caicr::CAICR_SPEC>;
#[doc = "CAC Interrupt Control Register"]
pub mod caicr;
#[doc = "CASTR (r) register accessor: an alias for `Reg<CASTR_SPEC>`"]
pub type CASTR = crate::Reg<castr::CASTR_SPEC>;
#[doc = "CAC Status Register"]
pub mod castr;
#[doc = "CAULVR (rw) register accessor: an alias for `Reg<CAULVR_SPEC>`"]
pub type CAULVR = crate::Reg<caulvr::CAULVR_SPEC>;
#[doc = "CAC Upper-Limit Value Setting Register"]
pub mod caulvr;
#[doc = "CALLVR (rw) register accessor: an alias for `Reg<CALLVR_SPEC>`"]
pub type CALLVR = crate::Reg<callvr::CALLVR_SPEC>;
#[doc = "CAC Lower-Limit Value Setting Register"]
pub mod callvr;
#[doc = "CACNTBR (r) register accessor: an alias for `Reg<CACNTBR_SPEC>`"]
pub type CACNTBR = crate::Reg<cacntbr::CACNTBR_SPEC>;
#[doc = "CAC Counter Buffer Register"]
pub mod cacntbr;
