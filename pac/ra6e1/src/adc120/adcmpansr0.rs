#[doc = "Register `ADCMPANSR0` reader"]
pub struct R(crate::R<ADCMPANSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPANSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPANSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPANSR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPANSR0` writer"]
pub struct W(crate::W<ADCMPANSR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPANSR0_SPEC>;
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
impl From<crate::W<ADCMPANSR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPANSR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPCHA0` reader - Compare Window A Channel Select n"]
pub type CMPCHA0_R = crate::BitReader<CMPCHA0_A>;
#[doc = "Compare Window A Channel Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA0_A {
    #[doc = "0: Disable compare function for associated input channel"]
    _0 = 0,
    #[doc = "1: Enable compare function for associated input channel"]
    _1 = 1,
}
impl From<CMPCHA0_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA0_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA0_A {
        match self.bits {
            false => CMPCHA0_A::_0,
            true => CMPCHA0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA0_A::_1
    }
}
#[doc = "Field `CMPCHA0` writer - Compare Window A Channel Select n"]
pub type CMPCHA0_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA0_A, O>;
impl<'a, const O: u8> CMPCHA0_W<'a, O> {
    #[doc = "Disable compare function for associated input channel"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA0_A::_0)
    }
    #[doc = "Enable compare function for associated input channel"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA0_A::_1)
    }
}
#[doc = "Field `CMPCHA1` reader - Compare Window A Channel Select n"]
pub type CMPCHA1_R = crate::BitReader<CMPCHA1_A>;
#[doc = "Compare Window A Channel Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA1_A {
    #[doc = "0: Disable compare function for associated input channel"]
    _0 = 0,
    #[doc = "1: Enable compare function for associated input channel"]
    _1 = 1,
}
impl From<CMPCHA1_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA1_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA1_A {
        match self.bits {
            false => CMPCHA1_A::_0,
            true => CMPCHA1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA1_A::_1
    }
}
#[doc = "Field `CMPCHA1` writer - Compare Window A Channel Select n"]
pub type CMPCHA1_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA1_A, O>;
impl<'a, const O: u8> CMPCHA1_W<'a, O> {
    #[doc = "Disable compare function for associated input channel"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA1_A::_0)
    }
    #[doc = "Enable compare function for associated input channel"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA1_A::_1)
    }
}
#[doc = "Field `CMPCHA2` reader - Compare Window A Channel Select n"]
pub type CMPCHA2_R = crate::BitReader<CMPCHA2_A>;
#[doc = "Compare Window A Channel Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA2_A {
    #[doc = "0: Disable compare function for associated input channel"]
    _0 = 0,
    #[doc = "1: Enable compare function for associated input channel"]
    _1 = 1,
}
impl From<CMPCHA2_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA2_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA2_A {
        match self.bits {
            false => CMPCHA2_A::_0,
            true => CMPCHA2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA2_A::_1
    }
}
#[doc = "Field `CMPCHA2` writer - Compare Window A Channel Select n"]
pub type CMPCHA2_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA2_A, O>;
impl<'a, const O: u8> CMPCHA2_W<'a, O> {
    #[doc = "Disable compare function for associated input channel"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA2_A::_0)
    }
    #[doc = "Enable compare function for associated input channel"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA2_A::_1)
    }
}
#[doc = "Field `CMPCHA3` reader - Compare Window A Channel Select n"]
pub type CMPCHA3_R = crate::BitReader<CMPCHA3_A>;
#[doc = "Compare Window A Channel Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA3_A {
    #[doc = "0: Disable compare function for associated input channel"]
    _0 = 0,
    #[doc = "1: Enable compare function for associated input channel"]
    _1 = 1,
}
impl From<CMPCHA3_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA3_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA3_A {
        match self.bits {
            false => CMPCHA3_A::_0,
            true => CMPCHA3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA3_A::_1
    }
}
#[doc = "Field `CMPCHA3` writer - Compare Window A Channel Select n"]
pub type CMPCHA3_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA3_A, O>;
impl<'a, const O: u8> CMPCHA3_W<'a, O> {
    #[doc = "Disable compare function for associated input channel"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA3_A::_0)
    }
    #[doc = "Enable compare function for associated input channel"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA3_A::_1)
    }
}
#[doc = "Field `CMPCHA4` reader - Compare Window A Channel Select n"]
pub type CMPCHA4_R = crate::BitReader<CMPCHA4_A>;
#[doc = "Compare Window A Channel Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA4_A {
    #[doc = "0: Disable compare function for associated input channel"]
    _0 = 0,
    #[doc = "1: Enable compare function for associated input channel"]
    _1 = 1,
}
impl From<CMPCHA4_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA4_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA4_A {
        match self.bits {
            false => CMPCHA4_A::_0,
            true => CMPCHA4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA4_A::_1
    }
}
#[doc = "Field `CMPCHA4` writer - Compare Window A Channel Select n"]
pub type CMPCHA4_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA4_A, O>;
impl<'a, const O: u8> CMPCHA4_W<'a, O> {
    #[doc = "Disable compare function for associated input channel"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA4_A::_0)
    }
    #[doc = "Enable compare function for associated input channel"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA4_A::_1)
    }
}
#[doc = "Field `CMPCHA5` reader - Compare Window A Channel Select n"]
pub type CMPCHA5_R = crate::BitReader<CMPCHA5_A>;
#[doc = "Compare Window A Channel Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA5_A {
    #[doc = "0: Disable compare function for associated input channel"]
    _0 = 0,
    #[doc = "1: Enable compare function for associated input channel"]
    _1 = 1,
}
impl From<CMPCHA5_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA5_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA5_A {
        match self.bits {
            false => CMPCHA5_A::_0,
            true => CMPCHA5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA5_A::_1
    }
}
#[doc = "Field `CMPCHA5` writer - Compare Window A Channel Select n"]
pub type CMPCHA5_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA5_A, O>;
impl<'a, const O: u8> CMPCHA5_W<'a, O> {
    #[doc = "Disable compare function for associated input channel"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA5_A::_0)
    }
    #[doc = "Enable compare function for associated input channel"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA5_A::_1)
    }
}
#[doc = "Field `CMPCHA6` reader - Compare Window A Channel Select n"]
pub type CMPCHA6_R = crate::BitReader<CMPCHA6_A>;
#[doc = "Compare Window A Channel Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA6_A {
    #[doc = "0: Disable compare function for associated input channel"]
    _0 = 0,
    #[doc = "1: Enable compare function for associated input channel"]
    _1 = 1,
}
impl From<CMPCHA6_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA6_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA6_A {
        match self.bits {
            false => CMPCHA6_A::_0,
            true => CMPCHA6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA6_A::_1
    }
}
#[doc = "Field `CMPCHA6` writer - Compare Window A Channel Select n"]
pub type CMPCHA6_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA6_A, O>;
impl<'a, const O: u8> CMPCHA6_W<'a, O> {
    #[doc = "Disable compare function for associated input channel"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA6_A::_0)
    }
    #[doc = "Enable compare function for associated input channel"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA6_A::_1)
    }
}
#[doc = "Field `CMPCHA7` reader - Compare Window A Channel Select n"]
pub type CMPCHA7_R = crate::BitReader<CMPCHA7_A>;
#[doc = "Compare Window A Channel Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA7_A {
    #[doc = "0: Disable compare function for associated input channel"]
    _0 = 0,
    #[doc = "1: Enable compare function for associated input channel"]
    _1 = 1,
}
impl From<CMPCHA7_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA7_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA7_A {
        match self.bits {
            false => CMPCHA7_A::_0,
            true => CMPCHA7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA7_A::_1
    }
}
#[doc = "Field `CMPCHA7` writer - Compare Window A Channel Select n"]
pub type CMPCHA7_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA7_A, O>;
impl<'a, const O: u8> CMPCHA7_W<'a, O> {
    #[doc = "Disable compare function for associated input channel"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA7_A::_0)
    }
    #[doc = "Enable compare function for associated input channel"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA7_A::_1)
    }
}
#[doc = "Field `CMPCHA8` reader - Compare Window A Channel Select n"]
pub type CMPCHA8_R = crate::BitReader<CMPCHA8_A>;
#[doc = "Compare Window A Channel Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA8_A {
    #[doc = "0: Disable compare function for associated input channel"]
    _0 = 0,
    #[doc = "1: Enable compare function for associated input channel"]
    _1 = 1,
}
impl From<CMPCHA8_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA8_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA8_A {
        match self.bits {
            false => CMPCHA8_A::_0,
            true => CMPCHA8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA8_A::_1
    }
}
#[doc = "Field `CMPCHA8` writer - Compare Window A Channel Select n"]
pub type CMPCHA8_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA8_A, O>;
impl<'a, const O: u8> CMPCHA8_W<'a, O> {
    #[doc = "Disable compare function for associated input channel"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA8_A::_0)
    }
    #[doc = "Enable compare function for associated input channel"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA8_A::_1)
    }
}
#[doc = "Field `CMPCHA12` reader - Compare Window A Channel Select n"]
pub type CMPCHA12_R = crate::BitReader<CMPCHA12_A>;
#[doc = "Compare Window A Channel Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA12_A {
    #[doc = "0: Disable compare function for associated input channel"]
    _0 = 0,
    #[doc = "1: Enable compare function for associated input channel"]
    _1 = 1,
}
impl From<CMPCHA12_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA12_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA12_A {
        match self.bits {
            false => CMPCHA12_A::_0,
            true => CMPCHA12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA12_A::_1
    }
}
#[doc = "Field `CMPCHA12` writer - Compare Window A Channel Select n"]
pub type CMPCHA12_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA12_A, O>;
impl<'a, const O: u8> CMPCHA12_W<'a, O> {
    #[doc = "Disable compare function for associated input channel"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA12_A::_0)
    }
    #[doc = "Enable compare function for associated input channel"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA12_A::_1)
    }
}
#[doc = "Field `CMPCHA13` reader - Compare Window A Channel Select n"]
pub type CMPCHA13_R = crate::BitReader<CMPCHA13_A>;
#[doc = "Compare Window A Channel Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA13_A {
    #[doc = "0: Disable compare function for associated input channel"]
    _0 = 0,
    #[doc = "1: Enable compare function for associated input channel"]
    _1 = 1,
}
impl From<CMPCHA13_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA13_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA13_A {
        match self.bits {
            false => CMPCHA13_A::_0,
            true => CMPCHA13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA13_A::_1
    }
}
#[doc = "Field `CMPCHA13` writer - Compare Window A Channel Select n"]
pub type CMPCHA13_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA13_A, O>;
impl<'a, const O: u8> CMPCHA13_W<'a, O> {
    #[doc = "Disable compare function for associated input channel"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA13_A::_0)
    }
    #[doc = "Enable compare function for associated input channel"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA13_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Compare Window A Channel Select n"]
    #[inline(always)]
    pub fn cmpcha0(&self) -> CMPCHA0_R {
        CMPCHA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare Window A Channel Select n"]
    #[inline(always)]
    pub fn cmpcha1(&self) -> CMPCHA1_R {
        CMPCHA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare Window A Channel Select n"]
    #[inline(always)]
    pub fn cmpcha2(&self) -> CMPCHA2_R {
        CMPCHA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare Window A Channel Select n"]
    #[inline(always)]
    pub fn cmpcha3(&self) -> CMPCHA3_R {
        CMPCHA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare Window A Channel Select n"]
    #[inline(always)]
    pub fn cmpcha4(&self) -> CMPCHA4_R {
        CMPCHA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Compare Window A Channel Select n"]
    #[inline(always)]
    pub fn cmpcha5(&self) -> CMPCHA5_R {
        CMPCHA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Compare Window A Channel Select n"]
    #[inline(always)]
    pub fn cmpcha6(&self) -> CMPCHA6_R {
        CMPCHA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Compare Window A Channel Select n"]
    #[inline(always)]
    pub fn cmpcha7(&self) -> CMPCHA7_R {
        CMPCHA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Compare Window A Channel Select n"]
    #[inline(always)]
    pub fn cmpcha8(&self) -> CMPCHA8_R {
        CMPCHA8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Compare Window A Channel Select n"]
    #[inline(always)]
    pub fn cmpcha12(&self) -> CMPCHA12_R {
        CMPCHA12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Compare Window A Channel Select n"]
    #[inline(always)]
    pub fn cmpcha13(&self) -> CMPCHA13_R {
        CMPCHA13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare Window A Channel Select n"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha0(&mut self) -> CMPCHA0_W<0> {
        CMPCHA0_W::new(self)
    }
    #[doc = "Bit 1 - Compare Window A Channel Select n"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha1(&mut self) -> CMPCHA1_W<1> {
        CMPCHA1_W::new(self)
    }
    #[doc = "Bit 2 - Compare Window A Channel Select n"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha2(&mut self) -> CMPCHA2_W<2> {
        CMPCHA2_W::new(self)
    }
    #[doc = "Bit 3 - Compare Window A Channel Select n"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha3(&mut self) -> CMPCHA3_W<3> {
        CMPCHA3_W::new(self)
    }
    #[doc = "Bit 4 - Compare Window A Channel Select n"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha4(&mut self) -> CMPCHA4_W<4> {
        CMPCHA4_W::new(self)
    }
    #[doc = "Bit 5 - Compare Window A Channel Select n"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha5(&mut self) -> CMPCHA5_W<5> {
        CMPCHA5_W::new(self)
    }
    #[doc = "Bit 6 - Compare Window A Channel Select n"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha6(&mut self) -> CMPCHA6_W<6> {
        CMPCHA6_W::new(self)
    }
    #[doc = "Bit 7 - Compare Window A Channel Select n"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha7(&mut self) -> CMPCHA7_W<7> {
        CMPCHA7_W::new(self)
    }
    #[doc = "Bit 8 - Compare Window A Channel Select n"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha8(&mut self) -> CMPCHA8_W<8> {
        CMPCHA8_W::new(self)
    }
    #[doc = "Bit 12 - Compare Window A Channel Select n"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha12(&mut self) -> CMPCHA12_W<12> {
        CMPCHA12_W::new(self)
    }
    #[doc = "Bit 13 - Compare Window A Channel Select n"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha13(&mut self) -> CMPCHA13_W<13> {
        CMPCHA13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Compare Function Window A Channel Select Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpansr0](index.html) module"]
pub struct ADCMPANSR0_SPEC;
impl crate::RegisterSpec for ADCMPANSR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcmpansr0::R](R) reader structure"]
impl crate::Readable for ADCMPANSR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmpansr0::W](W) writer structure"]
impl crate::Writable for ADCMPANSR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPANSR0 to value 0"]
impl crate::Resettable for ADCMPANSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
