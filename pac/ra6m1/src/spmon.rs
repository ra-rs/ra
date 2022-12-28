#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Stack Pointer Monitor Operation After Detection Register"]
    pub mspmpuoad: MSPMPUOAD,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - Stack Pointer Monitor Access Control Register"]
    pub mspmpuctl: MSPMPUCTL,
    #[doc = "0x06 - Stack Pointer Monitor Protection Register"]
    pub mspmpupt: MSPMPUPT,
    #[doc = "0x08 - Main Stack Pointer Monitor Start Address Register"]
    pub mspmpusa: MSPMPUSA,
    #[doc = "0x0c - Main Stack Pointer Monitor End Address Register"]
    pub mspmpuea: MSPMPUEA,
    #[doc = "0x10 - Stack Pointer Monitor Operation After Detection Register"]
    pub pspmpuoad: PSPMPUOAD,
    _reserved6: [u8; 0x02],
    #[doc = "0x14 - Stack Pointer Monitor Access Control Register"]
    pub pspmpuctl: PSPMPUCTL,
    #[doc = "0x16 - Stack Pointer Monitor Protection Register"]
    pub pspmpupt: PSPMPUPT,
    #[doc = "0x18 - Process Stack Pointer Monitor Start Address Register"]
    pub pspmpusa: PSPMPUSA,
    #[doc = "0x1c - Process Stack Pointer Monitor End Address Register"]
    pub pspmpuea: PSPMPUEA,
}
#[doc = "MSPMPUOAD (rw) register accessor: an alias for `Reg<MSPMPUOAD_SPEC>`"]
pub type MSPMPUOAD = crate::Reg<mspmpuoad::MSPMPUOAD_SPEC>;
#[doc = "Stack Pointer Monitor Operation After Detection Register"]
pub mod mspmpuoad;
#[doc = "MSPMPUCTL (rw) register accessor: an alias for `Reg<MSPMPUCTL_SPEC>`"]
pub type MSPMPUCTL = crate::Reg<mspmpuctl::MSPMPUCTL_SPEC>;
#[doc = "Stack Pointer Monitor Access Control Register"]
pub mod mspmpuctl;
#[doc = "MSPMPUPT (rw) register accessor: an alias for `Reg<MSPMPUPT_SPEC>`"]
pub type MSPMPUPT = crate::Reg<mspmpupt::MSPMPUPT_SPEC>;
#[doc = "Stack Pointer Monitor Protection Register"]
pub mod mspmpupt;
#[doc = "MSPMPUSA (rw) register accessor: an alias for `Reg<MSPMPUSA_SPEC>`"]
pub type MSPMPUSA = crate::Reg<mspmpusa::MSPMPUSA_SPEC>;
#[doc = "Main Stack Pointer Monitor Start Address Register"]
pub mod mspmpusa;
#[doc = "MSPMPUEA (rw) register accessor: an alias for `Reg<MSPMPUEA_SPEC>`"]
pub type MSPMPUEA = crate::Reg<mspmpuea::MSPMPUEA_SPEC>;
#[doc = "Main Stack Pointer Monitor End Address Register"]
pub mod mspmpuea;
#[doc = "PSPMPUOAD (rw) register accessor: an alias for `Reg<PSPMPUOAD_SPEC>`"]
pub type PSPMPUOAD = crate::Reg<pspmpuoad::PSPMPUOAD_SPEC>;
#[doc = "Stack Pointer Monitor Operation After Detection Register"]
pub mod pspmpuoad;
#[doc = "PSPMPUCTL (rw) register accessor: an alias for `Reg<PSPMPUCTL_SPEC>`"]
pub type PSPMPUCTL = crate::Reg<pspmpuctl::PSPMPUCTL_SPEC>;
#[doc = "Stack Pointer Monitor Access Control Register"]
pub mod pspmpuctl;
#[doc = "PSPMPUPT (rw) register accessor: an alias for `Reg<PSPMPUPT_SPEC>`"]
pub type PSPMPUPT = crate::Reg<pspmpupt::PSPMPUPT_SPEC>;
#[doc = "Stack Pointer Monitor Protection Register"]
pub mod pspmpupt;
#[doc = "PSPMPUSA (rw) register accessor: an alias for `Reg<PSPMPUSA_SPEC>`"]
pub type PSPMPUSA = crate::Reg<pspmpusa::PSPMPUSA_SPEC>;
#[doc = "Process Stack Pointer Monitor Start Address Register"]
pub mod pspmpusa;
#[doc = "PSPMPUEA (rw) register accessor: an alias for `Reg<PSPMPUEA_SPEC>`"]
pub type PSPMPUEA = crate::Reg<pspmpuea::PSPMPUEA_SPEC>;
#[doc = "Process Stack Pointer Monitor End Address Register"]
pub mod pspmpuea;
