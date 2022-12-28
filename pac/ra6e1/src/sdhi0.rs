#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Command Type Register"]
    pub sd_cmd: SD_CMD,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - SD Command Argument Register"]
    pub sd_arg: SD_ARG,
    #[doc = "0x0c - SD Command Argument Register 1"]
    pub sd_arg1: SD_ARG1,
    #[doc = "0x10 - Data Stop Register"]
    pub sd_stop: SD_STOP,
    #[doc = "0x14 - Block Count Register"]
    pub sd_seccnt: SD_SECCNT,
    #[doc = "0x18 - SD Card Response Register 10"]
    pub sd_rsp10: SD_RSP10,
    #[doc = "0x1c - SD Card Response Register 1"]
    pub sd_rsp1: SD_RSP1,
    #[doc = "0x20 - SD Card Response Register 32"]
    pub sd_rsp32: SD_RSP32,
    #[doc = "0x24 - SD Card Response Register 3"]
    pub sd_rsp3: SD_RSP3,
    #[doc = "0x28 - SD Card Response Register 54"]
    pub sd_rsp54: SD_RSP54,
    #[doc = "0x2c - SD Card Response Register 5"]
    pub sd_rsp5: SD_RSP5,
    #[doc = "0x30 - SD Card Response Register 76"]
    pub sd_rsp76: SD_RSP76,
    #[doc = "0x34 - SD Card Response Register 7"]
    pub sd_rsp7: SD_RSP7,
    #[doc = "0x38 - SD Card Interrupt Flag Register 1"]
    pub sd_info1: SD_INFO1,
    #[doc = "0x3c - SD Card Interrupt Flag Register 2"]
    pub sd_info2: SD_INFO2,
    #[doc = "0x40 - SD INFO1 Interrupt Mask Register"]
    pub sd_info1_mask: SD_INFO1_MASK,
    #[doc = "0x44 - SD INFO2 Interrupt Mask Register"]
    pub sd_info2_mask: SD_INFO2_MASK,
    #[doc = "0x48 - SD Clock Control Register"]
    pub sd_clk_ctrl: SD_CLK_CTRL,
    #[doc = "0x4c - Transfer Data Length Register"]
    pub sd_size: SD_SIZE,
    #[doc = "0x50 - SD Card Access Control Option Register"]
    pub sd_option: SD_OPTION,
    _reserved20: [u8; 0x04],
    #[doc = "0x58 - SD Error Status Register 1"]
    pub sd_err_sts1: SD_ERR_STS1,
    #[doc = "0x5c - SD Error Status Register 2"]
    pub sd_err_sts2: SD_ERR_STS2,
    #[doc = "0x60 - SD Buffer Register"]
    pub sd_buf0: SD_BUF0,
    _reserved23: [u8; 0x04],
    #[doc = "0x68 - SDIO Mode Control Register"]
    pub sdio_mode: SDIO_MODE,
    #[doc = "0x6c - SDIO Interrupt Flag Register"]
    pub sdio_info1: SDIO_INFO1,
    #[doc = "0x70 - SDIO INFO1 Interrupt Mask Register"]
    pub sdio_info1_mask: SDIO_INFO1_MASK,
    _reserved26: [u8; 0x013c],
    #[doc = "0x1b0 - DMA Mode Enable Register"]
    pub sd_dmaen: SD_DMAEN,
    _reserved27: [u8; 0x0c],
    #[doc = "0x1c0 - Software Reset Register"]
    pub soft_rst: SOFT_RST,
    _reserved28: [u8; 0x08],
    #[doc = "0x1cc - SD Interface Mode Setting Register"]
    pub sdif_mode: SDIF_MODE,
    _reserved29: [u8; 0x10],
    #[doc = "0x1e0 - Swap Control Register"]
    pub ext_swap: EXT_SWAP,
}
#[doc = "SD_CMD (rw) register accessor: an alias for `Reg<SD_CMD_SPEC>`"]
pub type SD_CMD = crate::Reg<sd_cmd::SD_CMD_SPEC>;
#[doc = "Command Type Register"]
pub mod sd_cmd;
#[doc = "SD_ARG (rw) register accessor: an alias for `Reg<SD_ARG_SPEC>`"]
pub type SD_ARG = crate::Reg<sd_arg::SD_ARG_SPEC>;
#[doc = "SD Command Argument Register"]
pub mod sd_arg;
#[doc = "SD_ARG1 (rw) register accessor: an alias for `Reg<SD_ARG1_SPEC>`"]
pub type SD_ARG1 = crate::Reg<sd_arg1::SD_ARG1_SPEC>;
#[doc = "SD Command Argument Register 1"]
pub mod sd_arg1;
#[doc = "SD_STOP (rw) register accessor: an alias for `Reg<SD_STOP_SPEC>`"]
pub type SD_STOP = crate::Reg<sd_stop::SD_STOP_SPEC>;
#[doc = "Data Stop Register"]
pub mod sd_stop;
#[doc = "SD_SECCNT (rw) register accessor: an alias for `Reg<SD_SECCNT_SPEC>`"]
pub type SD_SECCNT = crate::Reg<sd_seccnt::SD_SECCNT_SPEC>;
#[doc = "Block Count Register"]
pub mod sd_seccnt;
#[doc = "SD_RSP10 (rw) register accessor: an alias for `Reg<SD_RSP10_SPEC>`"]
pub type SD_RSP10 = crate::Reg<sd_rsp10::SD_RSP10_SPEC>;
#[doc = "SD Card Response Register 10"]
pub mod sd_rsp10;
#[doc = "SD_RSP1 (r) register accessor: an alias for `Reg<SD_RSP1_SPEC>`"]
pub type SD_RSP1 = crate::Reg<sd_rsp1::SD_RSP1_SPEC>;
#[doc = "SD Card Response Register 1"]
pub mod sd_rsp1;
#[doc = "SD_RSP32 (rw) register accessor: an alias for `Reg<SD_RSP32_SPEC>`"]
pub type SD_RSP32 = crate::Reg<sd_rsp32::SD_RSP32_SPEC>;
#[doc = "SD Card Response Register 32"]
pub mod sd_rsp32;
#[doc = "SD_RSP3 (r) register accessor: an alias for `Reg<SD_RSP3_SPEC>`"]
pub type SD_RSP3 = crate::Reg<sd_rsp3::SD_RSP3_SPEC>;
#[doc = "SD Card Response Register 3"]
pub mod sd_rsp3;
#[doc = "SD_RSP54 (rw) register accessor: an alias for `Reg<SD_RSP54_SPEC>`"]
pub type SD_RSP54 = crate::Reg<sd_rsp54::SD_RSP54_SPEC>;
#[doc = "SD Card Response Register 54"]
pub mod sd_rsp54;
#[doc = "SD_RSP5 (r) register accessor: an alias for `Reg<SD_RSP5_SPEC>`"]
pub type SD_RSP5 = crate::Reg<sd_rsp5::SD_RSP5_SPEC>;
#[doc = "SD Card Response Register 5"]
pub mod sd_rsp5;
#[doc = "SD_RSP76 (r) register accessor: an alias for `Reg<SD_RSP76_SPEC>`"]
pub type SD_RSP76 = crate::Reg<sd_rsp76::SD_RSP76_SPEC>;
#[doc = "SD Card Response Register 76"]
pub mod sd_rsp76;
#[doc = "SD_RSP7 (r) register accessor: an alias for `Reg<SD_RSP7_SPEC>`"]
pub type SD_RSP7 = crate::Reg<sd_rsp7::SD_RSP7_SPEC>;
#[doc = "SD Card Response Register 7"]
pub mod sd_rsp7;
#[doc = "SD_INFO1 (rw) register accessor: an alias for `Reg<SD_INFO1_SPEC>`"]
pub type SD_INFO1 = crate::Reg<sd_info1::SD_INFO1_SPEC>;
#[doc = "SD Card Interrupt Flag Register 1"]
pub mod sd_info1;
#[doc = "SD_INFO2 (rw) register accessor: an alias for `Reg<SD_INFO2_SPEC>`"]
pub type SD_INFO2 = crate::Reg<sd_info2::SD_INFO2_SPEC>;
#[doc = "SD Card Interrupt Flag Register 2"]
pub mod sd_info2;
#[doc = "SD_INFO1_MASK (rw) register accessor: an alias for `Reg<SD_INFO1_MASK_SPEC>`"]
pub type SD_INFO1_MASK = crate::Reg<sd_info1_mask::SD_INFO1_MASK_SPEC>;
#[doc = "SD INFO1 Interrupt Mask Register"]
pub mod sd_info1_mask;
#[doc = "SD_INFO2_MASK (rw) register accessor: an alias for `Reg<SD_INFO2_MASK_SPEC>`"]
pub type SD_INFO2_MASK = crate::Reg<sd_info2_mask::SD_INFO2_MASK_SPEC>;
#[doc = "SD INFO2 Interrupt Mask Register"]
pub mod sd_info2_mask;
#[doc = "SD_CLK_CTRL (rw) register accessor: an alias for `Reg<SD_CLK_CTRL_SPEC>`"]
pub type SD_CLK_CTRL = crate::Reg<sd_clk_ctrl::SD_CLK_CTRL_SPEC>;
#[doc = "SD Clock Control Register"]
pub mod sd_clk_ctrl;
#[doc = "SD_SIZE (rw) register accessor: an alias for `Reg<SD_SIZE_SPEC>`"]
pub type SD_SIZE = crate::Reg<sd_size::SD_SIZE_SPEC>;
#[doc = "Transfer Data Length Register"]
pub mod sd_size;
#[doc = "SD_OPTION (rw) register accessor: an alias for `Reg<SD_OPTION_SPEC>`"]
pub type SD_OPTION = crate::Reg<sd_option::SD_OPTION_SPEC>;
#[doc = "SD Card Access Control Option Register"]
pub mod sd_option;
#[doc = "SD_ERR_STS1 (r) register accessor: an alias for `Reg<SD_ERR_STS1_SPEC>`"]
pub type SD_ERR_STS1 = crate::Reg<sd_err_sts1::SD_ERR_STS1_SPEC>;
#[doc = "SD Error Status Register 1"]
pub mod sd_err_sts1;
#[doc = "SD_ERR_STS2 (r) register accessor: an alias for `Reg<SD_ERR_STS2_SPEC>`"]
pub type SD_ERR_STS2 = crate::Reg<sd_err_sts2::SD_ERR_STS2_SPEC>;
#[doc = "SD Error Status Register 2"]
pub mod sd_err_sts2;
#[doc = "SD_BUF0 (rw) register accessor: an alias for `Reg<SD_BUF0_SPEC>`"]
pub type SD_BUF0 = crate::Reg<sd_buf0::SD_BUF0_SPEC>;
#[doc = "SD Buffer Register"]
pub mod sd_buf0;
#[doc = "SDIO_MODE (rw) register accessor: an alias for `Reg<SDIO_MODE_SPEC>`"]
pub type SDIO_MODE = crate::Reg<sdio_mode::SDIO_MODE_SPEC>;
#[doc = "SDIO Mode Control Register"]
pub mod sdio_mode;
#[doc = "SDIO_INFO1 (rw) register accessor: an alias for `Reg<SDIO_INFO1_SPEC>`"]
pub type SDIO_INFO1 = crate::Reg<sdio_info1::SDIO_INFO1_SPEC>;
#[doc = "SDIO Interrupt Flag Register"]
pub mod sdio_info1;
#[doc = "SDIO_INFO1_MASK (rw) register accessor: an alias for `Reg<SDIO_INFO1_MASK_SPEC>`"]
pub type SDIO_INFO1_MASK = crate::Reg<sdio_info1_mask::SDIO_INFO1_MASK_SPEC>;
#[doc = "SDIO INFO1 Interrupt Mask Register"]
pub mod sdio_info1_mask;
#[doc = "SD_DMAEN (rw) register accessor: an alias for `Reg<SD_DMAEN_SPEC>`"]
pub type SD_DMAEN = crate::Reg<sd_dmaen::SD_DMAEN_SPEC>;
#[doc = "DMA Mode Enable Register"]
pub mod sd_dmaen;
#[doc = "SOFT_RST (rw) register accessor: an alias for `Reg<SOFT_RST_SPEC>`"]
pub type SOFT_RST = crate::Reg<soft_rst::SOFT_RST_SPEC>;
#[doc = "Software Reset Register"]
pub mod soft_rst;
#[doc = "SDIF_MODE (rw) register accessor: an alias for `Reg<SDIF_MODE_SPEC>`"]
pub type SDIF_MODE = crate::Reg<sdif_mode::SDIF_MODE_SPEC>;
#[doc = "SD Interface Mode Setting Register"]
pub mod sdif_mode;
#[doc = "EXT_SWAP (rw) register accessor: an alias for `Reg<EXT_SWAP_SPEC>`"]
pub type EXT_SWAP = crate::Reg<ext_swap::EXT_SWAP_SPEC>;
#[doc = "Swap Control Register"]
pub mod ext_swap;
