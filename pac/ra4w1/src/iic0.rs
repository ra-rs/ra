#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C Bus Control Register 1"]
    pub iccr1: ICCR1,
    #[doc = "0x01 - I2C Bus Control Register 2"]
    pub iccr2: ICCR2,
    #[doc = "0x02 - I2C Bus Mode Register 1"]
    pub icmr1: ICMR1,
    #[doc = "0x03 - I2C Bus Mode Register 2"]
    pub icmr2: ICMR2,
    #[doc = "0x04 - I2C Bus Mode Register 3"]
    pub icmr3: ICMR3,
    #[doc = "0x05 - I2C Bus Function Enable Register"]
    pub icfer: ICFER,
    #[doc = "0x06 - I2C Bus Status Enable Register"]
    pub icser: ICSER,
    #[doc = "0x07 - I2C Bus Interrupt Enable Register"]
    pub icier: ICIER,
    #[doc = "0x08 - I2C Bus Status Register 1"]
    pub icsr1: ICSR1,
    #[doc = "0x09 - I2C Bus Status Register 2"]
    pub icsr2: ICSR2,
    #[doc = "0x0a - Slave Address Register L%s"]
    pub sarl0: SARL,
    #[doc = "0x0b - Slave Address Register U%s"]
    pub saru0: SARU,
    #[doc = "0x0c - Slave Address Register L%s"]
    pub sarl1: SARL,
    #[doc = "0x0d - Slave Address Register U%s"]
    pub saru1: SARU,
    #[doc = "0x0e - Slave Address Register L%s"]
    pub sarl2: SARL,
    #[doc = "0x0f - Slave Address Register U%s"]
    pub saru2: SARU,
    #[doc = "0x10 - I2C Bus Bit Rate Low-Level Register"]
    pub icbrl: ICBRL,
    #[doc = "0x11 - I2C Bus Bit Rate High-Level Register"]
    pub icbrh: ICBRH,
    #[doc = "0x12 - I2C Bus Transmit Data Register"]
    pub icdrt: ICDRT,
    #[doc = "0x13 - I2C Bus Receive Data Register"]
    pub icdrr: ICDRR,
    _reserved20: [u8; 0x02],
    #[doc = "0x16 - I2C Bus Wake Up Unit Register"]
    pub icwur: ICWUR,
    #[doc = "0x17 - I2C Bus Wake up Unit Register 2"]
    pub icwur2: ICWUR2,
}
#[doc = "ICCR1 (rw) register accessor: an alias for `Reg<ICCR1_SPEC>`"]
pub type ICCR1 = crate::Reg<iccr1::ICCR1_SPEC>;
#[doc = "I2C Bus Control Register 1"]
pub mod iccr1;
#[doc = "ICCR2 (rw) register accessor: an alias for `Reg<ICCR2_SPEC>`"]
pub type ICCR2 = crate::Reg<iccr2::ICCR2_SPEC>;
#[doc = "I2C Bus Control Register 2"]
pub mod iccr2;
#[doc = "ICMR1 (rw) register accessor: an alias for `Reg<ICMR1_SPEC>`"]
pub type ICMR1 = crate::Reg<icmr1::ICMR1_SPEC>;
#[doc = "I2C Bus Mode Register 1"]
pub mod icmr1;
#[doc = "ICMR2 (rw) register accessor: an alias for `Reg<ICMR2_SPEC>`"]
pub type ICMR2 = crate::Reg<icmr2::ICMR2_SPEC>;
#[doc = "I2C Bus Mode Register 2"]
pub mod icmr2;
#[doc = "ICMR3 (rw) register accessor: an alias for `Reg<ICMR3_SPEC>`"]
pub type ICMR3 = crate::Reg<icmr3::ICMR3_SPEC>;
#[doc = "I2C Bus Mode Register 3"]
pub mod icmr3;
#[doc = "ICFER (rw) register accessor: an alias for `Reg<ICFER_SPEC>`"]
pub type ICFER = crate::Reg<icfer::ICFER_SPEC>;
#[doc = "I2C Bus Function Enable Register"]
pub mod icfer;
#[doc = "ICSER (rw) register accessor: an alias for `Reg<ICSER_SPEC>`"]
pub type ICSER = crate::Reg<icser::ICSER_SPEC>;
#[doc = "I2C Bus Status Enable Register"]
pub mod icser;
#[doc = "ICIER (rw) register accessor: an alias for `Reg<ICIER_SPEC>`"]
pub type ICIER = crate::Reg<icier::ICIER_SPEC>;
#[doc = "I2C Bus Interrupt Enable Register"]
pub mod icier;
#[doc = "ICSR1 (rw) register accessor: an alias for `Reg<ICSR1_SPEC>`"]
pub type ICSR1 = crate::Reg<icsr1::ICSR1_SPEC>;
#[doc = "I2C Bus Status Register 1"]
pub mod icsr1;
#[doc = "ICSR2 (rw) register accessor: an alias for `Reg<ICSR2_SPEC>`"]
pub type ICSR2 = crate::Reg<icsr2::ICSR2_SPEC>;
#[doc = "I2C Bus Status Register 2"]
pub mod icsr2;
#[doc = "SARL (rw) register accessor: an alias for `Reg<SARL_SPEC>`"]
pub type SARL = crate::Reg<sarl::SARL_SPEC>;
#[doc = "Slave Address Register L%s"]
pub mod sarl;
#[doc = "SARU (rw) register accessor: an alias for `Reg<SARU_SPEC>`"]
pub type SARU = crate::Reg<saru::SARU_SPEC>;
#[doc = "Slave Address Register U%s"]
pub mod saru;
#[doc = "ICBRL (rw) register accessor: an alias for `Reg<ICBRL_SPEC>`"]
pub type ICBRL = crate::Reg<icbrl::ICBRL_SPEC>;
#[doc = "I2C Bus Bit Rate Low-Level Register"]
pub mod icbrl;
#[doc = "ICBRH (rw) register accessor: an alias for `Reg<ICBRH_SPEC>`"]
pub type ICBRH = crate::Reg<icbrh::ICBRH_SPEC>;
#[doc = "I2C Bus Bit Rate High-Level Register"]
pub mod icbrh;
#[doc = "ICDRT (rw) register accessor: an alias for `Reg<ICDRT_SPEC>`"]
pub type ICDRT = crate::Reg<icdrt::ICDRT_SPEC>;
#[doc = "I2C Bus Transmit Data Register"]
pub mod icdrt;
#[doc = "ICDRR (r) register accessor: an alias for `Reg<ICDRR_SPEC>`"]
pub type ICDRR = crate::Reg<icdrr::ICDRR_SPEC>;
#[doc = "I2C Bus Receive Data Register"]
pub mod icdrr;
#[doc = "ICWUR (rw) register accessor: an alias for `Reg<ICWUR_SPEC>`"]
pub type ICWUR = crate::Reg<icwur::ICWUR_SPEC>;
#[doc = "I2C Bus Wake Up Unit Register"]
pub mod icwur;
#[doc = "ICWUR2 (rw) register accessor: an alias for `Reg<ICWUR2_SPEC>`"]
pub type ICWUR2 = crate::Reg<icwur2::ICWUR2_SPEC>;
#[doc = "I2C Bus Wake up Unit Register 2"]
pub mod icwur2;
