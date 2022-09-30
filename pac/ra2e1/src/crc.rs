#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC Control Register 0"]
    pub crccr0: CRCCR0,
    #[doc = "0x01 - CRC Control Register 1"]
    pub crccr1: CRCCR1,
    _reserved2: [u8; 0x02],
    _reserved_2_crcdir: [u8; 0x04],
    _reserved_3_crcdor: [u8; 0x04],
    #[doc = "0x0c - Snoop Address Register"]
    pub crcsar: CRCSAR,
}
impl RegisterBlock {
    #[doc = "0x04 - CRC Data Input Register"]
    #[inline(always)]
    pub fn crcdir_by(&self) -> &CRCDIR_BY {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const CRCDIR_BY) }
    }
    #[doc = "0x04 - CRC Data Input Register"]
    #[inline(always)]
    pub fn crcdir(&self) -> &CRCDIR {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const CRCDIR) }
    }
    #[doc = "0x08 - CRC Data Output Register"]
    #[inline(always)]
    pub fn crcdor_by(&self) -> &CRCDOR_BY {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const CRCDOR_BY) }
    }
    #[doc = "0x08 - CRC Data Output Register"]
    #[inline(always)]
    pub fn crcdor_ha(&self) -> &CRCDOR_HA {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const CRCDOR_HA) }
    }
    #[doc = "0x08 - CRC Data Output Register"]
    #[inline(always)]
    pub fn crcdor(&self) -> &CRCDOR {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const CRCDOR) }
    }
}
#[doc = "CRCCR0 (rw) register accessor: an alias for `Reg<CRCCR0_SPEC>`"]
pub type CRCCR0 = crate::Reg<crccr0::CRCCR0_SPEC>;
#[doc = "CRC Control Register 0"]
pub mod crccr0;
#[doc = "CRCCR1 (rw) register accessor: an alias for `Reg<CRCCR1_SPEC>`"]
pub type CRCCR1 = crate::Reg<crccr1::CRCCR1_SPEC>;
#[doc = "CRC Control Register 1"]
pub mod crccr1;
#[doc = "CRCDIR (rw) register accessor: an alias for `Reg<CRCDIR_SPEC>`"]
pub type CRCDIR = crate::Reg<crcdir::CRCDIR_SPEC>;
#[doc = "CRC Data Input Register"]
pub mod crcdir;
#[doc = "CRCDIR_BY (rw) register accessor: an alias for `Reg<CRCDIR_BY_SPEC>`"]
pub type CRCDIR_BY = crate::Reg<crcdir_by::CRCDIR_BY_SPEC>;
#[doc = "CRC Data Input Register"]
pub mod crcdir_by;
#[doc = "CRCDOR (rw) register accessor: an alias for `Reg<CRCDOR_SPEC>`"]
pub type CRCDOR = crate::Reg<crcdor::CRCDOR_SPEC>;
#[doc = "CRC Data Output Register"]
pub mod crcdor;
#[doc = "CRCDOR_HA (rw) register accessor: an alias for `Reg<CRCDOR_HA_SPEC>`"]
pub type CRCDOR_HA = crate::Reg<crcdor_ha::CRCDOR_HA_SPEC>;
#[doc = "CRC Data Output Register"]
pub mod crcdor_ha;
#[doc = "CRCDOR_BY (rw) register accessor: an alias for `Reg<CRCDOR_BY_SPEC>`"]
pub type CRCDOR_BY = crate::Reg<crcdor_by::CRCDOR_BY_SPEC>;
#[doc = "CRC Data Output Register"]
pub mod crcdor_by;
#[doc = "CRCSAR (rw) register accessor: an alias for `Reg<CRCSAR_SPEC>`"]
pub type CRCSAR = crate::Reg<crcsar::CRCSAR_SPEC>;
#[doc = "Snoop Address Register"]
pub mod crcsar;
