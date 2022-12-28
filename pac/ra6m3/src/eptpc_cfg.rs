#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EPTPC Reset Register"]
    pub ptrstr: PTRSTR,
    #[doc = "0x04 - STCA Clock Select Register"]
    pub stcselr: STCSELR,
    #[doc = "0x08 - Bypass 1588 module Register"]
    pub bypass: BYPASS,
}
#[doc = "PTRSTR (rw) register accessor: an alias for `Reg<PTRSTR_SPEC>`"]
pub type PTRSTR = crate::Reg<ptrstr::PTRSTR_SPEC>;
#[doc = "EPTPC Reset Register"]
pub mod ptrstr;
#[doc = "STCSELR (rw) register accessor: an alias for `Reg<STCSELR_SPEC>`"]
pub type STCSELR = crate::Reg<stcselr::STCSELR_SPEC>;
#[doc = "STCA Clock Select Register"]
pub mod stcselr;
#[doc = "BYPASS (rw) register accessor: an alias for `Reg<BYPASS_SPEC>`"]
pub type BYPASS = crate::Reg<bypass::BYPASS_SPEC>;
#[doc = "Bypass 1588 module Register"]
pub mod bypass;
