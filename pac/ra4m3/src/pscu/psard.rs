#[doc = "Register `PSARD` reader"]
pub struct R(crate::R<PSARD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSARD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSARD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSARD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSARD` writer"]
pub struct W(crate::W<PSARD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSARD_SPEC>;
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
impl From<crate::W<PSARD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSARD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSARD0` reader - AGT3 and the MSTPCRD.MSTPD0 bit security attribution"]
pub type PSARD0_R = crate::BitReader<PSARD0_A>;
#[doc = "AGT3 and the MSTPCRD.MSTPD0 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARD0_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARD0_A> for bool {
    #[inline(always)]
    fn from(variant: PSARD0_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARD0_A {
        match self.bits {
            false => PSARD0_A::_0,
            true => PSARD0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARD0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARD0_A::_1
    }
}
#[doc = "Field `PSARD0` writer - AGT3 and the MSTPCRD.MSTPD0 bit security attribution"]
pub type PSARD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARD_SPEC, PSARD0_A, O>;
impl<'a, const O: u8> PSARD0_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARD0_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARD0_A::_1)
    }
}
#[doc = "Field `PSARD1` reader - AGT2 and the MSTPCRD.MSTPD1 bit security attribution"]
pub type PSARD1_R = crate::BitReader<PSARD1_A>;
#[doc = "AGT2 and the MSTPCRD.MSTPD1 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARD1_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARD1_A> for bool {
    #[inline(always)]
    fn from(variant: PSARD1_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARD1_A {
        match self.bits {
            false => PSARD1_A::_0,
            true => PSARD1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARD1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARD1_A::_1
    }
}
#[doc = "Field `PSARD1` writer - AGT2 and the MSTPCRD.MSTPD1 bit security attribution"]
pub type PSARD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARD_SPEC, PSARD1_A, O>;
impl<'a, const O: u8> PSARD1_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARD1_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARD1_A::_1)
    }
}
#[doc = "Field `PSARD2` reader - AGT1 and the MSTPCRD.MSTPD2 bit security attribution"]
pub type PSARD2_R = crate::BitReader<PSARD2_A>;
#[doc = "AGT1 and the MSTPCRD.MSTPD2 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARD2_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARD2_A> for bool {
    #[inline(always)]
    fn from(variant: PSARD2_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARD2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARD2_A {
        match self.bits {
            false => PSARD2_A::_0,
            true => PSARD2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARD2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARD2_A::_1
    }
}
#[doc = "Field `PSARD2` writer - AGT1 and the MSTPCRD.MSTPD2 bit security attribution"]
pub type PSARD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARD_SPEC, PSARD2_A, O>;
impl<'a, const O: u8> PSARD2_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARD2_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARD2_A::_1)
    }
}
#[doc = "Field `PSARD3` reader - AGT0 and the MSTPCRD.MSTPD3 bit security attribution"]
pub type PSARD3_R = crate::BitReader<PSARD3_A>;
#[doc = "AGT0 and the MSTPCRD.MSTPD3 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARD3_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARD3_A> for bool {
    #[inline(always)]
    fn from(variant: PSARD3_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARD3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARD3_A {
        match self.bits {
            false => PSARD3_A::_0,
            true => PSARD3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARD3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARD3_A::_1
    }
}
#[doc = "Field `PSARD3` writer - AGT0 and the MSTPCRD.MSTPD3 bit security attribution"]
pub type PSARD3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARD_SPEC, PSARD3_A, O>;
impl<'a, const O: u8> PSARD3_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARD3_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARD3_A::_1)
    }
}
#[doc = "Field `PSARD11` reader - POEG Group D and the MSTPCRD.MSTPD11 bit security attribution"]
pub type PSARD11_R = crate::BitReader<PSARD11_A>;
#[doc = "POEG Group D and the MSTPCRD.MSTPD11 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARD11_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARD11_A> for bool {
    #[inline(always)]
    fn from(variant: PSARD11_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARD11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARD11_A {
        match self.bits {
            false => PSARD11_A::_0,
            true => PSARD11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARD11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARD11_A::_1
    }
}
#[doc = "Field `PSARD11` writer - POEG Group D and the MSTPCRD.MSTPD11 bit security attribution"]
pub type PSARD11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARD_SPEC, PSARD11_A, O>;
impl<'a, const O: u8> PSARD11_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARD11_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARD11_A::_1)
    }
}
#[doc = "Field `PSARD12` reader - POEG Group C and the MSTPCRD.MSTPD12 bit security attribution"]
pub type PSARD12_R = crate::BitReader<PSARD12_A>;
#[doc = "POEG Group C and the MSTPCRD.MSTPD12 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARD12_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARD12_A> for bool {
    #[inline(always)]
    fn from(variant: PSARD12_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARD12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARD12_A {
        match self.bits {
            false => PSARD12_A::_0,
            true => PSARD12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARD12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARD12_A::_1
    }
}
#[doc = "Field `PSARD12` writer - POEG Group C and the MSTPCRD.MSTPD12 bit security attribution"]
pub type PSARD12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARD_SPEC, PSARD12_A, O>;
impl<'a, const O: u8> PSARD12_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARD12_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARD12_A::_1)
    }
}
#[doc = "Field `PSARD13` reader - POEG Group B and the MSTPCRD.MSTPD13 bit security attribution"]
pub type PSARD13_R = crate::BitReader<PSARD13_A>;
#[doc = "POEG Group B and the MSTPCRD.MSTPD13 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARD13_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARD13_A> for bool {
    #[inline(always)]
    fn from(variant: PSARD13_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARD13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARD13_A {
        match self.bits {
            false => PSARD13_A::_0,
            true => PSARD13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARD13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARD13_A::_1
    }
}
#[doc = "Field `PSARD13` writer - POEG Group B and the MSTPCRD.MSTPD13 bit security attribution"]
pub type PSARD13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARD_SPEC, PSARD13_A, O>;
impl<'a, const O: u8> PSARD13_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARD13_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARD13_A::_1)
    }
}
#[doc = "Field `PSARD14` reader - POEG Group A and the MSTPCRD.MSTPD14 bit security attribution"]
pub type PSARD14_R = crate::BitReader<PSARD14_A>;
#[doc = "POEG Group A and the MSTPCRD.MSTPD14 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARD14_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARD14_A> for bool {
    #[inline(always)]
    fn from(variant: PSARD14_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARD14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARD14_A {
        match self.bits {
            false => PSARD14_A::_0,
            true => PSARD14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARD14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARD14_A::_1
    }
}
#[doc = "Field `PSARD14` writer - POEG Group A and the MSTPCRD.MSTPD14 bit security attribution"]
pub type PSARD14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARD_SPEC, PSARD14_A, O>;
impl<'a, const O: u8> PSARD14_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARD14_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARD14_A::_1)
    }
}
#[doc = "Field `PSARD15` reader - ADC121 and the MSTPCRD.MSTPD15 bit security attribution"]
pub type PSARD15_R = crate::BitReader<PSARD15_A>;
#[doc = "ADC121 and the MSTPCRD.MSTPD15 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARD15_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARD15_A> for bool {
    #[inline(always)]
    fn from(variant: PSARD15_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARD15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARD15_A {
        match self.bits {
            false => PSARD15_A::_0,
            true => PSARD15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARD15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARD15_A::_1
    }
}
#[doc = "Field `PSARD15` writer - ADC121 and the MSTPCRD.MSTPD15 bit security attribution"]
pub type PSARD15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARD_SPEC, PSARD15_A, O>;
impl<'a, const O: u8> PSARD15_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARD15_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARD15_A::_1)
    }
}
#[doc = "Field `PSARD16` reader - ADC120 and the MSTPCRD.MSTPD16 bit security attribution"]
pub type PSARD16_R = crate::BitReader<PSARD16_A>;
#[doc = "ADC120 and the MSTPCRD.MSTPD16 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARD16_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARD16_A> for bool {
    #[inline(always)]
    fn from(variant: PSARD16_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARD16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARD16_A {
        match self.bits {
            false => PSARD16_A::_0,
            true => PSARD16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARD16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARD16_A::_1
    }
}
#[doc = "Field `PSARD16` writer - ADC120 and the MSTPCRD.MSTPD16 bit security attribution"]
pub type PSARD16_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARD_SPEC, PSARD16_A, O>;
impl<'a, const O: u8> PSARD16_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARD16_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARD16_A::_1)
    }
}
#[doc = "Field `PSARD20` reader - DAC12 and the MSTPCRD.MSTPD20 bit security attribution"]
pub type PSARD20_R = crate::BitReader<PSARD20_A>;
#[doc = "DAC12 and the MSTPCRD.MSTPD20 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARD20_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARD20_A> for bool {
    #[inline(always)]
    fn from(variant: PSARD20_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARD20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARD20_A {
        match self.bits {
            false => PSARD20_A::_0,
            true => PSARD20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARD20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARD20_A::_1
    }
}
#[doc = "Field `PSARD20` writer - DAC12 and the MSTPCRD.MSTPD20 bit security attribution"]
pub type PSARD20_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARD_SPEC, PSARD20_A, O>;
impl<'a, const O: u8> PSARD20_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARD20_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARD20_A::_1)
    }
}
#[doc = "Field `PSARD22` reader - TSN and the MSTPCRD.MSTPD22 bit security attribution"]
pub type PSARD22_R = crate::BitReader<PSARD22_A>;
#[doc = "TSN and the MSTPCRD.MSTPD22 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARD22_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARD22_A> for bool {
    #[inline(always)]
    fn from(variant: PSARD22_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARD22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARD22_A {
        match self.bits {
            false => PSARD22_A::_0,
            true => PSARD22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARD22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARD22_A::_1
    }
}
#[doc = "Field `PSARD22` writer - TSN and the MSTPCRD.MSTPD22 bit security attribution"]
pub type PSARD22_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARD_SPEC, PSARD22_A, O>;
impl<'a, const O: u8> PSARD22_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARD22_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARD22_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AGT3 and the MSTPCRD.MSTPD0 bit security attribution"]
    #[inline(always)]
    pub fn psard0(&self) -> PSARD0_R {
        PSARD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AGT2 and the MSTPCRD.MSTPD1 bit security attribution"]
    #[inline(always)]
    pub fn psard1(&self) -> PSARD1_R {
        PSARD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AGT1 and the MSTPCRD.MSTPD2 bit security attribution"]
    #[inline(always)]
    pub fn psard2(&self) -> PSARD2_R {
        PSARD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AGT0 and the MSTPCRD.MSTPD3 bit security attribution"]
    #[inline(always)]
    pub fn psard3(&self) -> PSARD3_R {
        PSARD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 11 - POEG Group D and the MSTPCRD.MSTPD11 bit security attribution"]
    #[inline(always)]
    pub fn psard11(&self) -> PSARD11_R {
        PSARD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - POEG Group C and the MSTPCRD.MSTPD12 bit security attribution"]
    #[inline(always)]
    pub fn psard12(&self) -> PSARD12_R {
        PSARD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - POEG Group B and the MSTPCRD.MSTPD13 bit security attribution"]
    #[inline(always)]
    pub fn psard13(&self) -> PSARD13_R {
        PSARD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - POEG Group A and the MSTPCRD.MSTPD14 bit security attribution"]
    #[inline(always)]
    pub fn psard14(&self) -> PSARD14_R {
        PSARD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC121 and the MSTPCRD.MSTPD15 bit security attribution"]
    #[inline(always)]
    pub fn psard15(&self) -> PSARD15_R {
        PSARD15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC120 and the MSTPCRD.MSTPD16 bit security attribution"]
    #[inline(always)]
    pub fn psard16(&self) -> PSARD16_R {
        PSARD16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - DAC12 and the MSTPCRD.MSTPD20 bit security attribution"]
    #[inline(always)]
    pub fn psard20(&self) -> PSARD20_R {
        PSARD20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - TSN and the MSTPCRD.MSTPD22 bit security attribution"]
    #[inline(always)]
    pub fn psard22(&self) -> PSARD22_R {
        PSARD22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AGT3 and the MSTPCRD.MSTPD0 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psard0(&mut self) -> PSARD0_W<0> {
        PSARD0_W::new(self)
    }
    #[doc = "Bit 1 - AGT2 and the MSTPCRD.MSTPD1 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psard1(&mut self) -> PSARD1_W<1> {
        PSARD1_W::new(self)
    }
    #[doc = "Bit 2 - AGT1 and the MSTPCRD.MSTPD2 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psard2(&mut self) -> PSARD2_W<2> {
        PSARD2_W::new(self)
    }
    #[doc = "Bit 3 - AGT0 and the MSTPCRD.MSTPD3 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psard3(&mut self) -> PSARD3_W<3> {
        PSARD3_W::new(self)
    }
    #[doc = "Bit 11 - POEG Group D and the MSTPCRD.MSTPD11 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psard11(&mut self) -> PSARD11_W<11> {
        PSARD11_W::new(self)
    }
    #[doc = "Bit 12 - POEG Group C and the MSTPCRD.MSTPD12 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psard12(&mut self) -> PSARD12_W<12> {
        PSARD12_W::new(self)
    }
    #[doc = "Bit 13 - POEG Group B and the MSTPCRD.MSTPD13 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psard13(&mut self) -> PSARD13_W<13> {
        PSARD13_W::new(self)
    }
    #[doc = "Bit 14 - POEG Group A and the MSTPCRD.MSTPD14 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psard14(&mut self) -> PSARD14_W<14> {
        PSARD14_W::new(self)
    }
    #[doc = "Bit 15 - ADC121 and the MSTPCRD.MSTPD15 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psard15(&mut self) -> PSARD15_W<15> {
        PSARD15_W::new(self)
    }
    #[doc = "Bit 16 - ADC120 and the MSTPCRD.MSTPD16 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psard16(&mut self) -> PSARD16_W<16> {
        PSARD16_W::new(self)
    }
    #[doc = "Bit 20 - DAC12 and the MSTPCRD.MSTPD20 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psard20(&mut self) -> PSARD20_W<20> {
        PSARD20_W::new(self)
    }
    #[doc = "Bit 22 - TSN and the MSTPCRD.MSTPD22 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psard22(&mut self) -> PSARD22_W<22> {
        PSARD22_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Security Attribution Register D\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psard](index.html) module"]
pub struct PSARD_SPEC;
impl crate::RegisterSpec for PSARD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psard::R](R) reader structure"]
impl crate::Readable for PSARD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psard::W](W) writer structure"]
impl crate::Writable for PSARD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSARD to value 0xffff_ffff"]
impl crate::Resettable for PSARD_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
