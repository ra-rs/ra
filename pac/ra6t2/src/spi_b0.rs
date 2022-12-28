#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI Data Register"]
    pub spdr: SPDR,
    #[doc = "0x04 - SPI Delay Control Register"]
    pub spdecr: SPDECR,
    #[doc = "0x08 - SPI Control Register"]
    pub spcr: SPCR,
    #[doc = "0x0c - SPI Control Register 2"]
    pub spcr2: SPCR2,
    #[doc = "0x10 - SPI Control Register 3"]
    pub spcr3: SPCR3,
    #[doc = "0x14..0x34 - SPI Command Register"]
    pub spcmd: [SPCMD; 8],
    _reserved6: [u8; 0x0c],
    #[doc = "0x40 - SPI Data Control Register"]
    pub spdcr: SPDCR,
    #[doc = "0x44 - SPI Data Control Register 2"]
    pub spdcr2: SPDCR2,
    _reserved8: [u8; 0x08],
    #[doc = "0x50 - SPI Status Register"]
    pub spsr: SPSR,
    _reserved9: [u8; 0x04],
    #[doc = "0x58 - SPI Transfer FIFO Status Register"]
    pub sptfsr: SPTFSR,
    #[doc = "0x5c - SPI Receive FIFO Status Register"]
    pub sprfsr: SPRFSR,
    #[doc = "0x60 - SPI Polling Register"]
    pub sppsr: SPPSR,
    _reserved12: [u8; 0x04],
    #[doc = "0x68 - SPI Status Clear Register"]
    pub spsrc: SPSRC,
    #[doc = "0x6c - SPI FIFO Clear Register"]
    pub spfcr: SPFCR,
}
#[doc = "SPDR (rw) register accessor: an alias for `Reg<SPDR_SPEC>`"]
pub type SPDR = crate::Reg<spdr::SPDR_SPEC>;
#[doc = "SPI Data Register"]
pub mod spdr;
#[doc = "SPDECR (rw) register accessor: an alias for `Reg<SPDECR_SPEC>`"]
pub type SPDECR = crate::Reg<spdecr::SPDECR_SPEC>;
#[doc = "SPI Delay Control Register"]
pub mod spdecr;
#[doc = "SPCR (rw) register accessor: an alias for `Reg<SPCR_SPEC>`"]
pub type SPCR = crate::Reg<spcr::SPCR_SPEC>;
#[doc = "SPI Control Register"]
pub mod spcr;
#[doc = "SPCR2 (rw) register accessor: an alias for `Reg<SPCR2_SPEC>`"]
pub type SPCR2 = crate::Reg<spcr2::SPCR2_SPEC>;
#[doc = "SPI Control Register 2"]
pub mod spcr2;
#[doc = "SPCR3 (rw) register accessor: an alias for `Reg<SPCR3_SPEC>`"]
pub type SPCR3 = crate::Reg<spcr3::SPCR3_SPEC>;
#[doc = "SPI Control Register 3"]
pub mod spcr3;
#[doc = "SPCMD (rw) register accessor: an alias for `Reg<SPCMD_SPEC>`"]
pub type SPCMD = crate::Reg<spcmd::SPCMD_SPEC>;
#[doc = "SPI Command Register"]
pub mod spcmd;
#[doc = "SPDCR (rw) register accessor: an alias for `Reg<SPDCR_SPEC>`"]
pub type SPDCR = crate::Reg<spdcr::SPDCR_SPEC>;
#[doc = "SPI Data Control Register"]
pub mod spdcr;
#[doc = "SPDCR2 (rw) register accessor: an alias for `Reg<SPDCR2_SPEC>`"]
pub type SPDCR2 = crate::Reg<spdcr2::SPDCR2_SPEC>;
#[doc = "SPI Data Control Register 2"]
pub mod spdcr2;
#[doc = "SPSR (r) register accessor: an alias for `Reg<SPSR_SPEC>`"]
pub type SPSR = crate::Reg<spsr::SPSR_SPEC>;
#[doc = "SPI Status Register"]
pub mod spsr;
#[doc = "SPTFSR (r) register accessor: an alias for `Reg<SPTFSR_SPEC>`"]
pub type SPTFSR = crate::Reg<sptfsr::SPTFSR_SPEC>;
#[doc = "SPI Transfer FIFO Status Register"]
pub mod sptfsr;
#[doc = "SPRFSR (r) register accessor: an alias for `Reg<SPRFSR_SPEC>`"]
pub type SPRFSR = crate::Reg<sprfsr::SPRFSR_SPEC>;
#[doc = "SPI Receive FIFO Status Register"]
pub mod sprfsr;
#[doc = "SPPSR (r) register accessor: an alias for `Reg<SPPSR_SPEC>`"]
pub type SPPSR = crate::Reg<sppsr::SPPSR_SPEC>;
#[doc = "SPI Polling Register"]
pub mod sppsr;
#[doc = "SPSRC (rw) register accessor: an alias for `Reg<SPSRC_SPEC>`"]
pub type SPSRC = crate::Reg<spsrc::SPSRC_SPEC>;
#[doc = "SPI Status Clear Register"]
pub mod spsrc;
#[doc = "SPFCR (rw) register accessor: an alias for `Reg<SPFCR_SPEC>`"]
pub type SPFCR = crate::Reg<spfcr::SPFCR_SPEC>;
#[doc = "SPI FIFO Clear Register"]
pub mod spfcr;
