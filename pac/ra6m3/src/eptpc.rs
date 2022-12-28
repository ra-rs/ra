#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MINT Interrupt Source Status Register"]
    pub miesr: MIESR,
    #[doc = "0x04 - MINT Interrupt Request Permission Register"]
    pub mieipr: MIEIPR,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - ELC Output/IPLS Interrupt Request Permission Register"]
    pub elippr: ELIPPR,
    #[doc = "0x14 - ELC Output/IPLS Interrupt Permission Automatic Clearing Register"]
    pub elipacr: ELIPACR,
    _reserved4: [u8; 0x28],
    #[doc = "0x40 - STCA Status Register"]
    pub stsr: STSR,
    #[doc = "0x44 - STCA Status Notification Permission Register"]
    pub stipr: STIPR,
    _reserved6: [u8; 0x08],
    #[doc = "0x50 - STCA Clock Frequency Setting Register"]
    pub stcfr: STCFR,
    #[doc = "0x54 - STCA Operating Mode Register"]
    pub stmr: STMR,
    #[doc = "0x58 - Sync Message Reception Timeout Register"]
    pub syntor: SYNTOR,
    _reserved9: [u8; 0x04],
    #[doc = "0x60 - IPLS Interrupt Request Timer Select Register"]
    pub iptselr: IPTSELR,
    #[doc = "0x64 - MINT Interrupt Request Timer Select Register"]
    pub mitselr: MITSELR,
    #[doc = "0x68 - ELC Output Timer Select Register"]
    pub eltselr: ELTSELR,
    #[doc = "0x6c - Time Synchronization Channel Select Register"]
    pub stchselr: STCHSELR,
    _reserved13: [u8; 0x10],
    #[doc = "0x80 - Slave Time Synchronization Start Register"]
    pub synstartr: SYNSTARTR,
    #[doc = "0x84 - Local Time Counter Initial Value Load Directive Register"]
    pub lcivldr: LCIVLDR,
    _reserved15: [u8; 0x08],
    #[doc = "0x90 - Synchronization Loss Detection Threshold Registers"]
    pub syntdaru: SYNTDARU,
    #[doc = "0x94 - Synchronization Loss Detection Threshold Registers"]
    pub syntdarl: SYNTDARL,
    #[doc = "0x98 - Synchronization Detection Threshold Registers"]
    pub syntdbru: SYNTDBRU,
    #[doc = "0x9c - Synchronization Detection Threshold Registers"]
    pub syntdbrl: SYNTDBRL,
    _reserved19: [u8; 0x10],
    #[doc = "0xb0 - Local Time Counter Initial Value Registers"]
    pub lcivru: LCIVRU,
    #[doc = "0xb4 - Local Time Counter Initial Value Registers"]
    pub lcivrm: LCIVRM,
    #[doc = "0xb8 - Local Time Counter Initial Value Registers"]
    pub lcivrl: LCIVRL,
    _reserved22: [u8; 0x68],
    #[doc = "0x124 - Worst 10 Acquisition Directive Register"]
    pub getw10r: GETW10R,
    #[doc = "0x128 - Positive Gradient Limit Registers"]
    pub plimitru: PLIMITRU,
    #[doc = "0x12c - Positive Gradient Limit Registers"]
    pub plimitrm: PLIMITRM,
    #[doc = "0x130 - Positive Gradient Limit Registers"]
    pub plimitrl: PLIMITRL,
    #[doc = "0x134 - Negative Gradient Limit Registers"]
    pub mlimitru: MLIMITRU,
    #[doc = "0x138 - Negative Gradient Limit Registers"]
    pub mlimitrm: MLIMITRM,
    #[doc = "0x13c - Negative Gradient Limit Registers"]
    pub mlimitrl: MLIMITRL,
    #[doc = "0x140 - Statistical Information Retention Control Register"]
    pub getinfor: GETINFOR,
    _reserved30: [u8; 0x2c],
    #[doc = "0x170 - Local Time Counters"]
    pub lccvru: LCCVRU,
    #[doc = "0x174 - Local Time Counters"]
    pub lccvrm: LCCVRM,
    #[doc = "0x178 - Local Time Counters"]
    pub lccvrl: LCCVRL,
    _reserved33: [u8; 0x94],
    #[doc = "0x210 - Positive Gradient Worst 10 Value Registers"]
    pub pw10vru: PW10VRU,
    #[doc = "0x214 - Positive Gradient Worst 10 Value Registers"]
    pub pw10vrm: PW10VRM,
    #[doc = "0x218 - Positive Gradient Worst 10 Value Registers"]
    pub pw10vrl: PW10VRL,
    _reserved36: [u8; 0xb4],
    #[doc = "0x2d0 - Negative Gradient Worst 10 Value Registers"]
    pub mw10ru: MW10RU,
    #[doc = "0x2d4 - Negative Gradient Worst 10 Value Registers"]
    pub mw10rm: MW10RM,
    #[doc = "0x2d8 - Negative Gradient Worst 10 Value Registers"]
    pub mw10rl: MW10RL,
    _reserved39: [u8; 0x24],
    #[doc = "0x300 - Timer Start Time Setting Register %s"]
    pub tmsttru0: TMSTTRU,
    #[doc = "0x304 - Timer Start Time Setting Register %s"]
    pub tmsttrl0: TMSTTRL,
    #[doc = "0x308 - Timer Cycle Setting Registers %s"]
    pub tmcycr0: TMCYCR,
    #[doc = "0x30c - Timer Pulse Width Setting Register %s"]
    pub tmplsr0: TMPLSR,
    #[doc = "0x310 - Timer Start Time Setting Register %s"]
    pub tmsttru1: TMSTTRU,
    #[doc = "0x314 - Timer Start Time Setting Register %s"]
    pub tmsttrl1: TMSTTRL,
    #[doc = "0x318 - Timer Cycle Setting Registers %s"]
    pub tmcycr1: TMCYCR,
    #[doc = "0x31c - Timer Pulse Width Setting Register %s"]
    pub tmplsr1: TMPLSR,
    #[doc = "0x320 - Timer Start Time Setting Register %s"]
    pub tmsttru2: TMSTTRU,
    #[doc = "0x324 - Timer Start Time Setting Register %s"]
    pub tmsttrl2: TMSTTRL,
    #[doc = "0x328 - Timer Cycle Setting Registers %s"]
    pub tmcycr2: TMCYCR,
    #[doc = "0x32c - Timer Pulse Width Setting Register %s"]
    pub tmplsr2: TMPLSR,
    #[doc = "0x330 - Timer Start Time Setting Register %s"]
    pub tmsttru3: TMSTTRU,
    #[doc = "0x334 - Timer Start Time Setting Register %s"]
    pub tmsttrl3: TMSTTRL,
    #[doc = "0x338 - Timer Cycle Setting Registers %s"]
    pub tmcycr3: TMCYCR,
    #[doc = "0x33c - Timer Pulse Width Setting Register %s"]
    pub tmplsr3: TMPLSR,
    #[doc = "0x340 - Timer Start Time Setting Register %s"]
    pub tmsttru4: TMSTTRU,
    #[doc = "0x344 - Timer Start Time Setting Register %s"]
    pub tmsttrl4: TMSTTRL,
    #[doc = "0x348 - Timer Cycle Setting Registers %s"]
    pub tmcycr4: TMCYCR,
    #[doc = "0x34c - Timer Pulse Width Setting Register %s"]
    pub tmplsr4: TMPLSR,
    #[doc = "0x350 - Timer Start Time Setting Register %s"]
    pub tmsttru5: TMSTTRU,
    #[doc = "0x354 - Timer Start Time Setting Register %s"]
    pub tmsttrl5: TMSTTRL,
    #[doc = "0x358 - Timer Cycle Setting Registers %s"]
    pub tmcycr5: TMCYCR,
    #[doc = "0x35c - Timer Pulse Width Setting Register %s"]
    pub tmplsr5: TMPLSR,
    _reserved63: [u8; 0x1c],
    #[doc = "0x37c - Timer Start Register"]
    pub tmstartr: TMSTARTR,
}
#[doc = "MIESR (rw) register accessor: an alias for `Reg<MIESR_SPEC>`"]
pub type MIESR = crate::Reg<miesr::MIESR_SPEC>;
#[doc = "MINT Interrupt Source Status Register"]
pub mod miesr;
#[doc = "MIEIPR (rw) register accessor: an alias for `Reg<MIEIPR_SPEC>`"]
pub type MIEIPR = crate::Reg<mieipr::MIEIPR_SPEC>;
#[doc = "MINT Interrupt Request Permission Register"]
pub mod mieipr;
#[doc = "ELIPPR (rw) register accessor: an alias for `Reg<ELIPPR_SPEC>`"]
pub type ELIPPR = crate::Reg<elippr::ELIPPR_SPEC>;
#[doc = "ELC Output/IPLS Interrupt Request Permission Register"]
pub mod elippr;
#[doc = "ELIPACR (rw) register accessor: an alias for `Reg<ELIPACR_SPEC>`"]
pub type ELIPACR = crate::Reg<elipacr::ELIPACR_SPEC>;
#[doc = "ELC Output/IPLS Interrupt Permission Automatic Clearing Register"]
pub mod elipacr;
#[doc = "STSR (rw) register accessor: an alias for `Reg<STSR_SPEC>`"]
pub type STSR = crate::Reg<stsr::STSR_SPEC>;
#[doc = "STCA Status Register"]
pub mod stsr;
#[doc = "STIPR (rw) register accessor: an alias for `Reg<STIPR_SPEC>`"]
pub type STIPR = crate::Reg<stipr::STIPR_SPEC>;
#[doc = "STCA Status Notification Permission Register"]
pub mod stipr;
#[doc = "STCFR (rw) register accessor: an alias for `Reg<STCFR_SPEC>`"]
pub type STCFR = crate::Reg<stcfr::STCFR_SPEC>;
#[doc = "STCA Clock Frequency Setting Register"]
pub mod stcfr;
#[doc = "STMR (rw) register accessor: an alias for `Reg<STMR_SPEC>`"]
pub type STMR = crate::Reg<stmr::STMR_SPEC>;
#[doc = "STCA Operating Mode Register"]
pub mod stmr;
#[doc = "SYNTOR (rw) register accessor: an alias for `Reg<SYNTOR_SPEC>`"]
pub type SYNTOR = crate::Reg<syntor::SYNTOR_SPEC>;
#[doc = "Sync Message Reception Timeout Register"]
pub mod syntor;
#[doc = "IPTSELR (rw) register accessor: an alias for `Reg<IPTSELR_SPEC>`"]
pub type IPTSELR = crate::Reg<iptselr::IPTSELR_SPEC>;
#[doc = "IPLS Interrupt Request Timer Select Register"]
pub mod iptselr;
#[doc = "MITSELR (rw) register accessor: an alias for `Reg<MITSELR_SPEC>`"]
pub type MITSELR = crate::Reg<mitselr::MITSELR_SPEC>;
#[doc = "MINT Interrupt Request Timer Select Register"]
pub mod mitselr;
#[doc = "ELTSELR (rw) register accessor: an alias for `Reg<ELTSELR_SPEC>`"]
pub type ELTSELR = crate::Reg<eltselr::ELTSELR_SPEC>;
#[doc = "ELC Output Timer Select Register"]
pub mod eltselr;
#[doc = "STCHSELR (rw) register accessor: an alias for `Reg<STCHSELR_SPEC>`"]
pub type STCHSELR = crate::Reg<stchselr::STCHSELR_SPEC>;
#[doc = "Time Synchronization Channel Select Register"]
pub mod stchselr;
#[doc = "SYNSTARTR (rw) register accessor: an alias for `Reg<SYNSTARTR_SPEC>`"]
pub type SYNSTARTR = crate::Reg<synstartr::SYNSTARTR_SPEC>;
#[doc = "Slave Time Synchronization Start Register"]
pub mod synstartr;
#[doc = "LCIVLDR (w) register accessor: an alias for `Reg<LCIVLDR_SPEC>`"]
pub type LCIVLDR = crate::Reg<lcivldr::LCIVLDR_SPEC>;
#[doc = "Local Time Counter Initial Value Load Directive Register"]
pub mod lcivldr;
#[doc = "SYNTDARU (rw) register accessor: an alias for `Reg<SYNTDARU_SPEC>`"]
pub type SYNTDARU = crate::Reg<syntdaru::SYNTDARU_SPEC>;
#[doc = "Synchronization Loss Detection Threshold Registers"]
pub mod syntdaru;
#[doc = "SYNTDARL (rw) register accessor: an alias for `Reg<SYNTDARL_SPEC>`"]
pub type SYNTDARL = crate::Reg<syntdarl::SYNTDARL_SPEC>;
#[doc = "Synchronization Loss Detection Threshold Registers"]
pub mod syntdarl;
#[doc = "SYNTDBRU (rw) register accessor: an alias for `Reg<SYNTDBRU_SPEC>`"]
pub type SYNTDBRU = crate::Reg<syntdbru::SYNTDBRU_SPEC>;
#[doc = "Synchronization Detection Threshold Registers"]
pub mod syntdbru;
#[doc = "SYNTDBRL (rw) register accessor: an alias for `Reg<SYNTDBRL_SPEC>`"]
pub type SYNTDBRL = crate::Reg<syntdbrl::SYNTDBRL_SPEC>;
#[doc = "Synchronization Detection Threshold Registers"]
pub mod syntdbrl;
#[doc = "LCIVRU (rw) register accessor: an alias for `Reg<LCIVRU_SPEC>`"]
pub type LCIVRU = crate::Reg<lcivru::LCIVRU_SPEC>;
#[doc = "Local Time Counter Initial Value Registers"]
pub mod lcivru;
#[doc = "LCIVRM (rw) register accessor: an alias for `Reg<LCIVRM_SPEC>`"]
pub type LCIVRM = crate::Reg<lcivrm::LCIVRM_SPEC>;
#[doc = "Local Time Counter Initial Value Registers"]
pub mod lcivrm;
#[doc = "LCIVRL (rw) register accessor: an alias for `Reg<LCIVRL_SPEC>`"]
pub type LCIVRL = crate::Reg<lcivrl::LCIVRL_SPEC>;
#[doc = "Local Time Counter Initial Value Registers"]
pub mod lcivrl;
#[doc = "GETW10R (rw) register accessor: an alias for `Reg<GETW10R_SPEC>`"]
pub type GETW10R = crate::Reg<getw10r::GETW10R_SPEC>;
#[doc = "Worst 10 Acquisition Directive Register"]
pub mod getw10r;
#[doc = "PLIMITRU (rw) register accessor: an alias for `Reg<PLIMITRU_SPEC>`"]
pub type PLIMITRU = crate::Reg<plimitru::PLIMITRU_SPEC>;
#[doc = "Positive Gradient Limit Registers"]
pub mod plimitru;
#[doc = "PLIMITRM (rw) register accessor: an alias for `Reg<PLIMITRM_SPEC>`"]
pub type PLIMITRM = crate::Reg<plimitrm::PLIMITRM_SPEC>;
#[doc = "Positive Gradient Limit Registers"]
pub mod plimitrm;
#[doc = "PLIMITRL (rw) register accessor: an alias for `Reg<PLIMITRL_SPEC>`"]
pub type PLIMITRL = crate::Reg<plimitrl::PLIMITRL_SPEC>;
#[doc = "Positive Gradient Limit Registers"]
pub mod plimitrl;
#[doc = "MLIMITRU (rw) register accessor: an alias for `Reg<MLIMITRU_SPEC>`"]
pub type MLIMITRU = crate::Reg<mlimitru::MLIMITRU_SPEC>;
#[doc = "Negative Gradient Limit Registers"]
pub mod mlimitru;
#[doc = "MLIMITRM (rw) register accessor: an alias for `Reg<MLIMITRM_SPEC>`"]
pub type MLIMITRM = crate::Reg<mlimitrm::MLIMITRM_SPEC>;
#[doc = "Negative Gradient Limit Registers"]
pub mod mlimitrm;
#[doc = "MLIMITRL (rw) register accessor: an alias for `Reg<MLIMITRL_SPEC>`"]
pub type MLIMITRL = crate::Reg<mlimitrl::MLIMITRL_SPEC>;
#[doc = "Negative Gradient Limit Registers"]
pub mod mlimitrl;
#[doc = "GETINFOR (rw) register accessor: an alias for `Reg<GETINFOR_SPEC>`"]
pub type GETINFOR = crate::Reg<getinfor::GETINFOR_SPEC>;
#[doc = "Statistical Information Retention Control Register"]
pub mod getinfor;
#[doc = "LCCVRU (r) register accessor: an alias for `Reg<LCCVRU_SPEC>`"]
pub type LCCVRU = crate::Reg<lccvru::LCCVRU_SPEC>;
#[doc = "Local Time Counters"]
pub mod lccvru;
#[doc = "LCCVRM (r) register accessor: an alias for `Reg<LCCVRM_SPEC>`"]
pub type LCCVRM = crate::Reg<lccvrm::LCCVRM_SPEC>;
#[doc = "Local Time Counters"]
pub mod lccvrm;
#[doc = "LCCVRL (r) register accessor: an alias for `Reg<LCCVRL_SPEC>`"]
pub type LCCVRL = crate::Reg<lccvrl::LCCVRL_SPEC>;
#[doc = "Local Time Counters"]
pub mod lccvrl;
#[doc = "PW10VRU (r) register accessor: an alias for `Reg<PW10VRU_SPEC>`"]
pub type PW10VRU = crate::Reg<pw10vru::PW10VRU_SPEC>;
#[doc = "Positive Gradient Worst 10 Value Registers"]
pub mod pw10vru;
#[doc = "PW10VRM (r) register accessor: an alias for `Reg<PW10VRM_SPEC>`"]
pub type PW10VRM = crate::Reg<pw10vrm::PW10VRM_SPEC>;
#[doc = "Positive Gradient Worst 10 Value Registers"]
pub mod pw10vrm;
#[doc = "PW10VRL (r) register accessor: an alias for `Reg<PW10VRL_SPEC>`"]
pub type PW10VRL = crate::Reg<pw10vrl::PW10VRL_SPEC>;
#[doc = "Positive Gradient Worst 10 Value Registers"]
pub mod pw10vrl;
#[doc = "MW10RU (r) register accessor: an alias for `Reg<MW10RU_SPEC>`"]
pub type MW10RU = crate::Reg<mw10ru::MW10RU_SPEC>;
#[doc = "Negative Gradient Worst 10 Value Registers"]
pub mod mw10ru;
#[doc = "MW10RM (r) register accessor: an alias for `Reg<MW10RM_SPEC>`"]
pub type MW10RM = crate::Reg<mw10rm::MW10RM_SPEC>;
#[doc = "Negative Gradient Worst 10 Value Registers"]
pub mod mw10rm;
#[doc = "MW10RL (r) register accessor: an alias for `Reg<MW10RL_SPEC>`"]
pub type MW10RL = crate::Reg<mw10rl::MW10RL_SPEC>;
#[doc = "Negative Gradient Worst 10 Value Registers"]
pub mod mw10rl;
#[doc = "TMSTTRU (rw) register accessor: an alias for `Reg<TMSTTRU_SPEC>`"]
pub type TMSTTRU = crate::Reg<tmsttru::TMSTTRU_SPEC>;
#[doc = "Timer Start Time Setting Register %s"]
pub mod tmsttru;
#[doc = "TMSTTRL (rw) register accessor: an alias for `Reg<TMSTTRL_SPEC>`"]
pub type TMSTTRL = crate::Reg<tmsttrl::TMSTTRL_SPEC>;
#[doc = "Timer Start Time Setting Register %s"]
pub mod tmsttrl;
#[doc = "TMCYCR (rw) register accessor: an alias for `Reg<TMCYCR_SPEC>`"]
pub type TMCYCR = crate::Reg<tmcycr::TMCYCR_SPEC>;
#[doc = "Timer Cycle Setting Registers %s"]
pub mod tmcycr;
#[doc = "TMPLSR (rw) register accessor: an alias for `Reg<TMPLSR_SPEC>`"]
pub type TMPLSR = crate::Reg<tmplsr::TMPLSR_SPEC>;
#[doc = "Timer Pulse Width Setting Register %s"]
pub mod tmplsr;
#[doc = "TMSTARTR (rw) register accessor: an alias for `Reg<TMSTARTR_SPEC>`"]
pub type TMSTARTR = crate::Reg<tmstartr::TMSTARTR_SPEC>;
#[doc = "Timer Start Register"]
pub mod tmstartr;
