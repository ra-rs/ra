#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Slave MPU Control Register"]
    pub smpuctl: SMPUCTL,
    _reserved1: [u8; 0x0e],
    #[doc = "0x10 - Access Control Register for MBIU"]
    pub smpumbiu: SMPUMBIU,
    _reserved2: [u8; 0x02],
    #[doc = "0x14 - Access Control Register for FBIU"]
    pub smpufbiu: SMPUFBIU,
    _reserved3: [u8; 0x02],
    #[doc = "0x18 - Access Control Register for SRAM"]
    pub smpusram0: SMPUSRAM0,
    _reserved4: [u8; 0x06],
    #[doc = "0x20 - Access Control Register for P%sBIU"]
    pub smpup0biu: SMPUPBIU,
    _reserved5: [u8; 0x02],
    #[doc = "0x24 - Access Control Register for P%sBIU"]
    pub smpup2biu: SMPUPBIU,
    _reserved6: [u8; 0x02],
    #[doc = "0x28 - Access Control Register for P%sBIU"]
    pub smpup6biu: SMPUPBIU,
}
#[doc = "SMPUCTL (rw) register accessor: an alias for `Reg<SMPUCTL_SPEC>`"]
pub type SMPUCTL = crate::Reg<smpuctl::SMPUCTL_SPEC>;
#[doc = "Slave MPU Control Register"]
pub mod smpuctl;
#[doc = "SMPUMBIU (rw) register accessor: an alias for `Reg<SMPUMBIU_SPEC>`"]
pub type SMPUMBIU = crate::Reg<smpumbiu::SMPUMBIU_SPEC>;
#[doc = "Access Control Register for MBIU"]
pub mod smpumbiu;
#[doc = "SMPUFBIU (rw) register accessor: an alias for `Reg<SMPUFBIU_SPEC>`"]
pub type SMPUFBIU = crate::Reg<smpufbiu::SMPUFBIU_SPEC>;
#[doc = "Access Control Register for FBIU"]
pub mod smpufbiu;
#[doc = "SMPUSRAM0 (rw) register accessor: an alias for `Reg<SMPUSRAM0_SPEC>`"]
pub type SMPUSRAM0 = crate::Reg<smpusram0::SMPUSRAM0_SPEC>;
#[doc = "Access Control Register for SRAM"]
pub mod smpusram0;
#[doc = "SMPUPBIU (rw) register accessor: an alias for `Reg<SMPUPBIU_SPEC>`"]
pub type SMPUPBIU = crate::Reg<smpupbiu::SMPUPBIU_SPEC>;
#[doc = "Access Control Register for P%sBIU"]
pub mod smpupbiu;
