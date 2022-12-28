#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel Processing Status Register"]
    pub iircprcs: IIRCPRCS,
    #[doc = "0x04 - Channel Processing Completion Flag Register"]
    pub iircprcff: IIRCPRCFF,
    #[doc = "0x08 - Output Data Preparation Completion Flag Register"]
    pub iirordyf: IIRORDYF,
    #[doc = "0x0c - Operation Error Flag Register"]
    pub iircerrf: IIRCERRF,
    #[doc = "0x10 - Operation Control Register"]
    pub iiropcnt: IIROPCNT,
    _reserved5: [u8; 0x0c],
    #[doc = "0x20 - ECC Control Register"]
    pub iirecccnt: IIRECCCNT,
    _reserved6: [u8; 0x04],
    #[doc = "0x28 - ECC Interrupt Enable Register"]
    pub iireccint: IIRECCINT,
    _reserved7: [u8; 0x04],
    #[doc = "0x30 - ECC Error Flag Register"]
    pub iireccef: IIRECCEF,
    #[doc = "0x34 - ECC Error Flag Clear Register"]
    pub iireccefclr: IIRECCEFCLR,
    #[doc = "0x38 - ECC 1-bit Error Address Register"]
    pub iireseadr: IIRESEADR,
    #[doc = "0x3c - ECC 2-bit Error Address Register"]
    pub iiredeadr: IIREDEADR,
    _reserved11: [u8; 0xc0],
    #[doc = "0x100 - Channel %s Input Register"]
    pub iirch0inp: IIRCHINP,
    #[doc = "0x104 - Channel %s Output Register"]
    pub iirch0out: IIRCHOUT,
    #[doc = "0x108 - Channel %s Control Register"]
    pub iirch0cnt: IIRCHCNT,
    #[doc = "0x10c - Channel %s Interrupt Enable Register"]
    pub iirch0int: IIRCHINT,
    #[doc = "0x10d - Channel %s Status Register"]
    pub iirch0sts: IIRCHSTS,
    #[doc = "0x10e - Channel %s Flag Clear Register"]
    pub iirch0fclr: IIRCHFCLR,
    _reserved17: [u8; 0x01],
    #[doc = "0x110 - Channel %s Input Register"]
    pub iirch1inp: IIRCHINP,
    #[doc = "0x114 - Channel %s Output Register"]
    pub iirch1out: IIRCHOUT,
    #[doc = "0x118 - Channel %s Control Register"]
    pub iirch1cnt: IIRCHCNT,
    #[doc = "0x11c - Channel %s Interrupt Enable Register"]
    pub iirch1int: IIRCHINT,
    #[doc = "0x11d - Channel %s Status Register"]
    pub iirch1sts: IIRCHSTS,
    #[doc = "0x11e - Channel %s Flag Clear Register"]
    pub iirch1fclr: IIRCHFCLR,
    _reserved23: [u8; 0x01],
    #[doc = "0x120 - Channel %s Input Register"]
    pub iirch2inp: IIRCHINP,
    #[doc = "0x124 - Channel %s Output Register"]
    pub iirch2out: IIRCHOUT,
    #[doc = "0x128 - Channel %s Control Register"]
    pub iirch2cnt: IIRCHCNT,
    #[doc = "0x12c - Channel %s Interrupt Enable Register"]
    pub iirch2int: IIRCHINT,
    #[doc = "0x12d - Channel %s Status Register"]
    pub iirch2sts: IIRCHSTS,
    #[doc = "0x12e - Channel %s Flag Clear Register"]
    pub iirch2fclr: IIRCHFCLR,
    _reserved29: [u8; 0x01],
    #[doc = "0x130 - Channel %s Input Register"]
    pub iirch3inp: IIRCHINP,
    #[doc = "0x134 - Channel %s Output Register"]
    pub iirch3out: IIRCHOUT,
    #[doc = "0x138 - Channel %s Control Register"]
    pub iirch3cnt: IIRCHCNT,
    #[doc = "0x13c - Channel %s Interrupt Enable Register"]
    pub iirch3int: IIRCHINT,
    #[doc = "0x13d - Channel %s Status Register"]
    pub iirch3sts: IIRCHSTS,
    #[doc = "0x13e - Channel %s Flag Clear Register"]
    pub iirch3fclr: IIRCHFCLR,
    _reserved35: [u8; 0x01],
    #[doc = "0x140 - Channel %s Input Register"]
    pub iirch4inp: IIRCHINP,
    #[doc = "0x144 - Channel %s Output Register"]
    pub iirch4out: IIRCHOUT,
    #[doc = "0x148 - Channel %s Control Register"]
    pub iirch4cnt: IIRCHCNT,
    #[doc = "0x14c - Channel %s Interrupt Enable Register"]
    pub iirch4int: IIRCHINT,
    #[doc = "0x14d - Channel %s Status Register"]
    pub iirch4sts: IIRCHSTS,
    #[doc = "0x14e - Channel %s Flag Clear Register"]
    pub iirch4fclr: IIRCHFCLR,
    _reserved41: [u8; 0x01],
    #[doc = "0x150 - Channel %s Input Register"]
    pub iirch5inp: IIRCHINP,
    #[doc = "0x154 - Channel %s Output Register"]
    pub iirch5out: IIRCHOUT,
    #[doc = "0x158 - Channel %s Control Register"]
    pub iirch5cnt: IIRCHCNT,
    #[doc = "0x15c - Channel %s Interrupt Enable Register"]
    pub iirch5int: IIRCHINT,
    #[doc = "0x15d - Channel %s Status Register"]
    pub iirch5sts: IIRCHSTS,
    #[doc = "0x15e - Channel %s Flag Clear Register"]
    pub iirch5fclr: IIRCHFCLR,
    _reserved47: [u8; 0x01],
    #[doc = "0x160 - Channel %s Input Register"]
    pub iirch6inp: IIRCHINP,
    #[doc = "0x164 - Channel %s Output Register"]
    pub iirch6out: IIRCHOUT,
    #[doc = "0x168 - Channel %s Control Register"]
    pub iirch6cnt: IIRCHCNT,
    #[doc = "0x16c - Channel %s Interrupt Enable Register"]
    pub iirch6int: IIRCHINT,
    #[doc = "0x16d - Channel %s Status Register"]
    pub iirch6sts: IIRCHSTS,
    #[doc = "0x16e - Channel %s Flag Clear Register"]
    pub iirch6fclr: IIRCHFCLR,
    _reserved53: [u8; 0x01],
    #[doc = "0x170 - Channel %s Input Register"]
    pub iirch7inp: IIRCHINP,
    #[doc = "0x174 - Channel %s Output Register"]
    pub iirch7out: IIRCHOUT,
    #[doc = "0x178 - Channel %s Control Register"]
    pub iirch7cnt: IIRCHCNT,
    #[doc = "0x17c - Channel %s Interrupt Enable Register"]
    pub iirch7int: IIRCHINT,
    #[doc = "0x17d - Channel %s Status Register"]
    pub iirch7sts: IIRCHSTS,
    #[doc = "0x17e - Channel %s Flag Clear Register"]
    pub iirch7fclr: IIRCHFCLR,
    _reserved59: [u8; 0x01],
    #[doc = "0x180 - Channel %s Input Register"]
    pub iirch8inp: IIRCHINP,
    #[doc = "0x184 - Channel %s Output Register"]
    pub iirch8out: IIRCHOUT,
    #[doc = "0x188 - Channel %s Control Register"]
    pub iirch8cnt: IIRCHCNT,
    #[doc = "0x18c - Channel %s Interrupt Enable Register"]
    pub iirch8int: IIRCHINT,
    #[doc = "0x18d - Channel %s Status Register"]
    pub iirch8sts: IIRCHSTS,
    #[doc = "0x18e - Channel %s Flag Clear Register"]
    pub iirch8fclr: IIRCHFCLR,
    _reserved65: [u8; 0x01],
    #[doc = "0x190 - Channel %s Input Register"]
    pub iirch9inp: IIRCHINP,
    #[doc = "0x194 - Channel %s Output Register"]
    pub iirch9out: IIRCHOUT,
    #[doc = "0x198 - Channel %s Control Register"]
    pub iirch9cnt: IIRCHCNT,
    #[doc = "0x19c - Channel %s Interrupt Enable Register"]
    pub iirch9int: IIRCHINT,
    #[doc = "0x19d - Channel %s Status Register"]
    pub iirch9sts: IIRCHSTS,
    #[doc = "0x19e - Channel %s Flag Clear Register"]
    pub iirch9fclr: IIRCHFCLR,
    _reserved71: [u8; 0x01],
    #[doc = "0x1a0 - Channel %s Input Register"]
    pub iirch10inp: IIRCHINP,
    #[doc = "0x1a4 - Channel %s Output Register"]
    pub iirch10out: IIRCHOUT,
    #[doc = "0x1a8 - Channel %s Control Register"]
    pub iirch10cnt: IIRCHCNT,
    #[doc = "0x1ac - Channel %s Interrupt Enable Register"]
    pub iirch10int: IIRCHINT,
    #[doc = "0x1ad - Channel %s Status Register"]
    pub iirch10sts: IIRCHSTS,
    #[doc = "0x1ae - Channel %s Flag Clear Register"]
    pub iirch10fclr: IIRCHFCLR,
    _reserved77: [u8; 0x01],
    #[doc = "0x1b0 - Channel %s Input Register"]
    pub iirch11inp: IIRCHINP,
    #[doc = "0x1b4 - Channel %s Output Register"]
    pub iirch11out: IIRCHOUT,
    #[doc = "0x1b8 - Channel %s Control Register"]
    pub iirch11cnt: IIRCHCNT,
    #[doc = "0x1bc - Channel %s Interrupt Enable Register"]
    pub iirch11int: IIRCHINT,
    #[doc = "0x1bd - Channel %s Status Register"]
    pub iirch11sts: IIRCHSTS,
    #[doc = "0x1be - Channel %s Flag Clear Register"]
    pub iirch11fclr: IIRCHFCLR,
    _reserved83: [u8; 0x01],
    #[doc = "0x1c0 - Channel %s Input Register"]
    pub iirch12inp: IIRCHINP,
    #[doc = "0x1c4 - Channel %s Output Register"]
    pub iirch12out: IIRCHOUT,
    #[doc = "0x1c8 - Channel %s Control Register"]
    pub iirch12cnt: IIRCHCNT,
    #[doc = "0x1cc - Channel %s Interrupt Enable Register"]
    pub iirch12int: IIRCHINT,
    #[doc = "0x1cd - Channel %s Status Register"]
    pub iirch12sts: IIRCHSTS,
    #[doc = "0x1ce - Channel %s Flag Clear Register"]
    pub iirch12fclr: IIRCHFCLR,
    _reserved89: [u8; 0x01],
    #[doc = "0x1d0 - Channel %s Input Register"]
    pub iirch13inp: IIRCHINP,
    #[doc = "0x1d4 - Channel %s Output Register"]
    pub iirch13out: IIRCHOUT,
    #[doc = "0x1d8 - Channel %s Control Register"]
    pub iirch13cnt: IIRCHCNT,
    #[doc = "0x1dc - Channel %s Interrupt Enable Register"]
    pub iirch13int: IIRCHINT,
    #[doc = "0x1dd - Channel %s Status Register"]
    pub iirch13sts: IIRCHSTS,
    #[doc = "0x1de - Channel %s Flag Clear Register"]
    pub iirch13fclr: IIRCHFCLR,
    _reserved95: [u8; 0x01],
    #[doc = "0x1e0 - Channel %s Input Register"]
    pub iirch14inp: IIRCHINP,
    #[doc = "0x1e4 - Channel %s Output Register"]
    pub iirch14out: IIRCHOUT,
    #[doc = "0x1e8 - Channel %s Control Register"]
    pub iirch14cnt: IIRCHCNT,
    #[doc = "0x1ec - Channel %s Interrupt Enable Register"]
    pub iirch14int: IIRCHINT,
    #[doc = "0x1ed - Channel %s Status Register"]
    pub iirch14sts: IIRCHSTS,
    #[doc = "0x1ee - Channel %s Flag Clear Register"]
    pub iirch14fclr: IIRCHFCLR,
    _reserved101: [u8; 0x01],
    #[doc = "0x1f0 - Channel %s Input Register"]
    pub iirch15inp: IIRCHINP,
    #[doc = "0x1f4 - Channel %s Output Register"]
    pub iirch15out: IIRCHOUT,
    #[doc = "0x1f8 - Channel %s Control Register"]
    pub iirch15cnt: IIRCHCNT,
    #[doc = "0x1fc - Channel %s Interrupt Enable Register"]
    pub iirch15int: IIRCHINT,
    #[doc = "0x1fd - Channel %s Status Register"]
    pub iirch15sts: IIRCHSTS,
    #[doc = "0x1fe - Channel %s Flag Clear Register"]
    pub iirch15fclr: IIRCHFCLR,
    _reserved107: [u8; 0x0201],
    #[doc = "0x400 - Stage %s Coefficient b0 Register"]
    pub iirstg0b0: IIRSTGB0,
    #[doc = "0x404 - Stage %s Coefficient b1 Register"]
    pub iirstg0b1: IIRSTGB1,
    #[doc = "0x408 - Stage %s Coefficient b2 Register"]
    pub iirstg0b2: IIRSTGB2,
    #[doc = "0x40c - Stage %s Coefficient a1 Register"]
    pub iirstg0a1: IIRSTGA1,
    #[doc = "0x410 - Stage %s Coefficient a2 Register"]
    pub iirstg0a2: IIRSTGA2,
    #[doc = "0x414 - Stage %s Delay Data D0 Register"]
    pub iirstg0d0: IIRSTGD0,
    #[doc = "0x418 - Stage %s Delay Data D1 Register"]
    pub iirstg0d1: IIRSTGD1,
    _reserved114: [u8; 0x04],
    #[doc = "0x420 - Stage %s Coefficient b0 Register"]
    pub iirstg1b0: IIRSTGB0,
    #[doc = "0x424 - Stage %s Coefficient b1 Register"]
    pub iirstg1b1: IIRSTGB1,
    #[doc = "0x428 - Stage %s Coefficient b2 Register"]
    pub iirstg1b2: IIRSTGB2,
    #[doc = "0x42c - Stage %s Coefficient a1 Register"]
    pub iirstg1a1: IIRSTGA1,
    #[doc = "0x430 - Stage %s Coefficient a2 Register"]
    pub iirstg1a2: IIRSTGA2,
    #[doc = "0x434 - Stage %s Delay Data D0 Register"]
    pub iirstg1d0: IIRSTGD0,
    #[doc = "0x438 - Stage %s Delay Data D1 Register"]
    pub iirstg1d1: IIRSTGD1,
    _reserved121: [u8; 0x04],
    #[doc = "0x440 - Stage %s Coefficient b0 Register"]
    pub iirstg2b0: IIRSTGB0,
    #[doc = "0x444 - Stage %s Coefficient b1 Register"]
    pub iirstg2b1: IIRSTGB1,
    #[doc = "0x448 - Stage %s Coefficient b2 Register"]
    pub iirstg2b2: IIRSTGB2,
    #[doc = "0x44c - Stage %s Coefficient a1 Register"]
    pub iirstg2a1: IIRSTGA1,
    #[doc = "0x450 - Stage %s Coefficient a2 Register"]
    pub iirstg2a2: IIRSTGA2,
    #[doc = "0x454 - Stage %s Delay Data D0 Register"]
    pub iirstg2d0: IIRSTGD0,
    #[doc = "0x458 - Stage %s Delay Data D1 Register"]
    pub iirstg2d1: IIRSTGD1,
    _reserved128: [u8; 0x04],
    #[doc = "0x460 - Stage %s Coefficient b0 Register"]
    pub iirstg3b0: IIRSTGB0,
    #[doc = "0x464 - Stage %s Coefficient b1 Register"]
    pub iirstg3b1: IIRSTGB1,
    #[doc = "0x468 - Stage %s Coefficient b2 Register"]
    pub iirstg3b2: IIRSTGB2,
    #[doc = "0x46c - Stage %s Coefficient a1 Register"]
    pub iirstg3a1: IIRSTGA1,
    #[doc = "0x470 - Stage %s Coefficient a2 Register"]
    pub iirstg3a2: IIRSTGA2,
    #[doc = "0x474 - Stage %s Delay Data D0 Register"]
    pub iirstg3d0: IIRSTGD0,
    #[doc = "0x478 - Stage %s Delay Data D1 Register"]
    pub iirstg3d1: IIRSTGD1,
    _reserved135: [u8; 0x04],
    #[doc = "0x480 - Stage %s Coefficient b0 Register"]
    pub iirstg4b0: IIRSTGB0,
    #[doc = "0x484 - Stage %s Coefficient b1 Register"]
    pub iirstg4b1: IIRSTGB1,
    #[doc = "0x488 - Stage %s Coefficient b2 Register"]
    pub iirstg4b2: IIRSTGB2,
    #[doc = "0x48c - Stage %s Coefficient a1 Register"]
    pub iirstg4a1: IIRSTGA1,
    #[doc = "0x490 - Stage %s Coefficient a2 Register"]
    pub iirstg4a2: IIRSTGA2,
    #[doc = "0x494 - Stage %s Delay Data D0 Register"]
    pub iirstg4d0: IIRSTGD0,
    #[doc = "0x498 - Stage %s Delay Data D1 Register"]
    pub iirstg4d1: IIRSTGD1,
    _reserved142: [u8; 0x04],
    #[doc = "0x4a0 - Stage %s Coefficient b0 Register"]
    pub iirstg5b0: IIRSTGB0,
    #[doc = "0x4a4 - Stage %s Coefficient b1 Register"]
    pub iirstg5b1: IIRSTGB1,
    #[doc = "0x4a8 - Stage %s Coefficient b2 Register"]
    pub iirstg5b2: IIRSTGB2,
    #[doc = "0x4ac - Stage %s Coefficient a1 Register"]
    pub iirstg5a1: IIRSTGA1,
    #[doc = "0x4b0 - Stage %s Coefficient a2 Register"]
    pub iirstg5a2: IIRSTGA2,
    #[doc = "0x4b4 - Stage %s Delay Data D0 Register"]
    pub iirstg5d0: IIRSTGD0,
    #[doc = "0x4b8 - Stage %s Delay Data D1 Register"]
    pub iirstg5d1: IIRSTGD1,
    _reserved149: [u8; 0x04],
    #[doc = "0x4c0 - Stage %s Coefficient b0 Register"]
    pub iirstg6b0: IIRSTGB0,
    #[doc = "0x4c4 - Stage %s Coefficient b1 Register"]
    pub iirstg6b1: IIRSTGB1,
    #[doc = "0x4c8 - Stage %s Coefficient b2 Register"]
    pub iirstg6b2: IIRSTGB2,
    #[doc = "0x4cc - Stage %s Coefficient a1 Register"]
    pub iirstg6a1: IIRSTGA1,
    #[doc = "0x4d0 - Stage %s Coefficient a2 Register"]
    pub iirstg6a2: IIRSTGA2,
    #[doc = "0x4d4 - Stage %s Delay Data D0 Register"]
    pub iirstg6d0: IIRSTGD0,
    #[doc = "0x4d8 - Stage %s Delay Data D1 Register"]
    pub iirstg6d1: IIRSTGD1,
    _reserved156: [u8; 0x04],
    #[doc = "0x4e0 - Stage %s Coefficient b0 Register"]
    pub iirstg7b0: IIRSTGB0,
    #[doc = "0x4e4 - Stage %s Coefficient b1 Register"]
    pub iirstg7b1: IIRSTGB1,
    #[doc = "0x4e8 - Stage %s Coefficient b2 Register"]
    pub iirstg7b2: IIRSTGB2,
    #[doc = "0x4ec - Stage %s Coefficient a1 Register"]
    pub iirstg7a1: IIRSTGA1,
    #[doc = "0x4f0 - Stage %s Coefficient a2 Register"]
    pub iirstg7a2: IIRSTGA2,
    #[doc = "0x4f4 - Stage %s Delay Data D0 Register"]
    pub iirstg7d0: IIRSTGD0,
    #[doc = "0x4f8 - Stage %s Delay Data D1 Register"]
    pub iirstg7d1: IIRSTGD1,
    _reserved163: [u8; 0x04],
    #[doc = "0x500 - Stage %s Coefficient b0 Register"]
    pub iirstg8b0: IIRSTGB0,
    #[doc = "0x504 - Stage %s Coefficient b1 Register"]
    pub iirstg8b1: IIRSTGB1,
    #[doc = "0x508 - Stage %s Coefficient b2 Register"]
    pub iirstg8b2: IIRSTGB2,
    #[doc = "0x50c - Stage %s Coefficient a1 Register"]
    pub iirstg8a1: IIRSTGA1,
    #[doc = "0x510 - Stage %s Coefficient a2 Register"]
    pub iirstg8a2: IIRSTGA2,
    #[doc = "0x514 - Stage %s Delay Data D0 Register"]
    pub iirstg8d0: IIRSTGD0,
    #[doc = "0x518 - Stage %s Delay Data D1 Register"]
    pub iirstg8d1: IIRSTGD1,
    _reserved170: [u8; 0x04],
    #[doc = "0x520 - Stage %s Coefficient b0 Register"]
    pub iirstg9b0: IIRSTGB0,
    #[doc = "0x524 - Stage %s Coefficient b1 Register"]
    pub iirstg9b1: IIRSTGB1,
    #[doc = "0x528 - Stage %s Coefficient b2 Register"]
    pub iirstg9b2: IIRSTGB2,
    #[doc = "0x52c - Stage %s Coefficient a1 Register"]
    pub iirstg9a1: IIRSTGA1,
    #[doc = "0x530 - Stage %s Coefficient a2 Register"]
    pub iirstg9a2: IIRSTGA2,
    #[doc = "0x534 - Stage %s Delay Data D0 Register"]
    pub iirstg9d0: IIRSTGD0,
    #[doc = "0x538 - Stage %s Delay Data D1 Register"]
    pub iirstg9d1: IIRSTGD1,
    _reserved177: [u8; 0x04],
    #[doc = "0x540 - Stage %s Coefficient b0 Register"]
    pub iirstg10b0: IIRSTGB0,
    #[doc = "0x544 - Stage %s Coefficient b1 Register"]
    pub iirstg10b1: IIRSTGB1,
    #[doc = "0x548 - Stage %s Coefficient b2 Register"]
    pub iirstg10b2: IIRSTGB2,
    #[doc = "0x54c - Stage %s Coefficient a1 Register"]
    pub iirstg10a1: IIRSTGA1,
    #[doc = "0x550 - Stage %s Coefficient a2 Register"]
    pub iirstg10a2: IIRSTGA2,
    #[doc = "0x554 - Stage %s Delay Data D0 Register"]
    pub iirstg10d0: IIRSTGD0,
    #[doc = "0x558 - Stage %s Delay Data D1 Register"]
    pub iirstg10d1: IIRSTGD1,
    _reserved184: [u8; 0x04],
    #[doc = "0x560 - Stage %s Coefficient b0 Register"]
    pub iirstg11b0: IIRSTGB0,
    #[doc = "0x564 - Stage %s Coefficient b1 Register"]
    pub iirstg11b1: IIRSTGB1,
    #[doc = "0x568 - Stage %s Coefficient b2 Register"]
    pub iirstg11b2: IIRSTGB2,
    #[doc = "0x56c - Stage %s Coefficient a1 Register"]
    pub iirstg11a1: IIRSTGA1,
    #[doc = "0x570 - Stage %s Coefficient a2 Register"]
    pub iirstg11a2: IIRSTGA2,
    #[doc = "0x574 - Stage %s Delay Data D0 Register"]
    pub iirstg11d0: IIRSTGD0,
    #[doc = "0x578 - Stage %s Delay Data D1 Register"]
    pub iirstg11d1: IIRSTGD1,
    _reserved191: [u8; 0x04],
    #[doc = "0x580 - Stage %s Coefficient b0 Register"]
    pub iirstg12b0: IIRSTGB0,
    #[doc = "0x584 - Stage %s Coefficient b1 Register"]
    pub iirstg12b1: IIRSTGB1,
    #[doc = "0x588 - Stage %s Coefficient b2 Register"]
    pub iirstg12b2: IIRSTGB2,
    #[doc = "0x58c - Stage %s Coefficient a1 Register"]
    pub iirstg12a1: IIRSTGA1,
    #[doc = "0x590 - Stage %s Coefficient a2 Register"]
    pub iirstg12a2: IIRSTGA2,
    #[doc = "0x594 - Stage %s Delay Data D0 Register"]
    pub iirstg12d0: IIRSTGD0,
    #[doc = "0x598 - Stage %s Delay Data D1 Register"]
    pub iirstg12d1: IIRSTGD1,
    _reserved198: [u8; 0x04],
    #[doc = "0x5a0 - Stage %s Coefficient b0 Register"]
    pub iirstg13b0: IIRSTGB0,
    #[doc = "0x5a4 - Stage %s Coefficient b1 Register"]
    pub iirstg13b1: IIRSTGB1,
    #[doc = "0x5a8 - Stage %s Coefficient b2 Register"]
    pub iirstg13b2: IIRSTGB2,
    #[doc = "0x5ac - Stage %s Coefficient a1 Register"]
    pub iirstg13a1: IIRSTGA1,
    #[doc = "0x5b0 - Stage %s Coefficient a2 Register"]
    pub iirstg13a2: IIRSTGA2,
    #[doc = "0x5b4 - Stage %s Delay Data D0 Register"]
    pub iirstg13d0: IIRSTGD0,
    #[doc = "0x5b8 - Stage %s Delay Data D1 Register"]
    pub iirstg13d1: IIRSTGD1,
    _reserved205: [u8; 0x04],
    #[doc = "0x5c0 - Stage %s Coefficient b0 Register"]
    pub iirstg14b0: IIRSTGB0,
    #[doc = "0x5c4 - Stage %s Coefficient b1 Register"]
    pub iirstg14b1: IIRSTGB1,
    #[doc = "0x5c8 - Stage %s Coefficient b2 Register"]
    pub iirstg14b2: IIRSTGB2,
    #[doc = "0x5cc - Stage %s Coefficient a1 Register"]
    pub iirstg14a1: IIRSTGA1,
    #[doc = "0x5d0 - Stage %s Coefficient a2 Register"]
    pub iirstg14a2: IIRSTGA2,
    #[doc = "0x5d4 - Stage %s Delay Data D0 Register"]
    pub iirstg14d0: IIRSTGD0,
    #[doc = "0x5d8 - Stage %s Delay Data D1 Register"]
    pub iirstg14d1: IIRSTGD1,
    _reserved212: [u8; 0x04],
    #[doc = "0x5e0 - Stage %s Coefficient b0 Register"]
    pub iirstg15b0: IIRSTGB0,
    #[doc = "0x5e4 - Stage %s Coefficient b1 Register"]
    pub iirstg15b1: IIRSTGB1,
    #[doc = "0x5e8 - Stage %s Coefficient b2 Register"]
    pub iirstg15b2: IIRSTGB2,
    #[doc = "0x5ec - Stage %s Coefficient a1 Register"]
    pub iirstg15a1: IIRSTGA1,
    #[doc = "0x5f0 - Stage %s Coefficient a2 Register"]
    pub iirstg15a2: IIRSTGA2,
    #[doc = "0x5f4 - Stage %s Delay Data D0 Register"]
    pub iirstg15d0: IIRSTGD0,
    #[doc = "0x5f8 - Stage %s Delay Data D1 Register"]
    pub iirstg15d1: IIRSTGD1,
    _reserved219: [u8; 0x04],
    #[doc = "0x600 - Stage %s Coefficient b0 Register"]
    pub iirstg16b0: IIRSTGB0,
    #[doc = "0x604 - Stage %s Coefficient b1 Register"]
    pub iirstg16b1: IIRSTGB1,
    #[doc = "0x608 - Stage %s Coefficient b2 Register"]
    pub iirstg16b2: IIRSTGB2,
    #[doc = "0x60c - Stage %s Coefficient a1 Register"]
    pub iirstg16a1: IIRSTGA1,
    #[doc = "0x610 - Stage %s Coefficient a2 Register"]
    pub iirstg16a2: IIRSTGA2,
    #[doc = "0x614 - Stage %s Delay Data D0 Register"]
    pub iirstg16d0: IIRSTGD0,
    #[doc = "0x618 - Stage %s Delay Data D1 Register"]
    pub iirstg16d1: IIRSTGD1,
    _reserved226: [u8; 0x04],
    #[doc = "0x620 - Stage %s Coefficient b0 Register"]
    pub iirstg17b0: IIRSTGB0,
    #[doc = "0x624 - Stage %s Coefficient b1 Register"]
    pub iirstg17b1: IIRSTGB1,
    #[doc = "0x628 - Stage %s Coefficient b2 Register"]
    pub iirstg17b2: IIRSTGB2,
    #[doc = "0x62c - Stage %s Coefficient a1 Register"]
    pub iirstg17a1: IIRSTGA1,
    #[doc = "0x630 - Stage %s Coefficient a2 Register"]
    pub iirstg17a2: IIRSTGA2,
    #[doc = "0x634 - Stage %s Delay Data D0 Register"]
    pub iirstg17d0: IIRSTGD0,
    #[doc = "0x638 - Stage %s Delay Data D1 Register"]
    pub iirstg17d1: IIRSTGD1,
    _reserved233: [u8; 0x04],
    #[doc = "0x640 - Stage %s Coefficient b0 Register"]
    pub iirstg18b0: IIRSTGB0,
    #[doc = "0x644 - Stage %s Coefficient b1 Register"]
    pub iirstg18b1: IIRSTGB1,
    #[doc = "0x648 - Stage %s Coefficient b2 Register"]
    pub iirstg18b2: IIRSTGB2,
    #[doc = "0x64c - Stage %s Coefficient a1 Register"]
    pub iirstg18a1: IIRSTGA1,
    #[doc = "0x650 - Stage %s Coefficient a2 Register"]
    pub iirstg18a2: IIRSTGA2,
    #[doc = "0x654 - Stage %s Delay Data D0 Register"]
    pub iirstg18d0: IIRSTGD0,
    #[doc = "0x658 - Stage %s Delay Data D1 Register"]
    pub iirstg18d1: IIRSTGD1,
    _reserved240: [u8; 0x04],
    #[doc = "0x660 - Stage %s Coefficient b0 Register"]
    pub iirstg19b0: IIRSTGB0,
    #[doc = "0x664 - Stage %s Coefficient b1 Register"]
    pub iirstg19b1: IIRSTGB1,
    #[doc = "0x668 - Stage %s Coefficient b2 Register"]
    pub iirstg19b2: IIRSTGB2,
    #[doc = "0x66c - Stage %s Coefficient a1 Register"]
    pub iirstg19a1: IIRSTGA1,
    #[doc = "0x670 - Stage %s Coefficient a2 Register"]
    pub iirstg19a2: IIRSTGA2,
    #[doc = "0x674 - Stage %s Delay Data D0 Register"]
    pub iirstg19d0: IIRSTGD0,
    #[doc = "0x678 - Stage %s Delay Data D1 Register"]
    pub iirstg19d1: IIRSTGD1,
    _reserved247: [u8; 0x04],
    #[doc = "0x680 - Stage %s Coefficient b0 Register"]
    pub iirstg20b0: IIRSTGB0,
    #[doc = "0x684 - Stage %s Coefficient b1 Register"]
    pub iirstg20b1: IIRSTGB1,
    #[doc = "0x688 - Stage %s Coefficient b2 Register"]
    pub iirstg20b2: IIRSTGB2,
    #[doc = "0x68c - Stage %s Coefficient a1 Register"]
    pub iirstg20a1: IIRSTGA1,
    #[doc = "0x690 - Stage %s Coefficient a2 Register"]
    pub iirstg20a2: IIRSTGA2,
    #[doc = "0x694 - Stage %s Delay Data D0 Register"]
    pub iirstg20d0: IIRSTGD0,
    #[doc = "0x698 - Stage %s Delay Data D1 Register"]
    pub iirstg20d1: IIRSTGD1,
    _reserved254: [u8; 0x04],
    #[doc = "0x6a0 - Stage %s Coefficient b0 Register"]
    pub iirstg21b0: IIRSTGB0,
    #[doc = "0x6a4 - Stage %s Coefficient b1 Register"]
    pub iirstg21b1: IIRSTGB1,
    #[doc = "0x6a8 - Stage %s Coefficient b2 Register"]
    pub iirstg21b2: IIRSTGB2,
    #[doc = "0x6ac - Stage %s Coefficient a1 Register"]
    pub iirstg21a1: IIRSTGA1,
    #[doc = "0x6b0 - Stage %s Coefficient a2 Register"]
    pub iirstg21a2: IIRSTGA2,
    #[doc = "0x6b4 - Stage %s Delay Data D0 Register"]
    pub iirstg21d0: IIRSTGD0,
    #[doc = "0x6b8 - Stage %s Delay Data D1 Register"]
    pub iirstg21d1: IIRSTGD1,
    _reserved261: [u8; 0x04],
    #[doc = "0x6c0 - Stage %s Coefficient b0 Register"]
    pub iirstg22b0: IIRSTGB0,
    #[doc = "0x6c4 - Stage %s Coefficient b1 Register"]
    pub iirstg22b1: IIRSTGB1,
    #[doc = "0x6c8 - Stage %s Coefficient b2 Register"]
    pub iirstg22b2: IIRSTGB2,
    #[doc = "0x6cc - Stage %s Coefficient a1 Register"]
    pub iirstg22a1: IIRSTGA1,
    #[doc = "0x6d0 - Stage %s Coefficient a2 Register"]
    pub iirstg22a2: IIRSTGA2,
    #[doc = "0x6d4 - Stage %s Delay Data D0 Register"]
    pub iirstg22d0: IIRSTGD0,
    #[doc = "0x6d8 - Stage %s Delay Data D1 Register"]
    pub iirstg22d1: IIRSTGD1,
    _reserved268: [u8; 0x04],
    #[doc = "0x6e0 - Stage %s Coefficient b0 Register"]
    pub iirstg23b0: IIRSTGB0,
    #[doc = "0x6e4 - Stage %s Coefficient b1 Register"]
    pub iirstg23b1: IIRSTGB1,
    #[doc = "0x6e8 - Stage %s Coefficient b2 Register"]
    pub iirstg23b2: IIRSTGB2,
    #[doc = "0x6ec - Stage %s Coefficient a1 Register"]
    pub iirstg23a1: IIRSTGA1,
    #[doc = "0x6f0 - Stage %s Coefficient a2 Register"]
    pub iirstg23a2: IIRSTGA2,
    #[doc = "0x6f4 - Stage %s Delay Data D0 Register"]
    pub iirstg23d0: IIRSTGD0,
    #[doc = "0x6f8 - Stage %s Delay Data D1 Register"]
    pub iirstg23d1: IIRSTGD1,
    _reserved275: [u8; 0x04],
    #[doc = "0x700 - Stage %s Coefficient b0 Register"]
    pub iirstg24b0: IIRSTGB0,
    #[doc = "0x704 - Stage %s Coefficient b1 Register"]
    pub iirstg24b1: IIRSTGB1,
    #[doc = "0x708 - Stage %s Coefficient b2 Register"]
    pub iirstg24b2: IIRSTGB2,
    #[doc = "0x70c - Stage %s Coefficient a1 Register"]
    pub iirstg24a1: IIRSTGA1,
    #[doc = "0x710 - Stage %s Coefficient a2 Register"]
    pub iirstg24a2: IIRSTGA2,
    #[doc = "0x714 - Stage %s Delay Data D0 Register"]
    pub iirstg24d0: IIRSTGD0,
    #[doc = "0x718 - Stage %s Delay Data D1 Register"]
    pub iirstg24d1: IIRSTGD1,
    _reserved282: [u8; 0x04],
    #[doc = "0x720 - Stage %s Coefficient b0 Register"]
    pub iirstg25b0: IIRSTGB0,
    #[doc = "0x724 - Stage %s Coefficient b1 Register"]
    pub iirstg25b1: IIRSTGB1,
    #[doc = "0x728 - Stage %s Coefficient b2 Register"]
    pub iirstg25b2: IIRSTGB2,
    #[doc = "0x72c - Stage %s Coefficient a1 Register"]
    pub iirstg25a1: IIRSTGA1,
    #[doc = "0x730 - Stage %s Coefficient a2 Register"]
    pub iirstg25a2: IIRSTGA2,
    #[doc = "0x734 - Stage %s Delay Data D0 Register"]
    pub iirstg25d0: IIRSTGD0,
    #[doc = "0x738 - Stage %s Delay Data D1 Register"]
    pub iirstg25d1: IIRSTGD1,
    _reserved289: [u8; 0x04],
    #[doc = "0x740 - Stage %s Coefficient b0 Register"]
    pub iirstg26b0: IIRSTGB0,
    #[doc = "0x744 - Stage %s Coefficient b1 Register"]
    pub iirstg26b1: IIRSTGB1,
    #[doc = "0x748 - Stage %s Coefficient b2 Register"]
    pub iirstg26b2: IIRSTGB2,
    #[doc = "0x74c - Stage %s Coefficient a1 Register"]
    pub iirstg26a1: IIRSTGA1,
    #[doc = "0x750 - Stage %s Coefficient a2 Register"]
    pub iirstg26a2: IIRSTGA2,
    #[doc = "0x754 - Stage %s Delay Data D0 Register"]
    pub iirstg26d0: IIRSTGD0,
    #[doc = "0x758 - Stage %s Delay Data D1 Register"]
    pub iirstg26d1: IIRSTGD1,
    _reserved296: [u8; 0x04],
    #[doc = "0x760 - Stage %s Coefficient b0 Register"]
    pub iirstg27b0: IIRSTGB0,
    #[doc = "0x764 - Stage %s Coefficient b1 Register"]
    pub iirstg27b1: IIRSTGB1,
    #[doc = "0x768 - Stage %s Coefficient b2 Register"]
    pub iirstg27b2: IIRSTGB2,
    #[doc = "0x76c - Stage %s Coefficient a1 Register"]
    pub iirstg27a1: IIRSTGA1,
    #[doc = "0x770 - Stage %s Coefficient a2 Register"]
    pub iirstg27a2: IIRSTGA2,
    #[doc = "0x774 - Stage %s Delay Data D0 Register"]
    pub iirstg27d0: IIRSTGD0,
    #[doc = "0x778 - Stage %s Delay Data D1 Register"]
    pub iirstg27d1: IIRSTGD1,
    _reserved303: [u8; 0x04],
    #[doc = "0x780 - Stage %s Coefficient b0 Register"]
    pub iirstg28b0: IIRSTGB0,
    #[doc = "0x784 - Stage %s Coefficient b1 Register"]
    pub iirstg28b1: IIRSTGB1,
    #[doc = "0x788 - Stage %s Coefficient b2 Register"]
    pub iirstg28b2: IIRSTGB2,
    #[doc = "0x78c - Stage %s Coefficient a1 Register"]
    pub iirstg28a1: IIRSTGA1,
    #[doc = "0x790 - Stage %s Coefficient a2 Register"]
    pub iirstg28a2: IIRSTGA2,
    #[doc = "0x794 - Stage %s Delay Data D0 Register"]
    pub iirstg28d0: IIRSTGD0,
    #[doc = "0x798 - Stage %s Delay Data D1 Register"]
    pub iirstg28d1: IIRSTGD1,
    _reserved310: [u8; 0x04],
    #[doc = "0x7a0 - Stage %s Coefficient b0 Register"]
    pub iirstg29b0: IIRSTGB0,
    #[doc = "0x7a4 - Stage %s Coefficient b1 Register"]
    pub iirstg29b1: IIRSTGB1,
    #[doc = "0x7a8 - Stage %s Coefficient b2 Register"]
    pub iirstg29b2: IIRSTGB2,
    #[doc = "0x7ac - Stage %s Coefficient a1 Register"]
    pub iirstg29a1: IIRSTGA1,
    #[doc = "0x7b0 - Stage %s Coefficient a2 Register"]
    pub iirstg29a2: IIRSTGA2,
    #[doc = "0x7b4 - Stage %s Delay Data D0 Register"]
    pub iirstg29d0: IIRSTGD0,
    #[doc = "0x7b8 - Stage %s Delay Data D1 Register"]
    pub iirstg29d1: IIRSTGD1,
    _reserved317: [u8; 0x04],
    #[doc = "0x7c0 - Stage %s Coefficient b0 Register"]
    pub iirstg30b0: IIRSTGB0,
    #[doc = "0x7c4 - Stage %s Coefficient b1 Register"]
    pub iirstg30b1: IIRSTGB1,
    #[doc = "0x7c8 - Stage %s Coefficient b2 Register"]
    pub iirstg30b2: IIRSTGB2,
    #[doc = "0x7cc - Stage %s Coefficient a1 Register"]
    pub iirstg30a1: IIRSTGA1,
    #[doc = "0x7d0 - Stage %s Coefficient a2 Register"]
    pub iirstg30a2: IIRSTGA2,
    #[doc = "0x7d4 - Stage %s Delay Data D0 Register"]
    pub iirstg30d0: IIRSTGD0,
    #[doc = "0x7d8 - Stage %s Delay Data D1 Register"]
    pub iirstg30d1: IIRSTGD1,
    _reserved324: [u8; 0x04],
    #[doc = "0x7e0 - Stage %s Coefficient b0 Register"]
    pub iirstg31b0: IIRSTGB0,
    #[doc = "0x7e4 - Stage %s Coefficient b1 Register"]
    pub iirstg31b1: IIRSTGB1,
    #[doc = "0x7e8 - Stage %s Coefficient b2 Register"]
    pub iirstg31b2: IIRSTGB2,
    #[doc = "0x7ec - Stage %s Coefficient a1 Register"]
    pub iirstg31a1: IIRSTGA1,
    #[doc = "0x7f0 - Stage %s Coefficient a2 Register"]
    pub iirstg31a2: IIRSTGA2,
    #[doc = "0x7f4 - Stage %s Delay Data D0 Register"]
    pub iirstg31d0: IIRSTGD0,
    #[doc = "0x7f8 - Stage %s Delay Data D1 Register"]
    pub iirstg31d1: IIRSTGD1,
}
#[doc = "IIRCPRCS (r) register accessor: an alias for `Reg<IIRCPRCS_SPEC>`"]
pub type IIRCPRCS = crate::Reg<iircprcs::IIRCPRCS_SPEC>;
#[doc = "Channel Processing Status Register"]
pub mod iircprcs;
#[doc = "IIRCPRCFF (r) register accessor: an alias for `Reg<IIRCPRCFF_SPEC>`"]
pub type IIRCPRCFF = crate::Reg<iircprcff::IIRCPRCFF_SPEC>;
#[doc = "Channel Processing Completion Flag Register"]
pub mod iircprcff;
#[doc = "IIRORDYF (r) register accessor: an alias for `Reg<IIRORDYF_SPEC>`"]
pub type IIRORDYF = crate::Reg<iirordyf::IIRORDYF_SPEC>;
#[doc = "Output Data Preparation Completion Flag Register"]
pub mod iirordyf;
#[doc = "IIRCERRF (r) register accessor: an alias for `Reg<IIRCERRF_SPEC>`"]
pub type IIRCERRF = crate::Reg<iircerrf::IIRCERRF_SPEC>;
#[doc = "Operation Error Flag Register"]
pub mod iircerrf;
#[doc = "IIROPCNT (rw) register accessor: an alias for `Reg<IIROPCNT_SPEC>`"]
pub type IIROPCNT = crate::Reg<iiropcnt::IIROPCNT_SPEC>;
#[doc = "Operation Control Register"]
pub mod iiropcnt;
#[doc = "IIRECCCNT (rw) register accessor: an alias for `Reg<IIRECCCNT_SPEC>`"]
pub type IIRECCCNT = crate::Reg<iirecccnt::IIRECCCNT_SPEC>;
#[doc = "ECC Control Register"]
pub mod iirecccnt;
#[doc = "IIRECCINT (rw) register accessor: an alias for `Reg<IIRECCINT_SPEC>`"]
pub type IIRECCINT = crate::Reg<iireccint::IIRECCINT_SPEC>;
#[doc = "ECC Interrupt Enable Register"]
pub mod iireccint;
#[doc = "IIRECCEF (r) register accessor: an alias for `Reg<IIRECCEF_SPEC>`"]
pub type IIRECCEF = crate::Reg<iireccef::IIRECCEF_SPEC>;
#[doc = "ECC Error Flag Register"]
pub mod iireccef;
#[doc = "IIRECCEFCLR (w) register accessor: an alias for `Reg<IIRECCEFCLR_SPEC>`"]
pub type IIRECCEFCLR = crate::Reg<iireccefclr::IIRECCEFCLR_SPEC>;
#[doc = "ECC Error Flag Clear Register"]
pub mod iireccefclr;
#[doc = "IIRESEADR (r) register accessor: an alias for `Reg<IIRESEADR_SPEC>`"]
pub type IIRESEADR = crate::Reg<iireseadr::IIRESEADR_SPEC>;
#[doc = "ECC 1-bit Error Address Register"]
pub mod iireseadr;
#[doc = "IIREDEADR (r) register accessor: an alias for `Reg<IIREDEADR_SPEC>`"]
pub type IIREDEADR = crate::Reg<iiredeadr::IIREDEADR_SPEC>;
#[doc = "ECC 2-bit Error Address Register"]
pub mod iiredeadr;
#[doc = "IIRCHINP (w) register accessor: an alias for `Reg<IIRCHINP_SPEC>`"]
pub type IIRCHINP = crate::Reg<iirchinp::IIRCHINP_SPEC>;
#[doc = "Channel %s Input Register"]
pub mod iirchinp;
#[doc = "IIRCHOUT (r) register accessor: an alias for `Reg<IIRCHOUT_SPEC>`"]
pub type IIRCHOUT = crate::Reg<iirchout::IIRCHOUT_SPEC>;
#[doc = "Channel %s Output Register"]
pub mod iirchout;
#[doc = "IIRCHCNT (rw) register accessor: an alias for `Reg<IIRCHCNT_SPEC>`"]
pub type IIRCHCNT = crate::Reg<iirchcnt::IIRCHCNT_SPEC>;
#[doc = "Channel %s Control Register"]
pub mod iirchcnt;
#[doc = "IIRCHINT (rw) register accessor: an alias for `Reg<IIRCHINT_SPEC>`"]
pub type IIRCHINT = crate::Reg<iirchint::IIRCHINT_SPEC>;
#[doc = "Channel %s Interrupt Enable Register"]
pub mod iirchint;
#[doc = "IIRCHSTS (r) register accessor: an alias for `Reg<IIRCHSTS_SPEC>`"]
pub type IIRCHSTS = crate::Reg<iirchsts::IIRCHSTS_SPEC>;
#[doc = "Channel %s Status Register"]
pub mod iirchsts;
#[doc = "IIRCHFCLR (w) register accessor: an alias for `Reg<IIRCHFCLR_SPEC>`"]
pub type IIRCHFCLR = crate::Reg<iirchfclr::IIRCHFCLR_SPEC>;
#[doc = "Channel %s Flag Clear Register"]
pub mod iirchfclr;
#[doc = "IIRSTGB0 (rw) register accessor: an alias for `Reg<IIRSTGB0_SPEC>`"]
pub type IIRSTGB0 = crate::Reg<iirstgb0::IIRSTGB0_SPEC>;
#[doc = "Stage %s Coefficient b0 Register"]
pub mod iirstgb0;
#[doc = "IIRSTGB1 (rw) register accessor: an alias for `Reg<IIRSTGB1_SPEC>`"]
pub type IIRSTGB1 = crate::Reg<iirstgb1::IIRSTGB1_SPEC>;
#[doc = "Stage %s Coefficient b1 Register"]
pub mod iirstgb1;
#[doc = "IIRSTGB2 (rw) register accessor: an alias for `Reg<IIRSTGB2_SPEC>`"]
pub type IIRSTGB2 = crate::Reg<iirstgb2::IIRSTGB2_SPEC>;
#[doc = "Stage %s Coefficient b2 Register"]
pub mod iirstgb2;
#[doc = "IIRSTGA1 (rw) register accessor: an alias for `Reg<IIRSTGA1_SPEC>`"]
pub type IIRSTGA1 = crate::Reg<iirstga1::IIRSTGA1_SPEC>;
#[doc = "Stage %s Coefficient a1 Register"]
pub mod iirstga1;
#[doc = "IIRSTGA2 (rw) register accessor: an alias for `Reg<IIRSTGA2_SPEC>`"]
pub type IIRSTGA2 = crate::Reg<iirstga2::IIRSTGA2_SPEC>;
#[doc = "Stage %s Coefficient a2 Register"]
pub mod iirstga2;
#[doc = "IIRSTGD0 (rw) register accessor: an alias for `Reg<IIRSTGD0_SPEC>`"]
pub type IIRSTGD0 = crate::Reg<iirstgd0::IIRSTGD0_SPEC>;
#[doc = "Stage %s Delay Data D0 Register"]
pub mod iirstgd0;
#[doc = "IIRSTGD1 (rw) register accessor: an alias for `Reg<IIRSTGD1_SPEC>`"]
pub type IIRSTGD1 = crate::Reg<iirstgd1::IIRSTGD1_SPEC>;
#[doc = "Stage %s Delay Data D1 Register"]
pub mod iirstgd1;
