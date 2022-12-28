#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ACMPLP Mode Setting Register"]
    pub compmdr: COMPMDR,
    #[doc = "0x01 - ACMPLP Filter Control Register"]
    pub compfir: COMPFIR,
    #[doc = "0x02 - ACMPLP Output Control Register"]
    pub compocr: COMPOCR,
    _reserved3: [u8; 0x01],
    #[doc = "0x04 - Comparator Input Select Register"]
    pub compsel0: COMPSEL0,
    #[doc = "0x05 - Comparator Reference Voltage Select Register"]
    pub compsel1: COMPSEL1,
}
#[doc = "COMPMDR (rw) register accessor: an alias for `Reg<COMPMDR_SPEC>`"]
pub type COMPMDR = crate::Reg<compmdr::COMPMDR_SPEC>;
#[doc = "ACMPLP Mode Setting Register"]
pub mod compmdr;
#[doc = "COMPFIR (rw) register accessor: an alias for `Reg<COMPFIR_SPEC>`"]
pub type COMPFIR = crate::Reg<compfir::COMPFIR_SPEC>;
#[doc = "ACMPLP Filter Control Register"]
pub mod compfir;
#[doc = "COMPOCR (rw) register accessor: an alias for `Reg<COMPOCR_SPEC>`"]
pub type COMPOCR = crate::Reg<compocr::COMPOCR_SPEC>;
#[doc = "ACMPLP Output Control Register"]
pub mod compocr;
#[doc = "COMPSEL0 (rw) register accessor: an alias for `Reg<COMPSEL0_SPEC>`"]
pub type COMPSEL0 = crate::Reg<compsel0::COMPSEL0_SPEC>;
#[doc = "Comparator Input Select Register"]
pub mod compsel0;
#[doc = "COMPSEL1 (rw) register accessor: an alias for `Reg<COMPSEL1_SPEC>`"]
pub type COMPSEL1 = crate::Reg<compsel1::COMPSEL1_SPEC>;
#[doc = "Comparator Reference Voltage Select Register"]
pub mod compsel1;
