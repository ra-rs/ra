#[doc = "Register `ELIPPR` reader"]
pub struct R(crate::R<ELIPPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ELIPPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ELIPPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ELIPPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ELIPPR` writer"]
pub struct W(crate::W<ELIPPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ELIPPR_SPEC>;
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
impl From<crate::W<ELIPPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ELIPPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CYCP0` reader - Pulse Output Timer 0 Rising Edge Detection Event Output Enable"]
pub type CYCP0_R = crate::BitReader<CYCP0_A>;
#[doc = "Pulse Output Timer 0 Rising Edge Detection Event Output Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYCP0_A {
    #[doc = "0: Rising edges of the signal from pulse output timer 0 are not conveyed to the ELC as event signals."]
    _0 = 0,
    #[doc = "1: Rising edges of the signal from pulse output timer 0 are conveyed to the ELC as event signals."]
    _1 = 1,
}
impl From<CYCP0_A> for bool {
    #[inline(always)]
    fn from(variant: CYCP0_A) -> Self {
        variant as u8 != 0
    }
}
impl CYCP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CYCP0_A {
        match self.bits {
            false => CYCP0_A::_0,
            true => CYCP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYCP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYCP0_A::_1
    }
}
#[doc = "Field `CYCP0` writer - Pulse Output Timer 0 Rising Edge Detection Event Output Enable"]
pub type CYCP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELIPPR_SPEC, CYCP0_A, O>;
impl<'a, const O: u8> CYCP0_W<'a, O> {
    #[doc = "Rising edges of the signal from pulse output timer 0 are not conveyed to the ELC as event signals."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CYCP0_A::_0)
    }
    #[doc = "Rising edges of the signal from pulse output timer 0 are conveyed to the ELC as event signals."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CYCP0_A::_1)
    }
}
#[doc = "Field `CYCP1` reader - Pulse Output Timer 1 Rising Edge Detection Event Output Enable"]
pub type CYCP1_R = crate::BitReader<CYCP1_A>;
#[doc = "Pulse Output Timer 1 Rising Edge Detection Event Output Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYCP1_A {
    #[doc = "0: Rising edges of the signal from pulse output timer 1 are not conveyed to the ELC as event signals."]
    _0 = 0,
    #[doc = "1: Rising edges of the signal from pulse output timer 1 are conveyed to the ELC as event signals."]
    _1 = 1,
}
impl From<CYCP1_A> for bool {
    #[inline(always)]
    fn from(variant: CYCP1_A) -> Self {
        variant as u8 != 0
    }
}
impl CYCP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CYCP1_A {
        match self.bits {
            false => CYCP1_A::_0,
            true => CYCP1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYCP1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYCP1_A::_1
    }
}
#[doc = "Field `CYCP1` writer - Pulse Output Timer 1 Rising Edge Detection Event Output Enable"]
pub type CYCP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELIPPR_SPEC, CYCP1_A, O>;
impl<'a, const O: u8> CYCP1_W<'a, O> {
    #[doc = "Rising edges of the signal from pulse output timer 1 are not conveyed to the ELC as event signals."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CYCP1_A::_0)
    }
    #[doc = "Rising edges of the signal from pulse output timer 1 are conveyed to the ELC as event signals."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CYCP1_A::_1)
    }
}
#[doc = "Field `CYCP2` reader - Pulse Output Timer 2 Rising Edge Detection Event Output Enable"]
pub type CYCP2_R = crate::BitReader<CYCP2_A>;
#[doc = "Pulse Output Timer 2 Rising Edge Detection Event Output Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYCP2_A {
    #[doc = "0: Rising edges of the signal from pulse output timer 2 are not conveyed to the ELC as event signals."]
    _0 = 0,
    #[doc = "1: Rising edges of the signal from pulse output timer 2 are conveyed to the ELC as event signals."]
    _1 = 1,
}
impl From<CYCP2_A> for bool {
    #[inline(always)]
    fn from(variant: CYCP2_A) -> Self {
        variant as u8 != 0
    }
}
impl CYCP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CYCP2_A {
        match self.bits {
            false => CYCP2_A::_0,
            true => CYCP2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYCP2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYCP2_A::_1
    }
}
#[doc = "Field `CYCP2` writer - Pulse Output Timer 2 Rising Edge Detection Event Output Enable"]
pub type CYCP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELIPPR_SPEC, CYCP2_A, O>;
impl<'a, const O: u8> CYCP2_W<'a, O> {
    #[doc = "Rising edges of the signal from pulse output timer 2 are not conveyed to the ELC as event signals."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CYCP2_A::_0)
    }
    #[doc = "Rising edges of the signal from pulse output timer 2 are conveyed to the ELC as event signals."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CYCP2_A::_1)
    }
}
#[doc = "Field `CYCP3` reader - Pulse Output Timer 3 Rising Edge Detection Event Output Enable"]
pub type CYCP3_R = crate::BitReader<CYCP3_A>;
#[doc = "Pulse Output Timer 3 Rising Edge Detection Event Output Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYCP3_A {
    #[doc = "0: Rising edges of the signal from pulse output timer 3 are not conveyed to the ELC as event signals."]
    _0 = 0,
    #[doc = "1: Rising edges of the signal from pulse output timer 3 are conveyed to the ELC as event signals."]
    _1 = 1,
}
impl From<CYCP3_A> for bool {
    #[inline(always)]
    fn from(variant: CYCP3_A) -> Self {
        variant as u8 != 0
    }
}
impl CYCP3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CYCP3_A {
        match self.bits {
            false => CYCP3_A::_0,
            true => CYCP3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYCP3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYCP3_A::_1
    }
}
#[doc = "Field `CYCP3` writer - Pulse Output Timer 3 Rising Edge Detection Event Output Enable"]
pub type CYCP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELIPPR_SPEC, CYCP3_A, O>;
impl<'a, const O: u8> CYCP3_W<'a, O> {
    #[doc = "Rising edges of the signal from pulse output timer 3 are not conveyed to the ELC as event signals."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CYCP3_A::_0)
    }
    #[doc = "Rising edges of the signal from pulse output timer 3 are conveyed to the ELC as event signals."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CYCP3_A::_1)
    }
}
#[doc = "Field `CYCP4` reader - Pulse Output Timer 4 Rising Edge Detection Event Output Enable"]
pub type CYCP4_R = crate::BitReader<CYCP4_A>;
#[doc = "Pulse Output Timer 4 Rising Edge Detection Event Output Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYCP4_A {
    #[doc = "0: Rising edges of the signal from pulse output timer 4 are not conveyed to the ELC as event signals."]
    _0 = 0,
    #[doc = "1: Rising edges of the signal from pulse output timer 4 are conveyed to the ELC as event signals."]
    _1 = 1,
}
impl From<CYCP4_A> for bool {
    #[inline(always)]
    fn from(variant: CYCP4_A) -> Self {
        variant as u8 != 0
    }
}
impl CYCP4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CYCP4_A {
        match self.bits {
            false => CYCP4_A::_0,
            true => CYCP4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYCP4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYCP4_A::_1
    }
}
#[doc = "Field `CYCP4` writer - Pulse Output Timer 4 Rising Edge Detection Event Output Enable"]
pub type CYCP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELIPPR_SPEC, CYCP4_A, O>;
impl<'a, const O: u8> CYCP4_W<'a, O> {
    #[doc = "Rising edges of the signal from pulse output timer 4 are not conveyed to the ELC as event signals."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CYCP4_A::_0)
    }
    #[doc = "Rising edges of the signal from pulse output timer 4 are conveyed to the ELC as event signals."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CYCP4_A::_1)
    }
}
#[doc = "Field `CYCP5` reader - Pulse Output Timer 5 Rising Edge Detection Event Output Enable"]
pub type CYCP5_R = crate::BitReader<CYCP5_A>;
#[doc = "Pulse Output Timer 5 Rising Edge Detection Event Output Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYCP5_A {
    #[doc = "0: Rising edges of the signal from pulse output timer 5 are not conveyed to the ELC as event signals."]
    _0 = 0,
    #[doc = "1: Rising edges of the signal from pulse output timer 5 are conveyed to the ELC as event signals."]
    _1 = 1,
}
impl From<CYCP5_A> for bool {
    #[inline(always)]
    fn from(variant: CYCP5_A) -> Self {
        variant as u8 != 0
    }
}
impl CYCP5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CYCP5_A {
        match self.bits {
            false => CYCP5_A::_0,
            true => CYCP5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYCP5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYCP5_A::_1
    }
}
#[doc = "Field `CYCP5` writer - Pulse Output Timer 5 Rising Edge Detection Event Output Enable"]
pub type CYCP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELIPPR_SPEC, CYCP5_A, O>;
impl<'a, const O: u8> CYCP5_W<'a, O> {
    #[doc = "Rising edges of the signal from pulse output timer 5 are not conveyed to the ELC as event signals."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CYCP5_A::_0)
    }
    #[doc = "Rising edges of the signal from pulse output timer 5 are conveyed to the ELC as event signals."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CYCP5_A::_1)
    }
}
#[doc = "Field `CYCN0` reader - Pulse Output Timer 0 Falling Edge Detection Event Output Enable"]
pub type CYCN0_R = crate::BitReader<CYCN0_A>;
#[doc = "Pulse Output Timer 0 Falling Edge Detection Event Output Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYCN0_A {
    #[doc = "0: Falling edges of the signal from pulse output timer 0 are not conveyed to the ELC as event signals."]
    _0 = 0,
    #[doc = "1: Falling edges of the signal from pulse output timer 0 are conveyed to the ELC as event signals."]
    _1 = 1,
}
impl From<CYCN0_A> for bool {
    #[inline(always)]
    fn from(variant: CYCN0_A) -> Self {
        variant as u8 != 0
    }
}
impl CYCN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CYCN0_A {
        match self.bits {
            false => CYCN0_A::_0,
            true => CYCN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYCN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYCN0_A::_1
    }
}
#[doc = "Field `CYCN0` writer - Pulse Output Timer 0 Falling Edge Detection Event Output Enable"]
pub type CYCN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELIPPR_SPEC, CYCN0_A, O>;
impl<'a, const O: u8> CYCN0_W<'a, O> {
    #[doc = "Falling edges of the signal from pulse output timer 0 are not conveyed to the ELC as event signals."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CYCN0_A::_0)
    }
    #[doc = "Falling edges of the signal from pulse output timer 0 are conveyed to the ELC as event signals."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CYCN0_A::_1)
    }
}
#[doc = "Field `CYCN1` reader - Pulse Output Timer 1 Falling Edge Detection Event Output Enable"]
pub type CYCN1_R = crate::BitReader<CYCN1_A>;
#[doc = "Pulse Output Timer 1 Falling Edge Detection Event Output Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYCN1_A {
    #[doc = "0: Falling edges of the signal from pulse output timer 1 are not conveyed to the ELC as event signals."]
    _0 = 0,
    #[doc = "1: Falling edges of the signal from pulse output timer 1 are conveyed to the ELC as event signals."]
    _1 = 1,
}
impl From<CYCN1_A> for bool {
    #[inline(always)]
    fn from(variant: CYCN1_A) -> Self {
        variant as u8 != 0
    }
}
impl CYCN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CYCN1_A {
        match self.bits {
            false => CYCN1_A::_0,
            true => CYCN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYCN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYCN1_A::_1
    }
}
#[doc = "Field `CYCN1` writer - Pulse Output Timer 1 Falling Edge Detection Event Output Enable"]
pub type CYCN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELIPPR_SPEC, CYCN1_A, O>;
impl<'a, const O: u8> CYCN1_W<'a, O> {
    #[doc = "Falling edges of the signal from pulse output timer 1 are not conveyed to the ELC as event signals."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CYCN1_A::_0)
    }
    #[doc = "Falling edges of the signal from pulse output timer 1 are conveyed to the ELC as event signals."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CYCN1_A::_1)
    }
}
#[doc = "Field `CYCN2` reader - Pulse Output Timer 2 Falling Edge Detection Event Output Enable"]
pub type CYCN2_R = crate::BitReader<CYCN2_A>;
#[doc = "Pulse Output Timer 2 Falling Edge Detection Event Output Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYCN2_A {
    #[doc = "0: Falling edges of the signal from pulse output timer 2 are not conveyed to the ELC as event signals."]
    _0 = 0,
    #[doc = "1: Falling edges of the signal from pulse output timer 2 are conveyed to the ELC as event signals."]
    _1 = 1,
}
impl From<CYCN2_A> for bool {
    #[inline(always)]
    fn from(variant: CYCN2_A) -> Self {
        variant as u8 != 0
    }
}
impl CYCN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CYCN2_A {
        match self.bits {
            false => CYCN2_A::_0,
            true => CYCN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYCN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYCN2_A::_1
    }
}
#[doc = "Field `CYCN2` writer - Pulse Output Timer 2 Falling Edge Detection Event Output Enable"]
pub type CYCN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELIPPR_SPEC, CYCN2_A, O>;
impl<'a, const O: u8> CYCN2_W<'a, O> {
    #[doc = "Falling edges of the signal from pulse output timer 2 are not conveyed to the ELC as event signals."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CYCN2_A::_0)
    }
    #[doc = "Falling edges of the signal from pulse output timer 2 are conveyed to the ELC as event signals."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CYCN2_A::_1)
    }
}
#[doc = "Field `CYCN3` reader - Pulse Output Timer 3 Falling Edge Detection Event Output Enable"]
pub type CYCN3_R = crate::BitReader<CYCN3_A>;
#[doc = "Pulse Output Timer 3 Falling Edge Detection Event Output Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYCN3_A {
    #[doc = "0: Falling edges of the signal from pulse output timer 3 are not conveyed to the ELC as event signals."]
    _0 = 0,
    #[doc = "1: Falling edges of the signal from pulse output timer 3 are conveyed to the ELC as event signals."]
    _1 = 1,
}
impl From<CYCN3_A> for bool {
    #[inline(always)]
    fn from(variant: CYCN3_A) -> Self {
        variant as u8 != 0
    }
}
impl CYCN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CYCN3_A {
        match self.bits {
            false => CYCN3_A::_0,
            true => CYCN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYCN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYCN3_A::_1
    }
}
#[doc = "Field `CYCN3` writer - Pulse Output Timer 3 Falling Edge Detection Event Output Enable"]
pub type CYCN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELIPPR_SPEC, CYCN3_A, O>;
impl<'a, const O: u8> CYCN3_W<'a, O> {
    #[doc = "Falling edges of the signal from pulse output timer 3 are not conveyed to the ELC as event signals."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CYCN3_A::_0)
    }
    #[doc = "Falling edges of the signal from pulse output timer 3 are conveyed to the ELC as event signals."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CYCN3_A::_1)
    }
}
#[doc = "Field `CYCN4` reader - Pulse Output Timer 4 Falling Edge Detection Event Output Enable"]
pub type CYCN4_R = crate::BitReader<CYCN4_A>;
#[doc = "Pulse Output Timer 4 Falling Edge Detection Event Output Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYCN4_A {
    #[doc = "0: Falling edges of the signal from pulse output timer 4 are not conveyed to the ELC as event signals."]
    _0 = 0,
    #[doc = "1: Falling edges of the signal from pulse output timer 4 are conveyed to the ELC as event signals."]
    _1 = 1,
}
impl From<CYCN4_A> for bool {
    #[inline(always)]
    fn from(variant: CYCN4_A) -> Self {
        variant as u8 != 0
    }
}
impl CYCN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CYCN4_A {
        match self.bits {
            false => CYCN4_A::_0,
            true => CYCN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYCN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYCN4_A::_1
    }
}
#[doc = "Field `CYCN4` writer - Pulse Output Timer 4 Falling Edge Detection Event Output Enable"]
pub type CYCN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELIPPR_SPEC, CYCN4_A, O>;
impl<'a, const O: u8> CYCN4_W<'a, O> {
    #[doc = "Falling edges of the signal from pulse output timer 4 are not conveyed to the ELC as event signals."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CYCN4_A::_0)
    }
    #[doc = "Falling edges of the signal from pulse output timer 4 are conveyed to the ELC as event signals."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CYCN4_A::_1)
    }
}
#[doc = "Field `CYCN5` reader - Pulse Output Timer 5 Falling Edge Detection Event Output Enable"]
pub type CYCN5_R = crate::BitReader<CYCN5_A>;
#[doc = "Pulse Output Timer 5 Falling Edge Detection Event Output Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYCN5_A {
    #[doc = "0: Falling edges of the signal from pulse output timer 5 are not conveyed to the ELC as event signals."]
    _0 = 0,
    #[doc = "1: Falling edges of the signal from pulse output timer 5 are conveyed to the ELC as event signals."]
    _1 = 1,
}
impl From<CYCN5_A> for bool {
    #[inline(always)]
    fn from(variant: CYCN5_A) -> Self {
        variant as u8 != 0
    }
}
impl CYCN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CYCN5_A {
        match self.bits {
            false => CYCN5_A::_0,
            true => CYCN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYCN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYCN5_A::_1
    }
}
#[doc = "Field `CYCN5` writer - Pulse Output Timer 5 Falling Edge Detection Event Output Enable"]
pub type CYCN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELIPPR_SPEC, CYCN5_A, O>;
impl<'a, const O: u8> CYCN5_W<'a, O> {
    #[doc = "Falling edges of the signal from pulse output timer 5 are not conveyed to the ELC as event signals."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CYCN5_A::_0)
    }
    #[doc = "Falling edges of the signal from pulse output timer 5 are conveyed to the ELC as event signals."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CYCN5_A::_1)
    }
}
#[doc = "Field `PLSP` reader - Pulse Output Timer Rising Edge Detection IPLS Interrupt Request Permission"]
pub type PLSP_R = crate::BitReader<PLSP_A>;
#[doc = "Pulse Output Timer Rising Edge Detection IPLS Interrupt Request Permission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLSP_A {
    #[doc = "0: Prohibits IPLS interrupt requests due to rising edges of signals from the selected pulse output timer."]
    _0 = 0,
    #[doc = "1: Permits IPLS interrupt requests due to rising edges of signals from the selected pulse output timer."]
    _1 = 1,
}
impl From<PLSP_A> for bool {
    #[inline(always)]
    fn from(variant: PLSP_A) -> Self {
        variant as u8 != 0
    }
}
impl PLSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLSP_A {
        match self.bits {
            false => PLSP_A::_0,
            true => PLSP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLSP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLSP_A::_1
    }
}
#[doc = "Field `PLSP` writer - Pulse Output Timer Rising Edge Detection IPLS Interrupt Request Permission"]
pub type PLSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELIPPR_SPEC, PLSP_A, O>;
impl<'a, const O: u8> PLSP_W<'a, O> {
    #[doc = "Prohibits IPLS interrupt requests due to rising edges of signals from the selected pulse output timer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLSP_A::_0)
    }
    #[doc = "Permits IPLS interrupt requests due to rising edges of signals from the selected pulse output timer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLSP_A::_1)
    }
}
#[doc = "Field `PLSN` reader - Pulse Output Timer Falling Edge Detection IPLS Interrupt Request Permission"]
pub type PLSN_R = crate::BitReader<PLSN_A>;
#[doc = "Pulse Output Timer Falling Edge Detection IPLS Interrupt Request Permission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLSN_A {
    #[doc = "0: Prohibits IPLS interrupt requests due to falling edges of signals from the selected pulse output timer."]
    _0 = 0,
    #[doc = "1: Permits IPLS interrupt requests due to falling edges of signals from the selected pulse output timer."]
    _1 = 1,
}
impl From<PLSN_A> for bool {
    #[inline(always)]
    fn from(variant: PLSN_A) -> Self {
        variant as u8 != 0
    }
}
impl PLSN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLSN_A {
        match self.bits {
            false => PLSN_A::_0,
            true => PLSN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLSN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLSN_A::_1
    }
}
#[doc = "Field `PLSN` writer - Pulse Output Timer Falling Edge Detection IPLS Interrupt Request Permission"]
pub type PLSN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELIPPR_SPEC, PLSN_A, O>;
impl<'a, const O: u8> PLSN_W<'a, O> {
    #[doc = "Prohibits IPLS interrupt requests due to falling edges of signals from the selected pulse output timer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLSN_A::_0)
    }
    #[doc = "Permits IPLS interrupt requests due to falling edges of signals from the selected pulse output timer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLSN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Pulse Output Timer 0 Rising Edge Detection Event Output Enable"]
    #[inline(always)]
    pub fn cycp0(&self) -> CYCP0_R {
        CYCP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pulse Output Timer 1 Rising Edge Detection Event Output Enable"]
    #[inline(always)]
    pub fn cycp1(&self) -> CYCP1_R {
        CYCP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pulse Output Timer 2 Rising Edge Detection Event Output Enable"]
    #[inline(always)]
    pub fn cycp2(&self) -> CYCP2_R {
        CYCP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pulse Output Timer 3 Rising Edge Detection Event Output Enable"]
    #[inline(always)]
    pub fn cycp3(&self) -> CYCP3_R {
        CYCP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pulse Output Timer 4 Rising Edge Detection Event Output Enable"]
    #[inline(always)]
    pub fn cycp4(&self) -> CYCP4_R {
        CYCP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pulse Output Timer 5 Rising Edge Detection Event Output Enable"]
    #[inline(always)]
    pub fn cycp5(&self) -> CYCP5_R {
        CYCP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Pulse Output Timer 0 Falling Edge Detection Event Output Enable"]
    #[inline(always)]
    pub fn cycn0(&self) -> CYCN0_R {
        CYCN0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pulse Output Timer 1 Falling Edge Detection Event Output Enable"]
    #[inline(always)]
    pub fn cycn1(&self) -> CYCN1_R {
        CYCN1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pulse Output Timer 2 Falling Edge Detection Event Output Enable"]
    #[inline(always)]
    pub fn cycn2(&self) -> CYCN2_R {
        CYCN2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pulse Output Timer 3 Falling Edge Detection Event Output Enable"]
    #[inline(always)]
    pub fn cycn3(&self) -> CYCN3_R {
        CYCN3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pulse Output Timer 4 Falling Edge Detection Event Output Enable"]
    #[inline(always)]
    pub fn cycn4(&self) -> CYCN4_R {
        CYCN4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pulse Output Timer 5 Falling Edge Detection Event Output Enable"]
    #[inline(always)]
    pub fn cycn5(&self) -> CYCN5_R {
        CYCN5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Pulse Output Timer Rising Edge Detection IPLS Interrupt Request Permission"]
    #[inline(always)]
    pub fn plsp(&self) -> PLSP_R {
        PLSP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Pulse Output Timer Falling Edge Detection IPLS Interrupt Request Permission"]
    #[inline(always)]
    pub fn plsn(&self) -> PLSN_R {
        PLSN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pulse Output Timer 0 Rising Edge Detection Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cycp0(&mut self) -> CYCP0_W<0> {
        CYCP0_W::new(self)
    }
    #[doc = "Bit 1 - Pulse Output Timer 1 Rising Edge Detection Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cycp1(&mut self) -> CYCP1_W<1> {
        CYCP1_W::new(self)
    }
    #[doc = "Bit 2 - Pulse Output Timer 2 Rising Edge Detection Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cycp2(&mut self) -> CYCP2_W<2> {
        CYCP2_W::new(self)
    }
    #[doc = "Bit 3 - Pulse Output Timer 3 Rising Edge Detection Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cycp3(&mut self) -> CYCP3_W<3> {
        CYCP3_W::new(self)
    }
    #[doc = "Bit 4 - Pulse Output Timer 4 Rising Edge Detection Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cycp4(&mut self) -> CYCP4_W<4> {
        CYCP4_W::new(self)
    }
    #[doc = "Bit 5 - Pulse Output Timer 5 Rising Edge Detection Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cycp5(&mut self) -> CYCP5_W<5> {
        CYCP5_W::new(self)
    }
    #[doc = "Bit 8 - Pulse Output Timer 0 Falling Edge Detection Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cycn0(&mut self) -> CYCN0_W<8> {
        CYCN0_W::new(self)
    }
    #[doc = "Bit 9 - Pulse Output Timer 1 Falling Edge Detection Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cycn1(&mut self) -> CYCN1_W<9> {
        CYCN1_W::new(self)
    }
    #[doc = "Bit 10 - Pulse Output Timer 2 Falling Edge Detection Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cycn2(&mut self) -> CYCN2_W<10> {
        CYCN2_W::new(self)
    }
    #[doc = "Bit 11 - Pulse Output Timer 3 Falling Edge Detection Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cycn3(&mut self) -> CYCN3_W<11> {
        CYCN3_W::new(self)
    }
    #[doc = "Bit 12 - Pulse Output Timer 4 Falling Edge Detection Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cycn4(&mut self) -> CYCN4_W<12> {
        CYCN4_W::new(self)
    }
    #[doc = "Bit 13 - Pulse Output Timer 5 Falling Edge Detection Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cycn5(&mut self) -> CYCN5_W<13> {
        CYCN5_W::new(self)
    }
    #[doc = "Bit 16 - Pulse Output Timer Rising Edge Detection IPLS Interrupt Request Permission"]
    #[inline(always)]
    #[must_use]
    pub fn plsp(&mut self) -> PLSP_W<16> {
        PLSP_W::new(self)
    }
    #[doc = "Bit 24 - Pulse Output Timer Falling Edge Detection IPLS Interrupt Request Permission"]
    #[inline(always)]
    #[must_use]
    pub fn plsn(&mut self) -> PLSN_W<24> {
        PLSN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ELC Output/IPLS Interrupt Request Permission Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [elippr](index.html) module"]
pub struct ELIPPR_SPEC;
impl crate::RegisterSpec for ELIPPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [elippr::R](R) reader structure"]
impl crate::Readable for ELIPPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [elippr::W](W) writer structure"]
impl crate::Writable for ELIPPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ELIPPR to value 0x3f3f"]
impl crate::Resettable for ELIPPR_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f3f;
}
