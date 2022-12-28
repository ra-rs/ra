#[doc = "Register `ADCMPSR0` reader"]
pub struct R(crate::R<ADCMPSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPSR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPSR0` writer"]
pub struct W(crate::W<ADCMPSR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPSR0_SPEC>;
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
impl From<crate::W<ADCMPSR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPSR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPSTCHA0` reader - Compare Window A Flag n"]
pub type CMPSTCHA0_R = crate::BitReader<CMPSTCHA0_A>;
#[doc = "Compare Window A Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA0_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA0_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA0_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA0_A {
        match self.bits {
            false => CMPSTCHA0_A::_0,
            true => CMPSTCHA0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA0_A::_1
    }
}
#[doc = "Field `CMPSTCHA0` writer - Compare Window A Flag n"]
pub type CMPSTCHA0_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR0_SPEC, CMPSTCHA0_A, O>;
impl<'a, const O: u8> CMPSTCHA0_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA0_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA0_A::_1)
    }
}
#[doc = "Field `CMPSTCHA1` reader - Compare Window A Flag n"]
pub type CMPSTCHA1_R = crate::BitReader<CMPSTCHA1_A>;
#[doc = "Compare Window A Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA1_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA1_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA1_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA1_A {
        match self.bits {
            false => CMPSTCHA1_A::_0,
            true => CMPSTCHA1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA1_A::_1
    }
}
#[doc = "Field `CMPSTCHA1` writer - Compare Window A Flag n"]
pub type CMPSTCHA1_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR0_SPEC, CMPSTCHA1_A, O>;
impl<'a, const O: u8> CMPSTCHA1_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA1_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA1_A::_1)
    }
}
#[doc = "Field `CMPSTCHA2` reader - Compare Window A Flag n"]
pub type CMPSTCHA2_R = crate::BitReader<CMPSTCHA2_A>;
#[doc = "Compare Window A Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA2_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA2_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA2_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA2_A {
        match self.bits {
            false => CMPSTCHA2_A::_0,
            true => CMPSTCHA2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA2_A::_1
    }
}
#[doc = "Field `CMPSTCHA2` writer - Compare Window A Flag n"]
pub type CMPSTCHA2_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR0_SPEC, CMPSTCHA2_A, O>;
impl<'a, const O: u8> CMPSTCHA2_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA2_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA2_A::_1)
    }
}
#[doc = "Field `CMPSTCHA3` reader - Compare Window A Flag n"]
pub type CMPSTCHA3_R = crate::BitReader<CMPSTCHA3_A>;
#[doc = "Compare Window A Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA3_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA3_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA3_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA3_A {
        match self.bits {
            false => CMPSTCHA3_A::_0,
            true => CMPSTCHA3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA3_A::_1
    }
}
#[doc = "Field `CMPSTCHA3` writer - Compare Window A Flag n"]
pub type CMPSTCHA3_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR0_SPEC, CMPSTCHA3_A, O>;
impl<'a, const O: u8> CMPSTCHA3_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA3_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA3_A::_1)
    }
}
#[doc = "Field `CMPSTCHA4` reader - Compare Window A Flag n"]
pub type CMPSTCHA4_R = crate::BitReader<CMPSTCHA4_A>;
#[doc = "Compare Window A Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA4_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA4_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA4_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA4_A {
        match self.bits {
            false => CMPSTCHA4_A::_0,
            true => CMPSTCHA4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA4_A::_1
    }
}
#[doc = "Field `CMPSTCHA4` writer - Compare Window A Flag n"]
pub type CMPSTCHA4_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR0_SPEC, CMPSTCHA4_A, O>;
impl<'a, const O: u8> CMPSTCHA4_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA4_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA4_A::_1)
    }
}
#[doc = "Field `CMPSTCHA5` reader - Compare Window A Flag n"]
pub type CMPSTCHA5_R = crate::BitReader<CMPSTCHA5_A>;
#[doc = "Compare Window A Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA5_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA5_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA5_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA5_A {
        match self.bits {
            false => CMPSTCHA5_A::_0,
            true => CMPSTCHA5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA5_A::_1
    }
}
#[doc = "Field `CMPSTCHA5` writer - Compare Window A Flag n"]
pub type CMPSTCHA5_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR0_SPEC, CMPSTCHA5_A, O>;
impl<'a, const O: u8> CMPSTCHA5_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA5_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA5_A::_1)
    }
}
#[doc = "Field `CMPSTCHA6` reader - Compare Window A Flag n"]
pub type CMPSTCHA6_R = crate::BitReader<CMPSTCHA6_A>;
#[doc = "Compare Window A Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA6_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA6_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA6_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA6_A {
        match self.bits {
            false => CMPSTCHA6_A::_0,
            true => CMPSTCHA6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA6_A::_1
    }
}
#[doc = "Field `CMPSTCHA6` writer - Compare Window A Flag n"]
pub type CMPSTCHA6_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR0_SPEC, CMPSTCHA6_A, O>;
impl<'a, const O: u8> CMPSTCHA6_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA6_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA6_A::_1)
    }
}
#[doc = "Field `CMPSTCHA7` reader - Compare Window A Flag n"]
pub type CMPSTCHA7_R = crate::BitReader<CMPSTCHA7_A>;
#[doc = "Compare Window A Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA7_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA7_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA7_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA7_A {
        match self.bits {
            false => CMPSTCHA7_A::_0,
            true => CMPSTCHA7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA7_A::_1
    }
}
#[doc = "Field `CMPSTCHA7` writer - Compare Window A Flag n"]
pub type CMPSTCHA7_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR0_SPEC, CMPSTCHA7_A, O>;
impl<'a, const O: u8> CMPSTCHA7_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA7_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA7_A::_1)
    }
}
#[doc = "Field `CMPSTCHA8` reader - Compare Window A Flag n"]
pub type CMPSTCHA8_R = crate::BitReader<CMPSTCHA8_A>;
#[doc = "Compare Window A Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA8_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA8_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA8_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA8_A {
        match self.bits {
            false => CMPSTCHA8_A::_0,
            true => CMPSTCHA8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA8_A::_1
    }
}
#[doc = "Field `CMPSTCHA8` writer - Compare Window A Flag n"]
pub type CMPSTCHA8_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR0_SPEC, CMPSTCHA8_A, O>;
impl<'a, const O: u8> CMPSTCHA8_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA8_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA8_A::_1)
    }
}
#[doc = "Field `CMPSTCHA12` reader - Compare Window A Flag n"]
pub type CMPSTCHA12_R = crate::BitReader<CMPSTCHA12_A>;
#[doc = "Compare Window A Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA12_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA12_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA12_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA12_A {
        match self.bits {
            false => CMPSTCHA12_A::_0,
            true => CMPSTCHA12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA12_A::_1
    }
}
#[doc = "Field `CMPSTCHA12` writer - Compare Window A Flag n"]
pub type CMPSTCHA12_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR0_SPEC, CMPSTCHA12_A, O>;
impl<'a, const O: u8> CMPSTCHA12_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA12_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA12_A::_1)
    }
}
#[doc = "Field `CMPSTCHA13` reader - Compare Window A Flag n"]
pub type CMPSTCHA13_R = crate::BitReader<CMPSTCHA13_A>;
#[doc = "Compare Window A Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA13_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA13_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA13_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA13_A {
        match self.bits {
            false => CMPSTCHA13_A::_0,
            true => CMPSTCHA13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA13_A::_1
    }
}
#[doc = "Field `CMPSTCHA13` writer - Compare Window A Flag n"]
pub type CMPSTCHA13_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR0_SPEC, CMPSTCHA13_A, O>;
impl<'a, const O: u8> CMPSTCHA13_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA13_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA13_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Compare Window A Flag n"]
    #[inline(always)]
    pub fn cmpstcha0(&self) -> CMPSTCHA0_R {
        CMPSTCHA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare Window A Flag n"]
    #[inline(always)]
    pub fn cmpstcha1(&self) -> CMPSTCHA1_R {
        CMPSTCHA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare Window A Flag n"]
    #[inline(always)]
    pub fn cmpstcha2(&self) -> CMPSTCHA2_R {
        CMPSTCHA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare Window A Flag n"]
    #[inline(always)]
    pub fn cmpstcha3(&self) -> CMPSTCHA3_R {
        CMPSTCHA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare Window A Flag n"]
    #[inline(always)]
    pub fn cmpstcha4(&self) -> CMPSTCHA4_R {
        CMPSTCHA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Compare Window A Flag n"]
    #[inline(always)]
    pub fn cmpstcha5(&self) -> CMPSTCHA5_R {
        CMPSTCHA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Compare Window A Flag n"]
    #[inline(always)]
    pub fn cmpstcha6(&self) -> CMPSTCHA6_R {
        CMPSTCHA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Compare Window A Flag n"]
    #[inline(always)]
    pub fn cmpstcha7(&self) -> CMPSTCHA7_R {
        CMPSTCHA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Compare Window A Flag n"]
    #[inline(always)]
    pub fn cmpstcha8(&self) -> CMPSTCHA8_R {
        CMPSTCHA8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Compare Window A Flag n"]
    #[inline(always)]
    pub fn cmpstcha12(&self) -> CMPSTCHA12_R {
        CMPSTCHA12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Compare Window A Flag n"]
    #[inline(always)]
    pub fn cmpstcha13(&self) -> CMPSTCHA13_R {
        CMPSTCHA13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare Window A Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha0(&mut self) -> CMPSTCHA0_W<0> {
        CMPSTCHA0_W::new(self)
    }
    #[doc = "Bit 1 - Compare Window A Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha1(&mut self) -> CMPSTCHA1_W<1> {
        CMPSTCHA1_W::new(self)
    }
    #[doc = "Bit 2 - Compare Window A Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha2(&mut self) -> CMPSTCHA2_W<2> {
        CMPSTCHA2_W::new(self)
    }
    #[doc = "Bit 3 - Compare Window A Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha3(&mut self) -> CMPSTCHA3_W<3> {
        CMPSTCHA3_W::new(self)
    }
    #[doc = "Bit 4 - Compare Window A Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha4(&mut self) -> CMPSTCHA4_W<4> {
        CMPSTCHA4_W::new(self)
    }
    #[doc = "Bit 5 - Compare Window A Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha5(&mut self) -> CMPSTCHA5_W<5> {
        CMPSTCHA5_W::new(self)
    }
    #[doc = "Bit 6 - Compare Window A Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha6(&mut self) -> CMPSTCHA6_W<6> {
        CMPSTCHA6_W::new(self)
    }
    #[doc = "Bit 7 - Compare Window A Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha7(&mut self) -> CMPSTCHA7_W<7> {
        CMPSTCHA7_W::new(self)
    }
    #[doc = "Bit 8 - Compare Window A Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha8(&mut self) -> CMPSTCHA8_W<8> {
        CMPSTCHA8_W::new(self)
    }
    #[doc = "Bit 12 - Compare Window A Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha12(&mut self) -> CMPSTCHA12_W<12> {
        CMPSTCHA12_W::new(self)
    }
    #[doc = "Bit 13 - Compare Window A Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha13(&mut self) -> CMPSTCHA13_W<13> {
        CMPSTCHA13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Compare Function Window A Channel Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpsr0](index.html) module"]
pub struct ADCMPSR0_SPEC;
impl crate::RegisterSpec for ADCMPSR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcmpsr0::R](R) reader structure"]
impl crate::Readable for ADCMPSR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmpsr0::W](W) writer structure"]
impl crate::Writable for ADCMPSR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPSR0 to value 0"]
impl crate::Resettable for ADCMPSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
