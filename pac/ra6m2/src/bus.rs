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
    _reserved12: [u8; 0x06],
    #[doc = "0x42 - CS%s Mode Register"]
    pub cs4mod: CSMOD,
    #[doc = "0x44 - CS%s Wait Control Register 1"]
    pub cs4wcr1: CSWCR1,
    #[doc = "0x48 - CS%s Wait Control Register 2"]
    pub cs4wcr2: CSWCR2,
    _reserved15: [u8; 0x06],
    #[doc = "0x52 - CS%s Mode Register"]
    pub cs5mod: CSMOD,
    #[doc = "0x54 - CS%s Wait Control Register 1"]
    pub cs5wcr1: CSWCR1,
    #[doc = "0x58 - CS%s Wait Control Register 2"]
    pub cs5wcr2: CSWCR2,
    _reserved18: [u8; 0x06],
    #[doc = "0x62 - CS%s Mode Register"]
    pub cs6mod: CSMOD,
    #[doc = "0x64 - CS%s Wait Control Register 1"]
    pub cs6wcr1: CSWCR1,
    #[doc = "0x68 - CS%s Wait Control Register 2"]
    pub cs6wcr2: CSWCR2,
    _reserved21: [u8; 0x06],
    #[doc = "0x72 - CS%s Mode Register"]
    pub cs7mod: CSMOD,
    #[doc = "0x74 - CS%s Wait Control Register 1"]
    pub cs7wcr1: CSWCR1,
    #[doc = "0x78 - CS%s Wait Control Register 2"]
    pub cs7wcr2: CSWCR2,
    _reserved24: [u8; 0x0786],
    #[doc = "0x802 - CS0 Control Register"]
    pub cs0cr: CS0CR,
    _reserved25: [u8; 0x06],
    #[doc = "0x80a - CS%s Recovery Cycle Register"]
    pub cs0rec: CSREC,
    _reserved26: [u8; 0x06],
    #[doc = "0x812 - CS%s Control Register"]
    pub cs1cr: CSCR,
    _reserved27: [u8; 0x06],
    #[doc = "0x81a - CS%s Recovery Cycle Register"]
    pub cs1rec: CSREC,
    _reserved28: [u8; 0x06],
    #[doc = "0x822 - CS%s Control Register"]
    pub cs2cr: CSCR,
    _reserved29: [u8; 0x06],
    #[doc = "0x82a - CS%s Recovery Cycle Register"]
    pub cs2rec: CSREC,
    _reserved30: [u8; 0x06],
    #[doc = "0x832 - CS%s Control Register"]
    pub cs3cr: CSCR,
    _reserved31: [u8; 0x06],
    #[doc = "0x83a - CS%s Recovery Cycle Register"]
    pub cs3rec: CSREC,
    _reserved32: [u8; 0x06],
    #[doc = "0x842 - CS%s Control Register"]
    pub cs4cr: CSCR,
    _reserved33: [u8; 0x06],
    #[doc = "0x84a - CS%s Recovery Cycle Register"]
    pub cs4rec: CSREC,
    _reserved34: [u8; 0x06],
    #[doc = "0x852 - CS%s Control Register"]
    pub cs5cr: CSCR,
    _reserved35: [u8; 0x06],
    #[doc = "0x85a - CS%s Recovery Cycle Register"]
    pub cs5rec: CSREC,
    _reserved36: [u8; 0x06],
    #[doc = "0x862 - CS%s Control Register"]
    pub cs6cr: CSCR,
    _reserved37: [u8; 0x06],
    #[doc = "0x86a - CS%s Recovery Cycle Register"]
    pub cs6rec: CSREC,
    _reserved38: [u8; 0x06],
    #[doc = "0x872 - CS%s Control Register"]
    pub cs7cr: CSCR,
    _reserved39: [u8; 0x06],
    #[doc = "0x87a - CS%s Recovery Cycle Register"]
    pub cs7rec: CSREC,
    _reserved40: [u8; 0x04],
    #[doc = "0x880 - CS Recovery Cycle Insertion Enable Register"]
    pub csrecen: CSRECEN,
    _reserved41: [u8; 0x037e],
    #[doc = "0xc00 - SDC Control Register"]
    pub sdccr: SDCCR,
    #[doc = "0xc01 - SDC Mode Register"]
    pub sdcmod: SDCMOD,
    #[doc = "0xc02 - SDRAM Access Mode Register"]
    pub sdamod: SDAMOD,
    _reserved44: [u8; 0x0d],
    #[doc = "0xc10 - SDRAM Self-Refresh Control Register"]
    pub sdself: SDSELF,
    _reserved45: [u8; 0x03],
    #[doc = "0xc14 - SDRAM Refresh Control Register"]
    pub sdrfcr: SDRFCR,
    #[doc = "0xc16 - SDRAM Auto-Refresh Control Register"]
    pub sdrfen: SDRFEN,
    _reserved47: [u8; 0x09],
    #[doc = "0xc20 - SDRAM Initialization Sequence Control Register"]
    pub sdicr: SDICR,
    _reserved48: [u8; 0x03],
    #[doc = "0xc24 - SDRAM Initialization Register"]
    pub sdir: SDIR,
    _reserved49: [u8; 0x1a],
    #[doc = "0xc40 - SDRAM Address Register"]
    pub sdadr: SDADR,
    _reserved50: [u8; 0x03],
    #[doc = "0xc44 - SDRAM Timing Register"]
    pub sdtr: SDTR,
    #[doc = "0xc48 - SDRAM Mode Register"]
    pub sdmod: SDMOD,
    _reserved52: [u8; 0x06],
    #[doc = "0xc50 - SDRAM Status Register"]
    pub sdsr: SDSR,
    _reserved53: [u8; 0x03af],
    #[doc = "0x1000 - Master Bus Control Register %s"]
    pub busmcntm4i: BUSMCNT,
    _reserved54: [u8; 0x02],
    #[doc = "0x1004 - Master Bus Control Register %s"]
    pub busmcntm4d: BUSMCNT,
    _reserved55: [u8; 0x02],
    #[doc = "0x1008 - Master Bus Control Register SYS"]
    pub busmcntsys: BUSMCNTSYS,
    _reserved56: [u8; 0x02],
    #[doc = "0x100c - Master Bus Control Register DMA"]
    pub busmcntdma: BUSMCNTDMA,
    _reserved57: [u8; 0x02],
    #[doc = "0x1010 - Master Bus Control Register %s"]
    pub busmcntedm: BUSMCNTEDM,
    _reserved58: [u8; 0x02],
    #[doc = "0x1014 - Master Bus Control Register %s"]
    pub busmcntgpx: BUSMCNTEDM,
    _reserved59: [u8; 0xea],
    #[doc = "0x1100 - Slave Bus Control Register %s"]
    pub busscntfli: BUSSCNT,
    _reserved60: [u8; 0x02],
    #[doc = "0x1104 - Slave Bus Control Register %s"]
    pub busscntramh: BUSSCNT,
    _reserved61: [u8; 0x02],
    #[doc = "0x1108 - Slave Bus Control Register MBIU"]
    pub busscntmbiu: BUSSCNTMBIU,
    _reserved62: [u8; 0x02],
    #[doc = "0x110c - Slave Bus Control Register %s"]
    pub busscntram0: BUSSCNTRAM0,
    _reserved63: [u8; 0x02],
    #[doc = "0x1110 - Slave Bus Control Register %s"]
    pub busscntram1: BUSSCNTRAM0,
    _reserved64: [u8; 0x02],
    #[doc = "0x1114 - Slave Bus Control Register %s"]
    pub busscntp0b: BUSSCNTP0B,
    _reserved65: [u8; 0x02],
    #[doc = "0x1118 - Slave Bus Control Register %s"]
    pub busscntp2b: BUSSCNTP0B,
    _reserved66: [u8; 0x02],
    #[doc = "0x111c - Slave Bus Control Register %s"]
    pub busscntp3b: BUSSCNTP0B,
    _reserved67: [u8; 0x02],
    #[doc = "0x1120 - Slave Bus Control Register %s"]
    pub busscntp4b: BUSSCNTP0B,
    _reserved68: [u8; 0x06],
    #[doc = "0x1128 - Slave Bus Control Register %s"]
    pub busscntp6b: BUSSCNTP6B,
    _reserved69: [u8; 0x02],
    #[doc = "0x112c - Slave Bus Control Register %s"]
    pub busscntp7b: BUSSCNTP6B,
    _reserved70: [u8; 0x02],
    #[doc = "0x1130 - Slave Bus Control Register %s"]
    pub busscntfbu: BUSSCNTFBU,
    _reserved71: [u8; 0x02],
    #[doc = "0x1134 - Slave Bus Control Register %s"]
    pub busscntext: BUSSCNTFBU,
    _reserved72: [u8; 0x02],
    #[doc = "0x1138 - Slave Bus Control Register %s"]
    pub busscntext2: BUSSCNTFBU,
    _reserved73: [u8; 0x02],
    #[doc = "0x113c - Slave Bus Control Register %s"]
    pub busscntgpx: BUSSCNTFBU,
    _reserved74: [u8; 0x06c2],
    #[doc = "0x1800 - Bus Error Address Register %s"]
    pub bus1erradd: BUSERRADD,
    #[doc = "0x1804 - Bus Error Status Register %s"]
    pub bus1errstat: BUSERRSTAT,
    _reserved76: [u8; 0x0b],
    #[doc = "0x1810 - Bus Error Address Register %s"]
    pub bus2erradd: BUSERRADD,
    #[doc = "0x1814 - Bus Error Status Register %s"]
    pub bus2errstat: BUSERRSTAT,
    _reserved78: [u8; 0x0b],
    #[doc = "0x1820 - Bus Error Address Register %s"]
    pub bus3erradd: BUSERRADD,
    #[doc = "0x1824 - Bus Error Status Register %s"]
    pub bus3errstat: BUSERRSTAT,
    _reserved80: [u8; 0x0b],
    #[doc = "0x1830 - Bus Error Address Register %s"]
    pub bus4erradd: BUSERRADD,
    #[doc = "0x1834 - Bus Error Status Register %s"]
    pub bus4errstat: BUSERRSTAT,
    _reserved82: [u8; 0x0b],
    #[doc = "0x1840 - Bus Error Address Register %s"]
    pub bus5erradd: BUSERRADD,
    #[doc = "0x1844 - Bus Error Status Register %s"]
    pub bus5errstat: BUSERRSTAT,
    _reserved84: [u8; 0x0b],
    #[doc = "0x1850 - Bus Error Address Register %s"]
    pub bus6erradd: BUSERRADD,
    #[doc = "0x1854 - Bus Error Status Register %s"]
    pub bus6errstat: BUSERRSTAT,
    _reserved86: [u8; 0x0b],
    #[doc = "0x1860 - Bus Error Address Register %s"]
    pub bus7erradd: BUSERRADD,
    #[doc = "0x1864 - Bus Error Status Register %s"]
    pub bus7errstat: BUSERRSTAT,
    _reserved88: [u8; 0x0b],
    #[doc = "0x1870 - Bus Error Address Register %s"]
    pub bus8erradd: BUSERRADD,
    #[doc = "0x1874 - Bus Error Status Register %s"]
    pub bus8errstat: BUSERRSTAT,
    _reserved90: [u8; 0x0b],
    #[doc = "0x1880 - Bus Error Address Register %s"]
    pub bus9erradd: BUSERRADD,
    #[doc = "0x1884 - Bus Error Status Register %s"]
    pub bus9errstat: BUSERRSTAT,
    _reserved92: [u8; 0x0b],
    #[doc = "0x1890 - Bus Error Address Register %s"]
    pub bus10erradd: BUSERRADD,
    #[doc = "0x1894 - Bus Error Status Register %s"]
    pub bus10errstat: BUSERRSTAT,
    _reserved94: [u8; 0x0b],
    #[doc = "0x18a0 - Bus Error Address Register %s"]
    pub bus11erradd: BUSERRADD,
    #[doc = "0x18a4 - Bus Error Status Register %s"]
    pub bus11errstat: BUSERRSTAT,
}
#[doc = "CS0CR (rw) register accessor: an alias for `Reg<CS0CR_SPEC>`"]
pub type CS0CR = crate::Reg<cs0cr::CS0CR_SPEC>;
#[doc = "CS0 Control Register"]
pub mod cs0cr;
#[doc = "CSCR (rw) register accessor: an alias for `Reg<CSCR_SPEC>`"]
pub type CSCR = crate::Reg<cscr::CSCR_SPEC>;
#[doc = "CS%s Control Register"]
pub mod cscr;
#[doc = "CSREC (rw) register accessor: an alias for `Reg<CSREC_SPEC>`"]
pub type CSREC = crate::Reg<csrec::CSREC_SPEC>;
#[doc = "CS%s Recovery Cycle Register"]
pub mod csrec;
#[doc = "CSRECEN (rw) register accessor: an alias for `Reg<CSRECEN_SPEC>`"]
pub type CSRECEN = crate::Reg<csrecen::CSRECEN_SPEC>;
#[doc = "CS Recovery Cycle Insertion Enable Register"]
pub mod csrecen;
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
#[doc = "SDCCR (rw) register accessor: an alias for `Reg<SDCCR_SPEC>`"]
pub type SDCCR = crate::Reg<sdccr::SDCCR_SPEC>;
#[doc = "SDC Control Register"]
pub mod sdccr;
#[doc = "SDCMOD (rw) register accessor: an alias for `Reg<SDCMOD_SPEC>`"]
pub type SDCMOD = crate::Reg<sdcmod::SDCMOD_SPEC>;
#[doc = "SDC Mode Register"]
pub mod sdcmod;
#[doc = "SDAMOD (rw) register accessor: an alias for `Reg<SDAMOD_SPEC>`"]
pub type SDAMOD = crate::Reg<sdamod::SDAMOD_SPEC>;
#[doc = "SDRAM Access Mode Register"]
pub mod sdamod;
#[doc = "SDSELF (rw) register accessor: an alias for `Reg<SDSELF_SPEC>`"]
pub type SDSELF = crate::Reg<sdself::SDSELF_SPEC>;
#[doc = "SDRAM Self-Refresh Control Register"]
pub mod sdself;
#[doc = "SDRFCR (rw) register accessor: an alias for `Reg<SDRFCR_SPEC>`"]
pub type SDRFCR = crate::Reg<sdrfcr::SDRFCR_SPEC>;
#[doc = "SDRAM Refresh Control Register"]
pub mod sdrfcr;
#[doc = "SDRFEN (rw) register accessor: an alias for `Reg<SDRFEN_SPEC>`"]
pub type SDRFEN = crate::Reg<sdrfen::SDRFEN_SPEC>;
#[doc = "SDRAM Auto-Refresh Control Register"]
pub mod sdrfen;
#[doc = "SDICR (rw) register accessor: an alias for `Reg<SDICR_SPEC>`"]
pub type SDICR = crate::Reg<sdicr::SDICR_SPEC>;
#[doc = "SDRAM Initialization Sequence Control Register"]
pub mod sdicr;
#[doc = "SDIR (rw) register accessor: an alias for `Reg<SDIR_SPEC>`"]
pub type SDIR = crate::Reg<sdir::SDIR_SPEC>;
#[doc = "SDRAM Initialization Register"]
pub mod sdir;
#[doc = "SDADR (rw) register accessor: an alias for `Reg<SDADR_SPEC>`"]
pub type SDADR = crate::Reg<sdadr::SDADR_SPEC>;
#[doc = "SDRAM Address Register"]
pub mod sdadr;
#[doc = "SDTR (rw) register accessor: an alias for `Reg<SDTR_SPEC>`"]
pub type SDTR = crate::Reg<sdtr::SDTR_SPEC>;
#[doc = "SDRAM Timing Register"]
pub mod sdtr;
#[doc = "SDMOD (rw) register accessor: an alias for `Reg<SDMOD_SPEC>`"]
pub type SDMOD = crate::Reg<sdmod::SDMOD_SPEC>;
#[doc = "SDRAM Mode Register"]
pub mod sdmod;
#[doc = "SDSR (r) register accessor: an alias for `Reg<SDSR_SPEC>`"]
pub type SDSR = crate::Reg<sdsr::SDSR_SPEC>;
#[doc = "SDRAM Status Register"]
pub mod sdsr;
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
pub use busmcnt as busmcntedm;
pub use BUSMCNT as BUSMCNTEDM;
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
pub use busscnt as busscntp6b;
pub use busscnt as busscntfbu;
pub use BUSSCNT as BUSSCNTRAM0;
pub use BUSSCNT as BUSSCNTP0B;
pub use BUSSCNT as BUSSCNTP6B;
pub use BUSSCNT as BUSSCNTFBU;
