#[doc = "Register `ADFIFOERSR` reader"]
pub struct R(crate::R<ADFIFOERSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADFIFOERSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADFIFOERSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADFIFOERSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFOOVF0` reader - Scan Group n FIFO Overflow Flag"]
pub type FIFOOVF0_R = crate::BitReader<FIFOOVF0_A>;
#[doc = "Scan Group n FIFO Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOOVF0_A {
    #[doc = "0: No overflow"]
    _0 = 0,
    #[doc = "1: FIFO overflow is detected"]
    _1 = 1,
}
impl From<FIFOOVF0_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOOVF0_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOOVF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOOVF0_A {
        match self.bits {
            false => FIFOOVF0_A::_0,
            true => FIFOOVF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOOVF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOOVF0_A::_1
    }
}
#[doc = "Field `FIFOOVF1` reader - Scan Group n FIFO Overflow Flag"]
pub type FIFOOVF1_R = crate::BitReader<FIFOOVF1_A>;
#[doc = "Scan Group n FIFO Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOOVF1_A {
    #[doc = "0: No overflow"]
    _0 = 0,
    #[doc = "1: FIFO overflow is detected"]
    _1 = 1,
}
impl From<FIFOOVF1_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOOVF1_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOOVF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOOVF1_A {
        match self.bits {
            false => FIFOOVF1_A::_0,
            true => FIFOOVF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOOVF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOOVF1_A::_1
    }
}
#[doc = "Field `FIFOOVF2` reader - Scan Group n FIFO Overflow Flag"]
pub type FIFOOVF2_R = crate::BitReader<FIFOOVF2_A>;
#[doc = "Scan Group n FIFO Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOOVF2_A {
    #[doc = "0: No overflow"]
    _0 = 0,
    #[doc = "1: FIFO overflow is detected"]
    _1 = 1,
}
impl From<FIFOOVF2_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOOVF2_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOOVF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOOVF2_A {
        match self.bits {
            false => FIFOOVF2_A::_0,
            true => FIFOOVF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOOVF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOOVF2_A::_1
    }
}
#[doc = "Field `FIFOOVF3` reader - Scan Group n FIFO Overflow Flag"]
pub type FIFOOVF3_R = crate::BitReader<FIFOOVF3_A>;
#[doc = "Scan Group n FIFO Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOOVF3_A {
    #[doc = "0: No overflow"]
    _0 = 0,
    #[doc = "1: FIFO overflow is detected"]
    _1 = 1,
}
impl From<FIFOOVF3_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOOVF3_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOOVF3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOOVF3_A {
        match self.bits {
            false => FIFOOVF3_A::_0,
            true => FIFOOVF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOOVF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOOVF3_A::_1
    }
}
#[doc = "Field `FIFOOVF4` reader - Scan Group n FIFO Overflow Flag"]
pub type FIFOOVF4_R = crate::BitReader<FIFOOVF4_A>;
#[doc = "Scan Group n FIFO Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOOVF4_A {
    #[doc = "0: No overflow"]
    _0 = 0,
    #[doc = "1: FIFO overflow is detected"]
    _1 = 1,
}
impl From<FIFOOVF4_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOOVF4_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOOVF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOOVF4_A {
        match self.bits {
            false => FIFOOVF4_A::_0,
            true => FIFOOVF4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOOVF4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOOVF4_A::_1
    }
}
#[doc = "Field `FIFOOVF5` reader - Scan Group n FIFO Overflow Flag"]
pub type FIFOOVF5_R = crate::BitReader<FIFOOVF5_A>;
#[doc = "Scan Group n FIFO Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOOVF5_A {
    #[doc = "0: No overflow"]
    _0 = 0,
    #[doc = "1: FIFO overflow is detected"]
    _1 = 1,
}
impl From<FIFOOVF5_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOOVF5_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOOVF5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOOVF5_A {
        match self.bits {
            false => FIFOOVF5_A::_0,
            true => FIFOOVF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOOVF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOOVF5_A::_1
    }
}
#[doc = "Field `FIFOOVF6` reader - Scan Group n FIFO Overflow Flag"]
pub type FIFOOVF6_R = crate::BitReader<FIFOOVF6_A>;
#[doc = "Scan Group n FIFO Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOOVF6_A {
    #[doc = "0: No overflow"]
    _0 = 0,
    #[doc = "1: FIFO overflow is detected"]
    _1 = 1,
}
impl From<FIFOOVF6_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOOVF6_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOOVF6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOOVF6_A {
        match self.bits {
            false => FIFOOVF6_A::_0,
            true => FIFOOVF6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOOVF6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOOVF6_A::_1
    }
}
#[doc = "Field `FIFOOVF7` reader - Scan Group n FIFO Overflow Flag"]
pub type FIFOOVF7_R = crate::BitReader<FIFOOVF7_A>;
#[doc = "Scan Group n FIFO Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOOVF7_A {
    #[doc = "0: No overflow"]
    _0 = 0,
    #[doc = "1: FIFO overflow is detected"]
    _1 = 1,
}
impl From<FIFOOVF7_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOOVF7_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOOVF7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOOVF7_A {
        match self.bits {
            false => FIFOOVF7_A::_0,
            true => FIFOOVF7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOOVF7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOOVF7_A::_1
    }
}
#[doc = "Field `FIFOOVF8` reader - Scan Group n FIFO Overflow Flag"]
pub type FIFOOVF8_R = crate::BitReader<FIFOOVF8_A>;
#[doc = "Scan Group n FIFO Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOOVF8_A {
    #[doc = "0: No overflow"]
    _0 = 0,
    #[doc = "1: FIFO overflow is detected"]
    _1 = 1,
}
impl From<FIFOOVF8_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOOVF8_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOOVF8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOOVF8_A {
        match self.bits {
            false => FIFOOVF8_A::_0,
            true => FIFOOVF8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOOVF8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOOVF8_A::_1
    }
}
#[doc = "Field `FIFOFLF0` reader - Scan Group n FIFO Data Read Request Flag"]
pub type FIFOFLF0_R = crate::BitReader<FIFOFLF0_A>;
#[doc = "Scan Group n FIFO Data Read Request Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOFLF0_A {
    #[doc = "0: FIFO Data Read Request is not detected."]
    _0 = 0,
    #[doc = "1: FIFO Data Read Request is detected."]
    _1 = 1,
}
impl From<FIFOFLF0_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOFLF0_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOFLF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOFLF0_A {
        match self.bits {
            false => FIFOFLF0_A::_0,
            true => FIFOFLF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOFLF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOFLF0_A::_1
    }
}
#[doc = "Field `FIFOFLF1` reader - Scan Group n FIFO Data Read Request Flag"]
pub type FIFOFLF1_R = crate::BitReader<FIFOFLF1_A>;
#[doc = "Scan Group n FIFO Data Read Request Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOFLF1_A {
    #[doc = "0: FIFO Data Read Request is not detected."]
    _0 = 0,
    #[doc = "1: FIFO Data Read Request is detected."]
    _1 = 1,
}
impl From<FIFOFLF1_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOFLF1_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOFLF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOFLF1_A {
        match self.bits {
            false => FIFOFLF1_A::_0,
            true => FIFOFLF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOFLF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOFLF1_A::_1
    }
}
#[doc = "Field `FIFOFLF2` reader - Scan Group n FIFO Data Read Request Flag"]
pub type FIFOFLF2_R = crate::BitReader<FIFOFLF2_A>;
#[doc = "Scan Group n FIFO Data Read Request Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOFLF2_A {
    #[doc = "0: FIFO Data Read Request is not detected."]
    _0 = 0,
    #[doc = "1: FIFO Data Read Request is detected."]
    _1 = 1,
}
impl From<FIFOFLF2_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOFLF2_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOFLF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOFLF2_A {
        match self.bits {
            false => FIFOFLF2_A::_0,
            true => FIFOFLF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOFLF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOFLF2_A::_1
    }
}
#[doc = "Field `FIFOFLF3` reader - Scan Group n FIFO Data Read Request Flag"]
pub type FIFOFLF3_R = crate::BitReader<FIFOFLF3_A>;
#[doc = "Scan Group n FIFO Data Read Request Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOFLF3_A {
    #[doc = "0: FIFO Data Read Request is not detected."]
    _0 = 0,
    #[doc = "1: FIFO Data Read Request is detected."]
    _1 = 1,
}
impl From<FIFOFLF3_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOFLF3_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOFLF3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOFLF3_A {
        match self.bits {
            false => FIFOFLF3_A::_0,
            true => FIFOFLF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOFLF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOFLF3_A::_1
    }
}
#[doc = "Field `FIFOFLF4` reader - Scan Group n FIFO Data Read Request Flag"]
pub type FIFOFLF4_R = crate::BitReader<FIFOFLF4_A>;
#[doc = "Scan Group n FIFO Data Read Request Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOFLF4_A {
    #[doc = "0: FIFO Data Read Request is not detected."]
    _0 = 0,
    #[doc = "1: FIFO Data Read Request is detected."]
    _1 = 1,
}
impl From<FIFOFLF4_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOFLF4_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOFLF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOFLF4_A {
        match self.bits {
            false => FIFOFLF4_A::_0,
            true => FIFOFLF4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOFLF4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOFLF4_A::_1
    }
}
#[doc = "Field `FIFOFLF5` reader - Scan Group n FIFO Data Read Request Flag"]
pub type FIFOFLF5_R = crate::BitReader<FIFOFLF5_A>;
#[doc = "Scan Group n FIFO Data Read Request Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOFLF5_A {
    #[doc = "0: FIFO Data Read Request is not detected."]
    _0 = 0,
    #[doc = "1: FIFO Data Read Request is detected."]
    _1 = 1,
}
impl From<FIFOFLF5_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOFLF5_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOFLF5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOFLF5_A {
        match self.bits {
            false => FIFOFLF5_A::_0,
            true => FIFOFLF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOFLF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOFLF5_A::_1
    }
}
#[doc = "Field `FIFOFLF6` reader - Scan Group n FIFO Data Read Request Flag"]
pub type FIFOFLF6_R = crate::BitReader<FIFOFLF6_A>;
#[doc = "Scan Group n FIFO Data Read Request Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOFLF6_A {
    #[doc = "0: FIFO Data Read Request is not detected."]
    _0 = 0,
    #[doc = "1: FIFO Data Read Request is detected."]
    _1 = 1,
}
impl From<FIFOFLF6_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOFLF6_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOFLF6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOFLF6_A {
        match self.bits {
            false => FIFOFLF6_A::_0,
            true => FIFOFLF6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOFLF6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOFLF6_A::_1
    }
}
#[doc = "Field `FIFOFLF7` reader - Scan Group n FIFO Data Read Request Flag"]
pub type FIFOFLF7_R = crate::BitReader<FIFOFLF7_A>;
#[doc = "Scan Group n FIFO Data Read Request Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOFLF7_A {
    #[doc = "0: FIFO Data Read Request is not detected."]
    _0 = 0,
    #[doc = "1: FIFO Data Read Request is detected."]
    _1 = 1,
}
impl From<FIFOFLF7_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOFLF7_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOFLF7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOFLF7_A {
        match self.bits {
            false => FIFOFLF7_A::_0,
            true => FIFOFLF7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOFLF7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOFLF7_A::_1
    }
}
#[doc = "Field `FIFOFLF8` reader - Scan Group n FIFO Data Read Request Flag"]
pub type FIFOFLF8_R = crate::BitReader<FIFOFLF8_A>;
#[doc = "Scan Group n FIFO Data Read Request Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOFLF8_A {
    #[doc = "0: FIFO Data Read Request is not detected."]
    _0 = 0,
    #[doc = "1: FIFO Data Read Request is detected."]
    _1 = 1,
}
impl From<FIFOFLF8_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOFLF8_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOFLF8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOFLF8_A {
        match self.bits {
            false => FIFOFLF8_A::_0,
            true => FIFOFLF8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOFLF8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOFLF8_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Scan Group n FIFO Overflow Flag"]
    #[inline(always)]
    pub fn fifoovf0(&self) -> FIFOOVF0_R {
        FIFOOVF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan Group n FIFO Overflow Flag"]
    #[inline(always)]
    pub fn fifoovf1(&self) -> FIFOOVF1_R {
        FIFOOVF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Scan Group n FIFO Overflow Flag"]
    #[inline(always)]
    pub fn fifoovf2(&self) -> FIFOOVF2_R {
        FIFOOVF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Scan Group n FIFO Overflow Flag"]
    #[inline(always)]
    pub fn fifoovf3(&self) -> FIFOOVF3_R {
        FIFOOVF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Scan Group n FIFO Overflow Flag"]
    #[inline(always)]
    pub fn fifoovf4(&self) -> FIFOOVF4_R {
        FIFOOVF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Scan Group n FIFO Overflow Flag"]
    #[inline(always)]
    pub fn fifoovf5(&self) -> FIFOOVF5_R {
        FIFOOVF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Scan Group n FIFO Overflow Flag"]
    #[inline(always)]
    pub fn fifoovf6(&self) -> FIFOOVF6_R {
        FIFOOVF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Scan Group n FIFO Overflow Flag"]
    #[inline(always)]
    pub fn fifoovf7(&self) -> FIFOOVF7_R {
        FIFOOVF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Scan Group n FIFO Overflow Flag"]
    #[inline(always)]
    pub fn fifoovf8(&self) -> FIFOOVF8_R {
        FIFOOVF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Scan Group n FIFO Data Read Request Flag"]
    #[inline(always)]
    pub fn fifoflf0(&self) -> FIFOFLF0_R {
        FIFOFLF0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Scan Group n FIFO Data Read Request Flag"]
    #[inline(always)]
    pub fn fifoflf1(&self) -> FIFOFLF1_R {
        FIFOFLF1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Scan Group n FIFO Data Read Request Flag"]
    #[inline(always)]
    pub fn fifoflf2(&self) -> FIFOFLF2_R {
        FIFOFLF2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Scan Group n FIFO Data Read Request Flag"]
    #[inline(always)]
    pub fn fifoflf3(&self) -> FIFOFLF3_R {
        FIFOFLF3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Scan Group n FIFO Data Read Request Flag"]
    #[inline(always)]
    pub fn fifoflf4(&self) -> FIFOFLF4_R {
        FIFOFLF4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Scan Group n FIFO Data Read Request Flag"]
    #[inline(always)]
    pub fn fifoflf5(&self) -> FIFOFLF5_R {
        FIFOFLF5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Scan Group n FIFO Data Read Request Flag"]
    #[inline(always)]
    pub fn fifoflf6(&self) -> FIFOFLF6_R {
        FIFOFLF6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Scan Group n FIFO Data Read Request Flag"]
    #[inline(always)]
    pub fn fifoflf7(&self) -> FIFOFLF7_R {
        FIFOFLF7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Scan Group n FIFO Data Read Request Flag"]
    #[inline(always)]
    pub fn fifoflf8(&self) -> FIFOFLF8_R {
        FIFOFLF8_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "FIFO Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adfifoersr](index.html) module"]
pub struct ADFIFOERSR_SPEC;
impl crate::RegisterSpec for ADFIFOERSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adfifoersr::R](R) reader structure"]
impl crate::Readable for ADFIFOERSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADFIFOERSR to value 0"]
impl crate::Resettable for ADFIFOERSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
