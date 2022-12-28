#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EDMAC Mode Register"]
    pub edmr: EDMR,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - EDMAC Transmit Request Register"]
    pub edtrr: EDTRR,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - EDMAC Receive Request Register"]
    pub edrrr: EDRRR,
    _reserved3: [u8; 0x04],
    #[doc = "0x18 - Transmit Descriptor List Start Address Register"]
    pub tdlar: TDLAR,
    _reserved4: [u8; 0x04],
    #[doc = "0x20 - Receive Descriptor List Start Address Register"]
    pub rdlar: RDLAR,
    _reserved5: [u8; 0x04],
    #[doc = "0x28 - ETHERC/EDMAC Status Register"]
    pub eesr: EESR,
    _reserved6: [u8; 0x04],
    #[doc = "0x30 - ETHERC/EDMAC Status Interrupt Enable Register"]
    pub eesipr: EESIPR,
    _reserved7: [u8; 0x04],
    #[doc = "0x38 - ETHERC/EDMAC Transmit/Receive Status Copy Enable Register"]
    pub trscer: TRSCER,
    _reserved8: [u8; 0x04],
    #[doc = "0x40 - Missed-Frame Counter Register"]
    pub rmfcr: RMFCR,
    _reserved9: [u8; 0x04],
    #[doc = "0x48 - Transmit FIFO Threshold Register"]
    pub tftr: TFTR,
    _reserved10: [u8; 0x04],
    #[doc = "0x50 - FIFO Depth Register"]
    pub fdr: FDR,
    _reserved11: [u8; 0x04],
    #[doc = "0x58 - Receive Method Control Register"]
    pub rmcr: RMCR,
    _reserved12: [u8; 0x08],
    #[doc = "0x64 - Transmit FIFO Underflow Counter"]
    pub tfucr: TFUCR,
    #[doc = "0x68 - Receive FIFO Overflow Counter"]
    pub rfocr: RFOCR,
    #[doc = "0x6c - Independent Output Signal Setting Register"]
    pub iosr: IOSR,
    #[doc = "0x70 - Flow Control Start FIFO Threshold Setting Register"]
    pub fcftr: FCFTR,
    _reserved16: [u8; 0x04],
    #[doc = "0x78 - Receive Data Padding Insert Register"]
    pub rpadir: RPADIR,
    #[doc = "0x7c - Transmit Interrupt Setting Register"]
    pub trimd: TRIMD,
    _reserved18: [u8; 0x48],
    #[doc = "0xc8 - Receive Buffer Write Address Register"]
    pub rbwar: RBWAR,
    #[doc = "0xcc - Receive Descriptor Fetch Address Register"]
    pub rdfar: RDFAR,
    _reserved20: [u8; 0x04],
    #[doc = "0xd4 - Transmit Buffer Read Address Register"]
    pub tbrar: TBRAR,
    #[doc = "0xd8 - Transmit Descriptor Fetch Address Register"]
    pub tdfar: TDFAR,
}
#[doc = "EDMR (rw) register accessor: an alias for `Reg<EDMR_SPEC>`"]
pub type EDMR = crate::Reg<edmr::EDMR_SPEC>;
#[doc = "EDMAC Mode Register"]
pub mod edmr;
#[doc = "EDTRR (rw) register accessor: an alias for `Reg<EDTRR_SPEC>`"]
pub type EDTRR = crate::Reg<edtrr::EDTRR_SPEC>;
#[doc = "EDMAC Transmit Request Register"]
pub mod edtrr;
#[doc = "EDRRR (rw) register accessor: an alias for `Reg<EDRRR_SPEC>`"]
pub type EDRRR = crate::Reg<edrrr::EDRRR_SPEC>;
#[doc = "EDMAC Receive Request Register"]
pub mod edrrr;
#[doc = "TDLAR (rw) register accessor: an alias for `Reg<TDLAR_SPEC>`"]
pub type TDLAR = crate::Reg<tdlar::TDLAR_SPEC>;
#[doc = "Transmit Descriptor List Start Address Register"]
pub mod tdlar;
#[doc = "RDLAR (rw) register accessor: an alias for `Reg<RDLAR_SPEC>`"]
pub type RDLAR = crate::Reg<rdlar::RDLAR_SPEC>;
#[doc = "Receive Descriptor List Start Address Register"]
pub mod rdlar;
#[doc = "EESR (rw) register accessor: an alias for `Reg<EESR_SPEC>`"]
pub type EESR = crate::Reg<eesr::EESR_SPEC>;
#[doc = "ETHERC/EDMAC Status Register"]
pub mod eesr;
#[doc = "EESIPR (rw) register accessor: an alias for `Reg<EESIPR_SPEC>`"]
pub type EESIPR = crate::Reg<eesipr::EESIPR_SPEC>;
#[doc = "ETHERC/EDMAC Status Interrupt Enable Register"]
pub mod eesipr;
#[doc = "TRSCER (rw) register accessor: an alias for `Reg<TRSCER_SPEC>`"]
pub type TRSCER = crate::Reg<trscer::TRSCER_SPEC>;
#[doc = "ETHERC/EDMAC Transmit/Receive Status Copy Enable Register"]
pub mod trscer;
#[doc = "RMFCR (rw) register accessor: an alias for `Reg<RMFCR_SPEC>`"]
pub type RMFCR = crate::Reg<rmfcr::RMFCR_SPEC>;
#[doc = "Missed-Frame Counter Register"]
pub mod rmfcr;
#[doc = "TFTR (rw) register accessor: an alias for `Reg<TFTR_SPEC>`"]
pub type TFTR = crate::Reg<tftr::TFTR_SPEC>;
#[doc = "Transmit FIFO Threshold Register"]
pub mod tftr;
#[doc = "FDR (rw) register accessor: an alias for `Reg<FDR_SPEC>`"]
pub type FDR = crate::Reg<fdr::FDR_SPEC>;
#[doc = "FIFO Depth Register"]
pub mod fdr;
#[doc = "RMCR (rw) register accessor: an alias for `Reg<RMCR_SPEC>`"]
pub type RMCR = crate::Reg<rmcr::RMCR_SPEC>;
#[doc = "Receive Method Control Register"]
pub mod rmcr;
#[doc = "TFUCR (rw) register accessor: an alias for `Reg<TFUCR_SPEC>`"]
pub type TFUCR = crate::Reg<tfucr::TFUCR_SPEC>;
#[doc = "Transmit FIFO Underflow Counter"]
pub mod tfucr;
#[doc = "RFOCR (rw) register accessor: an alias for `Reg<RFOCR_SPEC>`"]
pub type RFOCR = crate::Reg<rfocr::RFOCR_SPEC>;
#[doc = "Receive FIFO Overflow Counter"]
pub mod rfocr;
#[doc = "IOSR (rw) register accessor: an alias for `Reg<IOSR_SPEC>`"]
pub type IOSR = crate::Reg<iosr::IOSR_SPEC>;
#[doc = "Independent Output Signal Setting Register"]
pub mod iosr;
#[doc = "FCFTR (rw) register accessor: an alias for `Reg<FCFTR_SPEC>`"]
pub type FCFTR = crate::Reg<fcftr::FCFTR_SPEC>;
#[doc = "Flow Control Start FIFO Threshold Setting Register"]
pub mod fcftr;
#[doc = "RPADIR (rw) register accessor: an alias for `Reg<RPADIR_SPEC>`"]
pub type RPADIR = crate::Reg<rpadir::RPADIR_SPEC>;
#[doc = "Receive Data Padding Insert Register"]
pub mod rpadir;
#[doc = "TRIMD (rw) register accessor: an alias for `Reg<TRIMD_SPEC>`"]
pub type TRIMD = crate::Reg<trimd::TRIMD_SPEC>;
#[doc = "Transmit Interrupt Setting Register"]
pub mod trimd;
#[doc = "RBWAR (r) register accessor: an alias for `Reg<RBWAR_SPEC>`"]
pub type RBWAR = crate::Reg<rbwar::RBWAR_SPEC>;
#[doc = "Receive Buffer Write Address Register"]
pub mod rbwar;
#[doc = "RDFAR (r) register accessor: an alias for `Reg<RDFAR_SPEC>`"]
pub type RDFAR = crate::Reg<rdfar::RDFAR_SPEC>;
#[doc = "Receive Descriptor Fetch Address Register"]
pub mod rdfar;
#[doc = "TBRAR (r) register accessor: an alias for `Reg<TBRAR_SPEC>`"]
pub type TBRAR = crate::Reg<tbrar::TBRAR_SPEC>;
#[doc = "Transmit Buffer Read Address Register"]
pub mod tbrar;
#[doc = "TDFAR (r) register accessor: an alias for `Reg<TDFAR_SPEC>`"]
pub type TDFAR = crate::Reg<tdfar::TDFAR_SPEC>;
#[doc = "Transmit Descriptor Fetch Address Register"]
pub mod tdfar;
