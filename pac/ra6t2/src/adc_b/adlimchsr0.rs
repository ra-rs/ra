#[doc = "Register `ADLIMCHSR0` reader"]
pub struct R(crate::R<ADLIMCHSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADLIMCHSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADLIMCHSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADLIMCHSR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LIMCHF0` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF0_R = crate::BitReader<LIMCHF0_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF0_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF0_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF0_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF0_A {
        match self.bits {
            false => LIMCHF0_A::_0,
            true => LIMCHF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF0_A::_1
    }
}
#[doc = "Field `LIMCHF1` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF1_R = crate::BitReader<LIMCHF1_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF1_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF1_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF1_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF1_A {
        match self.bits {
            false => LIMCHF1_A::_0,
            true => LIMCHF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF1_A::_1
    }
}
#[doc = "Field `LIMCHF2` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF2_R = crate::BitReader<LIMCHF2_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF2_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF2_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF2_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF2_A {
        match self.bits {
            false => LIMCHF2_A::_0,
            true => LIMCHF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF2_A::_1
    }
}
#[doc = "Field `LIMCHF3` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF3_R = crate::BitReader<LIMCHF3_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF3_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF3_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF3_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF3_A {
        match self.bits {
            false => LIMCHF3_A::_0,
            true => LIMCHF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF3_A::_1
    }
}
#[doc = "Field `LIMCHF4` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF4_R = crate::BitReader<LIMCHF4_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF4_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF4_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF4_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF4_A {
        match self.bits {
            false => LIMCHF4_A::_0,
            true => LIMCHF4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF4_A::_1
    }
}
#[doc = "Field `LIMCHF5` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF5_R = crate::BitReader<LIMCHF5_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF5_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF5_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF5_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF5_A {
        match self.bits {
            false => LIMCHF5_A::_0,
            true => LIMCHF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF5_A::_1
    }
}
#[doc = "Field `LIMCHF6` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF6_R = crate::BitReader<LIMCHF6_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF6_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF6_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF6_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF6_A {
        match self.bits {
            false => LIMCHF6_A::_0,
            true => LIMCHF6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF6_A::_1
    }
}
#[doc = "Field `LIMCHF7` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF7_R = crate::BitReader<LIMCHF7_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF7_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF7_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF7_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF7_A {
        match self.bits {
            false => LIMCHF7_A::_0,
            true => LIMCHF7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF7_A::_1
    }
}
#[doc = "Field `LIMCHF8` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF8_R = crate::BitReader<LIMCHF8_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF8_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF8_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF8_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF8_A {
        match self.bits {
            false => LIMCHF8_A::_0,
            true => LIMCHF8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF8_A::_1
    }
}
#[doc = "Field `LIMCHF9` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF9_R = crate::BitReader<LIMCHF9_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF9_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF9_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF9_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF9_A {
        match self.bits {
            false => LIMCHF9_A::_0,
            true => LIMCHF9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF9_A::_1
    }
}
#[doc = "Field `LIMCHF10` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF10_R = crate::BitReader<LIMCHF10_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF10_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF10_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF10_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF10_A {
        match self.bits {
            false => LIMCHF10_A::_0,
            true => LIMCHF10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF10_A::_1
    }
}
#[doc = "Field `LIMCHF11` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF11_R = crate::BitReader<LIMCHF11_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF11_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF11_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF11_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF11_A {
        match self.bits {
            false => LIMCHF11_A::_0,
            true => LIMCHF11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF11_A::_1
    }
}
#[doc = "Field `LIMCHF12` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF12_R = crate::BitReader<LIMCHF12_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF12_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF12_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF12_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF12_A {
        match self.bits {
            false => LIMCHF12_A::_0,
            true => LIMCHF12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF12_A::_1
    }
}
#[doc = "Field `LIMCHF13` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF13_R = crate::BitReader<LIMCHF13_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF13_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF13_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF13_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF13_A {
        match self.bits {
            false => LIMCHF13_A::_0,
            true => LIMCHF13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF13_A::_1
    }
}
#[doc = "Field `LIMCHF14` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF14_R = crate::BitReader<LIMCHF14_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF14_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF14_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF14_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF14_A {
        match self.bits {
            false => LIMCHF14_A::_0,
            true => LIMCHF14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF14_A::_1
    }
}
#[doc = "Field `LIMCHF15` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF15_R = crate::BitReader<LIMCHF15_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF15_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF15_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF15_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF15_A {
        match self.bits {
            false => LIMCHF15_A::_0,
            true => LIMCHF15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF15_A::_1
    }
}
#[doc = "Field `LIMCHF16` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF16_R = crate::BitReader<LIMCHF16_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF16_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF16_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF16_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF16_A {
        match self.bits {
            false => LIMCHF16_A::_0,
            true => LIMCHF16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF16_A::_1
    }
}
#[doc = "Field `LIMCHF17` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF17_R = crate::BitReader<LIMCHF17_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF17_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF17_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF17_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF17_A {
        match self.bits {
            false => LIMCHF17_A::_0,
            true => LIMCHF17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF17_A::_1
    }
}
#[doc = "Field `LIMCHF18` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF18_R = crate::BitReader<LIMCHF18_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF18_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF18_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF18_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF18_A {
        match self.bits {
            false => LIMCHF18_A::_0,
            true => LIMCHF18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF18_A::_1
    }
}
#[doc = "Field `LIMCHF19` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF19_R = crate::BitReader<LIMCHF19_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF19_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF19_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF19_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF19_A {
        match self.bits {
            false => LIMCHF19_A::_0,
            true => LIMCHF19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF19_A::_1
    }
}
#[doc = "Field `LIMCHF20` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF20_R = crate::BitReader<LIMCHF20_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF20_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF20_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF20_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF20_A {
        match self.bits {
            false => LIMCHF20_A::_0,
            true => LIMCHF20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF20_A::_1
    }
}
#[doc = "Field `LIMCHF21` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF21_R = crate::BitReader<LIMCHF21_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF21_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF21_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF21_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF21_A {
        match self.bits {
            false => LIMCHF21_A::_0,
            true => LIMCHF21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF21_A::_1
    }
}
#[doc = "Field `LIMCHF22` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF22_R = crate::BitReader<LIMCHF22_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF22_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF22_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF22_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF22_A {
        match self.bits {
            false => LIMCHF22_A::_0,
            true => LIMCHF22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF22_A::_1
    }
}
#[doc = "Field `LIMCHF23` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF23_R = crate::BitReader<LIMCHF23_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF23_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF23_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF23_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF23_A {
        match self.bits {
            false => LIMCHF23_A::_0,
            true => LIMCHF23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF23_A::_1
    }
}
#[doc = "Field `LIMCHF24` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF24_R = crate::BitReader<LIMCHF24_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF24_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF24_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF24_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF24_A {
        match self.bits {
            false => LIMCHF24_A::_0,
            true => LIMCHF24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF24_A::_1
    }
}
#[doc = "Field `LIMCHF25` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF25_R = crate::BitReader<LIMCHF25_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF25_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF25_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF25_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF25_A {
        match self.bits {
            false => LIMCHF25_A::_0,
            true => LIMCHF25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF25_A::_1
    }
}
#[doc = "Field `LIMCHF26` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF26_R = crate::BitReader<LIMCHF26_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF26_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF26_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF26_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF26_A {
        match self.bits {
            false => LIMCHF26_A::_0,
            true => LIMCHF26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF26_A::_1
    }
}
#[doc = "Field `LIMCHF27` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF27_R = crate::BitReader<LIMCHF27_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF27_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF27_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF27_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF27_A {
        match self.bits {
            false => LIMCHF27_A::_0,
            true => LIMCHF27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF27_A::_1
    }
}
#[doc = "Field `LIMCHF28` reader - Analog Channel n: Limiter Clip Flag"]
pub type LIMCHF28_R = crate::BitReader<LIMCHF28_A>;
#[doc = "Analog Channel n: Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHF28_A {
    #[doc = "0: Limiter clip is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip is detected"]
    _1 = 1,
}
impl From<LIMCHF28_A> for bool {
    #[inline(always)]
    fn from(variant: LIMCHF28_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMCHF28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMCHF28_A {
        match self.bits {
            false => LIMCHF28_A::_0,
            true => LIMCHF28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMCHF28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMCHF28_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf0(&self) -> LIMCHF0_R {
        LIMCHF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf1(&self) -> LIMCHF1_R {
        LIMCHF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf2(&self) -> LIMCHF2_R {
        LIMCHF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf3(&self) -> LIMCHF3_R {
        LIMCHF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf4(&self) -> LIMCHF4_R {
        LIMCHF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf5(&self) -> LIMCHF5_R {
        LIMCHF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf6(&self) -> LIMCHF6_R {
        LIMCHF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf7(&self) -> LIMCHF7_R {
        LIMCHF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf8(&self) -> LIMCHF8_R {
        LIMCHF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf9(&self) -> LIMCHF9_R {
        LIMCHF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf10(&self) -> LIMCHF10_R {
        LIMCHF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf11(&self) -> LIMCHF11_R {
        LIMCHF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf12(&self) -> LIMCHF12_R {
        LIMCHF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf13(&self) -> LIMCHF13_R {
        LIMCHF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf14(&self) -> LIMCHF14_R {
        LIMCHF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf15(&self) -> LIMCHF15_R {
        LIMCHF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf16(&self) -> LIMCHF16_R {
        LIMCHF16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf17(&self) -> LIMCHF17_R {
        LIMCHF17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf18(&self) -> LIMCHF18_R {
        LIMCHF18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf19(&self) -> LIMCHF19_R {
        LIMCHF19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf20(&self) -> LIMCHF20_R {
        LIMCHF20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf21(&self) -> LIMCHF21_R {
        LIMCHF21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf22(&self) -> LIMCHF22_R {
        LIMCHF22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf23(&self) -> LIMCHF23_R {
        LIMCHF23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf24(&self) -> LIMCHF24_R {
        LIMCHF24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf25(&self) -> LIMCHF25_R {
        LIMCHF25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf26(&self) -> LIMCHF26_R {
        LIMCHF26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf27(&self) -> LIMCHF27_R {
        LIMCHF27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Analog Channel n: Limiter Clip Flag"]
    #[inline(always)]
    pub fn limchf28(&self) -> LIMCHF28_R {
        LIMCHF28_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Limiter Clip Channel Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adlimchsr0](index.html) module"]
pub struct ADLIMCHSR0_SPEC;
impl crate::RegisterSpec for ADLIMCHSR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adlimchsr0::R](R) reader structure"]
impl crate::Readable for ADLIMCHSR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADLIMCHSR0 to value 0"]
impl crate::Resettable for ADLIMCHSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
