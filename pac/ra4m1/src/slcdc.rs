#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LCD Mode Register 0"]
    pub lcdm0: LCDM0,
    #[doc = "0x01 - LCD Mode Register 1"]
    pub lcdm1: LCDM1,
    #[doc = "0x02 - LCD Clock Control Register 0"]
    pub lcdc0: LCDC0,
    #[doc = "0x03 - LCD Boost Level Control Register"]
    pub vlcd: VLCD,
    _reserved4: [u8; 0xfc],
    #[doc = "0x100..0x126 - LCD Display Data Register %s"]
    pub seg: [SEG; 38],
}
#[doc = "LCDM0 (rw) register accessor: an alias for `Reg<LCDM0_SPEC>`"]
pub type LCDM0 = crate::Reg<lcdm0::LCDM0_SPEC>;
#[doc = "LCD Mode Register 0"]
pub mod lcdm0;
#[doc = "LCDM1 (rw) register accessor: an alias for `Reg<LCDM1_SPEC>`"]
pub type LCDM1 = crate::Reg<lcdm1::LCDM1_SPEC>;
#[doc = "LCD Mode Register 1"]
pub mod lcdm1;
#[doc = "LCDC0 (rw) register accessor: an alias for `Reg<LCDC0_SPEC>`"]
pub type LCDC0 = crate::Reg<lcdc0::LCDC0_SPEC>;
#[doc = "LCD Clock Control Register 0"]
pub mod lcdc0;
#[doc = "VLCD (rw) register accessor: an alias for `Reg<VLCD_SPEC>`"]
pub type VLCD = crate::Reg<vlcd::VLCD_SPEC>;
#[doc = "LCD Boost Level Control Register"]
pub mod vlcd;
#[doc = "SEG (rw) register accessor: an alias for `Reg<SEG_SPEC>`"]
pub type SEG = crate::Reg<seg::SEG_SPEC>;
#[doc = "LCD Display Data Register %s"]
pub mod seg;
