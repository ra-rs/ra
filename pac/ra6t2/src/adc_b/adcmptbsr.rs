#[doc = "Register `ADCMPTBSR` reader"]
pub struct R(crate::R<ADCMPTBSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPTBSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPTBSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPTBSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMPTBF0` reader - Compare Match Table n Match Flag"]
pub type CMPTBF0_R = crate::BitReader<CMPTBF0_A>;
#[doc = "Compare Match Table n Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPTBF0_A {
    #[doc = "0: Match event with compare match table n is not detected"]
    _0 = 0,
    #[doc = "1: Match event with compare match table n is detected"]
    _1 = 1,
}
impl From<CMPTBF0_A> for bool {
    #[inline(always)]
    fn from(variant: CMPTBF0_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPTBF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPTBF0_A {
        match self.bits {
            false => CMPTBF0_A::_0,
            true => CMPTBF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPTBF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPTBF0_A::_1
    }
}
#[doc = "Field `CMPTBF1` reader - Compare Match Table n Match Flag"]
pub type CMPTBF1_R = crate::BitReader<CMPTBF1_A>;
#[doc = "Compare Match Table n Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPTBF1_A {
    #[doc = "0: Match event with compare match table n is not detected"]
    _0 = 0,
    #[doc = "1: Match event with compare match table n is detected"]
    _1 = 1,
}
impl From<CMPTBF1_A> for bool {
    #[inline(always)]
    fn from(variant: CMPTBF1_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPTBF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPTBF1_A {
        match self.bits {
            false => CMPTBF1_A::_0,
            true => CMPTBF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPTBF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPTBF1_A::_1
    }
}
#[doc = "Field `CMPTBF2` reader - Compare Match Table n Match Flag"]
pub type CMPTBF2_R = crate::BitReader<CMPTBF2_A>;
#[doc = "Compare Match Table n Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPTBF2_A {
    #[doc = "0: Match event with compare match table n is not detected"]
    _0 = 0,
    #[doc = "1: Match event with compare match table n is detected"]
    _1 = 1,
}
impl From<CMPTBF2_A> for bool {
    #[inline(always)]
    fn from(variant: CMPTBF2_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPTBF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPTBF2_A {
        match self.bits {
            false => CMPTBF2_A::_0,
            true => CMPTBF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPTBF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPTBF2_A::_1
    }
}
#[doc = "Field `CMPTBF3` reader - Compare Match Table n Match Flag"]
pub type CMPTBF3_R = crate::BitReader<CMPTBF3_A>;
#[doc = "Compare Match Table n Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPTBF3_A {
    #[doc = "0: Match event with compare match table n is not detected"]
    _0 = 0,
    #[doc = "1: Match event with compare match table n is detected"]
    _1 = 1,
}
impl From<CMPTBF3_A> for bool {
    #[inline(always)]
    fn from(variant: CMPTBF3_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPTBF3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPTBF3_A {
        match self.bits {
            false => CMPTBF3_A::_0,
            true => CMPTBF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPTBF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPTBF3_A::_1
    }
}
#[doc = "Field `CMPTBF4` reader - Compare Match Table n Match Flag"]
pub type CMPTBF4_R = crate::BitReader<CMPTBF4_A>;
#[doc = "Compare Match Table n Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPTBF4_A {
    #[doc = "0: Match event with compare match table n is not detected"]
    _0 = 0,
    #[doc = "1: Match event with compare match table n is detected"]
    _1 = 1,
}
impl From<CMPTBF4_A> for bool {
    #[inline(always)]
    fn from(variant: CMPTBF4_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPTBF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPTBF4_A {
        match self.bits {
            false => CMPTBF4_A::_0,
            true => CMPTBF4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPTBF4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPTBF4_A::_1
    }
}
#[doc = "Field `CMPTBF5` reader - Compare Match Table n Match Flag"]
pub type CMPTBF5_R = crate::BitReader<CMPTBF5_A>;
#[doc = "Compare Match Table n Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPTBF5_A {
    #[doc = "0: Match event with compare match table n is not detected"]
    _0 = 0,
    #[doc = "1: Match event with compare match table n is detected"]
    _1 = 1,
}
impl From<CMPTBF5_A> for bool {
    #[inline(always)]
    fn from(variant: CMPTBF5_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPTBF5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPTBF5_A {
        match self.bits {
            false => CMPTBF5_A::_0,
            true => CMPTBF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPTBF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPTBF5_A::_1
    }
}
#[doc = "Field `CMPTBF6` reader - Compare Match Table n Match Flag"]
pub type CMPTBF6_R = crate::BitReader<CMPTBF6_A>;
#[doc = "Compare Match Table n Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPTBF6_A {
    #[doc = "0: Match event with compare match table n is not detected"]
    _0 = 0,
    #[doc = "1: Match event with compare match table n is detected"]
    _1 = 1,
}
impl From<CMPTBF6_A> for bool {
    #[inline(always)]
    fn from(variant: CMPTBF6_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPTBF6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPTBF6_A {
        match self.bits {
            false => CMPTBF6_A::_0,
            true => CMPTBF6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPTBF6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPTBF6_A::_1
    }
}
#[doc = "Field `CMPTBF7` reader - Compare Match Table n Match Flag"]
pub type CMPTBF7_R = crate::BitReader<CMPTBF7_A>;
#[doc = "Compare Match Table n Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPTBF7_A {
    #[doc = "0: Match event with compare match table n is not detected"]
    _0 = 0,
    #[doc = "1: Match event with compare match table n is detected"]
    _1 = 1,
}
impl From<CMPTBF7_A> for bool {
    #[inline(always)]
    fn from(variant: CMPTBF7_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPTBF7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPTBF7_A {
        match self.bits {
            false => CMPTBF7_A::_0,
            true => CMPTBF7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPTBF7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPTBF7_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Compare Match Table n Match Flag"]
    #[inline(always)]
    pub fn cmptbf0(&self) -> CMPTBF0_R {
        CMPTBF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare Match Table n Match Flag"]
    #[inline(always)]
    pub fn cmptbf1(&self) -> CMPTBF1_R {
        CMPTBF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare Match Table n Match Flag"]
    #[inline(always)]
    pub fn cmptbf2(&self) -> CMPTBF2_R {
        CMPTBF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare Match Table n Match Flag"]
    #[inline(always)]
    pub fn cmptbf3(&self) -> CMPTBF3_R {
        CMPTBF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare Match Table n Match Flag"]
    #[inline(always)]
    pub fn cmptbf4(&self) -> CMPTBF4_R {
        CMPTBF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Compare Match Table n Match Flag"]
    #[inline(always)]
    pub fn cmptbf5(&self) -> CMPTBF5_R {
        CMPTBF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Compare Match Table n Match Flag"]
    #[inline(always)]
    pub fn cmptbf6(&self) -> CMPTBF6_R {
        CMPTBF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Compare Match Table n Match Flag"]
    #[inline(always)]
    pub fn cmptbf7(&self) -> CMPTBF7_R {
        CMPTBF7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Compare Match Table Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmptbsr](index.html) module"]
pub struct ADCMPTBSR_SPEC;
impl crate::RegisterSpec for ADCMPTBSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcmptbsr::R](R) reader structure"]
impl crate::Readable for ADCMPTBSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADCMPTBSR to value 0"]
impl crate::Resettable for ADCMPTBSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
