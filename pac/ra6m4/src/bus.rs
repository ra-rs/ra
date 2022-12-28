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
    #[doc = "0x802 - CS%s Control Register"]
    pub cs0cr: CSCR,
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
    _reserved41: [u8; 0x087e],
    #[doc = "0x1100 - Slave Bus Control Register"]
    pub busscntfhbiu: BUSSCNTFHBIU,
    _reserved42: [u8; 0x02],
    #[doc = "0x1104 - Slave Bus Control Register"]
    pub busscntflbiu: BUSSCNTFLBIU,
    _reserved43: [u8; 0x0a],
    #[doc = "0x1110 - Slave Bus Control Register"]
    pub busscnts0biu: BUSSCNTS0BIU,
    _reserved44: [u8; 0x0e],
    #[doc = "0x1120 - Slave Bus Control Register"]
    pub busscntpsbiu: BUSSCNTPSBIU,
    _reserved45: [u8; 0x0e],
    #[doc = "0x1130 - Slave Bus Control Register"]
    pub busscntplbiu: BUSSCNTPLBIU,
    _reserved46: [u8; 0x02],
    #[doc = "0x1134 - Slave Bus Control Register"]
    pub busscntphbiu: BUSSCNTPHBIU,
    _reserved47: [u8; 0x0a],
    #[doc = "0x1140 - Slave Bus Control Register"]
    pub busscnteqbiu: BUSSCNTEQBIU,
    _reserved48: [u8; 0x02],
    #[doc = "0x1144 - Slave Bus Control Register"]
    pub busscnteobiu: BUSSCNTEOBIU,
    _reserved49: [u8; 0x02],
    #[doc = "0x1148 - Slave Bus Control Register"]
    pub busscntecbiu: BUSSCNTECBIU,
    _reserved50: [u8; 0x06b6],
    #[doc = "0x1800 - BUS Error Address Register"]
    pub bus1erradd: BUSERRADD,
    #[doc = "0x1804 - BUS Error Read Write Register"]
    pub bus1errrw: BUSERRRW,
    _reserved52: [u8; 0x0b],
    #[doc = "0x1810 - BUS Error Address Register"]
    pub bus2erradd: BUSERRADD,
    #[doc = "0x1814 - BUS Error Read Write Register"]
    pub bus2errrw: BUSERRRW,
    _reserved54: [u8; 0x0b],
    #[doc = "0x1820 - BUS Error Address Register"]
    pub bus3erradd: BUSERRADD,
    #[doc = "0x1824 - BUS Error Read Write Register"]
    pub bus3errrw: BUSERRRW,
    _reserved56: [u8; 0x0b],
    #[doc = "0x1830 - BUS Error Address Register"]
    pub bus4erradd: BUSERRADD,
    #[doc = "0x1834 - BUS Error Read Write Register"]
    pub bus4errrw: BUSERRRW,
    _reserved58: [u8; 0xcb],
    #[doc = "0x1900 - BUS TZF Error Address Register"]
    pub btzf1erradd: BTZFERRADD,
    #[doc = "0x1904 - BUS TZF Error Read Write Register"]
    pub btzf1errrw: BTZFERRRW,
    _reserved60: [u8; 0x0b],
    #[doc = "0x1910 - BUS TZF Error Address Register"]
    pub btzf2erradd: BTZFERRADD,
    #[doc = "0x1914 - BUS TZF Error Read Write Register"]
    pub btzf2errrw: BTZFERRRW,
    _reserved62: [u8; 0x0b],
    #[doc = "0x1920 - BUS TZF Error Address Register"]
    pub btzf3erradd: BTZFERRADD,
    #[doc = "0x1924 - BUS TZF Error Read Write Register"]
    pub btzf3errrw: BTZFERRRW,
    _reserved64: [u8; 0x0b],
    #[doc = "0x1930 - BUS TZF Error Address Register"]
    pub btzf4erradd: BTZFERRADD,
    #[doc = "0x1934 - BUS TZF Error Read Write Register"]
    pub btzf4errrw: BTZFERRRW,
    _reserved66: [u8; 0xcb],
    #[doc = "0x1a00 - BUS Error Status Register %s"]
    pub bus1errstat: BUSERRSTAT,
    _reserved67: [u8; 0x07],
    #[doc = "0x1a08 - BUS Error Clear Register %s"]
    pub bus1errclr: BUSERRCLR,
    _reserved68: [u8; 0x07],
    #[doc = "0x1a10 - BUS Error Status Register %s"]
    pub bus2errstat: BUSERRSTAT,
    _reserved69: [u8; 0x07],
    #[doc = "0x1a18 - BUS Error Clear Register %s"]
    pub bus2errclr: BUSERRCLR,
    _reserved70: [u8; 0x07],
    #[doc = "0x1a20 - BUS Error Status Register %s"]
    pub bus3errstat: BUSERRSTAT,
    _reserved71: [u8; 0x03],
    #[doc = "0x1a24 - DMAC/DTC Error Status Register"]
    pub dmacdtcerrstat: DMACDTCERRSTAT,
    _reserved72: [u8; 0x03],
    #[doc = "0x1a28 - BUS Error Clear Register %s"]
    pub bus3errclr: BUSERRCLR,
    _reserved73: [u8; 0x03],
    #[doc = "0x1a2c - DMAC/DTC Error Clear Register"]
    pub dmacdtcerrclr: DMACDTCERRCLR,
    _reserved74: [u8; 0x03],
    #[doc = "0x1a30 - BUS Error Status Register %s"]
    pub bus4errstat: BUSERRSTAT,
    _reserved75: [u8; 0x07],
    #[doc = "0x1a38 - BUS Error Clear Register %s"]
    pub bus4errclr: BUSERRCLR,
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
#[doc = "BUSSCNTFHBIU (rw) register accessor: an alias for `Reg<BUSSCNTFHBIU_SPEC>`"]
pub type BUSSCNTFHBIU = crate::Reg<busscntfhbiu::BUSSCNTFHBIU_SPEC>;
#[doc = "Slave Bus Control Register"]
pub mod busscntfhbiu;
#[doc = "BUSSCNTFLBIU (rw) register accessor: an alias for `Reg<BUSSCNTFLBIU_SPEC>`"]
pub type BUSSCNTFLBIU = crate::Reg<busscntflbiu::BUSSCNTFLBIU_SPEC>;
#[doc = "Slave Bus Control Register"]
pub mod busscntflbiu;
#[doc = "BUSSCNTS0BIU (rw) register accessor: an alias for `Reg<BUSSCNTS0BIU_SPEC>`"]
pub type BUSSCNTS0BIU = crate::Reg<busscnts0biu::BUSSCNTS0BIU_SPEC>;
#[doc = "Slave Bus Control Register"]
pub mod busscnts0biu;
#[doc = "BUSSCNTPSBIU (rw) register accessor: an alias for `Reg<BUSSCNTPSBIU_SPEC>`"]
pub type BUSSCNTPSBIU = crate::Reg<busscntpsbiu::BUSSCNTPSBIU_SPEC>;
#[doc = "Slave Bus Control Register"]
pub mod busscntpsbiu;
#[doc = "BUSSCNTPLBIU (rw) register accessor: an alias for `Reg<BUSSCNTPLBIU_SPEC>`"]
pub type BUSSCNTPLBIU = crate::Reg<busscntplbiu::BUSSCNTPLBIU_SPEC>;
#[doc = "Slave Bus Control Register"]
pub mod busscntplbiu;
#[doc = "BUSSCNTPHBIU (rw) register accessor: an alias for `Reg<BUSSCNTPHBIU_SPEC>`"]
pub type BUSSCNTPHBIU = crate::Reg<busscntphbiu::BUSSCNTPHBIU_SPEC>;
#[doc = "Slave Bus Control Register"]
pub mod busscntphbiu;
#[doc = "BUSSCNTEQBIU (rw) register accessor: an alias for `Reg<BUSSCNTEQBIU_SPEC>`"]
pub type BUSSCNTEQBIU = crate::Reg<busscnteqbiu::BUSSCNTEQBIU_SPEC>;
#[doc = "Slave Bus Control Register"]
pub mod busscnteqbiu;
#[doc = "BUSSCNTEOBIU (rw) register accessor: an alias for `Reg<BUSSCNTEOBIU_SPEC>`"]
pub type BUSSCNTEOBIU = crate::Reg<busscnteobiu::BUSSCNTEOBIU_SPEC>;
#[doc = "Slave Bus Control Register"]
pub mod busscnteobiu;
#[doc = "BUSSCNTECBIU (rw) register accessor: an alias for `Reg<BUSSCNTECBIU_SPEC>`"]
pub type BUSSCNTECBIU = crate::Reg<busscntecbiu::BUSSCNTECBIU_SPEC>;
#[doc = "Slave Bus Control Register"]
pub mod busscntecbiu;
#[doc = "BUSERRADD (r) register accessor: an alias for `Reg<BUSERRADD_SPEC>`"]
pub type BUSERRADD = crate::Reg<buserradd::BUSERRADD_SPEC>;
#[doc = "BUS Error Address Register"]
pub mod buserradd;
#[doc = "BUSERRRW (rw) register accessor: an alias for `Reg<BUSERRRW_SPEC>`"]
pub type BUSERRRW = crate::Reg<buserrrw::BUSERRRW_SPEC>;
#[doc = "BUS Error Read Write Register"]
pub mod buserrrw;
#[doc = "BTZFERRADD (r) register accessor: an alias for `Reg<BTZFERRADD_SPEC>`"]
pub type BTZFERRADD = crate::Reg<btzferradd::BTZFERRADD_SPEC>;
#[doc = "BUS TZF Error Address Register"]
pub mod btzferradd;
#[doc = "BTZFERRRW (rw) register accessor: an alias for `Reg<BTZFERRRW_SPEC>`"]
pub type BTZFERRRW = crate::Reg<btzferrrw::BTZFERRRW_SPEC>;
#[doc = "BUS TZF Error Read Write Register"]
pub mod btzferrrw;
#[doc = "BUSERRSTAT (r) register accessor: an alias for `Reg<BUSERRSTAT_SPEC>`"]
pub type BUSERRSTAT = crate::Reg<buserrstat::BUSERRSTAT_SPEC>;
#[doc = "BUS Error Status Register %s"]
pub mod buserrstat;
#[doc = "BUSERRCLR (rw) register accessor: an alias for `Reg<BUSERRCLR_SPEC>`"]
pub type BUSERRCLR = crate::Reg<buserrclr::BUSERRCLR_SPEC>;
#[doc = "BUS Error Clear Register %s"]
pub mod buserrclr;
#[doc = "DMACDTCERRSTAT (r) register accessor: an alias for `Reg<DMACDTCERRSTAT_SPEC>`"]
pub type DMACDTCERRSTAT = crate::Reg<dmacdtcerrstat::DMACDTCERRSTAT_SPEC>;
#[doc = "DMAC/DTC Error Status Register"]
pub mod dmacdtcerrstat;
#[doc = "DMACDTCERRCLR (rw) register accessor: an alias for `Reg<DMACDTCERRCLR_SPEC>`"]
pub type DMACDTCERRCLR = crate::Reg<dmacdtcerrclr::DMACDTCERRCLR_SPEC>;
#[doc = "DMAC/DTC Error Clear Register"]
pub mod dmacdtcerrclr;
