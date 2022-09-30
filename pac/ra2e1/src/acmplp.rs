#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ACMPLP Mode Setting Register"]
    pub compmdr: COMPMDR,
    #[doc = "0x01 - ACMPLP Filter Control Register"]
    pub compfir: COMPFIR,
    #[doc = "0x02 - ACMPLP Output Control Register"]
    pub compocr: COMPOCR,
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
