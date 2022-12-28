#[doc = "Register `ADLIMEXSR` reader"]
pub struct R(crate::R<ADLIMEXSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADLIMEXSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADLIMEXSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADLIMEXSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LIMEXF0` reader - Self-diagnosis Channel: Limiter Clip Flag"]
pub type LIMEXF0_R = crate::BitReader<LIMEXF0_A>;
#[doc = "Self-diagnosis Channel: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMEXF0_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMEXF0_A> for bool {
    #[inline(always)]
    fn from(variant: LIMEXF0_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMEXF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMEXF0_A {
        match self.bits {
            false => LIMEXF0_A::_0,
            true => LIMEXF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMEXF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMEXF0_A::_1
    }
}
#[doc = "Field `LIMEXF1` reader - Temperature Sensor Channel: Limiter Clip Flag"]
pub type LIMEXF1_R = crate::BitReader<LIMEXF1_A>;
#[doc = "Temperature Sensor Channel: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMEXF1_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMEXF1_A> for bool {
    #[inline(always)]
    fn from(variant: LIMEXF1_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMEXF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMEXF1_A {
        match self.bits {
            false => LIMEXF1_A::_0,
            true => LIMEXF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMEXF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMEXF1_A::_1
    }
}
#[doc = "Field `LIMEXF2` reader - Internal Reference Voltage Channel: Limiter Clip Flag"]
pub type LIMEXF2_R = crate::BitReader<LIMEXF2_A>;
#[doc = "Internal Reference Voltage Channel: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMEXF2_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMEXF2_A> for bool {
    #[inline(always)]
    fn from(variant: LIMEXF2_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMEXF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMEXF2_A {
        match self.bits {
            false => LIMEXF2_A::_0,
            true => LIMEXF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMEXF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMEXF2_A::_1
    }
}
#[doc = "Field `LIMEXF5` reader - D/A Converter 0 Channel: Limiter Clip Flag"]
pub type LIMEXF5_R = crate::BitReader<LIMEXF5_A>;
#[doc = "D/A Converter 0 Channel: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMEXF5_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMEXF5_A> for bool {
    #[inline(always)]
    fn from(variant: LIMEXF5_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMEXF5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMEXF5_A {
        match self.bits {
            false => LIMEXF5_A::_0,
            true => LIMEXF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMEXF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMEXF5_A::_1
    }
}
#[doc = "Field `LIMEXF6` reader - D/A Converter 1 Channel: Limiter Clip Flag"]
pub type LIMEXF6_R = crate::BitReader<LIMEXF6_A>;
#[doc = "D/A Converter 1 Channel: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMEXF6_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMEXF6_A> for bool {
    #[inline(always)]
    fn from(variant: LIMEXF6_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMEXF6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMEXF6_A {
        match self.bits {
            false => LIMEXF6_A::_0,
            true => LIMEXF6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMEXF6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMEXF6_A::_1
    }
}
#[doc = "Field `LIMEXF7` reader - D/A Converter 2 Channel: Limiter Clip Flag"]
pub type LIMEXF7_R = crate::BitReader<LIMEXF7_A>;
#[doc = "D/A Converter 2 Channel: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMEXF7_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMEXF7_A> for bool {
    #[inline(always)]
    fn from(variant: LIMEXF7_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMEXF7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMEXF7_A {
        match self.bits {
            false => LIMEXF7_A::_0,
            true => LIMEXF7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMEXF7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMEXF7_A::_1
    }
}
#[doc = "Field `LIMEXF8` reader - D/A Converter 3 Channel: Limiter Clip Flag"]
pub type LIMEXF8_R = crate::BitReader<LIMEXF8_A>;
#[doc = "D/A Converter 3 Channel: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMEXF8_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMEXF8_A> for bool {
    #[inline(always)]
    fn from(variant: LIMEXF8_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMEXF8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMEXF8_A {
        match self.bits {
            false => LIMEXF8_A::_0,
            true => LIMEXF8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMEXF8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMEXF8_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Self-diagnosis Channel: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limexf0(&self) -> LIMEXF0_R {
        LIMEXF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Temperature Sensor Channel: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limexf1(&self) -> LIMEXF1_R {
        LIMEXF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Internal Reference Voltage Channel: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limexf2(&self) -> LIMEXF2_R {
        LIMEXF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - D/A Converter 0 Channel: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limexf5(&self) -> LIMEXF5_R {
        LIMEXF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - D/A Converter 1 Channel: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limexf6(&self) -> LIMEXF6_R {
        LIMEXF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - D/A Converter 2 Channel: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limexf7(&self) -> LIMEXF7_R {
        LIMEXF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - D/A Converter 3 Channel: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limexf8(&self) -> LIMEXF8_R {
        LIMEXF8_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Extended Analog Limiter Clip Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adlimexsr](index.html) module"]
pub struct ADLIMEXSR_SPEC;
impl crate::RegisterSpec for ADLIMEXSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adlimexsr::R](R) reader structure"]
impl crate::Readable for ADLIMEXSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADLIMEXSR to value 0"]
impl crate::Resettable for ADLIMEXSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
