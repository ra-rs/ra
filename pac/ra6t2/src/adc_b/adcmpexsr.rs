#[doc = "Register `ADCMPEXSR` reader"]
pub struct R(crate::R<ADCMPEXSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPEXSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPEXSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPEXSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMPEXF0` reader - Self-diagnosis Channel: Compare Match Flag"]
pub type CMPEXF0_R = crate::BitReader<CMPEXF0_A>;
#[doc = "Self-diagnosis Channel: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPEXF0_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPEXF0_A> for bool {
    #[inline(always)]
    fn from(variant: CMPEXF0_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPEXF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPEXF0_A {
        match self.bits {
            false => CMPEXF0_A::_0,
            true => CMPEXF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPEXF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPEXF0_A::_1
    }
}
#[doc = "Field `CMPEXF1` reader - Temperature Sensor Channel: Compare Match Flag"]
pub type CMPEXF1_R = crate::BitReader<CMPEXF1_A>;
#[doc = "Temperature Sensor Channel: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPEXF1_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPEXF1_A> for bool {
    #[inline(always)]
    fn from(variant: CMPEXF1_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPEXF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPEXF1_A {
        match self.bits {
            false => CMPEXF1_A::_0,
            true => CMPEXF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPEXF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPEXF1_A::_1
    }
}
#[doc = "Field `CMPEXF2` reader - Internal Reference Voltage Channel: Compare Match Flag"]
pub type CMPEXF2_R = crate::BitReader<CMPEXF2_A>;
#[doc = "Internal Reference Voltage Channel: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPEXF2_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPEXF2_A> for bool {
    #[inline(always)]
    fn from(variant: CMPEXF2_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPEXF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPEXF2_A {
        match self.bits {
            false => CMPEXF2_A::_0,
            true => CMPEXF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPEXF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPEXF2_A::_1
    }
}
#[doc = "Field `CMPEXF5` reader - D/A Converter 0 Channel: Compare Match Flag"]
pub type CMPEXF5_R = crate::BitReader<CMPEXF5_A>;
#[doc = "D/A Converter 0 Channel: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPEXF5_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPEXF5_A> for bool {
    #[inline(always)]
    fn from(variant: CMPEXF5_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPEXF5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPEXF5_A {
        match self.bits {
            false => CMPEXF5_A::_0,
            true => CMPEXF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPEXF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPEXF5_A::_1
    }
}
#[doc = "Field `CMPEXF6` reader - D/A Converter 1 Channel: Compare Match Flag"]
pub type CMPEXF6_R = crate::BitReader<CMPEXF6_A>;
#[doc = "D/A Converter 1 Channel: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPEXF6_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPEXF6_A> for bool {
    #[inline(always)]
    fn from(variant: CMPEXF6_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPEXF6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPEXF6_A {
        match self.bits {
            false => CMPEXF6_A::_0,
            true => CMPEXF6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPEXF6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPEXF6_A::_1
    }
}
#[doc = "Field `CMPEXF7` reader - D/A Converter 2 Channel: Compare Match Flag"]
pub type CMPEXF7_R = crate::BitReader<CMPEXF7_A>;
#[doc = "D/A Converter 2 Channel: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPEXF7_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPEXF7_A> for bool {
    #[inline(always)]
    fn from(variant: CMPEXF7_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPEXF7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPEXF7_A {
        match self.bits {
            false => CMPEXF7_A::_0,
            true => CMPEXF7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPEXF7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPEXF7_A::_1
    }
}
#[doc = "Field `CMPEXF8` reader - D/A Converter 3 Channel: Compare Match Flag"]
pub type CMPEXF8_R = crate::BitReader<CMPEXF8_A>;
#[doc = "D/A Converter 3 Channel: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPEXF8_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPEXF8_A> for bool {
    #[inline(always)]
    fn from(variant: CMPEXF8_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPEXF8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPEXF8_A {
        match self.bits {
            false => CMPEXF8_A::_0,
            true => CMPEXF8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPEXF8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPEXF8_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Self-diagnosis Channel: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpexf0(&self) -> CMPEXF0_R {
        CMPEXF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Temperature Sensor Channel: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpexf1(&self) -> CMPEXF1_R {
        CMPEXF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Internal Reference Voltage Channel: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpexf2(&self) -> CMPEXF2_R {
        CMPEXF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - D/A Converter 0 Channel: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpexf5(&self) -> CMPEXF5_R {
        CMPEXF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - D/A Converter 1 Channel: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpexf6(&self) -> CMPEXF6_R {
        CMPEXF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - D/A Converter 2 Channel: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpexf7(&self) -> CMPEXF7_R {
        CMPEXF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - D/A Converter 3 Channel: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpexf8(&self) -> CMPEXF8_R {
        CMPEXF8_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Extended Analog Compare Match Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpexsr](index.html) module"]
pub struct ADCMPEXSR_SPEC;
impl crate::RegisterSpec for ADCMPEXSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcmpexsr::R](R) reader structure"]
impl crate::Readable for ADCMPEXSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADCMPEXSR to value 0"]
impl crate::Resettable for ADCMPEXSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
