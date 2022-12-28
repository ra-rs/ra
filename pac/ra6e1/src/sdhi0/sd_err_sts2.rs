#[doc = "Register `SD_ERR_STS2` reader"]
pub struct R(crate::R<SD_ERR_STS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_ERR_STS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_ERR_STS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_ERR_STS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RSPTO0` reader - Response Timeout Flag 0"]
pub type RSPTO0_R = crate::BitReader<RSPTO0_A>;
#[doc = "Response Timeout Flag 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPTO0_A {
    #[doc = "0: After command was issued, response was received in less than 640 cycles of the SD/MMC clock"]
    _0 = 0,
    #[doc = "1: After command was issued, response was not received in 640 or more cycles of the SD/MMC clock"]
    _1 = 1,
}
impl From<RSPTO0_A> for bool {
    #[inline(always)]
    fn from(variant: RSPTO0_A) -> Self {
        variant as u8 != 0
    }
}
impl RSPTO0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSPTO0_A {
        match self.bits {
            false => RSPTO0_A::_0,
            true => RSPTO0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSPTO0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSPTO0_A::_1
    }
}
#[doc = "Field `RSPTO1` reader - Response Timeout Flag 1"]
pub type RSPTO1_R = crate::BitReader<RSPTO1_A>;
#[doc = "Response Timeout Flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPTO1_A {
    #[doc = "0: After command was issued, response was received in less than 640 cycles of the SD/MMC clock"]
    _0 = 0,
    #[doc = "1: After command was issued, response was not received after 640 or more cycles of the SD/MMC clock (with SD_CMD.CMDIDX\\[5:0\\]
setting, an error that occurs with CMD12 issue is indicated in the RSPTO0 flag)"]
    _1 = 1,
}
impl From<RSPTO1_A> for bool {
    #[inline(always)]
    fn from(variant: RSPTO1_A) -> Self {
        variant as u8 != 0
    }
}
impl RSPTO1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSPTO1_A {
        match self.bits {
            false => RSPTO1_A::_0,
            true => RSPTO1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSPTO1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSPTO1_A::_1
    }
}
#[doc = "Field `BSYTO0` reader - Busy Timeout Flag 0"]
pub type BSYTO0_R = crate::BitReader<BSYTO0_A>;
#[doc = "Busy Timeout Flag 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSYTO0_A {
    #[doc = "0: After R1b response was received, SD/MMC was released from the busy state during the specified period"]
    _0 = 0,
    #[doc = "1: After R1b response was received, SD/MMC was in the busy state after the specified period elapsed"]
    _1 = 1,
}
impl From<BSYTO0_A> for bool {
    #[inline(always)]
    fn from(variant: BSYTO0_A) -> Self {
        variant as u8 != 0
    }
}
impl BSYTO0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSYTO0_A {
        match self.bits {
            false => BSYTO0_A::_0,
            true => BSYTO0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSYTO0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSYTO0_A::_1
    }
}
#[doc = "Field `BSYTO1` reader - Busy Timeout Flag 1"]
pub type BSYTO1_R = crate::BitReader<BSYTO1_A>;
#[doc = "Busy Timeout Flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSYTO1_A {
    #[doc = "0: After CMD12 was automatically issued, SD/MMC was released from the busy state during the specified period"]
    _0 = 0,
    #[doc = "1: After CMD12 was automatically issued, SD/MMC was in the busy state after the specified period elapsed (with SD_CMD.CMDIDX\\[5:0\\]
setting, an error that occurs with CMD12 issue is indicated in the BSYTO0 flag)"]
    _1 = 1,
}
impl From<BSYTO1_A> for bool {
    #[inline(always)]
    fn from(variant: BSYTO1_A) -> Self {
        variant as u8 != 0
    }
}
impl BSYTO1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSYTO1_A {
        match self.bits {
            false => BSYTO1_A::_0,
            true => BSYTO1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSYTO1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSYTO1_A::_1
    }
}
#[doc = "Field `RDTO` reader - Read Data Timeout Flag"]
pub type RDTO_R = crate::BitReader<bool>;
#[doc = "Field `CRCTO` reader - CRC Status Token Timeout Flag"]
pub type CRCTO_R = crate::BitReader<CRCTO_A>;
#[doc = "CRC Status Token Timeout Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCTO_A {
    #[doc = "0: After CRC data was written to the SD card/MMC, a CRC status token was received during the specified period"]
    _0 = 0,
    #[doc = "1: After CRC data was written to the SD card/MMC, a CRC status token was not received after the specified period elapsed"]
    _1 = 1,
}
impl From<CRCTO_A> for bool {
    #[inline(always)]
    fn from(variant: CRCTO_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCTO_A {
        match self.bits {
            false => CRCTO_A::_0,
            true => CRCTO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRCTO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRCTO_A::_1
    }
}
#[doc = "Field `CRCBSYTO` reader - CRC Status Token Busy Timeout Flag"]
pub type CRCBSYTO_R = crate::BitReader<CRCBSYTO_A>;
#[doc = "CRC Status Token Busy Timeout Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCBSYTO_A {
    #[doc = "0: After a CRC status token was received, the SD/MMC was released from the busy state during the specified period"]
    _0 = 0,
    #[doc = "1: After a CRC status token was received, the SD/MMC was in the busy state after the specified period elapsed"]
    _1 = 1,
}
impl From<CRCBSYTO_A> for bool {
    #[inline(always)]
    fn from(variant: CRCBSYTO_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCBSYTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCBSYTO_A {
        match self.bits {
            false => CRCBSYTO_A::_0,
            true => CRCBSYTO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRCBSYTO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRCBSYTO_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Response Timeout Flag 0"]
    #[inline(always)]
    pub fn rspto0(&self) -> RSPTO0_R {
        RSPTO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Response Timeout Flag 1"]
    #[inline(always)]
    pub fn rspto1(&self) -> RSPTO1_R {
        RSPTO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Busy Timeout Flag 0"]
    #[inline(always)]
    pub fn bsyto0(&self) -> BSYTO0_R {
        BSYTO0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Busy Timeout Flag 1"]
    #[inline(always)]
    pub fn bsyto1(&self) -> BSYTO1_R {
        BSYTO1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read Data Timeout Flag"]
    #[inline(always)]
    pub fn rdto(&self) -> RDTO_R {
        RDTO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CRC Status Token Timeout Flag"]
    #[inline(always)]
    pub fn crcto(&self) -> CRCTO_R {
        CRCTO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC Status Token Busy Timeout Flag"]
    #[inline(always)]
    pub fn crcbsyto(&self) -> CRCBSYTO_R {
        CRCBSYTO_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "SD Error Status Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_err_sts2](index.html) module"]
pub struct SD_ERR_STS2_SPEC;
impl crate::RegisterSpec for SD_ERR_STS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_err_sts2::R](R) reader structure"]
impl crate::Readable for SD_ERR_STS2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SD_ERR_STS2 to value 0"]
impl crate::Resettable for SD_ERR_STS2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
