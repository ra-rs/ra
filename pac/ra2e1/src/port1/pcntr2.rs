#[doc = "Register `PCNTR2` reader"]
pub struct R(crate::R<PCNTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCNTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCNTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCNTR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PIDR00` reader - Pmn State"]
pub type PIDR00_R = crate::BitReader<PIDR00_A>;
#[doc = "Pmn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDR00_A {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<PIDR00_A> for bool {
    #[inline(always)]
    fn from(variant: PIDR00_A) -> Self {
        variant as u8 != 0
    }
}
impl PIDR00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIDR00_A {
        match self.bits {
            false => PIDR00_A::_0,
            true => PIDR00_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDR00_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDR00_A::_1
    }
}
#[doc = "Field `PIDR01` reader - Pmn State"]
pub type PIDR01_R = crate::BitReader<PIDR01_A>;
#[doc = "Pmn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDR01_A {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<PIDR01_A> for bool {
    #[inline(always)]
    fn from(variant: PIDR01_A) -> Self {
        variant as u8 != 0
    }
}
impl PIDR01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIDR01_A {
        match self.bits {
            false => PIDR01_A::_0,
            true => PIDR01_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDR01_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDR01_A::_1
    }
}
#[doc = "Field `PIDR02` reader - Pmn State"]
pub type PIDR02_R = crate::BitReader<PIDR02_A>;
#[doc = "Pmn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDR02_A {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<PIDR02_A> for bool {
    #[inline(always)]
    fn from(variant: PIDR02_A) -> Self {
        variant as u8 != 0
    }
}
impl PIDR02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIDR02_A {
        match self.bits {
            false => PIDR02_A::_0,
            true => PIDR02_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDR02_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDR02_A::_1
    }
}
#[doc = "Field `PIDR03` reader - Pmn State"]
pub type PIDR03_R = crate::BitReader<PIDR03_A>;
#[doc = "Pmn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDR03_A {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<PIDR03_A> for bool {
    #[inline(always)]
    fn from(variant: PIDR03_A) -> Self {
        variant as u8 != 0
    }
}
impl PIDR03_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIDR03_A {
        match self.bits {
            false => PIDR03_A::_0,
            true => PIDR03_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDR03_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDR03_A::_1
    }
}
#[doc = "Field `PIDR04` reader - Pmn State"]
pub type PIDR04_R = crate::BitReader<PIDR04_A>;
#[doc = "Pmn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDR04_A {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<PIDR04_A> for bool {
    #[inline(always)]
    fn from(variant: PIDR04_A) -> Self {
        variant as u8 != 0
    }
}
impl PIDR04_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIDR04_A {
        match self.bits {
            false => PIDR04_A::_0,
            true => PIDR04_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDR04_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDR04_A::_1
    }
}
#[doc = "Field `PIDR05` reader - Pmn State"]
pub type PIDR05_R = crate::BitReader<PIDR05_A>;
#[doc = "Pmn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDR05_A {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<PIDR05_A> for bool {
    #[inline(always)]
    fn from(variant: PIDR05_A) -> Self {
        variant as u8 != 0
    }
}
impl PIDR05_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIDR05_A {
        match self.bits {
            false => PIDR05_A::_0,
            true => PIDR05_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDR05_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDR05_A::_1
    }
}
#[doc = "Field `PIDR06` reader - Pmn State"]
pub type PIDR06_R = crate::BitReader<PIDR06_A>;
#[doc = "Pmn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDR06_A {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<PIDR06_A> for bool {
    #[inline(always)]
    fn from(variant: PIDR06_A) -> Self {
        variant as u8 != 0
    }
}
impl PIDR06_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIDR06_A {
        match self.bits {
            false => PIDR06_A::_0,
            true => PIDR06_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDR06_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDR06_A::_1
    }
}
#[doc = "Field `PIDR07` reader - Pmn State"]
pub type PIDR07_R = crate::BitReader<PIDR07_A>;
#[doc = "Pmn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDR07_A {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<PIDR07_A> for bool {
    #[inline(always)]
    fn from(variant: PIDR07_A) -> Self {
        variant as u8 != 0
    }
}
impl PIDR07_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIDR07_A {
        match self.bits {
            false => PIDR07_A::_0,
            true => PIDR07_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDR07_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDR07_A::_1
    }
}
#[doc = "Field `PIDR08` reader - Pmn State"]
pub type PIDR08_R = crate::BitReader<PIDR08_A>;
#[doc = "Pmn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDR08_A {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<PIDR08_A> for bool {
    #[inline(always)]
    fn from(variant: PIDR08_A) -> Self {
        variant as u8 != 0
    }
}
impl PIDR08_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIDR08_A {
        match self.bits {
            false => PIDR08_A::_0,
            true => PIDR08_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDR08_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDR08_A::_1
    }
}
#[doc = "Field `PIDR09` reader - Pmn State"]
pub type PIDR09_R = crate::BitReader<PIDR09_A>;
#[doc = "Pmn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDR09_A {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<PIDR09_A> for bool {
    #[inline(always)]
    fn from(variant: PIDR09_A) -> Self {
        variant as u8 != 0
    }
}
impl PIDR09_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIDR09_A {
        match self.bits {
            false => PIDR09_A::_0,
            true => PIDR09_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDR09_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDR09_A::_1
    }
}
#[doc = "Field `PIDR10` reader - Pmn State"]
pub type PIDR10_R = crate::BitReader<PIDR10_A>;
#[doc = "Pmn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDR10_A {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<PIDR10_A> for bool {
    #[inline(always)]
    fn from(variant: PIDR10_A) -> Self {
        variant as u8 != 0
    }
}
impl PIDR10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIDR10_A {
        match self.bits {
            false => PIDR10_A::_0,
            true => PIDR10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDR10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDR10_A::_1
    }
}
#[doc = "Field `PIDR11` reader - Pmn State"]
pub type PIDR11_R = crate::BitReader<PIDR11_A>;
#[doc = "Pmn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDR11_A {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<PIDR11_A> for bool {
    #[inline(always)]
    fn from(variant: PIDR11_A) -> Self {
        variant as u8 != 0
    }
}
impl PIDR11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIDR11_A {
        match self.bits {
            false => PIDR11_A::_0,
            true => PIDR11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDR11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDR11_A::_1
    }
}
#[doc = "Field `PIDR12` reader - Pmn State"]
pub type PIDR12_R = crate::BitReader<PIDR12_A>;
#[doc = "Pmn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDR12_A {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<PIDR12_A> for bool {
    #[inline(always)]
    fn from(variant: PIDR12_A) -> Self {
        variant as u8 != 0
    }
}
impl PIDR12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIDR12_A {
        match self.bits {
            false => PIDR12_A::_0,
            true => PIDR12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDR12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDR12_A::_1
    }
}
#[doc = "Field `PIDR13` reader - Pmn State"]
pub type PIDR13_R = crate::BitReader<PIDR13_A>;
#[doc = "Pmn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDR13_A {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<PIDR13_A> for bool {
    #[inline(always)]
    fn from(variant: PIDR13_A) -> Self {
        variant as u8 != 0
    }
}
impl PIDR13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIDR13_A {
        match self.bits {
            false => PIDR13_A::_0,
            true => PIDR13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDR13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDR13_A::_1
    }
}
#[doc = "Field `PIDR14` reader - Pmn State"]
pub type PIDR14_R = crate::BitReader<PIDR14_A>;
#[doc = "Pmn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDR14_A {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<PIDR14_A> for bool {
    #[inline(always)]
    fn from(variant: PIDR14_A) -> Self {
        variant as u8 != 0
    }
}
impl PIDR14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIDR14_A {
        match self.bits {
            false => PIDR14_A::_0,
            true => PIDR14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDR14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDR14_A::_1
    }
}
#[doc = "Field `PIDR15` reader - Pmn State"]
pub type PIDR15_R = crate::BitReader<PIDR15_A>;
#[doc = "Pmn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDR15_A {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<PIDR15_A> for bool {
    #[inline(always)]
    fn from(variant: PIDR15_A) -> Self {
        variant as u8 != 0
    }
}
impl PIDR15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIDR15_A {
        match self.bits {
            false => PIDR15_A::_0,
            true => PIDR15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDR15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDR15_A::_1
    }
}
#[doc = "Field `EIDR00` reader - Port Event Input Data"]
pub type EIDR00_R = crate::BitReader<EIDR00_A>;
#[doc = "Port Event Input Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIDR00_A {
    #[doc = "0: Low input"]
    _0 = 0,
    #[doc = "1: High input"]
    _1 = 1,
}
impl From<EIDR00_A> for bool {
    #[inline(always)]
    fn from(variant: EIDR00_A) -> Self {
        variant as u8 != 0
    }
}
impl EIDR00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIDR00_A {
        match self.bits {
            false => EIDR00_A::_0,
            true => EIDR00_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EIDR00_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EIDR00_A::_1
    }
}
#[doc = "Field `EIDR01` reader - Port Event Input Data"]
pub type EIDR01_R = crate::BitReader<EIDR01_A>;
#[doc = "Port Event Input Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIDR01_A {
    #[doc = "0: Low input"]
    _0 = 0,
    #[doc = "1: High input"]
    _1 = 1,
}
impl From<EIDR01_A> for bool {
    #[inline(always)]
    fn from(variant: EIDR01_A) -> Self {
        variant as u8 != 0
    }
}
impl EIDR01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIDR01_A {
        match self.bits {
            false => EIDR01_A::_0,
            true => EIDR01_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EIDR01_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EIDR01_A::_1
    }
}
#[doc = "Field `EIDR02` reader - Port Event Input Data"]
pub type EIDR02_R = crate::BitReader<EIDR02_A>;
#[doc = "Port Event Input Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIDR02_A {
    #[doc = "0: Low input"]
    _0 = 0,
    #[doc = "1: High input"]
    _1 = 1,
}
impl From<EIDR02_A> for bool {
    #[inline(always)]
    fn from(variant: EIDR02_A) -> Self {
        variant as u8 != 0
    }
}
impl EIDR02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIDR02_A {
        match self.bits {
            false => EIDR02_A::_0,
            true => EIDR02_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EIDR02_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EIDR02_A::_1
    }
}
#[doc = "Field `EIDR03` reader - Port Event Input Data"]
pub type EIDR03_R = crate::BitReader<EIDR03_A>;
#[doc = "Port Event Input Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIDR03_A {
    #[doc = "0: Low input"]
    _0 = 0,
    #[doc = "1: High input"]
    _1 = 1,
}
impl From<EIDR03_A> for bool {
    #[inline(always)]
    fn from(variant: EIDR03_A) -> Self {
        variant as u8 != 0
    }
}
impl EIDR03_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIDR03_A {
        match self.bits {
            false => EIDR03_A::_0,
            true => EIDR03_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EIDR03_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EIDR03_A::_1
    }
}
#[doc = "Field `EIDR04` reader - Port Event Input Data"]
pub type EIDR04_R = crate::BitReader<EIDR04_A>;
#[doc = "Port Event Input Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIDR04_A {
    #[doc = "0: Low input"]
    _0 = 0,
    #[doc = "1: High input"]
    _1 = 1,
}
impl From<EIDR04_A> for bool {
    #[inline(always)]
    fn from(variant: EIDR04_A) -> Self {
        variant as u8 != 0
    }
}
impl EIDR04_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIDR04_A {
        match self.bits {
            false => EIDR04_A::_0,
            true => EIDR04_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EIDR04_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EIDR04_A::_1
    }
}
#[doc = "Field `EIDR05` reader - Port Event Input Data"]
pub type EIDR05_R = crate::BitReader<EIDR05_A>;
#[doc = "Port Event Input Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIDR05_A {
    #[doc = "0: Low input"]
    _0 = 0,
    #[doc = "1: High input"]
    _1 = 1,
}
impl From<EIDR05_A> for bool {
    #[inline(always)]
    fn from(variant: EIDR05_A) -> Self {
        variant as u8 != 0
    }
}
impl EIDR05_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIDR05_A {
        match self.bits {
            false => EIDR05_A::_0,
            true => EIDR05_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EIDR05_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EIDR05_A::_1
    }
}
#[doc = "Field `EIDR06` reader - Port Event Input Data"]
pub type EIDR06_R = crate::BitReader<EIDR06_A>;
#[doc = "Port Event Input Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIDR06_A {
    #[doc = "0: Low input"]
    _0 = 0,
    #[doc = "1: High input"]
    _1 = 1,
}
impl From<EIDR06_A> for bool {
    #[inline(always)]
    fn from(variant: EIDR06_A) -> Self {
        variant as u8 != 0
    }
}
impl EIDR06_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIDR06_A {
        match self.bits {
            false => EIDR06_A::_0,
            true => EIDR06_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EIDR06_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EIDR06_A::_1
    }
}
#[doc = "Field `EIDR07` reader - Port Event Input Data"]
pub type EIDR07_R = crate::BitReader<EIDR07_A>;
#[doc = "Port Event Input Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIDR07_A {
    #[doc = "0: Low input"]
    _0 = 0,
    #[doc = "1: High input"]
    _1 = 1,
}
impl From<EIDR07_A> for bool {
    #[inline(always)]
    fn from(variant: EIDR07_A) -> Self {
        variant as u8 != 0
    }
}
impl EIDR07_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIDR07_A {
        match self.bits {
            false => EIDR07_A::_0,
            true => EIDR07_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EIDR07_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EIDR07_A::_1
    }
}
#[doc = "Field `EIDR08` reader - Port Event Input Data"]
pub type EIDR08_R = crate::BitReader<EIDR08_A>;
#[doc = "Port Event Input Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIDR08_A {
    #[doc = "0: Low input"]
    _0 = 0,
    #[doc = "1: High input"]
    _1 = 1,
}
impl From<EIDR08_A> for bool {
    #[inline(always)]
    fn from(variant: EIDR08_A) -> Self {
        variant as u8 != 0
    }
}
impl EIDR08_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIDR08_A {
        match self.bits {
            false => EIDR08_A::_0,
            true => EIDR08_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EIDR08_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EIDR08_A::_1
    }
}
#[doc = "Field `EIDR09` reader - Port Event Input Data"]
pub type EIDR09_R = crate::BitReader<EIDR09_A>;
#[doc = "Port Event Input Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIDR09_A {
    #[doc = "0: Low input"]
    _0 = 0,
    #[doc = "1: High input"]
    _1 = 1,
}
impl From<EIDR09_A> for bool {
    #[inline(always)]
    fn from(variant: EIDR09_A) -> Self {
        variant as u8 != 0
    }
}
impl EIDR09_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIDR09_A {
        match self.bits {
            false => EIDR09_A::_0,
            true => EIDR09_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EIDR09_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EIDR09_A::_1
    }
}
#[doc = "Field `EIDR10` reader - Port Event Input Data"]
pub type EIDR10_R = crate::BitReader<EIDR10_A>;
#[doc = "Port Event Input Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIDR10_A {
    #[doc = "0: Low input"]
    _0 = 0,
    #[doc = "1: High input"]
    _1 = 1,
}
impl From<EIDR10_A> for bool {
    #[inline(always)]
    fn from(variant: EIDR10_A) -> Self {
        variant as u8 != 0
    }
}
impl EIDR10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIDR10_A {
        match self.bits {
            false => EIDR10_A::_0,
            true => EIDR10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EIDR10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EIDR10_A::_1
    }
}
#[doc = "Field `EIDR11` reader - Port Event Input Data"]
pub type EIDR11_R = crate::BitReader<EIDR11_A>;
#[doc = "Port Event Input Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIDR11_A {
    #[doc = "0: Low input"]
    _0 = 0,
    #[doc = "1: High input"]
    _1 = 1,
}
impl From<EIDR11_A> for bool {
    #[inline(always)]
    fn from(variant: EIDR11_A) -> Self {
        variant as u8 != 0
    }
}
impl EIDR11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIDR11_A {
        match self.bits {
            false => EIDR11_A::_0,
            true => EIDR11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EIDR11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EIDR11_A::_1
    }
}
#[doc = "Field `EIDR12` reader - Port Event Input Data"]
pub type EIDR12_R = crate::BitReader<EIDR12_A>;
#[doc = "Port Event Input Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIDR12_A {
    #[doc = "0: Low input"]
    _0 = 0,
    #[doc = "1: High input"]
    _1 = 1,
}
impl From<EIDR12_A> for bool {
    #[inline(always)]
    fn from(variant: EIDR12_A) -> Self {
        variant as u8 != 0
    }
}
impl EIDR12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIDR12_A {
        match self.bits {
            false => EIDR12_A::_0,
            true => EIDR12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EIDR12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EIDR12_A::_1
    }
}
#[doc = "Field `EIDR13` reader - Port Event Input Data"]
pub type EIDR13_R = crate::BitReader<EIDR13_A>;
#[doc = "Port Event Input Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIDR13_A {
    #[doc = "0: Low input"]
    _0 = 0,
    #[doc = "1: High input"]
    _1 = 1,
}
impl From<EIDR13_A> for bool {
    #[inline(always)]
    fn from(variant: EIDR13_A) -> Self {
        variant as u8 != 0
    }
}
impl EIDR13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIDR13_A {
        match self.bits {
            false => EIDR13_A::_0,
            true => EIDR13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EIDR13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EIDR13_A::_1
    }
}
#[doc = "Field `EIDR14` reader - Port Event Input Data"]
pub type EIDR14_R = crate::BitReader<EIDR14_A>;
#[doc = "Port Event Input Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIDR14_A {
    #[doc = "0: Low input"]
    _0 = 0,
    #[doc = "1: High input"]
    _1 = 1,
}
impl From<EIDR14_A> for bool {
    #[inline(always)]
    fn from(variant: EIDR14_A) -> Self {
        variant as u8 != 0
    }
}
impl EIDR14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIDR14_A {
        match self.bits {
            false => EIDR14_A::_0,
            true => EIDR14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EIDR14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EIDR14_A::_1
    }
}
#[doc = "Field `EIDR15` reader - Port Event Input Data"]
pub type EIDR15_R = crate::BitReader<EIDR15_A>;
#[doc = "Port Event Input Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIDR15_A {
    #[doc = "0: Low input"]
    _0 = 0,
    #[doc = "1: High input"]
    _1 = 1,
}
impl From<EIDR15_A> for bool {
    #[inline(always)]
    fn from(variant: EIDR15_A) -> Self {
        variant as u8 != 0
    }
}
impl EIDR15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIDR15_A {
        match self.bits {
            false => EIDR15_A::_0,
            true => EIDR15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EIDR15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EIDR15_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Pmn State"]
    #[inline(always)]
    pub fn pidr00(&self) -> PIDR00_R {
        PIDR00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pmn State"]
    #[inline(always)]
    pub fn pidr01(&self) -> PIDR01_R {
        PIDR01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pmn State"]
    #[inline(always)]
    pub fn pidr02(&self) -> PIDR02_R {
        PIDR02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pmn State"]
    #[inline(always)]
    pub fn pidr03(&self) -> PIDR03_R {
        PIDR03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pmn State"]
    #[inline(always)]
    pub fn pidr04(&self) -> PIDR04_R {
        PIDR04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pmn State"]
    #[inline(always)]
    pub fn pidr05(&self) -> PIDR05_R {
        PIDR05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pmn State"]
    #[inline(always)]
    pub fn pidr06(&self) -> PIDR06_R {
        PIDR06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pmn State"]
    #[inline(always)]
    pub fn pidr07(&self) -> PIDR07_R {
        PIDR07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pmn State"]
    #[inline(always)]
    pub fn pidr08(&self) -> PIDR08_R {
        PIDR08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pmn State"]
    #[inline(always)]
    pub fn pidr09(&self) -> PIDR09_R {
        PIDR09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pmn State"]
    #[inline(always)]
    pub fn pidr10(&self) -> PIDR10_R {
        PIDR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pmn State"]
    #[inline(always)]
    pub fn pidr11(&self) -> PIDR11_R {
        PIDR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pmn State"]
    #[inline(always)]
    pub fn pidr12(&self) -> PIDR12_R {
        PIDR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pmn State"]
    #[inline(always)]
    pub fn pidr13(&self) -> PIDR13_R {
        PIDR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pmn State"]
    #[inline(always)]
    pub fn pidr14(&self) -> PIDR14_R {
        PIDR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pmn State"]
    #[inline(always)]
    pub fn pidr15(&self) -> PIDR15_R {
        PIDR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr00(&self) -> EIDR00_R {
        EIDR00_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr01(&self) -> EIDR01_R {
        EIDR01_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr02(&self) -> EIDR02_R {
        EIDR02_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr03(&self) -> EIDR03_R {
        EIDR03_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr04(&self) -> EIDR04_R {
        EIDR04_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr05(&self) -> EIDR05_R {
        EIDR05_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr06(&self) -> EIDR06_R {
        EIDR06_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr07(&self) -> EIDR07_R {
        EIDR07_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr08(&self) -> EIDR08_R {
        EIDR08_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr09(&self) -> EIDR09_R {
        EIDR09_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr10(&self) -> EIDR10_R {
        EIDR10_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr11(&self) -> EIDR11_R {
        EIDR11_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr12(&self) -> EIDR12_R {
        EIDR12_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr13(&self) -> EIDR13_R {
        EIDR13_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr14(&self) -> EIDR14_R {
        EIDR14_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr15(&self) -> EIDR15_R {
        EIDR15_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Port Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcntr2](index.html) module"]
pub struct PCNTR2_SPEC;
impl crate::RegisterSpec for PCNTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcntr2::R](R) reader structure"]
impl crate::Readable for PCNTR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PCNTR2 to value 0"]
impl crate::Resettable for PCNTR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
