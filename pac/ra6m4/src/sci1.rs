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
    #[doc = "0x0e - Transmit Data Register for Non-Manchester mode (MMR.MANEN = 0)"]
    pub tdrhl: TDRHL,
    #[doc = "0x10 - Receive Data Register for Non-Manchester mode (MMR.MANEN = 0)"]
    pub rdrhl: RDRHL,
    #[doc = "0x12 - Modulation Duty Register"]
    pub mddr: MDDR,
    _reserved17: [u8; 0x0d],
    #[doc = "0x20 - Extended Serial Module Enable Register"]
    pub esmer: ESMER,
    #[doc = "0x21 - Control Register 0"]
    pub cr0: CR0,
    #[doc = "0x22 - Control Register 1"]
    pub cr1: CR1,
    #[doc = "0x23 - Control Register 2"]
    pub cr2: CR2,
    #[doc = "0x24 - Control Register 3"]
    pub cr3: CR3,
    #[doc = "0x25 - Port Control Register"]
    pub pcr: PCR,
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
    #[doc = "0x04 - Serial Status Register for Non-Smart Card Interface and Non-FIFO Mode (SCMR.SMIF = 0, FCR.FM = 0, and MMR.MANEN = 0)"]
    #[inline(always)]
    pub const fn ssr(&self) -> &SSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
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
#[doc = "TDRHL (rw) register accessor: an alias for `Reg<TDRHL_SPEC>`"]
pub type TDRHL = crate::Reg<tdrhl::TDRHL_SPEC>;
#[doc = "Transmit Data Register for Non-Manchester mode (MMR.MANEN = 0)"]
pub mod tdrhl;
#[doc = "RDRHL (r) register accessor: an alias for `Reg<RDRHL_SPEC>`"]
pub type RDRHL = crate::Reg<rdrhl::RDRHL_SPEC>;
#[doc = "Receive Data Register for Non-Manchester mode (MMR.MANEN = 0)"]
pub mod rdrhl;
#[doc = "MDDR (rw) register accessor: an alias for `Reg<MDDR_SPEC>`"]
pub type MDDR = crate::Reg<mddr::MDDR_SPEC>;
#[doc = "Modulation Duty Register"]
pub mod mddr;
#[doc = "ESMER (rw) register accessor: an alias for `Reg<ESMER_SPEC>`"]
pub type ESMER = crate::Reg<esmer::ESMER_SPEC>;
#[doc = "Extended Serial Module Enable Register"]
pub mod esmer;
#[doc = "CR0 (rw) register accessor: an alias for `Reg<CR0_SPEC>`"]
pub type CR0 = crate::Reg<cr0::CR0_SPEC>;
#[doc = "Control Register 0"]
pub mod cr0;
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "Control Register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "Control Register 2"]
pub mod cr2;
#[doc = "CR3 (rw) register accessor: an alias for `Reg<CR3_SPEC>`"]
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
#[doc = "Control Register 3"]
pub mod cr3;
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
