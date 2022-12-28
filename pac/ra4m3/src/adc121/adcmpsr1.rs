#[doc = "Register `ADCMPSR1` reader"]
pub struct R(crate::R<ADCMPSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPSR1` writer"]
pub struct W(crate::W<ADCMPSR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPSR1_SPEC>;
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
impl From<crate::W<ADCMPSR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPSR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPSTCHA16` reader - Compare Window A Flag"]
pub type CMPSTCHA16_R = crate::BitReader<CMPSTCHA16_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA16_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA16_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA16_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA16_A {
        match self.bits {
            false => CMPSTCHA16_A::_0,
            true => CMPSTCHA16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA16_A::_1
    }
}
#[doc = "Field `CMPSTCHA16` writer - Compare Window A Flag"]
pub type CMPSTCHA16_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR1_SPEC, CMPSTCHA16_A, O>;
impl<'a, const O: u8> CMPSTCHA16_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA16_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA16_A::_1)
    }
}
#[doc = "Field `CMPSTCHA17` reader - Compare Window A Flag"]
pub type CMPSTCHA17_R = crate::BitReader<CMPSTCHA17_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA17_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA17_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA17_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA17_A {
        match self.bits {
            false => CMPSTCHA17_A::_0,
            true => CMPSTCHA17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA17_A::_1
    }
}
#[doc = "Field `CMPSTCHA17` writer - Compare Window A Flag"]
pub type CMPSTCHA17_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR1_SPEC, CMPSTCHA17_A, O>;
impl<'a, const O: u8> CMPSTCHA17_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA17_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA17_A::_1)
    }
}
#[doc = "Field `CMPSTCHA18` reader - Compare Window A Flag"]
pub type CMPSTCHA18_R = crate::BitReader<CMPSTCHA18_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA18_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA18_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA18_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA18_A {
        match self.bits {
            false => CMPSTCHA18_A::_0,
            true => CMPSTCHA18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA18_A::_1
    }
}
#[doc = "Field `CMPSTCHA18` writer - Compare Window A Flag"]
pub type CMPSTCHA18_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR1_SPEC, CMPSTCHA18_A, O>;
impl<'a, const O: u8> CMPSTCHA18_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA18_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA18_A::_1)
    }
}
#[doc = "Field `CMPSTCHA19` reader - Compare Window A Flag"]
pub type CMPSTCHA19_R = crate::BitReader<CMPSTCHA19_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA19_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA19_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA19_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA19_A {
        match self.bits {
            false => CMPSTCHA19_A::_0,
            true => CMPSTCHA19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA19_A::_1
    }
}
#[doc = "Field `CMPSTCHA19` writer - Compare Window A Flag"]
pub type CMPSTCHA19_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR1_SPEC, CMPSTCHA19_A, O>;
impl<'a, const O: u8> CMPSTCHA19_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA19_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA19_A::_1)
    }
}
#[doc = "Field `CMPSTCHA20` reader - Compare Window A Flag"]
pub type CMPSTCHA20_R = crate::BitReader<CMPSTCHA20_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA20_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA20_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA20_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA20_A {
        match self.bits {
            false => CMPSTCHA20_A::_0,
            true => CMPSTCHA20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA20_A::_1
    }
}
#[doc = "Field `CMPSTCHA20` writer - Compare Window A Flag"]
pub type CMPSTCHA20_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR1_SPEC, CMPSTCHA20_A, O>;
impl<'a, const O: u8> CMPSTCHA20_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA20_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA20_A::_1)
    }
}
#[doc = "Field `CMPSTCHA21` reader - Compare Window A Flag"]
pub type CMPSTCHA21_R = crate::BitReader<CMPSTCHA21_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA21_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA21_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA21_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA21_A {
        match self.bits {
            false => CMPSTCHA21_A::_0,
            true => CMPSTCHA21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA21_A::_1
    }
}
#[doc = "Field `CMPSTCHA21` writer - Compare Window A Flag"]
pub type CMPSTCHA21_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR1_SPEC, CMPSTCHA21_A, O>;
impl<'a, const O: u8> CMPSTCHA21_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA21_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA21_A::_1)
    }
}
#[doc = "Field `CMPSTCHA22` reader - Compare Window A Flag"]
pub type CMPSTCHA22_R = crate::BitReader<CMPSTCHA22_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA22_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA22_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA22_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA22_A {
        match self.bits {
            false => CMPSTCHA22_A::_0,
            true => CMPSTCHA22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA22_A::_1
    }
}
#[doc = "Field `CMPSTCHA22` writer - Compare Window A Flag"]
pub type CMPSTCHA22_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR1_SPEC, CMPSTCHA22_A, O>;
impl<'a, const O: u8> CMPSTCHA22_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA22_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA22_A::_1)
    }
}
#[doc = "Field `CMPSTCHA23` reader - Compare Window A Flag"]
pub type CMPSTCHA23_R = crate::BitReader<CMPSTCHA23_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA23_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA23_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA23_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA23_A {
        match self.bits {
            false => CMPSTCHA23_A::_0,
            true => CMPSTCHA23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA23_A::_1
    }
}
#[doc = "Field `CMPSTCHA23` writer - Compare Window A Flag"]
pub type CMPSTCHA23_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR1_SPEC, CMPSTCHA23_A, O>;
impl<'a, const O: u8> CMPSTCHA23_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA23_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA23_A::_1)
    }
}
#[doc = "Field `CMPSTCHA24` reader - Compare Window A Flag"]
pub type CMPSTCHA24_R = crate::BitReader<CMPSTCHA24_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA24_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA24_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA24_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA24_A {
        match self.bits {
            false => CMPSTCHA24_A::_0,
            true => CMPSTCHA24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA24_A::_1
    }
}
#[doc = "Field `CMPSTCHA24` writer - Compare Window A Flag"]
pub type CMPSTCHA24_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR1_SPEC, CMPSTCHA24_A, O>;
impl<'a, const O: u8> CMPSTCHA24_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA24_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA24_A::_1)
    }
}
#[doc = "Field `CMPSTCHA25` reader - Compare Window A Flag"]
pub type CMPSTCHA25_R = crate::BitReader<CMPSTCHA25_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA25_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA25_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA25_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA25_A {
        match self.bits {
            false => CMPSTCHA25_A::_0,
            true => CMPSTCHA25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA25_A::_1
    }
}
#[doc = "Field `CMPSTCHA25` writer - Compare Window A Flag"]
pub type CMPSTCHA25_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR1_SPEC, CMPSTCHA25_A, O>;
impl<'a, const O: u8> CMPSTCHA25_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA25_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA25_A::_1)
    }
}
#[doc = "Field `CMPSTCHA26` reader - Compare Window A Flag"]
pub type CMPSTCHA26_R = crate::BitReader<CMPSTCHA26_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA26_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA26_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA26_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA26_A {
        match self.bits {
            false => CMPSTCHA26_A::_0,
            true => CMPSTCHA26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA26_A::_1
    }
}
#[doc = "Field `CMPSTCHA26` writer - Compare Window A Flag"]
pub type CMPSTCHA26_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR1_SPEC, CMPSTCHA26_A, O>;
impl<'a, const O: u8> CMPSTCHA26_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA26_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA26_A::_1)
    }
}
#[doc = "Field `CMPSTCHA27` reader - Compare Window A Flag"]
pub type CMPSTCHA27_R = crate::BitReader<CMPSTCHA27_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA27_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA27_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA27_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA27_A {
        match self.bits {
            false => CMPSTCHA27_A::_0,
            true => CMPSTCHA27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA27_A::_1
    }
}
#[doc = "Field `CMPSTCHA27` writer - Compare Window A Flag"]
pub type CMPSTCHA27_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR1_SPEC, CMPSTCHA27_A, O>;
impl<'a, const O: u8> CMPSTCHA27_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA27_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA27_A::_1)
    }
}
#[doc = "Field `CMPSTCHA28` reader - Compare Window A Flag"]
pub type CMPSTCHA28_R = crate::BitReader<CMPSTCHA28_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA28_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA28_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA28_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA28_A {
        match self.bits {
            false => CMPSTCHA28_A::_0,
            true => CMPSTCHA28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA28_A::_1
    }
}
#[doc = "Field `CMPSTCHA28` writer - Compare Window A Flag"]
pub type CMPSTCHA28_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR1_SPEC, CMPSTCHA28_A, O>;
impl<'a, const O: u8> CMPSTCHA28_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA28_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA28_A::_1)
    }
}
#[doc = "Field `CMPSTCHA29` reader - Compare Window A Flag"]
pub type CMPSTCHA29_R = crate::BitReader<CMPSTCHA29_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA29_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA29_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA29_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA29_A {
        match self.bits {
            false => CMPSTCHA29_A::_0,
            true => CMPSTCHA29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA29_A::_1
    }
}
#[doc = "Field `CMPSTCHA29` writer - Compare Window A Flag"]
pub type CMPSTCHA29_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR1_SPEC, CMPSTCHA29_A, O>;
impl<'a, const O: u8> CMPSTCHA29_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA29_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA29_A::_1)
    }
}
#[doc = "Field `CMPSTCHA30` reader - Compare Window A Flag"]
pub type CMPSTCHA30_R = crate::BitReader<CMPSTCHA30_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA30_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA30_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA30_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA30_A {
        match self.bits {
            false => CMPSTCHA30_A::_0,
            true => CMPSTCHA30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA30_A::_1
    }
}
#[doc = "Field `CMPSTCHA30` writer - Compare Window A Flag"]
pub type CMPSTCHA30_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR1_SPEC, CMPSTCHA30_A, O>;
impl<'a, const O: u8> CMPSTCHA30_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA30_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA30_A::_1)
    }
}
#[doc = "Field `CMPSTCHA31` reader - Compare Window A Flag"]
pub type CMPSTCHA31_R = crate::BitReader<CMPSTCHA31_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA31_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA31_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA31_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA31_A {
        match self.bits {
            false => CMPSTCHA31_A::_0,
            true => CMPSTCHA31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA31_A::_1
    }
}
#[doc = "Field `CMPSTCHA31` writer - Compare Window A Flag"]
pub type CMPSTCHA31_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR1_SPEC, CMPSTCHA31_A, O>;
impl<'a, const O: u8> CMPSTCHA31_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA31_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA31_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha16(&self) -> CMPSTCHA16_R {
        CMPSTCHA16_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha17(&self) -> CMPSTCHA17_R {
        CMPSTCHA17_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha18(&self) -> CMPSTCHA18_R {
        CMPSTCHA18_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha19(&self) -> CMPSTCHA19_R {
        CMPSTCHA19_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha20(&self) -> CMPSTCHA20_R {
        CMPSTCHA20_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha21(&self) -> CMPSTCHA21_R {
        CMPSTCHA21_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha22(&self) -> CMPSTCHA22_R {
        CMPSTCHA22_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha23(&self) -> CMPSTCHA23_R {
        CMPSTCHA23_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha24(&self) -> CMPSTCHA24_R {
        CMPSTCHA24_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha25(&self) -> CMPSTCHA25_R {
        CMPSTCHA25_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha26(&self) -> CMPSTCHA26_R {
        CMPSTCHA26_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha27(&self) -> CMPSTCHA27_R {
        CMPSTCHA27_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha28(&self) -> CMPSTCHA28_R {
        CMPSTCHA28_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha29(&self) -> CMPSTCHA29_R {
        CMPSTCHA29_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha30(&self) -> CMPSTCHA30_R {
        CMPSTCHA30_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha31(&self) -> CMPSTCHA31_R {
        CMPSTCHA31_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha16(&mut self) -> CMPSTCHA16_W<0> {
        CMPSTCHA16_W::new(self)
    }
    #[doc = "Bit 1 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha17(&mut self) -> CMPSTCHA17_W<1> {
        CMPSTCHA17_W::new(self)
    }
    #[doc = "Bit 2 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha18(&mut self) -> CMPSTCHA18_W<2> {
        CMPSTCHA18_W::new(self)
    }
    #[doc = "Bit 3 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha19(&mut self) -> CMPSTCHA19_W<3> {
        CMPSTCHA19_W::new(self)
    }
    #[doc = "Bit 4 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha20(&mut self) -> CMPSTCHA20_W<4> {
        CMPSTCHA20_W::new(self)
    }
    #[doc = "Bit 5 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha21(&mut self) -> CMPSTCHA21_W<5> {
        CMPSTCHA21_W::new(self)
    }
    #[doc = "Bit 6 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha22(&mut self) -> CMPSTCHA22_W<6> {
        CMPSTCHA22_W::new(self)
    }
    #[doc = "Bit 7 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha23(&mut self) -> CMPSTCHA23_W<7> {
        CMPSTCHA23_W::new(self)
    }
    #[doc = "Bit 8 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha24(&mut self) -> CMPSTCHA24_W<8> {
        CMPSTCHA24_W::new(self)
    }
    #[doc = "Bit 9 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha25(&mut self) -> CMPSTCHA25_W<9> {
        CMPSTCHA25_W::new(self)
    }
    #[doc = "Bit 10 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha26(&mut self) -> CMPSTCHA26_W<10> {
        CMPSTCHA26_W::new(self)
    }
    #[doc = "Bit 11 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha27(&mut self) -> CMPSTCHA27_W<11> {
        CMPSTCHA27_W::new(self)
    }
    #[doc = "Bit 12 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha28(&mut self) -> CMPSTCHA28_W<12> {
        CMPSTCHA28_W::new(self)
    }
    #[doc = "Bit 13 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha29(&mut self) -> CMPSTCHA29_W<13> {
        CMPSTCHA29_W::new(self)
    }
    #[doc = "Bit 14 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha30(&mut self) -> CMPSTCHA30_W<14> {
        CMPSTCHA30_W::new(self)
    }
    #[doc = "Bit 15 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha31(&mut self) -> CMPSTCHA31_W<15> {
        CMPSTCHA31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Compare Function Window A Channel Status Register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpsr1](index.html) module"]
pub struct ADCMPSR1_SPEC;
impl crate::RegisterSpec for ADCMPSR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcmpsr1::R](R) reader structure"]
impl crate::Readable for ADCMPSR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmpsr1::W](W) writer structure"]
impl crate::Writable for ADCMPSR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPSR1 to value 0"]
impl crate::Resettable for ADCMPSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
