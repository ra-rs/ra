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
    #[doc = "0x24 - RTC Control Register 2"]
    pub rcr2: RCR2,
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
    #[doc = "0x02 - Binary Counter 0"]
    #[inline(always)]
    pub const fn bcnt0(&self) -> &BCNT0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2usize).cast() }
    }
    #[doc = "0x02 - Second Counter"]
    #[inline(always)]
    pub const fn rseccnt(&self) -> &RSECCNT {
        unsafe { &*(self as *const Self).cast::<u8>().add(2usize).cast() }
    }
    #[doc = "0x04 - Binary Counter 1"]
    #[inline(always)]
    pub const fn bcnt1(&self) -> &BCNT1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - Minute Counter"]
    #[inline(always)]
    pub const fn rmincnt(&self) -> &RMINCNT {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x06 - Binary Counter 2"]
    #[inline(always)]
    pub const fn bcnt2(&self) -> &BCNT2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(6usize).cast() }
    }
    #[doc = "0x06 - Hour Counter"]
    #[inline(always)]
    pub const fn rhrcnt(&self) -> &RHRCNT {
        unsafe { &*(self as *const Self).cast::<u8>().add(6usize).cast() }
    }
    #[doc = "0x08 - Binary Counter 3"]
    #[inline(always)]
    pub const fn bcnt3(&self) -> &BCNT3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x08 - Day-of-Week Counter"]
    #[inline(always)]
    pub const fn rwkcnt(&self) -> &RWKCNT {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x10 - Binary Counter 0 Alarm Register"]
    #[inline(always)]
    pub const fn bcnt0ar(&self) -> &BCNT0AR {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x10 - Second Alarm Register"]
    #[inline(always)]
    pub const fn rsecar(&self) -> &RSECAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x12 - Binary Counter 1 Alarm Register"]
    #[inline(always)]
    pub const fn bcnt1ar(&self) -> &BCNT1AR {
        unsafe { &*(self as *const Self).cast::<u8>().add(18usize).cast() }
    }
    #[doc = "0x12 - Minute Alarm Register"]
    #[inline(always)]
    pub const fn rminar(&self) -> &RMINAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(18usize).cast() }
    }
    #[doc = "0x14 - Binary Counter 2 Alarm Register"]
    #[inline(always)]
    pub const fn bcnt2ar(&self) -> &BCNT2AR {
        unsafe { &*(self as *const Self).cast::<u8>().add(20usize).cast() }
    }
    #[doc = "0x14 - Hour Alarm Register"]
    #[inline(always)]
    pub const fn rhrar(&self) -> &RHRAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(20usize).cast() }
    }
    #[doc = "0x16 - Binary Counter 3 Alarm Register"]
    #[inline(always)]
    pub const fn bcnt3ar(&self) -> &BCNT3AR {
        unsafe { &*(self as *const Self).cast::<u8>().add(22usize).cast() }
    }
    #[doc = "0x16 - Day-of-Week Alarm Register"]
    #[inline(always)]
    pub const fn rwkar(&self) -> &RWKAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(22usize).cast() }
    }
    #[doc = "0x18 - Binary Counter 0 Alarm Enable Register"]
    #[inline(always)]
    pub const fn bcnt0aer(&self) -> &BCNT0AER {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x18 - Date Alarm Register"]
    #[inline(always)]
    pub const fn rdayar(&self) -> &RDAYAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x1a - Binary Counter 1 Alarm Enable Register"]
    #[inline(always)]
    pub const fn bcnt1aer(&self) -> &BCNT1AER {
        unsafe { &*(self as *const Self).cast::<u8>().add(26usize).cast() }
    }
    #[doc = "0x1a - Month Alarm Register"]
    #[inline(always)]
    pub const fn rmonar(&self) -> &RMONAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(26usize).cast() }
    }
    #[doc = "0x1c - Binary Counter 2 Alarm Enable Register"]
    #[inline(always)]
    pub const fn bcnt2aer(&self) -> &BCNT2AER {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1c - Year Alarm Register"]
    #[inline(always)]
    pub const fn ryrar(&self) -> &RYRAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1e - Binary Counter 3 Alarm Enable Register"]
    #[inline(always)]
    pub const fn bcnt3aer(&self) -> &BCNT3AER {
        unsafe { &*(self as *const Self).cast::<u8>().add(30usize).cast() }
    }
    #[doc = "0x1e - Year Alarm Enable Register"]
    #[inline(always)]
    pub const fn ryraren(&self) -> &RYRAREN {
        unsafe { &*(self as *const Self).cast::<u8>().add(30usize).cast() }
    }
}
#[doc = "R64CNT (r) register accessor: an alias for `Reg<R64CNT_SPEC>`"]
pub type R64CNT = crate::Reg<r64cnt::R64CNT_SPEC>;
#[doc = "64-Hz Counter"]
pub mod r64cnt;
#[doc = "RSECCNT (rw) register accessor: an alias for `Reg<RSECCNT_SPEC>`"]
pub type RSECCNT = crate::Reg<rseccnt::RSECCNT_SPEC>;
#[doc = "Second Counter"]
pub mod rseccnt;
#[doc = "BCNT0 (rw) register accessor: an alias for `Reg<BCNT0_SPEC>`"]
pub type BCNT0 = crate::Reg<bcnt0::BCNT0_SPEC>;
#[doc = "Binary Counter 0"]
pub mod bcnt0;
#[doc = "RMINCNT (rw) register accessor: an alias for `Reg<RMINCNT_SPEC>`"]
pub type RMINCNT = crate::Reg<rmincnt::RMINCNT_SPEC>;
#[doc = "Minute Counter"]
pub mod rmincnt;
#[doc = "BCNT1 (rw) register accessor: an alias for `Reg<BCNT1_SPEC>`"]
pub type BCNT1 = crate::Reg<bcnt1::BCNT1_SPEC>;
#[doc = "Binary Counter 1"]
pub mod bcnt1;
#[doc = "RHRCNT (rw) register accessor: an alias for `Reg<RHRCNT_SPEC>`"]
pub type RHRCNT = crate::Reg<rhrcnt::RHRCNT_SPEC>;
#[doc = "Hour Counter"]
pub mod rhrcnt;
#[doc = "BCNT2 (rw) register accessor: an alias for `Reg<BCNT2_SPEC>`"]
pub type BCNT2 = crate::Reg<bcnt2::BCNT2_SPEC>;
#[doc = "Binary Counter 2"]
pub mod bcnt2;
#[doc = "RWKCNT (rw) register accessor: an alias for `Reg<RWKCNT_SPEC>`"]
pub type RWKCNT = crate::Reg<rwkcnt::RWKCNT_SPEC>;
#[doc = "Day-of-Week Counter"]
pub mod rwkcnt;
#[doc = "BCNT3 (rw) register accessor: an alias for `Reg<BCNT3_SPEC>`"]
pub type BCNT3 = crate::Reg<bcnt3::BCNT3_SPEC>;
#[doc = "Binary Counter 3"]
pub mod bcnt3;
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
#[doc = "RSECAR (rw) register accessor: an alias for `Reg<RSECAR_SPEC>`"]
pub type RSECAR = crate::Reg<rsecar::RSECAR_SPEC>;
#[doc = "Second Alarm Register"]
pub mod rsecar;
#[doc = "BCNT0AR (rw) register accessor: an alias for `Reg<BCNT0AR_SPEC>`"]
pub type BCNT0AR = crate::Reg<bcnt0ar::BCNT0AR_SPEC>;
#[doc = "Binary Counter 0 Alarm Register"]
pub mod bcnt0ar;
#[doc = "RMINAR (rw) register accessor: an alias for `Reg<RMINAR_SPEC>`"]
pub type RMINAR = crate::Reg<rminar::RMINAR_SPEC>;
#[doc = "Minute Alarm Register"]
pub mod rminar;
#[doc = "BCNT1AR (rw) register accessor: an alias for `Reg<BCNT1AR_SPEC>`"]
pub type BCNT1AR = crate::Reg<bcnt1ar::BCNT1AR_SPEC>;
#[doc = "Binary Counter 1 Alarm Register"]
pub mod bcnt1ar;
#[doc = "RHRAR (rw) register accessor: an alias for `Reg<RHRAR_SPEC>`"]
pub type RHRAR = crate::Reg<rhrar::RHRAR_SPEC>;
#[doc = "Hour Alarm Register"]
pub mod rhrar;
#[doc = "BCNT2AR (rw) register accessor: an alias for `Reg<BCNT2AR_SPEC>`"]
pub type BCNT2AR = crate::Reg<bcnt2ar::BCNT2AR_SPEC>;
#[doc = "Binary Counter 2 Alarm Register"]
pub mod bcnt2ar;
#[doc = "RWKAR (rw) register accessor: an alias for `Reg<RWKAR_SPEC>`"]
pub type RWKAR = crate::Reg<rwkar::RWKAR_SPEC>;
#[doc = "Day-of-Week Alarm Register"]
pub mod rwkar;
#[doc = "BCNT3AR (rw) register accessor: an alias for `Reg<BCNT3AR_SPEC>`"]
pub type BCNT3AR = crate::Reg<bcnt3ar::BCNT3AR_SPEC>;
#[doc = "Binary Counter 3 Alarm Register"]
pub mod bcnt3ar;
#[doc = "RDAYAR (rw) register accessor: an alias for `Reg<RDAYAR_SPEC>`"]
pub type RDAYAR = crate::Reg<rdayar::RDAYAR_SPEC>;
#[doc = "Date Alarm Register"]
pub mod rdayar;
#[doc = "BCNT0AER (rw) register accessor: an alias for `Reg<BCNT0AER_SPEC>`"]
pub type BCNT0AER = crate::Reg<bcnt0aer::BCNT0AER_SPEC>;
#[doc = "Binary Counter 0 Alarm Enable Register"]
pub mod bcnt0aer;
#[doc = "RMONAR (rw) register accessor: an alias for `Reg<RMONAR_SPEC>`"]
pub type RMONAR = crate::Reg<rmonar::RMONAR_SPEC>;
#[doc = "Month Alarm Register"]
pub mod rmonar;
#[doc = "BCNT1AER (rw) register accessor: an alias for `Reg<BCNT1AER_SPEC>`"]
pub type BCNT1AER = crate::Reg<bcnt1aer::BCNT1AER_SPEC>;
#[doc = "Binary Counter 1 Alarm Enable Register"]
pub mod bcnt1aer;
#[doc = "RYRAR (rw) register accessor: an alias for `Reg<RYRAR_SPEC>`"]
pub type RYRAR = crate::Reg<ryrar::RYRAR_SPEC>;
#[doc = "Year Alarm Register"]
pub mod ryrar;
#[doc = "BCNT2AER (rw) register accessor: an alias for `Reg<BCNT2AER_SPEC>`"]
pub type BCNT2AER = crate::Reg<bcnt2aer::BCNT2AER_SPEC>;
#[doc = "Binary Counter 2 Alarm Enable Register"]
pub mod bcnt2aer;
#[doc = "RYRAREN (rw) register accessor: an alias for `Reg<RYRAREN_SPEC>`"]
pub type RYRAREN = crate::Reg<ryraren::RYRAREN_SPEC>;
#[doc = "Year Alarm Enable Register"]
pub mod ryraren;
#[doc = "BCNT3AER (rw) register accessor: an alias for `Reg<BCNT3AER_SPEC>`"]
pub type BCNT3AER = crate::Reg<bcnt3aer::BCNT3AER_SPEC>;
#[doc = "Binary Counter 3 Alarm Enable Register"]
pub mod bcnt3aer;
#[doc = "RCR1 (rw) register accessor: an alias for `Reg<RCR1_SPEC>`"]
pub type RCR1 = crate::Reg<rcr1::RCR1_SPEC>;
#[doc = "RTC Control Register 1"]
pub mod rcr1;
#[doc = "RCR2 (rw) register accessor: an alias for `Reg<RCR2_SPEC>`"]
pub type RCR2 = crate::Reg<rcr2::RCR2_SPEC>;
#[doc = "RTC Control Register 2"]
pub mod rcr2;
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
