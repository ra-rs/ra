#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0200],
    #[doc = "0x200 - Mailbox ID Register %s"]
    pub mb0_id: MB_ID,
    #[doc = "0x204 - Mailbox Data Length Register %s"]
    pub mb0_dl: MB_DL,
    #[doc = "0x206 - Mailbox Data Register %s"]
    pub mb0_d0: MB_D0,
    #[doc = "0x207 - Mailbox Data Register %s"]
    pub mb0_d1: MB_D1,
    #[doc = "0x208 - Mailbox Data Register %s"]
    pub mb0_d2: MB_D2,
    #[doc = "0x209 - Mailbox Data Register %s"]
    pub mb0_d3: MB_D3,
    #[doc = "0x20a - Mailbox Data Register %s"]
    pub mb0_d4: MB_D4,
    #[doc = "0x20b - Mailbox Data Register %s"]
    pub mb0_d5: MB_D5,
    #[doc = "0x20c - Mailbox Data Register %s"]
    pub mb0_d6: MB_D6,
    #[doc = "0x20d - Mailbox Data Register %s"]
    pub mb0_d7: MB_D7,
    #[doc = "0x20e - Mailbox Time Stamp Register %s"]
    pub mb0_ts: MB_TS,
    #[doc = "0x210 - Mailbox ID Register %s"]
    pub mb1_id: MB_ID,
    #[doc = "0x214 - Mailbox Data Length Register %s"]
    pub mb1_dl: MB_DL,
    #[doc = "0x216 - Mailbox Data Register %s"]
    pub mb1_d0: MB_D0,
    #[doc = "0x217 - Mailbox Data Register %s"]
    pub mb1_d1: MB_D1,
    #[doc = "0x218 - Mailbox Data Register %s"]
    pub mb1_d2: MB_D2,
    #[doc = "0x219 - Mailbox Data Register %s"]
    pub mb1_d3: MB_D3,
    #[doc = "0x21a - Mailbox Data Register %s"]
    pub mb1_d4: MB_D4,
    #[doc = "0x21b - Mailbox Data Register %s"]
    pub mb1_d5: MB_D5,
    #[doc = "0x21c - Mailbox Data Register %s"]
    pub mb1_d6: MB_D6,
    #[doc = "0x21d - Mailbox Data Register %s"]
    pub mb1_d7: MB_D7,
    #[doc = "0x21e - Mailbox Time Stamp Register %s"]
    pub mb1_ts: MB_TS,
    #[doc = "0x220 - Mailbox ID Register %s"]
    pub mb2_id: MB_ID,
    #[doc = "0x224 - Mailbox Data Length Register %s"]
    pub mb2_dl: MB_DL,
    #[doc = "0x226 - Mailbox Data Register %s"]
    pub mb2_d0: MB_D0,
    #[doc = "0x227 - Mailbox Data Register %s"]
    pub mb2_d1: MB_D1,
    #[doc = "0x228 - Mailbox Data Register %s"]
    pub mb2_d2: MB_D2,
    #[doc = "0x229 - Mailbox Data Register %s"]
    pub mb2_d3: MB_D3,
    #[doc = "0x22a - Mailbox Data Register %s"]
    pub mb2_d4: MB_D4,
    #[doc = "0x22b - Mailbox Data Register %s"]
    pub mb2_d5: MB_D5,
    #[doc = "0x22c - Mailbox Data Register %s"]
    pub mb2_d6: MB_D6,
    #[doc = "0x22d - Mailbox Data Register %s"]
    pub mb2_d7: MB_D7,
    #[doc = "0x22e - Mailbox Time Stamp Register %s"]
    pub mb2_ts: MB_TS,
    #[doc = "0x230 - Mailbox ID Register %s"]
    pub mb3_id: MB_ID,
    #[doc = "0x234 - Mailbox Data Length Register %s"]
    pub mb3_dl: MB_DL,
    #[doc = "0x236 - Mailbox Data Register %s"]
    pub mb3_d0: MB_D0,
    #[doc = "0x237 - Mailbox Data Register %s"]
    pub mb3_d1: MB_D1,
    #[doc = "0x238 - Mailbox Data Register %s"]
    pub mb3_d2: MB_D2,
    #[doc = "0x239 - Mailbox Data Register %s"]
    pub mb3_d3: MB_D3,
    #[doc = "0x23a - Mailbox Data Register %s"]
    pub mb3_d4: MB_D4,
    #[doc = "0x23b - Mailbox Data Register %s"]
    pub mb3_d5: MB_D5,
    #[doc = "0x23c - Mailbox Data Register %s"]
    pub mb3_d6: MB_D6,
    #[doc = "0x23d - Mailbox Data Register %s"]
    pub mb3_d7: MB_D7,
    #[doc = "0x23e - Mailbox Time Stamp Register %s"]
    pub mb3_ts: MB_TS,
    #[doc = "0x240 - Mailbox ID Register %s"]
    pub mb4_id: MB_ID,
    #[doc = "0x244 - Mailbox Data Length Register %s"]
    pub mb4_dl: MB_DL,
    #[doc = "0x246 - Mailbox Data Register %s"]
    pub mb4_d0: MB_D0,
    #[doc = "0x247 - Mailbox Data Register %s"]
    pub mb4_d1: MB_D1,
    #[doc = "0x248 - Mailbox Data Register %s"]
    pub mb4_d2: MB_D2,
    #[doc = "0x249 - Mailbox Data Register %s"]
    pub mb4_d3: MB_D3,
    #[doc = "0x24a - Mailbox Data Register %s"]
    pub mb4_d4: MB_D4,
    #[doc = "0x24b - Mailbox Data Register %s"]
    pub mb4_d5: MB_D5,
    #[doc = "0x24c - Mailbox Data Register %s"]
    pub mb4_d6: MB_D6,
    #[doc = "0x24d - Mailbox Data Register %s"]
    pub mb4_d7: MB_D7,
    #[doc = "0x24e - Mailbox Time Stamp Register %s"]
    pub mb4_ts: MB_TS,
    #[doc = "0x250 - Mailbox ID Register %s"]
    pub mb5_id: MB_ID,
    #[doc = "0x254 - Mailbox Data Length Register %s"]
    pub mb5_dl: MB_DL,
    #[doc = "0x256 - Mailbox Data Register %s"]
    pub mb5_d0: MB_D0,
    #[doc = "0x257 - Mailbox Data Register %s"]
    pub mb5_d1: MB_D1,
    #[doc = "0x258 - Mailbox Data Register %s"]
    pub mb5_d2: MB_D2,
    #[doc = "0x259 - Mailbox Data Register %s"]
    pub mb5_d3: MB_D3,
    #[doc = "0x25a - Mailbox Data Register %s"]
    pub mb5_d4: MB_D4,
    #[doc = "0x25b - Mailbox Data Register %s"]
    pub mb5_d5: MB_D5,
    #[doc = "0x25c - Mailbox Data Register %s"]
    pub mb5_d6: MB_D6,
    #[doc = "0x25d - Mailbox Data Register %s"]
    pub mb5_d7: MB_D7,
    #[doc = "0x25e - Mailbox Time Stamp Register %s"]
    pub mb5_ts: MB_TS,
    #[doc = "0x260 - Mailbox ID Register %s"]
    pub mb6_id: MB_ID,
    #[doc = "0x264 - Mailbox Data Length Register %s"]
    pub mb6_dl: MB_DL,
    #[doc = "0x266 - Mailbox Data Register %s"]
    pub mb6_d0: MB_D0,
    #[doc = "0x267 - Mailbox Data Register %s"]
    pub mb6_d1: MB_D1,
    #[doc = "0x268 - Mailbox Data Register %s"]
    pub mb6_d2: MB_D2,
    #[doc = "0x269 - Mailbox Data Register %s"]
    pub mb6_d3: MB_D3,
    #[doc = "0x26a - Mailbox Data Register %s"]
    pub mb6_d4: MB_D4,
    #[doc = "0x26b - Mailbox Data Register %s"]
    pub mb6_d5: MB_D5,
    #[doc = "0x26c - Mailbox Data Register %s"]
    pub mb6_d6: MB_D6,
    #[doc = "0x26d - Mailbox Data Register %s"]
    pub mb6_d7: MB_D7,
    #[doc = "0x26e - Mailbox Time Stamp Register %s"]
    pub mb6_ts: MB_TS,
    #[doc = "0x270 - Mailbox ID Register %s"]
    pub mb7_id: MB_ID,
    #[doc = "0x274 - Mailbox Data Length Register %s"]
    pub mb7_dl: MB_DL,
    #[doc = "0x276 - Mailbox Data Register %s"]
    pub mb7_d0: MB_D0,
    #[doc = "0x277 - Mailbox Data Register %s"]
    pub mb7_d1: MB_D1,
    #[doc = "0x278 - Mailbox Data Register %s"]
    pub mb7_d2: MB_D2,
    #[doc = "0x279 - Mailbox Data Register %s"]
    pub mb7_d3: MB_D3,
    #[doc = "0x27a - Mailbox Data Register %s"]
    pub mb7_d4: MB_D4,
    #[doc = "0x27b - Mailbox Data Register %s"]
    pub mb7_d5: MB_D5,
    #[doc = "0x27c - Mailbox Data Register %s"]
    pub mb7_d6: MB_D6,
    #[doc = "0x27d - Mailbox Data Register %s"]
    pub mb7_d7: MB_D7,
    #[doc = "0x27e - Mailbox Time Stamp Register %s"]
    pub mb7_ts: MB_TS,
    #[doc = "0x280 - Mailbox ID Register %s"]
    pub mb8_id: MB_ID,
    #[doc = "0x284 - Mailbox Data Length Register %s"]
    pub mb8_dl: MB_DL,
    #[doc = "0x286 - Mailbox Data Register %s"]
    pub mb8_d0: MB_D0,
    #[doc = "0x287 - Mailbox Data Register %s"]
    pub mb8_d1: MB_D1,
    #[doc = "0x288 - Mailbox Data Register %s"]
    pub mb8_d2: MB_D2,
    #[doc = "0x289 - Mailbox Data Register %s"]
    pub mb8_d3: MB_D3,
    #[doc = "0x28a - Mailbox Data Register %s"]
    pub mb8_d4: MB_D4,
    #[doc = "0x28b - Mailbox Data Register %s"]
    pub mb8_d5: MB_D5,
    #[doc = "0x28c - Mailbox Data Register %s"]
    pub mb8_d6: MB_D6,
    #[doc = "0x28d - Mailbox Data Register %s"]
    pub mb8_d7: MB_D7,
    #[doc = "0x28e - Mailbox Time Stamp Register %s"]
    pub mb8_ts: MB_TS,
    #[doc = "0x290 - Mailbox ID Register %s"]
    pub mb9_id: MB_ID,
    #[doc = "0x294 - Mailbox Data Length Register %s"]
    pub mb9_dl: MB_DL,
    #[doc = "0x296 - Mailbox Data Register %s"]
    pub mb9_d0: MB_D0,
    #[doc = "0x297 - Mailbox Data Register %s"]
    pub mb9_d1: MB_D1,
    #[doc = "0x298 - Mailbox Data Register %s"]
    pub mb9_d2: MB_D2,
    #[doc = "0x299 - Mailbox Data Register %s"]
    pub mb9_d3: MB_D3,
    #[doc = "0x29a - Mailbox Data Register %s"]
    pub mb9_d4: MB_D4,
    #[doc = "0x29b - Mailbox Data Register %s"]
    pub mb9_d5: MB_D5,
    #[doc = "0x29c - Mailbox Data Register %s"]
    pub mb9_d6: MB_D6,
    #[doc = "0x29d - Mailbox Data Register %s"]
    pub mb9_d7: MB_D7,
    #[doc = "0x29e - Mailbox Time Stamp Register %s"]
    pub mb9_ts: MB_TS,
    #[doc = "0x2a0 - Mailbox ID Register %s"]
    pub mb10_id: MB_ID,
    #[doc = "0x2a4 - Mailbox Data Length Register %s"]
    pub mb10_dl: MB_DL,
    #[doc = "0x2a6 - Mailbox Data Register %s"]
    pub mb10_d0: MB_D0,
    #[doc = "0x2a7 - Mailbox Data Register %s"]
    pub mb10_d1: MB_D1,
    #[doc = "0x2a8 - Mailbox Data Register %s"]
    pub mb10_d2: MB_D2,
    #[doc = "0x2a9 - Mailbox Data Register %s"]
    pub mb10_d3: MB_D3,
    #[doc = "0x2aa - Mailbox Data Register %s"]
    pub mb10_d4: MB_D4,
    #[doc = "0x2ab - Mailbox Data Register %s"]
    pub mb10_d5: MB_D5,
    #[doc = "0x2ac - Mailbox Data Register %s"]
    pub mb10_d6: MB_D6,
    #[doc = "0x2ad - Mailbox Data Register %s"]
    pub mb10_d7: MB_D7,
    #[doc = "0x2ae - Mailbox Time Stamp Register %s"]
    pub mb10_ts: MB_TS,
    #[doc = "0x2b0 - Mailbox ID Register %s"]
    pub mb11_id: MB_ID,
    #[doc = "0x2b4 - Mailbox Data Length Register %s"]
    pub mb11_dl: MB_DL,
    #[doc = "0x2b6 - Mailbox Data Register %s"]
    pub mb11_d0: MB_D0,
    #[doc = "0x2b7 - Mailbox Data Register %s"]
    pub mb11_d1: MB_D1,
    #[doc = "0x2b8 - Mailbox Data Register %s"]
    pub mb11_d2: MB_D2,
    #[doc = "0x2b9 - Mailbox Data Register %s"]
    pub mb11_d3: MB_D3,
    #[doc = "0x2ba - Mailbox Data Register %s"]
    pub mb11_d4: MB_D4,
    #[doc = "0x2bb - Mailbox Data Register %s"]
    pub mb11_d5: MB_D5,
    #[doc = "0x2bc - Mailbox Data Register %s"]
    pub mb11_d6: MB_D6,
    #[doc = "0x2bd - Mailbox Data Register %s"]
    pub mb11_d7: MB_D7,
    #[doc = "0x2be - Mailbox Time Stamp Register %s"]
    pub mb11_ts: MB_TS,
    #[doc = "0x2c0 - Mailbox ID Register %s"]
    pub mb12_id: MB_ID,
    #[doc = "0x2c4 - Mailbox Data Length Register %s"]
    pub mb12_dl: MB_DL,
    #[doc = "0x2c6 - Mailbox Data Register %s"]
    pub mb12_d0: MB_D0,
    #[doc = "0x2c7 - Mailbox Data Register %s"]
    pub mb12_d1: MB_D1,
    #[doc = "0x2c8 - Mailbox Data Register %s"]
    pub mb12_d2: MB_D2,
    #[doc = "0x2c9 - Mailbox Data Register %s"]
    pub mb12_d3: MB_D3,
    #[doc = "0x2ca - Mailbox Data Register %s"]
    pub mb12_d4: MB_D4,
    #[doc = "0x2cb - Mailbox Data Register %s"]
    pub mb12_d5: MB_D5,
    #[doc = "0x2cc - Mailbox Data Register %s"]
    pub mb12_d6: MB_D6,
    #[doc = "0x2cd - Mailbox Data Register %s"]
    pub mb12_d7: MB_D7,
    #[doc = "0x2ce - Mailbox Time Stamp Register %s"]
    pub mb12_ts: MB_TS,
    #[doc = "0x2d0 - Mailbox ID Register %s"]
    pub mb13_id: MB_ID,
    #[doc = "0x2d4 - Mailbox Data Length Register %s"]
    pub mb13_dl: MB_DL,
    #[doc = "0x2d6 - Mailbox Data Register %s"]
    pub mb13_d0: MB_D0,
    #[doc = "0x2d7 - Mailbox Data Register %s"]
    pub mb13_d1: MB_D1,
    #[doc = "0x2d8 - Mailbox Data Register %s"]
    pub mb13_d2: MB_D2,
    #[doc = "0x2d9 - Mailbox Data Register %s"]
    pub mb13_d3: MB_D3,
    #[doc = "0x2da - Mailbox Data Register %s"]
    pub mb13_d4: MB_D4,
    #[doc = "0x2db - Mailbox Data Register %s"]
    pub mb13_d5: MB_D5,
    #[doc = "0x2dc - Mailbox Data Register %s"]
    pub mb13_d6: MB_D6,
    #[doc = "0x2dd - Mailbox Data Register %s"]
    pub mb13_d7: MB_D7,
    #[doc = "0x2de - Mailbox Time Stamp Register %s"]
    pub mb13_ts: MB_TS,
    #[doc = "0x2e0 - Mailbox ID Register %s"]
    pub mb14_id: MB_ID,
    #[doc = "0x2e4 - Mailbox Data Length Register %s"]
    pub mb14_dl: MB_DL,
    #[doc = "0x2e6 - Mailbox Data Register %s"]
    pub mb14_d0: MB_D0,
    #[doc = "0x2e7 - Mailbox Data Register %s"]
    pub mb14_d1: MB_D1,
    #[doc = "0x2e8 - Mailbox Data Register %s"]
    pub mb14_d2: MB_D2,
    #[doc = "0x2e9 - Mailbox Data Register %s"]
    pub mb14_d3: MB_D3,
    #[doc = "0x2ea - Mailbox Data Register %s"]
    pub mb14_d4: MB_D4,
    #[doc = "0x2eb - Mailbox Data Register %s"]
    pub mb14_d5: MB_D5,
    #[doc = "0x2ec - Mailbox Data Register %s"]
    pub mb14_d6: MB_D6,
    #[doc = "0x2ed - Mailbox Data Register %s"]
    pub mb14_d7: MB_D7,
    #[doc = "0x2ee - Mailbox Time Stamp Register %s"]
    pub mb14_ts: MB_TS,
    #[doc = "0x2f0 - Mailbox ID Register %s"]
    pub mb15_id: MB_ID,
    #[doc = "0x2f4 - Mailbox Data Length Register %s"]
    pub mb15_dl: MB_DL,
    #[doc = "0x2f6 - Mailbox Data Register %s"]
    pub mb15_d0: MB_D0,
    #[doc = "0x2f7 - Mailbox Data Register %s"]
    pub mb15_d1: MB_D1,
    #[doc = "0x2f8 - Mailbox Data Register %s"]
    pub mb15_d2: MB_D2,
    #[doc = "0x2f9 - Mailbox Data Register %s"]
    pub mb15_d3: MB_D3,
    #[doc = "0x2fa - Mailbox Data Register %s"]
    pub mb15_d4: MB_D4,
    #[doc = "0x2fb - Mailbox Data Register %s"]
    pub mb15_d5: MB_D5,
    #[doc = "0x2fc - Mailbox Data Register %s"]
    pub mb15_d6: MB_D6,
    #[doc = "0x2fd - Mailbox Data Register %s"]
    pub mb15_d7: MB_D7,
    #[doc = "0x2fe - Mailbox Time Stamp Register %s"]
    pub mb15_ts: MB_TS,
    #[doc = "0x300 - Mailbox ID Register %s"]
    pub mb16_id: MB_ID,
    #[doc = "0x304 - Mailbox Data Length Register %s"]
    pub mb16_dl: MB_DL,
    #[doc = "0x306 - Mailbox Data Register %s"]
    pub mb16_d0: MB_D0,
    #[doc = "0x307 - Mailbox Data Register %s"]
    pub mb16_d1: MB_D1,
    #[doc = "0x308 - Mailbox Data Register %s"]
    pub mb16_d2: MB_D2,
    #[doc = "0x309 - Mailbox Data Register %s"]
    pub mb16_d3: MB_D3,
    #[doc = "0x30a - Mailbox Data Register %s"]
    pub mb16_d4: MB_D4,
    #[doc = "0x30b - Mailbox Data Register %s"]
    pub mb16_d5: MB_D5,
    #[doc = "0x30c - Mailbox Data Register %s"]
    pub mb16_d6: MB_D6,
    #[doc = "0x30d - Mailbox Data Register %s"]
    pub mb16_d7: MB_D7,
    #[doc = "0x30e - Mailbox Time Stamp Register %s"]
    pub mb16_ts: MB_TS,
    #[doc = "0x310 - Mailbox ID Register %s"]
    pub mb17_id: MB_ID,
    #[doc = "0x314 - Mailbox Data Length Register %s"]
    pub mb17_dl: MB_DL,
    #[doc = "0x316 - Mailbox Data Register %s"]
    pub mb17_d0: MB_D0,
    #[doc = "0x317 - Mailbox Data Register %s"]
    pub mb17_d1: MB_D1,
    #[doc = "0x318 - Mailbox Data Register %s"]
    pub mb17_d2: MB_D2,
    #[doc = "0x319 - Mailbox Data Register %s"]
    pub mb17_d3: MB_D3,
    #[doc = "0x31a - Mailbox Data Register %s"]
    pub mb17_d4: MB_D4,
    #[doc = "0x31b - Mailbox Data Register %s"]
    pub mb17_d5: MB_D5,
    #[doc = "0x31c - Mailbox Data Register %s"]
    pub mb17_d6: MB_D6,
    #[doc = "0x31d - Mailbox Data Register %s"]
    pub mb17_d7: MB_D7,
    #[doc = "0x31e - Mailbox Time Stamp Register %s"]
    pub mb17_ts: MB_TS,
    #[doc = "0x320 - Mailbox ID Register %s"]
    pub mb18_id: MB_ID,
    #[doc = "0x324 - Mailbox Data Length Register %s"]
    pub mb18_dl: MB_DL,
    #[doc = "0x326 - Mailbox Data Register %s"]
    pub mb18_d0: MB_D0,
    #[doc = "0x327 - Mailbox Data Register %s"]
    pub mb18_d1: MB_D1,
    #[doc = "0x328 - Mailbox Data Register %s"]
    pub mb18_d2: MB_D2,
    #[doc = "0x329 - Mailbox Data Register %s"]
    pub mb18_d3: MB_D3,
    #[doc = "0x32a - Mailbox Data Register %s"]
    pub mb18_d4: MB_D4,
    #[doc = "0x32b - Mailbox Data Register %s"]
    pub mb18_d5: MB_D5,
    #[doc = "0x32c - Mailbox Data Register %s"]
    pub mb18_d6: MB_D6,
    #[doc = "0x32d - Mailbox Data Register %s"]
    pub mb18_d7: MB_D7,
    #[doc = "0x32e - Mailbox Time Stamp Register %s"]
    pub mb18_ts: MB_TS,
    #[doc = "0x330 - Mailbox ID Register %s"]
    pub mb19_id: MB_ID,
    #[doc = "0x334 - Mailbox Data Length Register %s"]
    pub mb19_dl: MB_DL,
    #[doc = "0x336 - Mailbox Data Register %s"]
    pub mb19_d0: MB_D0,
    #[doc = "0x337 - Mailbox Data Register %s"]
    pub mb19_d1: MB_D1,
    #[doc = "0x338 - Mailbox Data Register %s"]
    pub mb19_d2: MB_D2,
    #[doc = "0x339 - Mailbox Data Register %s"]
    pub mb19_d3: MB_D3,
    #[doc = "0x33a - Mailbox Data Register %s"]
    pub mb19_d4: MB_D4,
    #[doc = "0x33b - Mailbox Data Register %s"]
    pub mb19_d5: MB_D5,
    #[doc = "0x33c - Mailbox Data Register %s"]
    pub mb19_d6: MB_D6,
    #[doc = "0x33d - Mailbox Data Register %s"]
    pub mb19_d7: MB_D7,
    #[doc = "0x33e - Mailbox Time Stamp Register %s"]
    pub mb19_ts: MB_TS,
    #[doc = "0x340 - Mailbox ID Register %s"]
    pub mb20_id: MB_ID,
    #[doc = "0x344 - Mailbox Data Length Register %s"]
    pub mb20_dl: MB_DL,
    #[doc = "0x346 - Mailbox Data Register %s"]
    pub mb20_d0: MB_D0,
    #[doc = "0x347 - Mailbox Data Register %s"]
    pub mb20_d1: MB_D1,
    #[doc = "0x348 - Mailbox Data Register %s"]
    pub mb20_d2: MB_D2,
    #[doc = "0x349 - Mailbox Data Register %s"]
    pub mb20_d3: MB_D3,
    #[doc = "0x34a - Mailbox Data Register %s"]
    pub mb20_d4: MB_D4,
    #[doc = "0x34b - Mailbox Data Register %s"]
    pub mb20_d5: MB_D5,
    #[doc = "0x34c - Mailbox Data Register %s"]
    pub mb20_d6: MB_D6,
    #[doc = "0x34d - Mailbox Data Register %s"]
    pub mb20_d7: MB_D7,
    #[doc = "0x34e - Mailbox Time Stamp Register %s"]
    pub mb20_ts: MB_TS,
    #[doc = "0x350 - Mailbox ID Register %s"]
    pub mb21_id: MB_ID,
    #[doc = "0x354 - Mailbox Data Length Register %s"]
    pub mb21_dl: MB_DL,
    #[doc = "0x356 - Mailbox Data Register %s"]
    pub mb21_d0: MB_D0,
    #[doc = "0x357 - Mailbox Data Register %s"]
    pub mb21_d1: MB_D1,
    #[doc = "0x358 - Mailbox Data Register %s"]
    pub mb21_d2: MB_D2,
    #[doc = "0x359 - Mailbox Data Register %s"]
    pub mb21_d3: MB_D3,
    #[doc = "0x35a - Mailbox Data Register %s"]
    pub mb21_d4: MB_D4,
    #[doc = "0x35b - Mailbox Data Register %s"]
    pub mb21_d5: MB_D5,
    #[doc = "0x35c - Mailbox Data Register %s"]
    pub mb21_d6: MB_D6,
    #[doc = "0x35d - Mailbox Data Register %s"]
    pub mb21_d7: MB_D7,
    #[doc = "0x35e - Mailbox Time Stamp Register %s"]
    pub mb21_ts: MB_TS,
    #[doc = "0x360 - Mailbox ID Register %s"]
    pub mb22_id: MB_ID,
    #[doc = "0x364 - Mailbox Data Length Register %s"]
    pub mb22_dl: MB_DL,
    #[doc = "0x366 - Mailbox Data Register %s"]
    pub mb22_d0: MB_D0,
    #[doc = "0x367 - Mailbox Data Register %s"]
    pub mb22_d1: MB_D1,
    #[doc = "0x368 - Mailbox Data Register %s"]
    pub mb22_d2: MB_D2,
    #[doc = "0x369 - Mailbox Data Register %s"]
    pub mb22_d3: MB_D3,
    #[doc = "0x36a - Mailbox Data Register %s"]
    pub mb22_d4: MB_D4,
    #[doc = "0x36b - Mailbox Data Register %s"]
    pub mb22_d5: MB_D5,
    #[doc = "0x36c - Mailbox Data Register %s"]
    pub mb22_d6: MB_D6,
    #[doc = "0x36d - Mailbox Data Register %s"]
    pub mb22_d7: MB_D7,
    #[doc = "0x36e - Mailbox Time Stamp Register %s"]
    pub mb22_ts: MB_TS,
    #[doc = "0x370 - Mailbox ID Register %s"]
    pub mb23_id: MB_ID,
    #[doc = "0x374 - Mailbox Data Length Register %s"]
    pub mb23_dl: MB_DL,
    #[doc = "0x376 - Mailbox Data Register %s"]
    pub mb23_d0: MB_D0,
    #[doc = "0x377 - Mailbox Data Register %s"]
    pub mb23_d1: MB_D1,
    #[doc = "0x378 - Mailbox Data Register %s"]
    pub mb23_d2: MB_D2,
    #[doc = "0x379 - Mailbox Data Register %s"]
    pub mb23_d3: MB_D3,
    #[doc = "0x37a - Mailbox Data Register %s"]
    pub mb23_d4: MB_D4,
    #[doc = "0x37b - Mailbox Data Register %s"]
    pub mb23_d5: MB_D5,
    #[doc = "0x37c - Mailbox Data Register %s"]
    pub mb23_d6: MB_D6,
    #[doc = "0x37d - Mailbox Data Register %s"]
    pub mb23_d7: MB_D7,
    #[doc = "0x37e - Mailbox Time Stamp Register %s"]
    pub mb23_ts: MB_TS,
    #[doc = "0x380 - Mailbox ID Register %s"]
    pub mb24_id: MB_ID,
    #[doc = "0x384 - Mailbox Data Length Register %s"]
    pub mb24_dl: MB_DL,
    #[doc = "0x386 - Mailbox Data Register %s"]
    pub mb24_d0: MB_D0,
    #[doc = "0x387 - Mailbox Data Register %s"]
    pub mb24_d1: MB_D1,
    #[doc = "0x388 - Mailbox Data Register %s"]
    pub mb24_d2: MB_D2,
    #[doc = "0x389 - Mailbox Data Register %s"]
    pub mb24_d3: MB_D3,
    #[doc = "0x38a - Mailbox Data Register %s"]
    pub mb24_d4: MB_D4,
    #[doc = "0x38b - Mailbox Data Register %s"]
    pub mb24_d5: MB_D5,
    #[doc = "0x38c - Mailbox Data Register %s"]
    pub mb24_d6: MB_D6,
    #[doc = "0x38d - Mailbox Data Register %s"]
    pub mb24_d7: MB_D7,
    #[doc = "0x38e - Mailbox Time Stamp Register %s"]
    pub mb24_ts: MB_TS,
    #[doc = "0x390 - Mailbox ID Register %s"]
    pub mb25_id: MB_ID,
    #[doc = "0x394 - Mailbox Data Length Register %s"]
    pub mb25_dl: MB_DL,
    #[doc = "0x396 - Mailbox Data Register %s"]
    pub mb25_d0: MB_D0,
    #[doc = "0x397 - Mailbox Data Register %s"]
    pub mb25_d1: MB_D1,
    #[doc = "0x398 - Mailbox Data Register %s"]
    pub mb25_d2: MB_D2,
    #[doc = "0x399 - Mailbox Data Register %s"]
    pub mb25_d3: MB_D3,
    #[doc = "0x39a - Mailbox Data Register %s"]
    pub mb25_d4: MB_D4,
    #[doc = "0x39b - Mailbox Data Register %s"]
    pub mb25_d5: MB_D5,
    #[doc = "0x39c - Mailbox Data Register %s"]
    pub mb25_d6: MB_D6,
    #[doc = "0x39d - Mailbox Data Register %s"]
    pub mb25_d7: MB_D7,
    #[doc = "0x39e - Mailbox Time Stamp Register %s"]
    pub mb25_ts: MB_TS,
    #[doc = "0x3a0 - Mailbox ID Register %s"]
    pub mb26_id: MB_ID,
    #[doc = "0x3a4 - Mailbox Data Length Register %s"]
    pub mb26_dl: MB_DL,
    #[doc = "0x3a6 - Mailbox Data Register %s"]
    pub mb26_d0: MB_D0,
    #[doc = "0x3a7 - Mailbox Data Register %s"]
    pub mb26_d1: MB_D1,
    #[doc = "0x3a8 - Mailbox Data Register %s"]
    pub mb26_d2: MB_D2,
    #[doc = "0x3a9 - Mailbox Data Register %s"]
    pub mb26_d3: MB_D3,
    #[doc = "0x3aa - Mailbox Data Register %s"]
    pub mb26_d4: MB_D4,
    #[doc = "0x3ab - Mailbox Data Register %s"]
    pub mb26_d5: MB_D5,
    #[doc = "0x3ac - Mailbox Data Register %s"]
    pub mb26_d6: MB_D6,
    #[doc = "0x3ad - Mailbox Data Register %s"]
    pub mb26_d7: MB_D7,
    #[doc = "0x3ae - Mailbox Time Stamp Register %s"]
    pub mb26_ts: MB_TS,
    #[doc = "0x3b0 - Mailbox ID Register %s"]
    pub mb27_id: MB_ID,
    #[doc = "0x3b4 - Mailbox Data Length Register %s"]
    pub mb27_dl: MB_DL,
    #[doc = "0x3b6 - Mailbox Data Register %s"]
    pub mb27_d0: MB_D0,
    #[doc = "0x3b7 - Mailbox Data Register %s"]
    pub mb27_d1: MB_D1,
    #[doc = "0x3b8 - Mailbox Data Register %s"]
    pub mb27_d2: MB_D2,
    #[doc = "0x3b9 - Mailbox Data Register %s"]
    pub mb27_d3: MB_D3,
    #[doc = "0x3ba - Mailbox Data Register %s"]
    pub mb27_d4: MB_D4,
    #[doc = "0x3bb - Mailbox Data Register %s"]
    pub mb27_d5: MB_D5,
    #[doc = "0x3bc - Mailbox Data Register %s"]
    pub mb27_d6: MB_D6,
    #[doc = "0x3bd - Mailbox Data Register %s"]
    pub mb27_d7: MB_D7,
    #[doc = "0x3be - Mailbox Time Stamp Register %s"]
    pub mb27_ts: MB_TS,
    #[doc = "0x3c0 - Mailbox ID Register %s"]
    pub mb28_id: MB_ID,
    #[doc = "0x3c4 - Mailbox Data Length Register %s"]
    pub mb28_dl: MB_DL,
    #[doc = "0x3c6 - Mailbox Data Register %s"]
    pub mb28_d0: MB_D0,
    #[doc = "0x3c7 - Mailbox Data Register %s"]
    pub mb28_d1: MB_D1,
    #[doc = "0x3c8 - Mailbox Data Register %s"]
    pub mb28_d2: MB_D2,
    #[doc = "0x3c9 - Mailbox Data Register %s"]
    pub mb28_d3: MB_D3,
    #[doc = "0x3ca - Mailbox Data Register %s"]
    pub mb28_d4: MB_D4,
    #[doc = "0x3cb - Mailbox Data Register %s"]
    pub mb28_d5: MB_D5,
    #[doc = "0x3cc - Mailbox Data Register %s"]
    pub mb28_d6: MB_D6,
    #[doc = "0x3cd - Mailbox Data Register %s"]
    pub mb28_d7: MB_D7,
    #[doc = "0x3ce - Mailbox Time Stamp Register %s"]
    pub mb28_ts: MB_TS,
    #[doc = "0x3d0 - Mailbox ID Register %s"]
    pub mb29_id: MB_ID,
    #[doc = "0x3d4 - Mailbox Data Length Register %s"]
    pub mb29_dl: MB_DL,
    #[doc = "0x3d6 - Mailbox Data Register %s"]
    pub mb29_d0: MB_D0,
    #[doc = "0x3d7 - Mailbox Data Register %s"]
    pub mb29_d1: MB_D1,
    #[doc = "0x3d8 - Mailbox Data Register %s"]
    pub mb29_d2: MB_D2,
    #[doc = "0x3d9 - Mailbox Data Register %s"]
    pub mb29_d3: MB_D3,
    #[doc = "0x3da - Mailbox Data Register %s"]
    pub mb29_d4: MB_D4,
    #[doc = "0x3db - Mailbox Data Register %s"]
    pub mb29_d5: MB_D5,
    #[doc = "0x3dc - Mailbox Data Register %s"]
    pub mb29_d6: MB_D6,
    #[doc = "0x3dd - Mailbox Data Register %s"]
    pub mb29_d7: MB_D7,
    #[doc = "0x3de - Mailbox Time Stamp Register %s"]
    pub mb29_ts: MB_TS,
    #[doc = "0x3e0 - Mailbox ID Register %s"]
    pub mb30_id: MB_ID,
    #[doc = "0x3e4 - Mailbox Data Length Register %s"]
    pub mb30_dl: MB_DL,
    #[doc = "0x3e6 - Mailbox Data Register %s"]
    pub mb30_d0: MB_D0,
    #[doc = "0x3e7 - Mailbox Data Register %s"]
    pub mb30_d1: MB_D1,
    #[doc = "0x3e8 - Mailbox Data Register %s"]
    pub mb30_d2: MB_D2,
    #[doc = "0x3e9 - Mailbox Data Register %s"]
    pub mb30_d3: MB_D3,
    #[doc = "0x3ea - Mailbox Data Register %s"]
    pub mb30_d4: MB_D4,
    #[doc = "0x3eb - Mailbox Data Register %s"]
    pub mb30_d5: MB_D5,
    #[doc = "0x3ec - Mailbox Data Register %s"]
    pub mb30_d6: MB_D6,
    #[doc = "0x3ed - Mailbox Data Register %s"]
    pub mb30_d7: MB_D7,
    #[doc = "0x3ee - Mailbox Time Stamp Register %s"]
    pub mb30_ts: MB_TS,
    #[doc = "0x3f0 - Mailbox ID Register %s"]
    pub mb31_id: MB_ID,
    #[doc = "0x3f4 - Mailbox Data Length Register %s"]
    pub mb31_dl: MB_DL,
    #[doc = "0x3f6 - Mailbox Data Register %s"]
    pub mb31_d0: MB_D0,
    #[doc = "0x3f7 - Mailbox Data Register %s"]
    pub mb31_d1: MB_D1,
    #[doc = "0x3f8 - Mailbox Data Register %s"]
    pub mb31_d2: MB_D2,
    #[doc = "0x3f9 - Mailbox Data Register %s"]
    pub mb31_d3: MB_D3,
    #[doc = "0x3fa - Mailbox Data Register %s"]
    pub mb31_d4: MB_D4,
    #[doc = "0x3fb - Mailbox Data Register %s"]
    pub mb31_d5: MB_D5,
    #[doc = "0x3fc - Mailbox Data Register %s"]
    pub mb31_d6: MB_D6,
    #[doc = "0x3fd - Mailbox Data Register %s"]
    pub mb31_d7: MB_D7,
    #[doc = "0x3fe - Mailbox Time Stamp Register %s"]
    pub mb31_ts: MB_TS,
    #[doc = "0x400..0x420 - Mask Register %s"]
    pub mkr: [MKR; 8],
    #[doc = "0x420..0x428 - FIFO Received ID Compare Register %s"]
    pub fidcr: [FIDCR; 2],
    #[doc = "0x428 - Mask Invalid Register"]
    pub mkivlr: MKIVLR,
    _reserved_355_mier: [u8; 0x04],
    _reserved356: [u8; 0x03f0],
    _reserved_356_mctl: [u8; 0x20],
    #[doc = "0x840 - Control Register"]
    pub ctlr: CTLR,
    #[doc = "0x842 - Status Register"]
    pub str: STR,
    #[doc = "0x844 - Bit Configuration Register"]
    pub bcr: BCR,
    #[doc = "0x848 - Receive FIFO Control Register"]
    pub rfcr: RFCR,
    #[doc = "0x849 - Receive FIFO Pointer Control Register"]
    pub rfpcr: RFPCR,
    #[doc = "0x84a - Transmit FIFO Control Register"]
    pub tfcr: TFCR,
    #[doc = "0x84b - Transmit FIFO Pointer Control Register"]
    pub tfpcr: TFPCR,
    #[doc = "0x84c - Error Interrupt Enable Register"]
    pub eier: EIER,
    #[doc = "0x84d - Error Interrupt Factor Judge Register"]
    pub eifr: EIFR,
    #[doc = "0x84e - Receive Error Count Register"]
    pub recr: RECR,
    #[doc = "0x84f - Transmit Error Count Register"]
    pub tecr: TECR,
    #[doc = "0x850 - Error Code Store Register"]
    pub ecsr: ECSR,
    #[doc = "0x851 - Channel Search Support Register"]
    pub cssr: CSSR,
    #[doc = "0x852 - Mailbox Search Status Register"]
    pub mssr: MSSR,
    #[doc = "0x853 - Mailbox Search Mode Register"]
    pub msmr: MSMR,
    #[doc = "0x854 - Time Stamp Register"]
    pub tsr: TSR,
    #[doc = "0x856 - Acceptance Filter Support Register"]
    pub afsr: AFSR,
    #[doc = "0x858 - Test Control Register"]
    pub tcr: TCR,
}
impl RegisterBlock {
    #[doc = "0x42c - Mailbox Interrupt Enable Register for FIFO Mailbox Mode"]
    #[inline(always)]
    pub const fn mier_fifo(&self) -> &MIER_FIFO {
        unsafe { &*(self as *const Self).cast::<u8>().add(1068usize).cast() }
    }
    #[doc = "0x42c - Mailbox Interrupt Enable Register"]
    #[inline(always)]
    pub const fn mier(&self) -> &MIER {
        unsafe { &*(self as *const Self).cast::<u8>().add(1068usize).cast() }
    }
    #[doc = "0x820..0x840 - Message Control Register for Transmit"]
    #[inline(always)]
    pub const fn mctl_tx(&self) -> &[MCTL_TX; 32] {
        unsafe { &*(self as *const Self).cast::<u8>().add(2080usize).cast() }
    }
    #[doc = "0x820..0x840 - Message Control Register for Receive"]
    #[inline(always)]
    pub const fn mctl_rx(&self) -> &[MCTL_RX; 32] {
        unsafe { &*(self as *const Self).cast::<u8>().add(2080usize).cast() }
    }
}
#[doc = "MB_ID (rw) register accessor: an alias for `Reg<MB_ID_SPEC>`"]
pub type MB_ID = crate::Reg<mb_id::MB_ID_SPEC>;
#[doc = "Mailbox ID Register %s"]
pub mod mb_id;
#[doc = "MB_DL (rw) register accessor: an alias for `Reg<MB_DL_SPEC>`"]
pub type MB_DL = crate::Reg<mb_dl::MB_DL_SPEC>;
#[doc = "Mailbox Data Length Register %s"]
pub mod mb_dl;
#[doc = "MB_D0 (rw) register accessor: an alias for `Reg<MB_D0_SPEC>`"]
pub type MB_D0 = crate::Reg<mb_d0::MB_D0_SPEC>;
#[doc = "Mailbox Data Register %s"]
pub mod mb_d0;
#[doc = "MB_D1 (rw) register accessor: an alias for `Reg<MB_D1_SPEC>`"]
pub type MB_D1 = crate::Reg<mb_d1::MB_D1_SPEC>;
#[doc = "Mailbox Data Register %s"]
pub mod mb_d1;
#[doc = "MB_D2 (rw) register accessor: an alias for `Reg<MB_D2_SPEC>`"]
pub type MB_D2 = crate::Reg<mb_d2::MB_D2_SPEC>;
#[doc = "Mailbox Data Register %s"]
pub mod mb_d2;
#[doc = "MB_D3 (rw) register accessor: an alias for `Reg<MB_D3_SPEC>`"]
pub type MB_D3 = crate::Reg<mb_d3::MB_D3_SPEC>;
#[doc = "Mailbox Data Register %s"]
pub mod mb_d3;
#[doc = "MB_D4 (rw) register accessor: an alias for `Reg<MB_D4_SPEC>`"]
pub type MB_D4 = crate::Reg<mb_d4::MB_D4_SPEC>;
#[doc = "Mailbox Data Register %s"]
pub mod mb_d4;
#[doc = "MB_D5 (rw) register accessor: an alias for `Reg<MB_D5_SPEC>`"]
pub type MB_D5 = crate::Reg<mb_d5::MB_D5_SPEC>;
#[doc = "Mailbox Data Register %s"]
pub mod mb_d5;
#[doc = "MB_D6 (rw) register accessor: an alias for `Reg<MB_D6_SPEC>`"]
pub type MB_D6 = crate::Reg<mb_d6::MB_D6_SPEC>;
#[doc = "Mailbox Data Register %s"]
pub mod mb_d6;
#[doc = "MB_D7 (rw) register accessor: an alias for `Reg<MB_D7_SPEC>`"]
pub type MB_D7 = crate::Reg<mb_d7::MB_D7_SPEC>;
#[doc = "Mailbox Data Register %s"]
pub mod mb_d7;
#[doc = "MB_TS (rw) register accessor: an alias for `Reg<MB_TS_SPEC>`"]
pub type MB_TS = crate::Reg<mb_ts::MB_TS_SPEC>;
#[doc = "Mailbox Time Stamp Register %s"]
pub mod mb_ts;
#[doc = "MKR (rw) register accessor: an alias for `Reg<MKR_SPEC>`"]
pub type MKR = crate::Reg<mkr::MKR_SPEC>;
#[doc = "Mask Register %s"]
pub mod mkr;
#[doc = "FIDCR (rw) register accessor: an alias for `Reg<FIDCR_SPEC>`"]
pub type FIDCR = crate::Reg<fidcr::FIDCR_SPEC>;
#[doc = "FIFO Received ID Compare Register %s"]
pub mod fidcr;
#[doc = "MKIVLR (rw) register accessor: an alias for `Reg<MKIVLR_SPEC>`"]
pub type MKIVLR = crate::Reg<mkivlr::MKIVLR_SPEC>;
#[doc = "Mask Invalid Register"]
pub mod mkivlr;
#[doc = "MIER (rw) register accessor: an alias for `Reg<MIER_SPEC>`"]
pub type MIER = crate::Reg<mier::MIER_SPEC>;
#[doc = "Mailbox Interrupt Enable Register"]
pub mod mier;
#[doc = "MIER_FIFO (rw) register accessor: an alias for `Reg<MIER_FIFO_SPEC>`"]
pub type MIER_FIFO = crate::Reg<mier_fifo::MIER_FIFO_SPEC>;
#[doc = "Mailbox Interrupt Enable Register for FIFO Mailbox Mode"]
pub mod mier_fifo;
#[doc = "MCTL_RX (rw) register accessor: an alias for `Reg<MCTL_RX_SPEC>`"]
pub type MCTL_RX = crate::Reg<mctl_rx::MCTL_RX_SPEC>;
#[doc = "Message Control Register for Receive"]
pub mod mctl_rx;
#[doc = "MCTL_TX (rw) register accessor: an alias for `Reg<MCTL_TX_SPEC>`"]
pub type MCTL_TX = crate::Reg<mctl_tx::MCTL_TX_SPEC>;
#[doc = "Message Control Register for Transmit"]
pub mod mctl_tx;
#[doc = "CTLR (rw) register accessor: an alias for `Reg<CTLR_SPEC>`"]
pub type CTLR = crate::Reg<ctlr::CTLR_SPEC>;
#[doc = "Control Register"]
pub mod ctlr;
#[doc = "STR (r) register accessor: an alias for `Reg<STR_SPEC>`"]
pub type STR = crate::Reg<str::STR_SPEC>;
#[doc = "Status Register"]
pub mod str;
#[doc = "BCR (rw) register accessor: an alias for `Reg<BCR_SPEC>`"]
pub type BCR = crate::Reg<bcr::BCR_SPEC>;
#[doc = "Bit Configuration Register"]
pub mod bcr;
#[doc = "RFCR (rw) register accessor: an alias for `Reg<RFCR_SPEC>`"]
pub type RFCR = crate::Reg<rfcr::RFCR_SPEC>;
#[doc = "Receive FIFO Control Register"]
pub mod rfcr;
#[doc = "RFPCR (w) register accessor: an alias for `Reg<RFPCR_SPEC>`"]
pub type RFPCR = crate::Reg<rfpcr::RFPCR_SPEC>;
#[doc = "Receive FIFO Pointer Control Register"]
pub mod rfpcr;
#[doc = "TFCR (rw) register accessor: an alias for `Reg<TFCR_SPEC>`"]
pub type TFCR = crate::Reg<tfcr::TFCR_SPEC>;
#[doc = "Transmit FIFO Control Register"]
pub mod tfcr;
#[doc = "TFPCR (w) register accessor: an alias for `Reg<TFPCR_SPEC>`"]
pub type TFPCR = crate::Reg<tfpcr::TFPCR_SPEC>;
#[doc = "Transmit FIFO Pointer Control Register"]
pub mod tfpcr;
#[doc = "EIER (rw) register accessor: an alias for `Reg<EIER_SPEC>`"]
pub type EIER = crate::Reg<eier::EIER_SPEC>;
#[doc = "Error Interrupt Enable Register"]
pub mod eier;
#[doc = "EIFR (rw) register accessor: an alias for `Reg<EIFR_SPEC>`"]
pub type EIFR = crate::Reg<eifr::EIFR_SPEC>;
#[doc = "Error Interrupt Factor Judge Register"]
pub mod eifr;
#[doc = "RECR (r) register accessor: an alias for `Reg<RECR_SPEC>`"]
pub type RECR = crate::Reg<recr::RECR_SPEC>;
#[doc = "Receive Error Count Register"]
pub mod recr;
#[doc = "TECR (r) register accessor: an alias for `Reg<TECR_SPEC>`"]
pub type TECR = crate::Reg<tecr::TECR_SPEC>;
#[doc = "Transmit Error Count Register"]
pub mod tecr;
#[doc = "ECSR (rw) register accessor: an alias for `Reg<ECSR_SPEC>`"]
pub type ECSR = crate::Reg<ecsr::ECSR_SPEC>;
#[doc = "Error Code Store Register"]
pub mod ecsr;
#[doc = "CSSR (rw) register accessor: an alias for `Reg<CSSR_SPEC>`"]
pub type CSSR = crate::Reg<cssr::CSSR_SPEC>;
#[doc = "Channel Search Support Register"]
pub mod cssr;
#[doc = "MSSR (r) register accessor: an alias for `Reg<MSSR_SPEC>`"]
pub type MSSR = crate::Reg<mssr::MSSR_SPEC>;
#[doc = "Mailbox Search Status Register"]
pub mod mssr;
#[doc = "MSMR (rw) register accessor: an alias for `Reg<MSMR_SPEC>`"]
pub type MSMR = crate::Reg<msmr::MSMR_SPEC>;
#[doc = "Mailbox Search Mode Register"]
pub mod msmr;
#[doc = "TSR (r) register accessor: an alias for `Reg<TSR_SPEC>`"]
pub type TSR = crate::Reg<tsr::TSR_SPEC>;
#[doc = "Time Stamp Register"]
pub mod tsr;
#[doc = "AFSR (rw) register accessor: an alias for `Reg<AFSR_SPEC>`"]
pub type AFSR = crate::Reg<afsr::AFSR_SPEC>;
#[doc = "Acceptance Filter Support Register"]
pub mod afsr;
#[doc = "TCR (rw) register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "Test Control Register"]
pub mod tcr;
