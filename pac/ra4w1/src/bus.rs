#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x02],
    #[doc = "0x02 - CS%s Mode Register"]
    pub cs0mod: CSMOD,
    #[doc = "0x04 - CS%s Wait Control Register 1"]
    pub cs0wcr1: CSWCR1,
    #[doc = "0x08 - CS%s Wait Control Register 2"]
    pub cs0wcr2: CSWCR2,
    _reserved3: [u8; 0x06],
    #[doc = "0x12 - CS%s Mode Register"]
    pub cs1mod: CSMOD,
    #[doc = "0x14 - CS%s Wait Control Register 1"]
    pub cs1wcr1: CSWCR1,
    #[doc = "0x18 - CS%s Wait Control Register 2"]
    pub cs1wcr2: CSWCR2,
    _reserved6: [u8; 0x06],
    #[doc = "0x22 - CS%s Mode Register"]
    pub cs2mod: CSMOD,
    #[doc = "0x24 - CS%s Wait Control Register 1"]
    pub cs2wcr1: CSWCR1,
    #[doc = "0x28 - CS%s Wait Control Register 2"]
    pub cs2wcr2: CSWCR2,
    _reserved9: [u8; 0x06],
    #[doc = "0x32 - CS%s Mode Register"]
    pub cs3mod: CSMOD,
    #[doc = "0x34 - CS%s Wait Control Register 1"]
    pub cs3wcr1: CSWCR1,
    #[doc = "0x38 - CS%s Wait Control Register 2"]
    pub cs3wcr2: CSWCR2,
    _reserved12: [u8; 0x07c6],
    #[doc = "0x802 - CS0 Control Register"]
    pub cs0cr: CS0CR,
    _reserved13: [u8; 0x06],
    #[doc = "0x80a - CS%s Recovery Cycle Register"]
    pub cs0rec: CSREC,
    _reserved14: [u8; 0x06],
    #[doc = "0x812 - CS%s Control Register"]
    pub cs1cr: CSCR,
    _reserved15: [u8; 0x06],
    #[doc = "0x81a - CS%s Recovery Cycle Register"]
    pub cs1rec: CSREC,
    _reserved16: [u8; 0x06],
    #[doc = "0x822 - CS%s Control Register"]
    pub cs2cr: CSCR,
    _reserved17: [u8; 0x06],
    #[doc = "0x82a - CS%s Recovery Cycle Register"]
    pub cs2rec: CSREC,
    _reserved18: [u8; 0x06],
    #[doc = "0x832 - CS%s Control Register"]
    pub cs3cr: CSCR,
    _reserved19: [u8; 0x06],
    #[doc = "0x83a - CS%s Recovery Cycle Register"]
    pub cs3rec: CSREC,
    _reserved20: [u8; 0x44],
    #[doc = "0x880 - CS Recovery Cycle Insertion Enable Register"]
    pub csrecen: CSRECEN,
    _reserved21: [u8; 0x077e],
    #[doc = "0x1000 - Master Bus Control Register %s"]
    pub busmcntm4i: BUSMCNT,
    _reserved22: [u8; 0x02],
    #[doc = "0x1004 - Master Bus Control Register %s"]
    pub busmcntm4d: BUSMCNT,
    _reserved23: [u8; 0x02],
    #[doc = "0x1008 - Master Bus Control Register %s"]
    pub busmcntsys: BUSMCNT,
    _reserved24: [u8; 0x02],
    #[doc = "0x100c - Master Bus Control Register %s"]
    pub busmcntdma: BUSMCNT,
    _reserved25: [u8; 0xf2],
    #[doc = "0x1100 - Slave Bus Control Register FLI"]
    pub busscntfli: BUSSCNTFLI,
    _reserved26: [u8; 0x06],
    #[doc = "0x1108 - Slave Bus Control Register %s"]
    pub busscntmbiu: BUSSCNT,
    _reserved27: [u8; 0x02],
    #[doc = "0x110c - Slave Bus Control Register %s"]
    pub busscntram0: BUSSCNT,
    _reserved28: [u8; 0x06],
    #[doc = "0x1114 - Slave Bus Control Register %s"]
    pub busscntp0b: BUSSCNTP0B,
    _reserved29: [u8; 0x02],
    #[doc = "0x1118 - Slave Bus Control Register %s"]
    pub busscntp2b: BUSSCNTP0B,
    _reserved30: [u8; 0x02],
    #[doc = "0x111c - Slave Bus Control Register %s"]
    pub busscntp3b: BUSSCNTP0B,
    _reserved31: [u8; 0x02],
    #[doc = "0x1120 - Slave Bus Control Register %s"]
    pub busscntp4b: BUSSCNTP0B,
    _reserved32: [u8; 0x06],
    #[doc = "0x1128 - Slave Bus Control Register P6B"]
    pub busscntp6b: BUSSCNTP6B,
    _reserved33: [u8; 0x06],
    #[doc = "0x1130 - Slave Bus Control Register %s"]
    pub busscntfbu: BUSSCNTFBU,
    _reserved34: [u8; 0x02],
    #[doc = "0x1134 - Slave Bus Control Register %s"]
    pub busscntext: BUSSCNTFBU,
    _reserved35: [u8; 0x02],
    #[doc = "0x1138 - Slave Bus Control Register %s"]
    pub busscntext2: BUSSCNTFBU,
    _reserved36: [u8; 0x06c6],
    #[doc = "0x1800 - Bus Error Address Register %s"]
    pub bus1erradd: BUSERRADD,
    #[doc = "0x1804 - Bus Error Status Register %s"]
    pub bus1errstat: BUSERRSTAT,
    _reserved38: [u8; 0x0b],
    #[doc = "0x1810 - Bus Error Address Register %s"]
    pub bus2erradd: BUSERRADD,
    #[doc = "0x1814 - Bus Error Status Register %s"]
    pub bus2errstat: BUSERRSTAT,
    _reserved40: [u8; 0x0b],
    #[doc = "0x1820 - Bus Error Address Register %s"]
    pub bus3erradd: BUSERRADD,
    #[doc = "0x1824 - Bus Error Status Register %s"]
    pub bus3errstat: BUSERRSTAT,
    _reserved42: [u8; 0x0b],
    #[doc = "0x1830 - Bus Error Address Register %s"]
    pub bus4erradd: BUSERRADD,
    #[doc = "0x1834 - Bus Error Status Register %s"]
    pub bus4errstat: BUSERRSTAT,
}
#[doc = "CSMOD (rw) register accessor: an alias for `Reg<CSMOD_SPEC>`"]
pub type CSMOD = crate::Reg<csmod::CSMOD_SPEC>;
#[doc = "CS%s Mode Register"]
pub mod csmod;
#[doc = "CSWCR1 (rw) register accessor: an alias for `Reg<CSWCR1_SPEC>`"]
pub type CSWCR1 = crate::Reg<cswcr1::CSWCR1_SPEC>;
#[doc = "CS%s Wait Control Register 1"]
pub mod cswcr1;
#[doc = "CSWCR2 (rw) register accessor: an alias for `Reg<CSWCR2_SPEC>`"]
pub type CSWCR2 = crate::Reg<cswcr2::CSWCR2_SPEC>;
#[doc = "CS%s Wait Control Register 2"]
pub mod cswcr2;
#[doc = "CS0CR (rw) register accessor: an alias for `Reg<CS0CR_SPEC>`"]
pub type CS0CR = crate::Reg<cs0cr::CS0CR_SPEC>;
#[doc = "CS0 Control Register"]
pub mod cs0cr;
#[doc = "CSREC (rw) register accessor: an alias for `Reg<CSREC_SPEC>`"]
pub type CSREC = crate::Reg<csrec::CSREC_SPEC>;
#[doc = "CS%s Recovery Cycle Register"]
pub mod csrec;
#[doc = "CSCR (rw) register accessor: an alias for `Reg<CSCR_SPEC>`"]
pub type CSCR = crate::Reg<cscr::CSCR_SPEC>;
#[doc = "CS%s Control Register"]
pub mod cscr;
#[doc = "CSRECEN (rw) register accessor: an alias for `Reg<CSRECEN_SPEC>`"]
pub type CSRECEN = crate::Reg<csrecen::CSRECEN_SPEC>;
#[doc = "CS Recovery Cycle Insertion Enable Register"]
pub mod csrecen;
#[doc = "BUSMCNT (rw) register accessor: an alias for `Reg<BUSMCNT_SPEC>`"]
pub type BUSMCNT = crate::Reg<busmcnt::BUSMCNT_SPEC>;
#[doc = "Master Bus Control Register %s"]
pub mod busmcnt;
#[doc = "BUSSCNTFLI (rw) register accessor: an alias for `Reg<BUSSCNTFLI_SPEC>`"]
pub type BUSSCNTFLI = crate::Reg<busscntfli::BUSSCNTFLI_SPEC>;
#[doc = "Slave Bus Control Register FLI"]
pub mod busscntfli;
#[doc = "BUSSCNT (rw) register accessor: an alias for `Reg<BUSSCNT_SPEC>`"]
pub type BUSSCNT = crate::Reg<busscnt::BUSSCNT_SPEC>;
#[doc = "Slave Bus Control Register %s"]
pub mod busscnt;
pub use busscnt as busscntp0b;
pub use BUSSCNT as BUSSCNTP0B;
#[doc = "BUSSCNTP6B (rw) register accessor: an alias for `Reg<BUSSCNTP6B_SPEC>`"]
pub type BUSSCNTP6B = crate::Reg<busscntp6b::BUSSCNTP6B_SPEC>;
#[doc = "Slave Bus Control Register P6B"]
pub mod busscntp6b;
pub use busscnt as busscntfbu;
pub use BUSSCNT as BUSSCNTFBU;
#[doc = "BUSERRADD (r) register accessor: an alias for `Reg<BUSERRADD_SPEC>`"]
pub type BUSERRADD = crate::Reg<buserradd::BUSERRADD_SPEC>;
#[doc = "Bus Error Address Register %s"]
pub mod buserradd;
#[doc = "BUSERRSTAT (r) register accessor: an alias for `Reg<BUSERRSTAT_SPEC>`"]
pub type BUSERRSTAT = crate::Reg<buserrstat::BUSERRSTAT_SPEC>;
#[doc = "Bus Error Status Register %s"]
pub mod buserrstat;
