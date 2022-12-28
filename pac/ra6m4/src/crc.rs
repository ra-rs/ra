#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC Control Register 0"]
    pub crccr0: CRCCR0,
    _reserved1: [u8; 0x03],
    _reserved_1_crcdir: [u8; 0x04],
    _reserved_2_crcdor: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x04 - CRC Data Input Register"]
    #[inline(always)]
    pub const fn crcdir_by(&self) -> &CRCDIR_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - CRC Data Input Register"]
    #[inline(always)]
    pub const fn crcdir(&self) -> &CRCDIR {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x08 - CRC Data Output Register"]
    #[inline(always)]
    pub const fn crcdor_by(&self) -> &CRCDOR_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x08 - CRC Data Output Register"]
    #[inline(always)]
    pub const fn crcdor_ha(&self) -> &CRCDOR_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x08 - CRC Data Output Register"]
    #[inline(always)]
    pub const fn crcdor(&self) -> &CRCDOR {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
}
#[doc = "CRCCR0 (rw) register accessor: an alias for `Reg<CRCCR0_SPEC>`"]
pub type CRCCR0 = crate::Reg<crccr0::CRCCR0_SPEC>;
#[doc = "CRC Control Register 0"]
pub mod crccr0;
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
