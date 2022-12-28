#[doc = "Register `ADANSB1` reader"]
pub struct R(crate::R<ADANSB1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADANSB1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADANSB1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADANSB1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADANSB1` writer"]
pub struct W(crate::W<ADANSB1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADANSB1_SPEC>;
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
impl From<crate::W<ADANSB1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADANSB1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANSB16` reader - A/D Conversion Channels Select"]
pub type ANSB16_R = crate::BitReader<ANSB16_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB16_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSB16_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB16_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB16_A {
        match self.bits {
            false => ANSB16_A::_0,
            true => ANSB16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB16_A::_1
    }
}
#[doc = "Field `ANSB16` writer - A/D Conversion Channels Select"]
pub type ANSB16_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB16_A, O>;
impl<'a, const O: u8> ANSB16_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB16_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB16_A::_1)
    }
}
#[doc = "Field `ANSB17` reader - A/D Conversion Channels Select"]
pub type ANSB17_R = crate::BitReader<ANSB17_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB17_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSB17_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB17_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB17_A {
        match self.bits {
            false => ANSB17_A::_0,
            true => ANSB17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB17_A::_1
    }
}
#[doc = "Field `ANSB17` writer - A/D Conversion Channels Select"]
pub type ANSB17_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB17_A, O>;
impl<'a, const O: u8> ANSB17_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB17_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB17_A::_1)
    }
}
#[doc = "Field `ANSB18` reader - A/D Conversion Channels Select"]
pub type ANSB18_R = crate::BitReader<ANSB18_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB18_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSB18_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB18_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB18_A {
        match self.bits {
            false => ANSB18_A::_0,
            true => ANSB18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB18_A::_1
    }
}
#[doc = "Field `ANSB18` writer - A/D Conversion Channels Select"]
pub type ANSB18_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB18_A, O>;
impl<'a, const O: u8> ANSB18_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB18_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB18_A::_1)
    }
}
#[doc = "Field `ANSB19` reader - A/D Conversion Channels Select"]
pub type ANSB19_R = crate::BitReader<ANSB19_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB19_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSB19_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB19_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB19_A {
        match self.bits {
            false => ANSB19_A::_0,
            true => ANSB19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB19_A::_1
    }
}
#[doc = "Field `ANSB19` writer - A/D Conversion Channels Select"]
pub type ANSB19_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB19_A, O>;
impl<'a, const O: u8> ANSB19_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB19_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB19_A::_1)
    }
}
#[doc = "Field `ANSB20` reader - A/D Conversion Channels Select"]
pub type ANSB20_R = crate::BitReader<ANSB20_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB20_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSB20_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB20_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB20_A {
        match self.bits {
            false => ANSB20_A::_0,
            true => ANSB20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB20_A::_1
    }
}
#[doc = "Field `ANSB20` writer - A/D Conversion Channels Select"]
pub type ANSB20_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB20_A, O>;
impl<'a, const O: u8> ANSB20_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB20_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB20_A::_1)
    }
}
#[doc = "Field `ANSB21` reader - A/D Conversion Channels Select"]
pub type ANSB21_R = crate::BitReader<ANSB21_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB21_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSB21_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB21_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB21_A {
        match self.bits {
            false => ANSB21_A::_0,
            true => ANSB21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB21_A::_1
    }
}
#[doc = "Field `ANSB21` writer - A/D Conversion Channels Select"]
pub type ANSB21_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB21_A, O>;
impl<'a, const O: u8> ANSB21_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB21_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB21_A::_1)
    }
}
#[doc = "Field `ANSB22` reader - A/D Conversion Channels Select"]
pub type ANSB22_R = crate::BitReader<ANSB22_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB22_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSB22_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB22_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB22_A {
        match self.bits {
            false => ANSB22_A::_0,
            true => ANSB22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB22_A::_1
    }
}
#[doc = "Field `ANSB22` writer - A/D Conversion Channels Select"]
pub type ANSB22_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB22_A, O>;
impl<'a, const O: u8> ANSB22_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB22_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB22_A::_1)
    }
}
#[doc = "Field `ANSB23` reader - A/D Conversion Channels Select"]
pub type ANSB23_R = crate::BitReader<ANSB23_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB23_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSB23_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB23_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB23_A {
        match self.bits {
            false => ANSB23_A::_0,
            true => ANSB23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB23_A::_1
    }
}
#[doc = "Field `ANSB23` writer - A/D Conversion Channels Select"]
pub type ANSB23_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB23_A, O>;
impl<'a, const O: u8> ANSB23_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB23_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB23_A::_1)
    }
}
#[doc = "Field `ANSB24` reader - A/D Conversion Channels Select"]
pub type ANSB24_R = crate::BitReader<ANSB24_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB24_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSB24_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB24_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB24_A {
        match self.bits {
            false => ANSB24_A::_0,
            true => ANSB24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB24_A::_1
    }
}
#[doc = "Field `ANSB24` writer - A/D Conversion Channels Select"]
pub type ANSB24_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB24_A, O>;
impl<'a, const O: u8> ANSB24_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB24_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB24_A::_1)
    }
}
#[doc = "Field `ANSB25` reader - A/D Conversion Channels Select"]
pub type ANSB25_R = crate::BitReader<ANSB25_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB25_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSB25_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB25_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB25_A {
        match self.bits {
            false => ANSB25_A::_0,
            true => ANSB25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB25_A::_1
    }
}
#[doc = "Field `ANSB25` writer - A/D Conversion Channels Select"]
pub type ANSB25_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB25_A, O>;
impl<'a, const O: u8> ANSB25_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB25_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB25_A::_1)
    }
}
#[doc = "Field `ANSB26` reader - A/D Conversion Channels Select"]
pub type ANSB26_R = crate::BitReader<ANSB26_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB26_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSB26_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB26_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB26_A {
        match self.bits {
            false => ANSB26_A::_0,
            true => ANSB26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB26_A::_1
    }
}
#[doc = "Field `ANSB26` writer - A/D Conversion Channels Select"]
pub type ANSB26_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB26_A, O>;
impl<'a, const O: u8> ANSB26_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB26_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB26_A::_1)
    }
}
#[doc = "Field `ANSB27` reader - A/D Conversion Channels Select"]
pub type ANSB27_R = crate::BitReader<ANSB27_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB27_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSB27_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB27_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB27_A {
        match self.bits {
            false => ANSB27_A::_0,
            true => ANSB27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB27_A::_1
    }
}
#[doc = "Field `ANSB27` writer - A/D Conversion Channels Select"]
pub type ANSB27_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB27_A, O>;
impl<'a, const O: u8> ANSB27_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB27_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB27_A::_1)
    }
}
#[doc = "Field `ANSB28` reader - A/D Conversion Channels Select"]
pub type ANSB28_R = crate::BitReader<ANSB28_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB28_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSB28_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB28_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB28_A {
        match self.bits {
            false => ANSB28_A::_0,
            true => ANSB28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB28_A::_1
    }
}
#[doc = "Field `ANSB28` writer - A/D Conversion Channels Select"]
pub type ANSB28_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB28_A, O>;
impl<'a, const O: u8> ANSB28_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB28_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB28_A::_1)
    }
}
#[doc = "Field `ANSB29` reader - A/D Conversion Channels Select"]
pub type ANSB29_R = crate::BitReader<ANSB29_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB29_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSB29_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB29_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB29_A {
        match self.bits {
            false => ANSB29_A::_0,
            true => ANSB29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB29_A::_1
    }
}
#[doc = "Field `ANSB29` writer - A/D Conversion Channels Select"]
pub type ANSB29_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB29_A, O>;
impl<'a, const O: u8> ANSB29_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB29_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB29_A::_1)
    }
}
#[doc = "Field `ANSB30` reader - A/D Conversion Channels Select"]
pub type ANSB30_R = crate::BitReader<ANSB30_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB30_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSB30_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB30_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB30_A {
        match self.bits {
            false => ANSB30_A::_0,
            true => ANSB30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB30_A::_1
    }
}
#[doc = "Field `ANSB30` writer - A/D Conversion Channels Select"]
pub type ANSB30_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB30_A, O>;
impl<'a, const O: u8> ANSB30_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB30_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB30_A::_1)
    }
}
#[doc = "Field `ANSB31` reader - A/D Conversion Channels Select"]
pub type ANSB31_R = crate::BitReader<ANSB31_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB31_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSB31_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB31_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB31_A {
        match self.bits {
            false => ANSB31_A::_0,
            true => ANSB31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB31_A::_1
    }
}
#[doc = "Field `ANSB31` writer - A/D Conversion Channels Select"]
pub type ANSB31_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB31_A, O>;
impl<'a, const O: u8> ANSB31_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB31_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB31_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb16(&self) -> ANSB16_R {
        ANSB16_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb17(&self) -> ANSB17_R {
        ANSB17_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb18(&self) -> ANSB18_R {
        ANSB18_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb19(&self) -> ANSB19_R {
        ANSB19_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb20(&self) -> ANSB20_R {
        ANSB20_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb21(&self) -> ANSB21_R {
        ANSB21_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb22(&self) -> ANSB22_R {
        ANSB22_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb23(&self) -> ANSB23_R {
        ANSB23_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb24(&self) -> ANSB24_R {
        ANSB24_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb25(&self) -> ANSB25_R {
        ANSB25_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb26(&self) -> ANSB26_R {
        ANSB26_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb27(&self) -> ANSB27_R {
        ANSB27_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb28(&self) -> ANSB28_R {
        ANSB28_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb29(&self) -> ANSB29_R {
        ANSB29_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb30(&self) -> ANSB30_R {
        ANSB30_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb31(&self) -> ANSB31_R {
        ANSB31_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb16(&mut self) -> ANSB16_W<0> {
        ANSB16_W::new(self)
    }
    #[doc = "Bit 1 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb17(&mut self) -> ANSB17_W<1> {
        ANSB17_W::new(self)
    }
    #[doc = "Bit 2 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb18(&mut self) -> ANSB18_W<2> {
        ANSB18_W::new(self)
    }
    #[doc = "Bit 3 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb19(&mut self) -> ANSB19_W<3> {
        ANSB19_W::new(self)
    }
    #[doc = "Bit 4 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb20(&mut self) -> ANSB20_W<4> {
        ANSB20_W::new(self)
    }
    #[doc = "Bit 5 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb21(&mut self) -> ANSB21_W<5> {
        ANSB21_W::new(self)
    }
    #[doc = "Bit 6 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb22(&mut self) -> ANSB22_W<6> {
        ANSB22_W::new(self)
    }
    #[doc = "Bit 7 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb23(&mut self) -> ANSB23_W<7> {
        ANSB23_W::new(self)
    }
    #[doc = "Bit 8 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb24(&mut self) -> ANSB24_W<8> {
        ANSB24_W::new(self)
    }
    #[doc = "Bit 9 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb25(&mut self) -> ANSB25_W<9> {
        ANSB25_W::new(self)
    }
    #[doc = "Bit 10 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb26(&mut self) -> ANSB26_W<10> {
        ANSB26_W::new(self)
    }
    #[doc = "Bit 11 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb27(&mut self) -> ANSB27_W<11> {
        ANSB27_W::new(self)
    }
    #[doc = "Bit 12 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb28(&mut self) -> ANSB28_W<12> {
        ANSB28_W::new(self)
    }
    #[doc = "Bit 13 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb29(&mut self) -> ANSB29_W<13> {
        ANSB29_W::new(self)
    }
    #[doc = "Bit 14 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb30(&mut self) -> ANSB30_W<14> {
        ANSB30_W::new(self)
    }
    #[doc = "Bit 15 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb31(&mut self) -> ANSB31_W<15> {
        ANSB31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Channel Select Register B1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adansb1](index.html) module"]
pub struct ADANSB1_SPEC;
impl crate::RegisterSpec for ADANSB1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adansb1::R](R) reader structure"]
impl crate::Readable for ADANSB1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adansb1::W](W) writer structure"]
impl crate::Writable for ADANSB1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADANSB1 to value 0"]
impl crate::Resettable for ADANSB1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
