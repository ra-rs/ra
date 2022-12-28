#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel 0 Nominal Bitrate Configuration Register"]
    pub cfdc0ncfg: CFDC0NCFG,
    #[doc = "0x04 - Channel 0 Control Register"]
    pub cfdc0ctr: CFDC0CTR,
    #[doc = "0x08 - Channel 0 Status Register"]
    pub cfdc0sts: CFDC0STS,
    #[doc = "0x0c - Channel 0 Error Flag Register"]
    pub cfdc0erfl: CFDC0ERFL,
    _reserved4: [u8; 0x04],
    #[doc = "0x14 - Global Configuration Register"]
    pub cfdgcfg: CFDGCFG,
    #[doc = "0x18 - Global Control Register"]
    pub cfdgctr: CFDGCTR,
    #[doc = "0x1c - Global Status Register"]
    pub cfdgsts: CFDGSTS,
    #[doc = "0x20 - Global Error Flag Register"]
    pub cfdgerfl: CFDGERFL,
    #[doc = "0x24 - Global Timestamp Counter Register"]
    pub cfdgtsc: CFDGTSC,
    #[doc = "0x28 - Global Acceptance Filter List Entry Control Register"]
    pub cfdgaflectr: CFDGAFLECTR,
    #[doc = "0x2c - Global Acceptance Filter List Configuration Register"]
    pub cfdgaflcfg: CFDGAFLCFG,
    #[doc = "0x30 - RX Message Buffer Number Register"]
    pub cfdrmnb: CFDRMNB,
    #[doc = "0x34 - RX Message Buffer New Data Register"]
    pub cfdrmnd: CFDRMND,
    #[doc = "0x38 - RX Message Buffer Interrupt Enable Configuration Register"]
    pub cfdrmiec: CFDRMIEC,
    #[doc = "0x3c..0x44 - RX FIFO Configuration/Control Registers %s"]
    pub cfdrfcc: [CFDRFCC; 2],
    #[doc = "0x44..0x4c - RX FIFO Status Registers %s"]
    pub cfdrfsts: [CFDRFSTS; 2],
    #[doc = "0x4c..0x54 - RX FIFO Pointer Control Registers %s"]
    pub cfdrfpctr: [CFDRFPCTR; 2],
    #[doc = "0x54 - Common FIFO Configuration/Control Register"]
    pub cfdcfcc: CFDCFCC,
    #[doc = "0x58 - Common FIFO Status Register"]
    pub cfdcfsts: CFDCFSTS,
    #[doc = "0x5c - Common FIFO Pointer Control Register"]
    pub cfdcfpctr: CFDCFPCTR,
    #[doc = "0x60 - FIFO Empty Status Register"]
    pub cfdfests: CFDFESTS,
    #[doc = "0x64 - FIFO Full Status Register"]
    pub cfdffsts: CFDFFSTS,
    #[doc = "0x68 - FIFO Message Lost Status Register"]
    pub cfdfmsts: CFDFMSTS,
    #[doc = "0x6c - RX FIFO Interrupt Flag Status Register"]
    pub cfdrfists: CFDRFISTS,
    #[doc = "0x70 - TX Message Buffer Control Registers %s"]
    pub cfdtmc: [CFDTMC; 4],
    #[doc = "0x74 - TX Message Buffer Status Registers %s"]
    pub cfdtmsts: [CFDTMSTS; 4],
    #[doc = "0x78 - TX Message Buffer Transmission Request Status Register"]
    pub cfdtmtrsts: CFDTMTRSTS,
    #[doc = "0x7c - TX Message Buffer Transmission Abort Request Status Register"]
    pub cfdtmtarsts: CFDTMTARSTS,
    #[doc = "0x80 - TX Message Buffer Transmission Completion Status Register"]
    pub cfdtmtcsts: CFDTMTCSTS,
    #[doc = "0x84 - TX Message Buffer Transmission Abort Status Register"]
    pub cfdtmtasts: CFDTMTASTS,
    #[doc = "0x88 - TX Message Buffer Interrupt Enable Configuration Register"]
    pub cfdtmiec: CFDTMIEC,
    #[doc = "0x8c - TX Queue Configuration/Control Register"]
    pub cfdtxqcc: CFDTXQCC,
    #[doc = "0x90 - TX Queue Status Register"]
    pub cfdtxqsts: CFDTXQSTS,
    #[doc = "0x94 - TX Queue Pointer Control Register"]
    pub cfdtxqpctr: CFDTXQPCTR,
    #[doc = "0x98 - TX History List Configuration/Control Register"]
    pub cfdthlcc: CFDTHLCC,
    #[doc = "0x9c - TX History List Status Register"]
    pub cfdthlsts: CFDTHLSTS,
    #[doc = "0xa0 - TX History List Pointer Control Register"]
    pub cfdthlpctr: CFDTHLPCTR,
    #[doc = "0xa4 - Global TX Interrupt Status Register"]
    pub cfdgtintsts: CFDGTINTSTS,
    #[doc = "0xa8 - Global Test Configuration Register"]
    pub cfdgtstcfg: CFDGTSTCFG,
    #[doc = "0xac - Global Test Control Register"]
    pub cfdgtstctr: CFDGTSTCTR,
    #[doc = "0xb0 - Global FD Configuration Register"]
    pub cfdgfdcfg: CFDGFDCFG,
    _reserved41: [u8; 0x04],
    #[doc = "0xb8 - Global Lock Key Register"]
    pub cfdglockk: CFDGLOCKK,
    _reserved42: [u8; 0x04],
    #[doc = "0xc0 - Global AFL Ignore Entry Register"]
    pub cfdgaflignent: CFDGAFLIGNENT,
    #[doc = "0xc4 - Global AFL Ignore Control Register"]
    pub cfdgaflignctr: CFDGAFLIGNCTR,
    #[doc = "0xc8 - DMA Transfer Control Register"]
    pub cfdcdtct: CFDCDTCT,
    #[doc = "0xcc - DMA Transfer Status Register"]
    pub cfdcdtsts: CFDCDTSTS,
    _reserved46: [u8; 0x08],
    #[doc = "0xd8 - Global SW reset Register"]
    pub cfdgrstc: CFDGRSTC,
    _reserved47: [u8; 0x24],
    #[doc = "0x100 - Channel 0 Data Bitrate Configuration Register"]
    pub cfdc0dcfg: CFDC0DCFG,
    #[doc = "0x104 - Channel 0 CANFD Configuration Register"]
    pub cfdc0fdcfg: CFDC0FDCFG,
    #[doc = "0x108 - Channel 0 CANFD Control Register"]
    pub cfdc0fdctr: CFDC0FDCTR,
    #[doc = "0x10c - Channel 0 CANFD Status Register"]
    pub cfdc0fdsts: CFDC0FDSTS,
    #[doc = "0x110 - Channel 0 CANFD CRC Register"]
    pub cfdc0fdcrc: CFDC0FDCRC,
    _reserved52: [u8; 0x0c],
    #[doc = "0x120 - Global Acceptance Filter List ID Registers"]
    pub cfdgaflid1: CFDGAFLID,
    #[doc = "0x124 - Global Acceptance Filter List Mask Registers"]
    pub cfdgaflm1: CFDGAFLM,
    #[doc = "0x128 - Global Acceptance Filter List Pointer 0 Registers"]
    pub cfdgaflp01: CFDGAFLP0,
    #[doc = "0x12c - Global Acceptance Filter List Pointer 1 Registers"]
    pub cfdgaflp11: CFDGAFLP1,
    #[doc = "0x130 - Global Acceptance Filter List ID Registers"]
    pub cfdgaflid2: CFDGAFLID,
    #[doc = "0x134 - Global Acceptance Filter List Mask Registers"]
    pub cfdgaflm2: CFDGAFLM,
    #[doc = "0x138 - Global Acceptance Filter List Pointer 0 Registers"]
    pub cfdgaflp02: CFDGAFLP0,
    #[doc = "0x13c - Global Acceptance Filter List Pointer 1 Registers"]
    pub cfdgaflp12: CFDGAFLP1,
    #[doc = "0x140 - Global Acceptance Filter List ID Registers"]
    pub cfdgaflid3: CFDGAFLID,
    #[doc = "0x144 - Global Acceptance Filter List Mask Registers"]
    pub cfdgaflm3: CFDGAFLM,
    #[doc = "0x148 - Global Acceptance Filter List Pointer 0 Registers"]
    pub cfdgaflp03: CFDGAFLP0,
    #[doc = "0x14c - Global Acceptance Filter List Pointer 1 Registers"]
    pub cfdgaflp13: CFDGAFLP1,
    #[doc = "0x150 - Global Acceptance Filter List ID Registers"]
    pub cfdgaflid4: CFDGAFLID,
    #[doc = "0x154 - Global Acceptance Filter List Mask Registers"]
    pub cfdgaflm4: CFDGAFLM,
    #[doc = "0x158 - Global Acceptance Filter List Pointer 0 Registers"]
    pub cfdgaflp04: CFDGAFLP0,
    #[doc = "0x15c - Global Acceptance Filter List Pointer 1 Registers"]
    pub cfdgaflp14: CFDGAFLP1,
    #[doc = "0x160 - Global Acceptance Filter List ID Registers"]
    pub cfdgaflid5: CFDGAFLID,
    #[doc = "0x164 - Global Acceptance Filter List Mask Registers"]
    pub cfdgaflm5: CFDGAFLM,
    #[doc = "0x168 - Global Acceptance Filter List Pointer 0 Registers"]
    pub cfdgaflp05: CFDGAFLP0,
    #[doc = "0x16c - Global Acceptance Filter List Pointer 1 Registers"]
    pub cfdgaflp15: CFDGAFLP1,
    #[doc = "0x170 - Global Acceptance Filter List ID Registers"]
    pub cfdgaflid6: CFDGAFLID,
    #[doc = "0x174 - Global Acceptance Filter List Mask Registers"]
    pub cfdgaflm6: CFDGAFLM,
    #[doc = "0x178 - Global Acceptance Filter List Pointer 0 Registers"]
    pub cfdgaflp06: CFDGAFLP0,
    #[doc = "0x17c - Global Acceptance Filter List Pointer 1 Registers"]
    pub cfdgaflp16: CFDGAFLP1,
    #[doc = "0x180 - Global Acceptance Filter List ID Registers"]
    pub cfdgaflid7: CFDGAFLID,
    #[doc = "0x184 - Global Acceptance Filter List Mask Registers"]
    pub cfdgaflm7: CFDGAFLM,
    #[doc = "0x188 - Global Acceptance Filter List Pointer 0 Registers"]
    pub cfdgaflp07: CFDGAFLP0,
    #[doc = "0x18c - Global Acceptance Filter List Pointer 1 Registers"]
    pub cfdgaflp17: CFDGAFLP1,
    #[doc = "0x190 - Global Acceptance Filter List ID Registers"]
    pub cfdgaflid8: CFDGAFLID,
    #[doc = "0x194 - Global Acceptance Filter List Mask Registers"]
    pub cfdgaflm8: CFDGAFLM,
    #[doc = "0x198 - Global Acceptance Filter List Pointer 0 Registers"]
    pub cfdgaflp08: CFDGAFLP0,
    #[doc = "0x19c - Global Acceptance Filter List Pointer 1 Registers"]
    pub cfdgaflp18: CFDGAFLP1,
    #[doc = "0x1a0 - Global Acceptance Filter List ID Registers"]
    pub cfdgaflid9: CFDGAFLID,
    #[doc = "0x1a4 - Global Acceptance Filter List Mask Registers"]
    pub cfdgaflm9: CFDGAFLM,
    #[doc = "0x1a8 - Global Acceptance Filter List Pointer 0 Registers"]
    pub cfdgaflp09: CFDGAFLP0,
    #[doc = "0x1ac - Global Acceptance Filter List Pointer 1 Registers"]
    pub cfdgaflp19: CFDGAFLP1,
    #[doc = "0x1b0 - Global Acceptance Filter List ID Registers"]
    pub cfdgaflid10: CFDGAFLID,
    #[doc = "0x1b4 - Global Acceptance Filter List Mask Registers"]
    pub cfdgaflm10: CFDGAFLM,
    #[doc = "0x1b8 - Global Acceptance Filter List Pointer 0 Registers"]
    pub cfdgaflp010: CFDGAFLP0,
    #[doc = "0x1bc - Global Acceptance Filter List Pointer 1 Registers"]
    pub cfdgaflp110: CFDGAFLP1,
    #[doc = "0x1c0 - Global Acceptance Filter List ID Registers"]
    pub cfdgaflid11: CFDGAFLID,
    #[doc = "0x1c4 - Global Acceptance Filter List Mask Registers"]
    pub cfdgaflm11: CFDGAFLM,
    #[doc = "0x1c8 - Global Acceptance Filter List Pointer 0 Registers"]
    pub cfdgaflp011: CFDGAFLP0,
    #[doc = "0x1cc - Global Acceptance Filter List Pointer 1 Registers"]
    pub cfdgaflp111: CFDGAFLP1,
    #[doc = "0x1d0 - Global Acceptance Filter List ID Registers"]
    pub cfdgaflid12: CFDGAFLID,
    #[doc = "0x1d4 - Global Acceptance Filter List Mask Registers"]
    pub cfdgaflm12: CFDGAFLM,
    #[doc = "0x1d8 - Global Acceptance Filter List Pointer 0 Registers"]
    pub cfdgaflp012: CFDGAFLP0,
    #[doc = "0x1dc - Global Acceptance Filter List Pointer 1 Registers"]
    pub cfdgaflp112: CFDGAFLP1,
    #[doc = "0x1e0 - Global Acceptance Filter List ID Registers"]
    pub cfdgaflid13: CFDGAFLID,
    #[doc = "0x1e4 - Global Acceptance Filter List Mask Registers"]
    pub cfdgaflm13: CFDGAFLM,
    #[doc = "0x1e8 - Global Acceptance Filter List Pointer 0 Registers"]
    pub cfdgaflp013: CFDGAFLP0,
    #[doc = "0x1ec - Global Acceptance Filter List Pointer 1 Registers"]
    pub cfdgaflp113: CFDGAFLP1,
    #[doc = "0x1f0 - Global Acceptance Filter List ID Registers"]
    pub cfdgaflid14: CFDGAFLID,
    #[doc = "0x1f4 - Global Acceptance Filter List Mask Registers"]
    pub cfdgaflm14: CFDGAFLM,
    #[doc = "0x1f8 - Global Acceptance Filter List Pointer 0 Registers"]
    pub cfdgaflp014: CFDGAFLP0,
    #[doc = "0x1fc - Global Acceptance Filter List Pointer 1 Registers"]
    pub cfdgaflp114: CFDGAFLP1,
    #[doc = "0x200 - Global Acceptance Filter List ID Registers"]
    pub cfdgaflid15: CFDGAFLID,
    #[doc = "0x204 - Global Acceptance Filter List Mask Registers"]
    pub cfdgaflm15: CFDGAFLM,
    #[doc = "0x208 - Global Acceptance Filter List Pointer 0 Registers"]
    pub cfdgaflp015: CFDGAFLP0,
    #[doc = "0x20c - Global Acceptance Filter List Pointer 1 Registers"]
    pub cfdgaflp115: CFDGAFLP1,
    #[doc = "0x210 - Global Acceptance Filter List ID Registers"]
    pub cfdgaflid16: CFDGAFLID,
    #[doc = "0x214 - Global Acceptance Filter List Mask Registers"]
    pub cfdgaflm16: CFDGAFLM,
    #[doc = "0x218 - Global Acceptance Filter List Pointer 0 Registers"]
    pub cfdgaflp016: CFDGAFLP0,
    #[doc = "0x21c - Global Acceptance Filter List Pointer 1 Registers"]
    pub cfdgaflp116: CFDGAFLP1,
    _reserved116: [u8; 0x60],
    #[doc = "0x280..0x380 - RAM Test Page Access Registers %s"]
    pub cfdrpgacc: [CFDRPGACC; 64],
    _reserved117: [u8; 0x01a0],
    #[doc = "0x520 - RX FIFO Access ID Register %s"]
    pub cfdrfid0: CFDRFID,
    #[doc = "0x524 - RX FIFO Access Pointer Register %s"]
    pub cfdrfptr0: CFDRFPTR,
    #[doc = "0x528 - RX FIFO Access CANFD Status Register %s"]
    pub cfdrffdsts0: CFDRFFDSTS,
    #[doc = "0x52c - RX FIFO Access Data Field 0 Register %s"]
    pub cfdrfdf0_0: CFDRFDF_0,
    #[doc = "0x530 - RX FIFO Access Data Field 1 Register %s"]
    pub cfdrfdf0_1: CFDRFDF_1,
    #[doc = "0x534 - RX FIFO Access Data Field 2 Register %s"]
    pub cfdrfdf0_2: CFDRFDF_2,
    #[doc = "0x538 - RX FIFO Access Data Field 3 Register %s"]
    pub cfdrfdf0_3: CFDRFDF_3,
    #[doc = "0x53c - RX FIFO Access Data Field 4 Register %s"]
    pub cfdrfdf0_4: CFDRFDF_4,
    #[doc = "0x540 - RX FIFO Access Data Field 5 Register %s"]
    pub cfdrfdf0_5: CFDRFDF_5,
    #[doc = "0x544 - RX FIFO Access Data Field 6 Register %s"]
    pub cfdrfdf0_6: CFDRFDF_6,
    #[doc = "0x548 - RX FIFO Access Data Field 7 Register %s"]
    pub cfdrfdf0_7: CFDRFDF_7,
    #[doc = "0x54c - RX FIFO Access Data Field 8 Register %s"]
    pub cfdrfdf0_8: CFDRFDF_8,
    #[doc = "0x550 - RX FIFO Access Data Field 9 Register %s"]
    pub cfdrfdf0_9: CFDRFDF_9,
    #[doc = "0x554 - RX FIFO Access Data Field 10 Register %s"]
    pub cfdrfdf0_10: CFDRFDF_10,
    #[doc = "0x558 - RX FIFO Access Data Field 11 Register %s"]
    pub cfdrfdf0_11: CFDRFDF_11,
    #[doc = "0x55c - RX FIFO Access Data Field 12 Register %s"]
    pub cfdrfdf0_12: CFDRFDF_12,
    #[doc = "0x560 - RX FIFO Access Data Field 13 Register %s"]
    pub cfdrfdf0_13: CFDRFDF_13,
    #[doc = "0x564 - RX FIFO Access Data Field 14 Register %s"]
    pub cfdrfdf0_14: CFDRFDF_14,
    #[doc = "0x568 - RX FIFO Access Data Field 15 Register %s"]
    pub cfdrfdf0_15: CFDRFDF_15,
    #[doc = "0x56c - RX FIFO Access ID Register %s"]
    pub cfdrfid1: CFDRFID,
    #[doc = "0x570 - RX FIFO Access Pointer Register %s"]
    pub cfdrfptr1: CFDRFPTR,
    #[doc = "0x574 - RX FIFO Access CANFD Status Register %s"]
    pub cfdrffdsts1: CFDRFFDSTS,
    #[doc = "0x578 - RX FIFO Access Data Field 0 Register %s"]
    pub cfdrfdf1_0: CFDRFDF_0,
    #[doc = "0x57c - RX FIFO Access Data Field 1 Register %s"]
    pub cfdrfdf1_1: CFDRFDF_1,
    #[doc = "0x580 - RX FIFO Access Data Field 2 Register %s"]
    pub cfdrfdf1_2: CFDRFDF_2,
    #[doc = "0x584 - RX FIFO Access Data Field 3 Register %s"]
    pub cfdrfdf1_3: CFDRFDF_3,
    #[doc = "0x588 - RX FIFO Access Data Field 4 Register %s"]
    pub cfdrfdf1_4: CFDRFDF_4,
    #[doc = "0x58c - RX FIFO Access Data Field 5 Register %s"]
    pub cfdrfdf1_5: CFDRFDF_5,
    #[doc = "0x590 - RX FIFO Access Data Field 6 Register %s"]
    pub cfdrfdf1_6: CFDRFDF_6,
    #[doc = "0x594 - RX FIFO Access Data Field 7 Register %s"]
    pub cfdrfdf1_7: CFDRFDF_7,
    #[doc = "0x598 - RX FIFO Access Data Field 8 Register %s"]
    pub cfdrfdf1_8: CFDRFDF_8,
    #[doc = "0x59c - RX FIFO Access Data Field 9 Register %s"]
    pub cfdrfdf1_9: CFDRFDF_9,
    #[doc = "0x5a0 - RX FIFO Access Data Field 10 Register %s"]
    pub cfdrfdf1_10: CFDRFDF_10,
    #[doc = "0x5a4 - RX FIFO Access Data Field 11 Register %s"]
    pub cfdrfdf1_11: CFDRFDF_11,
    #[doc = "0x5a8 - RX FIFO Access Data Field 12 Register %s"]
    pub cfdrfdf1_12: CFDRFDF_12,
    #[doc = "0x5ac - RX FIFO Access Data Field 13 Register %s"]
    pub cfdrfdf1_13: CFDRFDF_13,
    #[doc = "0x5b0 - RX FIFO Access Data Field 14 Register %s"]
    pub cfdrfdf1_14: CFDRFDF_14,
    #[doc = "0x5b4 - RX FIFO Access Data Field 15 Register %s"]
    pub cfdrfdf1_15: CFDRFDF_15,
    #[doc = "0x5b8 - Common FIFO Access ID Register"]
    pub cfdcfid: CFDCFID,
    #[doc = "0x5bc - Common FIFO Access Pointer Register"]
    pub cfdcfptr: CFDCFPTR,
    #[doc = "0x5c0 - Common FIFO Access CANFD Control/Status Register"]
    pub cfdcffdcsts: CFDCFFDCSTS,
    #[doc = "0x5c4..0x604 - Common FIFO Access Data Field %s Registers"]
    pub cfdcfdf: [CFDCFDF; 16],
    #[doc = "0x604 - TX Message Buffer ID Registers"]
    pub cfdtmid0: CFDTMID,
    #[doc = "0x608 - TX Message Buffer Pointer Register"]
    pub cfdtmptr0: CFDTMPTR,
    #[doc = "0x60c - TX Message Buffer CANFD Control Register"]
    pub cfdtmfdctr0: CFDTMFDCTR,
    #[doc = "0x610 - TX Message Buffer Data Field Register"]
    pub cfdtmdf0_0: CFDTMDF_0,
    #[doc = "0x614 - TX Message Buffer Data Field Register"]
    pub cfdtmdf0_1: CFDTMDF_1,
    #[doc = "0x618 - TX Message Buffer Data Field Register"]
    pub cfdtmdf0_2: CFDTMDF_2,
    #[doc = "0x61c - TX Message Buffer Data Field Register"]
    pub cfdtmdf0_3: CFDTMDF_3,
    #[doc = "0x620 - TX Message Buffer Data Field Register"]
    pub cfdtmdf0_4: CFDTMDF_4,
    #[doc = "0x624 - TX Message Buffer Data Field Register"]
    pub cfdtmdf0_5: CFDTMDF_5,
    #[doc = "0x628 - TX Message Buffer Data Field Register"]
    pub cfdtmdf0_6: CFDTMDF_6,
    #[doc = "0x62c - TX Message Buffer Data Field Register"]
    pub cfdtmdf0_7: CFDTMDF_7,
    #[doc = "0x630 - TX Message Buffer Data Field Register"]
    pub cfdtmdf0_8: CFDTMDF_8,
    #[doc = "0x634 - TX Message Buffer Data Field Register"]
    pub cfdtmdf0_9: CFDTMDF_9,
    #[doc = "0x638 - TX Message Buffer Data Field Register"]
    pub cfdtmdf0_10: CFDTMDF_10,
    #[doc = "0x63c - TX Message Buffer Data Field Register"]
    pub cfdtmdf0_11: CFDTMDF_11,
    #[doc = "0x640 - TX Message Buffer Data Field Register"]
    pub cfdtmdf0_12: CFDTMDF_12,
    #[doc = "0x644 - TX Message Buffer Data Field Register"]
    pub cfdtmdf0_13: CFDTMDF_13,
    #[doc = "0x648 - TX Message Buffer Data Field Register"]
    pub cfdtmdf0_14: CFDTMDF_14,
    #[doc = "0x64c - TX Message Buffer Data Field Register"]
    pub cfdtmdf0_15: CFDTMDF_15,
    #[doc = "0x650 - TX Message Buffer ID Registers"]
    pub cfdtmid1: CFDTMID,
    #[doc = "0x654 - TX Message Buffer Pointer Register"]
    pub cfdtmptr1: CFDTMPTR,
    #[doc = "0x658 - TX Message Buffer CANFD Control Register"]
    pub cfdtmfdctr1: CFDTMFDCTR,
    #[doc = "0x65c - TX Message Buffer Data Field Register"]
    pub cfdtmdf1_0: CFDTMDF_0,
    #[doc = "0x660 - TX Message Buffer Data Field Register"]
    pub cfdtmdf1_1: CFDTMDF_1,
    #[doc = "0x664 - TX Message Buffer Data Field Register"]
    pub cfdtmdf1_2: CFDTMDF_2,
    #[doc = "0x668 - TX Message Buffer Data Field Register"]
    pub cfdtmdf1_3: CFDTMDF_3,
    #[doc = "0x66c - TX Message Buffer Data Field Register"]
    pub cfdtmdf1_4: CFDTMDF_4,
    #[doc = "0x670 - TX Message Buffer Data Field Register"]
    pub cfdtmdf1_5: CFDTMDF_5,
    #[doc = "0x674 - TX Message Buffer Data Field Register"]
    pub cfdtmdf1_6: CFDTMDF_6,
    #[doc = "0x678 - TX Message Buffer Data Field Register"]
    pub cfdtmdf1_7: CFDTMDF_7,
    #[doc = "0x67c - TX Message Buffer Data Field Register"]
    pub cfdtmdf1_8: CFDTMDF_8,
    #[doc = "0x680 - TX Message Buffer Data Field Register"]
    pub cfdtmdf1_9: CFDTMDF_9,
    #[doc = "0x684 - TX Message Buffer Data Field Register"]
    pub cfdtmdf1_10: CFDTMDF_10,
    #[doc = "0x688 - TX Message Buffer Data Field Register"]
    pub cfdtmdf1_11: CFDTMDF_11,
    #[doc = "0x68c - TX Message Buffer Data Field Register"]
    pub cfdtmdf1_12: CFDTMDF_12,
    #[doc = "0x690 - TX Message Buffer Data Field Register"]
    pub cfdtmdf1_13: CFDTMDF_13,
    #[doc = "0x694 - TX Message Buffer Data Field Register"]
    pub cfdtmdf1_14: CFDTMDF_14,
    #[doc = "0x698 - TX Message Buffer Data Field Register"]
    pub cfdtmdf1_15: CFDTMDF_15,
    #[doc = "0x69c - TX Message Buffer ID Registers"]
    pub cfdtmid2: CFDTMID,
    #[doc = "0x6a0 - TX Message Buffer Pointer Register"]
    pub cfdtmptr2: CFDTMPTR,
    #[doc = "0x6a4 - TX Message Buffer CANFD Control Register"]
    pub cfdtmfdctr2: CFDTMFDCTR,
    #[doc = "0x6a8 - TX Message Buffer Data Field Register"]
    pub cfdtmdf2_0: CFDTMDF_0,
    #[doc = "0x6ac - TX Message Buffer Data Field Register"]
    pub cfdtmdf2_1: CFDTMDF_1,
    #[doc = "0x6b0 - TX Message Buffer Data Field Register"]
    pub cfdtmdf2_2: CFDTMDF_2,
    #[doc = "0x6b4 - TX Message Buffer Data Field Register"]
    pub cfdtmdf2_3: CFDTMDF_3,
    #[doc = "0x6b8 - TX Message Buffer Data Field Register"]
    pub cfdtmdf2_4: CFDTMDF_4,
    #[doc = "0x6bc - TX Message Buffer Data Field Register"]
    pub cfdtmdf2_5: CFDTMDF_5,
    #[doc = "0x6c0 - TX Message Buffer Data Field Register"]
    pub cfdtmdf2_6: CFDTMDF_6,
    #[doc = "0x6c4 - TX Message Buffer Data Field Register"]
    pub cfdtmdf2_7: CFDTMDF_7,
    #[doc = "0x6c8 - TX Message Buffer Data Field Register"]
    pub cfdtmdf2_8: CFDTMDF_8,
    #[doc = "0x6cc - TX Message Buffer Data Field Register"]
    pub cfdtmdf2_9: CFDTMDF_9,
    #[doc = "0x6d0 - TX Message Buffer Data Field Register"]
    pub cfdtmdf2_10: CFDTMDF_10,
    #[doc = "0x6d4 - TX Message Buffer Data Field Register"]
    pub cfdtmdf2_11: CFDTMDF_11,
    #[doc = "0x6d8 - TX Message Buffer Data Field Register"]
    pub cfdtmdf2_12: CFDTMDF_12,
    #[doc = "0x6dc - TX Message Buffer Data Field Register"]
    pub cfdtmdf2_13: CFDTMDF_13,
    #[doc = "0x6e0 - TX Message Buffer Data Field Register"]
    pub cfdtmdf2_14: CFDTMDF_14,
    #[doc = "0x6e4 - TX Message Buffer Data Field Register"]
    pub cfdtmdf2_15: CFDTMDF_15,
    #[doc = "0x6e8 - TX Message Buffer ID Registers"]
    pub cfdtmid3: CFDTMID,
    #[doc = "0x6ec - TX Message Buffer Pointer Register"]
    pub cfdtmptr3: CFDTMPTR,
    #[doc = "0x6f0 - TX Message Buffer CANFD Control Register"]
    pub cfdtmfdctr3: CFDTMFDCTR,
    #[doc = "0x6f4 - TX Message Buffer Data Field Register"]
    pub cfdtmdf3_0: CFDTMDF_0,
    #[doc = "0x6f8 - TX Message Buffer Data Field Register"]
    pub cfdtmdf3_1: CFDTMDF_1,
    #[doc = "0x6fc - TX Message Buffer Data Field Register"]
    pub cfdtmdf3_2: CFDTMDF_2,
    #[doc = "0x700 - TX Message Buffer Data Field Register"]
    pub cfdtmdf3_3: CFDTMDF_3,
    #[doc = "0x704 - TX Message Buffer Data Field Register"]
    pub cfdtmdf3_4: CFDTMDF_4,
    #[doc = "0x708 - TX Message Buffer Data Field Register"]
    pub cfdtmdf3_5: CFDTMDF_5,
    #[doc = "0x70c - TX Message Buffer Data Field Register"]
    pub cfdtmdf3_6: CFDTMDF_6,
    #[doc = "0x710 - TX Message Buffer Data Field Register"]
    pub cfdtmdf3_7: CFDTMDF_7,
    #[doc = "0x714 - TX Message Buffer Data Field Register"]
    pub cfdtmdf3_8: CFDTMDF_8,
    #[doc = "0x718 - TX Message Buffer Data Field Register"]
    pub cfdtmdf3_9: CFDTMDF_9,
    #[doc = "0x71c - TX Message Buffer Data Field Register"]
    pub cfdtmdf3_10: CFDTMDF_10,
    #[doc = "0x720 - TX Message Buffer Data Field Register"]
    pub cfdtmdf3_11: CFDTMDF_11,
    #[doc = "0x724 - TX Message Buffer Data Field Register"]
    pub cfdtmdf3_12: CFDTMDF_12,
    #[doc = "0x728 - TX Message Buffer Data Field Register"]
    pub cfdtmdf3_13: CFDTMDF_13,
    #[doc = "0x72c - TX Message Buffer Data Field Register"]
    pub cfdtmdf3_14: CFDTMDF_14,
    #[doc = "0x730 - TX Message Buffer Data Field Register"]
    pub cfdtmdf3_15: CFDTMDF_15,
    _reserved235: [u8; 0x0c],
    #[doc = "0x740 - TX History List Access Register 0"]
    pub cfdthlacc0: CFDTHLACC0,
    #[doc = "0x744 - TX History List Access Register 1"]
    pub cfdthlacc1: CFDTHLACC1,
    _reserved237: [u8; 0x01d8],
    #[doc = "0x920 - RX Message Buffer ID Registers"]
    pub cfdrmid0: CFDRMID,
    #[doc = "0x924 - RX Message Buffer Pointer Registers"]
    pub cfdrmptr0: CFDRMPTR,
    #[doc = "0x928 - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts0: CFDRMFDSTS,
    #[doc = "0x92c - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf0_0: CFDRMDF_0,
    #[doc = "0x930 - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf0_1: CFDRMDF_1,
    #[doc = "0x934 - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf0_2: CFDRMDF_2,
    #[doc = "0x938 - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf0_3: CFDRMDF_3,
    #[doc = "0x93c - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf0_4: CFDRMDF_4,
    #[doc = "0x940 - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf0_5: CFDRMDF_5,
    #[doc = "0x944 - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf0_6: CFDRMDF_6,
    #[doc = "0x948 - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf0_7: CFDRMDF_7,
    #[doc = "0x94c - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf0_8: CFDRMDF_8,
    #[doc = "0x950 - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf0_9: CFDRMDF_9,
    #[doc = "0x954 - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf0_10: CFDRMDF_10,
    #[doc = "0x958 - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf0_11: CFDRMDF_11,
    #[doc = "0x95c - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf0_12: CFDRMDF_12,
    #[doc = "0x960 - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf0_13: CFDRMDF_13,
    #[doc = "0x964 - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf0_14: CFDRMDF_14,
    #[doc = "0x968 - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf0_15: CFDRMDF_15,
    #[doc = "0x96c - RX Message Buffer ID Registers"]
    pub cfdrmid1: CFDRMID,
    #[doc = "0x970 - RX Message Buffer Pointer Registers"]
    pub cfdrmptr1: CFDRMPTR,
    #[doc = "0x974 - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts1: CFDRMFDSTS,
    #[doc = "0x978 - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf1_0: CFDRMDF_0,
    #[doc = "0x97c - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf1_1: CFDRMDF_1,
    #[doc = "0x980 - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf1_2: CFDRMDF_2,
    #[doc = "0x984 - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf1_3: CFDRMDF_3,
    #[doc = "0x988 - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf1_4: CFDRMDF_4,
    #[doc = "0x98c - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf1_5: CFDRMDF_5,
    #[doc = "0x990 - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf1_6: CFDRMDF_6,
    #[doc = "0x994 - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf1_7: CFDRMDF_7,
    #[doc = "0x998 - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf1_8: CFDRMDF_8,
    #[doc = "0x99c - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf1_9: CFDRMDF_9,
    #[doc = "0x9a0 - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf1_10: CFDRMDF_10,
    #[doc = "0x9a4 - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf1_11: CFDRMDF_11,
    #[doc = "0x9a8 - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf1_12: CFDRMDF_12,
    #[doc = "0x9ac - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf1_13: CFDRMDF_13,
    #[doc = "0x9b0 - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf1_14: CFDRMDF_14,
    #[doc = "0x9b4 - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf1_15: CFDRMDF_15,
    #[doc = "0x9b8 - RX Message Buffer ID Registers"]
    pub cfdrmid2: CFDRMID,
    #[doc = "0x9bc - RX Message Buffer Pointer Registers"]
    pub cfdrmptr2: CFDRMPTR,
    #[doc = "0x9c0 - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts2: CFDRMFDSTS,
    #[doc = "0x9c4 - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf2_0: CFDRMDF_0,
    #[doc = "0x9c8 - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf2_1: CFDRMDF_1,
    #[doc = "0x9cc - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf2_2: CFDRMDF_2,
    #[doc = "0x9d0 - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf2_3: CFDRMDF_3,
    #[doc = "0x9d4 - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf2_4: CFDRMDF_4,
    #[doc = "0x9d8 - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf2_5: CFDRMDF_5,
    #[doc = "0x9dc - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf2_6: CFDRMDF_6,
    #[doc = "0x9e0 - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf2_7: CFDRMDF_7,
    #[doc = "0x9e4 - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf2_8: CFDRMDF_8,
    #[doc = "0x9e8 - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf2_9: CFDRMDF_9,
    #[doc = "0x9ec - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf2_10: CFDRMDF_10,
    #[doc = "0x9f0 - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf2_11: CFDRMDF_11,
    #[doc = "0x9f4 - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf2_12: CFDRMDF_12,
    #[doc = "0x9f8 - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf2_13: CFDRMDF_13,
    #[doc = "0x9fc - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf2_14: CFDRMDF_14,
    #[doc = "0xa00 - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf2_15: CFDRMDF_15,
    #[doc = "0xa04 - RX Message Buffer ID Registers"]
    pub cfdrmid3: CFDRMID,
    #[doc = "0xa08 - RX Message Buffer Pointer Registers"]
    pub cfdrmptr3: CFDRMPTR,
    #[doc = "0xa0c - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts3: CFDRMFDSTS,
    #[doc = "0xa10 - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf3_0: CFDRMDF_0,
    #[doc = "0xa14 - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf3_1: CFDRMDF_1,
    #[doc = "0xa18 - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf3_2: CFDRMDF_2,
    #[doc = "0xa1c - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf3_3: CFDRMDF_3,
    #[doc = "0xa20 - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf3_4: CFDRMDF_4,
    #[doc = "0xa24 - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf3_5: CFDRMDF_5,
    #[doc = "0xa28 - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf3_6: CFDRMDF_6,
    #[doc = "0xa2c - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf3_7: CFDRMDF_7,
    #[doc = "0xa30 - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf3_8: CFDRMDF_8,
    #[doc = "0xa34 - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf3_9: CFDRMDF_9,
    #[doc = "0xa38 - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf3_10: CFDRMDF_10,
    #[doc = "0xa3c - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf3_11: CFDRMDF_11,
    #[doc = "0xa40 - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf3_12: CFDRMDF_12,
    #[doc = "0xa44 - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf3_13: CFDRMDF_13,
    #[doc = "0xa48 - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf3_14: CFDRMDF_14,
    #[doc = "0xa4c - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf3_15: CFDRMDF_15,
    #[doc = "0xa50 - RX Message Buffer ID Registers"]
    pub cfdrmid4: CFDRMID,
    #[doc = "0xa54 - RX Message Buffer Pointer Registers"]
    pub cfdrmptr4: CFDRMPTR,
    #[doc = "0xa58 - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts4: CFDRMFDSTS,
    #[doc = "0xa5c - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf4_0: CFDRMDF_0,
    #[doc = "0xa60 - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf4_1: CFDRMDF_1,
    #[doc = "0xa64 - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf4_2: CFDRMDF_2,
    #[doc = "0xa68 - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf4_3: CFDRMDF_3,
    #[doc = "0xa6c - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf4_4: CFDRMDF_4,
    #[doc = "0xa70 - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf4_5: CFDRMDF_5,
    #[doc = "0xa74 - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf4_6: CFDRMDF_6,
    #[doc = "0xa78 - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf4_7: CFDRMDF_7,
    #[doc = "0xa7c - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf4_8: CFDRMDF_8,
    #[doc = "0xa80 - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf4_9: CFDRMDF_9,
    #[doc = "0xa84 - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf4_10: CFDRMDF_10,
    #[doc = "0xa88 - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf4_11: CFDRMDF_11,
    #[doc = "0xa8c - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf4_12: CFDRMDF_12,
    #[doc = "0xa90 - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf4_13: CFDRMDF_13,
    #[doc = "0xa94 - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf4_14: CFDRMDF_14,
    #[doc = "0xa98 - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf4_15: CFDRMDF_15,
    #[doc = "0xa9c - RX Message Buffer ID Registers"]
    pub cfdrmid5: CFDRMID,
    #[doc = "0xaa0 - RX Message Buffer Pointer Registers"]
    pub cfdrmptr5: CFDRMPTR,
    #[doc = "0xaa4 - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts5: CFDRMFDSTS,
    #[doc = "0xaa8 - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf5_0: CFDRMDF_0,
    #[doc = "0xaac - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf5_1: CFDRMDF_1,
    #[doc = "0xab0 - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf5_2: CFDRMDF_2,
    #[doc = "0xab4 - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf5_3: CFDRMDF_3,
    #[doc = "0xab8 - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf5_4: CFDRMDF_4,
    #[doc = "0xabc - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf5_5: CFDRMDF_5,
    #[doc = "0xac0 - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf5_6: CFDRMDF_6,
    #[doc = "0xac4 - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf5_7: CFDRMDF_7,
    #[doc = "0xac8 - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf5_8: CFDRMDF_8,
    #[doc = "0xacc - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf5_9: CFDRMDF_9,
    #[doc = "0xad0 - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf5_10: CFDRMDF_10,
    #[doc = "0xad4 - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf5_11: CFDRMDF_11,
    #[doc = "0xad8 - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf5_12: CFDRMDF_12,
    #[doc = "0xadc - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf5_13: CFDRMDF_13,
    #[doc = "0xae0 - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf5_14: CFDRMDF_14,
    #[doc = "0xae4 - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf5_15: CFDRMDF_15,
    #[doc = "0xae8 - RX Message Buffer ID Registers"]
    pub cfdrmid6: CFDRMID,
    #[doc = "0xaec - RX Message Buffer Pointer Registers"]
    pub cfdrmptr6: CFDRMPTR,
    #[doc = "0xaf0 - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts6: CFDRMFDSTS,
    #[doc = "0xaf4 - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf6_0: CFDRMDF_0,
    #[doc = "0xaf8 - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf6_1: CFDRMDF_1,
    #[doc = "0xafc - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf6_2: CFDRMDF_2,
    #[doc = "0xb00 - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf6_3: CFDRMDF_3,
    #[doc = "0xb04 - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf6_4: CFDRMDF_4,
    #[doc = "0xb08 - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf6_5: CFDRMDF_5,
    #[doc = "0xb0c - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf6_6: CFDRMDF_6,
    #[doc = "0xb10 - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf6_7: CFDRMDF_7,
    #[doc = "0xb14 - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf6_8: CFDRMDF_8,
    #[doc = "0xb18 - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf6_9: CFDRMDF_9,
    #[doc = "0xb1c - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf6_10: CFDRMDF_10,
    #[doc = "0xb20 - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf6_11: CFDRMDF_11,
    #[doc = "0xb24 - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf6_12: CFDRMDF_12,
    #[doc = "0xb28 - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf6_13: CFDRMDF_13,
    #[doc = "0xb2c - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf6_14: CFDRMDF_14,
    #[doc = "0xb30 - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf6_15: CFDRMDF_15,
    #[doc = "0xb34 - RX Message Buffer ID Registers"]
    pub cfdrmid7: CFDRMID,
    #[doc = "0xb38 - RX Message Buffer Pointer Registers"]
    pub cfdrmptr7: CFDRMPTR,
    #[doc = "0xb3c - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts7: CFDRMFDSTS,
    #[doc = "0xb40 - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf7_0: CFDRMDF_0,
    #[doc = "0xb44 - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf7_1: CFDRMDF_1,
    #[doc = "0xb48 - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf7_2: CFDRMDF_2,
    #[doc = "0xb4c - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf7_3: CFDRMDF_3,
    #[doc = "0xb50 - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf7_4: CFDRMDF_4,
    #[doc = "0xb54 - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf7_5: CFDRMDF_5,
    #[doc = "0xb58 - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf7_6: CFDRMDF_6,
    #[doc = "0xb5c - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf7_7: CFDRMDF_7,
    #[doc = "0xb60 - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf7_8: CFDRMDF_8,
    #[doc = "0xb64 - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf7_9: CFDRMDF_9,
    #[doc = "0xb68 - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf7_10: CFDRMDF_10,
    #[doc = "0xb6c - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf7_11: CFDRMDF_11,
    #[doc = "0xb70 - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf7_12: CFDRMDF_12,
    #[doc = "0xb74 - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf7_13: CFDRMDF_13,
    #[doc = "0xb78 - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf7_14: CFDRMDF_14,
    #[doc = "0xb7c - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf7_15: CFDRMDF_15,
    _reserved389: [u8; 0x01a0],
    #[doc = "0xd20 - RX Message Buffer ID Registers"]
    pub cfdrmid8: CFDRMID8,
    #[doc = "0xd24 - RX Message Buffer Pointer Registers"]
    pub cfdrmptr8: CFDRMPTR8,
    #[doc = "0xd28 - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts8: CFDRMFDSTS8,
    #[doc = "0xd2c - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf8_0: CFDRMDF8_0,
    #[doc = "0xd30 - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf8_1: CFDRMDF8_1,
    #[doc = "0xd34 - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf8_2: CFDRMDF8_2,
    #[doc = "0xd38 - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf8_3: CFDRMDF8_3,
    #[doc = "0xd3c - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf8_4: CFDRMDF8_4,
    #[doc = "0xd40 - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf8_5: CFDRMDF8_5,
    #[doc = "0xd44 - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf8_6: CFDRMDF8_6,
    #[doc = "0xd48 - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf8_7: CFDRMDF8_7,
    #[doc = "0xd4c - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf8_8: CFDRMDF8_8,
    #[doc = "0xd50 - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf8_9: CFDRMDF8_9,
    #[doc = "0xd54 - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf8_10: CFDRMDF8_10,
    #[doc = "0xd58 - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf8_11: CFDRMDF8_11,
    #[doc = "0xd5c - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf8_12: CFDRMDF8_12,
    #[doc = "0xd60 - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf8_13: CFDRMDF8_13,
    #[doc = "0xd64 - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf8_14: CFDRMDF8_14,
    #[doc = "0xd68 - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf8_15: CFDRMDF8_15,
    #[doc = "0xd6c - RX Message Buffer ID Registers"]
    pub cfdrmid9: CFDRMID8,
    #[doc = "0xd70 - RX Message Buffer Pointer Registers"]
    pub cfdrmptr9: CFDRMPTR8,
    #[doc = "0xd74 - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts9: CFDRMFDSTS8,
    #[doc = "0xd78 - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf9_0: CFDRMDF8_0,
    #[doc = "0xd7c - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf9_1: CFDRMDF8_1,
    #[doc = "0xd80 - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf9_2: CFDRMDF8_2,
    #[doc = "0xd84 - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf9_3: CFDRMDF8_3,
    #[doc = "0xd88 - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf9_4: CFDRMDF8_4,
    #[doc = "0xd8c - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf9_5: CFDRMDF8_5,
    #[doc = "0xd90 - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf9_6: CFDRMDF8_6,
    #[doc = "0xd94 - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf9_7: CFDRMDF8_7,
    #[doc = "0xd98 - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf9_8: CFDRMDF8_8,
    #[doc = "0xd9c - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf9_9: CFDRMDF8_9,
    #[doc = "0xda0 - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf9_10: CFDRMDF8_10,
    #[doc = "0xda4 - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf9_11: CFDRMDF8_11,
    #[doc = "0xda8 - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf9_12: CFDRMDF8_12,
    #[doc = "0xdac - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf9_13: CFDRMDF8_13,
    #[doc = "0xdb0 - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf9_14: CFDRMDF8_14,
    #[doc = "0xdb4 - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf9_15: CFDRMDF8_15,
    #[doc = "0xdb8 - RX Message Buffer ID Registers"]
    pub cfdrmid10: CFDRMID8,
    #[doc = "0xdbc - RX Message Buffer Pointer Registers"]
    pub cfdrmptr10: CFDRMPTR8,
    #[doc = "0xdc0 - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts10: CFDRMFDSTS8,
    #[doc = "0xdc4 - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf10_0: CFDRMDF8_0,
    #[doc = "0xdc8 - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf10_1: CFDRMDF8_1,
    #[doc = "0xdcc - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf10_2: CFDRMDF8_2,
    #[doc = "0xdd0 - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf10_3: CFDRMDF8_3,
    #[doc = "0xdd4 - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf10_4: CFDRMDF8_4,
    #[doc = "0xdd8 - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf10_5: CFDRMDF8_5,
    #[doc = "0xddc - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf10_6: CFDRMDF8_6,
    #[doc = "0xde0 - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf10_7: CFDRMDF8_7,
    #[doc = "0xde4 - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf10_8: CFDRMDF8_8,
    #[doc = "0xde8 - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf10_9: CFDRMDF8_9,
    #[doc = "0xdec - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf10_10: CFDRMDF8_10,
    #[doc = "0xdf0 - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf10_11: CFDRMDF8_11,
    #[doc = "0xdf4 - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf10_12: CFDRMDF8_12,
    #[doc = "0xdf8 - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf10_13: CFDRMDF8_13,
    #[doc = "0xdfc - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf10_14: CFDRMDF8_14,
    #[doc = "0xe00 - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf10_15: CFDRMDF8_15,
    #[doc = "0xe04 - RX Message Buffer ID Registers"]
    pub cfdrmid11: CFDRMID8,
    #[doc = "0xe08 - RX Message Buffer Pointer Registers"]
    pub cfdrmptr11: CFDRMPTR8,
    #[doc = "0xe0c - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts11: CFDRMFDSTS8,
    #[doc = "0xe10 - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf11_0: CFDRMDF8_0,
    #[doc = "0xe14 - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf11_1: CFDRMDF8_1,
    #[doc = "0xe18 - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf11_2: CFDRMDF8_2,
    #[doc = "0xe1c - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf11_3: CFDRMDF8_3,
    #[doc = "0xe20 - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf11_4: CFDRMDF8_4,
    #[doc = "0xe24 - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf11_5: CFDRMDF8_5,
    #[doc = "0xe28 - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf11_6: CFDRMDF8_6,
    #[doc = "0xe2c - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf11_7: CFDRMDF8_7,
    #[doc = "0xe30 - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf11_8: CFDRMDF8_8,
    #[doc = "0xe34 - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf11_9: CFDRMDF8_9,
    #[doc = "0xe38 - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf11_10: CFDRMDF8_10,
    #[doc = "0xe3c - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf11_11: CFDRMDF8_11,
    #[doc = "0xe40 - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf11_12: CFDRMDF8_12,
    #[doc = "0xe44 - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf11_13: CFDRMDF8_13,
    #[doc = "0xe48 - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf11_14: CFDRMDF8_14,
    #[doc = "0xe4c - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf11_15: CFDRMDF8_15,
    #[doc = "0xe50 - RX Message Buffer ID Registers"]
    pub cfdrmid12: CFDRMID8,
    #[doc = "0xe54 - RX Message Buffer Pointer Registers"]
    pub cfdrmptr12: CFDRMPTR8,
    #[doc = "0xe58 - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts12: CFDRMFDSTS8,
    #[doc = "0xe5c - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf12_0: CFDRMDF8_0,
    #[doc = "0xe60 - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf12_1: CFDRMDF8_1,
    #[doc = "0xe64 - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf12_2: CFDRMDF8_2,
    #[doc = "0xe68 - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf12_3: CFDRMDF8_3,
    #[doc = "0xe6c - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf12_4: CFDRMDF8_4,
    #[doc = "0xe70 - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf12_5: CFDRMDF8_5,
    #[doc = "0xe74 - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf12_6: CFDRMDF8_6,
    #[doc = "0xe78 - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf12_7: CFDRMDF8_7,
    #[doc = "0xe7c - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf12_8: CFDRMDF8_8,
    #[doc = "0xe80 - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf12_9: CFDRMDF8_9,
    #[doc = "0xe84 - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf12_10: CFDRMDF8_10,
    #[doc = "0xe88 - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf12_11: CFDRMDF8_11,
    #[doc = "0xe8c - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf12_12: CFDRMDF8_12,
    #[doc = "0xe90 - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf12_13: CFDRMDF8_13,
    #[doc = "0xe94 - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf12_14: CFDRMDF8_14,
    #[doc = "0xe98 - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf12_15: CFDRMDF8_15,
    #[doc = "0xe9c - RX Message Buffer ID Registers"]
    pub cfdrmid13: CFDRMID8,
    #[doc = "0xea0 - RX Message Buffer Pointer Registers"]
    pub cfdrmptr13: CFDRMPTR8,
    #[doc = "0xea4 - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts13: CFDRMFDSTS8,
    #[doc = "0xea8 - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf13_0: CFDRMDF8_0,
    #[doc = "0xeac - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf13_1: CFDRMDF8_1,
    #[doc = "0xeb0 - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf13_2: CFDRMDF8_2,
    #[doc = "0xeb4 - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf13_3: CFDRMDF8_3,
    #[doc = "0xeb8 - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf13_4: CFDRMDF8_4,
    #[doc = "0xebc - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf13_5: CFDRMDF8_5,
    #[doc = "0xec0 - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf13_6: CFDRMDF8_6,
    #[doc = "0xec4 - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf13_7: CFDRMDF8_7,
    #[doc = "0xec8 - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf13_8: CFDRMDF8_8,
    #[doc = "0xecc - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf13_9: CFDRMDF8_9,
    #[doc = "0xed0 - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf13_10: CFDRMDF8_10,
    #[doc = "0xed4 - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf13_11: CFDRMDF8_11,
    #[doc = "0xed8 - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf13_12: CFDRMDF8_12,
    #[doc = "0xedc - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf13_13: CFDRMDF8_13,
    #[doc = "0xee0 - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf13_14: CFDRMDF8_14,
    #[doc = "0xee4 - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf13_15: CFDRMDF8_15,
    #[doc = "0xee8 - RX Message Buffer ID Registers"]
    pub cfdrmid14: CFDRMID8,
    #[doc = "0xeec - RX Message Buffer Pointer Registers"]
    pub cfdrmptr14: CFDRMPTR8,
    #[doc = "0xef0 - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts14: CFDRMFDSTS8,
    #[doc = "0xef4 - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf14_0: CFDRMDF8_0,
    #[doc = "0xef8 - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf14_1: CFDRMDF8_1,
    #[doc = "0xefc - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf14_2: CFDRMDF8_2,
    #[doc = "0xf00 - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf14_3: CFDRMDF8_3,
    #[doc = "0xf04 - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf14_4: CFDRMDF8_4,
    #[doc = "0xf08 - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf14_5: CFDRMDF8_5,
    #[doc = "0xf0c - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf14_6: CFDRMDF8_6,
    #[doc = "0xf10 - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf14_7: CFDRMDF8_7,
    #[doc = "0xf14 - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf14_8: CFDRMDF8_8,
    #[doc = "0xf18 - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf14_9: CFDRMDF8_9,
    #[doc = "0xf1c - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf14_10: CFDRMDF8_10,
    #[doc = "0xf20 - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf14_11: CFDRMDF8_11,
    #[doc = "0xf24 - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf14_12: CFDRMDF8_12,
    #[doc = "0xf28 - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf14_13: CFDRMDF8_13,
    #[doc = "0xf2c - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf14_14: CFDRMDF8_14,
    #[doc = "0xf30 - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf14_15: CFDRMDF8_15,
    #[doc = "0xf34 - RX Message Buffer ID Registers"]
    pub cfdrmid15: CFDRMID8,
    #[doc = "0xf38 - RX Message Buffer Pointer Registers"]
    pub cfdrmptr15: CFDRMPTR8,
    #[doc = "0xf3c - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts15: CFDRMFDSTS8,
    #[doc = "0xf40 - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf15_0: CFDRMDF8_0,
    #[doc = "0xf44 - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf15_1: CFDRMDF8_1,
    #[doc = "0xf48 - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf15_2: CFDRMDF8_2,
    #[doc = "0xf4c - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf15_3: CFDRMDF8_3,
    #[doc = "0xf50 - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf15_4: CFDRMDF8_4,
    #[doc = "0xf54 - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf15_5: CFDRMDF8_5,
    #[doc = "0xf58 - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf15_6: CFDRMDF8_6,
    #[doc = "0xf5c - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf15_7: CFDRMDF8_7,
    #[doc = "0xf60 - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf15_8: CFDRMDF8_8,
    #[doc = "0xf64 - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf15_9: CFDRMDF8_9,
    #[doc = "0xf68 - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf15_10: CFDRMDF8_10,
    #[doc = "0xf6c - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf15_11: CFDRMDF8_11,
    #[doc = "0xf70 - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf15_12: CFDRMDF8_12,
    #[doc = "0xf74 - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf15_13: CFDRMDF8_13,
    #[doc = "0xf78 - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf15_14: CFDRMDF8_14,
    #[doc = "0xf7c - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf15_15: CFDRMDF8_15,
    _reserved541: [u8; 0x01a0],
    #[doc = "0x1120 - RX Message Buffer ID Registers"]
    pub cfdrmid16: CFDRMID16,
    #[doc = "0x1124 - RX Message Buffer Pointer Registers"]
    pub cfdrmptr16: CFDRMPTR16,
    #[doc = "0x1128 - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts16: CFDRMFDSTS16,
    #[doc = "0x112c - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf16_0: CFDRMDF16_0,
    #[doc = "0x1130 - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf16_1: CFDRMDF16_1,
    #[doc = "0x1134 - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf16_2: CFDRMDF16_2,
    #[doc = "0x1138 - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf16_3: CFDRMDF16_3,
    #[doc = "0x113c - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf16_4: CFDRMDF16_4,
    #[doc = "0x1140 - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf16_5: CFDRMDF16_5,
    #[doc = "0x1144 - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf16_6: CFDRMDF16_6,
    #[doc = "0x1148 - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf16_7: CFDRMDF16_7,
    #[doc = "0x114c - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf16_8: CFDRMDF16_8,
    #[doc = "0x1150 - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf16_9: CFDRMDF16_9,
    #[doc = "0x1154 - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf16_10: CFDRMDF16_10,
    #[doc = "0x1158 - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf16_11: CFDRMDF16_11,
    #[doc = "0x115c - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf16_12: CFDRMDF16_12,
    #[doc = "0x1160 - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf16_13: CFDRMDF16_13,
    #[doc = "0x1164 - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf16_14: CFDRMDF16_14,
    #[doc = "0x1168 - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf16_15: CFDRMDF16_15,
    #[doc = "0x116c - RX Message Buffer ID Registers"]
    pub cfdrmid17: CFDRMID16,
    #[doc = "0x1170 - RX Message Buffer Pointer Registers"]
    pub cfdrmptr17: CFDRMPTR16,
    #[doc = "0x1174 - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts17: CFDRMFDSTS16,
    #[doc = "0x1178 - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf17_0: CFDRMDF16_0,
    #[doc = "0x117c - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf17_1: CFDRMDF16_1,
    #[doc = "0x1180 - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf17_2: CFDRMDF16_2,
    #[doc = "0x1184 - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf17_3: CFDRMDF16_3,
    #[doc = "0x1188 - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf17_4: CFDRMDF16_4,
    #[doc = "0x118c - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf17_5: CFDRMDF16_5,
    #[doc = "0x1190 - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf17_6: CFDRMDF16_6,
    #[doc = "0x1194 - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf17_7: CFDRMDF16_7,
    #[doc = "0x1198 - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf17_8: CFDRMDF16_8,
    #[doc = "0x119c - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf17_9: CFDRMDF16_9,
    #[doc = "0x11a0 - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf17_10: CFDRMDF16_10,
    #[doc = "0x11a4 - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf17_11: CFDRMDF16_11,
    #[doc = "0x11a8 - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf17_12: CFDRMDF16_12,
    #[doc = "0x11ac - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf17_13: CFDRMDF16_13,
    #[doc = "0x11b0 - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf17_14: CFDRMDF16_14,
    #[doc = "0x11b4 - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf17_15: CFDRMDF16_15,
    #[doc = "0x11b8 - RX Message Buffer ID Registers"]
    pub cfdrmid18: CFDRMID16,
    #[doc = "0x11bc - RX Message Buffer Pointer Registers"]
    pub cfdrmptr18: CFDRMPTR16,
    #[doc = "0x11c0 - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts18: CFDRMFDSTS16,
    #[doc = "0x11c4 - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf18_0: CFDRMDF16_0,
    #[doc = "0x11c8 - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf18_1: CFDRMDF16_1,
    #[doc = "0x11cc - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf18_2: CFDRMDF16_2,
    #[doc = "0x11d0 - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf18_3: CFDRMDF16_3,
    #[doc = "0x11d4 - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf18_4: CFDRMDF16_4,
    #[doc = "0x11d8 - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf18_5: CFDRMDF16_5,
    #[doc = "0x11dc - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf18_6: CFDRMDF16_6,
    #[doc = "0x11e0 - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf18_7: CFDRMDF16_7,
    #[doc = "0x11e4 - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf18_8: CFDRMDF16_8,
    #[doc = "0x11e8 - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf18_9: CFDRMDF16_9,
    #[doc = "0x11ec - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf18_10: CFDRMDF16_10,
    #[doc = "0x11f0 - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf18_11: CFDRMDF16_11,
    #[doc = "0x11f4 - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf18_12: CFDRMDF16_12,
    #[doc = "0x11f8 - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf18_13: CFDRMDF16_13,
    #[doc = "0x11fc - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf18_14: CFDRMDF16_14,
    #[doc = "0x1200 - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf18_15: CFDRMDF16_15,
    #[doc = "0x1204 - RX Message Buffer ID Registers"]
    pub cfdrmid19: CFDRMID16,
    #[doc = "0x1208 - RX Message Buffer Pointer Registers"]
    pub cfdrmptr19: CFDRMPTR16,
    #[doc = "0x120c - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts19: CFDRMFDSTS16,
    #[doc = "0x1210 - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf19_0: CFDRMDF16_0,
    #[doc = "0x1214 - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf19_1: CFDRMDF16_1,
    #[doc = "0x1218 - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf19_2: CFDRMDF16_2,
    #[doc = "0x121c - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf19_3: CFDRMDF16_3,
    #[doc = "0x1220 - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf19_4: CFDRMDF16_4,
    #[doc = "0x1224 - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf19_5: CFDRMDF16_5,
    #[doc = "0x1228 - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf19_6: CFDRMDF16_6,
    #[doc = "0x122c - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf19_7: CFDRMDF16_7,
    #[doc = "0x1230 - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf19_8: CFDRMDF16_8,
    #[doc = "0x1234 - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf19_9: CFDRMDF16_9,
    #[doc = "0x1238 - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf19_10: CFDRMDF16_10,
    #[doc = "0x123c - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf19_11: CFDRMDF16_11,
    #[doc = "0x1240 - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf19_12: CFDRMDF16_12,
    #[doc = "0x1244 - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf19_13: CFDRMDF16_13,
    #[doc = "0x1248 - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf19_14: CFDRMDF16_14,
    #[doc = "0x124c - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf19_15: CFDRMDF16_15,
    #[doc = "0x1250 - RX Message Buffer ID Registers"]
    pub cfdrmid20: CFDRMID16,
    #[doc = "0x1254 - RX Message Buffer Pointer Registers"]
    pub cfdrmptr20: CFDRMPTR16,
    #[doc = "0x1258 - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts20: CFDRMFDSTS16,
    #[doc = "0x125c - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf20_0: CFDRMDF16_0,
    #[doc = "0x1260 - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf20_1: CFDRMDF16_1,
    #[doc = "0x1264 - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf20_2: CFDRMDF16_2,
    #[doc = "0x1268 - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf20_3: CFDRMDF16_3,
    #[doc = "0x126c - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf20_4: CFDRMDF16_4,
    #[doc = "0x1270 - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf20_5: CFDRMDF16_5,
    #[doc = "0x1274 - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf20_6: CFDRMDF16_6,
    #[doc = "0x1278 - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf20_7: CFDRMDF16_7,
    #[doc = "0x127c - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf20_8: CFDRMDF16_8,
    #[doc = "0x1280 - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf20_9: CFDRMDF16_9,
    #[doc = "0x1284 - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf20_10: CFDRMDF16_10,
    #[doc = "0x1288 - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf20_11: CFDRMDF16_11,
    #[doc = "0x128c - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf20_12: CFDRMDF16_12,
    #[doc = "0x1290 - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf20_13: CFDRMDF16_13,
    #[doc = "0x1294 - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf20_14: CFDRMDF16_14,
    #[doc = "0x1298 - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf20_15: CFDRMDF16_15,
    #[doc = "0x129c - RX Message Buffer ID Registers"]
    pub cfdrmid21: CFDRMID16,
    #[doc = "0x12a0 - RX Message Buffer Pointer Registers"]
    pub cfdrmptr21: CFDRMPTR16,
    #[doc = "0x12a4 - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts21: CFDRMFDSTS16,
    #[doc = "0x12a8 - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf21_0: CFDRMDF16_0,
    #[doc = "0x12ac - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf21_1: CFDRMDF16_1,
    #[doc = "0x12b0 - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf21_2: CFDRMDF16_2,
    #[doc = "0x12b4 - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf21_3: CFDRMDF16_3,
    #[doc = "0x12b8 - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf21_4: CFDRMDF16_4,
    #[doc = "0x12bc - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf21_5: CFDRMDF16_5,
    #[doc = "0x12c0 - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf21_6: CFDRMDF16_6,
    #[doc = "0x12c4 - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf21_7: CFDRMDF16_7,
    #[doc = "0x12c8 - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf21_8: CFDRMDF16_8,
    #[doc = "0x12cc - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf21_9: CFDRMDF16_9,
    #[doc = "0x12d0 - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf21_10: CFDRMDF16_10,
    #[doc = "0x12d4 - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf21_11: CFDRMDF16_11,
    #[doc = "0x12d8 - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf21_12: CFDRMDF16_12,
    #[doc = "0x12dc - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf21_13: CFDRMDF16_13,
    #[doc = "0x12e0 - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf21_14: CFDRMDF16_14,
    #[doc = "0x12e4 - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf21_15: CFDRMDF16_15,
    #[doc = "0x12e8 - RX Message Buffer ID Registers"]
    pub cfdrmid22: CFDRMID16,
    #[doc = "0x12ec - RX Message Buffer Pointer Registers"]
    pub cfdrmptr22: CFDRMPTR16,
    #[doc = "0x12f0 - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts22: CFDRMFDSTS16,
    #[doc = "0x12f4 - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf22_0: CFDRMDF16_0,
    #[doc = "0x12f8 - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf22_1: CFDRMDF16_1,
    #[doc = "0x12fc - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf22_2: CFDRMDF16_2,
    #[doc = "0x1300 - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf22_3: CFDRMDF16_3,
    #[doc = "0x1304 - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf22_4: CFDRMDF16_4,
    #[doc = "0x1308 - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf22_5: CFDRMDF16_5,
    #[doc = "0x130c - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf22_6: CFDRMDF16_6,
    #[doc = "0x1310 - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf22_7: CFDRMDF16_7,
    #[doc = "0x1314 - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf22_8: CFDRMDF16_8,
    #[doc = "0x1318 - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf22_9: CFDRMDF16_9,
    #[doc = "0x131c - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf22_10: CFDRMDF16_10,
    #[doc = "0x1320 - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf22_11: CFDRMDF16_11,
    #[doc = "0x1324 - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf22_12: CFDRMDF16_12,
    #[doc = "0x1328 - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf22_13: CFDRMDF16_13,
    #[doc = "0x132c - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf22_14: CFDRMDF16_14,
    #[doc = "0x1330 - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf22_15: CFDRMDF16_15,
    #[doc = "0x1334 - RX Message Buffer ID Registers"]
    pub cfdrmid23: CFDRMID16,
    #[doc = "0x1338 - RX Message Buffer Pointer Registers"]
    pub cfdrmptr23: CFDRMPTR16,
    #[doc = "0x133c - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts23: CFDRMFDSTS16,
    #[doc = "0x1340 - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf23_0: CFDRMDF16_0,
    #[doc = "0x1344 - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf23_1: CFDRMDF16_1,
    #[doc = "0x1348 - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf23_2: CFDRMDF16_2,
    #[doc = "0x134c - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf23_3: CFDRMDF16_3,
    #[doc = "0x1350 - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf23_4: CFDRMDF16_4,
    #[doc = "0x1354 - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf23_5: CFDRMDF16_5,
    #[doc = "0x1358 - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf23_6: CFDRMDF16_6,
    #[doc = "0x135c - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf23_7: CFDRMDF16_7,
    #[doc = "0x1360 - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf23_8: CFDRMDF16_8,
    #[doc = "0x1364 - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf23_9: CFDRMDF16_9,
    #[doc = "0x1368 - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf23_10: CFDRMDF16_10,
    #[doc = "0x136c - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf23_11: CFDRMDF16_11,
    #[doc = "0x1370 - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf23_12: CFDRMDF16_12,
    #[doc = "0x1374 - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf23_13: CFDRMDF16_13,
    #[doc = "0x1378 - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf23_14: CFDRMDF16_14,
    #[doc = "0x137c - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf23_15: CFDRMDF16_15,
    _reserved693: [u8; 0x01a0],
    #[doc = "0x1520 - RX Message Buffer ID Registers"]
    pub cfdrmid24: CFDRMID24,
    #[doc = "0x1524 - RX Message Buffer Pointer Registers"]
    pub cfdrmptr24: CFDRMPTR24,
    #[doc = "0x1528 - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts24: CFDRMFDSTS24,
    #[doc = "0x152c - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf24_0: CFDRMDF24_0,
    #[doc = "0x1530 - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf24_1: CFDRMDF24_1,
    #[doc = "0x1534 - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf24_2: CFDRMDF24_2,
    #[doc = "0x1538 - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf24_3: CFDRMDF24_3,
    #[doc = "0x153c - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf24_4: CFDRMDF24_4,
    #[doc = "0x1540 - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf24_5: CFDRMDF24_5,
    #[doc = "0x1544 - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf24_6: CFDRMDF24_6,
    #[doc = "0x1548 - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf24_7: CFDRMDF24_7,
    #[doc = "0x154c - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf24_8: CFDRMDF24_8,
    #[doc = "0x1550 - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf24_9: CFDRMDF24_9,
    #[doc = "0x1554 - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf24_10: CFDRMDF24_10,
    #[doc = "0x1558 - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf24_11: CFDRMDF24_11,
    #[doc = "0x155c - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf24_12: CFDRMDF24_12,
    #[doc = "0x1560 - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf24_13: CFDRMDF24_13,
    #[doc = "0x1564 - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf24_14: CFDRMDF24_14,
    #[doc = "0x1568 - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf24_15: CFDRMDF24_15,
    #[doc = "0x156c - RX Message Buffer ID Registers"]
    pub cfdrmid25: CFDRMID24,
    #[doc = "0x1570 - RX Message Buffer Pointer Registers"]
    pub cfdrmptr25: CFDRMPTR24,
    #[doc = "0x1574 - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts25: CFDRMFDSTS24,
    #[doc = "0x1578 - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf25_0: CFDRMDF24_0,
    #[doc = "0x157c - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf25_1: CFDRMDF24_1,
    #[doc = "0x1580 - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf25_2: CFDRMDF24_2,
    #[doc = "0x1584 - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf25_3: CFDRMDF24_3,
    #[doc = "0x1588 - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf25_4: CFDRMDF24_4,
    #[doc = "0x158c - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf25_5: CFDRMDF24_5,
    #[doc = "0x1590 - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf25_6: CFDRMDF24_6,
    #[doc = "0x1594 - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf25_7: CFDRMDF24_7,
    #[doc = "0x1598 - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf25_8: CFDRMDF24_8,
    #[doc = "0x159c - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf25_9: CFDRMDF24_9,
    #[doc = "0x15a0 - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf25_10: CFDRMDF24_10,
    #[doc = "0x15a4 - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf25_11: CFDRMDF24_11,
    #[doc = "0x15a8 - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf25_12: CFDRMDF24_12,
    #[doc = "0x15ac - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf25_13: CFDRMDF24_13,
    #[doc = "0x15b0 - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf25_14: CFDRMDF24_14,
    #[doc = "0x15b4 - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf25_15: CFDRMDF24_15,
    #[doc = "0x15b8 - RX Message Buffer ID Registers"]
    pub cfdrmid26: CFDRMID24,
    #[doc = "0x15bc - RX Message Buffer Pointer Registers"]
    pub cfdrmptr26: CFDRMPTR24,
    #[doc = "0x15c0 - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts26: CFDRMFDSTS24,
    #[doc = "0x15c4 - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf26_0: CFDRMDF24_0,
    #[doc = "0x15c8 - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf26_1: CFDRMDF24_1,
    #[doc = "0x15cc - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf26_2: CFDRMDF24_2,
    #[doc = "0x15d0 - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf26_3: CFDRMDF24_3,
    #[doc = "0x15d4 - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf26_4: CFDRMDF24_4,
    #[doc = "0x15d8 - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf26_5: CFDRMDF24_5,
    #[doc = "0x15dc - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf26_6: CFDRMDF24_6,
    #[doc = "0x15e0 - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf26_7: CFDRMDF24_7,
    #[doc = "0x15e4 - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf26_8: CFDRMDF24_8,
    #[doc = "0x15e8 - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf26_9: CFDRMDF24_9,
    #[doc = "0x15ec - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf26_10: CFDRMDF24_10,
    #[doc = "0x15f0 - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf26_11: CFDRMDF24_11,
    #[doc = "0x15f4 - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf26_12: CFDRMDF24_12,
    #[doc = "0x15f8 - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf26_13: CFDRMDF24_13,
    #[doc = "0x15fc - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf26_14: CFDRMDF24_14,
    #[doc = "0x1600 - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf26_15: CFDRMDF24_15,
    #[doc = "0x1604 - RX Message Buffer ID Registers"]
    pub cfdrmid27: CFDRMID24,
    #[doc = "0x1608 - RX Message Buffer Pointer Registers"]
    pub cfdrmptr27: CFDRMPTR24,
    #[doc = "0x160c - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts27: CFDRMFDSTS24,
    #[doc = "0x1610 - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf27_0: CFDRMDF24_0,
    #[doc = "0x1614 - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf27_1: CFDRMDF24_1,
    #[doc = "0x1618 - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf27_2: CFDRMDF24_2,
    #[doc = "0x161c - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf27_3: CFDRMDF24_3,
    #[doc = "0x1620 - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf27_4: CFDRMDF24_4,
    #[doc = "0x1624 - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf27_5: CFDRMDF24_5,
    #[doc = "0x1628 - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf27_6: CFDRMDF24_6,
    #[doc = "0x162c - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf27_7: CFDRMDF24_7,
    #[doc = "0x1630 - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf27_8: CFDRMDF24_8,
    #[doc = "0x1634 - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf27_9: CFDRMDF24_9,
    #[doc = "0x1638 - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf27_10: CFDRMDF24_10,
    #[doc = "0x163c - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf27_11: CFDRMDF24_11,
    #[doc = "0x1640 - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf27_12: CFDRMDF24_12,
    #[doc = "0x1644 - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf27_13: CFDRMDF24_13,
    #[doc = "0x1648 - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf27_14: CFDRMDF24_14,
    #[doc = "0x164c - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf27_15: CFDRMDF24_15,
    #[doc = "0x1650 - RX Message Buffer ID Registers"]
    pub cfdrmid28: CFDRMID24,
    #[doc = "0x1654 - RX Message Buffer Pointer Registers"]
    pub cfdrmptr28: CFDRMPTR24,
    #[doc = "0x1658 - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts28: CFDRMFDSTS24,
    #[doc = "0x165c - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf28_0: CFDRMDF24_0,
    #[doc = "0x1660 - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf28_1: CFDRMDF24_1,
    #[doc = "0x1664 - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf28_2: CFDRMDF24_2,
    #[doc = "0x1668 - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf28_3: CFDRMDF24_3,
    #[doc = "0x166c - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf28_4: CFDRMDF24_4,
    #[doc = "0x1670 - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf28_5: CFDRMDF24_5,
    #[doc = "0x1674 - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf28_6: CFDRMDF24_6,
    #[doc = "0x1678 - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf28_7: CFDRMDF24_7,
    #[doc = "0x167c - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf28_8: CFDRMDF24_8,
    #[doc = "0x1680 - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf28_9: CFDRMDF24_9,
    #[doc = "0x1684 - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf28_10: CFDRMDF24_10,
    #[doc = "0x1688 - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf28_11: CFDRMDF24_11,
    #[doc = "0x168c - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf28_12: CFDRMDF24_12,
    #[doc = "0x1690 - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf28_13: CFDRMDF24_13,
    #[doc = "0x1694 - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf28_14: CFDRMDF24_14,
    #[doc = "0x1698 - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf28_15: CFDRMDF24_15,
    #[doc = "0x169c - RX Message Buffer ID Registers"]
    pub cfdrmid29: CFDRMID24,
    #[doc = "0x16a0 - RX Message Buffer Pointer Registers"]
    pub cfdrmptr29: CFDRMPTR24,
    #[doc = "0x16a4 - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts29: CFDRMFDSTS24,
    #[doc = "0x16a8 - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf29_0: CFDRMDF24_0,
    #[doc = "0x16ac - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf29_1: CFDRMDF24_1,
    #[doc = "0x16b0 - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf29_2: CFDRMDF24_2,
    #[doc = "0x16b4 - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf29_3: CFDRMDF24_3,
    #[doc = "0x16b8 - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf29_4: CFDRMDF24_4,
    #[doc = "0x16bc - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf29_5: CFDRMDF24_5,
    #[doc = "0x16c0 - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf29_6: CFDRMDF24_6,
    #[doc = "0x16c4 - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf29_7: CFDRMDF24_7,
    #[doc = "0x16c8 - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf29_8: CFDRMDF24_8,
    #[doc = "0x16cc - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf29_9: CFDRMDF24_9,
    #[doc = "0x16d0 - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf29_10: CFDRMDF24_10,
    #[doc = "0x16d4 - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf29_11: CFDRMDF24_11,
    #[doc = "0x16d8 - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf29_12: CFDRMDF24_12,
    #[doc = "0x16dc - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf29_13: CFDRMDF24_13,
    #[doc = "0x16e0 - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf29_14: CFDRMDF24_14,
    #[doc = "0x16e4 - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf29_15: CFDRMDF24_15,
    #[doc = "0x16e8 - RX Message Buffer ID Registers"]
    pub cfdrmid30: CFDRMID24,
    #[doc = "0x16ec - RX Message Buffer Pointer Registers"]
    pub cfdrmptr30: CFDRMPTR24,
    #[doc = "0x16f0 - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts30: CFDRMFDSTS24,
    #[doc = "0x16f4 - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf30_0: CFDRMDF24_0,
    #[doc = "0x16f8 - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf30_1: CFDRMDF24_1,
    #[doc = "0x16fc - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf30_2: CFDRMDF24_2,
    #[doc = "0x1700 - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf30_3: CFDRMDF24_3,
    #[doc = "0x1704 - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf30_4: CFDRMDF24_4,
    #[doc = "0x1708 - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf30_5: CFDRMDF24_5,
    #[doc = "0x170c - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf30_6: CFDRMDF24_6,
    #[doc = "0x1710 - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf30_7: CFDRMDF24_7,
    #[doc = "0x1714 - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf30_8: CFDRMDF24_8,
    #[doc = "0x1718 - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf30_9: CFDRMDF24_9,
    #[doc = "0x171c - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf30_10: CFDRMDF24_10,
    #[doc = "0x1720 - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf30_11: CFDRMDF24_11,
    #[doc = "0x1724 - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf30_12: CFDRMDF24_12,
    #[doc = "0x1728 - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf30_13: CFDRMDF24_13,
    #[doc = "0x172c - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf30_14: CFDRMDF24_14,
    #[doc = "0x1730 - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf30_15: CFDRMDF24_15,
    #[doc = "0x1734 - RX Message Buffer ID Registers"]
    pub cfdrmid31: CFDRMID24,
    #[doc = "0x1738 - RX Message Buffer Pointer Registers"]
    pub cfdrmptr31: CFDRMPTR24,
    #[doc = "0x173c - RX Message Buffer CANFD Status Registers"]
    pub cfdrmfdsts31: CFDRMFDSTS24,
    #[doc = "0x1740 - RX Message Buffer Data Field 0 Registers"]
    pub cfdrmdf31_0: CFDRMDF24_0,
    #[doc = "0x1744 - RX Message Buffer Data Field 1 Registers"]
    pub cfdrmdf31_1: CFDRMDF24_1,
    #[doc = "0x1748 - RX Message Buffer Data Field 2 Registers"]
    pub cfdrmdf31_2: CFDRMDF24_2,
    #[doc = "0x174c - RX Message Buffer Data Field 3 Registers"]
    pub cfdrmdf31_3: CFDRMDF24_3,
    #[doc = "0x1750 - RX Message Buffer Data Field 4 Registers"]
    pub cfdrmdf31_4: CFDRMDF24_4,
    #[doc = "0x1754 - RX Message Buffer Data Field 5 Registers"]
    pub cfdrmdf31_5: CFDRMDF24_5,
    #[doc = "0x1758 - RX Message Buffer Data Field 6 Registers"]
    pub cfdrmdf31_6: CFDRMDF24_6,
    #[doc = "0x175c - RX Message Buffer Data Field 7 Registers"]
    pub cfdrmdf31_7: CFDRMDF24_7,
    #[doc = "0x1760 - RX Message Buffer Data Field 8 Registers"]
    pub cfdrmdf31_8: CFDRMDF24_8,
    #[doc = "0x1764 - RX Message Buffer Data Field 9 Registers"]
    pub cfdrmdf31_9: CFDRMDF24_9,
    #[doc = "0x1768 - RX Message Buffer Data Field 10 Registers"]
    pub cfdrmdf31_10: CFDRMDF24_10,
    #[doc = "0x176c - RX Message Buffer Data Field 11 Registers"]
    pub cfdrmdf31_11: CFDRMDF24_11,
    #[doc = "0x1770 - RX Message Buffer Data Field 12 Registers"]
    pub cfdrmdf31_12: CFDRMDF24_12,
    #[doc = "0x1774 - RX Message Buffer Data Field 13 Registers"]
    pub cfdrmdf31_13: CFDRMDF24_13,
    #[doc = "0x1778 - RX Message Buffer Data Field 14 Registers"]
    pub cfdrmdf31_14: CFDRMDF24_14,
    #[doc = "0x177c - RX Message Buffer Data Field 15 Registers"]
    pub cfdrmdf31_15: CFDRMDF24_15,
}
#[doc = "CFDC0NCFG (rw) register accessor: an alias for `Reg<CFDC0NCFG_SPEC>`"]
pub type CFDC0NCFG = crate::Reg<cfdc0ncfg::CFDC0NCFG_SPEC>;
#[doc = "Channel 0 Nominal Bitrate Configuration Register"]
pub mod cfdc0ncfg;
#[doc = "CFDC0CTR (rw) register accessor: an alias for `Reg<CFDC0CTR_SPEC>`"]
pub type CFDC0CTR = crate::Reg<cfdc0ctr::CFDC0CTR_SPEC>;
#[doc = "Channel 0 Control Register"]
pub mod cfdc0ctr;
#[doc = "CFDC0STS (rw) register accessor: an alias for `Reg<CFDC0STS_SPEC>`"]
pub type CFDC0STS = crate::Reg<cfdc0sts::CFDC0STS_SPEC>;
#[doc = "Channel 0 Status Register"]
pub mod cfdc0sts;
#[doc = "CFDC0ERFL (rw) register accessor: an alias for `Reg<CFDC0ERFL_SPEC>`"]
pub type CFDC0ERFL = crate::Reg<cfdc0erfl::CFDC0ERFL_SPEC>;
#[doc = "Channel 0 Error Flag Register"]
pub mod cfdc0erfl;
#[doc = "CFDGCFG (rw) register accessor: an alias for `Reg<CFDGCFG_SPEC>`"]
pub type CFDGCFG = crate::Reg<cfdgcfg::CFDGCFG_SPEC>;
#[doc = "Global Configuration Register"]
pub mod cfdgcfg;
#[doc = "CFDGCTR (rw) register accessor: an alias for `Reg<CFDGCTR_SPEC>`"]
pub type CFDGCTR = crate::Reg<cfdgctr::CFDGCTR_SPEC>;
#[doc = "Global Control Register"]
pub mod cfdgctr;
#[doc = "CFDGSTS (r) register accessor: an alias for `Reg<CFDGSTS_SPEC>`"]
pub type CFDGSTS = crate::Reg<cfdgsts::CFDGSTS_SPEC>;
#[doc = "Global Status Register"]
pub mod cfdgsts;
#[doc = "CFDGERFL (rw) register accessor: an alias for `Reg<CFDGERFL_SPEC>`"]
pub type CFDGERFL = crate::Reg<cfdgerfl::CFDGERFL_SPEC>;
#[doc = "Global Error Flag Register"]
pub mod cfdgerfl;
#[doc = "CFDGTSC (r) register accessor: an alias for `Reg<CFDGTSC_SPEC>`"]
pub type CFDGTSC = crate::Reg<cfdgtsc::CFDGTSC_SPEC>;
#[doc = "Global Timestamp Counter Register"]
pub mod cfdgtsc;
#[doc = "CFDGAFLECTR (rw) register accessor: an alias for `Reg<CFDGAFLECTR_SPEC>`"]
pub type CFDGAFLECTR = crate::Reg<cfdgaflectr::CFDGAFLECTR_SPEC>;
#[doc = "Global Acceptance Filter List Entry Control Register"]
pub mod cfdgaflectr;
#[doc = "CFDGAFLCFG (rw) register accessor: an alias for `Reg<CFDGAFLCFG_SPEC>`"]
pub type CFDGAFLCFG = crate::Reg<cfdgaflcfg::CFDGAFLCFG_SPEC>;
#[doc = "Global Acceptance Filter List Configuration Register"]
pub mod cfdgaflcfg;
#[doc = "CFDRMNB (rw) register accessor: an alias for `Reg<CFDRMNB_SPEC>`"]
pub type CFDRMNB = crate::Reg<cfdrmnb::CFDRMNB_SPEC>;
#[doc = "RX Message Buffer Number Register"]
pub mod cfdrmnb;
#[doc = "CFDRMND (rw) register accessor: an alias for `Reg<CFDRMND_SPEC>`"]
pub type CFDRMND = crate::Reg<cfdrmnd::CFDRMND_SPEC>;
#[doc = "RX Message Buffer New Data Register"]
pub mod cfdrmnd;
#[doc = "CFDRMIEC (rw) register accessor: an alias for `Reg<CFDRMIEC_SPEC>`"]
pub type CFDRMIEC = crate::Reg<cfdrmiec::CFDRMIEC_SPEC>;
#[doc = "RX Message Buffer Interrupt Enable Configuration Register"]
pub mod cfdrmiec;
#[doc = "CFDRFCC (rw) register accessor: an alias for `Reg<CFDRFCC_SPEC>`"]
pub type CFDRFCC = crate::Reg<cfdrfcc::CFDRFCC_SPEC>;
#[doc = "RX FIFO Configuration/Control Registers %s"]
pub mod cfdrfcc;
#[doc = "CFDRFSTS (rw) register accessor: an alias for `Reg<CFDRFSTS_SPEC>`"]
pub type CFDRFSTS = crate::Reg<cfdrfsts::CFDRFSTS_SPEC>;
#[doc = "RX FIFO Status Registers %s"]
pub mod cfdrfsts;
#[doc = "CFDRFPCTR (w) register accessor: an alias for `Reg<CFDRFPCTR_SPEC>`"]
pub type CFDRFPCTR = crate::Reg<cfdrfpctr::CFDRFPCTR_SPEC>;
#[doc = "RX FIFO Pointer Control Registers %s"]
pub mod cfdrfpctr;
#[doc = "CFDCFCC (rw) register accessor: an alias for `Reg<CFDCFCC_SPEC>`"]
pub type CFDCFCC = crate::Reg<cfdcfcc::CFDCFCC_SPEC>;
#[doc = "Common FIFO Configuration/Control Register"]
pub mod cfdcfcc;
#[doc = "CFDCFSTS (rw) register accessor: an alias for `Reg<CFDCFSTS_SPEC>`"]
pub type CFDCFSTS = crate::Reg<cfdcfsts::CFDCFSTS_SPEC>;
#[doc = "Common FIFO Status Register"]
pub mod cfdcfsts;
#[doc = "CFDCFPCTR (w) register accessor: an alias for `Reg<CFDCFPCTR_SPEC>`"]
pub type CFDCFPCTR = crate::Reg<cfdcfpctr::CFDCFPCTR_SPEC>;
#[doc = "Common FIFO Pointer Control Register"]
pub mod cfdcfpctr;
#[doc = "CFDFESTS (r) register accessor: an alias for `Reg<CFDFESTS_SPEC>`"]
pub type CFDFESTS = crate::Reg<cfdfests::CFDFESTS_SPEC>;
#[doc = "FIFO Empty Status Register"]
pub mod cfdfests;
#[doc = "CFDFFSTS (r) register accessor: an alias for `Reg<CFDFFSTS_SPEC>`"]
pub type CFDFFSTS = crate::Reg<cfdffsts::CFDFFSTS_SPEC>;
#[doc = "FIFO Full Status Register"]
pub mod cfdffsts;
#[doc = "CFDFMSTS (r) register accessor: an alias for `Reg<CFDFMSTS_SPEC>`"]
pub type CFDFMSTS = crate::Reg<cfdfmsts::CFDFMSTS_SPEC>;
#[doc = "FIFO Message Lost Status Register"]
pub mod cfdfmsts;
#[doc = "CFDRFISTS (r) register accessor: an alias for `Reg<CFDRFISTS_SPEC>`"]
pub type CFDRFISTS = crate::Reg<cfdrfists::CFDRFISTS_SPEC>;
#[doc = "RX FIFO Interrupt Flag Status Register"]
pub mod cfdrfists;
#[doc = "CFDTMC (rw) register accessor: an alias for `Reg<CFDTMC_SPEC>`"]
pub type CFDTMC = crate::Reg<cfdtmc::CFDTMC_SPEC>;
#[doc = "TX Message Buffer Control Registers %s"]
pub mod cfdtmc;
#[doc = "CFDTMSTS (rw) register accessor: an alias for `Reg<CFDTMSTS_SPEC>`"]
pub type CFDTMSTS = crate::Reg<cfdtmsts::CFDTMSTS_SPEC>;
#[doc = "TX Message Buffer Status Registers %s"]
pub mod cfdtmsts;
#[doc = "CFDTMTRSTS (r) register accessor: an alias for `Reg<CFDTMTRSTS_SPEC>`"]
pub type CFDTMTRSTS = crate::Reg<cfdtmtrsts::CFDTMTRSTS_SPEC>;
#[doc = "TX Message Buffer Transmission Request Status Register"]
pub mod cfdtmtrsts;
#[doc = "CFDTMTARSTS (r) register accessor: an alias for `Reg<CFDTMTARSTS_SPEC>`"]
pub type CFDTMTARSTS = crate::Reg<cfdtmtarsts::CFDTMTARSTS_SPEC>;
#[doc = "TX Message Buffer Transmission Abort Request Status Register"]
pub mod cfdtmtarsts;
#[doc = "CFDTMTCSTS (r) register accessor: an alias for `Reg<CFDTMTCSTS_SPEC>`"]
pub type CFDTMTCSTS = crate::Reg<cfdtmtcsts::CFDTMTCSTS_SPEC>;
#[doc = "TX Message Buffer Transmission Completion Status Register"]
pub mod cfdtmtcsts;
#[doc = "CFDTMTASTS (r) register accessor: an alias for `Reg<CFDTMTASTS_SPEC>`"]
pub type CFDTMTASTS = crate::Reg<cfdtmtasts::CFDTMTASTS_SPEC>;
#[doc = "TX Message Buffer Transmission Abort Status Register"]
pub mod cfdtmtasts;
#[doc = "CFDTMIEC (rw) register accessor: an alias for `Reg<CFDTMIEC_SPEC>`"]
pub type CFDTMIEC = crate::Reg<cfdtmiec::CFDTMIEC_SPEC>;
#[doc = "TX Message Buffer Interrupt Enable Configuration Register"]
pub mod cfdtmiec;
#[doc = "CFDTXQCC (rw) register accessor: an alias for `Reg<CFDTXQCC_SPEC>`"]
pub type CFDTXQCC = crate::Reg<cfdtxqcc::CFDTXQCC_SPEC>;
#[doc = "TX Queue Configuration/Control Register"]
pub mod cfdtxqcc;
#[doc = "CFDTXQSTS (rw) register accessor: an alias for `Reg<CFDTXQSTS_SPEC>`"]
pub type CFDTXQSTS = crate::Reg<cfdtxqsts::CFDTXQSTS_SPEC>;
#[doc = "TX Queue Status Register"]
pub mod cfdtxqsts;
#[doc = "CFDTXQPCTR (w) register accessor: an alias for `Reg<CFDTXQPCTR_SPEC>`"]
pub type CFDTXQPCTR = crate::Reg<cfdtxqpctr::CFDTXQPCTR_SPEC>;
#[doc = "TX Queue Pointer Control Register"]
pub mod cfdtxqpctr;
#[doc = "CFDTHLCC (rw) register accessor: an alias for `Reg<CFDTHLCC_SPEC>`"]
pub type CFDTHLCC = crate::Reg<cfdthlcc::CFDTHLCC_SPEC>;
#[doc = "TX History List Configuration/Control Register"]
pub mod cfdthlcc;
#[doc = "CFDTHLSTS (rw) register accessor: an alias for `Reg<CFDTHLSTS_SPEC>`"]
pub type CFDTHLSTS = crate::Reg<cfdthlsts::CFDTHLSTS_SPEC>;
#[doc = "TX History List Status Register"]
pub mod cfdthlsts;
#[doc = "CFDTHLPCTR (w) register accessor: an alias for `Reg<CFDTHLPCTR_SPEC>`"]
pub type CFDTHLPCTR = crate::Reg<cfdthlpctr::CFDTHLPCTR_SPEC>;
#[doc = "TX History List Pointer Control Register"]
pub mod cfdthlpctr;
#[doc = "CFDGTINTSTS (r) register accessor: an alias for `Reg<CFDGTINTSTS_SPEC>`"]
pub type CFDGTINTSTS = crate::Reg<cfdgtintsts::CFDGTINTSTS_SPEC>;
#[doc = "Global TX Interrupt Status Register"]
pub mod cfdgtintsts;
#[doc = "CFDGTSTCFG (rw) register accessor: an alias for `Reg<CFDGTSTCFG_SPEC>`"]
pub type CFDGTSTCFG = crate::Reg<cfdgtstcfg::CFDGTSTCFG_SPEC>;
#[doc = "Global Test Configuration Register"]
pub mod cfdgtstcfg;
#[doc = "CFDGTSTCTR (rw) register accessor: an alias for `Reg<CFDGTSTCTR_SPEC>`"]
pub type CFDGTSTCTR = crate::Reg<cfdgtstctr::CFDGTSTCTR_SPEC>;
#[doc = "Global Test Control Register"]
pub mod cfdgtstctr;
#[doc = "CFDGFDCFG (rw) register accessor: an alias for `Reg<CFDGFDCFG_SPEC>`"]
pub type CFDGFDCFG = crate::Reg<cfdgfdcfg::CFDGFDCFG_SPEC>;
#[doc = "Global FD Configuration Register"]
pub mod cfdgfdcfg;
#[doc = "CFDGLOCKK (w) register accessor: an alias for `Reg<CFDGLOCKK_SPEC>`"]
pub type CFDGLOCKK = crate::Reg<cfdglockk::CFDGLOCKK_SPEC>;
#[doc = "Global Lock Key Register"]
pub mod cfdglockk;
#[doc = "CFDGAFLIGNENT (rw) register accessor: an alias for `Reg<CFDGAFLIGNENT_SPEC>`"]
pub type CFDGAFLIGNENT = crate::Reg<cfdgaflignent::CFDGAFLIGNENT_SPEC>;
#[doc = "Global AFL Ignore Entry Register"]
pub mod cfdgaflignent;
#[doc = "CFDGAFLIGNCTR (rw) register accessor: an alias for `Reg<CFDGAFLIGNCTR_SPEC>`"]
pub type CFDGAFLIGNCTR = crate::Reg<cfdgaflignctr::CFDGAFLIGNCTR_SPEC>;
#[doc = "Global AFL Ignore Control Register"]
pub mod cfdgaflignctr;
#[doc = "CFDCDTCT (rw) register accessor: an alias for `Reg<CFDCDTCT_SPEC>`"]
pub type CFDCDTCT = crate::Reg<cfdcdtct::CFDCDTCT_SPEC>;
#[doc = "DMA Transfer Control Register"]
pub mod cfdcdtct;
#[doc = "CFDCDTSTS (r) register accessor: an alias for `Reg<CFDCDTSTS_SPEC>`"]
pub type CFDCDTSTS = crate::Reg<cfdcdtsts::CFDCDTSTS_SPEC>;
#[doc = "DMA Transfer Status Register"]
pub mod cfdcdtsts;
#[doc = "CFDGRSTC (rw) register accessor: an alias for `Reg<CFDGRSTC_SPEC>`"]
pub type CFDGRSTC = crate::Reg<cfdgrstc::CFDGRSTC_SPEC>;
#[doc = "Global SW reset Register"]
pub mod cfdgrstc;
#[doc = "CFDC0DCFG (rw) register accessor: an alias for `Reg<CFDC0DCFG_SPEC>`"]
pub type CFDC0DCFG = crate::Reg<cfdc0dcfg::CFDC0DCFG_SPEC>;
#[doc = "Channel 0 Data Bitrate Configuration Register"]
pub mod cfdc0dcfg;
#[doc = "CFDC0FDCFG (rw) register accessor: an alias for `Reg<CFDC0FDCFG_SPEC>`"]
pub type CFDC0FDCFG = crate::Reg<cfdc0fdcfg::CFDC0FDCFG_SPEC>;
#[doc = "Channel 0 CANFD Configuration Register"]
pub mod cfdc0fdcfg;
#[doc = "CFDC0FDCTR (rw) register accessor: an alias for `Reg<CFDC0FDCTR_SPEC>`"]
pub type CFDC0FDCTR = crate::Reg<cfdc0fdctr::CFDC0FDCTR_SPEC>;
#[doc = "Channel 0 CANFD Control Register"]
pub mod cfdc0fdctr;
#[doc = "CFDC0FDSTS (rw) register accessor: an alias for `Reg<CFDC0FDSTS_SPEC>`"]
pub type CFDC0FDSTS = crate::Reg<cfdc0fdsts::CFDC0FDSTS_SPEC>;
#[doc = "Channel 0 CANFD Status Register"]
pub mod cfdc0fdsts;
#[doc = "CFDC0FDCRC (rw) register accessor: an alias for `Reg<CFDC0FDCRC_SPEC>`"]
pub type CFDC0FDCRC = crate::Reg<cfdc0fdcrc::CFDC0FDCRC_SPEC>;
#[doc = "Channel 0 CANFD CRC Register"]
pub mod cfdc0fdcrc;
#[doc = "CFDGAFLID (rw) register accessor: an alias for `Reg<CFDGAFLID_SPEC>`"]
pub type CFDGAFLID = crate::Reg<cfdgaflid::CFDGAFLID_SPEC>;
#[doc = "Global Acceptance Filter List ID Registers"]
pub mod cfdgaflid;
#[doc = "CFDGAFLM (rw) register accessor: an alias for `Reg<CFDGAFLM_SPEC>`"]
pub type CFDGAFLM = crate::Reg<cfdgaflm::CFDGAFLM_SPEC>;
#[doc = "Global Acceptance Filter List Mask Registers"]
pub mod cfdgaflm;
#[doc = "CFDGAFLP0 (rw) register accessor: an alias for `Reg<CFDGAFLP0_SPEC>`"]
pub type CFDGAFLP0 = crate::Reg<cfdgaflp0::CFDGAFLP0_SPEC>;
#[doc = "Global Acceptance Filter List Pointer 0 Registers"]
pub mod cfdgaflp0;
#[doc = "CFDGAFLP1 (rw) register accessor: an alias for `Reg<CFDGAFLP1_SPEC>`"]
pub type CFDGAFLP1 = crate::Reg<cfdgaflp1::CFDGAFLP1_SPEC>;
#[doc = "Global Acceptance Filter List Pointer 1 Registers"]
pub mod cfdgaflp1;
#[doc = "CFDRPGACC (rw) register accessor: an alias for `Reg<CFDRPGACC_SPEC>`"]
pub type CFDRPGACC = crate::Reg<cfdrpgacc::CFDRPGACC_SPEC>;
#[doc = "RAM Test Page Access Registers %s"]
pub mod cfdrpgacc;
#[doc = "CFDRFID (r) register accessor: an alias for `Reg<CFDRFID_SPEC>`"]
pub type CFDRFID = crate::Reg<cfdrfid::CFDRFID_SPEC>;
#[doc = "RX FIFO Access ID Register %s"]
pub mod cfdrfid;
#[doc = "CFDRFPTR (r) register accessor: an alias for `Reg<CFDRFPTR_SPEC>`"]
pub type CFDRFPTR = crate::Reg<cfdrfptr::CFDRFPTR_SPEC>;
#[doc = "RX FIFO Access Pointer Register %s"]
pub mod cfdrfptr;
#[doc = "CFDRFFDSTS (r) register accessor: an alias for `Reg<CFDRFFDSTS_SPEC>`"]
pub type CFDRFFDSTS = crate::Reg<cfdrffdsts::CFDRFFDSTS_SPEC>;
#[doc = "RX FIFO Access CANFD Status Register %s"]
pub mod cfdrffdsts;
#[doc = "CFDRFDF_0 (r) register accessor: an alias for `Reg<CFDRFDF_0_SPEC>`"]
pub type CFDRFDF_0 = crate::Reg<cfdrfdf_0::CFDRFDF_0_SPEC>;
#[doc = "RX FIFO Access Data Field 0 Register %s"]
pub mod cfdrfdf_0;
#[doc = "CFDRFDF_1 (r) register accessor: an alias for `Reg<CFDRFDF_1_SPEC>`"]
pub type CFDRFDF_1 = crate::Reg<cfdrfdf_1::CFDRFDF_1_SPEC>;
#[doc = "RX FIFO Access Data Field 1 Register %s"]
pub mod cfdrfdf_1;
#[doc = "CFDRFDF_2 (r) register accessor: an alias for `Reg<CFDRFDF_2_SPEC>`"]
pub type CFDRFDF_2 = crate::Reg<cfdrfdf_2::CFDRFDF_2_SPEC>;
#[doc = "RX FIFO Access Data Field 2 Register %s"]
pub mod cfdrfdf_2;
#[doc = "CFDRFDF_3 (r) register accessor: an alias for `Reg<CFDRFDF_3_SPEC>`"]
pub type CFDRFDF_3 = crate::Reg<cfdrfdf_3::CFDRFDF_3_SPEC>;
#[doc = "RX FIFO Access Data Field 3 Register %s"]
pub mod cfdrfdf_3;
#[doc = "CFDRFDF_4 (r) register accessor: an alias for `Reg<CFDRFDF_4_SPEC>`"]
pub type CFDRFDF_4 = crate::Reg<cfdrfdf_4::CFDRFDF_4_SPEC>;
#[doc = "RX FIFO Access Data Field 4 Register %s"]
pub mod cfdrfdf_4;
#[doc = "CFDRFDF_5 (r) register accessor: an alias for `Reg<CFDRFDF_5_SPEC>`"]
pub type CFDRFDF_5 = crate::Reg<cfdrfdf_5::CFDRFDF_5_SPEC>;
#[doc = "RX FIFO Access Data Field 5 Register %s"]
pub mod cfdrfdf_5;
#[doc = "CFDRFDF_6 (r) register accessor: an alias for `Reg<CFDRFDF_6_SPEC>`"]
pub type CFDRFDF_6 = crate::Reg<cfdrfdf_6::CFDRFDF_6_SPEC>;
#[doc = "RX FIFO Access Data Field 6 Register %s"]
pub mod cfdrfdf_6;
#[doc = "CFDRFDF_7 (r) register accessor: an alias for `Reg<CFDRFDF_7_SPEC>`"]
pub type CFDRFDF_7 = crate::Reg<cfdrfdf_7::CFDRFDF_7_SPEC>;
#[doc = "RX FIFO Access Data Field 7 Register %s"]
pub mod cfdrfdf_7;
#[doc = "CFDRFDF_8 (r) register accessor: an alias for `Reg<CFDRFDF_8_SPEC>`"]
pub type CFDRFDF_8 = crate::Reg<cfdrfdf_8::CFDRFDF_8_SPEC>;
#[doc = "RX FIFO Access Data Field 8 Register %s"]
pub mod cfdrfdf_8;
#[doc = "CFDRFDF_9 (r) register accessor: an alias for `Reg<CFDRFDF_9_SPEC>`"]
pub type CFDRFDF_9 = crate::Reg<cfdrfdf_9::CFDRFDF_9_SPEC>;
#[doc = "RX FIFO Access Data Field 9 Register %s"]
pub mod cfdrfdf_9;
#[doc = "CFDRFDF_10 (r) register accessor: an alias for `Reg<CFDRFDF_10_SPEC>`"]
pub type CFDRFDF_10 = crate::Reg<cfdrfdf_10::CFDRFDF_10_SPEC>;
#[doc = "RX FIFO Access Data Field 10 Register %s"]
pub mod cfdrfdf_10;
#[doc = "CFDRFDF_11 (r) register accessor: an alias for `Reg<CFDRFDF_11_SPEC>`"]
pub type CFDRFDF_11 = crate::Reg<cfdrfdf_11::CFDRFDF_11_SPEC>;
#[doc = "RX FIFO Access Data Field 11 Register %s"]
pub mod cfdrfdf_11;
#[doc = "CFDRFDF_12 (r) register accessor: an alias for `Reg<CFDRFDF_12_SPEC>`"]
pub type CFDRFDF_12 = crate::Reg<cfdrfdf_12::CFDRFDF_12_SPEC>;
#[doc = "RX FIFO Access Data Field 12 Register %s"]
pub mod cfdrfdf_12;
#[doc = "CFDRFDF_13 (r) register accessor: an alias for `Reg<CFDRFDF_13_SPEC>`"]
pub type CFDRFDF_13 = crate::Reg<cfdrfdf_13::CFDRFDF_13_SPEC>;
#[doc = "RX FIFO Access Data Field 13 Register %s"]
pub mod cfdrfdf_13;
#[doc = "CFDRFDF_14 (r) register accessor: an alias for `Reg<CFDRFDF_14_SPEC>`"]
pub type CFDRFDF_14 = crate::Reg<cfdrfdf_14::CFDRFDF_14_SPEC>;
#[doc = "RX FIFO Access Data Field 14 Register %s"]
pub mod cfdrfdf_14;
#[doc = "CFDRFDF_15 (r) register accessor: an alias for `Reg<CFDRFDF_15_SPEC>`"]
pub type CFDRFDF_15 = crate::Reg<cfdrfdf_15::CFDRFDF_15_SPEC>;
#[doc = "RX FIFO Access Data Field 15 Register %s"]
pub mod cfdrfdf_15;
#[doc = "CFDCFID (rw) register accessor: an alias for `Reg<CFDCFID_SPEC>`"]
pub type CFDCFID = crate::Reg<cfdcfid::CFDCFID_SPEC>;
#[doc = "Common FIFO Access ID Register"]
pub mod cfdcfid;
#[doc = "CFDCFPTR (rw) register accessor: an alias for `Reg<CFDCFPTR_SPEC>`"]
pub type CFDCFPTR = crate::Reg<cfdcfptr::CFDCFPTR_SPEC>;
#[doc = "Common FIFO Access Pointer Register"]
pub mod cfdcfptr;
#[doc = "CFDCFFDCSTS (rw) register accessor: an alias for `Reg<CFDCFFDCSTS_SPEC>`"]
pub type CFDCFFDCSTS = crate::Reg<cfdcffdcsts::CFDCFFDCSTS_SPEC>;
#[doc = "Common FIFO Access CANFD Control/Status Register"]
pub mod cfdcffdcsts;
#[doc = "CFDCFDF (rw) register accessor: an alias for `Reg<CFDCFDF_SPEC>`"]
pub type CFDCFDF = crate::Reg<cfdcfdf::CFDCFDF_SPEC>;
#[doc = "Common FIFO Access Data Field %s Registers"]
pub mod cfdcfdf;
#[doc = "CFDTMID (rw) register accessor: an alias for `Reg<CFDTMID_SPEC>`"]
pub type CFDTMID = crate::Reg<cfdtmid::CFDTMID_SPEC>;
#[doc = "TX Message Buffer ID Registers"]
pub mod cfdtmid;
#[doc = "CFDTMPTR (rw) register accessor: an alias for `Reg<CFDTMPTR_SPEC>`"]
pub type CFDTMPTR = crate::Reg<cfdtmptr::CFDTMPTR_SPEC>;
#[doc = "TX Message Buffer Pointer Register"]
pub mod cfdtmptr;
#[doc = "CFDTMFDCTR (rw) register accessor: an alias for `Reg<CFDTMFDCTR_SPEC>`"]
pub type CFDTMFDCTR = crate::Reg<cfdtmfdctr::CFDTMFDCTR_SPEC>;
#[doc = "TX Message Buffer CANFD Control Register"]
pub mod cfdtmfdctr;
#[doc = "CFDTMDF_0 (rw) register accessor: an alias for `Reg<CFDTMDF_0_SPEC>`"]
pub type CFDTMDF_0 = crate::Reg<cfdtmdf_0::CFDTMDF_0_SPEC>;
#[doc = "TX Message Buffer Data Field Register"]
pub mod cfdtmdf_0;
#[doc = "CFDTMDF_1 (rw) register accessor: an alias for `Reg<CFDTMDF_1_SPEC>`"]
pub type CFDTMDF_1 = crate::Reg<cfdtmdf_1::CFDTMDF_1_SPEC>;
#[doc = "TX Message Buffer Data Field Register"]
pub mod cfdtmdf_1;
#[doc = "CFDTMDF_2 (rw) register accessor: an alias for `Reg<CFDTMDF_2_SPEC>`"]
pub type CFDTMDF_2 = crate::Reg<cfdtmdf_2::CFDTMDF_2_SPEC>;
#[doc = "TX Message Buffer Data Field Register"]
pub mod cfdtmdf_2;
#[doc = "CFDTMDF_3 (rw) register accessor: an alias for `Reg<CFDTMDF_3_SPEC>`"]
pub type CFDTMDF_3 = crate::Reg<cfdtmdf_3::CFDTMDF_3_SPEC>;
#[doc = "TX Message Buffer Data Field Register"]
pub mod cfdtmdf_3;
#[doc = "CFDTMDF_4 (rw) register accessor: an alias for `Reg<CFDTMDF_4_SPEC>`"]
pub type CFDTMDF_4 = crate::Reg<cfdtmdf_4::CFDTMDF_4_SPEC>;
#[doc = "TX Message Buffer Data Field Register"]
pub mod cfdtmdf_4;
#[doc = "CFDTMDF_5 (rw) register accessor: an alias for `Reg<CFDTMDF_5_SPEC>`"]
pub type CFDTMDF_5 = crate::Reg<cfdtmdf_5::CFDTMDF_5_SPEC>;
#[doc = "TX Message Buffer Data Field Register"]
pub mod cfdtmdf_5;
#[doc = "CFDTMDF_6 (rw) register accessor: an alias for `Reg<CFDTMDF_6_SPEC>`"]
pub type CFDTMDF_6 = crate::Reg<cfdtmdf_6::CFDTMDF_6_SPEC>;
#[doc = "TX Message Buffer Data Field Register"]
pub mod cfdtmdf_6;
#[doc = "CFDTMDF_7 (rw) register accessor: an alias for `Reg<CFDTMDF_7_SPEC>`"]
pub type CFDTMDF_7 = crate::Reg<cfdtmdf_7::CFDTMDF_7_SPEC>;
#[doc = "TX Message Buffer Data Field Register"]
pub mod cfdtmdf_7;
#[doc = "CFDTMDF_8 (rw) register accessor: an alias for `Reg<CFDTMDF_8_SPEC>`"]
pub type CFDTMDF_8 = crate::Reg<cfdtmdf_8::CFDTMDF_8_SPEC>;
#[doc = "TX Message Buffer Data Field Register"]
pub mod cfdtmdf_8;
#[doc = "CFDTMDF_9 (rw) register accessor: an alias for `Reg<CFDTMDF_9_SPEC>`"]
pub type CFDTMDF_9 = crate::Reg<cfdtmdf_9::CFDTMDF_9_SPEC>;
#[doc = "TX Message Buffer Data Field Register"]
pub mod cfdtmdf_9;
#[doc = "CFDTMDF_10 (rw) register accessor: an alias for `Reg<CFDTMDF_10_SPEC>`"]
pub type CFDTMDF_10 = crate::Reg<cfdtmdf_10::CFDTMDF_10_SPEC>;
#[doc = "TX Message Buffer Data Field Register"]
pub mod cfdtmdf_10;
#[doc = "CFDTMDF_11 (rw) register accessor: an alias for `Reg<CFDTMDF_11_SPEC>`"]
pub type CFDTMDF_11 = crate::Reg<cfdtmdf_11::CFDTMDF_11_SPEC>;
#[doc = "TX Message Buffer Data Field Register"]
pub mod cfdtmdf_11;
#[doc = "CFDTMDF_12 (rw) register accessor: an alias for `Reg<CFDTMDF_12_SPEC>`"]
pub type CFDTMDF_12 = crate::Reg<cfdtmdf_12::CFDTMDF_12_SPEC>;
#[doc = "TX Message Buffer Data Field Register"]
pub mod cfdtmdf_12;
#[doc = "CFDTMDF_13 (rw) register accessor: an alias for `Reg<CFDTMDF_13_SPEC>`"]
pub type CFDTMDF_13 = crate::Reg<cfdtmdf_13::CFDTMDF_13_SPEC>;
#[doc = "TX Message Buffer Data Field Register"]
pub mod cfdtmdf_13;
#[doc = "CFDTMDF_14 (rw) register accessor: an alias for `Reg<CFDTMDF_14_SPEC>`"]
pub type CFDTMDF_14 = crate::Reg<cfdtmdf_14::CFDTMDF_14_SPEC>;
#[doc = "TX Message Buffer Data Field Register"]
pub mod cfdtmdf_14;
#[doc = "CFDTMDF_15 (rw) register accessor: an alias for `Reg<CFDTMDF_15_SPEC>`"]
pub type CFDTMDF_15 = crate::Reg<cfdtmdf_15::CFDTMDF_15_SPEC>;
#[doc = "TX Message Buffer Data Field Register"]
pub mod cfdtmdf_15;
#[doc = "CFDTHLACC0 (r) register accessor: an alias for `Reg<CFDTHLACC0_SPEC>`"]
pub type CFDTHLACC0 = crate::Reg<cfdthlacc0::CFDTHLACC0_SPEC>;
#[doc = "TX History List Access Register 0"]
pub mod cfdthlacc0;
#[doc = "CFDTHLACC1 (r) register accessor: an alias for `Reg<CFDTHLACC1_SPEC>`"]
pub type CFDTHLACC1 = crate::Reg<cfdthlacc1::CFDTHLACC1_SPEC>;
#[doc = "TX History List Access Register 1"]
pub mod cfdthlacc1;
#[doc = "CFDRMID (r) register accessor: an alias for `Reg<CFDRMID_SPEC>`"]
pub type CFDRMID = crate::Reg<cfdrmid::CFDRMID_SPEC>;
#[doc = "RX Message Buffer ID Registers"]
pub mod cfdrmid;
#[doc = "CFDRMPTR (r) register accessor: an alias for `Reg<CFDRMPTR_SPEC>`"]
pub type CFDRMPTR = crate::Reg<cfdrmptr::CFDRMPTR_SPEC>;
#[doc = "RX Message Buffer Pointer Registers"]
pub mod cfdrmptr;
#[doc = "CFDRMFDSTS (r) register accessor: an alias for `Reg<CFDRMFDSTS_SPEC>`"]
pub type CFDRMFDSTS = crate::Reg<cfdrmfdsts::CFDRMFDSTS_SPEC>;
#[doc = "RX Message Buffer CANFD Status Registers"]
pub mod cfdrmfdsts;
#[doc = "CFDRMDF_0 (r) register accessor: an alias for `Reg<CFDRMDF_0_SPEC>`"]
pub type CFDRMDF_0 = crate::Reg<cfdrmdf_0::CFDRMDF_0_SPEC>;
#[doc = "RX Message Buffer Data Field 0 Registers"]
pub mod cfdrmdf_0;
#[doc = "CFDRMDF_1 (r) register accessor: an alias for `Reg<CFDRMDF_1_SPEC>`"]
pub type CFDRMDF_1 = crate::Reg<cfdrmdf_1::CFDRMDF_1_SPEC>;
#[doc = "RX Message Buffer Data Field 1 Registers"]
pub mod cfdrmdf_1;
#[doc = "CFDRMDF_2 (r) register accessor: an alias for `Reg<CFDRMDF_2_SPEC>`"]
pub type CFDRMDF_2 = crate::Reg<cfdrmdf_2::CFDRMDF_2_SPEC>;
#[doc = "RX Message Buffer Data Field 2 Registers"]
pub mod cfdrmdf_2;
#[doc = "CFDRMDF_3 (r) register accessor: an alias for `Reg<CFDRMDF_3_SPEC>`"]
pub type CFDRMDF_3 = crate::Reg<cfdrmdf_3::CFDRMDF_3_SPEC>;
#[doc = "RX Message Buffer Data Field 3 Registers"]
pub mod cfdrmdf_3;
#[doc = "CFDRMDF_4 (r) register accessor: an alias for `Reg<CFDRMDF_4_SPEC>`"]
pub type CFDRMDF_4 = crate::Reg<cfdrmdf_4::CFDRMDF_4_SPEC>;
#[doc = "RX Message Buffer Data Field 4 Registers"]
pub mod cfdrmdf_4;
#[doc = "CFDRMDF_5 (r) register accessor: an alias for `Reg<CFDRMDF_5_SPEC>`"]
pub type CFDRMDF_5 = crate::Reg<cfdrmdf_5::CFDRMDF_5_SPEC>;
#[doc = "RX Message Buffer Data Field 5 Registers"]
pub mod cfdrmdf_5;
#[doc = "CFDRMDF_6 (r) register accessor: an alias for `Reg<CFDRMDF_6_SPEC>`"]
pub type CFDRMDF_6 = crate::Reg<cfdrmdf_6::CFDRMDF_6_SPEC>;
#[doc = "RX Message Buffer Data Field 6 Registers"]
pub mod cfdrmdf_6;
#[doc = "CFDRMDF_7 (r) register accessor: an alias for `Reg<CFDRMDF_7_SPEC>`"]
pub type CFDRMDF_7 = crate::Reg<cfdrmdf_7::CFDRMDF_7_SPEC>;
#[doc = "RX Message Buffer Data Field 7 Registers"]
pub mod cfdrmdf_7;
#[doc = "CFDRMDF_8 (r) register accessor: an alias for `Reg<CFDRMDF_8_SPEC>`"]
pub type CFDRMDF_8 = crate::Reg<cfdrmdf_8::CFDRMDF_8_SPEC>;
#[doc = "RX Message Buffer Data Field 8 Registers"]
pub mod cfdrmdf_8;
#[doc = "CFDRMDF_9 (r) register accessor: an alias for `Reg<CFDRMDF_9_SPEC>`"]
pub type CFDRMDF_9 = crate::Reg<cfdrmdf_9::CFDRMDF_9_SPEC>;
#[doc = "RX Message Buffer Data Field 9 Registers"]
pub mod cfdrmdf_9;
#[doc = "CFDRMDF_10 (r) register accessor: an alias for `Reg<CFDRMDF_10_SPEC>`"]
pub type CFDRMDF_10 = crate::Reg<cfdrmdf_10::CFDRMDF_10_SPEC>;
#[doc = "RX Message Buffer Data Field 10 Registers"]
pub mod cfdrmdf_10;
#[doc = "CFDRMDF_11 (r) register accessor: an alias for `Reg<CFDRMDF_11_SPEC>`"]
pub type CFDRMDF_11 = crate::Reg<cfdrmdf_11::CFDRMDF_11_SPEC>;
#[doc = "RX Message Buffer Data Field 11 Registers"]
pub mod cfdrmdf_11;
#[doc = "CFDRMDF_12 (r) register accessor: an alias for `Reg<CFDRMDF_12_SPEC>`"]
pub type CFDRMDF_12 = crate::Reg<cfdrmdf_12::CFDRMDF_12_SPEC>;
#[doc = "RX Message Buffer Data Field 12 Registers"]
pub mod cfdrmdf_12;
#[doc = "CFDRMDF_13 (r) register accessor: an alias for `Reg<CFDRMDF_13_SPEC>`"]
pub type CFDRMDF_13 = crate::Reg<cfdrmdf_13::CFDRMDF_13_SPEC>;
#[doc = "RX Message Buffer Data Field 13 Registers"]
pub mod cfdrmdf_13;
#[doc = "CFDRMDF_14 (r) register accessor: an alias for `Reg<CFDRMDF_14_SPEC>`"]
pub type CFDRMDF_14 = crate::Reg<cfdrmdf_14::CFDRMDF_14_SPEC>;
#[doc = "RX Message Buffer Data Field 14 Registers"]
pub mod cfdrmdf_14;
#[doc = "CFDRMDF_15 (r) register accessor: an alias for `Reg<CFDRMDF_15_SPEC>`"]
pub type CFDRMDF_15 = crate::Reg<cfdrmdf_15::CFDRMDF_15_SPEC>;
#[doc = "RX Message Buffer Data Field 15 Registers"]
pub mod cfdrmdf_15;
pub use cfdrmdf_0 as cfdrmdf8_0;
pub use cfdrmdf_0 as cfdrmdf16_0;
pub use cfdrmdf_0 as cfdrmdf24_0;
pub use cfdrmdf_1 as cfdrmdf8_1;
pub use cfdrmdf_1 as cfdrmdf16_1;
pub use cfdrmdf_1 as cfdrmdf24_1;
pub use cfdrmdf_10 as cfdrmdf8_10;
pub use cfdrmdf_10 as cfdrmdf16_10;
pub use cfdrmdf_10 as cfdrmdf24_10;
pub use cfdrmdf_11 as cfdrmdf8_11;
pub use cfdrmdf_11 as cfdrmdf16_11;
pub use cfdrmdf_11 as cfdrmdf24_11;
pub use cfdrmdf_12 as cfdrmdf8_12;
pub use cfdrmdf_12 as cfdrmdf16_12;
pub use cfdrmdf_12 as cfdrmdf24_12;
pub use cfdrmdf_13 as cfdrmdf8_13;
pub use cfdrmdf_13 as cfdrmdf16_13;
pub use cfdrmdf_13 as cfdrmdf24_13;
pub use cfdrmdf_14 as cfdrmdf8_14;
pub use cfdrmdf_14 as cfdrmdf16_14;
pub use cfdrmdf_14 as cfdrmdf24_14;
pub use cfdrmdf_15 as cfdrmdf8_15;
pub use cfdrmdf_15 as cfdrmdf16_15;
pub use cfdrmdf_15 as cfdrmdf24_15;
pub use cfdrmdf_2 as cfdrmdf8_2;
pub use cfdrmdf_2 as cfdrmdf16_2;
pub use cfdrmdf_2 as cfdrmdf24_2;
pub use cfdrmdf_3 as cfdrmdf8_3;
pub use cfdrmdf_3 as cfdrmdf16_3;
pub use cfdrmdf_3 as cfdrmdf24_3;
pub use cfdrmdf_4 as cfdrmdf8_4;
pub use cfdrmdf_4 as cfdrmdf16_4;
pub use cfdrmdf_4 as cfdrmdf24_4;
pub use cfdrmdf_5 as cfdrmdf8_5;
pub use cfdrmdf_5 as cfdrmdf16_5;
pub use cfdrmdf_5 as cfdrmdf24_5;
pub use cfdrmdf_6 as cfdrmdf8_6;
pub use cfdrmdf_6 as cfdrmdf16_6;
pub use cfdrmdf_6 as cfdrmdf24_6;
pub use cfdrmdf_7 as cfdrmdf8_7;
pub use cfdrmdf_7 as cfdrmdf16_7;
pub use cfdrmdf_7 as cfdrmdf24_7;
pub use cfdrmdf_8 as cfdrmdf8_8;
pub use cfdrmdf_8 as cfdrmdf16_8;
pub use cfdrmdf_8 as cfdrmdf24_8;
pub use cfdrmdf_9 as cfdrmdf8_9;
pub use cfdrmdf_9 as cfdrmdf16_9;
pub use cfdrmdf_9 as cfdrmdf24_9;
pub use cfdrmfdsts as cfdrmfdsts8;
pub use cfdrmfdsts as cfdrmfdsts16;
pub use cfdrmfdsts as cfdrmfdsts24;
pub use cfdrmid as cfdrmid8;
pub use cfdrmid as cfdrmid16;
pub use cfdrmid as cfdrmid24;
pub use cfdrmptr as cfdrmptr8;
pub use cfdrmptr as cfdrmptr16;
pub use cfdrmptr as cfdrmptr24;
pub use CFDRMDF_0 as CFDRMDF8_0;
pub use CFDRMDF_0 as CFDRMDF16_0;
pub use CFDRMDF_0 as CFDRMDF24_0;
pub use CFDRMDF_1 as CFDRMDF8_1;
pub use CFDRMDF_1 as CFDRMDF16_1;
pub use CFDRMDF_1 as CFDRMDF24_1;
pub use CFDRMDF_10 as CFDRMDF8_10;
pub use CFDRMDF_10 as CFDRMDF16_10;
pub use CFDRMDF_10 as CFDRMDF24_10;
pub use CFDRMDF_11 as CFDRMDF8_11;
pub use CFDRMDF_11 as CFDRMDF16_11;
pub use CFDRMDF_11 as CFDRMDF24_11;
pub use CFDRMDF_12 as CFDRMDF8_12;
pub use CFDRMDF_12 as CFDRMDF16_12;
pub use CFDRMDF_12 as CFDRMDF24_12;
pub use CFDRMDF_13 as CFDRMDF8_13;
pub use CFDRMDF_13 as CFDRMDF16_13;
pub use CFDRMDF_13 as CFDRMDF24_13;
pub use CFDRMDF_14 as CFDRMDF8_14;
pub use CFDRMDF_14 as CFDRMDF16_14;
pub use CFDRMDF_14 as CFDRMDF24_14;
pub use CFDRMDF_15 as CFDRMDF8_15;
pub use CFDRMDF_15 as CFDRMDF16_15;
pub use CFDRMDF_15 as CFDRMDF24_15;
pub use CFDRMDF_2 as CFDRMDF8_2;
pub use CFDRMDF_2 as CFDRMDF16_2;
pub use CFDRMDF_2 as CFDRMDF24_2;
pub use CFDRMDF_3 as CFDRMDF8_3;
pub use CFDRMDF_3 as CFDRMDF16_3;
pub use CFDRMDF_3 as CFDRMDF24_3;
pub use CFDRMDF_4 as CFDRMDF8_4;
pub use CFDRMDF_4 as CFDRMDF16_4;
pub use CFDRMDF_4 as CFDRMDF24_4;
pub use CFDRMDF_5 as CFDRMDF8_5;
pub use CFDRMDF_5 as CFDRMDF16_5;
pub use CFDRMDF_5 as CFDRMDF24_5;
pub use CFDRMDF_6 as CFDRMDF8_6;
pub use CFDRMDF_6 as CFDRMDF16_6;
pub use CFDRMDF_6 as CFDRMDF24_6;
pub use CFDRMDF_7 as CFDRMDF8_7;
pub use CFDRMDF_7 as CFDRMDF16_7;
pub use CFDRMDF_7 as CFDRMDF24_7;
pub use CFDRMDF_8 as CFDRMDF8_8;
pub use CFDRMDF_8 as CFDRMDF16_8;
pub use CFDRMDF_8 as CFDRMDF24_8;
pub use CFDRMDF_9 as CFDRMDF8_9;
pub use CFDRMDF_9 as CFDRMDF16_9;
pub use CFDRMDF_9 as CFDRMDF24_9;
pub use CFDRMFDSTS as CFDRMFDSTS8;
pub use CFDRMFDSTS as CFDRMFDSTS16;
pub use CFDRMFDSTS as CFDRMFDSTS24;
pub use CFDRMID as CFDRMID8;
pub use CFDRMID as CFDRMID16;
pub use CFDRMID as CFDRMID24;
pub use CFDRMPTR as CFDRMPTR8;
pub use CFDRMPTR as CFDRMPTR16;
pub use CFDRMPTR as CFDRMPTR24;
