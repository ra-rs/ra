#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_smr: [u8; 0x01],
    #[doc = "0x01 - Bit Rate Register"]
    pub brr: BRR,
    _reserved_2_scr: [u8; 0x01],
    #[doc = "0x03 - Transmit Data Register"]
    pub tdr: TDR,
    _reserved_4_ssr: [u8; 0x01],
    #[doc = "0x05 - Receive Data Register"]
    pub rdr: RDR,
    #[doc = "0x06 - Smart Card Mode Register"]
    pub scmr: SCMR,
    #[doc = "0x07 - Serial Extended Mode Register"]
    pub semr: SEMR,
    #[doc = "0x08 - Noise Filter Setting Register"]
    pub snfr: SNFR,
    #[doc = "0x09 - I2C Mode Register 1"]
    pub simr1: SIMR1,
    #[doc = "0x0a - I2C Mode Register 2"]
    pub simr2: SIMR2,
    #[doc = "0x0b - IIC Mode Register 3"]
    pub simr3: SIMR3,
    #[doc = "0x0c - IIC Status Register"]
    pub sisr: SISR,
    #[doc = "0x0d - SPI Mode Register"]
    pub spmr: SPMR,
    _reserved_14_ftdrh: [u8; 0x02],
    _reserved_15_frdrh: [u8; 0x02],
    #[doc = "0x12 - Modulation Duty Register"]
    pub mddr: MDDR,
    #[doc = "0x13 - Data Compare Match Control Register"]
    pub dccr: DCCR,
    #[doc = "0x14 - FIFO Control Register"]
    pub fcr: FCR,
    #[doc = "0x16 - FIFO Data Count Register"]
    pub fdr: FDR,
    #[doc = "0x18 - Line Status Register"]
    pub lsr: LSR,
    #[doc = "0x1a - Compare Match Data Register"]
    pub cdr: CDR,
    #[doc = "0x1c - Serial Port Register"]
    pub sptr: SPTR,
}
impl RegisterBlock {
    #[doc = "0x00 - Serial mode register (SCMR.SMIF = 1)"]
    #[inline(always)]
    pub const fn smr_smci(&self) -> &SMR_SMCI {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - Serial Mode Register (SCMR.SMIF = 0)"]
    #[inline(always)]
    pub const fn smr(&self) -> &SMR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x02 - Serial Control Register (SCMR.SMIF =1)"]
    #[inline(always)]
    pub const fn scr_smci(&self) -> &SCR_SMCI {
        unsafe { &*(self as *const Self).cast::<u8>().add(2usize).cast() }
    }
    #[doc = "0x02 - Serial Control Register (SCMR.SMIF = 0)"]
    #[inline(always)]
    pub const fn scr(&self) -> &SCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(2usize).cast() }
    }
    #[doc = "0x04 - Serial Status Register(SCMR.SMIF = 1)"]
    #[inline(always)]
    pub const fn ssr_smci(&self) -> &SSR_SMCI {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - Serial Status Register(SCMR.SMIF = 0 and FCR.FM=1)"]
    #[inline(always)]
    pub const fn ssr_fifo(&self) -> &SSR_FIFO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - Serial Status Register(SCMR.SMIF = 0 and FCR.FM=0)"]
    #[inline(always)]
    pub const fn ssr(&self) -> &SSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x0e - Transmit FIFO Data Register H"]
    #[inline(always)]
    pub const fn ftdrh(&self) -> &FTDRH {
        unsafe { &*(self as *const Self).cast::<u8>().add(14usize).cast() }
    }
    #[doc = "0x0e - Transmit FIFO Data Register HL"]
    #[inline(always)]
    pub const fn ftdrhl(&self) -> &FTDRHL {
        unsafe { &*(self as *const Self).cast::<u8>().add(14usize).cast() }
    }
    #[doc = "0x0e - Transmit 9-bit Data Register"]
    #[inline(always)]
    pub const fn tdrhl(&self) -> &TDRHL {
        unsafe { &*(self as *const Self).cast::<u8>().add(14usize).cast() }
    }
    #[doc = "0x0f - Transmit FIFO Data Register L"]
    #[inline(always)]
    pub const fn ftdrl(&self) -> &FTDRL {
        unsafe { &*(self as *const Self).cast::<u8>().add(15usize).cast() }
    }
    #[doc = "0x10 - Receive FIFO Data Register H"]
    #[inline(always)]
    pub const fn frdrh(&self) -> &FRDRH {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x10 - Receive FIFO Data Register HL"]
    #[inline(always)]
    pub const fn frdrhl(&self) -> &FRDRHL {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x10 - Receive 9-bit Data Register"]
    #[inline(always)]
    pub const fn rdrhl(&self) -> &RDRHL {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x11 - Receive FIFO Data Register L"]
    #[inline(always)]
    pub const fn frdrl(&self) -> &FRDRL {
        unsafe { &*(self as *const Self).cast::<u8>().add(17usize).cast() }
    }
}
#[doc = "SMR (rw) register accessor: an alias for `Reg<SMR_SPEC>`"]
pub type SMR = crate::Reg<smr::SMR_SPEC>;
#[doc = "Serial Mode Register (SCMR.SMIF = 0)"]
pub mod smr;
#[doc = "SMR_SMCI (rw) register accessor: an alias for `Reg<SMR_SMCI_SPEC>`"]
pub type SMR_SMCI = crate::Reg<smr_smci::SMR_SMCI_SPEC>;
#[doc = "Serial mode register (SCMR.SMIF = 1)"]
pub mod smr_smci;
#[doc = "BRR (rw) register accessor: an alias for `Reg<BRR_SPEC>`"]
pub type BRR = crate::Reg<brr::BRR_SPEC>;
#[doc = "Bit Rate Register"]
pub mod brr;
#[doc = "SCR (rw) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Serial Control Register (SCMR.SMIF = 0)"]
pub mod scr;
#[doc = "SCR_SMCI (rw) register accessor: an alias for `Reg<SCR_SMCI_SPEC>`"]
pub type SCR_SMCI = crate::Reg<scr_smci::SCR_SMCI_SPEC>;
#[doc = "Serial Control Register (SCMR.SMIF =1)"]
pub mod scr_smci;
#[doc = "TDR (rw) register accessor: an alias for `Reg<TDR_SPEC>`"]
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
#[doc = "Transmit Data Register"]
pub mod tdr;
#[doc = "SSR (rw) register accessor: an alias for `Reg<SSR_SPEC>`"]
pub type SSR = crate::Reg<ssr::SSR_SPEC>;
#[doc = "Serial Status Register(SCMR.SMIF = 0 and FCR.FM=0)"]
pub mod ssr;
#[doc = "SSR_FIFO (rw) register accessor: an alias for `Reg<SSR_FIFO_SPEC>`"]
pub type SSR_FIFO = crate::Reg<ssr_fifo::SSR_FIFO_SPEC>;
#[doc = "Serial Status Register(SCMR.SMIF = 0 and FCR.FM=1)"]
pub mod ssr_fifo;
#[doc = "SSR_SMCI (rw) register accessor: an alias for `Reg<SSR_SMCI_SPEC>`"]
pub type SSR_SMCI = crate::Reg<ssr_smci::SSR_SMCI_SPEC>;
#[doc = "Serial Status Register(SCMR.SMIF = 1)"]
pub mod ssr_smci;
#[doc = "RDR (r) register accessor: an alias for `Reg<RDR_SPEC>`"]
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
#[doc = "Receive Data Register"]
pub mod rdr;
#[doc = "SCMR (rw) register accessor: an alias for `Reg<SCMR_SPEC>`"]
pub type SCMR = crate::Reg<scmr::SCMR_SPEC>;
#[doc = "Smart Card Mode Register"]
pub mod scmr;
#[doc = "SEMR (rw) register accessor: an alias for `Reg<SEMR_SPEC>`"]
pub type SEMR = crate::Reg<semr::SEMR_SPEC>;
#[doc = "Serial Extended Mode Register"]
pub mod semr;
#[doc = "SNFR (rw) register accessor: an alias for `Reg<SNFR_SPEC>`"]
pub type SNFR = crate::Reg<snfr::SNFR_SPEC>;
#[doc = "Noise Filter Setting Register"]
pub mod snfr;
#[doc = "SIMR1 (rw) register accessor: an alias for `Reg<SIMR1_SPEC>`"]
pub type SIMR1 = crate::Reg<simr1::SIMR1_SPEC>;
#[doc = "I2C Mode Register 1"]
pub mod simr1;
#[doc = "SIMR2 (rw) register accessor: an alias for `Reg<SIMR2_SPEC>`"]
pub type SIMR2 = crate::Reg<simr2::SIMR2_SPEC>;
#[doc = "I2C Mode Register 2"]
pub mod simr2;
#[doc = "SIMR3 (rw) register accessor: an alias for `Reg<SIMR3_SPEC>`"]
pub type SIMR3 = crate::Reg<simr3::SIMR3_SPEC>;
#[doc = "IIC Mode Register 3"]
pub mod simr3;
#[doc = "SISR (r) register accessor: an alias for `Reg<SISR_SPEC>`"]
pub type SISR = crate::Reg<sisr::SISR_SPEC>;
#[doc = "IIC Status Register"]
pub mod sisr;
#[doc = "SPMR (rw) register accessor: an alias for `Reg<SPMR_SPEC>`"]
pub type SPMR = crate::Reg<spmr::SPMR_SPEC>;
#[doc = "SPI Mode Register"]
pub mod spmr;
#[doc = "TDRHL (rw) register accessor: an alias for `Reg<TDRHL_SPEC>`"]
pub type TDRHL = crate::Reg<tdrhl::TDRHL_SPEC>;
#[doc = "Transmit 9-bit Data Register"]
pub mod tdrhl;
#[doc = "FTDRHL (w) register accessor: an alias for `Reg<FTDRHL_SPEC>`"]
pub type FTDRHL = crate::Reg<ftdrhl::FTDRHL_SPEC>;
#[doc = "Transmit FIFO Data Register HL"]
pub mod ftdrhl;
#[doc = "FTDRH (w) register accessor: an alias for `Reg<FTDRH_SPEC>`"]
pub type FTDRH = crate::Reg<ftdrh::FTDRH_SPEC>;
#[doc = "Transmit FIFO Data Register H"]
pub mod ftdrh;
#[doc = "FTDRL (w) register accessor: an alias for `Reg<FTDRL_SPEC>`"]
pub type FTDRL = crate::Reg<ftdrl::FTDRL_SPEC>;
#[doc = "Transmit FIFO Data Register L"]
pub mod ftdrl;
#[doc = "RDRHL (r) register accessor: an alias for `Reg<RDRHL_SPEC>`"]
pub type RDRHL = crate::Reg<rdrhl::RDRHL_SPEC>;
#[doc = "Receive 9-bit Data Register"]
pub mod rdrhl;
#[doc = "FRDRHL (r) register accessor: an alias for `Reg<FRDRHL_SPEC>`"]
pub type FRDRHL = crate::Reg<frdrhl::FRDRHL_SPEC>;
#[doc = "Receive FIFO Data Register HL"]
pub mod frdrhl;
#[doc = "FRDRH (r) register accessor: an alias for `Reg<FRDRH_SPEC>`"]
pub type FRDRH = crate::Reg<frdrh::FRDRH_SPEC>;
#[doc = "Receive FIFO Data Register H"]
pub mod frdrh;
#[doc = "FRDRL (r) register accessor: an alias for `Reg<FRDRL_SPEC>`"]
pub type FRDRL = crate::Reg<frdrl::FRDRL_SPEC>;
#[doc = "Receive FIFO Data Register L"]
pub mod frdrl;
#[doc = "MDDR (rw) register accessor: an alias for `Reg<MDDR_SPEC>`"]
pub type MDDR = crate::Reg<mddr::MDDR_SPEC>;
#[doc = "Modulation Duty Register"]
pub mod mddr;
#[doc = "DCCR (rw) register accessor: an alias for `Reg<DCCR_SPEC>`"]
pub type DCCR = crate::Reg<dccr::DCCR_SPEC>;
#[doc = "Data Compare Match Control Register"]
pub mod dccr;
#[doc = "FCR (rw) register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "FIFO Control Register"]
pub mod fcr;
#[doc = "FDR (r) register accessor: an alias for `Reg<FDR_SPEC>`"]
pub type FDR = crate::Reg<fdr::FDR_SPEC>;
#[doc = "FIFO Data Count Register"]
pub mod fdr;
#[doc = "LSR (r) register accessor: an alias for `Reg<LSR_SPEC>`"]
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
#[doc = "Line Status Register"]
pub mod lsr;
#[doc = "CDR (rw) register accessor: an alias for `Reg<CDR_SPEC>`"]
pub type CDR = crate::Reg<cdr::CDR_SPEC>;
#[doc = "Compare Match Data Register"]
pub mod cdr;
#[doc = "SPTR (rw) register accessor: an alias for `Reg<SPTR_SPEC>`"]
pub type SPTR = crate::Reg<sptr::SPTR_SPEC>;
#[doc = "Serial Port Register"]
pub mod sptr;
