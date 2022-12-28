#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    #[doc = "0x0c - Standby Control Register"]
    pub sbycr: SBYCR,
    _reserved1: [u8; 0x0e],
    #[doc = "0x1c - Module Stop Control Register A"]
    pub mstpcra: MSTPCRA,
    #[doc = "0x20 - System Clock Division Control Register"]
    pub sckdivcr: SCKDIVCR,
    #[doc = "0x24 - System Clock Division Control Register 2"]
    pub sckdivcr2: SCKDIVCR2,
    _reserved4: [u8; 0x01],
    #[doc = "0x26 - System Clock Source Control Register"]
    pub sckscr: SCKSCR,
    _reserved5: [u8; 0x01],
    #[doc = "0x28 - PLL Clock Control Register"]
    pub pllccr: PLLCCR,
    #[doc = "0x2a - PLL Control Register"]
    pub pllcr: PLLCR,
    _reserved7: [u8; 0x05],
    #[doc = "0x30 - External Bus Clock Control Register"]
    pub bckcr: BCKCR,
    _reserved8: [u8; 0x01],
    #[doc = "0x32 - Main Clock Oscillator Control Register"]
    pub mosccr: MOSCCR,
    _reserved9: [u8; 0x03],
    #[doc = "0x36 - High-Speed On-Chip Oscillator Control Register"]
    pub hococr: HOCOCR,
    _reserved10: [u8; 0x01],
    #[doc = "0x38 - Middle-Speed On-Chip Oscillator Control Register"]
    pub mococr: MOCOCR,
    #[doc = "0x39 - FLL Control Register 1"]
    pub fllcr1: FLLCR1,
    #[doc = "0x3a - FLL Control Register 2"]
    pub fllcr2: FLLCR2,
    #[doc = "0x3c - Oscillation Stabilization Flag Register"]
    pub oscsf: OSCSF,
    _reserved14: [u8; 0x01],
    #[doc = "0x3e - Clock Out Control Register"]
    pub ckocr: CKOCR,
    #[doc = "0x3f - Trace Clock Control Register"]
    pub trckcr: TRCKCR,
    #[doc = "0x40 - Oscillation Stop Detection Control Register"]
    pub ostdcr: OSTDCR,
    #[doc = "0x41 - Oscillation Stop Detection Status Register"]
    pub ostdsr: OSTDSR,
    _reserved18: [u8; 0x10],
    #[doc = "0x52 - External Bus Clock Output Control Register"]
    pub ebckocr: EBCKOCR,
    #[doc = "0x53 - SDRAM Clock Output Control Register"]
    pub sdckocr: SDCKOCR,
    _reserved20: [u8; 0x0d],
    #[doc = "0x61 - MOCO User Trimming Control Register"]
    pub mocoutcr: MOCOUTCR,
    #[doc = "0x62 - HOCO User Trimming Control Register"]
    pub hocoutcr: HOCOUTCR,
    _reserved22: [u8; 0x2f],
    #[doc = "0x92 - Snooze Control Register"]
    pub snzcr: SNZCR,
    _reserved23: [u8; 0x01],
    #[doc = "0x94 - Snooze End Control Register"]
    pub snzedcr: SNZEDCR,
    _reserved24: [u8; 0x03],
    #[doc = "0x98 - Snooze Request Control Register"]
    pub snzreqcr: SNZREQCR,
    _reserved25: [u8; 0x04],
    #[doc = "0xa0 - Operating Power Control Register"]
    pub opccr: OPCCR,
    _reserved26: [u8; 0x01],
    #[doc = "0xa2 - Main Clock Oscillator Wait Control Register"]
    pub moscwtcr: MOSCWTCR,
    _reserved27: [u8; 0x02],
    #[doc = "0xa5 - High-speed on-chip oscillator wait control register"]
    pub hocowtcr: HOCOWTCR,
    _reserved28: [u8; 0x04],
    #[doc = "0xaa - Sub Operating Power Control Register"]
    pub sopccr: SOPCCR,
    _reserved29: [u8; 0x15],
    #[doc = "0xc0 - Reset Status Register 1"]
    pub rstsr1: RSTSR1,
    _reserved30: [u8; 0x1e],
    #[doc = "0xe0 - Voltage Monitor %s Circuit Control Register 1"]
    pub lvd1cr1: LVDCR1,
    #[doc = "0xe1 - Voltage Monitor %s Circuit Status Register"]
    pub lvd1sr: LVDSR,
    #[doc = "0xe2 - Voltage Monitor %s Circuit Control Register 1"]
    pub lvd2cr1: LVDCR1,
    #[doc = "0xe3 - Voltage Monitor %s Circuit Status Register"]
    pub lvd2sr: LVDSR,
    _reserved34: [u8; 0x031a],
    #[doc = "0x3fe - Protect Register"]
    pub prcr: PRCR,
    #[doc = "0x400 - Deep Standby Control Register"]
    pub dpsbycr: DPSBYCR,
    _reserved36: [u8; 0x01],
    #[doc = "0x402 - Deep Standby Interrupt Enable Register 0"]
    pub dpsier0: DPSIER0,
    #[doc = "0x403 - Deep Standby Interrupt Enable Register 1"]
    pub dpsier1: DPSIER1,
    #[doc = "0x404 - Deep Standby Interrupt Enable Register 2"]
    pub dpsier2: DPSIER2,
    #[doc = "0x405 - Deep Standby Interrupt Enable Register 3"]
    pub dpsier3: DPSIER3,
    #[doc = "0x406 - Deep Standby Interrupt Flag Register 0"]
    pub dpsifr0: DPSIFR0,
    #[doc = "0x407 - Deep Standby Interrupt Flag Register 1"]
    pub dpsifr1: DPSIFR1,
    #[doc = "0x408 - Deep Standby Interrupt Flag Register 2"]
    pub dpsifr2: DPSIFR2,
    #[doc = "0x409 - Deep Standby Interrupt Flag Register 3"]
    pub dpsifr3: DPSIFR3,
    #[doc = "0x40a - Deep Standby Interrupt Edge Register 0"]
    pub dpsiegr0: DPSIEGR0,
    #[doc = "0x40b - Deep Standby Interrupt Edge Register 1"]
    pub dpsiegr1: DPSIEGR1,
    #[doc = "0x40c - Deep Standby Interrupt Edge Register 2"]
    pub dpsiegr2: DPSIEGR2,
    _reserved47: [u8; 0x01],
    #[doc = "0x40e - System Control OCD Control Register"]
    pub syocdcr: SYOCDCR,
    #[doc = "0x40f - Standby Condition Register"]
    pub stconr: STCONR,
    #[doc = "0x410 - Reset Status Register 0"]
    pub rstsr0: RSTSR0,
    #[doc = "0x411 - Reset Status Register 2"]
    pub rstsr2: RSTSR2,
    _reserved51: [u8; 0x01],
    #[doc = "0x413 - Main Clock Oscillator Mode Oscillation Control Register"]
    pub momcr: MOMCR,
    _reserved52: [u8; 0x02],
    #[doc = "0x416 - Flash P/E Protect Register"]
    pub fwepror: FWEPROR,
    #[doc = "0x417 - Voltage Monitor Circuit Control Register"]
    pub lvcmpcr: LVCMPCR,
    #[doc = "0x418 - Voltage Detection Level Select Register"]
    pub lvdlvlr: LVDLVLR,
    _reserved55: [u8; 0x01],
    #[doc = "0x41a - Voltage Monitor %s Circuit Control Register 0"]
    pub lvdcr0: [LVDCR0; 2],
    _reserved56: [u8; 0x64],
    #[doc = "0x480 - Sub-clock oscillator control register"]
    pub sosccr: SOSCCR,
    #[doc = "0x481 - Sub Clock Oscillator Mode Control Register"]
    pub somcr: SOMCR,
    _reserved58: [u8; 0x0e],
    #[doc = "0x490 - Low-Speed On-Chip Oscillator Control Register"]
    pub lococr: LOCOCR,
    _reserved59: [u8; 0x01],
    #[doc = "0x492 - LOCO User Trimming Control Register"]
    pub locoutcr: LOCOUTCR,
    _reserved60: [u8; 0x28],
    #[doc = "0x4bb - VBATT Input Control Register"]
    pub vbtictlr: VBTICTLR,
    _reserved61: [u8; 0x44],
    #[doc = "0x500..0x700 - VBATT Backup Register \\[%s\\]"]
    pub vbtbkr: [VBTBKR; 512],
}
impl RegisterBlock {
    #[doc = "0x41a - Voltage Monitor %s Circuit Control Register 0"]
    #[inline(always)]
    pub fn lvd1cr0(&self) -> &LVDCR0 {
        &self.lvdcr0[0]
    }
    #[doc = "0x41b - Voltage Monitor %s Circuit Control Register 0"]
    #[inline(always)]
    pub fn lvd2cr0(&self) -> &LVDCR0 {
        &self.lvdcr0[1]
    }
}
#[doc = "FWEPROR (rw) register accessor: an alias for `Reg<FWEPROR_SPEC>`"]
pub type FWEPROR = crate::Reg<fwepror::FWEPROR_SPEC>;
#[doc = "Flash P/E Protect Register"]
pub mod fwepror;
#[doc = "VBTICTLR (rw) register accessor: an alias for `Reg<VBTICTLR_SPEC>`"]
pub type VBTICTLR = crate::Reg<vbtictlr::VBTICTLR_SPEC>;
#[doc = "VBATT Input Control Register"]
pub mod vbtictlr;
#[doc = "VBTBKR (rw) register accessor: an alias for `Reg<VBTBKR_SPEC>`"]
pub type VBTBKR = crate::Reg<vbtbkr::VBTBKR_SPEC>;
#[doc = "VBATT Backup Register \\[%s\\]"]
pub mod vbtbkr;
#[doc = "SCKDIVCR (rw) register accessor: an alias for `Reg<SCKDIVCR_SPEC>`"]
pub type SCKDIVCR = crate::Reg<sckdivcr::SCKDIVCR_SPEC>;
#[doc = "System Clock Division Control Register"]
pub mod sckdivcr;
#[doc = "SCKDIVCR2 (rw) register accessor: an alias for `Reg<SCKDIVCR2_SPEC>`"]
pub type SCKDIVCR2 = crate::Reg<sckdivcr2::SCKDIVCR2_SPEC>;
#[doc = "System Clock Division Control Register 2"]
pub mod sckdivcr2;
#[doc = "SCKSCR (rw) register accessor: an alias for `Reg<SCKSCR_SPEC>`"]
pub type SCKSCR = crate::Reg<sckscr::SCKSCR_SPEC>;
#[doc = "System Clock Source Control Register"]
pub mod sckscr;
#[doc = "PLLCCR (rw) register accessor: an alias for `Reg<PLLCCR_SPEC>`"]
pub type PLLCCR = crate::Reg<pllccr::PLLCCR_SPEC>;
#[doc = "PLL Clock Control Register"]
pub mod pllccr;
#[doc = "PLLCR (rw) register accessor: an alias for `Reg<PLLCR_SPEC>`"]
pub type PLLCR = crate::Reg<pllcr::PLLCR_SPEC>;
#[doc = "PLL Control Register"]
pub mod pllcr;
#[doc = "BCKCR (rw) register accessor: an alias for `Reg<BCKCR_SPEC>`"]
pub type BCKCR = crate::Reg<bckcr::BCKCR_SPEC>;
#[doc = "External Bus Clock Control Register"]
pub mod bckcr;
#[doc = "MOSCCR (rw) register accessor: an alias for `Reg<MOSCCR_SPEC>`"]
pub type MOSCCR = crate::Reg<mosccr::MOSCCR_SPEC>;
#[doc = "Main Clock Oscillator Control Register"]
pub mod mosccr;
#[doc = "HOCOCR (rw) register accessor: an alias for `Reg<HOCOCR_SPEC>`"]
pub type HOCOCR = crate::Reg<hococr::HOCOCR_SPEC>;
#[doc = "High-Speed On-Chip Oscillator Control Register"]
pub mod hococr;
#[doc = "MOCOCR (rw) register accessor: an alias for `Reg<MOCOCR_SPEC>`"]
pub type MOCOCR = crate::Reg<mococr::MOCOCR_SPEC>;
#[doc = "Middle-Speed On-Chip Oscillator Control Register"]
pub mod mococr;
#[doc = "FLLCR1 (rw) register accessor: an alias for `Reg<FLLCR1_SPEC>`"]
pub type FLLCR1 = crate::Reg<fllcr1::FLLCR1_SPEC>;
#[doc = "FLL Control Register 1"]
pub mod fllcr1;
#[doc = "FLLCR2 (rw) register accessor: an alias for `Reg<FLLCR2_SPEC>`"]
pub type FLLCR2 = crate::Reg<fllcr2::FLLCR2_SPEC>;
#[doc = "FLL Control Register 2"]
pub mod fllcr2;
#[doc = "OSCSF (r) register accessor: an alias for `Reg<OSCSF_SPEC>`"]
pub type OSCSF = crate::Reg<oscsf::OSCSF_SPEC>;
#[doc = "Oscillation Stabilization Flag Register"]
pub mod oscsf;
#[doc = "CKOCR (rw) register accessor: an alias for `Reg<CKOCR_SPEC>`"]
pub type CKOCR = crate::Reg<ckocr::CKOCR_SPEC>;
#[doc = "Clock Out Control Register"]
pub mod ckocr;
#[doc = "TRCKCR (rw) register accessor: an alias for `Reg<TRCKCR_SPEC>`"]
pub type TRCKCR = crate::Reg<trckcr::TRCKCR_SPEC>;
#[doc = "Trace Clock Control Register"]
pub mod trckcr;
#[doc = "OSTDCR (rw) register accessor: an alias for `Reg<OSTDCR_SPEC>`"]
pub type OSTDCR = crate::Reg<ostdcr::OSTDCR_SPEC>;
#[doc = "Oscillation Stop Detection Control Register"]
pub mod ostdcr;
#[doc = "OSTDSR (rw) register accessor: an alias for `Reg<OSTDSR_SPEC>`"]
pub type OSTDSR = crate::Reg<ostdsr::OSTDSR_SPEC>;
#[doc = "Oscillation Stop Detection Status Register"]
pub mod ostdsr;
#[doc = "EBCKOCR (rw) register accessor: an alias for `Reg<EBCKOCR_SPEC>`"]
pub type EBCKOCR = crate::Reg<ebckocr::EBCKOCR_SPEC>;
#[doc = "External Bus Clock Output Control Register"]
pub mod ebckocr;
#[doc = "SDCKOCR (rw) register accessor: an alias for `Reg<SDCKOCR_SPEC>`"]
pub type SDCKOCR = crate::Reg<sdckocr::SDCKOCR_SPEC>;
#[doc = "SDRAM Clock Output Control Register"]
pub mod sdckocr;
#[doc = "MOCOUTCR (rw) register accessor: an alias for `Reg<MOCOUTCR_SPEC>`"]
pub type MOCOUTCR = crate::Reg<mocoutcr::MOCOUTCR_SPEC>;
#[doc = "MOCO User Trimming Control Register"]
pub mod mocoutcr;
#[doc = "HOCOUTCR (rw) register accessor: an alias for `Reg<HOCOUTCR_SPEC>`"]
pub type HOCOUTCR = crate::Reg<hocoutcr::HOCOUTCR_SPEC>;
#[doc = "HOCO User Trimming Control Register"]
pub mod hocoutcr;
#[doc = "MOMCR (rw) register accessor: an alias for `Reg<MOMCR_SPEC>`"]
pub type MOMCR = crate::Reg<momcr::MOMCR_SPEC>;
#[doc = "Main Clock Oscillator Mode Oscillation Control Register"]
pub mod momcr;
#[doc = "SOSCCR (rw) register accessor: an alias for `Reg<SOSCCR_SPEC>`"]
pub type SOSCCR = crate::Reg<sosccr::SOSCCR_SPEC>;
#[doc = "Sub-clock oscillator control register"]
pub mod sosccr;
#[doc = "SOMCR (rw) register accessor: an alias for `Reg<SOMCR_SPEC>`"]
pub type SOMCR = crate::Reg<somcr::SOMCR_SPEC>;
#[doc = "Sub Clock Oscillator Mode Control Register"]
pub mod somcr;
#[doc = "LOCOCR (rw) register accessor: an alias for `Reg<LOCOCR_SPEC>`"]
pub type LOCOCR = crate::Reg<lococr::LOCOCR_SPEC>;
#[doc = "Low-Speed On-Chip Oscillator Control Register"]
pub mod lococr;
#[doc = "LOCOUTCR (rw) register accessor: an alias for `Reg<LOCOUTCR_SPEC>`"]
pub type LOCOUTCR = crate::Reg<locoutcr::LOCOUTCR_SPEC>;
#[doc = "LOCO User Trimming Control Register"]
pub mod locoutcr;
#[doc = "MOSCWTCR (rw) register accessor: an alias for `Reg<MOSCWTCR_SPEC>`"]
pub type MOSCWTCR = crate::Reg<moscwtcr::MOSCWTCR_SPEC>;
#[doc = "Main Clock Oscillator Wait Control Register"]
pub mod moscwtcr;
#[doc = "HOCOWTCR (rw) register accessor: an alias for `Reg<HOCOWTCR_SPEC>`"]
pub type HOCOWTCR = crate::Reg<hocowtcr::HOCOWTCR_SPEC>;
#[doc = "High-speed on-chip oscillator wait control register"]
pub mod hocowtcr;
#[doc = "SBYCR (rw) register accessor: an alias for `Reg<SBYCR_SPEC>`"]
pub type SBYCR = crate::Reg<sbycr::SBYCR_SPEC>;
#[doc = "Standby Control Register"]
pub mod sbycr;
#[doc = "MSTPCRA (rw) register accessor: an alias for `Reg<MSTPCRA_SPEC>`"]
pub type MSTPCRA = crate::Reg<mstpcra::MSTPCRA_SPEC>;
#[doc = "Module Stop Control Register A"]
pub mod mstpcra;
#[doc = "SNZCR (rw) register accessor: an alias for `Reg<SNZCR_SPEC>`"]
pub type SNZCR = crate::Reg<snzcr::SNZCR_SPEC>;
#[doc = "Snooze Control Register"]
pub mod snzcr;
#[doc = "SNZEDCR (rw) register accessor: an alias for `Reg<SNZEDCR_SPEC>`"]
pub type SNZEDCR = crate::Reg<snzedcr::SNZEDCR_SPEC>;
#[doc = "Snooze End Control Register"]
pub mod snzedcr;
#[doc = "SNZREQCR (rw) register accessor: an alias for `Reg<SNZREQCR_SPEC>`"]
pub type SNZREQCR = crate::Reg<snzreqcr::SNZREQCR_SPEC>;
#[doc = "Snooze Request Control Register"]
pub mod snzreqcr;
#[doc = "OPCCR (rw) register accessor: an alias for `Reg<OPCCR_SPEC>`"]
pub type OPCCR = crate::Reg<opccr::OPCCR_SPEC>;
#[doc = "Operating Power Control Register"]
pub mod opccr;
#[doc = "SOPCCR (rw) register accessor: an alias for `Reg<SOPCCR_SPEC>`"]
pub type SOPCCR = crate::Reg<sopccr::SOPCCR_SPEC>;
#[doc = "Sub Operating Power Control Register"]
pub mod sopccr;
#[doc = "DPSBYCR (rw) register accessor: an alias for `Reg<DPSBYCR_SPEC>`"]
pub type DPSBYCR = crate::Reg<dpsbycr::DPSBYCR_SPEC>;
#[doc = "Deep Standby Control Register"]
pub mod dpsbycr;
#[doc = "DPSIER0 (rw) register accessor: an alias for `Reg<DPSIER0_SPEC>`"]
pub type DPSIER0 = crate::Reg<dpsier0::DPSIER0_SPEC>;
#[doc = "Deep Standby Interrupt Enable Register 0"]
pub mod dpsier0;
#[doc = "DPSIER1 (rw) register accessor: an alias for `Reg<DPSIER1_SPEC>`"]
pub type DPSIER1 = crate::Reg<dpsier1::DPSIER1_SPEC>;
#[doc = "Deep Standby Interrupt Enable Register 1"]
pub mod dpsier1;
#[doc = "DPSIER2 (rw) register accessor: an alias for `Reg<DPSIER2_SPEC>`"]
pub type DPSIER2 = crate::Reg<dpsier2::DPSIER2_SPEC>;
#[doc = "Deep Standby Interrupt Enable Register 2"]
pub mod dpsier2;
#[doc = "DPSIER3 (rw) register accessor: an alias for `Reg<DPSIER3_SPEC>`"]
pub type DPSIER3 = crate::Reg<dpsier3::DPSIER3_SPEC>;
#[doc = "Deep Standby Interrupt Enable Register 3"]
pub mod dpsier3;
#[doc = "DPSIFR0 (rw) register accessor: an alias for `Reg<DPSIFR0_SPEC>`"]
pub type DPSIFR0 = crate::Reg<dpsifr0::DPSIFR0_SPEC>;
#[doc = "Deep Standby Interrupt Flag Register 0"]
pub mod dpsifr0;
#[doc = "DPSIFR1 (rw) register accessor: an alias for `Reg<DPSIFR1_SPEC>`"]
pub type DPSIFR1 = crate::Reg<dpsifr1::DPSIFR1_SPEC>;
#[doc = "Deep Standby Interrupt Flag Register 1"]
pub mod dpsifr1;
#[doc = "DPSIFR2 (rw) register accessor: an alias for `Reg<DPSIFR2_SPEC>`"]
pub type DPSIFR2 = crate::Reg<dpsifr2::DPSIFR2_SPEC>;
#[doc = "Deep Standby Interrupt Flag Register 2"]
pub mod dpsifr2;
#[doc = "DPSIFR3 (rw) register accessor: an alias for `Reg<DPSIFR3_SPEC>`"]
pub type DPSIFR3 = crate::Reg<dpsifr3::DPSIFR3_SPEC>;
#[doc = "Deep Standby Interrupt Flag Register 3"]
pub mod dpsifr3;
#[doc = "DPSIEGR0 (rw) register accessor: an alias for `Reg<DPSIEGR0_SPEC>`"]
pub type DPSIEGR0 = crate::Reg<dpsiegr0::DPSIEGR0_SPEC>;
#[doc = "Deep Standby Interrupt Edge Register 0"]
pub mod dpsiegr0;
#[doc = "DPSIEGR1 (rw) register accessor: an alias for `Reg<DPSIEGR1_SPEC>`"]
pub type DPSIEGR1 = crate::Reg<dpsiegr1::DPSIEGR1_SPEC>;
#[doc = "Deep Standby Interrupt Edge Register 1"]
pub mod dpsiegr1;
#[doc = "DPSIEGR2 (rw) register accessor: an alias for `Reg<DPSIEGR2_SPEC>`"]
pub type DPSIEGR2 = crate::Reg<dpsiegr2::DPSIEGR2_SPEC>;
#[doc = "Deep Standby Interrupt Edge Register 2"]
pub mod dpsiegr2;
#[doc = "SYOCDCR (rw) register accessor: an alias for `Reg<SYOCDCR_SPEC>`"]
pub type SYOCDCR = crate::Reg<syocdcr::SYOCDCR_SPEC>;
#[doc = "System Control OCD Control Register"]
pub mod syocdcr;
#[doc = "STCONR (rw) register accessor: an alias for `Reg<STCONR_SPEC>`"]
pub type STCONR = crate::Reg<stconr::STCONR_SPEC>;
#[doc = "Standby Condition Register"]
pub mod stconr;
#[doc = "LVDCR1 (rw) register accessor: an alias for `Reg<LVDCR1_SPEC>`"]
pub type LVDCR1 = crate::Reg<lvdcr1::LVDCR1_SPEC>;
#[doc = "Voltage Monitor %s Circuit Control Register 1"]
pub mod lvdcr1;
#[doc = "LVDSR (rw) register accessor: an alias for `Reg<LVDSR_SPEC>`"]
pub type LVDSR = crate::Reg<lvdsr::LVDSR_SPEC>;
#[doc = "Voltage Monitor %s Circuit Status Register"]
pub mod lvdsr;
#[doc = "LVCMPCR (rw) register accessor: an alias for `Reg<LVCMPCR_SPEC>`"]
pub type LVCMPCR = crate::Reg<lvcmpcr::LVCMPCR_SPEC>;
#[doc = "Voltage Monitor Circuit Control Register"]
pub mod lvcmpcr;
#[doc = "LVDLVLR (rw) register accessor: an alias for `Reg<LVDLVLR_SPEC>`"]
pub type LVDLVLR = crate::Reg<lvdlvlr::LVDLVLR_SPEC>;
#[doc = "Voltage Detection Level Select Register"]
pub mod lvdlvlr;
#[doc = "LVDCR0 (rw) register accessor: an alias for `Reg<LVDCR0_SPEC>`"]
pub type LVDCR0 = crate::Reg<lvdcr0::LVDCR0_SPEC>;
#[doc = "Voltage Monitor %s Circuit Control Register 0"]
pub mod lvdcr0;
#[doc = "PRCR (rw) register accessor: an alias for `Reg<PRCR_SPEC>`"]
pub type PRCR = crate::Reg<prcr::PRCR_SPEC>;
#[doc = "Protect Register"]
pub mod prcr;
#[doc = "RSTSR0 (rw) register accessor: an alias for `Reg<RSTSR0_SPEC>`"]
pub type RSTSR0 = crate::Reg<rstsr0::RSTSR0_SPEC>;
#[doc = "Reset Status Register 0"]
pub mod rstsr0;
#[doc = "RSTSR2 (rw) register accessor: an alias for `Reg<RSTSR2_SPEC>`"]
pub type RSTSR2 = crate::Reg<rstsr2::RSTSR2_SPEC>;
#[doc = "Reset Status Register 2"]
pub mod rstsr2;
#[doc = "RSTSR1 (rw) register accessor: an alias for `Reg<RSTSR1_SPEC>`"]
pub type RSTSR1 = crate::Reg<rstsr1::RSTSR1_SPEC>;
#[doc = "Reset Status Register 1"]
pub mod rstsr1;
