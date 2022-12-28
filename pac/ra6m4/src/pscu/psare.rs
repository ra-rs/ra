#[doc = "Register `PSARE` reader"]
pub struct R(crate::R<PSARE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSARE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSARE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSARE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSARE` writer"]
pub struct W(crate::W<PSARE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSARE_SPEC>;
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
impl From<crate::W<PSARE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSARE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSARE0` reader - WDT security attribution"]
pub type PSARE0_R = crate::BitReader<PSARE0_A>;
#[doc = "WDT security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARE0_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARE0_A> for bool {
    #[inline(always)]
    fn from(variant: PSARE0_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARE0_A {
        match self.bits {
            false => PSARE0_A::_0,
            true => PSARE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARE0_A::_1
    }
}
#[doc = "Field `PSARE0` writer - WDT security attribution"]
pub type PSARE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARE_SPEC, PSARE0_A, O>;
impl<'a, const O: u8> PSARE0_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARE0_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARE0_A::_1)
    }
}
#[doc = "Field `PSARE1` reader - IWDT security attribution"]
pub type PSARE1_R = crate::BitReader<PSARE1_A>;
#[doc = "IWDT security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARE1_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARE1_A> for bool {
    #[inline(always)]
    fn from(variant: PSARE1_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARE1_A {
        match self.bits {
            false => PSARE1_A::_0,
            true => PSARE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARE1_A::_1
    }
}
#[doc = "Field `PSARE1` writer - IWDT security attribution"]
pub type PSARE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARE_SPEC, PSARE1_A, O>;
impl<'a, const O: u8> PSARE1_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARE1_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARE1_A::_1)
    }
}
#[doc = "Field `PSARE2` reader - RTC security attribution"]
pub type PSARE2_R = crate::BitReader<PSARE2_A>;
#[doc = "RTC security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARE2_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARE2_A> for bool {
    #[inline(always)]
    fn from(variant: PSARE2_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARE2_A {
        match self.bits {
            false => PSARE2_A::_0,
            true => PSARE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARE2_A::_1
    }
}
#[doc = "Field `PSARE2` writer - RTC security attribution"]
pub type PSARE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARE_SPEC, PSARE2_A, O>;
impl<'a, const O: u8> PSARE2_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARE2_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARE2_A::_1)
    }
}
#[doc = "Field `PSARE14` reader - AGT5 and the MSTPCRE.MSTPE14 bit security attribution"]
pub type PSARE14_R = crate::BitReader<PSARE14_A>;
#[doc = "AGT5 and the MSTPCRE.MSTPE14 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARE14_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARE14_A> for bool {
    #[inline(always)]
    fn from(variant: PSARE14_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARE14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARE14_A {
        match self.bits {
            false => PSARE14_A::_0,
            true => PSARE14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARE14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARE14_A::_1
    }
}
#[doc = "Field `PSARE14` writer - AGT5 and the MSTPCRE.MSTPE14 bit security attribution"]
pub type PSARE14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARE_SPEC, PSARE14_A, O>;
impl<'a, const O: u8> PSARE14_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARE14_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARE14_A::_1)
    }
}
#[doc = "Field `PSARE15` reader - AGT4 and the MSTPCRE.MSTPE15 bit security attribution"]
pub type PSARE15_R = crate::BitReader<PSARE15_A>;
#[doc = "AGT4 and the MSTPCRE.MSTPE15 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARE15_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARE15_A> for bool {
    #[inline(always)]
    fn from(variant: PSARE15_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARE15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARE15_A {
        match self.bits {
            false => PSARE15_A::_0,
            true => PSARE15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARE15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARE15_A::_1
    }
}
#[doc = "Field `PSARE15` writer - AGT4 and the MSTPCRE.MSTPE15 bit security attribution"]
pub type PSARE15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARE_SPEC, PSARE15_A, O>;
impl<'a, const O: u8> PSARE15_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARE15_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARE15_A::_1)
    }
}
#[doc = "Field `PSARE22` reader - GPT9 and the MSTPCRE.MSTPE22 bit security attribution"]
pub type PSARE22_R = crate::BitReader<PSARE22_A>;
#[doc = "GPT9 and the MSTPCRE.MSTPE22 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARE22_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARE22_A> for bool {
    #[inline(always)]
    fn from(variant: PSARE22_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARE22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARE22_A {
        match self.bits {
            false => PSARE22_A::_0,
            true => PSARE22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARE22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARE22_A::_1
    }
}
#[doc = "Field `PSARE22` writer - GPT9 and the MSTPCRE.MSTPE22 bit security attribution"]
pub type PSARE22_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARE_SPEC, PSARE22_A, O>;
impl<'a, const O: u8> PSARE22_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARE22_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARE22_A::_1)
    }
}
#[doc = "Field `PSARE23` reader - GPT8 and the MSTPCRE.MSTPE23 bit security attribution"]
pub type PSARE23_R = crate::BitReader<PSARE23_A>;
#[doc = "GPT8 and the MSTPCRE.MSTPE23 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARE23_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARE23_A> for bool {
    #[inline(always)]
    fn from(variant: PSARE23_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARE23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARE23_A {
        match self.bits {
            false => PSARE23_A::_0,
            true => PSARE23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARE23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARE23_A::_1
    }
}
#[doc = "Field `PSARE23` writer - GPT8 and the MSTPCRE.MSTPE23 bit security attribution"]
pub type PSARE23_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARE_SPEC, PSARE23_A, O>;
impl<'a, const O: u8> PSARE23_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARE23_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARE23_A::_1)
    }
}
#[doc = "Field `PSARE24` reader - GPT7 and the MSTPCRE.MSTPE24 bit security attribution"]
pub type PSARE24_R = crate::BitReader<PSARE24_A>;
#[doc = "GPT7 and the MSTPCRE.MSTPE24 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARE24_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARE24_A> for bool {
    #[inline(always)]
    fn from(variant: PSARE24_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARE24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARE24_A {
        match self.bits {
            false => PSARE24_A::_0,
            true => PSARE24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARE24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARE24_A::_1
    }
}
#[doc = "Field `PSARE24` writer - GPT7 and the MSTPCRE.MSTPE24 bit security attribution"]
pub type PSARE24_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARE_SPEC, PSARE24_A, O>;
impl<'a, const O: u8> PSARE24_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARE24_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARE24_A::_1)
    }
}
#[doc = "Field `PSARE25` reader - GPT6 and the MSTPCRE.MSTPE25 bit security attribution"]
pub type PSARE25_R = crate::BitReader<PSARE25_A>;
#[doc = "GPT6 and the MSTPCRE.MSTPE25 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARE25_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARE25_A> for bool {
    #[inline(always)]
    fn from(variant: PSARE25_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARE25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARE25_A {
        match self.bits {
            false => PSARE25_A::_0,
            true => PSARE25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARE25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARE25_A::_1
    }
}
#[doc = "Field `PSARE25` writer - GPT6 and the MSTPCRE.MSTPE25 bit security attribution"]
pub type PSARE25_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARE_SPEC, PSARE25_A, O>;
impl<'a, const O: u8> PSARE25_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARE25_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARE25_A::_1)
    }
}
#[doc = "Field `PSARE26` reader - GPT5 and the MSTPCRE.MSTPE26 bit security attribution"]
pub type PSARE26_R = crate::BitReader<PSARE26_A>;
#[doc = "GPT5 and the MSTPCRE.MSTPE26 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARE26_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARE26_A> for bool {
    #[inline(always)]
    fn from(variant: PSARE26_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARE26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARE26_A {
        match self.bits {
            false => PSARE26_A::_0,
            true => PSARE26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARE26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARE26_A::_1
    }
}
#[doc = "Field `PSARE26` writer - GPT5 and the MSTPCRE.MSTPE26 bit security attribution"]
pub type PSARE26_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARE_SPEC, PSARE26_A, O>;
impl<'a, const O: u8> PSARE26_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARE26_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARE26_A::_1)
    }
}
#[doc = "Field `PSARE27` reader - GPT4 and the MSTPCRE.MSTPE27 bit security attribution"]
pub type PSARE27_R = crate::BitReader<PSARE27_A>;
#[doc = "GPT4 and the MSTPCRE.MSTPE27 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARE27_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARE27_A> for bool {
    #[inline(always)]
    fn from(variant: PSARE27_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARE27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARE27_A {
        match self.bits {
            false => PSARE27_A::_0,
            true => PSARE27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARE27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARE27_A::_1
    }
}
#[doc = "Field `PSARE27` writer - GPT4 and the MSTPCRE.MSTPE27 bit security attribution"]
pub type PSARE27_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARE_SPEC, PSARE27_A, O>;
impl<'a, const O: u8> PSARE27_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARE27_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARE27_A::_1)
    }
}
#[doc = "Field `PSARE28` reader - GPT3 and the MSTPCRE.MSTPE28 bit security attribution"]
pub type PSARE28_R = crate::BitReader<PSARE28_A>;
#[doc = "GPT3 and the MSTPCRE.MSTPE28 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARE28_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARE28_A> for bool {
    #[inline(always)]
    fn from(variant: PSARE28_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARE28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARE28_A {
        match self.bits {
            false => PSARE28_A::_0,
            true => PSARE28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARE28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARE28_A::_1
    }
}
#[doc = "Field `PSARE28` writer - GPT3 and the MSTPCRE.MSTPE28 bit security attribution"]
pub type PSARE28_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARE_SPEC, PSARE28_A, O>;
impl<'a, const O: u8> PSARE28_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARE28_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARE28_A::_1)
    }
}
#[doc = "Field `PSARE29` reader - GPT2 and the MSTPCRE.MSTPE29 bit security attribution"]
pub type PSARE29_R = crate::BitReader<PSARE29_A>;
#[doc = "GPT2 and the MSTPCRE.MSTPE29 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARE29_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARE29_A> for bool {
    #[inline(always)]
    fn from(variant: PSARE29_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARE29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARE29_A {
        match self.bits {
            false => PSARE29_A::_0,
            true => PSARE29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARE29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARE29_A::_1
    }
}
#[doc = "Field `PSARE29` writer - GPT2 and the MSTPCRE.MSTPE29 bit security attribution"]
pub type PSARE29_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARE_SPEC, PSARE29_A, O>;
impl<'a, const O: u8> PSARE29_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARE29_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARE29_A::_1)
    }
}
#[doc = "Field `PSARE30` reader - GPT1 and the MSTPCRE.MSTPE30 bit security attribution"]
pub type PSARE30_R = crate::BitReader<PSARE30_A>;
#[doc = "GPT1 and the MSTPCRE.MSTPE30 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARE30_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARE30_A> for bool {
    #[inline(always)]
    fn from(variant: PSARE30_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARE30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARE30_A {
        match self.bits {
            false => PSARE30_A::_0,
            true => PSARE30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARE30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARE30_A::_1
    }
}
#[doc = "Field `PSARE30` writer - GPT1 and the MSTPCRE.MSTPE30 bit security attribution"]
pub type PSARE30_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARE_SPEC, PSARE30_A, O>;
impl<'a, const O: u8> PSARE30_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARE30_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARE30_A::_1)
    }
}
#[doc = "Field `PSARE31` reader - GPT0, GPT_OPS and the MSTPCRE.MSTPE31 bit security attribution"]
pub type PSARE31_R = crate::BitReader<PSARE31_A>;
#[doc = "GPT0, GPT_OPS and the MSTPCRE.MSTPE31 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARE31_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARE31_A> for bool {
    #[inline(always)]
    fn from(variant: PSARE31_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARE31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARE31_A {
        match self.bits {
            false => PSARE31_A::_0,
            true => PSARE31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARE31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARE31_A::_1
    }
}
#[doc = "Field `PSARE31` writer - GPT0, GPT_OPS and the MSTPCRE.MSTPE31 bit security attribution"]
pub type PSARE31_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARE_SPEC, PSARE31_A, O>;
impl<'a, const O: u8> PSARE31_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARE31_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARE31_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - WDT security attribution"]
    #[inline(always)]
    pub fn psare0(&self) -> PSARE0_R {
        PSARE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IWDT security attribution"]
    #[inline(always)]
    pub fn psare1(&self) -> PSARE1_R {
        PSARE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC security attribution"]
    #[inline(always)]
    pub fn psare2(&self) -> PSARE2_R {
        PSARE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 14 - AGT5 and the MSTPCRE.MSTPE14 bit security attribution"]
    #[inline(always)]
    pub fn psare14(&self) -> PSARE14_R {
        PSARE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AGT4 and the MSTPCRE.MSTPE15 bit security attribution"]
    #[inline(always)]
    pub fn psare15(&self) -> PSARE15_R {
        PSARE15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 22 - GPT9 and the MSTPCRE.MSTPE22 bit security attribution"]
    #[inline(always)]
    pub fn psare22(&self) -> PSARE22_R {
        PSARE22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPT8 and the MSTPCRE.MSTPE23 bit security attribution"]
    #[inline(always)]
    pub fn psare23(&self) -> PSARE23_R {
        PSARE23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GPT7 and the MSTPCRE.MSTPE24 bit security attribution"]
    #[inline(always)]
    pub fn psare24(&self) -> PSARE24_R {
        PSARE24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GPT6 and the MSTPCRE.MSTPE25 bit security attribution"]
    #[inline(always)]
    pub fn psare25(&self) -> PSARE25_R {
        PSARE25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - GPT5 and the MSTPCRE.MSTPE26 bit security attribution"]
    #[inline(always)]
    pub fn psare26(&self) -> PSARE26_R {
        PSARE26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - GPT4 and the MSTPCRE.MSTPE27 bit security attribution"]
    #[inline(always)]
    pub fn psare27(&self) -> PSARE27_R {
        PSARE27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - GPT3 and the MSTPCRE.MSTPE28 bit security attribution"]
    #[inline(always)]
    pub fn psare28(&self) -> PSARE28_R {
        PSARE28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - GPT2 and the MSTPCRE.MSTPE29 bit security attribution"]
    #[inline(always)]
    pub fn psare29(&self) -> PSARE29_R {
        PSARE29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - GPT1 and the MSTPCRE.MSTPE30 bit security attribution"]
    #[inline(always)]
    pub fn psare30(&self) -> PSARE30_R {
        PSARE30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPT0, GPT_OPS and the MSTPCRE.MSTPE31 bit security attribution"]
    #[inline(always)]
    pub fn psare31(&self) -> PSARE31_R {
        PSARE31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDT security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psare0(&mut self) -> PSARE0_W<0> {
        PSARE0_W::new(self)
    }
    #[doc = "Bit 1 - IWDT security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psare1(&mut self) -> PSARE1_W<1> {
        PSARE1_W::new(self)
    }
    #[doc = "Bit 2 - RTC security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psare2(&mut self) -> PSARE2_W<2> {
        PSARE2_W::new(self)
    }
    #[doc = "Bit 14 - AGT5 and the MSTPCRE.MSTPE14 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psare14(&mut self) -> PSARE14_W<14> {
        PSARE14_W::new(self)
    }
    #[doc = "Bit 15 - AGT4 and the MSTPCRE.MSTPE15 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psare15(&mut self) -> PSARE15_W<15> {
        PSARE15_W::new(self)
    }
    #[doc = "Bit 22 - GPT9 and the MSTPCRE.MSTPE22 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psare22(&mut self) -> PSARE22_W<22> {
        PSARE22_W::new(self)
    }
    #[doc = "Bit 23 - GPT8 and the MSTPCRE.MSTPE23 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psare23(&mut self) -> PSARE23_W<23> {
        PSARE23_W::new(self)
    }
    #[doc = "Bit 24 - GPT7 and the MSTPCRE.MSTPE24 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psare24(&mut self) -> PSARE24_W<24> {
        PSARE24_W::new(self)
    }
    #[doc = "Bit 25 - GPT6 and the MSTPCRE.MSTPE25 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psare25(&mut self) -> PSARE25_W<25> {
        PSARE25_W::new(self)
    }
    #[doc = "Bit 26 - GPT5 and the MSTPCRE.MSTPE26 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psare26(&mut self) -> PSARE26_W<26> {
        PSARE26_W::new(self)
    }
    #[doc = "Bit 27 - GPT4 and the MSTPCRE.MSTPE27 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psare27(&mut self) -> PSARE27_W<27> {
        PSARE27_W::new(self)
    }
    #[doc = "Bit 28 - GPT3 and the MSTPCRE.MSTPE28 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psare28(&mut self) -> PSARE28_W<28> {
        PSARE28_W::new(self)
    }
    #[doc = "Bit 29 - GPT2 and the MSTPCRE.MSTPE29 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psare29(&mut self) -> PSARE29_W<29> {
        PSARE29_W::new(self)
    }
    #[doc = "Bit 30 - GPT1 and the MSTPCRE.MSTPE30 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psare30(&mut self) -> PSARE30_W<30> {
        PSARE30_W::new(self)
    }
    #[doc = "Bit 31 - GPT0, GPT_OPS and the MSTPCRE.MSTPE31 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psare31(&mut self) -> PSARE31_W<31> {
        PSARE31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Security Attribution Register E\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psare](index.html) module"]
pub struct PSARE_SPEC;
impl crate::RegisterSpec for PSARE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psare::R](R) reader structure"]
impl crate::Readable for PSARE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psare::W](W) writer structure"]
impl crate::Writable for PSARE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSARE to value 0xffff_ffff"]
impl crate::Resettable for PSARE_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
