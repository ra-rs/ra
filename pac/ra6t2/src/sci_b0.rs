#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Receive Data Register"]
    pub rdr: RDR,
    #[doc = "0x04 - Transmit Data Register"]
    pub tdr: TDR,
    #[doc = "0x08 - Common Control Register 0"]
    pub ccr0: CCR0,
    #[doc = "0x0c - Common Control Register 1"]
    pub ccr1: CCR1,
    #[doc = "0x10 - Common Control Register 2"]
    pub ccr2: CCR2,
    #[doc = "0x14 - Common Control Register 3"]
    pub ccr3: CCR3,
    #[doc = "0x18 - Common Control Register 4"]
    pub ccr4: CCR4,
    #[doc = "0x1c - Communication Enable Status Register"]
    pub cesr: CESR,
    _reserved8: [u8; 0x03],
    #[doc = "0x20 - Simple IIC Control Register"]
    pub icr: ICR,
    #[doc = "0x24 - FIFO Control Register"]
    pub fcr: FCR,
    _reserved10: [u8; 0x04],
    #[doc = "0x2c - Manchester Control Register"]
    pub mcr: MCR,
    #[doc = "0x30 - Driver Control Register"]
    pub dcr: DCR,
    #[doc = "0x34 - Simple LIN Control Register 0"]
    pub xcr0: XCR0,
    #[doc = "0x38 - Simple LIN Control Register 1"]
    pub xcr1: XCR1,
    #[doc = "0x3c - Simple LIN Control Register 2"]
    pub xcr2: XCR2,
    _reserved15: [u8; 0x08],
    #[doc = "0x48 - Common Status Register"]
    pub csr: CSR,
    #[doc = "0x4c - Simple IIC Status Register"]
    pub isr: ISR,
    #[doc = "0x50 - FIFO Receive Status Register"]
    pub frsr: FRSR,
    #[doc = "0x54 - FIFO Transmit Status Register"]
    pub ftsr: FTSR,
    #[doc = "0x58 - Manchester Status Register"]
    pub msr: MSR,
    #[doc = "0x5c - Simple LIN Status Register 0"]
    pub xsr0: XSR0,
    #[doc = "0x60 - Simple LIN Status Register 1"]
    pub xsr1: XSR1,
    _reserved22: [u8; 0x04],
    #[doc = "0x68 - Common Flag Clear Register"]
    pub cfclr: CFCLR,
    #[doc = "0x6c - Simple IIC Flag Clear Register"]
    pub icfclr: ICFCLR,
    #[doc = "0x70 - FIFO Flag Clear Register"]
    pub ffclr: FFCLR,
    #[doc = "0x74 - Manchester Flag Clear Register"]
    pub mfclr: MFCLR,
    #[doc = "0x78 - Simple LIN Flag Clear Register"]
    pub xfclr: XFCLR,
}
#[doc = "RDR (r) register accessor: an alias for `Reg<RDR_SPEC>`"]
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
#[doc = "Receive Data Register"]
pub mod rdr;
#[doc = "TDR (rw) register accessor: an alias for `Reg<TDR_SPEC>`"]
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
#[doc = "Transmit Data Register"]
pub mod tdr;
#[doc = "CCR0 (rw) register accessor: an alias for `Reg<CCR0_SPEC>`"]
pub type CCR0 = crate::Reg<ccr0::CCR0_SPEC>;
#[doc = "Common Control Register 0"]
pub mod ccr0;
#[doc = "CCR1 (rw) register accessor: an alias for `Reg<CCR1_SPEC>`"]
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
#[doc = "Common Control Register 1"]
pub mod ccr1;
#[doc = "CCR2 (rw) register accessor: an alias for `Reg<CCR2_SPEC>`"]
pub type CCR2 = crate::Reg<ccr2::CCR2_SPEC>;
#[doc = "Common Control Register 2"]
pub mod ccr2;
#[doc = "CCR3 (rw) register accessor: an alias for `Reg<CCR3_SPEC>`"]
pub type CCR3 = crate::Reg<ccr3::CCR3_SPEC>;
#[doc = "Common Control Register 3"]
pub mod ccr3;
#[doc = "CCR4 (rw) register accessor: an alias for `Reg<CCR4_SPEC>`"]
pub type CCR4 = crate::Reg<ccr4::CCR4_SPEC>;
#[doc = "Common Control Register 4"]
pub mod ccr4;
#[doc = "CESR (r) register accessor: an alias for `Reg<CESR_SPEC>`"]
pub type CESR = crate::Reg<cesr::CESR_SPEC>;
#[doc = "Communication Enable Status Register"]
pub mod cesr;
#[doc = "ICR (rw) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Simple IIC Control Register"]
pub mod icr;
#[doc = "FCR (rw) register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "FIFO Control Register"]
pub mod fcr;
#[doc = "MCR (rw) register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Manchester Control Register"]
pub mod mcr;
#[doc = "DCR (rw) register accessor: an alias for `Reg<DCR_SPEC>`"]
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
#[doc = "Driver Control Register"]
pub mod dcr;
#[doc = "XCR0 (rw) register accessor: an alias for `Reg<XCR0_SPEC>`"]
pub type XCR0 = crate::Reg<xcr0::XCR0_SPEC>;
#[doc = "Simple LIN Control Register 0"]
pub mod xcr0;
#[doc = "XCR1 (rw) register accessor: an alias for `Reg<XCR1_SPEC>`"]
pub type XCR1 = crate::Reg<xcr1::XCR1_SPEC>;
#[doc = "Simple LIN Control Register 1"]
pub mod xcr1;
#[doc = "XCR2 (rw) register accessor: an alias for `Reg<XCR2_SPEC>`"]
pub type XCR2 = crate::Reg<xcr2::XCR2_SPEC>;
#[doc = "Simple LIN Control Register 2"]
pub mod xcr2;
#[doc = "CSR (r) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Common Status Register"]
pub mod csr;
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Simple IIC Status Register"]
pub mod isr;
#[doc = "FRSR (r) register accessor: an alias for `Reg<FRSR_SPEC>`"]
pub type FRSR = crate::Reg<frsr::FRSR_SPEC>;
#[doc = "FIFO Receive Status Register"]
pub mod frsr;
#[doc = "FTSR (r) register accessor: an alias for `Reg<FTSR_SPEC>`"]
pub type FTSR = crate::Reg<ftsr::FTSR_SPEC>;
#[doc = "FIFO Transmit Status Register"]
pub mod ftsr;
#[doc = "MSR (r) register accessor: an alias for `Reg<MSR_SPEC>`"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "Manchester Status Register"]
pub mod msr;
#[doc = "XSR0 (r) register accessor: an alias for `Reg<XSR0_SPEC>`"]
pub type XSR0 = crate::Reg<xsr0::XSR0_SPEC>;
#[doc = "Simple LIN Status Register 0"]
pub mod xsr0;
#[doc = "XSR1 (r) register accessor: an alias for `Reg<XSR1_SPEC>`"]
pub type XSR1 = crate::Reg<xsr1::XSR1_SPEC>;
#[doc = "Simple LIN Status Register 1"]
pub mod xsr1;
#[doc = "CFCLR (w) register accessor: an alias for `Reg<CFCLR_SPEC>`"]
pub type CFCLR = crate::Reg<cfclr::CFCLR_SPEC>;
#[doc = "Common Flag Clear Register"]
pub mod cfclr;
#[doc = "ICFCLR (w) register accessor: an alias for `Reg<ICFCLR_SPEC>`"]
pub type ICFCLR = crate::Reg<icfclr::ICFCLR_SPEC>;
#[doc = "Simple IIC Flag Clear Register"]
pub mod icfclr;
#[doc = "FFCLR (w) register accessor: an alias for `Reg<FFCLR_SPEC>`"]
pub type FFCLR = crate::Reg<ffclr::FFCLR_SPEC>;
#[doc = "FIFO Flag Clear Register"]
pub mod ffclr;
#[doc = "MFCLR (w) register accessor: an alias for `Reg<MFCLR_SPEC>`"]
pub type MFCLR = crate::Reg<mfclr::MFCLR_SPEC>;
#[doc = "Manchester Flag Clear Register"]
pub mod mfclr;
#[doc = "XFCLR (w) register accessor: an alias for `Reg<XFCLR_SPEC>`"]
pub type XFCLR = crate::Reg<xfclr::XFCLR_SPEC>;
#[doc = "Simple LIN Flag Clear Register"]
pub mod xfclr;
