#[doc = "Register `EORR` reader"]
pub struct R(crate::R<EORR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EORR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EORR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EORR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EORR` writer"]
pub struct W(crate::W<EORR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EORR_SPEC>;
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
impl From<crate::W<EORR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EORR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EORR00` reader - Pmn Event Output Reset"]
pub type EORR00_R = crate::BitReader<EORR00_A>;
#[doc = "Pmn Event Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EORR00_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<EORR00_A> for bool {
    #[inline(always)]
    fn from(variant: EORR00_A) -> Self {
        variant as u8 != 0
    }
}
impl EORR00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EORR00_A {
        match self.bits {
            false => EORR00_A::_0,
            true => EORR00_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EORR00_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EORR00_A::_1
    }
}
#[doc = "Field `EORR00` writer - Pmn Event Output Reset"]
pub type EORR00_W<'a, const O: u8> = crate::BitWriter<'a, u16, EORR_SPEC, EORR00_A, O>;
impl<'a, const O: u8> EORR00_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EORR00_A::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EORR00_A::_1)
    }
}
#[doc = "Field `EORR01` reader - Pmn Event Output Reset"]
pub type EORR01_R = crate::BitReader<EORR01_A>;
#[doc = "Pmn Event Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EORR01_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<EORR01_A> for bool {
    #[inline(always)]
    fn from(variant: EORR01_A) -> Self {
        variant as u8 != 0
    }
}
impl EORR01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EORR01_A {
        match self.bits {
            false => EORR01_A::_0,
            true => EORR01_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EORR01_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EORR01_A::_1
    }
}
#[doc = "Field `EORR01` writer - Pmn Event Output Reset"]
pub type EORR01_W<'a, const O: u8> = crate::BitWriter<'a, u16, EORR_SPEC, EORR01_A, O>;
impl<'a, const O: u8> EORR01_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EORR01_A::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EORR01_A::_1)
    }
}
#[doc = "Field `EORR02` reader - Pmn Event Output Reset"]
pub type EORR02_R = crate::BitReader<EORR02_A>;
#[doc = "Pmn Event Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EORR02_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<EORR02_A> for bool {
    #[inline(always)]
    fn from(variant: EORR02_A) -> Self {
        variant as u8 != 0
    }
}
impl EORR02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EORR02_A {
        match self.bits {
            false => EORR02_A::_0,
            true => EORR02_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EORR02_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EORR02_A::_1
    }
}
#[doc = "Field `EORR02` writer - Pmn Event Output Reset"]
pub type EORR02_W<'a, const O: u8> = crate::BitWriter<'a, u16, EORR_SPEC, EORR02_A, O>;
impl<'a, const O: u8> EORR02_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EORR02_A::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EORR02_A::_1)
    }
}
#[doc = "Field `EORR03` reader - Pmn Event Output Reset"]
pub type EORR03_R = crate::BitReader<EORR03_A>;
#[doc = "Pmn Event Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EORR03_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<EORR03_A> for bool {
    #[inline(always)]
    fn from(variant: EORR03_A) -> Self {
        variant as u8 != 0
    }
}
impl EORR03_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EORR03_A {
        match self.bits {
            false => EORR03_A::_0,
            true => EORR03_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EORR03_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EORR03_A::_1
    }
}
#[doc = "Field `EORR03` writer - Pmn Event Output Reset"]
pub type EORR03_W<'a, const O: u8> = crate::BitWriter<'a, u16, EORR_SPEC, EORR03_A, O>;
impl<'a, const O: u8> EORR03_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EORR03_A::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EORR03_A::_1)
    }
}
#[doc = "Field `EORR04` reader - Pmn Event Output Reset"]
pub type EORR04_R = crate::BitReader<EORR04_A>;
#[doc = "Pmn Event Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EORR04_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<EORR04_A> for bool {
    #[inline(always)]
    fn from(variant: EORR04_A) -> Self {
        variant as u8 != 0
    }
}
impl EORR04_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EORR04_A {
        match self.bits {
            false => EORR04_A::_0,
            true => EORR04_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EORR04_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EORR04_A::_1
    }
}
#[doc = "Field `EORR04` writer - Pmn Event Output Reset"]
pub type EORR04_W<'a, const O: u8> = crate::BitWriter<'a, u16, EORR_SPEC, EORR04_A, O>;
impl<'a, const O: u8> EORR04_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EORR04_A::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EORR04_A::_1)
    }
}
#[doc = "Field `EORR05` reader - Pmn Event Output Reset"]
pub type EORR05_R = crate::BitReader<EORR05_A>;
#[doc = "Pmn Event Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EORR05_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<EORR05_A> for bool {
    #[inline(always)]
    fn from(variant: EORR05_A) -> Self {
        variant as u8 != 0
    }
}
impl EORR05_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EORR05_A {
        match self.bits {
            false => EORR05_A::_0,
            true => EORR05_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EORR05_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EORR05_A::_1
    }
}
#[doc = "Field `EORR05` writer - Pmn Event Output Reset"]
pub type EORR05_W<'a, const O: u8> = crate::BitWriter<'a, u16, EORR_SPEC, EORR05_A, O>;
impl<'a, const O: u8> EORR05_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EORR05_A::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EORR05_A::_1)
    }
}
#[doc = "Field `EORR06` reader - Pmn Event Output Reset"]
pub type EORR06_R = crate::BitReader<EORR06_A>;
#[doc = "Pmn Event Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EORR06_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<EORR06_A> for bool {
    #[inline(always)]
    fn from(variant: EORR06_A) -> Self {
        variant as u8 != 0
    }
}
impl EORR06_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EORR06_A {
        match self.bits {
            false => EORR06_A::_0,
            true => EORR06_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EORR06_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EORR06_A::_1
    }
}
#[doc = "Field `EORR06` writer - Pmn Event Output Reset"]
pub type EORR06_W<'a, const O: u8> = crate::BitWriter<'a, u16, EORR_SPEC, EORR06_A, O>;
impl<'a, const O: u8> EORR06_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EORR06_A::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EORR06_A::_1)
    }
}
#[doc = "Field `EORR07` reader - Pmn Event Output Reset"]
pub type EORR07_R = crate::BitReader<EORR07_A>;
#[doc = "Pmn Event Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EORR07_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<EORR07_A> for bool {
    #[inline(always)]
    fn from(variant: EORR07_A) -> Self {
        variant as u8 != 0
    }
}
impl EORR07_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EORR07_A {
        match self.bits {
            false => EORR07_A::_0,
            true => EORR07_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EORR07_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EORR07_A::_1
    }
}
#[doc = "Field `EORR07` writer - Pmn Event Output Reset"]
pub type EORR07_W<'a, const O: u8> = crate::BitWriter<'a, u16, EORR_SPEC, EORR07_A, O>;
impl<'a, const O: u8> EORR07_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EORR07_A::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EORR07_A::_1)
    }
}
#[doc = "Field `EORR08` reader - Pmn Event Output Reset"]
pub type EORR08_R = crate::BitReader<EORR08_A>;
#[doc = "Pmn Event Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EORR08_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<EORR08_A> for bool {
    #[inline(always)]
    fn from(variant: EORR08_A) -> Self {
        variant as u8 != 0
    }
}
impl EORR08_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EORR08_A {
        match self.bits {
            false => EORR08_A::_0,
            true => EORR08_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EORR08_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EORR08_A::_1
    }
}
#[doc = "Field `EORR08` writer - Pmn Event Output Reset"]
pub type EORR08_W<'a, const O: u8> = crate::BitWriter<'a, u16, EORR_SPEC, EORR08_A, O>;
impl<'a, const O: u8> EORR08_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EORR08_A::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EORR08_A::_1)
    }
}
#[doc = "Field `EORR09` reader - Pmn Event Output Reset"]
pub type EORR09_R = crate::BitReader<EORR09_A>;
#[doc = "Pmn Event Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EORR09_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<EORR09_A> for bool {
    #[inline(always)]
    fn from(variant: EORR09_A) -> Self {
        variant as u8 != 0
    }
}
impl EORR09_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EORR09_A {
        match self.bits {
            false => EORR09_A::_0,
            true => EORR09_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EORR09_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EORR09_A::_1
    }
}
#[doc = "Field `EORR09` writer - Pmn Event Output Reset"]
pub type EORR09_W<'a, const O: u8> = crate::BitWriter<'a, u16, EORR_SPEC, EORR09_A, O>;
impl<'a, const O: u8> EORR09_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EORR09_A::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EORR09_A::_1)
    }
}
#[doc = "Field `EORR10` reader - Pmn Event Output Reset"]
pub type EORR10_R = crate::BitReader<EORR10_A>;
#[doc = "Pmn Event Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EORR10_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<EORR10_A> for bool {
    #[inline(always)]
    fn from(variant: EORR10_A) -> Self {
        variant as u8 != 0
    }
}
impl EORR10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EORR10_A {
        match self.bits {
            false => EORR10_A::_0,
            true => EORR10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EORR10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EORR10_A::_1
    }
}
#[doc = "Field `EORR10` writer - Pmn Event Output Reset"]
pub type EORR10_W<'a, const O: u8> = crate::BitWriter<'a, u16, EORR_SPEC, EORR10_A, O>;
impl<'a, const O: u8> EORR10_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EORR10_A::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EORR10_A::_1)
    }
}
#[doc = "Field `EORR11` reader - Pmn Event Output Reset"]
pub type EORR11_R = crate::BitReader<EORR11_A>;
#[doc = "Pmn Event Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EORR11_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<EORR11_A> for bool {
    #[inline(always)]
    fn from(variant: EORR11_A) -> Self {
        variant as u8 != 0
    }
}
impl EORR11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EORR11_A {
        match self.bits {
            false => EORR11_A::_0,
            true => EORR11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EORR11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EORR11_A::_1
    }
}
#[doc = "Field `EORR11` writer - Pmn Event Output Reset"]
pub type EORR11_W<'a, const O: u8> = crate::BitWriter<'a, u16, EORR_SPEC, EORR11_A, O>;
impl<'a, const O: u8> EORR11_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EORR11_A::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EORR11_A::_1)
    }
}
#[doc = "Field `EORR12` reader - Pmn Event Output Reset"]
pub type EORR12_R = crate::BitReader<EORR12_A>;
#[doc = "Pmn Event Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EORR12_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<EORR12_A> for bool {
    #[inline(always)]
    fn from(variant: EORR12_A) -> Self {
        variant as u8 != 0
    }
}
impl EORR12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EORR12_A {
        match self.bits {
            false => EORR12_A::_0,
            true => EORR12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EORR12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EORR12_A::_1
    }
}
#[doc = "Field `EORR12` writer - Pmn Event Output Reset"]
pub type EORR12_W<'a, const O: u8> = crate::BitWriter<'a, u16, EORR_SPEC, EORR12_A, O>;
impl<'a, const O: u8> EORR12_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EORR12_A::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EORR12_A::_1)
    }
}
#[doc = "Field `EORR13` reader - Pmn Event Output Reset"]
pub type EORR13_R = crate::BitReader<EORR13_A>;
#[doc = "Pmn Event Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EORR13_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<EORR13_A> for bool {
    #[inline(always)]
    fn from(variant: EORR13_A) -> Self {
        variant as u8 != 0
    }
}
impl EORR13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EORR13_A {
        match self.bits {
            false => EORR13_A::_0,
            true => EORR13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EORR13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EORR13_A::_1
    }
}
#[doc = "Field `EORR13` writer - Pmn Event Output Reset"]
pub type EORR13_W<'a, const O: u8> = crate::BitWriter<'a, u16, EORR_SPEC, EORR13_A, O>;
impl<'a, const O: u8> EORR13_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EORR13_A::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EORR13_A::_1)
    }
}
#[doc = "Field `EORR14` reader - Pmn Event Output Reset"]
pub type EORR14_R = crate::BitReader<EORR14_A>;
#[doc = "Pmn Event Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EORR14_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<EORR14_A> for bool {
    #[inline(always)]
    fn from(variant: EORR14_A) -> Self {
        variant as u8 != 0
    }
}
impl EORR14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EORR14_A {
        match self.bits {
            false => EORR14_A::_0,
            true => EORR14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EORR14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EORR14_A::_1
    }
}
#[doc = "Field `EORR14` writer - Pmn Event Output Reset"]
pub type EORR14_W<'a, const O: u8> = crate::BitWriter<'a, u16, EORR_SPEC, EORR14_A, O>;
impl<'a, const O: u8> EORR14_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EORR14_A::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EORR14_A::_1)
    }
}
#[doc = "Field `EORR15` reader - Pmn Event Output Reset"]
pub type EORR15_R = crate::BitReader<EORR15_A>;
#[doc = "Pmn Event Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EORR15_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<EORR15_A> for bool {
    #[inline(always)]
    fn from(variant: EORR15_A) -> Self {
        variant as u8 != 0
    }
}
impl EORR15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EORR15_A {
        match self.bits {
            false => EORR15_A::_0,
            true => EORR15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EORR15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EORR15_A::_1
    }
}
#[doc = "Field `EORR15` writer - Pmn Event Output Reset"]
pub type EORR15_W<'a, const O: u8> = crate::BitWriter<'a, u16, EORR_SPEC, EORR15_A, O>;
impl<'a, const O: u8> EORR15_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EORR15_A::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EORR15_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr00(&self) -> EORR00_R {
        EORR00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr01(&self) -> EORR01_R {
        EORR01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr02(&self) -> EORR02_R {
        EORR02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr03(&self) -> EORR03_R {
        EORR03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr04(&self) -> EORR04_R {
        EORR04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr05(&self) -> EORR05_R {
        EORR05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr06(&self) -> EORR06_R {
        EORR06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr07(&self) -> EORR07_R {
        EORR07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr08(&self) -> EORR08_R {
        EORR08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr09(&self) -> EORR09_R {
        EORR09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr10(&self) -> EORR10_R {
        EORR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr11(&self) -> EORR11_R {
        EORR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr12(&self) -> EORR12_R {
        EORR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr13(&self) -> EORR13_R {
        EORR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr14(&self) -> EORR14_R {
        EORR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr15(&self) -> EORR15_R {
        EORR15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pmn Event Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eorr00(&mut self) -> EORR00_W<0> {
        EORR00_W::new(self)
    }
    #[doc = "Bit 1 - Pmn Event Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eorr01(&mut self) -> EORR01_W<1> {
        EORR01_W::new(self)
    }
    #[doc = "Bit 2 - Pmn Event Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eorr02(&mut self) -> EORR02_W<2> {
        EORR02_W::new(self)
    }
    #[doc = "Bit 3 - Pmn Event Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eorr03(&mut self) -> EORR03_W<3> {
        EORR03_W::new(self)
    }
    #[doc = "Bit 4 - Pmn Event Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eorr04(&mut self) -> EORR04_W<4> {
        EORR04_W::new(self)
    }
    #[doc = "Bit 5 - Pmn Event Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eorr05(&mut self) -> EORR05_W<5> {
        EORR05_W::new(self)
    }
    #[doc = "Bit 6 - Pmn Event Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eorr06(&mut self) -> EORR06_W<6> {
        EORR06_W::new(self)
    }
    #[doc = "Bit 7 - Pmn Event Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eorr07(&mut self) -> EORR07_W<7> {
        EORR07_W::new(self)
    }
    #[doc = "Bit 8 - Pmn Event Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eorr08(&mut self) -> EORR08_W<8> {
        EORR08_W::new(self)
    }
    #[doc = "Bit 9 - Pmn Event Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eorr09(&mut self) -> EORR09_W<9> {
        EORR09_W::new(self)
    }
    #[doc = "Bit 10 - Pmn Event Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eorr10(&mut self) -> EORR10_W<10> {
        EORR10_W::new(self)
    }
    #[doc = "Bit 11 - Pmn Event Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eorr11(&mut self) -> EORR11_W<11> {
        EORR11_W::new(self)
    }
    #[doc = "Bit 12 - Pmn Event Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eorr12(&mut self) -> EORR12_W<12> {
        EORR12_W::new(self)
    }
    #[doc = "Bit 13 - Pmn Event Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eorr13(&mut self) -> EORR13_W<13> {
        EORR13_W::new(self)
    }
    #[doc = "Bit 14 - Pmn Event Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eorr14(&mut self) -> EORR14_W<14> {
        EORR14_W::new(self)
    }
    #[doc = "Bit 15 - Pmn Event Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eorr15(&mut self) -> EORR15_W<15> {
        EORR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eorr](index.html) module"]
pub struct EORR_SPEC;
impl crate::RegisterSpec for EORR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [eorr::R](R) reader structure"]
impl crate::Readable for EORR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eorr::W](W) writer structure"]
impl crate::Writable for EORR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EORR to value 0"]
impl crate::Resettable for EORR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
