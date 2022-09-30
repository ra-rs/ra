#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - 64-Hz Counter"]
    pub r64cnt: R64CNT,
    _reserved1: [u8; 0x01],
    _reserved_1_bcnt0: [u8; 0x01],
    _reserved2: [u8; 0x01],
    _reserved_2_bcnt1: [u8; 0x01],
    _reserved3: [u8; 0x01],
    _reserved_3_bcnt2: [u8; 0x01],
    _reserved4: [u8; 0x01],
    _reserved_4_bcnt3: [u8; 0x01],
    _reserved5: [u8; 0x01],
    #[doc = "0x0a - Day Counter"]
    pub rdaycnt: RDAYCNT,
    _reserved6: [u8; 0x01],
    #[doc = "0x0c - Month Counter"]
    pub rmoncnt: RMONCNT,
    _reserved7: [u8; 0x01],
    #[doc = "0x0e - Year Counter"]
    pub ryrcnt: RYRCNT,
    _reserved_8_rsecar: [u8; 0x01],
    _reserved9: [u8; 0x01],
    _reserved_9_rminar: [u8; 0x01],
    _reserved10: [u8; 0x01],
    _reserved_10_rhrar: [u8; 0x01],
    _reserved11: [u8; 0x01],
    _reserved_11_rwkar: [u8; 0x01],
    _reserved12: [u8; 0x01],
    _reserved_12_rdayar: [u8; 0x01],
    _reserved13: [u8; 0x01],
    _reserved_13_rmonar: [u8; 0x01],
    _reserved14: [u8; 0x01],
    _reserved_14_ryrar: [u8; 0x02],
    _reserved_15_ryraren: [u8; 0x01],
    _reserved16: [u8; 0x03],
    #[doc = "0x22 - RTC Control Register 1"]
    pub rcr1: RCR1,
    _reserved17: [u8; 0x01],
    _reserved_17_rcr: [u8; 0x01],
    _reserved18: [u8; 0x03],
    #[doc = "0x28 - RTC Control Register 4"]
    pub rcr4: RCR4,
    _reserved19: [u8; 0x01],
    #[doc = "0x2a - Frequency Register H"]
    pub rfrh: RFRH,
    #[doc = "0x2c - Frequency Register L"]
    pub rfrl: RFRL,
    #[doc = "0x2e - Time Error Adjustment Register"]
    pub radj: RADJ,
}
impl RegisterBlock {
    #[doc = "0x02 - Second Counter (in Calendar Count Mode)"]
    #[inline(always)]
    pub fn rseccnt(&self) -> &RSECCNT {
        unsafe { &*(((self as *const Self) as *const u8).add(2usize) as *const RSECCNT) }
    }
    #[doc = "0x02 - Binary Counter %s"]
    #[inline(always)]
    pub fn bcnt0(&self) -> &BCNT {
        unsafe { &*(((self as *const Self) as *const u8).add(2usize) as *const BCNT) }
    }
    #[doc = "0x04 - Minute Counter (in Calendar Count Mode)"]
    #[inline(always)]
    pub fn rmincnt(&self) -> &RMINCNT {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const RMINCNT) }
    }
    #[doc = "0x04 - Binary Counter %s"]
    #[inline(always)]
    pub fn bcnt1(&self) -> &BCNT {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const BCNT) }
    }
    #[doc = "0x06 - Hour Counter (in Calendar Count Mode)"]
    #[inline(always)]
    pub fn rhrcnt(&self) -> &RHRCNT {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const RHRCNT) }
    }
    #[doc = "0x06 - Binary Counter %s"]
    #[inline(always)]
    pub fn bcnt2(&self) -> &BCNT {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const BCNT) }
    }
    #[doc = "0x08 - Day-of-Week Counter (in Calendar Count Mode)"]
    #[inline(always)]
    pub fn rwkcnt(&self) -> &RWKCNT {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const RWKCNT) }
    }
    #[doc = "0x08 - Binary Counter %s"]
    #[inline(always)]
    pub fn bcnt3(&self) -> &BCNT {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const BCNT) }
    }
    #[doc = "0x10 - Second Alarm Register (in Calendar Count Mode)"]
    #[inline(always)]
    pub fn rsecar(&self) -> &RSECAR {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const RSECAR) }
    }
    #[doc = "0x10 - Binary Counter %s Alarm Register"]
    #[inline(always)]
    pub fn bcnt0ar(&self) -> &BCNTAR {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const BCNTAR) }
    }
    #[doc = "0x12 - Minute Alarm Register (in Calendar Count Mode)"]
    #[inline(always)]
    pub fn rminar(&self) -> &RMINAR {
        unsafe { &*(((self as *const Self) as *const u8).add(18usize) as *const RMINAR) }
    }
    #[doc = "0x12 - Binary Counter %s Alarm Register"]
    #[inline(always)]
    pub fn bcnt1ar(&self) -> &BCNTAR {
        unsafe { &*(((self as *const Self) as *const u8).add(18usize) as *const BCNTAR) }
    }
    #[doc = "0x14 - Hour Alarm Register (in Calendar Count Mode)"]
    #[inline(always)]
    pub fn rhrar(&self) -> &RHRAR {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const RHRAR) }
    }
    #[doc = "0x14 - Binary Counter %s Alarm Register"]
    #[inline(always)]
    pub fn bcnt2ar(&self) -> &BCNTAR {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const BCNTAR) }
    }
    #[doc = "0x16 - Day-of-Week Alarm Register (in Calendar Count Mode)"]
    #[inline(always)]
    pub fn rwkar(&self) -> &RWKAR {
        unsafe { &*(((self as *const Self) as *const u8).add(22usize) as *const RWKAR) }
    }
    #[doc = "0x16 - Binary Counter %s Alarm Register"]
    #[inline(always)]
    pub fn bcnt3ar(&self) -> &BCNTAR {
        unsafe { &*(((self as *const Self) as *const u8).add(22usize) as *const BCNTAR) }
    }
    #[doc = "0x18 - Date Alarm Register (in Calendar Count Mode)"]
    #[inline(always)]
    pub fn rdayar(&self) -> &RDAYAR {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const RDAYAR) }
    }
    #[doc = "0x18 - Binary Counter %s Alarm Enable Register"]
    #[inline(always)]
    pub fn bcnt0aer(&self) -> &BCNTAER {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const BCNTAER) }
    }
    #[doc = "0x1a - Month Alarm Register (in Calendar Count Mode)"]
    #[inline(always)]
    pub fn rmonar(&self) -> &RMONAR {
        unsafe { &*(((self as *const Self) as *const u8).add(26usize) as *const RMONAR) }
    }
    #[doc = "0x1a - Binary Counter %s Alarm Enable Register"]
    #[inline(always)]
    pub fn bcnt1aer(&self) -> &BCNTAER {
        unsafe { &*(((self as *const Self) as *const u8).add(26usize) as *const BCNTAER) }
    }
    #[doc = "0x1c - Year Alarm Register (in Calendar Count Mode)"]
    #[inline(always)]
    pub fn ryrar(&self) -> &RYRAR {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const RYRAR) }
    }
    #[doc = "0x1c - Binary Counter 2 Alarm Enable Register"]
    #[inline(always)]
    pub fn bcnt2aer(&self) -> &BCNT2AER {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const BCNT2AER) }
    }
    #[doc = "0x1e - Year Alarm Enable Register (in Calendar Count Mode)"]
    #[inline(always)]
    pub fn ryraren(&self) -> &RYRAREN {
        unsafe { &*(((self as *const Self) as *const u8).add(30usize) as *const RYRAREN) }
    }
    #[doc = "0x1e - Binary Counter 3 Alarm Enable Register"]
    #[inline(always)]
    pub fn bcnt3aer(&self) -> &BCNT3AER {
        unsafe { &*(((self as *const Self) as *const u8).add(30usize) as *const BCNT3AER) }
    }
    #[doc = "0x24 - RTC Control Register 2 (in Binary Count Mode)"]
    #[inline(always)]
    pub fn rcr2_bcnt(&self) -> &RCR2_BCNT {
        unsafe { &*(((self as *const Self) as *const u8).add(36usize) as *const RCR2_BCNT) }
    }
    #[doc = "0x24 - RTC Control Register 2 (in Calendar Count Mode)"]
    #[inline(always)]
    pub fn rcr2(&self) -> &RCR2 {
        unsafe { &*(((self as *const Self) as *const u8).add(36usize) as *const RCR2) }
    }
}
#[doc = "R64CNT (r) register accessor: an alias for `Reg<R64CNT_SPEC>`"]
pub type R64CNT = crate::Reg<r64cnt::R64CNT_SPEC>;
#[doc = "64-Hz Counter"]
pub mod r64cnt;
#[doc = "BCNT (rw) register accessor: an alias for `Reg<BCNT_SPEC>`"]
pub type BCNT = crate::Reg<bcnt::BCNT_SPEC>;
#[doc = "Binary Counter %s"]
pub mod bcnt;
#[doc = "RSECCNT (rw) register accessor: an alias for `Reg<RSECCNT_SPEC>`"]
pub type RSECCNT = crate::Reg<rseccnt::RSECCNT_SPEC>;
#[doc = "Second Counter (in Calendar Count Mode)"]
pub mod rseccnt;
#[doc = "RMINCNT (rw) register accessor: an alias for `Reg<RMINCNT_SPEC>`"]
pub type RMINCNT = crate::Reg<rmincnt::RMINCNT_SPEC>;
#[doc = "Minute Counter (in Calendar Count Mode)"]
pub mod rmincnt;
#[doc = "RHRCNT (rw) register accessor: an alias for `Reg<RHRCNT_SPEC>`"]
pub type RHRCNT = crate::Reg<rhrcnt::RHRCNT_SPEC>;
#[doc = "Hour Counter (in Calendar Count Mode)"]
pub mod rhrcnt;
#[doc = "RWKCNT (rw) register accessor: an alias for `Reg<RWKCNT_SPEC>`"]
pub type RWKCNT = crate::Reg<rwkcnt::RWKCNT_SPEC>;
#[doc = "Day-of-Week Counter (in Calendar Count Mode)"]
pub mod rwkcnt;
#[doc = "RDAYCNT (rw) register accessor: an alias for `Reg<RDAYCNT_SPEC>`"]
pub type RDAYCNT = crate::Reg<rdaycnt::RDAYCNT_SPEC>;
#[doc = "Day Counter"]
pub mod rdaycnt;
#[doc = "RMONCNT (rw) register accessor: an alias for `Reg<RMONCNT_SPEC>`"]
pub type RMONCNT = crate::Reg<rmoncnt::RMONCNT_SPEC>;
#[doc = "Month Counter"]
pub mod rmoncnt;
#[doc = "RYRCNT (rw) register accessor: an alias for `Reg<RYRCNT_SPEC>`"]
pub type RYRCNT = crate::Reg<ryrcnt::RYRCNT_SPEC>;
#[doc = "Year Counter"]
pub mod ryrcnt;
#[doc = "BCNTAR (rw) register accessor: an alias for `Reg<BCNTAR_SPEC>`"]
pub type BCNTAR = crate::Reg<bcntar::BCNTAR_SPEC>;
#[doc = "Binary Counter %s Alarm Register"]
pub mod bcntar;
#[doc = "RSECAR (rw) register accessor: an alias for `Reg<RSECAR_SPEC>`"]
pub type RSECAR = crate::Reg<rsecar::RSECAR_SPEC>;
#[doc = "Second Alarm Register (in Calendar Count Mode)"]
pub mod rsecar;
#[doc = "RMINAR (rw) register accessor: an alias for `Reg<RMINAR_SPEC>`"]
pub type RMINAR = crate::Reg<rminar::RMINAR_SPEC>;
#[doc = "Minute Alarm Register (in Calendar Count Mode)"]
pub mod rminar;
#[doc = "RHRAR (rw) register accessor: an alias for `Reg<RHRAR_SPEC>`"]
pub type RHRAR = crate::Reg<rhrar::RHRAR_SPEC>;
#[doc = "Hour Alarm Register (in Calendar Count Mode)"]
pub mod rhrar;
#[doc = "RWKAR (rw) register accessor: an alias for `Reg<RWKAR_SPEC>`"]
pub type RWKAR = crate::Reg<rwkar::RWKAR_SPEC>;
#[doc = "Day-of-Week Alarm Register (in Calendar Count Mode)"]
pub mod rwkar;
#[doc = "BCNTAER (rw) register accessor: an alias for `Reg<BCNTAER_SPEC>`"]
pub type BCNTAER = crate::Reg<bcntaer::BCNTAER_SPEC>;
#[doc = "Binary Counter %s Alarm Enable Register"]
pub mod bcntaer;
#[doc = "RDAYAR (rw) register accessor: an alias for `Reg<RDAYAR_SPEC>`"]
pub type RDAYAR = crate::Reg<rdayar::RDAYAR_SPEC>;
#[doc = "Date Alarm Register (in Calendar Count Mode)"]
pub mod rdayar;
#[doc = "RMONAR (rw) register accessor: an alias for `Reg<RMONAR_SPEC>`"]
pub type RMONAR = crate::Reg<rmonar::RMONAR_SPEC>;
#[doc = "Month Alarm Register (in Calendar Count Mode)"]
pub mod rmonar;
#[doc = "BCNT2AER (rw) register accessor: an alias for `Reg<BCNT2AER_SPEC>`"]
pub type BCNT2AER = crate::Reg<bcnt2aer::BCNT2AER_SPEC>;
#[doc = "Binary Counter 2 Alarm Enable Register"]
pub mod bcnt2aer;
#[doc = "RYRAR (rw) register accessor: an alias for `Reg<RYRAR_SPEC>`"]
pub type RYRAR = crate::Reg<ryrar::RYRAR_SPEC>;
#[doc = "Year Alarm Register (in Calendar Count Mode)"]
pub mod ryrar;
#[doc = "BCNT3AER (rw) register accessor: an alias for `Reg<BCNT3AER_SPEC>`"]
pub type BCNT3AER = crate::Reg<bcnt3aer::BCNT3AER_SPEC>;
#[doc = "Binary Counter 3 Alarm Enable Register"]
pub mod bcnt3aer;
#[doc = "RYRAREN (rw) register accessor: an alias for `Reg<RYRAREN_SPEC>`"]
pub type RYRAREN = crate::Reg<ryraren::RYRAREN_SPEC>;
#[doc = "Year Alarm Enable Register (in Calendar Count Mode)"]
pub mod ryraren;
#[doc = "RCR1 (rw) register accessor: an alias for `Reg<RCR1_SPEC>`"]
pub type RCR1 = crate::Reg<rcr1::RCR1_SPEC>;
#[doc = "RTC Control Register 1"]
pub mod rcr1;
#[doc = "RCR2 (rw) register accessor: an alias for `Reg<RCR2_SPEC>`"]
pub type RCR2 = crate::Reg<rcr2::RCR2_SPEC>;
#[doc = "RTC Control Register 2 (in Calendar Count Mode)"]
pub mod rcr2;
#[doc = "RCR2_BCNT (rw) register accessor: an alias for `Reg<RCR2_BCNT_SPEC>`"]
pub type RCR2_BCNT = crate::Reg<rcr2_bcnt::RCR2_BCNT_SPEC>;
#[doc = "RTC Control Register 2 (in Binary Count Mode)"]
pub mod rcr2_bcnt;
#[doc = "RCR4 (rw) register accessor: an alias for `Reg<RCR4_SPEC>`"]
pub type RCR4 = crate::Reg<rcr4::RCR4_SPEC>;
#[doc = "RTC Control Register 4"]
pub mod rcr4;
#[doc = "RFRH (rw) register accessor: an alias for `Reg<RFRH_SPEC>`"]
pub type RFRH = crate::Reg<rfrh::RFRH_SPEC>;
#[doc = "Frequency Register H"]
pub mod rfrh;
#[doc = "RFRL (rw) register accessor: an alias for `Reg<RFRL_SPEC>`"]
pub type RFRL = crate::Reg<rfrl::RFRL_SPEC>;
#[doc = "Frequency Register L"]
pub mod rfrl;
#[doc = "RADJ (rw) register accessor: an alias for `Reg<RADJ_SPEC>`"]
pub type RADJ = crate::Reg<radj::RADJ_SPEC>;
#[doc = "Time Error Adjustment Register"]
pub mod radj;
