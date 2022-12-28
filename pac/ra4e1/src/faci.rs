#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - Flash Access Status Register"]
    pub fastat: FASTAT,
    _reserved1: [u8; 0x03],
    #[doc = "0x14 - Flash Access Error Interrupt Enable Register"]
    pub faeint: FAEINT,
    _reserved2: [u8; 0x03],
    #[doc = "0x18 - Flash Ready Interrupt Enable Register"]
    pub frdyie: FRDYIE,
    _reserved3: [u8; 0x17],
    #[doc = "0x30 - FACI Command Start Address Register"]
    pub fsaddr: FSADDR,
    #[doc = "0x34 - FACI Command End Address Register"]
    pub feaddr: FEADDR,
    _reserved5: [u8; 0x0c],
    #[doc = "0x44 - Flash P/E Mode Entry Protection Register"]
    pub fmeprot: FMEPROT,
    _reserved6: [u8; 0x32],
    #[doc = "0x78 - Flash Block Protection Register"]
    pub fbprot0: FBPROT0,
    _reserved7: [u8; 0x02],
    #[doc = "0x7c - Flash Block Protection for Secure Register"]
    pub fbprot1: FBPROT1,
    _reserved8: [u8; 0x02],
    #[doc = "0x80 - Flash Status Register"]
    pub fstatr: FSTATR,
    #[doc = "0x84 - Flash P/E Mode Entry Register"]
    pub fentryr: FENTRYR,
    _reserved10: [u8; 0x06],
    #[doc = "0x8c - Flash Sequencer Setup Initialization Register"]
    pub fsuinitr: FSUINITR,
    _reserved11: [u8; 0x12],
    #[doc = "0xa0 - FACI Command Register"]
    pub fcmdr: FCMDR,
    _reserved12: [u8; 0x2e],
    #[doc = "0xd0 - Blank Check Control Register"]
    pub fbccnt: FBCCNT,
    _reserved13: [u8; 0x03],
    #[doc = "0xd4 - Blank Check Status Register"]
    pub fbcstat: FBCSTAT,
    _reserved14: [u8; 0x03],
    #[doc = "0xd8 - Data Flash Programming Start Address Register"]
    pub fpsaddr: FPSADDR,
    #[doc = "0xdc - Flash Startup Area Select Monitor Register"]
    pub fsuasmon: FSUASMON,
    #[doc = "0xe0 - Flash Sequencer Processing Switching Register"]
    pub fcpsr: FCPSR,
    _reserved17: [u8; 0x02],
    #[doc = "0xe4 - Flash Sequencer Processing Clock Notification Register"]
    pub fpckar: FPCKAR,
    _reserved18: [u8; 0x02],
    #[doc = "0xe8 - Flash Startup Area Control Register"]
    pub fsuacr: FSUACR,
}
#[doc = "FASTAT (rw) register accessor: an alias for `Reg<FASTAT_SPEC>`"]
pub type FASTAT = crate::Reg<fastat::FASTAT_SPEC>;
#[doc = "Flash Access Status Register"]
pub mod fastat;
#[doc = "FAEINT (rw) register accessor: an alias for `Reg<FAEINT_SPEC>`"]
pub type FAEINT = crate::Reg<faeint::FAEINT_SPEC>;
#[doc = "Flash Access Error Interrupt Enable Register"]
pub mod faeint;
#[doc = "FRDYIE (rw) register accessor: an alias for `Reg<FRDYIE_SPEC>`"]
pub type FRDYIE = crate::Reg<frdyie::FRDYIE_SPEC>;
#[doc = "Flash Ready Interrupt Enable Register"]
pub mod frdyie;
#[doc = "FSADDR (rw) register accessor: an alias for `Reg<FSADDR_SPEC>`"]
pub type FSADDR = crate::Reg<fsaddr::FSADDR_SPEC>;
#[doc = "FACI Command Start Address Register"]
pub mod fsaddr;
#[doc = "FEADDR (rw) register accessor: an alias for `Reg<FEADDR_SPEC>`"]
pub type FEADDR = crate::Reg<feaddr::FEADDR_SPEC>;
#[doc = "FACI Command End Address Register"]
pub mod feaddr;
#[doc = "FMEPROT (rw) register accessor: an alias for `Reg<FMEPROT_SPEC>`"]
pub type FMEPROT = crate::Reg<fmeprot::FMEPROT_SPEC>;
#[doc = "Flash P/E Mode Entry Protection Register"]
pub mod fmeprot;
#[doc = "FBPROT0 (rw) register accessor: an alias for `Reg<FBPROT0_SPEC>`"]
pub type FBPROT0 = crate::Reg<fbprot0::FBPROT0_SPEC>;
#[doc = "Flash Block Protection Register"]
pub mod fbprot0;
#[doc = "FBPROT1 (rw) register accessor: an alias for `Reg<FBPROT1_SPEC>`"]
pub type FBPROT1 = crate::Reg<fbprot1::FBPROT1_SPEC>;
#[doc = "Flash Block Protection for Secure Register"]
pub mod fbprot1;
#[doc = "FSTATR (rw) register accessor: an alias for `Reg<FSTATR_SPEC>`"]
pub type FSTATR = crate::Reg<fstatr::FSTATR_SPEC>;
#[doc = "Flash Status Register"]
pub mod fstatr;
#[doc = "FENTRYR (rw) register accessor: an alias for `Reg<FENTRYR_SPEC>`"]
pub type FENTRYR = crate::Reg<fentryr::FENTRYR_SPEC>;
#[doc = "Flash P/E Mode Entry Register"]
pub mod fentryr;
#[doc = "FSUINITR (rw) register accessor: an alias for `Reg<FSUINITR_SPEC>`"]
pub type FSUINITR = crate::Reg<fsuinitr::FSUINITR_SPEC>;
#[doc = "Flash Sequencer Setup Initialization Register"]
pub mod fsuinitr;
#[doc = "FCMDR (r) register accessor: an alias for `Reg<FCMDR_SPEC>`"]
pub type FCMDR = crate::Reg<fcmdr::FCMDR_SPEC>;
#[doc = "FACI Command Register"]
pub mod fcmdr;
#[doc = "FBCCNT (rw) register accessor: an alias for `Reg<FBCCNT_SPEC>`"]
pub type FBCCNT = crate::Reg<fbccnt::FBCCNT_SPEC>;
#[doc = "Blank Check Control Register"]
pub mod fbccnt;
#[doc = "FBCSTAT (rw) register accessor: an alias for `Reg<FBCSTAT_SPEC>`"]
pub type FBCSTAT = crate::Reg<fbcstat::FBCSTAT_SPEC>;
#[doc = "Blank Check Status Register"]
pub mod fbcstat;
#[doc = "FPSADDR (rw) register accessor: an alias for `Reg<FPSADDR_SPEC>`"]
pub type FPSADDR = crate::Reg<fpsaddr::FPSADDR_SPEC>;
#[doc = "Data Flash Programming Start Address Register"]
pub mod fpsaddr;
#[doc = "FSUASMON (r) register accessor: an alias for `Reg<FSUASMON_SPEC>`"]
pub type FSUASMON = crate::Reg<fsuasmon::FSUASMON_SPEC>;
#[doc = "Flash Startup Area Select Monitor Register"]
pub mod fsuasmon;
#[doc = "FCPSR (rw) register accessor: an alias for `Reg<FCPSR_SPEC>`"]
pub type FCPSR = crate::Reg<fcpsr::FCPSR_SPEC>;
#[doc = "Flash Sequencer Processing Switching Register"]
pub mod fcpsr;
#[doc = "FPCKAR (rw) register accessor: an alias for `Reg<FPCKAR_SPEC>`"]
pub type FPCKAR = crate::Reg<fpckar::FPCKAR_SPEC>;
#[doc = "Flash Sequencer Processing Clock Notification Register"]
pub mod fpckar;
#[doc = "FSUACR (rw) register accessor: an alias for `Reg<FSUACR_SPEC>`"]
pub type FSUACR = crate::Reg<fsuacr::FSUACR_SPEC>;
#[doc = "Flash Startup Area Control Register"]
pub mod fsuacr;
