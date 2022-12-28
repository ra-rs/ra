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
    _reserved6: [u8; 0x26],
    #[doc = "0x42 - CS%s Mode Register"]
    pub cs4mod: CS4MOD,
    #[doc = "0x44 - CS%s Wait Control Register 1"]
    pub cs4wcr1: CS4WCR1,
    #[doc = "0x48 - CS%s Wait Control Register 2"]
    pub cs4wcr2: CS4WCR2,
    _reserved9: [u8; 0x06],
    #[doc = "0x52 - CS%s Mode Register"]
    pub cs5mod: CS4MOD,
    #[doc = "0x54 - CS%s Wait Control Register 1"]
    pub cs5wcr1: CS4WCR1,
    #[doc = "0x58 - CS%s Wait Control Register 2"]
    pub cs5wcr2: CS4WCR2,
    _reserved12: [u8; 0x06],
    #[doc = "0x62 - CS%s Mode Register"]
    pub cs6mod: CS4MOD,
    #[doc = "0x64 - CS%s Wait Control Register 1"]
    pub cs6wcr1: CS4WCR1,
    #[doc = "0x68 - CS%s Wait Control Register 2"]
    pub cs6wcr2: CS4WCR2,
    _reserved15: [u8; 0x06],
    #[doc = "0x72 - CS%s Mode Register"]
    pub cs7mod: CS4MOD,
    #[doc = "0x74 - CS%s Wait Control Register 1"]
    pub cs7wcr1: CS4WCR1,
    #[doc = "0x78 - CS%s Wait Control Register 2"]
    pub cs7wcr2: CS4WCR2,
    _reserved18: [u8; 0x0786],
    #[doc = "0x802 - CS0 Control Register"]
    pub cs0cr: CS0CR,
    _reserved19: [u8; 0x06],
    #[doc = "0x80a - CS%s Recovery Cycle Register"]
    pub cs0rec: CSREC,
    _reserved20: [u8; 0x06],
    #[doc = "0x812 - CS1 Control Register"]
    pub cs1cr: CS1CR,
    _reserved21: [u8; 0x06],
    #[doc = "0x81a - CS%s Recovery Cycle Register"]
    pub cs1rec: CSREC,
    _reserved22: [u8; 0x26],
    #[doc = "0x842 - CS%s Control Register"]
    pub cs4cr: CSCR,
    _reserved23: [u8; 0x06],
    #[doc = "0x84a - CS%s Recovery Cycle Register"]
    pub cs4rec: CS4REC,
    _reserved24: [u8; 0x06],
    #[doc = "0x852 - CS%s Control Register"]
    pub cs5cr: CSCR,
    _reserved25: [u8; 0x06],
    #[doc = "0x85a - CS%s Recovery Cycle Register"]
    pub cs5rec: CS4REC,
    _reserved26: [u8; 0x06],
    #[doc = "0x862 - CS%s Control Register"]
    pub cs6cr: CSCR,
    _reserved27: [u8; 0x06],
    #[doc = "0x86a - CS%s Recovery Cycle Register"]
    pub cs6rec: CS4REC,
    _reserved28: [u8; 0x06],
    #[doc = "0x872 - CS%s Control Register"]
    pub cs7cr: CSCR,
    _reserved29: [u8; 0x06],
    #[doc = "0x87a - CS%s Recovery Cycle Register"]
    pub cs7rec: CS4REC,
    _reserved30: [u8; 0x04],
    #[doc = "0x880 - CS Recovery Cycle Insertion Enable Register"]
    pub csrecen: CSRECEN,
    _reserved31: [u8; 0x077e],
    #[doc = "0x1000 - Master Bus Control Register %s"]
    pub busmcntm4i: BUSMCNT,
    _reserved32: [u8; 0x02],
    #[doc = "0x1004 - Master Bus Control Register %s"]
    pub busmcntm4d: BUSMCNT,
    _reserved33: [u8; 0x02],
    #[doc = "0x1008 - Master Bus Control Register SYS"]
    pub busmcntsys: BUSMCNTSYS,
    _reserved34: [u8; 0x02],
    #[doc = "0x100c - Master Bus Control Register DMA"]
    pub busmcntdma: BUSMCNTDMA,
    _reserved35: [u8; 0xf2],
    #[doc = "0x1100 - Slave Bus Control Register %s"]
    pub busscntfli: BUSSCNT,
    _reserved36: [u8; 0x02],
    #[doc = "0x1104 - Slave Bus Control Register %s"]
    pub busscntramh: BUSSCNT,
    _reserved37: [u8; 0x02],
    #[doc = "0x1108 - Slave Bus Control Register MBIU"]
    pub busscntmbiu: BUSSCNTMBIU,
    _reserved38: [u8; 0x02],
    #[doc = "0x110c - Slave Bus Control Register %s"]
    pub busscntram0: BUSSCNTRAM0,
    _reserved39: [u8; 0x02],
    #[doc = "0x1110 - Slave Bus Control Register %s"]
    pub busscntram1: BUSSCNTRAM0,
    _reserved40: [u8; 0x02],
    #[doc = "0x1114 - Slave Bus Control Register %s"]
    pub busscntp0b: BUSSCNTP0B,
    _reserved41: [u8; 0x02],
    #[doc = "0x1118 - Slave Bus Control Register %s"]
    pub busscntp2b: BUSSCNTP0B,
    _reserved42: [u8; 0x02],
    #[doc = "0x111c - Slave Bus Control Register %s"]
    pub busscntp3b: BUSSCNTP0B,
    _reserved43: [u8; 0x02],
    #[doc = "0x1120 - Slave Bus Control Register %s"]
    pub busscntp4b: BUSSCNTP0B,
    _reserved44: [u8; 0x06],
    #[doc = "0x1128 - Slave Bus Control Register P6B"]
    pub busscntp6b: BUSSCNTP6B,
    _reserved45: [u8; 0x06],
    #[doc = "0x1130 - Slave Bus Control Register %s"]
    pub busscntfbu: BUSSCNTFBU,
    _reserved46: [u8; 0x02],
    #[doc = "0x1134 - Slave Bus Control Register %s"]
    pub busscntext: BUSSCNTFBU,
    _reserved47: [u8; 0x02],
    #[doc = "0x1138 - Slave Bus Control Register %s"]
    pub busscntext2: BUSSCNTFBU,
    _reserved48: [u8; 0x06c6],
    #[doc = "0x1800 - Bus Error Address Register %s"]
    pub bus1erradd: BUSERRADD,
    #[doc = "0x1804 - Bus Error Status Register %s"]
    pub bus1errstat: BUSERRSTAT,
    _reserved50: [u8; 0x0b],
    #[doc = "0x1810 - Bus Error Address Register %s"]
    pub bus2erradd: BUSERRADD,
    #[doc = "0x1814 - Bus Error Status Register %s"]
    pub bus2errstat: BUSERRSTAT,
    _reserved52: [u8; 0x0b],
    #[doc = "0x1820 - Bus Error Address Register %s"]
    pub bus3erradd: BUSERRADD,
    #[doc = "0x1824 - Bus Error Status Register %s"]
    pub bus3errstat: BUSERRSTAT,
    _reserved54: [u8; 0x0b],
    #[doc = "0x1830 - Bus Error Address Register %s"]
    pub bus4erradd: BUSERRADD,
    #[doc = "0x1834 - Bus Error Status Register %s"]
    pub bus4errstat: BUSERRSTAT,
}
#[doc = "CS0CR (rw) register accessor: an alias for `Reg<CS0CR_SPEC>`"]
pub type CS0CR = crate::Reg<cs0cr::CS0CR_SPEC>;
#[doc = "CS0 Control Register"]
pub mod cs0cr;
pub use cscr as cs1cr;
pub use CSCR as CS1CR;
#[doc = "CSCR (rw) register accessor: an alias for `Reg<CSCR_SPEC>`"]
pub type CSCR = crate::Reg<cscr::CSCR_SPEC>;
#[doc = "CS%s Control Register"]
pub mod cscr;
#[doc = "CSREC (rw) register accessor: an alias for `Reg<CSREC_SPEC>`"]
pub type CSREC = crate::Reg<csrec::CSREC_SPEC>;
#[doc = "CS%s Recovery Cycle Register"]
pub mod csrec;
pub use csrec as cs4rec;
pub use CSREC as CS4REC;
#[doc = "CSRECEN (rw) register accessor: an alias for `Reg<CSRECEN_SPEC>`"]
pub type CSRECEN = crate::Reg<csrecen::CSRECEN_SPEC>;
#[doc = "CS Recovery Cycle Insertion Enable Register"]
pub mod csrecen;
#[doc = "CSMOD (rw) register accessor: an alias for `Reg<CSMOD_SPEC>`"]
pub type CSMOD = crate::Reg<csmod::CSMOD_SPEC>;
#[doc = "CS%s Mode Register"]
pub mod csmod;
pub use csmod as cs4mod;
pub use CSMOD as CS4MOD;
#[doc = "CSWCR1 (rw) register accessor: an alias for `Reg<CSWCR1_SPEC>`"]
pub type CSWCR1 = crate::Reg<cswcr1::CSWCR1_SPEC>;
#[doc = "CS%s Wait Control Register 1"]
pub mod cswcr1;
pub use cswcr1 as cs4wcr1;
pub use CSWCR1 as CS4WCR1;
#[doc = "CSWCR2 (rw) register accessor: an alias for `Reg<CSWCR2_SPEC>`"]
pub type CSWCR2 = crate::Reg<cswcr2::CSWCR2_SPEC>;
#[doc = "CS%s Wait Control Register 2"]
pub mod cswcr2;
pub use cswcr2 as cs4wcr2;
pub use CSWCR2 as CS4WCR2;
#[doc = "BUSERRADD (r) register accessor: an alias for `Reg<BUSERRADD_SPEC>`"]
pub type BUSERRADD = crate::Reg<buserradd::BUSERRADD_SPEC>;
#[doc = "Bus Error Address Register %s"]
pub mod buserradd;
#[doc = "BUSERRSTAT (r) register accessor: an alias for `Reg<BUSERRSTAT_SPEC>`"]
pub type BUSERRSTAT = crate::Reg<buserrstat::BUSERRSTAT_SPEC>;
#[doc = "Bus Error Status Register %s"]
pub mod buserrstat;
#[doc = "BUSMCNT (rw) register accessor: an alias for `Reg<BUSMCNT_SPEC>`"]
pub type BUSMCNT = crate::Reg<busmcnt::BUSMCNT_SPEC>;
#[doc = "Master Bus Control Register %s"]
pub mod busmcnt;
#[doc = "BUSMCNTSYS (rw) register accessor: an alias for `Reg<BUSMCNTSYS_SPEC>`"]
pub type BUSMCNTSYS = crate::Reg<busmcntsys::BUSMCNTSYS_SPEC>;
#[doc = "Master Bus Control Register SYS"]
pub mod busmcntsys;
#[doc = "BUSMCNTDMA (rw) register accessor: an alias for `Reg<BUSMCNTDMA_SPEC>`"]
pub type BUSMCNTDMA = crate::Reg<busmcntdma::BUSMCNTDMA_SPEC>;
#[doc = "Master Bus Control Register DMA"]
pub mod busmcntdma;
#[doc = "BUSSCNT (rw) register accessor: an alias for `Reg<BUSSCNT_SPEC>`"]
pub type BUSSCNT = crate::Reg<busscnt::BUSSCNT_SPEC>;
#[doc = "Slave Bus Control Register %s"]
pub mod busscnt;
#[doc = "BUSSCNTMBIU (rw) register accessor: an alias for `Reg<BUSSCNTMBIU_SPEC>`"]
pub type BUSSCNTMBIU = crate::Reg<busscntmbiu::BUSSCNTMBIU_SPEC>;
#[doc = "Slave Bus Control Register MBIU"]
pub mod busscntmbiu;
pub use busscnt as busscntram0;
pub use busscnt as busscntp0b;
pub use BUSSCNT as BUSSCNTRAM0;
pub use BUSSCNT as BUSSCNTP0B;
#[doc = "BUSSCNTP6B (rw) register accessor: an alias for `Reg<BUSSCNTP6B_SPEC>`"]
pub type BUSSCNTP6B = crate::Reg<busscntp6b::BUSSCNTP6B_SPEC>;
#[doc = "Slave Bus Control Register P6B"]
pub mod busscntp6b;
pub use busscnt as busscntfbu;
pub use BUSSCNT as BUSSCNTFBU;
