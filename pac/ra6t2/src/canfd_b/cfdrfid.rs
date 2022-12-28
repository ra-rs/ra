#[doc = "Register `CFDRFID%s` reader"]
pub struct R(crate::R<CFDRFID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDRFID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDRFID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDRFID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RFID` reader - RX FIFO Buffer ID Field"]
pub type RFID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RFRTR` reader - RX FIFO Buffer RTR bit"]
pub type RFRTR_R = crate::BitReader<RFRTR_A>;
#[doc = "RX FIFO Buffer RTR bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFRTR_A {
    #[doc = "0: Data frame"]
    _0 = 0,
    #[doc = "1: Remote frame"]
    _1 = 1,
}
impl From<RFRTR_A> for bool {
    #[inline(always)]
    fn from(variant: RFRTR_A) -> Self {
        variant as u8 != 0
    }
}
impl RFRTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFRTR_A {
        match self.bits {
            false => RFRTR_A::_0,
            true => RFRTR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFRTR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFRTR_A::_1
    }
}
#[doc = "Field `RFIDE` reader - RX FIFO Buffer IDE bit"]
pub type RFIDE_R = crate::BitReader<RFIDE_A>;
#[doc = "RX FIFO Buffer IDE bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFIDE_A {
    #[doc = "0: STD-ID has been received"]
    _0 = 0,
    #[doc = "1: EXT-ID has been received"]
    _1 = 1,
}
impl From<RFIDE_A> for bool {
    #[inline(always)]
    fn from(variant: RFIDE_A) -> Self {
        variant as u8 != 0
    }
}
impl RFIDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFIDE_A {
        match self.bits {
            false => RFIDE_A::_0,
            true => RFIDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFIDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFIDE_A::_1
    }
}
impl R {
    #[doc = "Bits 0:28 - RX FIFO Buffer ID Field"]
    #[inline(always)]
    pub fn rfid(&self) -> RFID_R {
        RFID_R::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 30 - RX FIFO Buffer RTR bit"]
    #[inline(always)]
    pub fn rfrtr(&self) -> RFRTR_R {
        RFRTR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RX FIFO Buffer IDE bit"]
    #[inline(always)]
    pub fn rfide(&self) -> RFIDE_R {
        RFIDE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "RX FIFO Access ID Register %s\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdrfid](index.html) module"]
pub struct CFDRFID_SPEC;
impl crate::RegisterSpec for CFDRFID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdrfid::R](R) reader structure"]
impl crate::Readable for CFDRFID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFDRFID%s to value 0"]
impl crate::Resettable for CFDRFID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
