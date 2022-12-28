#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI Control Register"]
    pub spcr: SPCR,
    #[doc = "0x01 - SPI Slave Select Polarity Register"]
    pub sslp: SSLP,
    #[doc = "0x02 - RSPI Pin Control Register"]
    pub sppcr: SPPCR,
    #[doc = "0x03 - SPI Status Register"]
    pub spsr: SPSR,
    _reserved_4_spdr: [u8; 0x04],
    #[doc = "0x08 - SPI Sequence Control Register"]
    pub spscr: SPSCR,
    #[doc = "0x09 - SPI Sequence Status Register"]
    pub spssr: SPSSR,
    #[doc = "0x0a - SPI Bit Rate Register"]
    pub spbr: SPBR,
    #[doc = "0x0b - SPI Data Control Register"]
    pub spdcr: SPDCR,
    #[doc = "0x0c - SPI Clock Delay Register"]
    pub spckd: SPCKD,
    #[doc = "0x0d - SPI Slave Select Negation Delay Register"]
    pub sslnd: SSLND,
    #[doc = "0x0e - SPI Next-Access Delay Register"]
    pub spnd: SPND,
    #[doc = "0x0f - SPI Control Register 2"]
    pub spcr2: SPCR2,
    #[doc = "0x10..0x20 - SPI Command Register %s"]
    pub spcmd: [SPCMD; 8],
    #[doc = "0x20 - SPI Data Control Register 2"]
    pub spdcr2: SPDCR2,
}
impl RegisterBlock {
    #[doc = "0x04 - SPI Data Register ( halfword access )"]
    #[inline(always)]
    pub const fn spdr_ha(&self) -> &SPDR_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - SPI Data Register"]
    #[inline(always)]
    pub const fn spdr(&self) -> &SPDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
}
#[doc = "SPCR (rw) register accessor: an alias for `Reg<SPCR_SPEC>`"]
pub type SPCR = crate::Reg<spcr::SPCR_SPEC>;
#[doc = "SPI Control Register"]
pub mod spcr;
#[doc = "SSLP (rw) register accessor: an alias for `Reg<SSLP_SPEC>`"]
pub type SSLP = crate::Reg<sslp::SSLP_SPEC>;
#[doc = "SPI Slave Select Polarity Register"]
pub mod sslp;
#[doc = "SPPCR (rw) register accessor: an alias for `Reg<SPPCR_SPEC>`"]
pub type SPPCR = crate::Reg<sppcr::SPPCR_SPEC>;
#[doc = "RSPI Pin Control Register"]
pub mod sppcr;
#[doc = "SPSR (rw) register accessor: an alias for `Reg<SPSR_SPEC>`"]
pub type SPSR = crate::Reg<spsr::SPSR_SPEC>;
#[doc = "SPI Status Register"]
pub mod spsr;
#[doc = "SPDR (rw) register accessor: an alias for `Reg<SPDR_SPEC>`"]
pub type SPDR = crate::Reg<spdr::SPDR_SPEC>;
#[doc = "SPI Data Register"]
pub mod spdr;
#[doc = "SPDR_HA (rw) register accessor: an alias for `Reg<SPDR_HA_SPEC>`"]
pub type SPDR_HA = crate::Reg<spdr_ha::SPDR_HA_SPEC>;
#[doc = "SPI Data Register ( halfword access )"]
pub mod spdr_ha;
#[doc = "SPSCR (rw) register accessor: an alias for `Reg<SPSCR_SPEC>`"]
pub type SPSCR = crate::Reg<spscr::SPSCR_SPEC>;
#[doc = "SPI Sequence Control Register"]
pub mod spscr;
#[doc = "SPSSR (r) register accessor: an alias for `Reg<SPSSR_SPEC>`"]
pub type SPSSR = crate::Reg<spssr::SPSSR_SPEC>;
#[doc = "SPI Sequence Status Register"]
pub mod spssr;
#[doc = "SPBR (rw) register accessor: an alias for `Reg<SPBR_SPEC>`"]
pub type SPBR = crate::Reg<spbr::SPBR_SPEC>;
#[doc = "SPI Bit Rate Register"]
pub mod spbr;
#[doc = "SPDCR (rw) register accessor: an alias for `Reg<SPDCR_SPEC>`"]
pub type SPDCR = crate::Reg<spdcr::SPDCR_SPEC>;
#[doc = "SPI Data Control Register"]
pub mod spdcr;
#[doc = "SPCKD (rw) register accessor: an alias for `Reg<SPCKD_SPEC>`"]
pub type SPCKD = crate::Reg<spckd::SPCKD_SPEC>;
#[doc = "SPI Clock Delay Register"]
pub mod spckd;
#[doc = "SSLND (rw) register accessor: an alias for `Reg<SSLND_SPEC>`"]
pub type SSLND = crate::Reg<sslnd::SSLND_SPEC>;
#[doc = "SPI Slave Select Negation Delay Register"]
pub mod sslnd;
#[doc = "SPND (rw) register accessor: an alias for `Reg<SPND_SPEC>`"]
pub type SPND = crate::Reg<spnd::SPND_SPEC>;
#[doc = "SPI Next-Access Delay Register"]
pub mod spnd;
#[doc = "SPCR2 (rw) register accessor: an alias for `Reg<SPCR2_SPEC>`"]
pub type SPCR2 = crate::Reg<spcr2::SPCR2_SPEC>;
#[doc = "SPI Control Register 2"]
pub mod spcr2;
#[doc = "SPCMD (rw) register accessor: an alias for `Reg<SPCMD_SPEC>`"]
pub type SPCMD = crate::Reg<spcmd::SPCMD_SPEC>;
#[doc = "SPI Command Register %s"]
pub mod spcmd;
#[doc = "SPDCR2 (rw) register accessor: an alias for `Reg<SPDCR2_SPEC>`"]
pub type SPDCR2 = crate::Reg<spdcr2::SPDCR2_SPEC>;
#[doc = "SPI Data Control Register 2"]
pub mod spdcr2;
