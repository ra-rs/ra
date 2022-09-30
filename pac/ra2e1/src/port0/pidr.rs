#[doc = "Register `PIDR` reader"]
pub struct R(crate::R<PIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PIDR00` reader - Pmn State"]
pub type PIDR00_R = crate::BitReader<PIDR00_A>;
#[doc = "Pmn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
}
#[doc = "Port Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr](index.html) module"]
pub struct PIDR_SPEC;
impl crate::RegisterSpec for PIDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pidr::R](R) reader structure"]
impl crate::Readable for PIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIDR to value 0"]
impl crate::Resettable for PIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
