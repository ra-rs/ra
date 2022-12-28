#[doc = "Register `ADOVFCHSR0` reader"]
pub struct R(crate::R<ADOVFCHSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADOVFCHSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADOVFCHSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADOVFCHSR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OFVCHF0` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF0_R = crate::BitReader<OFVCHF0_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF0_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF0_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF0_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF0_A {
        match self.bits {
            false => OFVCHF0_A::_0,
            true => OFVCHF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF0_A::_1
    }
}
#[doc = "Field `OFVCHF1` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF1_R = crate::BitReader<OFVCHF1_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF1_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF1_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF1_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF1_A {
        match self.bits {
            false => OFVCHF1_A::_0,
            true => OFVCHF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF1_A::_1
    }
}
#[doc = "Field `OFVCHF2` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF2_R = crate::BitReader<OFVCHF2_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF2_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF2_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF2_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF2_A {
        match self.bits {
            false => OFVCHF2_A::_0,
            true => OFVCHF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF2_A::_1
    }
}
#[doc = "Field `OFVCHF3` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF3_R = crate::BitReader<OFVCHF3_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF3_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF3_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF3_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF3_A {
        match self.bits {
            false => OFVCHF3_A::_0,
            true => OFVCHF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF3_A::_1
    }
}
#[doc = "Field `OFVCHF4` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF4_R = crate::BitReader<OFVCHF4_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF4_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF4_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF4_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF4_A {
        match self.bits {
            false => OFVCHF4_A::_0,
            true => OFVCHF4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF4_A::_1
    }
}
#[doc = "Field `OFVCHF5` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF5_R = crate::BitReader<OFVCHF5_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF5_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF5_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF5_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF5_A {
        match self.bits {
            false => OFVCHF5_A::_0,
            true => OFVCHF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF5_A::_1
    }
}
#[doc = "Field `OFVCHF6` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF6_R = crate::BitReader<OFVCHF6_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF6_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF6_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF6_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF6_A {
        match self.bits {
            false => OFVCHF6_A::_0,
            true => OFVCHF6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF6_A::_1
    }
}
#[doc = "Field `OFVCHF7` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF7_R = crate::BitReader<OFVCHF7_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF7_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF7_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF7_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF7_A {
        match self.bits {
            false => OFVCHF7_A::_0,
            true => OFVCHF7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF7_A::_1
    }
}
#[doc = "Field `OFVCHF8` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF8_R = crate::BitReader<OFVCHF8_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF8_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF8_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF8_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF8_A {
        match self.bits {
            false => OFVCHF8_A::_0,
            true => OFVCHF8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF8_A::_1
    }
}
#[doc = "Field `OFVCHF9` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF9_R = crate::BitReader<OFVCHF9_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF9_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF9_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF9_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF9_A {
        match self.bits {
            false => OFVCHF9_A::_0,
            true => OFVCHF9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF9_A::_1
    }
}
#[doc = "Field `OFVCHF10` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF10_R = crate::BitReader<OFVCHF10_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF10_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF10_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF10_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF10_A {
        match self.bits {
            false => OFVCHF10_A::_0,
            true => OFVCHF10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF10_A::_1
    }
}
#[doc = "Field `OFVCHF11` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF11_R = crate::BitReader<OFVCHF11_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF11_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF11_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF11_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF11_A {
        match self.bits {
            false => OFVCHF11_A::_0,
            true => OFVCHF11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF11_A::_1
    }
}
#[doc = "Field `OFVCHF12` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF12_R = crate::BitReader<OFVCHF12_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF12_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF12_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF12_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF12_A {
        match self.bits {
            false => OFVCHF12_A::_0,
            true => OFVCHF12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF12_A::_1
    }
}
#[doc = "Field `OFVCHF13` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF13_R = crate::BitReader<OFVCHF13_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF13_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF13_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF13_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF13_A {
        match self.bits {
            false => OFVCHF13_A::_0,
            true => OFVCHF13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF13_A::_1
    }
}
#[doc = "Field `OFVCHF14` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF14_R = crate::BitReader<OFVCHF14_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF14_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF14_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF14_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF14_A {
        match self.bits {
            false => OFVCHF14_A::_0,
            true => OFVCHF14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF14_A::_1
    }
}
#[doc = "Field `OFVCHF15` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF15_R = crate::BitReader<OFVCHF15_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF15_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF15_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF15_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF15_A {
        match self.bits {
            false => OFVCHF15_A::_0,
            true => OFVCHF15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF15_A::_1
    }
}
#[doc = "Field `OFVCHF16` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF16_R = crate::BitReader<OFVCHF16_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF16_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF16_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF16_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF16_A {
        match self.bits {
            false => OFVCHF16_A::_0,
            true => OFVCHF16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF16_A::_1
    }
}
#[doc = "Field `OFVCHF17` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF17_R = crate::BitReader<OFVCHF17_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF17_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF17_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF17_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF17_A {
        match self.bits {
            false => OFVCHF17_A::_0,
            true => OFVCHF17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF17_A::_1
    }
}
#[doc = "Field `OFVCHF18` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF18_R = crate::BitReader<OFVCHF18_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF18_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF18_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF18_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF18_A {
        match self.bits {
            false => OFVCHF18_A::_0,
            true => OFVCHF18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF18_A::_1
    }
}
#[doc = "Field `OFVCHF19` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF19_R = crate::BitReader<OFVCHF19_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF19_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF19_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF19_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF19_A {
        match self.bits {
            false => OFVCHF19_A::_0,
            true => OFVCHF19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF19_A::_1
    }
}
#[doc = "Field `OFVCHF20` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF20_R = crate::BitReader<OFVCHF20_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF20_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF20_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF20_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF20_A {
        match self.bits {
            false => OFVCHF20_A::_0,
            true => OFVCHF20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF20_A::_1
    }
}
#[doc = "Field `OFVCHF21` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF21_R = crate::BitReader<OFVCHF21_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF21_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF21_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF21_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF21_A {
        match self.bits {
            false => OFVCHF21_A::_0,
            true => OFVCHF21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF21_A::_1
    }
}
#[doc = "Field `OFVCHF22` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF22_R = crate::BitReader<OFVCHF22_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF22_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF22_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF22_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF22_A {
        match self.bits {
            false => OFVCHF22_A::_0,
            true => OFVCHF22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF22_A::_1
    }
}
#[doc = "Field `OFVCHF23` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF23_R = crate::BitReader<OFVCHF23_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF23_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF23_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF23_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF23_A {
        match self.bits {
            false => OFVCHF23_A::_0,
            true => OFVCHF23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF23_A::_1
    }
}
#[doc = "Field `OFVCHF24` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF24_R = crate::BitReader<OFVCHF24_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF24_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF24_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF24_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF24_A {
        match self.bits {
            false => OFVCHF24_A::_0,
            true => OFVCHF24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF24_A::_1
    }
}
#[doc = "Field `OFVCHF25` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF25_R = crate::BitReader<OFVCHF25_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF25_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF25_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF25_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF25_A {
        match self.bits {
            false => OFVCHF25_A::_0,
            true => OFVCHF25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF25_A::_1
    }
}
#[doc = "Field `OFVCHF26` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF26_R = crate::BitReader<OFVCHF26_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF26_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF26_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF26_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF26_A {
        match self.bits {
            false => OFVCHF26_A::_0,
            true => OFVCHF26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF26_A::_1
    }
}
#[doc = "Field `OFVCHF27` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF27_R = crate::BitReader<OFVCHF27_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF27_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF27_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF27_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF27_A {
        match self.bits {
            false => OFVCHF27_A::_0,
            true => OFVCHF27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF27_A::_1
    }
}
#[doc = "Field `OFVCHF28` reader - Analog Channel n: Overflow Flag"]
pub type OFVCHF28_R = crate::BitReader<OFVCHF28_A>;
#[doc = "Analog Channel n: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFVCHF28_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OFVCHF28_A> for bool {
    #[inline(always)]
    fn from(variant: OFVCHF28_A) -> Self {
        variant as u8 != 0
    }
}
impl OFVCHF28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFVCHF28_A {
        match self.bits {
            false => OFVCHF28_A::_0,
            true => OFVCHF28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFVCHF28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFVCHF28_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf0(&self) -> OFVCHF0_R {
        OFVCHF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf1(&self) -> OFVCHF1_R {
        OFVCHF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf2(&self) -> OFVCHF2_R {
        OFVCHF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf3(&self) -> OFVCHF3_R {
        OFVCHF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf4(&self) -> OFVCHF4_R {
        OFVCHF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf5(&self) -> OFVCHF5_R {
        OFVCHF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf6(&self) -> OFVCHF6_R {
        OFVCHF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf7(&self) -> OFVCHF7_R {
        OFVCHF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf8(&self) -> OFVCHF8_R {
        OFVCHF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf9(&self) -> OFVCHF9_R {
        OFVCHF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf10(&self) -> OFVCHF10_R {
        OFVCHF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf11(&self) -> OFVCHF11_R {
        OFVCHF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf12(&self) -> OFVCHF12_R {
        OFVCHF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf13(&self) -> OFVCHF13_R {
        OFVCHF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf14(&self) -> OFVCHF14_R {
        OFVCHF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf15(&self) -> OFVCHF15_R {
        OFVCHF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf16(&self) -> OFVCHF16_R {
        OFVCHF16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf17(&self) -> OFVCHF17_R {
        OFVCHF17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf18(&self) -> OFVCHF18_R {
        OFVCHF18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf19(&self) -> OFVCHF19_R {
        OFVCHF19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf20(&self) -> OFVCHF20_R {
        OFVCHF20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf21(&self) -> OFVCHF21_R {
        OFVCHF21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf22(&self) -> OFVCHF22_R {
        OFVCHF22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf23(&self) -> OFVCHF23_R {
        OFVCHF23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf24(&self) -> OFVCHF24_R {
        OFVCHF24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf25(&self) -> OFVCHF25_R {
        OFVCHF25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf26(&self) -> OFVCHF26_R {
        OFVCHF26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf27(&self) -> OFVCHF27_R {
        OFVCHF27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Analog Channel n: Overflow Flag"]
    #[inline(always)]
    pub fn ofvchf28(&self) -> OFVCHF28_R {
        OFVCHF28_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "A/D Conversion Overflow Channel Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adovfchsr0](index.html) module"]
pub struct ADOVFCHSR0_SPEC;
impl crate::RegisterSpec for ADOVFCHSR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adovfchsr0::R](R) reader structure"]
impl crate::Readable for ADOVFCHSR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADOVFCHSR0 to value 0"]
impl crate::Resettable for ADOVFCHSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
