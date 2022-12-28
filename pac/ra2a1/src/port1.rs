#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_pdr: [u8; 0x04],
    _reserved_1_eidr: [u8; 0x04],
    _reserved_2_porr: [u8; 0x04],
    _reserved_3_eorr: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - Output data register"]
    #[inline(always)]
    pub const fn podr(&self) -> &PODR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - Port Control Register 1"]
    #[inline(always)]
    pub const fn pcntr1(&self) -> &PCNTR1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x02 - Direction register"]
    #[inline(always)]
    pub const fn pdr(&self) -> &PDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(2usize).cast() }
    }
    #[doc = "0x04 - Event input data register"]
    #[inline(always)]
    pub const fn eidr(&self) -> &EIDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - Port Control Register 2"]
    #[inline(always)]
    pub const fn pcntr2(&self) -> &PCNTR2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x06 - Input data register"]
    #[inline(always)]
    pub const fn pidr(&self) -> &PIDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(6usize).cast() }
    }
    #[doc = "0x08 - Output reset register"]
    #[inline(always)]
    pub const fn porr(&self) -> &PORR {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x08 - Port Control Register 3"]
    #[inline(always)]
    pub const fn pcntr3(&self) -> &PCNTR3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x0a - Output set register"]
    #[inline(always)]
    pub const fn posr(&self) -> &POSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(10usize).cast() }
    }
    #[doc = "0x0c - Event output reset register"]
    #[inline(always)]
    pub const fn eorr(&self) -> &EORR {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
    #[doc = "0x0c - Port Control Register 4"]
    #[inline(always)]
    pub const fn pcntr4(&self) -> &PCNTR4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
    #[doc = "0x0e - Event output set register"]
    #[inline(always)]
    pub const fn eosr(&self) -> &EOSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(14usize).cast() }
    }
}
#[doc = "PCNTR1 (rw) register accessor: an alias for `Reg<PCNTR1_SPEC>`"]
pub type PCNTR1 = crate::Reg<pcntr1::PCNTR1_SPEC>;
#[doc = "Port Control Register 1"]
pub mod pcntr1;
#[doc = "PODR (rw) register accessor: an alias for `Reg<PODR_SPEC>`"]
pub type PODR = crate::Reg<podr::PODR_SPEC>;
#[doc = "Output data register"]
pub mod podr;
#[doc = "PDR (rw) register accessor: an alias for `Reg<PDR_SPEC>`"]
pub type PDR = crate::Reg<pdr::PDR_SPEC>;
#[doc = "Direction register"]
pub mod pdr;
#[doc = "PCNTR2 (r) register accessor: an alias for `Reg<PCNTR2_SPEC>`"]
pub type PCNTR2 = crate::Reg<pcntr2::PCNTR2_SPEC>;
#[doc = "Port Control Register 2"]
pub mod pcntr2;
#[doc = "EIDR (r) register accessor: an alias for `Reg<EIDR_SPEC>`"]
pub type EIDR = crate::Reg<eidr::EIDR_SPEC>;
#[doc = "Event input data register"]
pub mod eidr;
#[doc = "PIDR (r) register accessor: an alias for `Reg<PIDR_SPEC>`"]
pub type PIDR = crate::Reg<pidr::PIDR_SPEC>;
#[doc = "Input data register"]
pub mod pidr;
#[doc = "PCNTR3 (w) register accessor: an alias for `Reg<PCNTR3_SPEC>`"]
pub type PCNTR3 = crate::Reg<pcntr3::PCNTR3_SPEC>;
#[doc = "Port Control Register 3"]
pub mod pcntr3;
#[doc = "PORR (w) register accessor: an alias for `Reg<PORR_SPEC>`"]
pub type PORR = crate::Reg<porr::PORR_SPEC>;
#[doc = "Output reset register"]
pub mod porr;
#[doc = "POSR (w) register accessor: an alias for `Reg<POSR_SPEC>`"]
pub type POSR = crate::Reg<posr::POSR_SPEC>;
#[doc = "Output set register"]
pub mod posr;
#[doc = "PCNTR4 (rw) register accessor: an alias for `Reg<PCNTR4_SPEC>`"]
pub type PCNTR4 = crate::Reg<pcntr4::PCNTR4_SPEC>;
#[doc = "Port Control Register 4"]
pub mod pcntr4;
#[doc = "EORR (rw) register accessor: an alias for `Reg<EORR_SPEC>`"]
pub type EORR = crate::Reg<eorr::EORR_SPEC>;
#[doc = "Event output reset register"]
pub mod eorr;
#[doc = "EOSR (rw) register accessor: an alias for `Reg<EOSR_SPEC>`"]
pub type EOSR = crate::Reg<eosr::EOSR_SPEC>;
#[doc = "Event output set register"]
pub mod eosr;
