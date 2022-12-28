#[doc = "Register `ADCMPCHSR0` reader"]
pub struct R(crate::R<ADCMPCHSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPCHSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPCHSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPCHSR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMPCHF0` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF0_R = crate::BitReader<CMPCHF0_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF0_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF0_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF0_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF0_A {
        match self.bits {
            false => CMPCHF0_A::_0,
            true => CMPCHF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF0_A::_1
    }
}
#[doc = "Field `CMPCHF1` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF1_R = crate::BitReader<CMPCHF1_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF1_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF1_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF1_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF1_A {
        match self.bits {
            false => CMPCHF1_A::_0,
            true => CMPCHF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF1_A::_1
    }
}
#[doc = "Field `CMPCHF2` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF2_R = crate::BitReader<CMPCHF2_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF2_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF2_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF2_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF2_A {
        match self.bits {
            false => CMPCHF2_A::_0,
            true => CMPCHF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF2_A::_1
    }
}
#[doc = "Field `CMPCHF3` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF3_R = crate::BitReader<CMPCHF3_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF3_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF3_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF3_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF3_A {
        match self.bits {
            false => CMPCHF3_A::_0,
            true => CMPCHF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF3_A::_1
    }
}
#[doc = "Field `CMPCHF4` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF4_R = crate::BitReader<CMPCHF4_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF4_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF4_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF4_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF4_A {
        match self.bits {
            false => CMPCHF4_A::_0,
            true => CMPCHF4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF4_A::_1
    }
}
#[doc = "Field `CMPCHF5` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF5_R = crate::BitReader<CMPCHF5_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF5_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF5_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF5_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF5_A {
        match self.bits {
            false => CMPCHF5_A::_0,
            true => CMPCHF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF5_A::_1
    }
}
#[doc = "Field `CMPCHF6` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF6_R = crate::BitReader<CMPCHF6_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF6_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF6_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF6_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF6_A {
        match self.bits {
            false => CMPCHF6_A::_0,
            true => CMPCHF6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF6_A::_1
    }
}
#[doc = "Field `CMPCHF7` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF7_R = crate::BitReader<CMPCHF7_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF7_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF7_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF7_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF7_A {
        match self.bits {
            false => CMPCHF7_A::_0,
            true => CMPCHF7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF7_A::_1
    }
}
#[doc = "Field `CMPCHF8` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF8_R = crate::BitReader<CMPCHF8_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF8_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF8_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF8_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF8_A {
        match self.bits {
            false => CMPCHF8_A::_0,
            true => CMPCHF8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF8_A::_1
    }
}
#[doc = "Field `CMPCHF9` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF9_R = crate::BitReader<CMPCHF9_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF9_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF9_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF9_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF9_A {
        match self.bits {
            false => CMPCHF9_A::_0,
            true => CMPCHF9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF9_A::_1
    }
}
#[doc = "Field `CMPCHF10` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF10_R = crate::BitReader<CMPCHF10_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF10_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF10_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF10_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF10_A {
        match self.bits {
            false => CMPCHF10_A::_0,
            true => CMPCHF10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF10_A::_1
    }
}
#[doc = "Field `CMPCHF11` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF11_R = crate::BitReader<CMPCHF11_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF11_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF11_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF11_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF11_A {
        match self.bits {
            false => CMPCHF11_A::_0,
            true => CMPCHF11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF11_A::_1
    }
}
#[doc = "Field `CMPCHF12` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF12_R = crate::BitReader<CMPCHF12_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF12_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF12_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF12_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF12_A {
        match self.bits {
            false => CMPCHF12_A::_0,
            true => CMPCHF12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF12_A::_1
    }
}
#[doc = "Field `CMPCHF13` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF13_R = crate::BitReader<CMPCHF13_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF13_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF13_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF13_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF13_A {
        match self.bits {
            false => CMPCHF13_A::_0,
            true => CMPCHF13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF13_A::_1
    }
}
#[doc = "Field `CMPCHF14` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF14_R = crate::BitReader<CMPCHF14_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF14_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF14_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF14_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF14_A {
        match self.bits {
            false => CMPCHF14_A::_0,
            true => CMPCHF14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF14_A::_1
    }
}
#[doc = "Field `CMPCHF15` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF15_R = crate::BitReader<CMPCHF15_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF15_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF15_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF15_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF15_A {
        match self.bits {
            false => CMPCHF15_A::_0,
            true => CMPCHF15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF15_A::_1
    }
}
#[doc = "Field `CMPCHF16` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF16_R = crate::BitReader<CMPCHF16_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF16_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF16_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF16_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF16_A {
        match self.bits {
            false => CMPCHF16_A::_0,
            true => CMPCHF16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF16_A::_1
    }
}
#[doc = "Field `CMPCHF17` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF17_R = crate::BitReader<CMPCHF17_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF17_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF17_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF17_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF17_A {
        match self.bits {
            false => CMPCHF17_A::_0,
            true => CMPCHF17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF17_A::_1
    }
}
#[doc = "Field `CMPCHF18` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF18_R = crate::BitReader<CMPCHF18_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF18_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF18_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF18_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF18_A {
        match self.bits {
            false => CMPCHF18_A::_0,
            true => CMPCHF18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF18_A::_1
    }
}
#[doc = "Field `CMPCHF19` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF19_R = crate::BitReader<CMPCHF19_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF19_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF19_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF19_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF19_A {
        match self.bits {
            false => CMPCHF19_A::_0,
            true => CMPCHF19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF19_A::_1
    }
}
#[doc = "Field `CMPCHF20` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF20_R = crate::BitReader<CMPCHF20_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF20_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF20_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF20_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF20_A {
        match self.bits {
            false => CMPCHF20_A::_0,
            true => CMPCHF20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF20_A::_1
    }
}
#[doc = "Field `CMPCHF21` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF21_R = crate::BitReader<CMPCHF21_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF21_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF21_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF21_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF21_A {
        match self.bits {
            false => CMPCHF21_A::_0,
            true => CMPCHF21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF21_A::_1
    }
}
#[doc = "Field `CMPCHF22` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF22_R = crate::BitReader<CMPCHF22_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF22_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF22_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF22_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF22_A {
        match self.bits {
            false => CMPCHF22_A::_0,
            true => CMPCHF22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF22_A::_1
    }
}
#[doc = "Field `CMPCHF23` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF23_R = crate::BitReader<CMPCHF23_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF23_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF23_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF23_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF23_A {
        match self.bits {
            false => CMPCHF23_A::_0,
            true => CMPCHF23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF23_A::_1
    }
}
#[doc = "Field `CMPCHF24` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF24_R = crate::BitReader<CMPCHF24_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF24_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF24_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF24_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF24_A {
        match self.bits {
            false => CMPCHF24_A::_0,
            true => CMPCHF24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF24_A::_1
    }
}
#[doc = "Field `CMPCHF25` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF25_R = crate::BitReader<CMPCHF25_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF25_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF25_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF25_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF25_A {
        match self.bits {
            false => CMPCHF25_A::_0,
            true => CMPCHF25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF25_A::_1
    }
}
#[doc = "Field `CMPCHF26` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF26_R = crate::BitReader<CMPCHF26_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF26_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF26_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF26_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF26_A {
        match self.bits {
            false => CMPCHF26_A::_0,
            true => CMPCHF26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF26_A::_1
    }
}
#[doc = "Field `CMPCHF27` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF27_R = crate::BitReader<CMPCHF27_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF27_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF27_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF27_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF27_A {
        match self.bits {
            false => CMPCHF27_A::_0,
            true => CMPCHF27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF27_A::_1
    }
}
#[doc = "Field `CMPCHF28` reader - Analog Channel n: Compare Match Flag"]
pub type CMPCHF28_R = crate::BitReader<CMPCHF28_A>;
#[doc = "Analog Channel n: Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHF28_A {
    #[doc = "0: Compare match is not detected"]
    _0 = 0,
    #[doc = "1: Compare match is detected"]
    _1 = 1,
}
impl From<CMPCHF28_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHF28_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHF28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHF28_A {
        match self.bits {
            false => CMPCHF28_A::_0,
            true => CMPCHF28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHF28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHF28_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf0(&self) -> CMPCHF0_R {
        CMPCHF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf1(&self) -> CMPCHF1_R {
        CMPCHF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf2(&self) -> CMPCHF2_R {
        CMPCHF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf3(&self) -> CMPCHF3_R {
        CMPCHF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf4(&self) -> CMPCHF4_R {
        CMPCHF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf5(&self) -> CMPCHF5_R {
        CMPCHF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf6(&self) -> CMPCHF6_R {
        CMPCHF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf7(&self) -> CMPCHF7_R {
        CMPCHF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf8(&self) -> CMPCHF8_R {
        CMPCHF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf9(&self) -> CMPCHF9_R {
        CMPCHF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf10(&self) -> CMPCHF10_R {
        CMPCHF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf11(&self) -> CMPCHF11_R {
        CMPCHF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf12(&self) -> CMPCHF12_R {
        CMPCHF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf13(&self) -> CMPCHF13_R {
        CMPCHF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf14(&self) -> CMPCHF14_R {
        CMPCHF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf15(&self) -> CMPCHF15_R {
        CMPCHF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf16(&self) -> CMPCHF16_R {
        CMPCHF16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf17(&self) -> CMPCHF17_R {
        CMPCHF17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf18(&self) -> CMPCHF18_R {
        CMPCHF18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf19(&self) -> CMPCHF19_R {
        CMPCHF19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf20(&self) -> CMPCHF20_R {
        CMPCHF20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf21(&self) -> CMPCHF21_R {
        CMPCHF21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf22(&self) -> CMPCHF22_R {
        CMPCHF22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf23(&self) -> CMPCHF23_R {
        CMPCHF23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf24(&self) -> CMPCHF24_R {
        CMPCHF24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf25(&self) -> CMPCHF25_R {
        CMPCHF25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf26(&self) -> CMPCHF26_R {
        CMPCHF26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf27(&self) -> CMPCHF27_R {
        CMPCHF27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Analog Channel n: Compare Match Flag"]
    #[inline(always)]
    pub fn cmpchf28(&self) -> CMPCHF28_R {
        CMPCHF28_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Compare Match Channel Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpchsr0](index.html) module"]
pub struct ADCMPCHSR0_SPEC;
impl crate::RegisterSpec for ADCMPCHSR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcmpchsr0::R](R) reader structure"]
impl crate::Readable for ADCMPCHSR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADCMPCHSR0 to value 0"]
impl crate::Resettable for ADCMPCHSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
