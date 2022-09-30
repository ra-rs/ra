#[doc = "Register `EIDR` reader"]
pub struct R(crate::R<EIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EIDR00` reader - Port Event Input Data"]
pub type EIDR00_R = crate::BitReader<EIDR00_A>;
#[doc = "Port Event Input Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[doc = "Bit 0 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr00(&self) -> EIDR00_R {
        EIDR00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr01(&self) -> EIDR01_R {
        EIDR01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr02(&self) -> EIDR02_R {
        EIDR02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr03(&self) -> EIDR03_R {
        EIDR03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr04(&self) -> EIDR04_R {
        EIDR04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr05(&self) -> EIDR05_R {
        EIDR05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr06(&self) -> EIDR06_R {
        EIDR06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr07(&self) -> EIDR07_R {
        EIDR07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr08(&self) -> EIDR08_R {
        EIDR08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr09(&self) -> EIDR09_R {
        EIDR09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr10(&self) -> EIDR10_R {
        EIDR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr11(&self) -> EIDR11_R {
        EIDR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr12(&self) -> EIDR12_R {
        EIDR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr13(&self) -> EIDR13_R {
        EIDR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr14(&self) -> EIDR14_R {
        EIDR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port Event Input Data"]
    #[inline(always)]
    pub fn eidr15(&self) -> EIDR15_R {
        EIDR15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Port Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eidr](index.html) module"]
pub struct EIDR_SPEC;
impl crate::RegisterSpec for EIDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [eidr::R](R) reader structure"]
impl crate::Readable for EIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EIDR to value 0"]
impl crate::Resettable for EIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
