#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_ctsucr0: [u8; 0x04],
    _reserved_1_ctsucrb: [u8; 0x04],
    _reserved_2_ctsumch: [u8; 0x04],
    _reserved_3_ctsuchac0: [u8; 0x04],
    _reserved_4_ctsuchac4: [u8; 0x04],
    _reserved_5_ctsuchtrc0: [u8; 0x04],
    _reserved_6_ctsuchtrc4: [u8; 0x04],
    _reserved_7_ctsusr: [u8; 0x04],
    _reserved_8_ctsuso: [u8; 0x04],
    _reserved_9_ctsusc: [u8; 0x04],
    _reserved_10_ctsucalib: [u8; 0x04],
    _reserved_11_ctsusuclk0: [u8; 0x04],
    _reserved_12_ctsusuclk2: [u8; 0x04],
    _reserved_13_ctsucfccnt: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - CTSU Control Register A"]
    #[inline(always)]
    pub const fn ctsucr0(&self) -> &CTSUCR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - CTSU Control Register A"]
    #[inline(always)]
    pub const fn ctsucral(&self) -> &CTSUCRAL {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - CTSU Control Register A"]
    #[inline(always)]
    pub const fn ctsucra(&self) -> &CTSUCRA {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x01 - CTSU Control Register A"]
    #[inline(always)]
    pub const fn ctsucr1(&self) -> &CTSUCR1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1usize).cast() }
    }
    #[doc = "0x02 - CTSU Control Register A"]
    #[inline(always)]
    pub const fn ctsucr2(&self) -> &CTSUCR2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2usize).cast() }
    }
    #[doc = "0x03 - CTSU Control Register A"]
    #[inline(always)]
    pub const fn ctsucr3(&self) -> &CTSUCR3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3usize).cast() }
    }
    #[doc = "0x04 - CTSU Control Register B"]
    #[inline(always)]
    pub const fn ctsusdprs(&self) -> &CTSUSDPRS {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - CTSU Control Register B"]
    #[inline(always)]
    pub const fn ctsucrbl(&self) -> &CTSUCRBL {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - CTSU Control Register B"]
    #[inline(always)]
    pub const fn ctsucrb(&self) -> &CTSUCRB {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x05 - CTSU Control Register B"]
    #[inline(always)]
    pub const fn ctsusst(&self) -> &CTSUSST {
        unsafe { &*(self as *const Self).cast::<u8>().add(5usize).cast() }
    }
    #[doc = "0x06 - CTSU Control Register B"]
    #[inline(always)]
    pub const fn ctsucrbh(&self) -> &CTSUCRBH {
        unsafe { &*(self as *const Self).cast::<u8>().add(6usize).cast() }
    }
    #[doc = "0x07 - CTSU Control Register B"]
    #[inline(always)]
    pub const fn ctsudclkc(&self) -> &CTSUDCLKC {
        unsafe { &*(self as *const Self).cast::<u8>().add(7usize).cast() }
    }
    #[doc = "0x08 - CTSU Measurement Channel Register"]
    #[inline(always)]
    pub const fn ctsumch0(&self) -> &CTSUMCH0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x08 - CTSU Measurement Channel Register"]
    #[inline(always)]
    pub const fn ctsumchl(&self) -> &CTSUMCHL {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x08 - CTSU Measurement Channel Register"]
    #[inline(always)]
    pub const fn ctsumch(&self) -> &CTSUMCH {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x09 - CTSU Measurement Channel Register"]
    #[inline(always)]
    pub const fn ctsumch1(&self) -> &CTSUMCH1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(9usize).cast() }
    }
    #[doc = "0x0a - CTSU Measurement Channel Register"]
    #[inline(always)]
    pub const fn ctsumfaf(&self) -> &CTSUMFAF {
        unsafe { &*(self as *const Self).cast::<u8>().add(10usize).cast() }
    }
    #[doc = "0x0a - CTSU Measurement Channel Register"]
    #[inline(always)]
    pub const fn ctsumchh(&self) -> &CTSUMCHH {
        unsafe { &*(self as *const Self).cast::<u8>().add(10usize).cast() }
    }
    #[doc = "0x0c - CTSU Channel Enable Control Register A"]
    #[inline(always)]
    pub const fn ctsuchac0(&self) -> &CTSUCHAC0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
    #[doc = "0x0c - CTSU Channel Enable Control Register A"]
    #[inline(always)]
    pub const fn ctsuchacal(&self) -> &CTSUCHACAL {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
    #[doc = "0x0c - CTSU Channel Enable Control Register A"]
    #[inline(always)]
    pub const fn ctsuchaca(&self) -> &CTSUCHACA {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
    #[doc = "0x0d - CTSU Channel Enable Control Register A"]
    #[inline(always)]
    pub const fn ctsuchac1(&self) -> &CTSUCHAC1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(13usize).cast() }
    }
    #[doc = "0x0e - CTSU Channel Enable Control Register A"]
    #[inline(always)]
    pub const fn ctsuchac2(&self) -> &CTSUCHAC2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(14usize).cast() }
    }
    #[doc = "0x0e - CTSU Channel Enable Control Register A"]
    #[inline(always)]
    pub const fn ctsuchacah(&self) -> &CTSUCHACAH {
        unsafe { &*(self as *const Self).cast::<u8>().add(14usize).cast() }
    }
    #[doc = "0x0f - CTSU Channel Enable Control Register A"]
    #[inline(always)]
    pub const fn ctsuchac3(&self) -> &CTSUCHAC3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(15usize).cast() }
    }
    #[doc = "0x10 - CTSU Channel Enable Control Register B"]
    #[inline(always)]
    pub const fn ctsuchac4(&self) -> &CTSUCHAC4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x10 - CTSU Channel Enable Control Register B"]
    #[inline(always)]
    pub const fn ctsuchacbl(&self) -> &CTSUCHACBL {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x10 - CTSU Channel Enable Control Register B"]
    #[inline(always)]
    pub const fn ctsuchacb(&self) -> &CTSUCHACB {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x14 - CTSU Channel Transmit/Receive Control Register A"]
    #[inline(always)]
    pub const fn ctsuchtrc0(&self) -> &CTSUCHTRC0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(20usize).cast() }
    }
    #[doc = "0x14 - CTSU Channel Transmit/Receive Control Register A"]
    #[inline(always)]
    pub const fn ctsuchtrcal(&self) -> &CTSUCHTRCAL {
        unsafe { &*(self as *const Self).cast::<u8>().add(20usize).cast() }
    }
    #[doc = "0x14 - CTSU Channel Transmit/Receive Control Register A"]
    #[inline(always)]
    pub const fn ctsuchtrca(&self) -> &CTSUCHTRCA {
        unsafe { &*(self as *const Self).cast::<u8>().add(20usize).cast() }
    }
    #[doc = "0x15 - CTSU Channel Transmit/Receive Control Register A"]
    #[inline(always)]
    pub const fn ctsuchtrc1(&self) -> &CTSUCHTRC1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(21usize).cast() }
    }
    #[doc = "0x16 - CTSU Channel Transmit/Receive Control Register A"]
    #[inline(always)]
    pub const fn ctsuchtrc2(&self) -> &CTSUCHTRC2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(22usize).cast() }
    }
    #[doc = "0x16 - CTSU Channel Transmit/Receive Control Register A"]
    #[inline(always)]
    pub const fn ctsuchtrcah(&self) -> &CTSUCHTRCAH {
        unsafe { &*(self as *const Self).cast::<u8>().add(22usize).cast() }
    }
    #[doc = "0x17 - CTSU Channel Transmit/Receive Control Register A"]
    #[inline(always)]
    pub const fn ctsuchtrc3(&self) -> &CTSUCHTRC3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(23usize).cast() }
    }
    #[doc = "0x18 - CTSU Channel Transmit/Receive Control Register B"]
    #[inline(always)]
    pub const fn ctsuchtrc4(&self) -> &CTSUCHTRC4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x18 - CTSU Channel Transmit/Receive Control Register B"]
    #[inline(always)]
    pub const fn ctsuchtrcbl(&self) -> &CTSUCHTRCBL {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x18 - CTSU Channel Transmit/Receive Control Register B"]
    #[inline(always)]
    pub const fn ctsuchtrcb(&self) -> &CTSUCHTRCB {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x1c - CTSU Status Register"]
    #[inline(always)]
    pub const fn ctsusr0(&self) -> &CTSUSR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1c - CTSU Status Register"]
    #[inline(always)]
    pub const fn ctsusrl(&self) -> &CTSUSRL {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1c - CTSU Status Register"]
    #[inline(always)]
    pub const fn ctsusr(&self) -> &CTSUSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1d - CTSU Status Register"]
    #[inline(always)]
    pub const fn ctsust(&self) -> &CTSUST {
        unsafe { &*(self as *const Self).cast::<u8>().add(29usize).cast() }
    }
    #[doc = "0x1e - CTSU Status Register"]
    #[inline(always)]
    pub const fn ctsusr2(&self) -> &CTSUSR2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(30usize).cast() }
    }
    #[doc = "0x1e - CTSU Status Register"]
    #[inline(always)]
    pub const fn ctsusrh(&self) -> &CTSUSRH {
        unsafe { &*(self as *const Self).cast::<u8>().add(30usize).cast() }
    }
    #[doc = "0x20 - CTSU Sensor Offset Register"]
    #[inline(always)]
    pub const fn ctsuso0(&self) -> &CTSUSO0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    #[doc = "0x20 - CTSU Sensor Offset Register"]
    #[inline(always)]
    pub const fn ctsuso(&self) -> &CTSUSO {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    #[doc = "0x22 - CTSU Sensor Offset Register"]
    #[inline(always)]
    pub const fn ctsuso1(&self) -> &CTSUSO1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(34usize).cast() }
    }
    #[doc = "0x24 - CTSU Sensor Counter Register"]
    #[inline(always)]
    pub const fn ctsusc(&self) -> &CTSUSC {
        unsafe { &*(self as *const Self).cast::<u8>().add(36usize).cast() }
    }
    #[doc = "0x24 - CTSU Sensor Counter Register"]
    #[inline(always)]
    pub const fn ctsuscnt(&self) -> &CTSUSCNT {
        unsafe { &*(self as *const Self).cast::<u8>().add(36usize).cast() }
    }
    #[doc = "0x28 - CTSU Calibration Register"]
    #[inline(always)]
    pub const fn ctsudbgr0(&self) -> &CTSUDBGR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
    #[doc = "0x28 - CTSU Calibration Register"]
    #[inline(always)]
    pub const fn ctsucalib(&self) -> &CTSUCALIB {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
    #[doc = "0x2a - CTSU Calibration Register"]
    #[inline(always)]
    pub const fn ctsudbgr1(&self) -> &CTSUDBGR1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(42usize).cast() }
    }
    #[doc = "0x2c - CTSU Sensor Unit Clock Control Register A"]
    #[inline(always)]
    pub const fn ctsusuclk0(&self) -> &CTSUSUCLK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(44usize).cast() }
    }
    #[doc = "0x2c - CTSU Sensor Unit Clock Control Register A"]
    #[inline(always)]
    pub const fn ctsusuclka(&self) -> &CTSUSUCLKA {
        unsafe { &*(self as *const Self).cast::<u8>().add(44usize).cast() }
    }
    #[doc = "0x2e - CTSU Sensor Unit Clock Control Register A"]
    #[inline(always)]
    pub const fn ctsusuclk1(&self) -> &CTSUSUCLK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(46usize).cast() }
    }
    #[doc = "0x30 - CTSU Sensor Unit Clock Control Register B"]
    #[inline(always)]
    pub const fn ctsusuclk2(&self) -> &CTSUSUCLK2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(48usize).cast() }
    }
    #[doc = "0x30 - CTSU Sensor Unit Clock Control Register B"]
    #[inline(always)]
    pub const fn ctsusuclkb(&self) -> &CTSUSUCLKB {
        unsafe { &*(self as *const Self).cast::<u8>().add(48usize).cast() }
    }
    #[doc = "0x32 - CTSU Sensor Unit Clock Control Register B"]
    #[inline(always)]
    pub const fn ctsusuclk3(&self) -> &CTSUSUCLK3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(50usize).cast() }
    }
    #[doc = "0x34 - CTSU CFC Counter Register"]
    #[inline(always)]
    pub const fn ctsucfccntl(&self) -> &CTSUCFCCNTL {
        unsafe { &*(self as *const Self).cast::<u8>().add(52usize).cast() }
    }
    #[doc = "0x34 - CTSU CFC Counter Register"]
    #[inline(always)]
    pub const fn ctsucfccnt(&self) -> &CTSUCFCCNT {
        unsafe { &*(self as *const Self).cast::<u8>().add(52usize).cast() }
    }
}
#[doc = "CTSUCRA (rw) register accessor: an alias for `Reg<CTSUCRA_SPEC>`"]
pub type CTSUCRA = crate::Reg<ctsucra::CTSUCRA_SPEC>;
#[doc = "CTSU Control Register A"]
pub mod ctsucra;
#[doc = "CTSUCRAL (rw) register accessor: an alias for `Reg<CTSUCRAL_SPEC>`"]
pub type CTSUCRAL = crate::Reg<ctsucral::CTSUCRAL_SPEC>;
#[doc = "CTSU Control Register A"]
pub mod ctsucral;
#[doc = "CTSUCR0 (rw) register accessor: an alias for `Reg<CTSUCR0_SPEC>`"]
pub type CTSUCR0 = crate::Reg<ctsucr0::CTSUCR0_SPEC>;
#[doc = "CTSU Control Register A"]
pub mod ctsucr0;
#[doc = "CTSUCR1 (rw) register accessor: an alias for `Reg<CTSUCR1_SPEC>`"]
pub type CTSUCR1 = crate::Reg<ctsucr1::CTSUCR1_SPEC>;
#[doc = "CTSU Control Register A"]
pub mod ctsucr1;
#[doc = "CTSUCR2 (rw) register accessor: an alias for `Reg<CTSUCR2_SPEC>`"]
pub type CTSUCR2 = crate::Reg<ctsucr2::CTSUCR2_SPEC>;
#[doc = "CTSU Control Register A"]
pub mod ctsucr2;
#[doc = "CTSUCR3 (rw) register accessor: an alias for `Reg<CTSUCR3_SPEC>`"]
pub type CTSUCR3 = crate::Reg<ctsucr3::CTSUCR3_SPEC>;
#[doc = "CTSU Control Register A"]
pub mod ctsucr3;
#[doc = "CTSUCRB (rw) register accessor: an alias for `Reg<CTSUCRB_SPEC>`"]
pub type CTSUCRB = crate::Reg<ctsucrb::CTSUCRB_SPEC>;
#[doc = "CTSU Control Register B"]
pub mod ctsucrb;
#[doc = "CTSUCRBL (rw) register accessor: an alias for `Reg<CTSUCRBL_SPEC>`"]
pub type CTSUCRBL = crate::Reg<ctsucrbl::CTSUCRBL_SPEC>;
#[doc = "CTSU Control Register B"]
pub mod ctsucrbl;
#[doc = "CTSUSDPRS (rw) register accessor: an alias for `Reg<CTSUSDPRS_SPEC>`"]
pub type CTSUSDPRS = crate::Reg<ctsusdprs::CTSUSDPRS_SPEC>;
#[doc = "CTSU Control Register B"]
pub mod ctsusdprs;
#[doc = "CTSUSST (rw) register accessor: an alias for `Reg<CTSUSST_SPEC>`"]
pub type CTSUSST = crate::Reg<ctsusst::CTSUSST_SPEC>;
#[doc = "CTSU Control Register B"]
pub mod ctsusst;
#[doc = "CTSUCRBH (rw) register accessor: an alias for `Reg<CTSUCRBH_SPEC>`"]
pub type CTSUCRBH = crate::Reg<ctsucrbh::CTSUCRBH_SPEC>;
#[doc = "CTSU Control Register B"]
pub mod ctsucrbh;
#[doc = "CTSUDCLKC (rw) register accessor: an alias for `Reg<CTSUDCLKC_SPEC>`"]
pub type CTSUDCLKC = crate::Reg<ctsudclkc::CTSUDCLKC_SPEC>;
#[doc = "CTSU Control Register B"]
pub mod ctsudclkc;
#[doc = "CTSUMCH (rw) register accessor: an alias for `Reg<CTSUMCH_SPEC>`"]
pub type CTSUMCH = crate::Reg<ctsumch::CTSUMCH_SPEC>;
#[doc = "CTSU Measurement Channel Register"]
pub mod ctsumch;
#[doc = "CTSUMCHL (rw) register accessor: an alias for `Reg<CTSUMCHL_SPEC>`"]
pub type CTSUMCHL = crate::Reg<ctsumchl::CTSUMCHL_SPEC>;
#[doc = "CTSU Measurement Channel Register"]
pub mod ctsumchl;
#[doc = "CTSUMCH0 (rw) register accessor: an alias for `Reg<CTSUMCH0_SPEC>`"]
pub type CTSUMCH0 = crate::Reg<ctsumch0::CTSUMCH0_SPEC>;
#[doc = "CTSU Measurement Channel Register"]
pub mod ctsumch0;
#[doc = "CTSUMCH1 (rw) register accessor: an alias for `Reg<CTSUMCH1_SPEC>`"]
pub type CTSUMCH1 = crate::Reg<ctsumch1::CTSUMCH1_SPEC>;
#[doc = "CTSU Measurement Channel Register"]
pub mod ctsumch1;
#[doc = "CTSUMCHH (rw) register accessor: an alias for `Reg<CTSUMCHH_SPEC>`"]
pub type CTSUMCHH = crate::Reg<ctsumchh::CTSUMCHH_SPEC>;
#[doc = "CTSU Measurement Channel Register"]
pub mod ctsumchh;
#[doc = "CTSUMFAF (rw) register accessor: an alias for `Reg<CTSUMFAF_SPEC>`"]
pub type CTSUMFAF = crate::Reg<ctsumfaf::CTSUMFAF_SPEC>;
#[doc = "CTSU Measurement Channel Register"]
pub mod ctsumfaf;
#[doc = "CTSUCHACA (rw) register accessor: an alias for `Reg<CTSUCHACA_SPEC>`"]
pub type CTSUCHACA = crate::Reg<ctsuchaca::CTSUCHACA_SPEC>;
#[doc = "CTSU Channel Enable Control Register A"]
pub mod ctsuchaca;
#[doc = "CTSUCHACAL (rw) register accessor: an alias for `Reg<CTSUCHACAL_SPEC>`"]
pub type CTSUCHACAL = crate::Reg<ctsuchacal::CTSUCHACAL_SPEC>;
#[doc = "CTSU Channel Enable Control Register A"]
pub mod ctsuchacal;
#[doc = "CTSUCHAC0 (rw) register accessor: an alias for `Reg<CTSUCHAC0_SPEC>`"]
pub type CTSUCHAC0 = crate::Reg<ctsuchac0::CTSUCHAC0_SPEC>;
#[doc = "CTSU Channel Enable Control Register A"]
pub mod ctsuchac0;
#[doc = "CTSUCHAC1 (rw) register accessor: an alias for `Reg<CTSUCHAC1_SPEC>`"]
pub type CTSUCHAC1 = crate::Reg<ctsuchac1::CTSUCHAC1_SPEC>;
#[doc = "CTSU Channel Enable Control Register A"]
pub mod ctsuchac1;
#[doc = "CTSUCHACAH (rw) register accessor: an alias for `Reg<CTSUCHACAH_SPEC>`"]
pub type CTSUCHACAH = crate::Reg<ctsuchacah::CTSUCHACAH_SPEC>;
#[doc = "CTSU Channel Enable Control Register A"]
pub mod ctsuchacah;
#[doc = "CTSUCHAC2 (rw) register accessor: an alias for `Reg<CTSUCHAC2_SPEC>`"]
pub type CTSUCHAC2 = crate::Reg<ctsuchac2::CTSUCHAC2_SPEC>;
#[doc = "CTSU Channel Enable Control Register A"]
pub mod ctsuchac2;
#[doc = "CTSUCHAC3 (rw) register accessor: an alias for `Reg<CTSUCHAC3_SPEC>`"]
pub type CTSUCHAC3 = crate::Reg<ctsuchac3::CTSUCHAC3_SPEC>;
#[doc = "CTSU Channel Enable Control Register A"]
pub mod ctsuchac3;
#[doc = "CTSUCHACB (rw) register accessor: an alias for `Reg<CTSUCHACB_SPEC>`"]
pub type CTSUCHACB = crate::Reg<ctsuchacb::CTSUCHACB_SPEC>;
#[doc = "CTSU Channel Enable Control Register B"]
pub mod ctsuchacb;
#[doc = "CTSUCHACBL (rw) register accessor: an alias for `Reg<CTSUCHACBL_SPEC>`"]
pub type CTSUCHACBL = crate::Reg<ctsuchacbl::CTSUCHACBL_SPEC>;
#[doc = "CTSU Channel Enable Control Register B"]
pub mod ctsuchacbl;
#[doc = "CTSUCHAC4 (rw) register accessor: an alias for `Reg<CTSUCHAC4_SPEC>`"]
pub type CTSUCHAC4 = crate::Reg<ctsuchac4::CTSUCHAC4_SPEC>;
#[doc = "CTSU Channel Enable Control Register B"]
pub mod ctsuchac4;
#[doc = "CTSUCHTRCA (rw) register accessor: an alias for `Reg<CTSUCHTRCA_SPEC>`"]
pub type CTSUCHTRCA = crate::Reg<ctsuchtrca::CTSUCHTRCA_SPEC>;
#[doc = "CTSU Channel Transmit/Receive Control Register A"]
pub mod ctsuchtrca;
#[doc = "CTSUCHTRCAL (rw) register accessor: an alias for `Reg<CTSUCHTRCAL_SPEC>`"]
pub type CTSUCHTRCAL = crate::Reg<ctsuchtrcal::CTSUCHTRCAL_SPEC>;
#[doc = "CTSU Channel Transmit/Receive Control Register A"]
pub mod ctsuchtrcal;
#[doc = "CTSUCHTRC0 (rw) register accessor: an alias for `Reg<CTSUCHTRC0_SPEC>`"]
pub type CTSUCHTRC0 = crate::Reg<ctsuchtrc0::CTSUCHTRC0_SPEC>;
#[doc = "CTSU Channel Transmit/Receive Control Register A"]
pub mod ctsuchtrc0;
#[doc = "CTSUCHTRC1 (rw) register accessor: an alias for `Reg<CTSUCHTRC1_SPEC>`"]
pub type CTSUCHTRC1 = crate::Reg<ctsuchtrc1::CTSUCHTRC1_SPEC>;
#[doc = "CTSU Channel Transmit/Receive Control Register A"]
pub mod ctsuchtrc1;
#[doc = "CTSUCHTRCAH (rw) register accessor: an alias for `Reg<CTSUCHTRCAH_SPEC>`"]
pub type CTSUCHTRCAH = crate::Reg<ctsuchtrcah::CTSUCHTRCAH_SPEC>;
#[doc = "CTSU Channel Transmit/Receive Control Register A"]
pub mod ctsuchtrcah;
#[doc = "CTSUCHTRC2 (rw) register accessor: an alias for `Reg<CTSUCHTRC2_SPEC>`"]
pub type CTSUCHTRC2 = crate::Reg<ctsuchtrc2::CTSUCHTRC2_SPEC>;
#[doc = "CTSU Channel Transmit/Receive Control Register A"]
pub mod ctsuchtrc2;
#[doc = "CTSUCHTRC3 (rw) register accessor: an alias for `Reg<CTSUCHTRC3_SPEC>`"]
pub type CTSUCHTRC3 = crate::Reg<ctsuchtrc3::CTSUCHTRC3_SPEC>;
#[doc = "CTSU Channel Transmit/Receive Control Register A"]
pub mod ctsuchtrc3;
#[doc = "CTSUCHTRCB (rw) register accessor: an alias for `Reg<CTSUCHTRCB_SPEC>`"]
pub type CTSUCHTRCB = crate::Reg<ctsuchtrcb::CTSUCHTRCB_SPEC>;
#[doc = "CTSU Channel Transmit/Receive Control Register B"]
pub mod ctsuchtrcb;
#[doc = "CTSUCHTRCBL (rw) register accessor: an alias for `Reg<CTSUCHTRCBL_SPEC>`"]
pub type CTSUCHTRCBL = crate::Reg<ctsuchtrcbl::CTSUCHTRCBL_SPEC>;
#[doc = "CTSU Channel Transmit/Receive Control Register B"]
pub mod ctsuchtrcbl;
#[doc = "CTSUCHTRC4 (rw) register accessor: an alias for `Reg<CTSUCHTRC4_SPEC>`"]
pub type CTSUCHTRC4 = crate::Reg<ctsuchtrc4::CTSUCHTRC4_SPEC>;
#[doc = "CTSU Channel Transmit/Receive Control Register B"]
pub mod ctsuchtrc4;
#[doc = "CTSUSR (rw) register accessor: an alias for `Reg<CTSUSR_SPEC>`"]
pub type CTSUSR = crate::Reg<ctsusr::CTSUSR_SPEC>;
#[doc = "CTSU Status Register"]
pub mod ctsusr;
#[doc = "CTSUSRL (rw) register accessor: an alias for `Reg<CTSUSRL_SPEC>`"]
pub type CTSUSRL = crate::Reg<ctsusrl::CTSUSRL_SPEC>;
#[doc = "CTSU Status Register"]
pub mod ctsusrl;
#[doc = "CTSUSR0 (rw) register accessor: an alias for `Reg<CTSUSR0_SPEC>`"]
pub type CTSUSR0 = crate::Reg<ctsusr0::CTSUSR0_SPEC>;
#[doc = "CTSU Status Register"]
pub mod ctsusr0;
#[doc = "CTSUST (rw) register accessor: an alias for `Reg<CTSUST_SPEC>`"]
pub type CTSUST = crate::Reg<ctsust::CTSUST_SPEC>;
#[doc = "CTSU Status Register"]
pub mod ctsust;
#[doc = "CTSUSRH (rw) register accessor: an alias for `Reg<CTSUSRH_SPEC>`"]
pub type CTSUSRH = crate::Reg<ctsusrh::CTSUSRH_SPEC>;
#[doc = "CTSU Status Register"]
pub mod ctsusrh;
#[doc = "CTSUSR2 (rw) register accessor: an alias for `Reg<CTSUSR2_SPEC>`"]
pub type CTSUSR2 = crate::Reg<ctsusr2::CTSUSR2_SPEC>;
#[doc = "CTSU Status Register"]
pub mod ctsusr2;
#[doc = "CTSUSO (rw) register accessor: an alias for `Reg<CTSUSO_SPEC>`"]
pub type CTSUSO = crate::Reg<ctsuso::CTSUSO_SPEC>;
#[doc = "CTSU Sensor Offset Register"]
pub mod ctsuso;
#[doc = "CTSUSO0 (rw) register accessor: an alias for `Reg<CTSUSO0_SPEC>`"]
pub type CTSUSO0 = crate::Reg<ctsuso0::CTSUSO0_SPEC>;
#[doc = "CTSU Sensor Offset Register"]
pub mod ctsuso0;
#[doc = "CTSUSO1 (rw) register accessor: an alias for `Reg<CTSUSO1_SPEC>`"]
pub type CTSUSO1 = crate::Reg<ctsuso1::CTSUSO1_SPEC>;
#[doc = "CTSU Sensor Offset Register"]
pub mod ctsuso1;
#[doc = "CTSUSCNT (r) register accessor: an alias for `Reg<CTSUSCNT_SPEC>`"]
pub type CTSUSCNT = crate::Reg<ctsuscnt::CTSUSCNT_SPEC>;
#[doc = "CTSU Sensor Counter Register"]
pub mod ctsuscnt;
#[doc = "CTSUSC (r) register accessor: an alias for `Reg<CTSUSC_SPEC>`"]
pub type CTSUSC = crate::Reg<ctsusc::CTSUSC_SPEC>;
#[doc = "CTSU Sensor Counter Register"]
pub mod ctsusc;
#[doc = "CTSUCALIB (rw) register accessor: an alias for `Reg<CTSUCALIB_SPEC>`"]
pub type CTSUCALIB = crate::Reg<ctsucalib::CTSUCALIB_SPEC>;
#[doc = "CTSU Calibration Register"]
pub mod ctsucalib;
#[doc = "CTSUDBGR0 (rw) register accessor: an alias for `Reg<CTSUDBGR0_SPEC>`"]
pub type CTSUDBGR0 = crate::Reg<ctsudbgr0::CTSUDBGR0_SPEC>;
#[doc = "CTSU Calibration Register"]
pub mod ctsudbgr0;
#[doc = "CTSUDBGR1 (rw) register accessor: an alias for `Reg<CTSUDBGR1_SPEC>`"]
pub type CTSUDBGR1 = crate::Reg<ctsudbgr1::CTSUDBGR1_SPEC>;
#[doc = "CTSU Calibration Register"]
pub mod ctsudbgr1;
#[doc = "CTSUSUCLKA (rw) register accessor: an alias for `Reg<CTSUSUCLKA_SPEC>`"]
pub type CTSUSUCLKA = crate::Reg<ctsusuclka::CTSUSUCLKA_SPEC>;
#[doc = "CTSU Sensor Unit Clock Control Register A"]
pub mod ctsusuclka;
#[doc = "CTSUSUCLK0 (rw) register accessor: an alias for `Reg<CTSUSUCLK0_SPEC>`"]
pub type CTSUSUCLK0 = crate::Reg<ctsusuclk0::CTSUSUCLK0_SPEC>;
#[doc = "CTSU Sensor Unit Clock Control Register A"]
pub mod ctsusuclk0;
#[doc = "CTSUSUCLK1 (rw) register accessor: an alias for `Reg<CTSUSUCLK1_SPEC>`"]
pub type CTSUSUCLK1 = crate::Reg<ctsusuclk1::CTSUSUCLK1_SPEC>;
#[doc = "CTSU Sensor Unit Clock Control Register A"]
pub mod ctsusuclk1;
#[doc = "CTSUSUCLKB (rw) register accessor: an alias for `Reg<CTSUSUCLKB_SPEC>`"]
pub type CTSUSUCLKB = crate::Reg<ctsusuclkb::CTSUSUCLKB_SPEC>;
#[doc = "CTSU Sensor Unit Clock Control Register B"]
pub mod ctsusuclkb;
#[doc = "CTSUSUCLK2 (rw) register accessor: an alias for `Reg<CTSUSUCLK2_SPEC>`"]
pub type CTSUSUCLK2 = crate::Reg<ctsusuclk2::CTSUSUCLK2_SPEC>;
#[doc = "CTSU Sensor Unit Clock Control Register B"]
pub mod ctsusuclk2;
#[doc = "CTSUSUCLK3 (rw) register accessor: an alias for `Reg<CTSUSUCLK3_SPEC>`"]
pub type CTSUSUCLK3 = crate::Reg<ctsusuclk3::CTSUSUCLK3_SPEC>;
#[doc = "CTSU Sensor Unit Clock Control Register B"]
pub mod ctsusuclk3;
#[doc = "CTSUCFCCNT (r) register accessor: an alias for `Reg<CTSUCFCCNT_SPEC>`"]
pub type CTSUCFCCNT = crate::Reg<ctsucfccnt::CTSUCFCCNT_SPEC>;
#[doc = "CTSU CFC Counter Register"]
pub mod ctsucfccnt;
#[doc = "CTSUCFCCNTL (r) register accessor: an alias for `Reg<CTSUCFCCNTL_SPEC>`"]
pub type CTSUCFCCNTL = crate::Reg<ctsucfccntl::CTSUCFCCNTL_SPEC>;
#[doc = "CTSU CFC Counter Register"]
pub mod ctsucfccntl;
