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
    _reserved22: [u8; 0x11],
    #[doc = "0x40 - Time Capture Control Register %s"]
    pub rtccr0: RTCCR,
    _reserved23: [u8; 0x01],
    #[doc = "0x42 - Time Capture Control Register %s"]
    pub rtccr1: RTCCR,
    _reserved24: [u8; 0x01],
    #[doc = "0x44 - Time Capture Control Register %s"]
    pub rtccr2: RTCCR,
    _reserved25: [u8; 0x0d],
    _reserved_25_rseccp0: [u8; 0x01],
    _reserved26: [u8; 0x01],
    _reserved_26_rmincp0: [u8; 0x01],
    _reserved27: [u8; 0x01],
    _reserved_27_rhrcp0: [u8; 0x01],
    _reserved28: [u8; 0x03],
    _reserved_28_rdaycp0: [u8; 0x01],
    _reserved29: [u8; 0x01],
    #[doc = "0x5c - Month Capture Register %s"]
    pub rmoncp0: RMONCP,
    _reserved30: [u8; 0x05],
    _reserved_30_rseccp1: [u8; 0x01],
    _reserved31: [u8; 0x01],
    _reserved_31_rmincp1: [u8; 0x01],
    _reserved32: [u8; 0x01],
    _reserved_32_rhrcp1: [u8; 0x01],
    _reserved33: [u8; 0x03],
    _reserved_33_rdaycp1: [u8; 0x01],
    _reserved34: [u8; 0x01],
    #[doc = "0x6c - Month Capture Register %s"]
    pub rmoncp1: RMONCP,
    _reserved35: [u8; 0x05],
    _reserved_35_rseccp2: [u8; 0x01],
    _reserved36: [u8; 0x01],
    _reserved_36_rmincp2: [u8; 0x01],
    _reserved37: [u8; 0x01],
    _reserved_37_rhrcp2: [u8; 0x01],
    _reserved38: [u8; 0x03],
    _reserved_38_rdaycp2: [u8; 0x01],
    _reserved39: [u8; 0x01],
    #[doc = "0x7c - Month Capture Register %s"]
    pub rmoncp2: RMONCP,
}
impl RegisterBlock {
    #[doc = "0x02 - Second Counter (in Calendar Count Mode)"]
    #[inline(always)]
    pub const fn rseccnt(&self) -> &RSECCNT {
        unsafe { &*(self as *const Self).cast::<u8>().add(2usize).cast() }
    }
    #[doc = "0x02 - Binary Counter %s"]
    #[inline(always)]
    pub const fn bcnt0(&self) -> &BCNT {
        unsafe { &*(self as *const Self).cast::<u8>().add(2usize).cast() }
    }
    #[doc = "0x04 - Minute Counter (in Calendar Count Mode)"]
    #[inline(always)]
    pub const fn rmincnt(&self) -> &RMINCNT {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - Binary Counter %s"]
    #[inline(always)]
    pub const fn bcnt1(&self) -> &BCNT {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x06 - Hour Counter (in Calendar Count Mode)"]
    #[inline(always)]
    pub const fn rhrcnt(&self) -> &RHRCNT {
        unsafe { &*(self as *const Self).cast::<u8>().add(6usize).cast() }
    }
    #[doc = "0x06 - Binary Counter %s"]
    #[inline(always)]
    pub const fn bcnt2(&self) -> &BCNT {
        unsafe { &*(self as *const Self).cast::<u8>().add(6usize).cast() }
    }
    #[doc = "0x08 - Day-of-Week Counter (in Calendar Count Mode)"]
    #[inline(always)]
    pub const fn rwkcnt(&self) -> &RWKCNT {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x08 - Binary Counter %s"]
    #[inline(always)]
    pub const fn bcnt3(&self) -> &BCNT {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x10 - Second Alarm Register (in Calendar Count Mode)"]
    #[inline(always)]
    pub const fn rsecar(&self) -> &RSECAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x10 - Binary Counter %s Alarm Register"]
    #[inline(always)]
    pub const fn bcnt0ar(&self) -> &BCNTAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x12 - Minute Alarm Register (in Calendar Count Mode)"]
    #[inline(always)]
    pub const fn rminar(&self) -> &RMINAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(18usize).cast() }
    }
    #[doc = "0x12 - Binary Counter %s Alarm Register"]
    #[inline(always)]
    pub const fn bcnt1ar(&self) -> &BCNTAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(18usize).cast() }
    }
    #[doc = "0x14 - Hour Alarm Register (in Calendar Count Mode)"]
    #[inline(always)]
    pub const fn rhrar(&self) -> &RHRAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(20usize).cast() }
    }
    #[doc = "0x14 - Binary Counter %s Alarm Register"]
    #[inline(always)]
    pub const fn bcnt2ar(&self) -> &BCNTAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(20usize).cast() }
    }
    #[doc = "0x16 - Day-of-Week Alarm Register (in Calendar Count Mode)"]
    #[inline(always)]
    pub const fn rwkar(&self) -> &RWKAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(22usize).cast() }
    }
    #[doc = "0x16 - Binary Counter %s Alarm Register"]
    #[inline(always)]
    pub const fn bcnt3ar(&self) -> &BCNTAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(22usize).cast() }
    }
    #[doc = "0x18 - Date Alarm Register (in Calendar Count Mode)"]
    #[inline(always)]
    pub const fn rdayar(&self) -> &RDAYAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x18 - Binary Counter %s Alarm Enable Register"]
    #[inline(always)]
    pub const fn bcnt0aer(&self) -> &BCNTAER {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x1a - Month Alarm Register (in Calendar Count Mode)"]
    #[inline(always)]
    pub const fn rmonar(&self) -> &RMONAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(26usize).cast() }
    }
    #[doc = "0x1a - Binary Counter %s Alarm Enable Register"]
    #[inline(always)]
    pub const fn bcnt1aer(&self) -> &BCNTAER {
        unsafe { &*(self as *const Self).cast::<u8>().add(26usize).cast() }
    }
    #[doc = "0x1c - Year Alarm Register (in Calendar Count Mode)"]
    #[inline(always)]
    pub const fn ryrar(&self) -> &RYRAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1c - Binary Counter 2 Alarm Enable Register"]
    #[inline(always)]
    pub const fn bcnt2aer(&self) -> &BCNT2AER {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1e - Year Alarm Enable Register (in Calendar Count Mode)"]
    #[inline(always)]
    pub const fn ryraren(&self) -> &RYRAREN {
        unsafe { &*(self as *const Self).cast::<u8>().add(30usize).cast() }
    }
    #[doc = "0x1e - Binary Counter 3 Alarm Enable Register"]
    #[inline(always)]
    pub const fn bcnt3aer(&self) -> &BCNT3AER {
        unsafe { &*(self as *const Self).cast::<u8>().add(30usize).cast() }
    }
    #[doc = "0x24 - RTC Control Register 2 (in Binary Count Mode)"]
    #[inline(always)]
    pub const fn rcr2_bcnt(&self) -> &RCR2_BCNT {
        unsafe { &*(self as *const Self).cast::<u8>().add(36usize).cast() }
    }
    #[doc = "0x24 - RTC Control Register 2 (in Calendar Count Mode)"]
    #[inline(always)]
    pub const fn rcr2(&self) -> &RCR2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(36usize).cast() }
    }
    #[doc = "0x52 - Second Capture Register %s"]
    #[inline(always)]
    pub const fn rseccp0(&self) -> &RSECCP {
        unsafe { &*(self as *const Self).cast::<u8>().add(82usize).cast() }
    }
    #[doc = "0x52 - BCNT0 Capture Register %s"]
    #[inline(always)]
    pub const fn bcnt0cp0(&self) -> &BCNT0CP {
        unsafe { &*(self as *const Self).cast::<u8>().add(82usize).cast() }
    }
    #[doc = "0x54 - Minute Capture Register %s"]
    #[inline(always)]
    pub const fn rmincp0(&self) -> &RMINCP {
        unsafe { &*(self as *const Self).cast::<u8>().add(84usize).cast() }
    }
    #[doc = "0x54 - BCNT1 Capture Register %s"]
    #[inline(always)]
    pub const fn bcnt1cp0(&self) -> &BCNT1CP {
        unsafe { &*(self as *const Self).cast::<u8>().add(84usize).cast() }
    }
    #[doc = "0x56 - Hour Capture Register %s"]
    #[inline(always)]
    pub const fn rhrcp0(&self) -> &RHRCP {
        unsafe { &*(self as *const Self).cast::<u8>().add(86usize).cast() }
    }
    #[doc = "0x56 - BCNT2 Capture Register %s"]
    #[inline(always)]
    pub const fn bcnt2cp0(&self) -> &BCNT2CP {
        unsafe { &*(self as *const Self).cast::<u8>().add(86usize).cast() }
    }
    #[doc = "0x5a - Date Capture Register %s"]
    #[inline(always)]
    pub const fn rdaycp0(&self) -> &RDAYCP {
        unsafe { &*(self as *const Self).cast::<u8>().add(90usize).cast() }
    }
    #[doc = "0x5a - BCNT3 Capture Register %s"]
    #[inline(always)]
    pub const fn bcnt3cp0(&self) -> &BCNT3CP {
        unsafe { &*(self as *const Self).cast::<u8>().add(90usize).cast() }
    }
    #[doc = "0x62 - Second Capture Register %s"]
    #[inline(always)]
    pub const fn rseccp1(&self) -> &RSECCP {
        unsafe { &*(self as *const Self).cast::<u8>().add(98usize).cast() }
    }
    #[doc = "0x62 - BCNT0 Capture Register %s"]
    #[inline(always)]
    pub const fn bcnt0cp1(&self) -> &BCNT0CP {
        unsafe { &*(self as *const Self).cast::<u8>().add(98usize).cast() }
    }
    #[doc = "0x64 - Minute Capture Register %s"]
    #[inline(always)]
    pub const fn rmincp1(&self) -> &RMINCP {
        unsafe { &*(self as *const Self).cast::<u8>().add(100usize).cast() }
    }
    #[doc = "0x64 - BCNT1 Capture Register %s"]
    #[inline(always)]
    pub const fn bcnt1cp1(&self) -> &BCNT1CP {
        unsafe { &*(self as *const Self).cast::<u8>().add(100usize).cast() }
    }
    #[doc = "0x66 - Hour Capture Register %s"]
    #[inline(always)]
    pub const fn rhrcp1(&self) -> &RHRCP {
        unsafe { &*(self as *const Self).cast::<u8>().add(102usize).cast() }
    }
    #[doc = "0x66 - BCNT2 Capture Register %s"]
    #[inline(always)]
    pub const fn bcnt2cp1(&self) -> &BCNT2CP {
        unsafe { &*(self as *const Self).cast::<u8>().add(102usize).cast() }
    }
    #[doc = "0x6a - Date Capture Register %s"]
    #[inline(always)]
    pub const fn rdaycp1(&self) -> &RDAYCP {
        unsafe { &*(self as *const Self).cast::<u8>().add(106usize).cast() }
    }
    #[doc = "0x6a - BCNT3 Capture Register %s"]
    #[inline(always)]
    pub const fn bcnt3cp1(&self) -> &BCNT3CP {
        unsafe { &*(self as *const Self).cast::<u8>().add(106usize).cast() }
    }
    #[doc = "0x72 - Second Capture Register %s"]
    #[inline(always)]
    pub const fn rseccp2(&self) -> &RSECCP {
        unsafe { &*(self as *const Self).cast::<u8>().add(114usize).cast() }
    }
    #[doc = "0x72 - BCNT0 Capture Register %s"]
    #[inline(always)]
    pub const fn bcnt0cp2(&self) -> &BCNT0CP {
        unsafe { &*(self as *const Self).cast::<u8>().add(114usize).cast() }
    }
    #[doc = "0x74 - Minute Capture Register %s"]
    #[inline(always)]
    pub const fn rmincp2(&self) -> &RMINCP {
        unsafe { &*(self as *const Self).cast::<u8>().add(116usize).cast() }
    }
    #[doc = "0x74 - BCNT1 Capture Register %s"]
    #[inline(always)]
    pub const fn bcnt1cp2(&self) -> &BCNT1CP {
        unsafe { &*(self as *const Self).cast::<u8>().add(116usize).cast() }
    }
    #[doc = "0x76 - Hour Capture Register %s"]
    #[inline(always)]
    pub const fn rhrcp2(&self) -> &RHRCP {
        unsafe { &*(self as *const Self).cast::<u8>().add(118usize).cast() }
    }
    #[doc = "0x76 - BCNT2 Capture Register %s"]
    #[inline(always)]
    pub const fn bcnt2cp2(&self) -> &BCNT2CP {
        unsafe { &*(self as *const Self).cast::<u8>().add(118usize).cast() }
    }
    #[doc = "0x7a - Date Capture Register %s"]
    #[inline(always)]
    pub const fn rdaycp2(&self) -> &RDAYCP {
        unsafe { &*(self as *const Self).cast::<u8>().add(122usize).cast() }
    }
    #[doc = "0x7a - BCNT3 Capture Register %s"]
    #[inline(always)]
    pub const fn bcnt3cp2(&self) -> &BCNT3CP {
        unsafe { &*(self as *const Self).cast::<u8>().add(122usize).cast() }
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
pub use bcntaer as bcnt3aer;
pub use BCNTAER as BCNT3AER;
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
#[doc = "RTCCR (rw) register accessor: an alias for `Reg<RTCCR_SPEC>`"]
pub type RTCCR = crate::Reg<rtccr::RTCCR_SPEC>;
#[doc = "Time Capture Control Register %s"]
pub mod rtccr;
#[doc = "BCNT0CP (r) register accessor: an alias for `Reg<BCNT0CP_SPEC>`"]
pub type BCNT0CP = crate::Reg<bcnt0cp::BCNT0CP_SPEC>;
#[doc = "BCNT0 Capture Register %s"]
pub mod bcnt0cp;
#[doc = "RSECCP (r) register accessor: an alias for `Reg<RSECCP_SPEC>`"]
pub type RSECCP = crate::Reg<rseccp::RSECCP_SPEC>;
#[doc = "Second Capture Register %s"]
pub mod rseccp;
#[doc = "BCNT1CP (r) register accessor: an alias for `Reg<BCNT1CP_SPEC>`"]
pub type BCNT1CP = crate::Reg<bcnt1cp::BCNT1CP_SPEC>;
#[doc = "BCNT1 Capture Register %s"]
pub mod bcnt1cp;
#[doc = "RMINCP (r) register accessor: an alias for `Reg<RMINCP_SPEC>`"]
pub type RMINCP = crate::Reg<rmincp::RMINCP_SPEC>;
#[doc = "Minute Capture Register %s"]
pub mod rmincp;
#[doc = "BCNT2CP (r) register accessor: an alias for `Reg<BCNT2CP_SPEC>`"]
pub type BCNT2CP = crate::Reg<bcnt2cp::BCNT2CP_SPEC>;
#[doc = "BCNT2 Capture Register %s"]
pub mod bcnt2cp;
#[doc = "RHRCP (r) register accessor: an alias for `Reg<RHRCP_SPEC>`"]
pub type RHRCP = crate::Reg<rhrcp::RHRCP_SPEC>;
#[doc = "Hour Capture Register %s"]
pub mod rhrcp;
#[doc = "BCNT3CP (r) register accessor: an alias for `Reg<BCNT3CP_SPEC>`"]
pub type BCNT3CP = crate::Reg<bcnt3cp::BCNT3CP_SPEC>;
#[doc = "BCNT3 Capture Register %s"]
pub mod bcnt3cp;
#[doc = "RDAYCP (r) register accessor: an alias for `Reg<RDAYCP_SPEC>`"]
pub type RDAYCP = crate::Reg<rdaycp::RDAYCP_SPEC>;
#[doc = "Date Capture Register %s"]
pub mod rdaycp;
#[doc = "RMONCP (r) register accessor: an alias for `Reg<RMONCP_SPEC>`"]
pub type RMONCP = crate::Reg<rmoncp::RMONCP_SPEC>;
#[doc = "Month Capture Register %s"]
pub mod rmoncp;
