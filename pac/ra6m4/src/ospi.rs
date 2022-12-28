#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device Command Register"]
    pub dcr: DCR,
    #[doc = "0x04 - Device Address Register"]
    pub dar: DAR,
    #[doc = "0x08 - Device Command Setting Register"]
    pub dcsr: DCSR,
    #[doc = "0x0c - Device Size Register 0"]
    pub dsr0: DSR0,
    #[doc = "0x10 - Device Size Register 1"]
    pub dsr1: DSR1,
    #[doc = "0x14 - Memory Delay Trim Register"]
    pub mdtr: MDTR,
    #[doc = "0x18 - Auto-Calibration Timer Register"]
    pub actr: ACTR,
    #[doc = "0x1c - Auto-Calibration Address Register 0"]
    pub acar0: ACAR0,
    #[doc = "0x20 - Auto-Calibration Address Register 1"]
    pub acar1: ACAR1,
    _reserved9: [u8; 0x10],
    #[doc = "0x34 - Device Memory Map Read Chip Select Timing Setting Register"]
    pub drcstr: DRCSTR,
    #[doc = "0x38 - Device Memory Map Write Chip Select Timing Setting Register"]
    pub dwcstr: DWCSTR,
    #[doc = "0x3c - Device Chip Select Timing Setting Register"]
    pub dcstr: DCSTR,
    #[doc = "0x40 - Controller and Device Setting Register"]
    pub cdsr: CDSR,
    #[doc = "0x44 - Memory Map Dummy Length Register"]
    pub mdlr: MDLR,
    #[doc = "0x48 - Memory Map Read/Write Command Register 0"]
    pub mrwcr0: MRWCR0,
    #[doc = "0x4c - Memory Map Read/Write Command Register 1"]
    pub mrwcr1: MRWCR1,
    #[doc = "0x50 - Memory Map Read/Write Setting Register"]
    pub mrwcsr: MRWCSR,
    #[doc = "0x54 - Error Status Register"]
    pub esr: ESR,
    #[doc = "0x58 - Configure Write without Data Register"]
    pub cwndr: CWNDR,
    #[doc = "0x5c - Configure Write Data Register"]
    pub cwdr: CWDR,
    #[doc = "0x60 - Configure Read Register"]
    pub crr: CRR,
    #[doc = "0x64 - Auto-Calibration Status Register"]
    pub acsr: ACSR,
    _reserved22: [u8; 0x14],
    #[doc = "0x7c - Device Chip Select Maximum Period Register"]
    pub dcsmxr: DCSMXR,
    #[doc = "0x80 - Device Memory Map Write single continuous translating size Register"]
    pub dwsctsr: DWSCTSR,
}
#[doc = "DCR (rw) register accessor: an alias for `Reg<DCR_SPEC>`"]
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
#[doc = "Device Command Register"]
pub mod dcr;
#[doc = "DAR (rw) register accessor: an alias for `Reg<DAR_SPEC>`"]
pub type DAR = crate::Reg<dar::DAR_SPEC>;
#[doc = "Device Address Register"]
pub mod dar;
#[doc = "DCSR (rw) register accessor: an alias for `Reg<DCSR_SPEC>`"]
pub type DCSR = crate::Reg<dcsr::DCSR_SPEC>;
#[doc = "Device Command Setting Register"]
pub mod dcsr;
#[doc = "DSR0 (rw) register accessor: an alias for `Reg<DSR0_SPEC>`"]
pub type DSR0 = crate::Reg<dsr0::DSR0_SPEC>;
#[doc = "Device Size Register 0"]
pub mod dsr0;
#[doc = "DSR1 (rw) register accessor: an alias for `Reg<DSR1_SPEC>`"]
pub type DSR1 = crate::Reg<dsr1::DSR1_SPEC>;
#[doc = "Device Size Register 1"]
pub mod dsr1;
#[doc = "MDTR (rw) register accessor: an alias for `Reg<MDTR_SPEC>`"]
pub type MDTR = crate::Reg<mdtr::MDTR_SPEC>;
#[doc = "Memory Delay Trim Register"]
pub mod mdtr;
#[doc = "ACTR (rw) register accessor: an alias for `Reg<ACTR_SPEC>`"]
pub type ACTR = crate::Reg<actr::ACTR_SPEC>;
#[doc = "Auto-Calibration Timer Register"]
pub mod actr;
#[doc = "ACAR0 (rw) register accessor: an alias for `Reg<ACAR0_SPEC>`"]
pub type ACAR0 = crate::Reg<acar0::ACAR0_SPEC>;
#[doc = "Auto-Calibration Address Register 0"]
pub mod acar0;
#[doc = "ACAR1 (rw) register accessor: an alias for `Reg<ACAR1_SPEC>`"]
pub type ACAR1 = crate::Reg<acar1::ACAR1_SPEC>;
#[doc = "Auto-Calibration Address Register 1"]
pub mod acar1;
#[doc = "DRCSTR (rw) register accessor: an alias for `Reg<DRCSTR_SPEC>`"]
pub type DRCSTR = crate::Reg<drcstr::DRCSTR_SPEC>;
#[doc = "Device Memory Map Read Chip Select Timing Setting Register"]
pub mod drcstr;
#[doc = "DWCSTR (rw) register accessor: an alias for `Reg<DWCSTR_SPEC>`"]
pub type DWCSTR = crate::Reg<dwcstr::DWCSTR_SPEC>;
#[doc = "Device Memory Map Write Chip Select Timing Setting Register"]
pub mod dwcstr;
#[doc = "DCSTR (rw) register accessor: an alias for `Reg<DCSTR_SPEC>`"]
pub type DCSTR = crate::Reg<dcstr::DCSTR_SPEC>;
#[doc = "Device Chip Select Timing Setting Register"]
pub mod dcstr;
#[doc = "CDSR (rw) register accessor: an alias for `Reg<CDSR_SPEC>`"]
pub type CDSR = crate::Reg<cdsr::CDSR_SPEC>;
#[doc = "Controller and Device Setting Register"]
pub mod cdsr;
#[doc = "MDLR (rw) register accessor: an alias for `Reg<MDLR_SPEC>`"]
pub type MDLR = crate::Reg<mdlr::MDLR_SPEC>;
#[doc = "Memory Map Dummy Length Register"]
pub mod mdlr;
#[doc = "MRWCR0 (rw) register accessor: an alias for `Reg<MRWCR0_SPEC>`"]
pub type MRWCR0 = crate::Reg<mrwcr0::MRWCR0_SPEC>;
#[doc = "Memory Map Read/Write Command Register 0"]
pub mod mrwcr0;
#[doc = "MRWCR1 (rw) register accessor: an alias for `Reg<MRWCR1_SPEC>`"]
pub type MRWCR1 = crate::Reg<mrwcr1::MRWCR1_SPEC>;
#[doc = "Memory Map Read/Write Command Register 1"]
pub mod mrwcr1;
#[doc = "MRWCSR (rw) register accessor: an alias for `Reg<MRWCSR_SPEC>`"]
pub type MRWCSR = crate::Reg<mrwcsr::MRWCSR_SPEC>;
#[doc = "Memory Map Read/Write Setting Register"]
pub mod mrwcsr;
#[doc = "ESR (rw) register accessor: an alias for `Reg<ESR_SPEC>`"]
pub type ESR = crate::Reg<esr::ESR_SPEC>;
#[doc = "Error Status Register"]
pub mod esr;
#[doc = "CWNDR (w) register accessor: an alias for `Reg<CWNDR_SPEC>`"]
pub type CWNDR = crate::Reg<cwndr::CWNDR_SPEC>;
#[doc = "Configure Write without Data Register"]
pub mod cwndr;
#[doc = "CWDR (w) register accessor: an alias for `Reg<CWDR_SPEC>`"]
pub type CWDR = crate::Reg<cwdr::CWDR_SPEC>;
#[doc = "Configure Write Data Register"]
pub mod cwdr;
#[doc = "CRR (r) register accessor: an alias for `Reg<CRR_SPEC>`"]
pub type CRR = crate::Reg<crr::CRR_SPEC>;
#[doc = "Configure Read Register"]
pub mod crr;
#[doc = "ACSR (rw) register accessor: an alias for `Reg<ACSR_SPEC>`"]
pub type ACSR = crate::Reg<acsr::ACSR_SPEC>;
#[doc = "Auto-Calibration Status Register"]
pub mod acsr;
#[doc = "DCSMXR (rw) register accessor: an alias for `Reg<DCSMXR_SPEC>`"]
pub type DCSMXR = crate::Reg<dcsmxr::DCSMXR_SPEC>;
#[doc = "Device Chip Select Maximum Period Register"]
pub mod dcsmxr;
#[doc = "DWSCTSR (rw) register accessor: an alias for `Reg<DWSCTSR_SPEC>`"]
pub type DWSCTSR = crate::Reg<dwsctsr::DWSCTSR_SPEC>;
#[doc = "Device Memory Map Write single continuous translating size Register"]
pub mod dwsctsr;
