#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    #[doc = "0x0c - Standby Control Register"]
    pub sbycr: SBYCR,
    _reserved1: [u8; 0x12],
    #[doc = "0x20 - System Clock Division Control Register"]
    pub sckdivcr: SCKDIVCR,
    _reserved2: [u8; 0x02],
    #[doc = "0x26 - System Clock Source Control Register"]
    pub sckscr: SCKSCR,
    _reserved3: [u8; 0x01],
    #[doc = "0x28 - PLL Clock Control Register"]
    pub pllccr: PLLCCR,
    #[doc = "0x2a - PLL Control Register"]
    pub pllcr: PLLCR,
    _reserved5: [u8; 0x07],
    #[doc = "0x32 - Main Clock Oscillator Control Register"]
    pub mosccr: MOSCCR,
    _reserved6: [u8; 0x03],
    #[doc = "0x36 - High-Speed On-Chip Oscillator Control Register"]
    pub hococr: HOCOCR,
    _reserved7: [u8; 0x01],
    #[doc = "0x38 - Middle-Speed On-Chip Oscillator Control Register"]
    pub mococr: MOCOCR,
    #[doc = "0x39 - FLL Control Register1"]
    pub fllcr1: FLLCR1,
    #[doc = "0x3a - FLL Control Register2"]
    pub fllcr2: FLLCR2,
    #[doc = "0x3c - Oscillation Stabilization Flag Register"]
    pub oscsf: OSCSF,
    _reserved11: [u8; 0x01],
    #[doc = "0x3e - Clock Out Control Register"]
    pub ckocr: CKOCR,
    #[doc = "0x3f - Trace Clock Control Register"]
    pub trckcr: TRCKCR,
    #[doc = "0x40 - Oscillation Stop Detection Control Register"]
    pub ostdcr: OSTDCR,
    #[doc = "0x41 - Oscillation Stop Detection Status Register"]
    pub ostdsr: OSTDSR,
    _reserved15: [u8; 0x06],
    #[doc = "0x48 - PLL2 Clock Control Register"]
    pub pll2ccr: PLL2CCR,
    #[doc = "0x4a - PLL2 Control Register"]
    pub pll2cr: PLL2CR,
    _reserved17: [u8; 0x16],
    #[doc = "0x61 - MOCO User Trimming Control Register"]
    pub mocoutcr: MOCOUTCR,
    #[doc = "0x62 - HOCO User Trimming Control Register"]
    pub hocoutcr: HOCOUTCR,
    _reserved19: [u8; 0x09],
    #[doc = "0x6c - USB Clock Division Control Register"]
    pub usbckdivcr: USBCKDIVCR,
    _reserved20: [u8; 0x07],
    #[doc = "0x74 - USB Clock Control Register"]
    pub usbckcr: USBCKCR,
    _reserved21: [u8; 0x13],
    #[doc = "0x88 - Snooze Request Control Register 1"]
    pub snzreqcr1: SNZREQCR1,
    _reserved22: [u8; 0x06],
    #[doc = "0x92 - Snooze Control Register"]
    pub snzcr: SNZCR,
    _reserved23: [u8; 0x01],
    #[doc = "0x94 - Snooze End Control Register 0"]
    pub snzedcr0: SNZEDCR0,
    #[doc = "0x95 - Snooze End Control Register 1"]
    pub snzedcr1: SNZEDCR1,
    _reserved25: [u8; 0x02],
    #[doc = "0x98 - Snooze Request Control Register 0"]
    pub snzreqcr0: SNZREQCR0,
    _reserved26: [u8; 0x04],
    #[doc = "0xa0 - Operating Power Control Register"]
    pub opccr: OPCCR,
    _reserved27: [u8; 0x01],
    #[doc = "0xa2 - Main Clock Oscillator Wait Control Register"]
    pub moscwtcr: MOSCWTCR,
    _reserved28: [u8; 0x07],
    #[doc = "0xaa - Sub Operating Power Control Register"]
    pub sopccr: SOPCCR,
    _reserved29: [u8; 0x15],
    #[doc = "0xc0 - Reset Status Register 1"]
    pub rstsr1: RSTSR1,
    _reserved30: [u8; 0x1e],
    #[doc = "0xe0 - Voltage Monitor 1 Circuit Control Register"]
    pub lvd1cr1: LVD1CR1,
    #[doc = "0xe1 - Voltage Monitor 1 Circuit Status Register"]
    pub lvd1sr: LVD1SR,
    #[doc = "0xe2 - Voltage Monitor 2 Circuit Control Register 1"]
    pub lvd2cr1: LVD2CR1,
    #[doc = "0xe3 - Voltage Monitor 2 Circuit Status Register"]
    pub lvd2sr: LVD2SR,
    _reserved34: [u8; 0x02dc],
    #[doc = "0x3c0 - Clock Generation Function Security Attribute Register"]
    pub cgfsar: CGFSAR,
    #[doc = "0x3c4 - Reset Security Attribution Register"]
    pub rstsar: RSTSAR,
    #[doc = "0x3c8 - Low Power Mode Security Attribution Register"]
    pub lpmsar: LPMSAR,
    #[doc = "0x3cc - Low Voltage Detection Security Attribution Register"]
    pub lvdsar: LVDSAR,
    #[doc = "0x3d0 - Battery Backup Function Security Attribute Register"]
    pub bbfsar: BBFSAR,
    _reserved39: [u8; 0x0c],
    #[doc = "0x3e0 - Deep Standby Interrupt Factor Security Attribution Register"]
    pub dpfsar: DPFSAR,
    _reserved40: [u8; 0x1a],
    #[doc = "0x3fe - Protect Register"]
    pub prcr: PRCR,
    #[doc = "0x400 - Deep Standby Control Register"]
    pub dpsbycr: DPSBYCR,
    #[doc = "0x401 - Deep Standby Wait Control Register"]
    pub dpswcr: DPSWCR,
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
    _reserved54: [u8; 0x01],
    #[doc = "0x40e - System Control OCD Control Register"]
    pub syocdcr: SYOCDCR,
    _reserved55: [u8; 0x01],
    #[doc = "0x410 - Reset Status Register 0"]
    pub rstsr0: RSTSR0,
    #[doc = "0x411 - Reset Status Register 2"]
    pub rstsr2: RSTSR2,
    _reserved57: [u8; 0x01],
    #[doc = "0x413 - Main Clock Oscillator Mode Oscillation Control Register"]
    pub momcr: MOMCR,
    _reserved58: [u8; 0x02],
    #[doc = "0x416 - Flash P/E Protect Register"]
    pub fwepror: FWEPROR,
    #[doc = "0x417 - Voltage Monitoring 1 Comparator Control Register"]
    pub lvd1cmpcr: LVD1CMPCR,
    #[doc = "0x418 - Voltage Monitoring 2 Comparator Control Register"]
    pub lvd2cmpcr: LVD2CMPCR,
    _reserved61: [u8; 0x01],
    #[doc = "0x41a - Voltage Monitor 1 Circuit Control Register 0"]
    pub lvd1cr0: LVD1CR0,
    #[doc = "0x41b - Voltage Monitor 2 Circuit Control Register 0"]
    pub lvd2cr0: LVD2CR0,
    _reserved63: [u8; 0x01],
    #[doc = "0x41d - Battery Backup Voltage Monitor Function Select Register"]
    pub vbattmnselr: VBATTMNSELR,
    #[doc = "0x41e - Battery Backup Voltage Monitor Register"]
    pub vbattmonr: VBATTMONR,
    _reserved65: [u8; 0x61],
    #[doc = "0x480 - Sub-Clock Oscillator Control Register"]
    pub sosccr: SOSCCR,
    #[doc = "0x481 - Sub-Clock Oscillator Mode Control Register"]
    pub somcr: SOMCR,
    _reserved67: [u8; 0x0e],
    #[doc = "0x490 - Low-Speed On-Chip Oscillator Control Register"]
    pub lococr: LOCOCR,
    _reserved68: [u8; 0x01],
    #[doc = "0x492 - LOCO User Trimming Control Register"]
    pub locoutcr: LOCOUTCR,
    _reserved69: [u8; 0x28],
    #[doc = "0x4bb - VBATT Input Control Register"]
    pub vbtictlr: VBTICTLR,
    _reserved70: [u8; 0x04],
    #[doc = "0x4c0 - VBATT Backup Enable Register"]
    pub vbtber: VBTBER,
    _reserved71: [u8; 0x3f],
    #[doc = "0x500..0x580 - VBATT Backup Register"]
    pub vbtbkr: [VBTBKR; 128],
}
#[doc = "SBYCR (rw) register accessor: an alias for `Reg<SBYCR_SPEC>`"]
pub type SBYCR = crate::Reg<sbycr::SBYCR_SPEC>;
#[doc = "Standby Control Register"]
pub mod sbycr;
#[doc = "SCKDIVCR (rw) register accessor: an alias for `Reg<SCKDIVCR_SPEC>`"]
pub type SCKDIVCR = crate::Reg<sckdivcr::SCKDIVCR_SPEC>;
#[doc = "System Clock Division Control Register"]
pub mod sckdivcr;
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
#[doc = "FLL Control Register1"]
pub mod fllcr1;
#[doc = "FLLCR2 (rw) register accessor: an alias for `Reg<FLLCR2_SPEC>`"]
pub type FLLCR2 = crate::Reg<fllcr2::FLLCR2_SPEC>;
#[doc = "FLL Control Register2"]
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
#[doc = "PLL2CCR (rw) register accessor: an alias for `Reg<PLL2CCR_SPEC>`"]
pub type PLL2CCR = crate::Reg<pll2ccr::PLL2CCR_SPEC>;
#[doc = "PLL2 Clock Control Register"]
pub mod pll2ccr;
#[doc = "PLL2CR (rw) register accessor: an alias for `Reg<PLL2CR_SPEC>`"]
pub type PLL2CR = crate::Reg<pll2cr::PLL2CR_SPEC>;
#[doc = "PLL2 Control Register"]
pub mod pll2cr;
#[doc = "MOCOUTCR (rw) register accessor: an alias for `Reg<MOCOUTCR_SPEC>`"]
pub type MOCOUTCR = crate::Reg<mocoutcr::MOCOUTCR_SPEC>;
#[doc = "MOCO User Trimming Control Register"]
pub mod mocoutcr;
#[doc = "HOCOUTCR (rw) register accessor: an alias for `Reg<HOCOUTCR_SPEC>`"]
pub type HOCOUTCR = crate::Reg<hocoutcr::HOCOUTCR_SPEC>;
#[doc = "HOCO User Trimming Control Register"]
pub mod hocoutcr;
#[doc = "USBCKDIVCR (rw) register accessor: an alias for `Reg<USBCKDIVCR_SPEC>`"]
pub type USBCKDIVCR = crate::Reg<usbckdivcr::USBCKDIVCR_SPEC>;
#[doc = "USB Clock Division Control Register"]
pub mod usbckdivcr;
#[doc = "USBCKCR (rw) register accessor: an alias for `Reg<USBCKCR_SPEC>`"]
pub type USBCKCR = crate::Reg<usbckcr::USBCKCR_SPEC>;
#[doc = "USB Clock Control Register"]
pub mod usbckcr;
#[doc = "SNZREQCR1 (rw) register accessor: an alias for `Reg<SNZREQCR1_SPEC>`"]
pub type SNZREQCR1 = crate::Reg<snzreqcr1::SNZREQCR1_SPEC>;
#[doc = "Snooze Request Control Register 1"]
pub mod snzreqcr1;
#[doc = "SNZCR (rw) register accessor: an alias for `Reg<SNZCR_SPEC>`"]
pub type SNZCR = crate::Reg<snzcr::SNZCR_SPEC>;
#[doc = "Snooze Control Register"]
pub mod snzcr;
#[doc = "SNZEDCR0 (rw) register accessor: an alias for `Reg<SNZEDCR0_SPEC>`"]
pub type SNZEDCR0 = crate::Reg<snzedcr0::SNZEDCR0_SPEC>;
#[doc = "Snooze End Control Register 0"]
pub mod snzedcr0;
#[doc = "SNZEDCR1 (rw) register accessor: an alias for `Reg<SNZEDCR1_SPEC>`"]
pub type SNZEDCR1 = crate::Reg<snzedcr1::SNZEDCR1_SPEC>;
#[doc = "Snooze End Control Register 1"]
pub mod snzedcr1;
#[doc = "SNZREQCR0 (rw) register accessor: an alias for `Reg<SNZREQCR0_SPEC>`"]
pub type SNZREQCR0 = crate::Reg<snzreqcr0::SNZREQCR0_SPEC>;
#[doc = "Snooze Request Control Register 0"]
pub mod snzreqcr0;
#[doc = "OPCCR (rw) register accessor: an alias for `Reg<OPCCR_SPEC>`"]
pub type OPCCR = crate::Reg<opccr::OPCCR_SPEC>;
#[doc = "Operating Power Control Register"]
pub mod opccr;
#[doc = "MOSCWTCR (rw) register accessor: an alias for `Reg<MOSCWTCR_SPEC>`"]
pub type MOSCWTCR = crate::Reg<moscwtcr::MOSCWTCR_SPEC>;
#[doc = "Main Clock Oscillator Wait Control Register"]
pub mod moscwtcr;
#[doc = "SOPCCR (rw) register accessor: an alias for `Reg<SOPCCR_SPEC>`"]
pub type SOPCCR = crate::Reg<sopccr::SOPCCR_SPEC>;
#[doc = "Sub Operating Power Control Register"]
pub mod sopccr;
#[doc = "RSTSR1 (rw) register accessor: an alias for `Reg<RSTSR1_SPEC>`"]
pub type RSTSR1 = crate::Reg<rstsr1::RSTSR1_SPEC>;
#[doc = "Reset Status Register 1"]
pub mod rstsr1;
#[doc = "LVD1CR1 (rw) register accessor: an alias for `Reg<LVD1CR1_SPEC>`"]
pub type LVD1CR1 = crate::Reg<lvd1cr1::LVD1CR1_SPEC>;
#[doc = "Voltage Monitor 1 Circuit Control Register"]
pub mod lvd1cr1;
#[doc = "LVD1SR (rw) register accessor: an alias for `Reg<LVD1SR_SPEC>`"]
pub type LVD1SR = crate::Reg<lvd1sr::LVD1SR_SPEC>;
#[doc = "Voltage Monitor 1 Circuit Status Register"]
pub mod lvd1sr;
#[doc = "LVD2CR1 (rw) register accessor: an alias for `Reg<LVD2CR1_SPEC>`"]
pub type LVD2CR1 = crate::Reg<lvd2cr1::LVD2CR1_SPEC>;
#[doc = "Voltage Monitor 2 Circuit Control Register 1"]
pub mod lvd2cr1;
#[doc = "LVD2SR (rw) register accessor: an alias for `Reg<LVD2SR_SPEC>`"]
pub type LVD2SR = crate::Reg<lvd2sr::LVD2SR_SPEC>;
#[doc = "Voltage Monitor 2 Circuit Status Register"]
pub mod lvd2sr;
#[doc = "CGFSAR (rw) register accessor: an alias for `Reg<CGFSAR_SPEC>`"]
pub type CGFSAR = crate::Reg<cgfsar::CGFSAR_SPEC>;
#[doc = "Clock Generation Function Security Attribute Register"]
pub mod cgfsar;
#[doc = "RSTSAR (rw) register accessor: an alias for `Reg<RSTSAR_SPEC>`"]
pub type RSTSAR = crate::Reg<rstsar::RSTSAR_SPEC>;
#[doc = "Reset Security Attribution Register"]
pub mod rstsar;
#[doc = "LPMSAR (rw) register accessor: an alias for `Reg<LPMSAR_SPEC>`"]
pub type LPMSAR = crate::Reg<lpmsar::LPMSAR_SPEC>;
#[doc = "Low Power Mode Security Attribution Register"]
pub mod lpmsar;
#[doc = "LVDSAR (rw) register accessor: an alias for `Reg<LVDSAR_SPEC>`"]
pub type LVDSAR = crate::Reg<lvdsar::LVDSAR_SPEC>;
#[doc = "Low Voltage Detection Security Attribution Register"]
pub mod lvdsar;
#[doc = "BBFSAR (rw) register accessor: an alias for `Reg<BBFSAR_SPEC>`"]
pub type BBFSAR = crate::Reg<bbfsar::BBFSAR_SPEC>;
#[doc = "Battery Backup Function Security Attribute Register"]
pub mod bbfsar;
#[doc = "DPFSAR (rw) register accessor: an alias for `Reg<DPFSAR_SPEC>`"]
pub type DPFSAR = crate::Reg<dpfsar::DPFSAR_SPEC>;
#[doc = "Deep Standby Interrupt Factor Security Attribution Register"]
pub mod dpfsar;
#[doc = "PRCR (rw) register accessor: an alias for `Reg<PRCR_SPEC>`"]
pub type PRCR = crate::Reg<prcr::PRCR_SPEC>;
#[doc = "Protect Register"]
pub mod prcr;
#[doc = "DPSBYCR (rw) register accessor: an alias for `Reg<DPSBYCR_SPEC>`"]
pub type DPSBYCR = crate::Reg<dpsbycr::DPSBYCR_SPEC>;
#[doc = "Deep Standby Control Register"]
pub mod dpsbycr;
#[doc = "DPSWCR (rw) register accessor: an alias for `Reg<DPSWCR_SPEC>`"]
pub type DPSWCR = crate::Reg<dpswcr::DPSWCR_SPEC>;
#[doc = "Deep Standby Wait Control Register"]
pub mod dpswcr;
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
#[doc = "RSTSR0 (rw) register accessor: an alias for `Reg<RSTSR0_SPEC>`"]
pub type RSTSR0 = crate::Reg<rstsr0::RSTSR0_SPEC>;
#[doc = "Reset Status Register 0"]
pub mod rstsr0;
#[doc = "RSTSR2 (rw) register accessor: an alias for `Reg<RSTSR2_SPEC>`"]
pub type RSTSR2 = crate::Reg<rstsr2::RSTSR2_SPEC>;
#[doc = "Reset Status Register 2"]
pub mod rstsr2;
#[doc = "MOMCR (rw) register accessor: an alias for `Reg<MOMCR_SPEC>`"]
pub type MOMCR = crate::Reg<momcr::MOMCR_SPEC>;
#[doc = "Main Clock Oscillator Mode Oscillation Control Register"]
pub mod momcr;
#[doc = "FWEPROR (rw) register accessor: an alias for `Reg<FWEPROR_SPEC>`"]
pub type FWEPROR = crate::Reg<fwepror::FWEPROR_SPEC>;
#[doc = "Flash P/E Protect Register"]
pub mod fwepror;
#[doc = "LVD1CMPCR (rw) register accessor: an alias for `Reg<LVD1CMPCR_SPEC>`"]
pub type LVD1CMPCR = crate::Reg<lvd1cmpcr::LVD1CMPCR_SPEC>;
#[doc = "Voltage Monitoring 1 Comparator Control Register"]
pub mod lvd1cmpcr;
#[doc = "LVD2CMPCR (rw) register accessor: an alias for `Reg<LVD2CMPCR_SPEC>`"]
pub type LVD2CMPCR = crate::Reg<lvd2cmpcr::LVD2CMPCR_SPEC>;
#[doc = "Voltage Monitoring 2 Comparator Control Register"]
pub mod lvd2cmpcr;
#[doc = "LVD1CR0 (rw) register accessor: an alias for `Reg<LVD1CR0_SPEC>`"]
pub type LVD1CR0 = crate::Reg<lvd1cr0::LVD1CR0_SPEC>;
#[doc = "Voltage Monitor 1 Circuit Control Register 0"]
pub mod lvd1cr0;
#[doc = "LVD2CR0 (rw) register accessor: an alias for `Reg<LVD2CR0_SPEC>`"]
pub type LVD2CR0 = crate::Reg<lvd2cr0::LVD2CR0_SPEC>;
#[doc = "Voltage Monitor 2 Circuit Control Register 0"]
pub mod lvd2cr0;
#[doc = "VBATTMNSELR (rw) register accessor: an alias for `Reg<VBATTMNSELR_SPEC>`"]
pub type VBATTMNSELR = crate::Reg<vbattmnselr::VBATTMNSELR_SPEC>;
#[doc = "Battery Backup Voltage Monitor Function Select Register"]
pub mod vbattmnselr;
#[doc = "VBATTMONR (r) register accessor: an alias for `Reg<VBATTMONR_SPEC>`"]
pub type VBATTMONR = crate::Reg<vbattmonr::VBATTMONR_SPEC>;
#[doc = "Battery Backup Voltage Monitor Register"]
pub mod vbattmonr;
#[doc = "SOSCCR (rw) register accessor: an alias for `Reg<SOSCCR_SPEC>`"]
pub type SOSCCR = crate::Reg<sosccr::SOSCCR_SPEC>;
#[doc = "Sub-Clock Oscillator Control Register"]
pub mod sosccr;
#[doc = "SOMCR (rw) register accessor: an alias for `Reg<SOMCR_SPEC>`"]
pub type SOMCR = crate::Reg<somcr::SOMCR_SPEC>;
#[doc = "Sub-Clock Oscillator Mode Control Register"]
pub mod somcr;
#[doc = "LOCOCR (rw) register accessor: an alias for `Reg<LOCOCR_SPEC>`"]
pub type LOCOCR = crate::Reg<lococr::LOCOCR_SPEC>;
#[doc = "Low-Speed On-Chip Oscillator Control Register"]
pub mod lococr;
#[doc = "LOCOUTCR (rw) register accessor: an alias for `Reg<LOCOUTCR_SPEC>`"]
pub type LOCOUTCR = crate::Reg<locoutcr::LOCOUTCR_SPEC>;
#[doc = "LOCO User Trimming Control Register"]
pub mod locoutcr;
#[doc = "VBTICTLR (rw) register accessor: an alias for `Reg<VBTICTLR_SPEC>`"]
pub type VBTICTLR = crate::Reg<vbtictlr::VBTICTLR_SPEC>;
#[doc = "VBATT Input Control Register"]
pub mod vbtictlr;
#[doc = "VBTBER (rw) register accessor: an alias for `Reg<VBTBER_SPEC>`"]
pub type VBTBER = crate::Reg<vbtber::VBTBER_SPEC>;
#[doc = "VBATT Backup Enable Register"]
pub mod vbtber;
#[doc = "VBTBKR (rw) register accessor: an alias for `Reg<VBTBKR_SPEC>`"]
pub type VBTBKR = crate::Reg<vbtbkr::VBTBKR_SPEC>;
#[doc = "VBATT Backup Register"]
pub mod vbtbkr;
