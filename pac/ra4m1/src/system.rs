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
    _reserved3: [u8; 0x02],
    #[doc = "0x26 - System Clock Source Control Register"]
    pub sckscr: SCKSCR,
    _reserved4: [u8; 0x03],
    #[doc = "0x2a - PLL Control Register"]
    pub pllcr: PLLCR,
    #[doc = "0x2b - PLL Clock Control Register2"]
    pub pllccr2: PLLCCR2,
    _reserved6: [u8; 0x05],
    #[doc = "0x31 - Memory Wait Cycle Control Register"]
    pub memwait: MEMWAIT,
    #[doc = "0x32 - Main Clock Oscillator Control Register"]
    pub mosccr: MOSCCR,
    _reserved8: [u8; 0x03],
    #[doc = "0x36 - High-Speed On-Chip Oscillator Control Register"]
    pub hococr: HOCOCR,
    _reserved9: [u8; 0x01],
    #[doc = "0x38 - Middle-Speed On-Chip Oscillator Control Register"]
    pub mococr: MOCOCR,
    _reserved10: [u8; 0x03],
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
    _reserved15: [u8; 0x0e],
    #[doc = "0x50 - Segment LCD Source Clock Control Register"]
    pub slcdsckcr: SLCDSCKCR,
    _reserved16: [u8; 0x10],
    #[doc = "0x61 - MOCO User Trimming Control Register"]
    pub mocoutcr: MOCOUTCR,
    #[doc = "0x62 - HOCO User Trimming Control Register"]
    pub hocoutcr: HOCOUTCR,
    _reserved18: [u8; 0x2f],
    #[doc = "0x92 - Snooze Control Register"]
    pub snzcr: SNZCR,
    _reserved19: [u8; 0x01],
    #[doc = "0x94 - Snooze End Control Register"]
    pub snzedcr: SNZEDCR,
    _reserved20: [u8; 0x03],
    #[doc = "0x98 - Snooze Request Control Register"]
    pub snzreqcr: SNZREQCR,
    _reserved21: [u8; 0x02],
    #[doc = "0x9e - Flash Operation Control Register"]
    pub flstop: FLSTOP,
    _reserved22: [u8; 0x01],
    #[doc = "0xa0 - Operating Power Control Register"]
    pub opccr: OPCCR,
    _reserved23: [u8; 0x01],
    #[doc = "0xa2 - Main Clock Oscillator Wait Control Register"]
    pub moscwtcr: MOSCWTCR,
    _reserved24: [u8; 0x02],
    #[doc = "0xa5 - High-Speed On-Chip Oscillator Wait Control Register"]
    pub hocowtcr: HOCOWTCR,
    _reserved25: [u8; 0x04],
    #[doc = "0xaa - Sub Operating Power Control Register"]
    pub sopccr: SOPCCR,
    _reserved26: [u8; 0x15],
    #[doc = "0xc0 - Reset Status Register 1"]
    pub rstsr1: RSTSR1,
    _reserved27: [u8; 0x04],
    #[doc = "0xc6 - Backup Register Access Control Register"]
    pub bkracr: BKRACR,
    _reserved28: [u8; 0x09],
    #[doc = "0xd0 - USB Clock Control register"]
    pub usbckcr: USBCKCR,
    _reserved29: [u8; 0x0f],
    #[doc = "0xe0 - Voltage Monitor %s Circuit Control Register 1"]
    pub lvd1cr1: LVDCR1,
    #[doc = "0xe1 - Voltage Monitor %s Circuit Status Register"]
    pub lvd1sr: LVDSR,
    #[doc = "0xe2 - Voltage Monitor %s Circuit Control Register 1"]
    pub lvd2cr1: LVDCR1,
    #[doc = "0xe3 - Voltage Monitor %s Circuit Status Register"]
    pub lvd2sr: LVDSR,
    _reserved33: [u8; 0x031a],
    #[doc = "0x3fe - Protect Register"]
    pub prcr: PRCR,
    _reserved34: [u8; 0x0e],
    #[doc = "0x40e - System Control OCD Control Register"]
    pub syocdcr: SYOCDCR,
    _reserved35: [u8; 0x01],
    #[doc = "0x410 - Reset Status Register 0"]
    pub rstsr0: RSTSR0,
    #[doc = "0x411 - Reset Status Register 2"]
    pub rstsr2: RSTSR2,
    _reserved37: [u8; 0x01],
    #[doc = "0x413 - Main Clock Oscillator Mode Oscillation Control Register"]
    pub momcr: MOMCR,
    _reserved38: [u8; 0x03],
    #[doc = "0x417 - Voltage Monitor Circuit Control Register"]
    pub lvcmpcr: LVCMPCR,
    #[doc = "0x418 - Voltage Detection Level Select Register"]
    pub lvdlvlr: LVDLVLR,
    _reserved40: [u8; 0x01],
    #[doc = "0x41a - Voltage Monitor %s Circuit Control Register 0"]
    pub lvdcr0: [LVDCR0; 2],
    _reserved41: [u8; 0x03],
    #[doc = "0x41f - VBATT Control Register1"]
    pub vbtcr1: VBTCR1,
    _reserved42: [u8; 0x60],
    #[doc = "0x480 - Sub-Clock Oscillator Control Register"]
    pub sosccr: SOSCCR,
    #[doc = "0x481 - Sub Clock Oscillator Mode Control Register"]
    pub somcr: SOMCR,
    _reserved44: [u8; 0x0e],
    #[doc = "0x490 - Low-Speed On-Chip Oscillator Control Register"]
    pub lococr: LOCOCR,
    _reserved45: [u8; 0x01],
    #[doc = "0x492 - LOCO User Trimming Control Register"]
    pub locoutcr: LOCOUTCR,
    _reserved46: [u8; 0x1d],
    #[doc = "0x4b0 - VBATT Control Register2"]
    pub vbtcr2: VBTCR2,
    #[doc = "0x4b1 - VBATT Status Register"]
    pub vbtsr: VBTSR,
    #[doc = "0x4b2 - VBATT Comparator Control Register"]
    pub vbtcmpcr: VBTCMPCR,
    _reserved49: [u8; 0x01],
    #[doc = "0x4b4 - VBATT Pin Low Voltage Detect Interrupt Control Register"]
    pub vbtlvdicr: VBTLVDICR,
    _reserved50: [u8; 0x01],
    #[doc = "0x4b6 - VBATT Wakeup function Control Register"]
    pub vbtwctlr: VBTWCTLR,
    _reserved51: [u8; 0x01],
    #[doc = "0x4b8 - VBATT Wakeup I/O 0 Output Trigger Select Register"]
    pub vbtwch0otsr: VBTWCH0OTSR,
    #[doc = "0x4b9 - VBATT Wakeup I/O 1 Output Trigger Select Register"]
    pub vbtwch1otsr: VBTWCH1OTSR,
    #[doc = "0x4ba - VBATT Wakeup I/O 2 Output Trigger Select Register"]
    pub vbtwch2otsr: VBTWCH2OTSR,
    #[doc = "0x4bb - VBATT Input Control Register"]
    pub vbtictlr: VBTICTLR,
    #[doc = "0x4bc - VBATT Output Control Register"]
    pub vbtoctlr: VBTOCTLR,
    #[doc = "0x4bd - VBATT Wakeup Trigger source Enable Register"]
    pub vbtwter: VBTWTER,
    #[doc = "0x4be - VBATT Wakeup Trigger source Edge Register"]
    pub vbtwegr: VBTWEGR,
    #[doc = "0x4bf - VBATT Wakeup trigger source Flag Register"]
    pub vbtwfr: VBTWFR,
    _reserved59: [u8; 0x40],
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
#[doc = "VBTCR1 (rw) register accessor: an alias for `Reg<VBTCR1_SPEC>`"]
pub type VBTCR1 = crate::Reg<vbtcr1::VBTCR1_SPEC>;
#[doc = "VBATT Control Register1"]
pub mod vbtcr1;
#[doc = "VBTCR2 (rw) register accessor: an alias for `Reg<VBTCR2_SPEC>`"]
pub type VBTCR2 = crate::Reg<vbtcr2::VBTCR2_SPEC>;
#[doc = "VBATT Control Register2"]
pub mod vbtcr2;
#[doc = "VBTSR (rw) register accessor: an alias for `Reg<VBTSR_SPEC>`"]
pub type VBTSR = crate::Reg<vbtsr::VBTSR_SPEC>;
#[doc = "VBATT Status Register"]
pub mod vbtsr;
#[doc = "VBTCMPCR (rw) register accessor: an alias for `Reg<VBTCMPCR_SPEC>`"]
pub type VBTCMPCR = crate::Reg<vbtcmpcr::VBTCMPCR_SPEC>;
#[doc = "VBATT Comparator Control Register"]
pub mod vbtcmpcr;
#[doc = "VBTLVDICR (rw) register accessor: an alias for `Reg<VBTLVDICR_SPEC>`"]
pub type VBTLVDICR = crate::Reg<vbtlvdicr::VBTLVDICR_SPEC>;
#[doc = "VBATT Pin Low Voltage Detect Interrupt Control Register"]
pub mod vbtlvdicr;
#[doc = "VBTWCTLR (rw) register accessor: an alias for `Reg<VBTWCTLR_SPEC>`"]
pub type VBTWCTLR = crate::Reg<vbtwctlr::VBTWCTLR_SPEC>;
#[doc = "VBATT Wakeup function Control Register"]
pub mod vbtwctlr;
#[doc = "VBTWCH0OTSR (rw) register accessor: an alias for `Reg<VBTWCH0OTSR_SPEC>`"]
pub type VBTWCH0OTSR = crate::Reg<vbtwch0otsr::VBTWCH0OTSR_SPEC>;
#[doc = "VBATT Wakeup I/O 0 Output Trigger Select Register"]
pub mod vbtwch0otsr;
#[doc = "VBTWCH1OTSR (rw) register accessor: an alias for `Reg<VBTWCH1OTSR_SPEC>`"]
pub type VBTWCH1OTSR = crate::Reg<vbtwch1otsr::VBTWCH1OTSR_SPEC>;
#[doc = "VBATT Wakeup I/O 1 Output Trigger Select Register"]
pub mod vbtwch1otsr;
#[doc = "VBTWCH2OTSR (rw) register accessor: an alias for `Reg<VBTWCH2OTSR_SPEC>`"]
pub type VBTWCH2OTSR = crate::Reg<vbtwch2otsr::VBTWCH2OTSR_SPEC>;
#[doc = "VBATT Wakeup I/O 2 Output Trigger Select Register"]
pub mod vbtwch2otsr;
#[doc = "VBTICTLR (rw) register accessor: an alias for `Reg<VBTICTLR_SPEC>`"]
pub type VBTICTLR = crate::Reg<vbtictlr::VBTICTLR_SPEC>;
#[doc = "VBATT Input Control Register"]
pub mod vbtictlr;
#[doc = "VBTOCTLR (rw) register accessor: an alias for `Reg<VBTOCTLR_SPEC>`"]
pub type VBTOCTLR = crate::Reg<vbtoctlr::VBTOCTLR_SPEC>;
#[doc = "VBATT Output Control Register"]
pub mod vbtoctlr;
#[doc = "VBTWTER (rw) register accessor: an alias for `Reg<VBTWTER_SPEC>`"]
pub type VBTWTER = crate::Reg<vbtwter::VBTWTER_SPEC>;
#[doc = "VBATT Wakeup Trigger source Enable Register"]
pub mod vbtwter;
#[doc = "VBTWEGR (rw) register accessor: an alias for `Reg<VBTWEGR_SPEC>`"]
pub type VBTWEGR = crate::Reg<vbtwegr::VBTWEGR_SPEC>;
#[doc = "VBATT Wakeup Trigger source Edge Register"]
pub mod vbtwegr;
#[doc = "VBTWFR (rw) register accessor: an alias for `Reg<VBTWFR_SPEC>`"]
pub type VBTWFR = crate::Reg<vbtwfr::VBTWFR_SPEC>;
#[doc = "VBATT Wakeup trigger source Flag Register"]
pub mod vbtwfr;
#[doc = "VBTBKR (rw) register accessor: an alias for `Reg<VBTBKR_SPEC>`"]
pub type VBTBKR = crate::Reg<vbtbkr::VBTBKR_SPEC>;
#[doc = "VBATT Backup Register \\[%s\\]"]
pub mod vbtbkr;
#[doc = "SCKDIVCR (rw) register accessor: an alias for `Reg<SCKDIVCR_SPEC>`"]
pub type SCKDIVCR = crate::Reg<sckdivcr::SCKDIVCR_SPEC>;
#[doc = "System Clock Division Control Register"]
pub mod sckdivcr;
#[doc = "SCKSCR (rw) register accessor: an alias for `Reg<SCKSCR_SPEC>`"]
pub type SCKSCR = crate::Reg<sckscr::SCKSCR_SPEC>;
#[doc = "System Clock Source Control Register"]
pub mod sckscr;
#[doc = "PLLCR (rw) register accessor: an alias for `Reg<PLLCR_SPEC>`"]
pub type PLLCR = crate::Reg<pllcr::PLLCR_SPEC>;
#[doc = "PLL Control Register"]
pub mod pllcr;
#[doc = "PLLCCR2 (rw) register accessor: an alias for `Reg<PLLCCR2_SPEC>`"]
pub type PLLCCR2 = crate::Reg<pllccr2::PLLCCR2_SPEC>;
#[doc = "PLL Clock Control Register2"]
pub mod pllccr2;
#[doc = "MEMWAIT (rw) register accessor: an alias for `Reg<MEMWAIT_SPEC>`"]
pub type MEMWAIT = crate::Reg<memwait::MEMWAIT_SPEC>;
#[doc = "Memory Wait Cycle Control Register"]
pub mod memwait;
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
#[doc = "SLCDSCKCR (rw) register accessor: an alias for `Reg<SLCDSCKCR_SPEC>`"]
pub type SLCDSCKCR = crate::Reg<slcdsckcr::SLCDSCKCR_SPEC>;
#[doc = "Segment LCD Source Clock Control Register"]
pub mod slcdsckcr;
#[doc = "MOCOUTCR (rw) register accessor: an alias for `Reg<MOCOUTCR_SPEC>`"]
pub type MOCOUTCR = crate::Reg<mocoutcr::MOCOUTCR_SPEC>;
#[doc = "MOCO User Trimming Control Register"]
pub mod mocoutcr;
#[doc = "HOCOUTCR (rw) register accessor: an alias for `Reg<HOCOUTCR_SPEC>`"]
pub type HOCOUTCR = crate::Reg<hocoutcr::HOCOUTCR_SPEC>;
#[doc = "HOCO User Trimming Control Register"]
pub mod hocoutcr;
#[doc = "MOSCWTCR (rw) register accessor: an alias for `Reg<MOSCWTCR_SPEC>`"]
pub type MOSCWTCR = crate::Reg<moscwtcr::MOSCWTCR_SPEC>;
#[doc = "Main Clock Oscillator Wait Control Register"]
pub mod moscwtcr;
#[doc = "HOCOWTCR (rw) register accessor: an alias for `Reg<HOCOWTCR_SPEC>`"]
pub type HOCOWTCR = crate::Reg<hocowtcr::HOCOWTCR_SPEC>;
#[doc = "High-Speed On-Chip Oscillator Wait Control Register"]
pub mod hocowtcr;
#[doc = "USBCKCR (rw) register accessor: an alias for `Reg<USBCKCR_SPEC>`"]
pub type USBCKCR = crate::Reg<usbckcr::USBCKCR_SPEC>;
#[doc = "USB Clock Control register"]
pub mod usbckcr;
#[doc = "MOMCR (rw) register accessor: an alias for `Reg<MOMCR_SPEC>`"]
pub type MOMCR = crate::Reg<momcr::MOMCR_SPEC>;
#[doc = "Main Clock Oscillator Mode Oscillation Control Register"]
pub mod momcr;
#[doc = "SOSCCR (rw) register accessor: an alias for `Reg<SOSCCR_SPEC>`"]
pub type SOSCCR = crate::Reg<sosccr::SOSCCR_SPEC>;
#[doc = "Sub-Clock Oscillator Control Register"]
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
#[doc = "FLSTOP (rw) register accessor: an alias for `Reg<FLSTOP_SPEC>`"]
pub type FLSTOP = crate::Reg<flstop::FLSTOP_SPEC>;
#[doc = "Flash Operation Control Register"]
pub mod flstop;
#[doc = "OPCCR (rw) register accessor: an alias for `Reg<OPCCR_SPEC>`"]
pub type OPCCR = crate::Reg<opccr::OPCCR_SPEC>;
#[doc = "Operating Power Control Register"]
pub mod opccr;
#[doc = "SOPCCR (rw) register accessor: an alias for `Reg<SOPCCR_SPEC>`"]
pub type SOPCCR = crate::Reg<sopccr::SOPCCR_SPEC>;
#[doc = "Sub Operating Power Control Register"]
pub mod sopccr;
#[doc = "SYOCDCR (rw) register accessor: an alias for `Reg<SYOCDCR_SPEC>`"]
pub type SYOCDCR = crate::Reg<syocdcr::SYOCDCR_SPEC>;
#[doc = "System Control OCD Control Register"]
pub mod syocdcr;
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
#[doc = "LVDCR1 (rw) register accessor: an alias for `Reg<LVDCR1_SPEC>`"]
pub type LVDCR1 = crate::Reg<lvdcr1::LVDCR1_SPEC>;
#[doc = "Voltage Monitor %s Circuit Control Register 1"]
pub mod lvdcr1;
#[doc = "LVDSR (rw) register accessor: an alias for `Reg<LVDSR_SPEC>`"]
pub type LVDSR = crate::Reg<lvdsr::LVDSR_SPEC>;
#[doc = "Voltage Monitor %s Circuit Status Register"]
pub mod lvdsr;
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
#[doc = "BKRACR (rw) register accessor: an alias for `Reg<BKRACR_SPEC>`"]
pub type BKRACR = crate::Reg<bkracr::BKRACR_SPEC>;
#[doc = "Backup Register Access Control Register"]
pub mod bkracr;
