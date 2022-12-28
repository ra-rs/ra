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
    #[doc = "0x09 - IIC Mode Register 1"]
    pub simr1: SIMR1,
    #[doc = "0x0a - IIC Mode Register 2"]
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
    #[doc = "0x1d - Adjustment Communication Timing Register"]
    pub actr: ACTR,
    _reserved24: [u8; 0x02],
    _reserved_24_mmr: [u8; 0x01],
    #[doc = "0x21 - Control Register 0"]
    pub cr0: CR0,
    _reserved_26_cr1: [u8; 0x01],
    _reserved_27_cr2: [u8; 0x01],
    _reserved_28_cr3: [u8; 0x01],
    _reserved_29_pcr: [u8; 0x01],
    #[doc = "0x26 - Interrupt Control Register"]
    pub icr: ICR,
    #[doc = "0x27 - Status Register"]
    pub str: STR,
    #[doc = "0x28 - Status Clear Register"]
    pub stcr: STCR,
    #[doc = "0x29 - Control Field 0 Data Register"]
    pub cf0dr: CF0DR,
    #[doc = "0x2a - Control Field 0 Compare Enable Register"]
    pub cf0cr: CF0CR,
    #[doc = "0x2b - Control Field 0 Receive Data Register"]
    pub cf0rr: CF0RR,
    #[doc = "0x2c - Primary Control Field 1 Data Register"]
    pub pcf1dr: PCF1DR,
    #[doc = "0x2d - Secondary Control Field 1 Data Register"]
    pub scf1dr: SCF1DR,
    #[doc = "0x2e - Control Field 1 Compare Enable Register"]
    pub cf1cr: CF1CR,
    #[doc = "0x2f - Control Field 1 Receive Data Register"]
    pub cf1rr: CF1RR,
    #[doc = "0x30 - Timer Control Register"]
    pub tcr: TCR,
    #[doc = "0x31 - Timer Mode Register"]
    pub tmr: TMR,
    #[doc = "0x32 - Timer Prescaler Register"]
    pub tpre: TPRE,
    #[doc = "0x33 - Timer Count Register"]
    pub tcnt: TCNT,
}
impl RegisterBlock {
    #[doc = "0x00 - Serial Mode Register for Smart Card Interface Mode (SCMR.SMIF = 1)"]
    #[inline(always)]
    pub const fn smr_smci(&self) -> &SMR_SMCI {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - Serial Mode Register for Non-Smart Card Interface Mode (SCMR.SMIF = 0)"]
    #[inline(always)]
    pub const fn smr(&self) -> &SMR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x02 - Serial Control Register for Smart Card Interface Mode (SCMR.SMIF = 1)"]
    #[inline(always)]
    pub const fn scr_smci(&self) -> &SCR_SMCI {
        unsafe { &*(self as *const Self).cast::<u8>().add(2usize).cast() }
    }
    #[doc = "0x02 - Serial Control Register for Non-Smart Card Interface Mode (SCMR.SMIF = 0)"]
    #[inline(always)]
    pub const fn scr(&self) -> &SCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(2usize).cast() }
    }
    #[doc = "0x04 - Serial Status Register for Smart Card Interface Mode (SCMR.SMIF = 1, and MMR.MANEN = 0)"]
    #[inline(always)]
    pub const fn ssr_smci(&self) -> &SSR_SMCI {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - Serial Status Register for Manchester Mode (SCMR.SMIF = 0, and MMR.MANEN = 1)"]
    #[inline(always)]
    pub const fn ssr_manc(&self) -> &SSR_MANC {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - Serial Status Register for Non-Smart Card Interface and FIFO Mode (SCMR.SMIF = 0, FCR.FM = 1, and MMR.MANEN = 0)"]
    #[inline(always)]
    pub const fn ssr_fifo(&self) -> &SSR_FIFO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - Serial Status Register for Non-Smart Card Interface and Non-FIFO Mode (SCMR.SMIF = 0, FCR.FM = 0, and MMR.MANEN = 0)"]
    #[inline(always)]
    pub const fn ssr(&self) -> &SSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x0e - Transmit FIFO Data Register"]
    #[inline(always)]
    pub const fn ftdrh(&self) -> &FTDRH {
        unsafe { &*(self as *const Self).cast::<u8>().add(14usize).cast() }
    }
    #[doc = "0x0e - Transmit Data Register for Non-Manchester mode (MMR.MANEN = 0)"]
    #[inline(always)]
    pub const fn tdrhl(&self) -> &TDRHL {
        unsafe { &*(self as *const Self).cast::<u8>().add(14usize).cast() }
    }
    #[doc = "0x0e - Transmit FIFO Data Register"]
    #[inline(always)]
    pub const fn ftdrhl(&self) -> &FTDRHL {
        unsafe { &*(self as *const Self).cast::<u8>().add(14usize).cast() }
    }
    #[doc = "0x0f - Transmit FIFO Data Register"]
    #[inline(always)]
    pub const fn ftdrl(&self) -> &FTDRL {
        unsafe { &*(self as *const Self).cast::<u8>().add(15usize).cast() }
    }
    #[doc = "0x10 - Receive FIFO Data Register"]
    #[inline(always)]
    pub const fn frdrh(&self) -> &FRDRH {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x10 - Receive Data Register for Non-Manchester mode (MMR.MANEN = 0)"]
    #[inline(always)]
    pub const fn rdrhl(&self) -> &RDRHL {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x10 - Receive FIFO Data Register"]
    #[inline(always)]
    pub const fn frdrhl(&self) -> &FRDRHL {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x11 - Receive FIFO Data Register"]
    #[inline(always)]
    pub const fn frdrl(&self) -> &FRDRL {
        unsafe { &*(self as *const Self).cast::<u8>().add(17usize).cast() }
    }
    #[doc = "0x20 - Manchester Mode Register"]
    #[inline(always)]
    pub const fn mmr(&self) -> &MMR {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    #[doc = "0x20 - Extended Serial Module Enable Register"]
    #[inline(always)]
    pub const fn esmer(&self) -> &ESMER {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    #[doc = "0x22 - Transmit Manchester Preface Setting Register"]
    #[inline(always)]
    pub const fn tmpr(&self) -> &TMPR {
        unsafe { &*(self as *const Self).cast::<u8>().add(34usize).cast() }
    }
    #[doc = "0x22 - Control Register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(34usize).cast() }
    }
    #[doc = "0x23 - Receive Manchester Preface Setting Register"]
    #[inline(always)]
    pub const fn rmpr(&self) -> &RMPR {
        unsafe { &*(self as *const Self).cast::<u8>().add(35usize).cast() }
    }
    #[doc = "0x23 - Control Register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(35usize).cast() }
    }
    #[doc = "0x24 - Manchester Extended Error Status Register"]
    #[inline(always)]
    pub const fn mesr(&self) -> &MESR {
        unsafe { &*(self as *const Self).cast::<u8>().add(36usize).cast() }
    }
    #[doc = "0x24 - Control Register 3"]
    #[inline(always)]
    pub const fn cr3(&self) -> &CR3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(36usize).cast() }
    }
    #[doc = "0x25 - Port Control Register"]
    #[inline(always)]
    pub const fn pcr(&self) -> &PCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(37usize).cast() }
    }
    #[doc = "0x25 - Manchester Extended Error Control Register"]
    #[inline(always)]
    pub const fn mecr(&self) -> &MECR {
        unsafe { &*(self as *const Self).cast::<u8>().add(37usize).cast() }
    }
}
#[doc = "SMR (rw) register accessor: an alias for `Reg<SMR_SPEC>`"]
pub type SMR = crate::Reg<smr::SMR_SPEC>;
#[doc = "Serial Mode Register for Non-Smart Card Interface Mode (SCMR.SMIF = 0)"]
pub mod smr;
#[doc = "SMR_SMCI (rw) register accessor: an alias for `Reg<SMR_SMCI_SPEC>`"]
pub type SMR_SMCI = crate::Reg<smr_smci::SMR_SMCI_SPEC>;
#[doc = "Serial Mode Register for Smart Card Interface Mode (SCMR.SMIF = 1)"]
pub mod smr_smci;
#[doc = "BRR (rw) register accessor: an alias for `Reg<BRR_SPEC>`"]
pub type BRR = crate::Reg<brr::BRR_SPEC>;
#[doc = "Bit Rate Register"]
pub mod brr;
#[doc = "SCR (rw) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Serial Control Register for Non-Smart Card Interface Mode (SCMR.SMIF = 0)"]
pub mod scr;
#[doc = "SCR_SMCI (rw) register accessor: an alias for `Reg<SCR_SMCI_SPEC>`"]
pub type SCR_SMCI = crate::Reg<scr_smci::SCR_SMCI_SPEC>;
#[doc = "Serial Control Register for Smart Card Interface Mode (SCMR.SMIF = 1)"]
pub mod scr_smci;
#[doc = "TDR (rw) register accessor: an alias for `Reg<TDR_SPEC>`"]
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
#[doc = "Transmit Data Register"]
pub mod tdr;
#[doc = "SSR (rw) register accessor: an alias for `Reg<SSR_SPEC>`"]
pub type SSR = crate::Reg<ssr::SSR_SPEC>;
#[doc = "Serial Status Register for Non-Smart Card Interface and Non-FIFO Mode (SCMR.SMIF = 0, FCR.FM = 0, and MMR.MANEN = 0)"]
pub mod ssr;
#[doc = "SSR_FIFO (rw) register accessor: an alias for `Reg<SSR_FIFO_SPEC>`"]
pub type SSR_FIFO = crate::Reg<ssr_fifo::SSR_FIFO_SPEC>;
#[doc = "Serial Status Register for Non-Smart Card Interface and FIFO Mode (SCMR.SMIF = 0, FCR.FM = 1, and MMR.MANEN = 0)"]
pub mod ssr_fifo;
#[doc = "SSR_MANC (rw) register accessor: an alias for `Reg<SSR_MANC_SPEC>`"]
pub type SSR_MANC = crate::Reg<ssr_manc::SSR_MANC_SPEC>;
#[doc = "Serial Status Register for Manchester Mode (SCMR.SMIF = 0, and MMR.MANEN = 1)"]
pub mod ssr_manc;
#[doc = "SSR_SMCI (rw) register accessor: an alias for `Reg<SSR_SMCI_SPEC>`"]
pub type SSR_SMCI = crate::Reg<ssr_smci::SSR_SMCI_SPEC>;
#[doc = "Serial Status Register for Smart Card Interface Mode (SCMR.SMIF = 1, and MMR.MANEN = 0)"]
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
#[doc = "IIC Mode Register 1"]
pub mod simr1;
#[doc = "SIMR2 (rw) register accessor: an alias for `Reg<SIMR2_SPEC>`"]
pub type SIMR2 = crate::Reg<simr2::SIMR2_SPEC>;
#[doc = "IIC Mode Register 2"]
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
#[doc = "FTDRHL (w) register accessor: an alias for `Reg<FTDRHL_SPEC>`"]
pub type FTDRHL = crate::Reg<ftdrhl::FTDRHL_SPEC>;
#[doc = "Transmit FIFO Data Register"]
pub mod ftdrhl;
#[doc = "TDRHL (rw) register accessor: an alias for `Reg<TDRHL_SPEC>`"]
pub type TDRHL = crate::Reg<tdrhl::TDRHL_SPEC>;
#[doc = "Transmit Data Register for Non-Manchester mode (MMR.MANEN = 0)"]
pub mod tdrhl;
#[doc = "FTDRH (w) register accessor: an alias for `Reg<FTDRH_SPEC>`"]
pub type FTDRH = crate::Reg<ftdrh::FTDRH_SPEC>;
#[doc = "Transmit FIFO Data Register"]
pub mod ftdrh;
#[doc = "FTDRL (w) register accessor: an alias for `Reg<FTDRL_SPEC>`"]
pub type FTDRL = crate::Reg<ftdrl::FTDRL_SPEC>;
#[doc = "Transmit FIFO Data Register"]
pub mod ftdrl;
#[doc = "FRDRHL (r) register accessor: an alias for `Reg<FRDRHL_SPEC>`"]
pub type FRDRHL = crate::Reg<frdrhl::FRDRHL_SPEC>;
#[doc = "Receive FIFO Data Register"]
pub mod frdrhl;
#[doc = "RDRHL (r) register accessor: an alias for `Reg<RDRHL_SPEC>`"]
pub type RDRHL = crate::Reg<rdrhl::RDRHL_SPEC>;
#[doc = "Receive Data Register for Non-Manchester mode (MMR.MANEN = 0)"]
pub mod rdrhl;
#[doc = "FRDRH (r) register accessor: an alias for `Reg<FRDRH_SPEC>`"]
pub type FRDRH = crate::Reg<frdrh::FRDRH_SPEC>;
#[doc = "Receive FIFO Data Register"]
pub mod frdrh;
#[doc = "FRDRL (r) register accessor: an alias for `Reg<FRDRL_SPEC>`"]
pub type FRDRL = crate::Reg<frdrl::FRDRL_SPEC>;
#[doc = "Receive FIFO Data Register"]
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
#[doc = "ACTR (rw) register accessor: an alias for `Reg<ACTR_SPEC>`"]
pub type ACTR = crate::Reg<actr::ACTR_SPEC>;
#[doc = "Adjustment Communication Timing Register"]
pub mod actr;
#[doc = "ESMER (rw) register accessor: an alias for `Reg<ESMER_SPEC>`"]
pub type ESMER = crate::Reg<esmer::ESMER_SPEC>;
#[doc = "Extended Serial Module Enable Register"]
pub mod esmer;
#[doc = "MMR (rw) register accessor: an alias for `Reg<MMR_SPEC>`"]
pub type MMR = crate::Reg<mmr::MMR_SPEC>;
#[doc = "Manchester Mode Register"]
pub mod mmr;
#[doc = "CR0 (rw) register accessor: an alias for `Reg<CR0_SPEC>`"]
pub type CR0 = crate::Reg<cr0::CR0_SPEC>;
#[doc = "Control Register 0"]
pub mod cr0;
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "Control Register 1"]
pub mod cr1;
#[doc = "TMPR (rw) register accessor: an alias for `Reg<TMPR_SPEC>`"]
pub type TMPR = crate::Reg<tmpr::TMPR_SPEC>;
#[doc = "Transmit Manchester Preface Setting Register"]
pub mod tmpr;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "Control Register 2"]
pub mod cr2;
#[doc = "RMPR (rw) register accessor: an alias for `Reg<RMPR_SPEC>`"]
pub type RMPR = crate::Reg<rmpr::RMPR_SPEC>;
#[doc = "Receive Manchester Preface Setting Register"]
pub mod rmpr;
#[doc = "CR3 (rw) register accessor: an alias for `Reg<CR3_SPEC>`"]
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
#[doc = "Control Register 3"]
pub mod cr3;
#[doc = "MESR (rw) register accessor: an alias for `Reg<MESR_SPEC>`"]
pub type MESR = crate::Reg<mesr::MESR_SPEC>;
#[doc = "Manchester Extended Error Status Register"]
pub mod mesr;
#[doc = "MECR (rw) register accessor: an alias for `Reg<MECR_SPEC>`"]
pub type MECR = crate::Reg<mecr::MECR_SPEC>;
#[doc = "Manchester Extended Error Control Register"]
pub mod mecr;
#[doc = "PCR (rw) register accessor: an alias for `Reg<PCR_SPEC>`"]
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
#[doc = "Port Control Register"]
pub mod pcr;
#[doc = "ICR (rw) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt Control Register"]
pub mod icr;
#[doc = "STR (r) register accessor: an alias for `Reg<STR_SPEC>`"]
pub type STR = crate::Reg<str::STR_SPEC>;
#[doc = "Status Register"]
pub mod str;
#[doc = "STCR (rw) register accessor: an alias for `Reg<STCR_SPEC>`"]
pub type STCR = crate::Reg<stcr::STCR_SPEC>;
#[doc = "Status Clear Register"]
pub mod stcr;
#[doc = "CF0DR (rw) register accessor: an alias for `Reg<CF0DR_SPEC>`"]
pub type CF0DR = crate::Reg<cf0dr::CF0DR_SPEC>;
#[doc = "Control Field 0 Data Register"]
pub mod cf0dr;
#[doc = "CF0CR (rw) register accessor: an alias for `Reg<CF0CR_SPEC>`"]
pub type CF0CR = crate::Reg<cf0cr::CF0CR_SPEC>;
#[doc = "Control Field 0 Compare Enable Register"]
pub mod cf0cr;
#[doc = "CF0RR (rw) register accessor: an alias for `Reg<CF0RR_SPEC>`"]
pub type CF0RR = crate::Reg<cf0rr::CF0RR_SPEC>;
#[doc = "Control Field 0 Receive Data Register"]
pub mod cf0rr;
#[doc = "PCF1DR (rw) register accessor: an alias for `Reg<PCF1DR_SPEC>`"]
pub type PCF1DR = crate::Reg<pcf1dr::PCF1DR_SPEC>;
#[doc = "Primary Control Field 1 Data Register"]
pub mod pcf1dr;
#[doc = "SCF1DR (rw) register accessor: an alias for `Reg<SCF1DR_SPEC>`"]
pub type SCF1DR = crate::Reg<scf1dr::SCF1DR_SPEC>;
#[doc = "Secondary Control Field 1 Data Register"]
pub mod scf1dr;
#[doc = "CF1CR (rw) register accessor: an alias for `Reg<CF1CR_SPEC>`"]
pub type CF1CR = crate::Reg<cf1cr::CF1CR_SPEC>;
#[doc = "Control Field 1 Compare Enable Register"]
pub mod cf1cr;
#[doc = "CF1RR (rw) register accessor: an alias for `Reg<CF1RR_SPEC>`"]
pub type CF1RR = crate::Reg<cf1rr::CF1RR_SPEC>;
#[doc = "Control Field 1 Receive Data Register"]
pub mod cf1rr;
#[doc = "TCR (rw) register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "Timer Control Register"]
pub mod tcr;
#[doc = "TMR (rw) register accessor: an alias for `Reg<TMR_SPEC>`"]
pub type TMR = crate::Reg<tmr::TMR_SPEC>;
#[doc = "Timer Mode Register"]
pub mod tmr;
#[doc = "TPRE (rw) register accessor: an alias for `Reg<TPRE_SPEC>`"]
pub type TPRE = crate::Reg<tpre::TPRE_SPEC>;
#[doc = "Timer Prescaler Register"]
pub mod tpre;
#[doc = "TCNT (rw) register accessor: an alias for `Reg<TCNT_SPEC>`"]
pub type TCNT = crate::Reg<tcnt::TCNT_SPEC>;
#[doc = "Timer Count Register"]
pub mod tcnt;
