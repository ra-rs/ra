#[doc = "Register `PODR` reader"]
pub struct R(crate::R<PODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PODR` writer"]
pub struct W(crate::W<PODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PODR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PODR00` reader - Pmn Output Data"]
pub type PODR00_R = crate::BitReader<PODR00_A>;
#[doc = "Pmn Output Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PODR00_A {
    #[doc = "0: Low output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<PODR00_A> for bool {
    #[inline(always)]
    fn from(variant: PODR00_A) -> Self {
        variant as u8 != 0
    }
}
impl PODR00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PODR00_A {
        match self.bits {
            false => PODR00_A::_0,
            true => PODR00_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PODR00_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PODR00_A::_1
    }
}
#[doc = "Field `PODR00` writer - Pmn Output Data"]
pub type PODR00_W<'a, const O: u8> = crate::BitWriter<'a, u16, PODR_SPEC, PODR00_A, O>;
impl<'a, const O: u8> PODR00_W<'a, O> {
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PODR00_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PODR00_A::_1)
    }
}
#[doc = "Field `PODR01` reader - Pmn Output Data"]
pub type PODR01_R = crate::BitReader<PODR01_A>;
#[doc = "Pmn Output Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PODR01_A {
    #[doc = "0: Low output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<PODR01_A> for bool {
    #[inline(always)]
    fn from(variant: PODR01_A) -> Self {
        variant as u8 != 0
    }
}
impl PODR01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PODR01_A {
        match self.bits {
            false => PODR01_A::_0,
            true => PODR01_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PODR01_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PODR01_A::_1
    }
}
#[doc = "Field `PODR01` writer - Pmn Output Data"]
pub type PODR01_W<'a, const O: u8> = crate::BitWriter<'a, u16, PODR_SPEC, PODR01_A, O>;
impl<'a, const O: u8> PODR01_W<'a, O> {
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PODR01_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PODR01_A::_1)
    }
}
#[doc = "Field `PODR02` reader - Pmn Output Data"]
pub type PODR02_R = crate::BitReader<PODR02_A>;
#[doc = "Pmn Output Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PODR02_A {
    #[doc = "0: Low output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<PODR02_A> for bool {
    #[inline(always)]
    fn from(variant: PODR02_A) -> Self {
        variant as u8 != 0
    }
}
impl PODR02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PODR02_A {
        match self.bits {
            false => PODR02_A::_0,
            true => PODR02_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PODR02_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PODR02_A::_1
    }
}
#[doc = "Field `PODR02` writer - Pmn Output Data"]
pub type PODR02_W<'a, const O: u8> = crate::BitWriter<'a, u16, PODR_SPEC, PODR02_A, O>;
impl<'a, const O: u8> PODR02_W<'a, O> {
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PODR02_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PODR02_A::_1)
    }
}
#[doc = "Field `PODR03` reader - Pmn Output Data"]
pub type PODR03_R = crate::BitReader<PODR03_A>;
#[doc = "Pmn Output Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PODR03_A {
    #[doc = "0: Low output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<PODR03_A> for bool {
    #[inline(always)]
    fn from(variant: PODR03_A) -> Self {
        variant as u8 != 0
    }
}
impl PODR03_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PODR03_A {
        match self.bits {
            false => PODR03_A::_0,
            true => PODR03_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PODR03_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PODR03_A::_1
    }
}
#[doc = "Field `PODR03` writer - Pmn Output Data"]
pub type PODR03_W<'a, const O: u8> = crate::BitWriter<'a, u16, PODR_SPEC, PODR03_A, O>;
impl<'a, const O: u8> PODR03_W<'a, O> {
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PODR03_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PODR03_A::_1)
    }
}
#[doc = "Field `PODR04` reader - Pmn Output Data"]
pub type PODR04_R = crate::BitReader<PODR04_A>;
#[doc = "Pmn Output Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PODR04_A {
    #[doc = "0: Low output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<PODR04_A> for bool {
    #[inline(always)]
    fn from(variant: PODR04_A) -> Self {
        variant as u8 != 0
    }
}
impl PODR04_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PODR04_A {
        match self.bits {
            false => PODR04_A::_0,
            true => PODR04_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PODR04_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PODR04_A::_1
    }
}
#[doc = "Field `PODR04` writer - Pmn Output Data"]
pub type PODR04_W<'a, const O: u8> = crate::BitWriter<'a, u16, PODR_SPEC, PODR04_A, O>;
impl<'a, const O: u8> PODR04_W<'a, O> {
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PODR04_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PODR04_A::_1)
    }
}
#[doc = "Field `PODR05` reader - Pmn Output Data"]
pub type PODR05_R = crate::BitReader<PODR05_A>;
#[doc = "Pmn Output Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PODR05_A {
    #[doc = "0: Low output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<PODR05_A> for bool {
    #[inline(always)]
    fn from(variant: PODR05_A) -> Self {
        variant as u8 != 0
    }
}
impl PODR05_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PODR05_A {
        match self.bits {
            false => PODR05_A::_0,
            true => PODR05_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PODR05_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PODR05_A::_1
    }
}
#[doc = "Field `PODR05` writer - Pmn Output Data"]
pub type PODR05_W<'a, const O: u8> = crate::BitWriter<'a, u16, PODR_SPEC, PODR05_A, O>;
impl<'a, const O: u8> PODR05_W<'a, O> {
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PODR05_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PODR05_A::_1)
    }
}
#[doc = "Field `PODR06` reader - Pmn Output Data"]
pub type PODR06_R = crate::BitReader<PODR06_A>;
#[doc = "Pmn Output Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PODR06_A {
    #[doc = "0: Low output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<PODR06_A> for bool {
    #[inline(always)]
    fn from(variant: PODR06_A) -> Self {
        variant as u8 != 0
    }
}
impl PODR06_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PODR06_A {
        match self.bits {
            false => PODR06_A::_0,
            true => PODR06_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PODR06_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PODR06_A::_1
    }
}
#[doc = "Field `PODR06` writer - Pmn Output Data"]
pub type PODR06_W<'a, const O: u8> = crate::BitWriter<'a, u16, PODR_SPEC, PODR06_A, O>;
impl<'a, const O: u8> PODR06_W<'a, O> {
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PODR06_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PODR06_A::_1)
    }
}
#[doc = "Field `PODR07` reader - Pmn Output Data"]
pub type PODR07_R = crate::BitReader<PODR07_A>;
#[doc = "Pmn Output Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PODR07_A {
    #[doc = "0: Low output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<PODR07_A> for bool {
    #[inline(always)]
    fn from(variant: PODR07_A) -> Self {
        variant as u8 != 0
    }
}
impl PODR07_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PODR07_A {
        match self.bits {
            false => PODR07_A::_0,
            true => PODR07_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PODR07_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PODR07_A::_1
    }
}
#[doc = "Field `PODR07` writer - Pmn Output Data"]
pub type PODR07_W<'a, const O: u8> = crate::BitWriter<'a, u16, PODR_SPEC, PODR07_A, O>;
impl<'a, const O: u8> PODR07_W<'a, O> {
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PODR07_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PODR07_A::_1)
    }
}
#[doc = "Field `PODR08` reader - Pmn Output Data"]
pub type PODR08_R = crate::BitReader<PODR08_A>;
#[doc = "Pmn Output Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PODR08_A {
    #[doc = "0: Low output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<PODR08_A> for bool {
    #[inline(always)]
    fn from(variant: PODR08_A) -> Self {
        variant as u8 != 0
    }
}
impl PODR08_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PODR08_A {
        match self.bits {
            false => PODR08_A::_0,
            true => PODR08_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PODR08_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PODR08_A::_1
    }
}
#[doc = "Field `PODR08` writer - Pmn Output Data"]
pub type PODR08_W<'a, const O: u8> = crate::BitWriter<'a, u16, PODR_SPEC, PODR08_A, O>;
impl<'a, const O: u8> PODR08_W<'a, O> {
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PODR08_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PODR08_A::_1)
    }
}
#[doc = "Field `PODR09` reader - Pmn Output Data"]
pub type PODR09_R = crate::BitReader<PODR09_A>;
#[doc = "Pmn Output Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PODR09_A {
    #[doc = "0: Low output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<PODR09_A> for bool {
    #[inline(always)]
    fn from(variant: PODR09_A) -> Self {
        variant as u8 != 0
    }
}
impl PODR09_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PODR09_A {
        match self.bits {
            false => PODR09_A::_0,
            true => PODR09_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PODR09_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PODR09_A::_1
    }
}
#[doc = "Field `PODR09` writer - Pmn Output Data"]
pub type PODR09_W<'a, const O: u8> = crate::BitWriter<'a, u16, PODR_SPEC, PODR09_A, O>;
impl<'a, const O: u8> PODR09_W<'a, O> {
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PODR09_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PODR09_A::_1)
    }
}
#[doc = "Field `PODR10` reader - Pmn Output Data"]
pub type PODR10_R = crate::BitReader<PODR10_A>;
#[doc = "Pmn Output Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PODR10_A {
    #[doc = "0: Low output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<PODR10_A> for bool {
    #[inline(always)]
    fn from(variant: PODR10_A) -> Self {
        variant as u8 != 0
    }
}
impl PODR10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PODR10_A {
        match self.bits {
            false => PODR10_A::_0,
            true => PODR10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PODR10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PODR10_A::_1
    }
}
#[doc = "Field `PODR10` writer - Pmn Output Data"]
pub type PODR10_W<'a, const O: u8> = crate::BitWriter<'a, u16, PODR_SPEC, PODR10_A, O>;
impl<'a, const O: u8> PODR10_W<'a, O> {
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PODR10_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PODR10_A::_1)
    }
}
#[doc = "Field `PODR11` reader - Pmn Output Data"]
pub type PODR11_R = crate::BitReader<PODR11_A>;
#[doc = "Pmn Output Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PODR11_A {
    #[doc = "0: Low output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<PODR11_A> for bool {
    #[inline(always)]
    fn from(variant: PODR11_A) -> Self {
        variant as u8 != 0
    }
}
impl PODR11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PODR11_A {
        match self.bits {
            false => PODR11_A::_0,
            true => PODR11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PODR11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PODR11_A::_1
    }
}
#[doc = "Field `PODR11` writer - Pmn Output Data"]
pub type PODR11_W<'a, const O: u8> = crate::BitWriter<'a, u16, PODR_SPEC, PODR11_A, O>;
impl<'a, const O: u8> PODR11_W<'a, O> {
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PODR11_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PODR11_A::_1)
    }
}
#[doc = "Field `PODR12` reader - Pmn Output Data"]
pub type PODR12_R = crate::BitReader<PODR12_A>;
#[doc = "Pmn Output Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PODR12_A {
    #[doc = "0: Low output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<PODR12_A> for bool {
    #[inline(always)]
    fn from(variant: PODR12_A) -> Self {
        variant as u8 != 0
    }
}
impl PODR12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PODR12_A {
        match self.bits {
            false => PODR12_A::_0,
            true => PODR12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PODR12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PODR12_A::_1
    }
}
#[doc = "Field `PODR12` writer - Pmn Output Data"]
pub type PODR12_W<'a, const O: u8> = crate::BitWriter<'a, u16, PODR_SPEC, PODR12_A, O>;
impl<'a, const O: u8> PODR12_W<'a, O> {
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PODR12_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PODR12_A::_1)
    }
}
#[doc = "Field `PODR13` reader - Pmn Output Data"]
pub type PODR13_R = crate::BitReader<PODR13_A>;
#[doc = "Pmn Output Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PODR13_A {
    #[doc = "0: Low output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<PODR13_A> for bool {
    #[inline(always)]
    fn from(variant: PODR13_A) -> Self {
        variant as u8 != 0
    }
}
impl PODR13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PODR13_A {
        match self.bits {
            false => PODR13_A::_0,
            true => PODR13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PODR13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PODR13_A::_1
    }
}
#[doc = "Field `PODR13` writer - Pmn Output Data"]
pub type PODR13_W<'a, const O: u8> = crate::BitWriter<'a, u16, PODR_SPEC, PODR13_A, O>;
impl<'a, const O: u8> PODR13_W<'a, O> {
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PODR13_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PODR13_A::_1)
    }
}
#[doc = "Field `PODR14` reader - Pmn Output Data"]
pub type PODR14_R = crate::BitReader<PODR14_A>;
#[doc = "Pmn Output Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PODR14_A {
    #[doc = "0: Low output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<PODR14_A> for bool {
    #[inline(always)]
    fn from(variant: PODR14_A) -> Self {
        variant as u8 != 0
    }
}
impl PODR14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PODR14_A {
        match self.bits {
            false => PODR14_A::_0,
            true => PODR14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PODR14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PODR14_A::_1
    }
}
#[doc = "Field `PODR14` writer - Pmn Output Data"]
pub type PODR14_W<'a, const O: u8> = crate::BitWriter<'a, u16, PODR_SPEC, PODR14_A, O>;
impl<'a, const O: u8> PODR14_W<'a, O> {
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PODR14_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PODR14_A::_1)
    }
}
#[doc = "Field `PODR15` reader - Pmn Output Data"]
pub type PODR15_R = crate::BitReader<PODR15_A>;
#[doc = "Pmn Output Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PODR15_A {
    #[doc = "0: Low output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<PODR15_A> for bool {
    #[inline(always)]
    fn from(variant: PODR15_A) -> Self {
        variant as u8 != 0
    }
}
impl PODR15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PODR15_A {
        match self.bits {
            false => PODR15_A::_0,
            true => PODR15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PODR15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PODR15_A::_1
    }
}
#[doc = "Field `PODR15` writer - Pmn Output Data"]
pub type PODR15_W<'a, const O: u8> = crate::BitWriter<'a, u16, PODR_SPEC, PODR15_A, O>;
impl<'a, const O: u8> PODR15_W<'a, O> {
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PODR15_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PODR15_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr00(&self) -> PODR00_R {
        PODR00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr01(&self) -> PODR01_R {
        PODR01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr02(&self) -> PODR02_R {
        PODR02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr03(&self) -> PODR03_R {
        PODR03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr04(&self) -> PODR04_R {
        PODR04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr05(&self) -> PODR05_R {
        PODR05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr06(&self) -> PODR06_R {
        PODR06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr07(&self) -> PODR07_R {
        PODR07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr08(&self) -> PODR08_R {
        PODR08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr09(&self) -> PODR09_R {
        PODR09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr10(&self) -> PODR10_R {
        PODR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr11(&self) -> PODR11_R {
        PODR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr12(&self) -> PODR12_R {
        PODR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr13(&self) -> PODR13_R {
        PODR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr14(&self) -> PODR14_R {
        PODR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr15(&self) -> PODR15_R {
        PODR15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr00(&mut self) -> PODR00_W<0> {
        PODR00_W::new(self)
    }
    #[doc = "Bit 1 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr01(&mut self) -> PODR01_W<1> {
        PODR01_W::new(self)
    }
    #[doc = "Bit 2 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr02(&mut self) -> PODR02_W<2> {
        PODR02_W::new(self)
    }
    #[doc = "Bit 3 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr03(&mut self) -> PODR03_W<3> {
        PODR03_W::new(self)
    }
    #[doc = "Bit 4 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr04(&mut self) -> PODR04_W<4> {
        PODR04_W::new(self)
    }
    #[doc = "Bit 5 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr05(&mut self) -> PODR05_W<5> {
        PODR05_W::new(self)
    }
    #[doc = "Bit 6 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr06(&mut self) -> PODR06_W<6> {
        PODR06_W::new(self)
    }
    #[doc = "Bit 7 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr07(&mut self) -> PODR07_W<7> {
        PODR07_W::new(self)
    }
    #[doc = "Bit 8 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr08(&mut self) -> PODR08_W<8> {
        PODR08_W::new(self)
    }
    #[doc = "Bit 9 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr09(&mut self) -> PODR09_W<9> {
        PODR09_W::new(self)
    }
    #[doc = "Bit 10 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr10(&mut self) -> PODR10_W<10> {
        PODR10_W::new(self)
    }
    #[doc = "Bit 11 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr11(&mut self) -> PODR11_W<11> {
        PODR11_W::new(self)
    }
    #[doc = "Bit 12 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr12(&mut self) -> PODR12_W<12> {
        PODR12_W::new(self)
    }
    #[doc = "Bit 13 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr13(&mut self) -> PODR13_W<13> {
        PODR13_W::new(self)
    }
    #[doc = "Bit 14 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr14(&mut self) -> PODR14_W<14> {
        PODR14_W::new(self)
    }
    #[doc = "Bit 15 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr15(&mut self) -> PODR15_W<15> {
        PODR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [podr](index.html) module"]
pub struct PODR_SPEC;
impl crate::RegisterSpec for PODR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [podr::R](R) reader structure"]
impl crate::Readable for PODR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [podr::W](W) writer structure"]
impl crate::Writable for PODR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PODR to value 0"]
impl crate::Resettable for PODR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
