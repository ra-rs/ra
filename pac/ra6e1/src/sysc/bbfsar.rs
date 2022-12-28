#[doc = "Register `BBFSAR` reader"]
pub struct R(crate::R<BBFSAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BBFSAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BBFSAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BBFSAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BBFSAR` writer"]
pub struct W(crate::W<BBFSAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BBFSAR_SPEC>;
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
impl From<crate::W<BBFSAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BBFSAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NONSEC0` reader - Non Secure Attribute bit 0"]
pub type NONSEC0_R = crate::BitReader<NONSEC0_A>;
#[doc = "Non Secure Attribute bit 0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC0_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC0_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC0_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC0_A {
        match self.bits {
            false => NONSEC0_A::_0,
            true => NONSEC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC0_A::_1
    }
}
#[doc = "Field `NONSEC0` writer - Non Secure Attribute bit 0"]
pub type NONSEC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BBFSAR_SPEC, NONSEC0_A, O>;
impl<'a, const O: u8> NONSEC0_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC0_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC0_A::_1)
    }
}
#[doc = "Field `NONSEC1` reader - Non Secure Attribute bit 1"]
pub type NONSEC1_R = crate::BitReader<NONSEC1_A>;
#[doc = "Non Secure Attribute bit 1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC1_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC1_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC1_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC1_A {
        match self.bits {
            false => NONSEC1_A::_0,
            true => NONSEC1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC1_A::_1
    }
}
#[doc = "Field `NONSEC1` writer - Non Secure Attribute bit 1"]
pub type NONSEC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BBFSAR_SPEC, NONSEC1_A, O>;
impl<'a, const O: u8> NONSEC1_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC1_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC1_A::_1)
    }
}
#[doc = "Field `NONSEC2` reader - Non Secure Attribute bit 2"]
pub type NONSEC2_R = crate::BitReader<NONSEC2_A>;
#[doc = "Non Secure Attribute bit 2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC2_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC2_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC2_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC2_A {
        match self.bits {
            false => NONSEC2_A::_0,
            true => NONSEC2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC2_A::_1
    }
}
#[doc = "Field `NONSEC2` writer - Non Secure Attribute bit 2"]
pub type NONSEC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BBFSAR_SPEC, NONSEC2_A, O>;
impl<'a, const O: u8> NONSEC2_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC2_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC2_A::_1)
    }
}
#[doc = "Field `NONSEC16` reader - Non Secure Attribute bit 16"]
pub type NONSEC16_R = crate::BitReader<NONSEC16_A>;
#[doc = "Non Secure Attribute bit 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC16_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC16_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC16_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC16_A {
        match self.bits {
            false => NONSEC16_A::_0,
            true => NONSEC16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC16_A::_1
    }
}
#[doc = "Field `NONSEC16` writer - Non Secure Attribute bit 16"]
pub type NONSEC16_W<'a, const O: u8> = crate::BitWriter<'a, u32, BBFSAR_SPEC, NONSEC16_A, O>;
impl<'a, const O: u8> NONSEC16_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC16_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC16_A::_1)
    }
}
#[doc = "Field `NONSEC17` reader - Non Secure Attribute bit 17"]
pub type NONSEC17_R = crate::BitReader<NONSEC17_A>;
#[doc = "Non Secure Attribute bit 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC17_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC17_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC17_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC17_A {
        match self.bits {
            false => NONSEC17_A::_0,
            true => NONSEC17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC17_A::_1
    }
}
#[doc = "Field `NONSEC17` writer - Non Secure Attribute bit 17"]
pub type NONSEC17_W<'a, const O: u8> = crate::BitWriter<'a, u32, BBFSAR_SPEC, NONSEC17_A, O>;
impl<'a, const O: u8> NONSEC17_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC17_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC17_A::_1)
    }
}
#[doc = "Field `NONSEC18` reader - Non Secure Attribute bit 18"]
pub type NONSEC18_R = crate::BitReader<NONSEC18_A>;
#[doc = "Non Secure Attribute bit 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC18_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC18_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC18_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC18_A {
        match self.bits {
            false => NONSEC18_A::_0,
            true => NONSEC18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC18_A::_1
    }
}
#[doc = "Field `NONSEC18` writer - Non Secure Attribute bit 18"]
pub type NONSEC18_W<'a, const O: u8> = crate::BitWriter<'a, u32, BBFSAR_SPEC, NONSEC18_A, O>;
impl<'a, const O: u8> NONSEC18_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC18_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC18_A::_1)
    }
}
#[doc = "Field `NONSEC19` reader - Non Secure Attribute bit 19"]
pub type NONSEC19_R = crate::BitReader<NONSEC19_A>;
#[doc = "Non Secure Attribute bit 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC19_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC19_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC19_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC19_A {
        match self.bits {
            false => NONSEC19_A::_0,
            true => NONSEC19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC19_A::_1
    }
}
#[doc = "Field `NONSEC19` writer - Non Secure Attribute bit 19"]
pub type NONSEC19_W<'a, const O: u8> = crate::BitWriter<'a, u32, BBFSAR_SPEC, NONSEC19_A, O>;
impl<'a, const O: u8> NONSEC19_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC19_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC19_A::_1)
    }
}
#[doc = "Field `NONSEC20` reader - Non Secure Attribute bit 20"]
pub type NONSEC20_R = crate::BitReader<NONSEC20_A>;
#[doc = "Non Secure Attribute bit 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC20_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC20_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC20_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC20_A {
        match self.bits {
            false => NONSEC20_A::_0,
            true => NONSEC20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC20_A::_1
    }
}
#[doc = "Field `NONSEC20` writer - Non Secure Attribute bit 20"]
pub type NONSEC20_W<'a, const O: u8> = crate::BitWriter<'a, u32, BBFSAR_SPEC, NONSEC20_A, O>;
impl<'a, const O: u8> NONSEC20_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC20_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC20_A::_1)
    }
}
#[doc = "Field `NONSEC21` reader - Non Secure Attribute bit 21"]
pub type NONSEC21_R = crate::BitReader<NONSEC21_A>;
#[doc = "Non Secure Attribute bit 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC21_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC21_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC21_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC21_A {
        match self.bits {
            false => NONSEC21_A::_0,
            true => NONSEC21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC21_A::_1
    }
}
#[doc = "Field `NONSEC21` writer - Non Secure Attribute bit 21"]
pub type NONSEC21_W<'a, const O: u8> = crate::BitWriter<'a, u32, BBFSAR_SPEC, NONSEC21_A, O>;
impl<'a, const O: u8> NONSEC21_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC21_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC21_A::_1)
    }
}
#[doc = "Field `NONSEC22` reader - Non Secure Attribute bit 22"]
pub type NONSEC22_R = crate::BitReader<NONSEC22_A>;
#[doc = "Non Secure Attribute bit 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC22_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC22_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC22_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC22_A {
        match self.bits {
            false => NONSEC22_A::_0,
            true => NONSEC22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC22_A::_1
    }
}
#[doc = "Field `NONSEC22` writer - Non Secure Attribute bit 22"]
pub type NONSEC22_W<'a, const O: u8> = crate::BitWriter<'a, u32, BBFSAR_SPEC, NONSEC22_A, O>;
impl<'a, const O: u8> NONSEC22_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC22_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC22_A::_1)
    }
}
#[doc = "Field `NONSEC23` reader - Non Secure Attribute bit 23"]
pub type NONSEC23_R = crate::BitReader<NONSEC23_A>;
#[doc = "Non Secure Attribute bit 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC23_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC23_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC23_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC23_A {
        match self.bits {
            false => NONSEC23_A::_0,
            true => NONSEC23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC23_A::_1
    }
}
#[doc = "Field `NONSEC23` writer - Non Secure Attribute bit 23"]
pub type NONSEC23_W<'a, const O: u8> = crate::BitWriter<'a, u32, BBFSAR_SPEC, NONSEC23_A, O>;
impl<'a, const O: u8> NONSEC23_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC23_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC23_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Non Secure Attribute bit 0"]
    #[inline(always)]
    pub fn nonsec0(&self) -> NONSEC0_R {
        NONSEC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Non Secure Attribute bit 1"]
    #[inline(always)]
    pub fn nonsec1(&self) -> NONSEC1_R {
        NONSEC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Non Secure Attribute bit 2"]
    #[inline(always)]
    pub fn nonsec2(&self) -> NONSEC2_R {
        NONSEC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Non Secure Attribute bit 16"]
    #[inline(always)]
    pub fn nonsec16(&self) -> NONSEC16_R {
        NONSEC16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Non Secure Attribute bit 17"]
    #[inline(always)]
    pub fn nonsec17(&self) -> NONSEC17_R {
        NONSEC17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Non Secure Attribute bit 18"]
    #[inline(always)]
    pub fn nonsec18(&self) -> NONSEC18_R {
        NONSEC18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Non Secure Attribute bit 19"]
    #[inline(always)]
    pub fn nonsec19(&self) -> NONSEC19_R {
        NONSEC19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Non Secure Attribute bit 20"]
    #[inline(always)]
    pub fn nonsec20(&self) -> NONSEC20_R {
        NONSEC20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Non Secure Attribute bit 21"]
    #[inline(always)]
    pub fn nonsec21(&self) -> NONSEC21_R {
        NONSEC21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Non Secure Attribute bit 22"]
    #[inline(always)]
    pub fn nonsec22(&self) -> NONSEC22_R {
        NONSEC22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Non Secure Attribute bit 23"]
    #[inline(always)]
    pub fn nonsec23(&self) -> NONSEC23_R {
        NONSEC23_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non Secure Attribute bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec0(&mut self) -> NONSEC0_W<0> {
        NONSEC0_W::new(self)
    }
    #[doc = "Bit 1 - Non Secure Attribute bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec1(&mut self) -> NONSEC1_W<1> {
        NONSEC1_W::new(self)
    }
    #[doc = "Bit 2 - Non Secure Attribute bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec2(&mut self) -> NONSEC2_W<2> {
        NONSEC2_W::new(self)
    }
    #[doc = "Bit 16 - Non Secure Attribute bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec16(&mut self) -> NONSEC16_W<16> {
        NONSEC16_W::new(self)
    }
    #[doc = "Bit 17 - Non Secure Attribute bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec17(&mut self) -> NONSEC17_W<17> {
        NONSEC17_W::new(self)
    }
    #[doc = "Bit 18 - Non Secure Attribute bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec18(&mut self) -> NONSEC18_W<18> {
        NONSEC18_W::new(self)
    }
    #[doc = "Bit 19 - Non Secure Attribute bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec19(&mut self) -> NONSEC19_W<19> {
        NONSEC19_W::new(self)
    }
    #[doc = "Bit 20 - Non Secure Attribute bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec20(&mut self) -> NONSEC20_W<20> {
        NONSEC20_W::new(self)
    }
    #[doc = "Bit 21 - Non Secure Attribute bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec21(&mut self) -> NONSEC21_W<21> {
        NONSEC21_W::new(self)
    }
    #[doc = "Bit 22 - Non Secure Attribute bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec22(&mut self) -> NONSEC22_W<22> {
        NONSEC22_W::new(self)
    }
    #[doc = "Bit 23 - Non Secure Attribute bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec23(&mut self) -> NONSEC23_W<23> {
        NONSEC23_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Battery Backup Function Security Attribute Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bbfsar](index.html) module"]
pub struct BBFSAR_SPEC;
impl crate::RegisterSpec for BBFSAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bbfsar::R](R) reader structure"]
impl crate::Readable for BBFSAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bbfsar::W](W) writer structure"]
impl crate::Writable for BBFSAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BBFSAR to value 0xffff"]
impl crate::Resettable for BBFSAR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
