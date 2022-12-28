#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - A/D Conversion Clock Enable Register"]
    pub adclkenr: ADCLKENR,
    #[doc = "0x04 - A/D Conversion Clock Status Register"]
    pub adclksr: ADCLKSR,
    #[doc = "0x08 - A/D Conversion Clock Control Register"]
    pub adclkcr: ADCLKCR,
    #[doc = "0x0c - A/D Converter Synchronous Operation Control Register"]
    pub adsycr: ADSYCR,
    _reserved4: [u8; 0x10],
    #[doc = "0x20 - A/D Conversion Error Interrupt Enable Register"]
    pub aderintcr: ADERINTCR,
    #[doc = "0x24 - A/D Conversion Overflow Interrupt Enable Register"]
    pub adovfintcr: ADOVFINTCR,
    #[doc = "0x28 - Calibration interrupt Enable Register"]
    pub adcalintcr: ADCALINTCR,
    _reserved7: [u8; 0x14],
    #[doc = "0x40 - A/D Converter Mode Selection Register"]
    pub admdr: ADMDR,
    #[doc = "0x44 - A/D Group scan Priority Control Register"]
    pub adgspcr: ADGSPCR,
    #[doc = "0x48 - Scan Group Enable Register"]
    pub adsger: ADSGER,
    #[doc = "0x4c - Scan Group Control Register 0"]
    pub adsgcr0: ADSGCR0,
    #[doc = "0x50 - Scan Group Control Register 1"]
    pub adsgcr1: ADSGCR1,
    #[doc = "0x54 - Scan Group Control Register 2"]
    pub adsgcr2: ADSGCR2,
    _reserved13: [u8; 0x04],
    #[doc = "0x5c - Scan End Interrupt Enable Register"]
    pub adintcr: ADINTCR,
    _reserved14: [u8; 0x60],
    #[doc = "0xc0 - External Trigger Enable Register %s"]
    pub adtrgext0: ADTRGEXT,
    #[doc = "0xc4 - ELC Trigger Enable Register %s"]
    pub adtrgelc0: ADTRGELC,
    #[doc = "0xc8 - GPT Trigger Enable Register %s"]
    pub adtrggpt0: ADTRGGPT,
    _reserved17: [u8; 0x04],
    #[doc = "0xd0 - External Trigger Enable Register %s"]
    pub adtrgext1: ADTRGEXT,
    #[doc = "0xd4 - ELC Trigger Enable Register %s"]
    pub adtrgelc1: ADTRGELC,
    #[doc = "0xd8 - GPT Trigger Enable Register %s"]
    pub adtrggpt1: ADTRGGPT,
    _reserved20: [u8; 0x04],
    #[doc = "0xe0 - External Trigger Enable Register %s"]
    pub adtrgext2: ADTRGEXT,
    #[doc = "0xe4 - ELC Trigger Enable Register %s"]
    pub adtrgelc2: ADTRGELC,
    #[doc = "0xe8 - GPT Trigger Enable Register %s"]
    pub adtrggpt2: ADTRGGPT,
    _reserved23: [u8; 0x04],
    #[doc = "0xf0 - External Trigger Enable Register %s"]
    pub adtrgext3: ADTRGEXT,
    #[doc = "0xf4 - ELC Trigger Enable Register %s"]
    pub adtrgelc3: ADTRGELC,
    #[doc = "0xf8 - GPT Trigger Enable Register %s"]
    pub adtrggpt3: ADTRGGPT,
    _reserved26: [u8; 0x04],
    #[doc = "0x100 - External Trigger Enable Register %s"]
    pub adtrgext4: ADTRGEXT,
    #[doc = "0x104 - ELC Trigger Enable Register %s"]
    pub adtrgelc4: ADTRGELC,
    #[doc = "0x108 - GPT Trigger Enable Register %s"]
    pub adtrggpt4: ADTRGGPT,
    _reserved29: [u8; 0x04],
    #[doc = "0x110 - External Trigger Enable Register %s"]
    pub adtrgext5: ADTRGEXT,
    #[doc = "0x114 - ELC Trigger Enable Register %s"]
    pub adtrgelc5: ADTRGELC,
    #[doc = "0x118 - GPT Trigger Enable Register %s"]
    pub adtrggpt5: ADTRGGPT,
    _reserved32: [u8; 0x04],
    #[doc = "0x120 - External Trigger Enable Register %s"]
    pub adtrgext6: ADTRGEXT,
    #[doc = "0x124 - ELC Trigger Enable Register %s"]
    pub adtrgelc6: ADTRGELC,
    #[doc = "0x128 - GPT Trigger Enable Register %s"]
    pub adtrggpt6: ADTRGGPT,
    _reserved35: [u8; 0x04],
    #[doc = "0x130 - External Trigger Enable Register %s"]
    pub adtrgext7: ADTRGEXT,
    #[doc = "0x134 - ELC Trigger Enable Register %s"]
    pub adtrgelc7: ADTRGELC,
    #[doc = "0x138 - GPT Trigger Enable Register %s"]
    pub adtrggpt7: ADTRGGPT,
    _reserved38: [u8; 0x04],
    #[doc = "0x140 - External Trigger Enable Register %s"]
    pub adtrgext8: ADTRGEXT,
    #[doc = "0x144 - ELC Trigger Enable Register %s"]
    pub adtrgelc8: ADTRGELC,
    #[doc = "0x148 - GPT Trigger Enable Register %s"]
    pub adtrggpt8: ADTRGGPT,
    _reserved41: [u8; 0x74],
    #[doc = "0x1c0 - A/D Conversion Start Trigger Delay Register 0"]
    pub adtrgdlr0: ADTRGDLR0,
    #[doc = "0x1c4 - A/D Conversion Start Trigger Delay Register 1"]
    pub adtrgdlr1: ADTRGDLR1,
    #[doc = "0x1c8 - A/D Conversion Start Trigger Delay Register 2"]
    pub adtrgdlr2: ADTRGDLR2,
    #[doc = "0x1cc - A/D Conversion Start Trigger Delay Register 3"]
    pub adtrgdlr3: ADTRGDLR3,
    #[doc = "0x1d0 - A/D Conversion Start Trigger Delay Register 4"]
    pub adtrgdlr4: ADTRGDLR4,
    _reserved46: [u8; 0x2c],
    #[doc = "0x200..0x224 - Scan Group Diagnosis Function Control Register %s"]
    pub adsgdcr: [ADSGDCR; 9],
    _reserved47: [u8; 0x1c],
    #[doc = "0x240 - Sampling State Table Register 0"]
    pub adsstr0: ADSSTR0,
    #[doc = "0x244 - Sampling State Table Register 1"]
    pub adsstr1: ADSSTR1,
    #[doc = "0x248 - Sampling State Table Register 2"]
    pub adsstr2: ADSSTR2,
    #[doc = "0x24c - Sampling State Table Register 3"]
    pub adsstr3: ADSSTR3,
    #[doc = "0x250 - Sampling State Table Register 4"]
    pub adsstr4: ADSSTR4,
    #[doc = "0x254 - Sampling State Table Register 5"]
    pub adsstr5: ADSSTR5,
    #[doc = "0x258 - Sampling State Table Register 6"]
    pub adsstr6: ADSSTR6,
    #[doc = "0x25c - Sampling State Table Register 7"]
    pub adsstr7: ADSSTR7,
    #[doc = "0x260 - A/D Conversion State Register"]
    pub adcnvstr: ADCNVSTR,
    #[doc = "0x264 - A/D Converter Self-calibration State Register"]
    pub adcalstcr: ADCALSTCR,
    _reserved57: [u8; 0x18],
    #[doc = "0x280 - Channel-dedicated Sample-and-hold Circuit Control Register 0"]
    pub adshcr0: ADSHCR0,
    _reserved58: [u8; 0x04],
    #[doc = "0x288 - Channel-dedicated Sample-and-hold Circuit State Register 0"]
    pub adshstr0: ADSHSTR0,
    #[doc = "0x28c - Channel-dedicated Sample-and-hold Circuit Control Register 1"]
    pub adshcr1: ADSHCR1,
    _reserved60: [u8; 0x04],
    #[doc = "0x294 - Channel-dedicated Sample-and-hold Circuit State Register 1"]
    pub adshstr1: ADSHSTR1,
    _reserved61: [u8; 0x18],
    #[doc = "0x2b0 - Channel-dedicated Sample-and-hold Circuit Self-calibration State Register"]
    pub adcalshcr: ADCALSHCR,
    _reserved62: [u8; 0x0c],
    #[doc = "0x2c0..0x2d0 - Programmable Gain Amplifier Control Register %s"]
    pub adpgacr: [ADPGACR; 4],
    _reserved63: [u8; 0x30],
    #[doc = "0x300 - Programable Gain Amp Monitor Output Control Register"]
    pub adpgamoncr: ADPGAMONCR,
    _reserved64: [u8; 0x1c],
    #[doc = "0x320 - Internal Reference Voltage Monitor Enable Register"]
    pub adrefcr: ADREFCR,
    _reserved65: [u8; 0x1c],
    #[doc = "0x340..0x348 - A/D Converter Digital Filter Selection Register %s"]
    pub addfsr: [ADDFSR; 2],
    _reserved66: [u8; 0x18],
    #[doc = "0x360..0x380 - User Offset Table Register %s"]
    pub aduoftr: [ADUOFTR; 8],
    #[doc = "0x380..0x3a0 - User Gain Table Register %s"]
    pub adugtr: [ADUGTR; 8],
    #[doc = "0x3a0 - Limiter Clip Interrupt Enable Register"]
    pub adlimintcr: ADLIMINTCR,
    #[doc = "0x3a4..0x3c4 - Limiter Clip Table Register %s"]
    pub adlimtr: [ADLIMTR; 8],
    _reserved70: [u8; 0x3c],
    #[doc = "0x400 - Compare Match Enable Register"]
    pub adcmpenr: ADCMPENR,
    #[doc = "0x404 - Compare Match Interrupt Enable Register"]
    pub adcmpintcr: ADCMPINTCR,
    #[doc = "0x408..0x410 - Composite Compare Match Configuration Register %s"]
    pub adccmpcr: [ADCCMPCR; 2],
    _reserved73: [u8; 0x38],
    #[doc = "0x448 - Compare Match Mode Selection Register 0"]
    pub adcmpmdr0: ADCMPMDR0,
    #[doc = "0x44c - Compare Match Mode Selection Register 1"]
    pub adcmpmdr1: ADCMPMDR1,
    _reserved75: [u8; 0x08],
    #[doc = "0x458..0x478 - Compare Match Table Register %s"]
    pub adcmptbr: [ADCMPTBR; 8],
    _reserved76: [u8; 0x48],
    #[doc = "0x4c0 - FIFO Control Register"]
    pub adfifocr: ADFIFOCR,
    #[doc = "0x4c4 - FIFO Interrupt Control Register"]
    pub adfifointcr: ADFIFOINTCR,
    #[doc = "0x4c8 - FIFO Interrupt Generation Level Register 0"]
    pub adfifointlr0: ADFIFOINTLR0,
    #[doc = "0x4cc - FIFO Interrupt Generation Level Register 1"]
    pub adfifointlr1: ADFIFOINTLR1,
    #[doc = "0x4d0 - FIFO Interrupt Generation Level Register 2"]
    pub adfifointlr2: ADFIFOINTLR2,
    #[doc = "0x4d4 - FIFO Interrupt Generation Level Register 3"]
    pub adfifointlr3: ADFIFOINTLR3,
    #[doc = "0x4d8 - FIFO Interrupt Generation Level Register 4"]
    pub adfifointlr4: ADFIFOINTLR4,
    _reserved83: [u8; 0x0124],
    #[doc = "0x600 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr0: ADCHCR,
    #[doc = "0x604 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra0: ADDOPCRA,
    #[doc = "0x608 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb0: ADDOPCRB,
    #[doc = "0x60c - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc0: ADDOPCRC,
    #[doc = "0x610 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr1: ADCHCR,
    #[doc = "0x614 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra1: ADDOPCRA,
    #[doc = "0x618 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb1: ADDOPCRB,
    #[doc = "0x61c - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc1: ADDOPCRC,
    #[doc = "0x620 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr2: ADCHCR,
    #[doc = "0x624 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra2: ADDOPCRA,
    #[doc = "0x628 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb2: ADDOPCRB,
    #[doc = "0x62c - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc2: ADDOPCRC,
    #[doc = "0x630 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr3: ADCHCR,
    #[doc = "0x634 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra3: ADDOPCRA,
    #[doc = "0x638 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb3: ADDOPCRB,
    #[doc = "0x63c - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc3: ADDOPCRC,
    #[doc = "0x640 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr4: ADCHCR,
    #[doc = "0x644 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra4: ADDOPCRA,
    #[doc = "0x648 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb4: ADDOPCRB,
    #[doc = "0x64c - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc4: ADDOPCRC,
    #[doc = "0x650 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr5: ADCHCR,
    #[doc = "0x654 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra5: ADDOPCRA,
    #[doc = "0x658 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb5: ADDOPCRB,
    #[doc = "0x65c - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc5: ADDOPCRC,
    #[doc = "0x660 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr6: ADCHCR,
    #[doc = "0x664 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra6: ADDOPCRA,
    #[doc = "0x668 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb6: ADDOPCRB,
    #[doc = "0x66c - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc6: ADDOPCRC,
    #[doc = "0x670 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr7: ADCHCR,
    #[doc = "0x674 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra7: ADDOPCRA,
    #[doc = "0x678 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb7: ADDOPCRB,
    #[doc = "0x67c - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc7: ADDOPCRC,
    #[doc = "0x680 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr8: ADCHCR,
    #[doc = "0x684 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra8: ADDOPCRA,
    #[doc = "0x688 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb8: ADDOPCRB,
    #[doc = "0x68c - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc8: ADDOPCRC,
    #[doc = "0x690 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr9: ADCHCR,
    #[doc = "0x694 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra9: ADDOPCRA,
    #[doc = "0x698 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb9: ADDOPCRB,
    #[doc = "0x69c - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc9: ADDOPCRC,
    #[doc = "0x6a0 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr10: ADCHCR,
    #[doc = "0x6a4 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra10: ADDOPCRA,
    #[doc = "0x6a8 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb10: ADDOPCRB,
    #[doc = "0x6ac - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc10: ADDOPCRC,
    #[doc = "0x6b0 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr11: ADCHCR,
    #[doc = "0x6b4 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra11: ADDOPCRA,
    #[doc = "0x6b8 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb11: ADDOPCRB,
    #[doc = "0x6bc - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc11: ADDOPCRC,
    #[doc = "0x6c0 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr12: ADCHCR,
    #[doc = "0x6c4 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra12: ADDOPCRA,
    #[doc = "0x6c8 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb12: ADDOPCRB,
    #[doc = "0x6cc - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc12: ADDOPCRC,
    #[doc = "0x6d0 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr13: ADCHCR,
    #[doc = "0x6d4 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra13: ADDOPCRA,
    #[doc = "0x6d8 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb13: ADDOPCRB,
    #[doc = "0x6dc - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc13: ADDOPCRC,
    #[doc = "0x6e0 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr14: ADCHCR,
    #[doc = "0x6e4 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra14: ADDOPCRA,
    #[doc = "0x6e8 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb14: ADDOPCRB,
    #[doc = "0x6ec - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc14: ADDOPCRC,
    #[doc = "0x6f0 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr15: ADCHCR,
    #[doc = "0x6f4 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra15: ADDOPCRA,
    #[doc = "0x6f8 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb15: ADDOPCRB,
    #[doc = "0x6fc - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc15: ADDOPCRC,
    #[doc = "0x700 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr16: ADCHCR,
    #[doc = "0x704 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra16: ADDOPCRA,
    #[doc = "0x708 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb16: ADDOPCRB,
    #[doc = "0x70c - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc16: ADDOPCRC,
    #[doc = "0x710 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr17: ADCHCR,
    #[doc = "0x714 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra17: ADDOPCRA,
    #[doc = "0x718 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb17: ADDOPCRB,
    #[doc = "0x71c - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc17: ADDOPCRC,
    #[doc = "0x720 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr18: ADCHCR,
    #[doc = "0x724 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra18: ADDOPCRA,
    #[doc = "0x728 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb18: ADDOPCRB,
    #[doc = "0x72c - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc18: ADDOPCRC,
    #[doc = "0x730 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr19: ADCHCR,
    #[doc = "0x734 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra19: ADDOPCRA,
    #[doc = "0x738 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb19: ADDOPCRB,
    #[doc = "0x73c - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc19: ADDOPCRC,
    #[doc = "0x740 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr20: ADCHCR,
    #[doc = "0x744 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra20: ADDOPCRA,
    #[doc = "0x748 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb20: ADDOPCRB,
    #[doc = "0x74c - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc20: ADDOPCRC,
    #[doc = "0x750 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr21: ADCHCR,
    #[doc = "0x754 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra21: ADDOPCRA,
    #[doc = "0x758 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb21: ADDOPCRB,
    #[doc = "0x75c - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc21: ADDOPCRC,
    #[doc = "0x760 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr22: ADCHCR,
    #[doc = "0x764 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra22: ADDOPCRA,
    #[doc = "0x768 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb22: ADDOPCRB,
    #[doc = "0x76c - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc22: ADDOPCRC,
    #[doc = "0x770 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr23: ADCHCR,
    #[doc = "0x774 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra23: ADDOPCRA,
    #[doc = "0x778 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb23: ADDOPCRB,
    #[doc = "0x77c - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc23: ADDOPCRC,
    #[doc = "0x780 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr24: ADCHCR,
    #[doc = "0x784 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra24: ADDOPCRA,
    #[doc = "0x788 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb24: ADDOPCRB,
    #[doc = "0x78c - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc24: ADDOPCRC,
    #[doc = "0x790 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr25: ADCHCR,
    #[doc = "0x794 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra25: ADDOPCRA,
    #[doc = "0x798 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb25: ADDOPCRB,
    #[doc = "0x79c - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc25: ADDOPCRC,
    #[doc = "0x7a0 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr26: ADCHCR,
    #[doc = "0x7a4 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra26: ADDOPCRA,
    #[doc = "0x7a8 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb26: ADDOPCRB,
    #[doc = "0x7ac - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc26: ADDOPCRC,
    #[doc = "0x7b0 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr27: ADCHCR,
    #[doc = "0x7b4 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra27: ADDOPCRA,
    #[doc = "0x7b8 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb27: ADDOPCRB,
    #[doc = "0x7bc - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc27: ADDOPCRC,
    #[doc = "0x7c0 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr28: ADCHCR,
    #[doc = "0x7c4 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra28: ADDOPCRA,
    #[doc = "0x7c8 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb28: ADDOPCRB,
    #[doc = "0x7cc - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc28: ADDOPCRC,
    #[doc = "0x7d0 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr29: ADCHCR,
    #[doc = "0x7d4 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra29: ADDOPCRA,
    #[doc = "0x7d8 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb29: ADDOPCRB,
    #[doc = "0x7dc - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc29: ADDOPCRC,
    #[doc = "0x7e0 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr30: ADCHCR,
    #[doc = "0x7e4 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra30: ADDOPCRA,
    #[doc = "0x7e8 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb30: ADDOPCRB,
    #[doc = "0x7ec - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc30: ADDOPCRC,
    #[doc = "0x7f0 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr31: ADCHCR,
    #[doc = "0x7f4 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra31: ADDOPCRA,
    #[doc = "0x7f8 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb31: ADDOPCRB,
    #[doc = "0x7fc - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc31: ADDOPCRC,
    #[doc = "0x800 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr32: ADCHCR,
    #[doc = "0x804 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra32: ADDOPCRA,
    #[doc = "0x808 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb32: ADDOPCRB,
    #[doc = "0x80c - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc32: ADDOPCRC,
    #[doc = "0x810 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr33: ADCHCR,
    #[doc = "0x814 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra33: ADDOPCRA,
    #[doc = "0x818 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb33: ADDOPCRB,
    #[doc = "0x81c - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc33: ADDOPCRC,
    #[doc = "0x820 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr34: ADCHCR,
    #[doc = "0x824 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra34: ADDOPCRA,
    #[doc = "0x828 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb34: ADDOPCRB,
    #[doc = "0x82c - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc34: ADDOPCRC,
    #[doc = "0x830 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr35: ADCHCR,
    #[doc = "0x834 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra35: ADDOPCRA,
    #[doc = "0x838 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb35: ADDOPCRB,
    #[doc = "0x83c - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc35: ADDOPCRC,
    #[doc = "0x840 - A/D Conversion Channel Configuration Register %s"]
    pub adchcr36: ADCHCR,
    #[doc = "0x844 - A/D Conversion Data Operation Control A Register %s"]
    pub addopcra36: ADDOPCRA,
    #[doc = "0x848 - A/D Conversion Data Operation Control B Register %s"]
    pub addopcrb36: ADDOPCRB,
    #[doc = "0x84c - A/D Conversion Data Operation Control C Register %s"]
    pub addopcrc36: ADDOPCRC,
    _reserved231: [u8; 0x03b0],
    #[doc = "0xc00 - A/D Converter Self-calibration Start Register"]
    pub adcalstr: ADCALSTR,
    _reserved232: [u8; 0x04],
    #[doc = "0xc08 - A/D Conversion Start Trigger Enable Register"]
    pub adtrgenr: ADTRGENR,
    _reserved233: [u8; 0x04],
    #[doc = "0xc10 - A/D Conversion Synchronous Software Start Register"]
    pub adsystr: ADSYSTR,
    _reserved234: [u8; 0x0c],
    #[doc = "0xc20..0xc44 - A/D Conversion Software Start Register %s"]
    pub adstr: [ADSTR; 9],
    _reserved235: [u8; 0x1c],
    #[doc = "0xc60 - A/D Conversion Stop Register"]
    pub adstopr: ADSTOPR,
    _reserved236: [u8; 0x1c],
    #[doc = "0xc80 - A/D Conversion Status Register"]
    pub adsr: ADSR,
    #[doc = "0xc84 - Scan Group Status Register"]
    pub adgrsr: ADGRSR,
    #[doc = "0xc88 - A/D Conversion Error Status Register"]
    pub adersr: ADERSR,
    #[doc = "0xc8c - A/D Conversion Error Status Clear Register"]
    pub aderscr: ADERSCR,
    _reserved240: [u8; 0x08],
    #[doc = "0xc98 - A/D Converter Calibration End Status Register"]
    pub adcalendsr: ADCALENDSR,
    #[doc = "0xc9c - A/D Converter Calibration End Status Clear Register"]
    pub adcalendscr: ADCALENDSCR,
    #[doc = "0xca0 - A/D Conversion Overflow Error Status Register"]
    pub adovfersr: ADOVFERSR,
    #[doc = "0xca4 - A/D Conversion Overflow Channel Status Register 0"]
    pub adovfchsr0: ADOVFCHSR0,
    _reserved244: [u8; 0x08],
    #[doc = "0xcb0 - Extended Analog A/D Conversion Overflow Status Register"]
    pub adovfexsr: ADOVFEXSR,
    #[doc = "0xcb4 - A/D Conversion Overflow Error Status Clear Register"]
    pub adovferscr: ADOVFERSCR,
    #[doc = "0xcb8 - A/D Conversion Overflow Channel Status Clear Register 0"]
    pub adovfchscr0: ADOVFCHSCR0,
    _reserved247: [u8; 0x08],
    #[doc = "0xcc4 - Extended Analog A/D Conversion Overflow Status Clear Register"]
    pub adovfexscr: ADOVFEXSCR,
    _reserved248: [u8; 0x08],
    #[doc = "0xcd0 - FIFO Status Register 0"]
    pub adfifosr0: ADFIFOSR0,
    #[doc = "0xcd4 - FIFO Status Register 1"]
    pub adfifosr1: ADFIFOSR1,
    #[doc = "0xcd8 - FIFO Status Register 2"]
    pub adfifosr2: ADFIFOSR2,
    #[doc = "0xcdc - FIFO Status Register 3"]
    pub adfifosr3: ADFIFOSR3,
    #[doc = "0xce0 - FIFO Status Register 4"]
    pub adfifosr4: ADFIFOSR4,
    _reserved253: [u8; 0x0c],
    #[doc = "0xcf0 - FIFO Data Clear Register"]
    pub adfifodcr: ADFIFODCR,
    #[doc = "0xcf4 - FIFO Error Status Register"]
    pub adfifoersr: ADFIFOERSR,
    #[doc = "0xcf8 - FIFO Error Status Clear Register"]
    pub adfifoerscr: ADFIFOERSCR,
    _reserved256: [u8; 0x04],
    #[doc = "0xd00 - Compare Match Table Status Register"]
    pub adcmptbsr: ADCMPTBSR,
    #[doc = "0xd04 - Compare Match Table Status Clear Register"]
    pub adcmptbscr: ADCMPTBSCR,
    #[doc = "0xd08 - Compare Match Channel Status Register 0"]
    pub adcmpchsr0: ADCMPCHSR0,
    _reserved259: [u8; 0x08],
    #[doc = "0xd14 - Extended Analog Compare Match Status Register"]
    pub adcmpexsr: ADCMPEXSR,
    #[doc = "0xd18 - Compare Match Channel Status Clear Register 0"]
    pub adcmpchscr0: ADCMPCHSCR0,
    _reserved261: [u8; 0x08],
    #[doc = "0xd24 - Extended Analog Compare Match Status Clear Register"]
    pub adcmpexscr: ADCMPEXSCR,
    #[doc = "0xd28 - Limiter Clip Scan Group Status Register"]
    pub adlimgrsr: ADLIMGRSR,
    #[doc = "0xd2c - Limiter Clip Channel Status Register 0"]
    pub adlimchsr0: ADLIMCHSR0,
    _reserved264: [u8; 0x08],
    #[doc = "0xd38 - Extended Analog Limiter Clip Status Register"]
    pub adlimexsr: ADLIMEXSR,
    #[doc = "0xd3c - Limiter Clip Scan Group Status Clear Register"]
    pub adlimgrscr: ADLIMGRSCR,
    #[doc = "0xd40 - Limiter Clip Channel Status Clear Register 0"]
    pub adlimchscr0: ADLIMCHSCR0,
    _reserved267: [u8; 0x08],
    #[doc = "0xd4c - Extended Analog Limiter Clip Status Clear Register"]
    pub adlimexscr: ADLIMEXSCR,
    #[doc = "0xd50 - Scan End Status Register"]
    pub adscanendsr: ADSCANENDSR,
    #[doc = "0xd54 - Scan End Status Clear Register"]
    pub adscanendscr: ADSCANENDSCR,
    _reserved270: [u8; 0x02a8],
    #[doc = "0x1000..0x1074 - A/D Data Register %s"]
    pub addr: [ADDR; 29],
    _reserved271: [u8; 0x010c],
    #[doc = "0x1180..0x118c - A/D Extended Analog Data Register %s"]
    pub adexdr: [ADEXDR; 3],
    _reserved272: [u8; 0x08],
    #[doc = "0x1194 - A/D Extended Analog Data Register %s"]
    pub adexdr5: ADEXDR5,
    #[doc = "0x1198 - A/D Extended Analog Data Register %s"]
    pub adexdr6: ADEXDR5,
    #[doc = "0x119c - A/D Extended Analog Data Register %s"]
    pub adexdr7: ADEXDR5,
    #[doc = "0x11a0 - A/D Extended Analog Data Register %s"]
    pub adexdr8: ADEXDR5,
    _reserved276: [u8; 0x5c],
    #[doc = "0x1200..0x1224 - FIFO Data Register %s"]
    pub adfifodr: [ADFIFODR; 9],
}
#[doc = "ADCLKENR (rw) register accessor: an alias for `Reg<ADCLKENR_SPEC>`"]
pub type ADCLKENR = crate::Reg<adclkenr::ADCLKENR_SPEC>;
#[doc = "A/D Conversion Clock Enable Register"]
pub mod adclkenr;
#[doc = "ADCLKSR (r) register accessor: an alias for `Reg<ADCLKSR_SPEC>`"]
pub type ADCLKSR = crate::Reg<adclksr::ADCLKSR_SPEC>;
#[doc = "A/D Conversion Clock Status Register"]
pub mod adclksr;
#[doc = "ADCLKCR (rw) register accessor: an alias for `Reg<ADCLKCR_SPEC>`"]
pub type ADCLKCR = crate::Reg<adclkcr::ADCLKCR_SPEC>;
#[doc = "A/D Conversion Clock Control Register"]
pub mod adclkcr;
#[doc = "ADSYCR (rw) register accessor: an alias for `Reg<ADSYCR_SPEC>`"]
pub type ADSYCR = crate::Reg<adsycr::ADSYCR_SPEC>;
#[doc = "A/D Converter Synchronous Operation Control Register"]
pub mod adsycr;
#[doc = "ADERINTCR (rw) register accessor: an alias for `Reg<ADERINTCR_SPEC>`"]
pub type ADERINTCR = crate::Reg<aderintcr::ADERINTCR_SPEC>;
#[doc = "A/D Conversion Error Interrupt Enable Register"]
pub mod aderintcr;
#[doc = "ADOVFINTCR (rw) register accessor: an alias for `Reg<ADOVFINTCR_SPEC>`"]
pub type ADOVFINTCR = crate::Reg<adovfintcr::ADOVFINTCR_SPEC>;
#[doc = "A/D Conversion Overflow Interrupt Enable Register"]
pub mod adovfintcr;
#[doc = "ADCALINTCR (rw) register accessor: an alias for `Reg<ADCALINTCR_SPEC>`"]
pub type ADCALINTCR = crate::Reg<adcalintcr::ADCALINTCR_SPEC>;
#[doc = "Calibration interrupt Enable Register"]
pub mod adcalintcr;
#[doc = "ADMDR (rw) register accessor: an alias for `Reg<ADMDR_SPEC>`"]
pub type ADMDR = crate::Reg<admdr::ADMDR_SPEC>;
#[doc = "A/D Converter Mode Selection Register"]
pub mod admdr;
#[doc = "ADGSPCR (rw) register accessor: an alias for `Reg<ADGSPCR_SPEC>`"]
pub type ADGSPCR = crate::Reg<adgspcr::ADGSPCR_SPEC>;
#[doc = "A/D Group scan Priority Control Register"]
pub mod adgspcr;
#[doc = "ADSGER (rw) register accessor: an alias for `Reg<ADSGER_SPEC>`"]
pub type ADSGER = crate::Reg<adsger::ADSGER_SPEC>;
#[doc = "Scan Group Enable Register"]
pub mod adsger;
#[doc = "ADSGCR0 (rw) register accessor: an alias for `Reg<ADSGCR0_SPEC>`"]
pub type ADSGCR0 = crate::Reg<adsgcr0::ADSGCR0_SPEC>;
#[doc = "Scan Group Control Register 0"]
pub mod adsgcr0;
#[doc = "ADSGCR1 (rw) register accessor: an alias for `Reg<ADSGCR1_SPEC>`"]
pub type ADSGCR1 = crate::Reg<adsgcr1::ADSGCR1_SPEC>;
#[doc = "Scan Group Control Register 1"]
pub mod adsgcr1;
#[doc = "ADSGCR2 (rw) register accessor: an alias for `Reg<ADSGCR2_SPEC>`"]
pub type ADSGCR2 = crate::Reg<adsgcr2::ADSGCR2_SPEC>;
#[doc = "Scan Group Control Register 2"]
pub mod adsgcr2;
#[doc = "ADINTCR (rw) register accessor: an alias for `Reg<ADINTCR_SPEC>`"]
pub type ADINTCR = crate::Reg<adintcr::ADINTCR_SPEC>;
#[doc = "Scan End Interrupt Enable Register"]
pub mod adintcr;
#[doc = "ADTRGEXT (rw) register accessor: an alias for `Reg<ADTRGEXT_SPEC>`"]
pub type ADTRGEXT = crate::Reg<adtrgext::ADTRGEXT_SPEC>;
#[doc = "External Trigger Enable Register %s"]
pub mod adtrgext;
#[doc = "ADTRGELC (rw) register accessor: an alias for `Reg<ADTRGELC_SPEC>`"]
pub type ADTRGELC = crate::Reg<adtrgelc::ADTRGELC_SPEC>;
#[doc = "ELC Trigger Enable Register %s"]
pub mod adtrgelc;
#[doc = "ADTRGGPT (rw) register accessor: an alias for `Reg<ADTRGGPT_SPEC>`"]
pub type ADTRGGPT = crate::Reg<adtrggpt::ADTRGGPT_SPEC>;
#[doc = "GPT Trigger Enable Register %s"]
pub mod adtrggpt;
#[doc = "ADTRGDLR0 (rw) register accessor: an alias for `Reg<ADTRGDLR0_SPEC>`"]
pub type ADTRGDLR0 = crate::Reg<adtrgdlr0::ADTRGDLR0_SPEC>;
#[doc = "A/D Conversion Start Trigger Delay Register 0"]
pub mod adtrgdlr0;
#[doc = "ADTRGDLR1 (rw) register accessor: an alias for `Reg<ADTRGDLR1_SPEC>`"]
pub type ADTRGDLR1 = crate::Reg<adtrgdlr1::ADTRGDLR1_SPEC>;
#[doc = "A/D Conversion Start Trigger Delay Register 1"]
pub mod adtrgdlr1;
#[doc = "ADTRGDLR2 (rw) register accessor: an alias for `Reg<ADTRGDLR2_SPEC>`"]
pub type ADTRGDLR2 = crate::Reg<adtrgdlr2::ADTRGDLR2_SPEC>;
#[doc = "A/D Conversion Start Trigger Delay Register 2"]
pub mod adtrgdlr2;
#[doc = "ADTRGDLR3 (rw) register accessor: an alias for `Reg<ADTRGDLR3_SPEC>`"]
pub type ADTRGDLR3 = crate::Reg<adtrgdlr3::ADTRGDLR3_SPEC>;
#[doc = "A/D Conversion Start Trigger Delay Register 3"]
pub mod adtrgdlr3;
#[doc = "ADTRGDLR4 (rw) register accessor: an alias for `Reg<ADTRGDLR4_SPEC>`"]
pub type ADTRGDLR4 = crate::Reg<adtrgdlr4::ADTRGDLR4_SPEC>;
#[doc = "A/D Conversion Start Trigger Delay Register 4"]
pub mod adtrgdlr4;
#[doc = "ADSGDCR (rw) register accessor: an alias for `Reg<ADSGDCR_SPEC>`"]
pub type ADSGDCR = crate::Reg<adsgdcr::ADSGDCR_SPEC>;
#[doc = "Scan Group Diagnosis Function Control Register %s"]
pub mod adsgdcr;
#[doc = "ADSSTR0 (rw) register accessor: an alias for `Reg<ADSSTR0_SPEC>`"]
pub type ADSSTR0 = crate::Reg<adsstr0::ADSSTR0_SPEC>;
#[doc = "Sampling State Table Register 0"]
pub mod adsstr0;
#[doc = "ADSSTR1 (rw) register accessor: an alias for `Reg<ADSSTR1_SPEC>`"]
pub type ADSSTR1 = crate::Reg<adsstr1::ADSSTR1_SPEC>;
#[doc = "Sampling State Table Register 1"]
pub mod adsstr1;
#[doc = "ADSSTR2 (rw) register accessor: an alias for `Reg<ADSSTR2_SPEC>`"]
pub type ADSSTR2 = crate::Reg<adsstr2::ADSSTR2_SPEC>;
#[doc = "Sampling State Table Register 2"]
pub mod adsstr2;
#[doc = "ADSSTR3 (rw) register accessor: an alias for `Reg<ADSSTR3_SPEC>`"]
pub type ADSSTR3 = crate::Reg<adsstr3::ADSSTR3_SPEC>;
#[doc = "Sampling State Table Register 3"]
pub mod adsstr3;
#[doc = "ADSSTR4 (rw) register accessor: an alias for `Reg<ADSSTR4_SPEC>`"]
pub type ADSSTR4 = crate::Reg<adsstr4::ADSSTR4_SPEC>;
#[doc = "Sampling State Table Register 4"]
pub mod adsstr4;
#[doc = "ADSSTR5 (rw) register accessor: an alias for `Reg<ADSSTR5_SPEC>`"]
pub type ADSSTR5 = crate::Reg<adsstr5::ADSSTR5_SPEC>;
#[doc = "Sampling State Table Register 5"]
pub mod adsstr5;
#[doc = "ADSSTR6 (rw) register accessor: an alias for `Reg<ADSSTR6_SPEC>`"]
pub type ADSSTR6 = crate::Reg<adsstr6::ADSSTR6_SPEC>;
#[doc = "Sampling State Table Register 6"]
pub mod adsstr6;
#[doc = "ADSSTR7 (rw) register accessor: an alias for `Reg<ADSSTR7_SPEC>`"]
pub type ADSSTR7 = crate::Reg<adsstr7::ADSSTR7_SPEC>;
#[doc = "Sampling State Table Register 7"]
pub mod adsstr7;
#[doc = "ADCNVSTR (rw) register accessor: an alias for `Reg<ADCNVSTR_SPEC>`"]
pub type ADCNVSTR = crate::Reg<adcnvstr::ADCNVSTR_SPEC>;
#[doc = "A/D Conversion State Register"]
pub mod adcnvstr;
#[doc = "ADCALSTCR (rw) register accessor: an alias for `Reg<ADCALSTCR_SPEC>`"]
pub type ADCALSTCR = crate::Reg<adcalstcr::ADCALSTCR_SPEC>;
#[doc = "A/D Converter Self-calibration State Register"]
pub mod adcalstcr;
#[doc = "ADSHCR0 (rw) register accessor: an alias for `Reg<ADSHCR0_SPEC>`"]
pub type ADSHCR0 = crate::Reg<adshcr0::ADSHCR0_SPEC>;
#[doc = "Channel-dedicated Sample-and-hold Circuit Control Register 0"]
pub mod adshcr0;
#[doc = "ADSHSTR0 (rw) register accessor: an alias for `Reg<ADSHSTR0_SPEC>`"]
pub type ADSHSTR0 = crate::Reg<adshstr0::ADSHSTR0_SPEC>;
#[doc = "Channel-dedicated Sample-and-hold Circuit State Register 0"]
pub mod adshstr0;
#[doc = "ADSHCR1 (rw) register accessor: an alias for `Reg<ADSHCR1_SPEC>`"]
pub type ADSHCR1 = crate::Reg<adshcr1::ADSHCR1_SPEC>;
#[doc = "Channel-dedicated Sample-and-hold Circuit Control Register 1"]
pub mod adshcr1;
#[doc = "ADSHSTR1 (rw) register accessor: an alias for `Reg<ADSHSTR1_SPEC>`"]
pub type ADSHSTR1 = crate::Reg<adshstr1::ADSHSTR1_SPEC>;
#[doc = "Channel-dedicated Sample-and-hold Circuit State Register 1"]
pub mod adshstr1;
#[doc = "ADCALSHCR (rw) register accessor: an alias for `Reg<ADCALSHCR_SPEC>`"]
pub type ADCALSHCR = crate::Reg<adcalshcr::ADCALSHCR_SPEC>;
#[doc = "Channel-dedicated Sample-and-hold Circuit Self-calibration State Register"]
pub mod adcalshcr;
#[doc = "ADPGACR (rw) register accessor: an alias for `Reg<ADPGACR_SPEC>`"]
pub type ADPGACR = crate::Reg<adpgacr::ADPGACR_SPEC>;
#[doc = "Programmable Gain Amplifier Control Register %s"]
pub mod adpgacr;
#[doc = "ADPGAMONCR (rw) register accessor: an alias for `Reg<ADPGAMONCR_SPEC>`"]
pub type ADPGAMONCR = crate::Reg<adpgamoncr::ADPGAMONCR_SPEC>;
#[doc = "Programable Gain Amp Monitor Output Control Register"]
pub mod adpgamoncr;
#[doc = "ADREFCR (rw) register accessor: an alias for `Reg<ADREFCR_SPEC>`"]
pub type ADREFCR = crate::Reg<adrefcr::ADREFCR_SPEC>;
#[doc = "Internal Reference Voltage Monitor Enable Register"]
pub mod adrefcr;
#[doc = "ADDFSR (rw) register accessor: an alias for `Reg<ADDFSR_SPEC>`"]
pub type ADDFSR = crate::Reg<addfsr::ADDFSR_SPEC>;
#[doc = "A/D Converter Digital Filter Selection Register %s"]
pub mod addfsr;
#[doc = "ADUOFTR (rw) register accessor: an alias for `Reg<ADUOFTR_SPEC>`"]
pub type ADUOFTR = crate::Reg<aduoftr::ADUOFTR_SPEC>;
#[doc = "User Offset Table Register %s"]
pub mod aduoftr;
#[doc = "ADUGTR (rw) register accessor: an alias for `Reg<ADUGTR_SPEC>`"]
pub type ADUGTR = crate::Reg<adugtr::ADUGTR_SPEC>;
#[doc = "User Gain Table Register %s"]
pub mod adugtr;
#[doc = "ADLIMINTCR (rw) register accessor: an alias for `Reg<ADLIMINTCR_SPEC>`"]
pub type ADLIMINTCR = crate::Reg<adlimintcr::ADLIMINTCR_SPEC>;
#[doc = "Limiter Clip Interrupt Enable Register"]
pub mod adlimintcr;
#[doc = "ADLIMTR (rw) register accessor: an alias for `Reg<ADLIMTR_SPEC>`"]
pub type ADLIMTR = crate::Reg<adlimtr::ADLIMTR_SPEC>;
#[doc = "Limiter Clip Table Register %s"]
pub mod adlimtr;
#[doc = "ADCMPENR (rw) register accessor: an alias for `Reg<ADCMPENR_SPEC>`"]
pub type ADCMPENR = crate::Reg<adcmpenr::ADCMPENR_SPEC>;
#[doc = "Compare Match Enable Register"]
pub mod adcmpenr;
#[doc = "ADCMPINTCR (rw) register accessor: an alias for `Reg<ADCMPINTCR_SPEC>`"]
pub type ADCMPINTCR = crate::Reg<adcmpintcr::ADCMPINTCR_SPEC>;
#[doc = "Compare Match Interrupt Enable Register"]
pub mod adcmpintcr;
#[doc = "ADCCMPCR (rw) register accessor: an alias for `Reg<ADCCMPCR_SPEC>`"]
pub type ADCCMPCR = crate::Reg<adccmpcr::ADCCMPCR_SPEC>;
#[doc = "Composite Compare Match Configuration Register %s"]
pub mod adccmpcr;
#[doc = "ADCMPMDR0 (rw) register accessor: an alias for `Reg<ADCMPMDR0_SPEC>`"]
pub type ADCMPMDR0 = crate::Reg<adcmpmdr0::ADCMPMDR0_SPEC>;
#[doc = "Compare Match Mode Selection Register 0"]
pub mod adcmpmdr0;
#[doc = "ADCMPMDR1 (rw) register accessor: an alias for `Reg<ADCMPMDR1_SPEC>`"]
pub type ADCMPMDR1 = crate::Reg<adcmpmdr1::ADCMPMDR1_SPEC>;
#[doc = "Compare Match Mode Selection Register 1"]
pub mod adcmpmdr1;
#[doc = "ADCMPTBR (rw) register accessor: an alias for `Reg<ADCMPTBR_SPEC>`"]
pub type ADCMPTBR = crate::Reg<adcmptbr::ADCMPTBR_SPEC>;
#[doc = "Compare Match Table Register %s"]
pub mod adcmptbr;
#[doc = "ADFIFOCR (rw) register accessor: an alias for `Reg<ADFIFOCR_SPEC>`"]
pub type ADFIFOCR = crate::Reg<adfifocr::ADFIFOCR_SPEC>;
#[doc = "FIFO Control Register"]
pub mod adfifocr;
#[doc = "ADFIFOINTCR (rw) register accessor: an alias for `Reg<ADFIFOINTCR_SPEC>`"]
pub type ADFIFOINTCR = crate::Reg<adfifointcr::ADFIFOINTCR_SPEC>;
#[doc = "FIFO Interrupt Control Register"]
pub mod adfifointcr;
#[doc = "ADFIFOINTLR0 (rw) register accessor: an alias for `Reg<ADFIFOINTLR0_SPEC>`"]
pub type ADFIFOINTLR0 = crate::Reg<adfifointlr0::ADFIFOINTLR0_SPEC>;
#[doc = "FIFO Interrupt Generation Level Register 0"]
pub mod adfifointlr0;
#[doc = "ADFIFOINTLR1 (rw) register accessor: an alias for `Reg<ADFIFOINTLR1_SPEC>`"]
pub type ADFIFOINTLR1 = crate::Reg<adfifointlr1::ADFIFOINTLR1_SPEC>;
#[doc = "FIFO Interrupt Generation Level Register 1"]
pub mod adfifointlr1;
#[doc = "ADFIFOINTLR2 (rw) register accessor: an alias for `Reg<ADFIFOINTLR2_SPEC>`"]
pub type ADFIFOINTLR2 = crate::Reg<adfifointlr2::ADFIFOINTLR2_SPEC>;
#[doc = "FIFO Interrupt Generation Level Register 2"]
pub mod adfifointlr2;
#[doc = "ADFIFOINTLR3 (rw) register accessor: an alias for `Reg<ADFIFOINTLR3_SPEC>`"]
pub type ADFIFOINTLR3 = crate::Reg<adfifointlr3::ADFIFOINTLR3_SPEC>;
#[doc = "FIFO Interrupt Generation Level Register 3"]
pub mod adfifointlr3;
#[doc = "ADFIFOINTLR4 (rw) register accessor: an alias for `Reg<ADFIFOINTLR4_SPEC>`"]
pub type ADFIFOINTLR4 = crate::Reg<adfifointlr4::ADFIFOINTLR4_SPEC>;
#[doc = "FIFO Interrupt Generation Level Register 4"]
pub mod adfifointlr4;
#[doc = "ADCHCR (rw) register accessor: an alias for `Reg<ADCHCR_SPEC>`"]
pub type ADCHCR = crate::Reg<adchcr::ADCHCR_SPEC>;
#[doc = "A/D Conversion Channel Configuration Register %s"]
pub mod adchcr;
#[doc = "ADDOPCRA (rw) register accessor: an alias for `Reg<ADDOPCRA_SPEC>`"]
pub type ADDOPCRA = crate::Reg<addopcra::ADDOPCRA_SPEC>;
#[doc = "A/D Conversion Data Operation Control A Register %s"]
pub mod addopcra;
#[doc = "ADDOPCRB (rw) register accessor: an alias for `Reg<ADDOPCRB_SPEC>`"]
pub type ADDOPCRB = crate::Reg<addopcrb::ADDOPCRB_SPEC>;
#[doc = "A/D Conversion Data Operation Control B Register %s"]
pub mod addopcrb;
#[doc = "ADDOPCRC (rw) register accessor: an alias for `Reg<ADDOPCRC_SPEC>`"]
pub type ADDOPCRC = crate::Reg<addopcrc::ADDOPCRC_SPEC>;
#[doc = "A/D Conversion Data Operation Control C Register %s"]
pub mod addopcrc;
#[doc = "ADCALSTR (w) register accessor: an alias for `Reg<ADCALSTR_SPEC>`"]
pub type ADCALSTR = crate::Reg<adcalstr::ADCALSTR_SPEC>;
#[doc = "A/D Converter Self-calibration Start Register"]
pub mod adcalstr;
#[doc = "ADTRGENR (rw) register accessor: an alias for `Reg<ADTRGENR_SPEC>`"]
pub type ADTRGENR = crate::Reg<adtrgenr::ADTRGENR_SPEC>;
#[doc = "A/D Conversion Start Trigger Enable Register"]
pub mod adtrgenr;
#[doc = "ADSYSTR (w) register accessor: an alias for `Reg<ADSYSTR_SPEC>`"]
pub type ADSYSTR = crate::Reg<adsystr::ADSYSTR_SPEC>;
#[doc = "A/D Conversion Synchronous Software Start Register"]
pub mod adsystr;
#[doc = "ADSTR (w) register accessor: an alias for `Reg<ADSTR_SPEC>`"]
pub type ADSTR = crate::Reg<adstr::ADSTR_SPEC>;
#[doc = "A/D Conversion Software Start Register %s"]
pub mod adstr;
#[doc = "ADSTOPR (w) register accessor: an alias for `Reg<ADSTOPR_SPEC>`"]
pub type ADSTOPR = crate::Reg<adstopr::ADSTOPR_SPEC>;
#[doc = "A/D Conversion Stop Register"]
pub mod adstopr;
#[doc = "ADSR (r) register accessor: an alias for `Reg<ADSR_SPEC>`"]
pub type ADSR = crate::Reg<adsr::ADSR_SPEC>;
#[doc = "A/D Conversion Status Register"]
pub mod adsr;
#[doc = "ADGRSR (r) register accessor: an alias for `Reg<ADGRSR_SPEC>`"]
pub type ADGRSR = crate::Reg<adgrsr::ADGRSR_SPEC>;
#[doc = "Scan Group Status Register"]
pub mod adgrsr;
#[doc = "ADERSR (r) register accessor: an alias for `Reg<ADERSR_SPEC>`"]
pub type ADERSR = crate::Reg<adersr::ADERSR_SPEC>;
#[doc = "A/D Conversion Error Status Register"]
pub mod adersr;
#[doc = "ADERSCR (w) register accessor: an alias for `Reg<ADERSCR_SPEC>`"]
pub type ADERSCR = crate::Reg<aderscr::ADERSCR_SPEC>;
#[doc = "A/D Conversion Error Status Clear Register"]
pub mod aderscr;
#[doc = "ADCALENDSR (r) register accessor: an alias for `Reg<ADCALENDSR_SPEC>`"]
pub type ADCALENDSR = crate::Reg<adcalendsr::ADCALENDSR_SPEC>;
#[doc = "A/D Converter Calibration End Status Register"]
pub mod adcalendsr;
#[doc = "ADCALENDSCR (w) register accessor: an alias for `Reg<ADCALENDSCR_SPEC>`"]
pub type ADCALENDSCR = crate::Reg<adcalendscr::ADCALENDSCR_SPEC>;
#[doc = "A/D Converter Calibration End Status Clear Register"]
pub mod adcalendscr;
#[doc = "ADOVFERSR (r) register accessor: an alias for `Reg<ADOVFERSR_SPEC>`"]
pub type ADOVFERSR = crate::Reg<adovfersr::ADOVFERSR_SPEC>;
#[doc = "A/D Conversion Overflow Error Status Register"]
pub mod adovfersr;
#[doc = "ADOVFCHSR0 (r) register accessor: an alias for `Reg<ADOVFCHSR0_SPEC>`"]
pub type ADOVFCHSR0 = crate::Reg<adovfchsr0::ADOVFCHSR0_SPEC>;
#[doc = "A/D Conversion Overflow Channel Status Register 0"]
pub mod adovfchsr0;
#[doc = "ADOVFEXSR (r) register accessor: an alias for `Reg<ADOVFEXSR_SPEC>`"]
pub type ADOVFEXSR = crate::Reg<adovfexsr::ADOVFEXSR_SPEC>;
#[doc = "Extended Analog A/D Conversion Overflow Status Register"]
pub mod adovfexsr;
#[doc = "ADOVFERSCR (w) register accessor: an alias for `Reg<ADOVFERSCR_SPEC>`"]
pub type ADOVFERSCR = crate::Reg<adovferscr::ADOVFERSCR_SPEC>;
#[doc = "A/D Conversion Overflow Error Status Clear Register"]
pub mod adovferscr;
#[doc = "ADOVFCHSCR0 (w) register accessor: an alias for `Reg<ADOVFCHSCR0_SPEC>`"]
pub type ADOVFCHSCR0 = crate::Reg<adovfchscr0::ADOVFCHSCR0_SPEC>;
#[doc = "A/D Conversion Overflow Channel Status Clear Register 0"]
pub mod adovfchscr0;
#[doc = "ADOVFEXSCR (w) register accessor: an alias for `Reg<ADOVFEXSCR_SPEC>`"]
pub type ADOVFEXSCR = crate::Reg<adovfexscr::ADOVFEXSCR_SPEC>;
#[doc = "Extended Analog A/D Conversion Overflow Status Clear Register"]
pub mod adovfexscr;
#[doc = "ADFIFOSR0 (r) register accessor: an alias for `Reg<ADFIFOSR0_SPEC>`"]
pub type ADFIFOSR0 = crate::Reg<adfifosr0::ADFIFOSR0_SPEC>;
#[doc = "FIFO Status Register 0"]
pub mod adfifosr0;
#[doc = "ADFIFOSR1 (r) register accessor: an alias for `Reg<ADFIFOSR1_SPEC>`"]
pub type ADFIFOSR1 = crate::Reg<adfifosr1::ADFIFOSR1_SPEC>;
#[doc = "FIFO Status Register 1"]
pub mod adfifosr1;
#[doc = "ADFIFOSR2 (r) register accessor: an alias for `Reg<ADFIFOSR2_SPEC>`"]
pub type ADFIFOSR2 = crate::Reg<adfifosr2::ADFIFOSR2_SPEC>;
#[doc = "FIFO Status Register 2"]
pub mod adfifosr2;
#[doc = "ADFIFOSR3 (r) register accessor: an alias for `Reg<ADFIFOSR3_SPEC>`"]
pub type ADFIFOSR3 = crate::Reg<adfifosr3::ADFIFOSR3_SPEC>;
#[doc = "FIFO Status Register 3"]
pub mod adfifosr3;
#[doc = "ADFIFOSR4 (r) register accessor: an alias for `Reg<ADFIFOSR4_SPEC>`"]
pub type ADFIFOSR4 = crate::Reg<adfifosr4::ADFIFOSR4_SPEC>;
#[doc = "FIFO Status Register 4"]
pub mod adfifosr4;
#[doc = "ADFIFODCR (w) register accessor: an alias for `Reg<ADFIFODCR_SPEC>`"]
pub type ADFIFODCR = crate::Reg<adfifodcr::ADFIFODCR_SPEC>;
#[doc = "FIFO Data Clear Register"]
pub mod adfifodcr;
#[doc = "ADFIFOERSR (r) register accessor: an alias for `Reg<ADFIFOERSR_SPEC>`"]
pub type ADFIFOERSR = crate::Reg<adfifoersr::ADFIFOERSR_SPEC>;
#[doc = "FIFO Error Status Register"]
pub mod adfifoersr;
#[doc = "ADFIFOERSCR (w) register accessor: an alias for `Reg<ADFIFOERSCR_SPEC>`"]
pub type ADFIFOERSCR = crate::Reg<adfifoerscr::ADFIFOERSCR_SPEC>;
#[doc = "FIFO Error Status Clear Register"]
pub mod adfifoerscr;
#[doc = "ADCMPTBSR (r) register accessor: an alias for `Reg<ADCMPTBSR_SPEC>`"]
pub type ADCMPTBSR = crate::Reg<adcmptbsr::ADCMPTBSR_SPEC>;
#[doc = "Compare Match Table Status Register"]
pub mod adcmptbsr;
#[doc = "ADCMPTBSCR (w) register accessor: an alias for `Reg<ADCMPTBSCR_SPEC>`"]
pub type ADCMPTBSCR = crate::Reg<adcmptbscr::ADCMPTBSCR_SPEC>;
#[doc = "Compare Match Table Status Clear Register"]
pub mod adcmptbscr;
#[doc = "ADCMPCHSR0 (r) register accessor: an alias for `Reg<ADCMPCHSR0_SPEC>`"]
pub type ADCMPCHSR0 = crate::Reg<adcmpchsr0::ADCMPCHSR0_SPEC>;
#[doc = "Compare Match Channel Status Register 0"]
pub mod adcmpchsr0;
#[doc = "ADCMPEXSR (r) register accessor: an alias for `Reg<ADCMPEXSR_SPEC>`"]
pub type ADCMPEXSR = crate::Reg<adcmpexsr::ADCMPEXSR_SPEC>;
#[doc = "Extended Analog Compare Match Status Register"]
pub mod adcmpexsr;
#[doc = "ADCMPCHSCR0 (w) register accessor: an alias for `Reg<ADCMPCHSCR0_SPEC>`"]
pub type ADCMPCHSCR0 = crate::Reg<adcmpchscr0::ADCMPCHSCR0_SPEC>;
#[doc = "Compare Match Channel Status Clear Register 0"]
pub mod adcmpchscr0;
#[doc = "ADCMPEXSCR (w) register accessor: an alias for `Reg<ADCMPEXSCR_SPEC>`"]
pub type ADCMPEXSCR = crate::Reg<adcmpexscr::ADCMPEXSCR_SPEC>;
#[doc = "Extended Analog Compare Match Status Clear Register"]
pub mod adcmpexscr;
#[doc = "ADLIMGRSR (r) register accessor: an alias for `Reg<ADLIMGRSR_SPEC>`"]
pub type ADLIMGRSR = crate::Reg<adlimgrsr::ADLIMGRSR_SPEC>;
#[doc = "Limiter Clip Scan Group Status Register"]
pub mod adlimgrsr;
#[doc = "ADLIMCHSR0 (r) register accessor: an alias for `Reg<ADLIMCHSR0_SPEC>`"]
pub type ADLIMCHSR0 = crate::Reg<adlimchsr0::ADLIMCHSR0_SPEC>;
#[doc = "Limiter Clip Channel Status Register 0"]
pub mod adlimchsr0;
#[doc = "ADLIMEXSR (r) register accessor: an alias for `Reg<ADLIMEXSR_SPEC>`"]
pub type ADLIMEXSR = crate::Reg<adlimexsr::ADLIMEXSR_SPEC>;
#[doc = "Extended Analog Limiter Clip Status Register"]
pub mod adlimexsr;
#[doc = "ADLIMGRSCR (w) register accessor: an alias for `Reg<ADLIMGRSCR_SPEC>`"]
pub type ADLIMGRSCR = crate::Reg<adlimgrscr::ADLIMGRSCR_SPEC>;
#[doc = "Limiter Clip Scan Group Status Clear Register"]
pub mod adlimgrscr;
#[doc = "ADLIMCHSCR0 (w) register accessor: an alias for `Reg<ADLIMCHSCR0_SPEC>`"]
pub type ADLIMCHSCR0 = crate::Reg<adlimchscr0::ADLIMCHSCR0_SPEC>;
#[doc = "Limiter Clip Channel Status Clear Register 0"]
pub mod adlimchscr0;
#[doc = "ADLIMEXSCR (w) register accessor: an alias for `Reg<ADLIMEXSCR_SPEC>`"]
pub type ADLIMEXSCR = crate::Reg<adlimexscr::ADLIMEXSCR_SPEC>;
#[doc = "Extended Analog Limiter Clip Status Clear Register"]
pub mod adlimexscr;
#[doc = "ADSCANENDSR (r) register accessor: an alias for `Reg<ADSCANENDSR_SPEC>`"]
pub type ADSCANENDSR = crate::Reg<adscanendsr::ADSCANENDSR_SPEC>;
#[doc = "Scan End Status Register"]
pub mod adscanendsr;
#[doc = "ADSCANENDSCR (w) register accessor: an alias for `Reg<ADSCANENDSCR_SPEC>`"]
pub type ADSCANENDSCR = crate::Reg<adscanendscr::ADSCANENDSCR_SPEC>;
#[doc = "Scan End Status Clear Register"]
pub mod adscanendscr;
#[doc = "ADDR (r) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "A/D Data Register %s"]
pub mod addr;
#[doc = "ADEXDR (r) register accessor: an alias for `Reg<ADEXDR_SPEC>`"]
pub type ADEXDR = crate::Reg<adexdr::ADEXDR_SPEC>;
#[doc = "A/D Extended Analog Data Register %s"]
pub mod adexdr;
pub use adexdr as adexdr5;
pub use ADEXDR as ADEXDR5;
#[doc = "ADFIFODR (r) register accessor: an alias for `Reg<ADFIFODR_SPEC>`"]
pub type ADFIFODR = crate::Reg<adfifodr::ADFIFODR_SPEC>;
#[doc = "FIFO Data Register %s"]
pub mod adfifodr;
