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
#[doc = "Field `PSARD16` reader - ADC and the MSTPCRD.MSTPD16 bit security attribution"]
pub type PSARD16_R = crate::BitReader<PSARD16_A>;
#[doc = "ADC and the MSTPCRD.MSTPD16 bit security attribution\n\nValue on reset: 1"]
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
#[doc = "Field `PSARD16` writer - ADC and the MSTPCRD.MSTPD16 bit security attribution"]
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
#[doc = "Field `PSARD19` reader - DAC12 unit1 and the MSTPCRD.MSTPD19 bit security attribution"]
pub type PSARD19_R = crate::BitReader<PSARD19_A>;
#[doc = "DAC12 unit1 and the MSTPCRD.MSTPD19 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARD19_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARD19_A> for bool {
    #[inline(always)]
    fn from(variant: PSARD19_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARD19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARD19_A {
        match self.bits {
            false => PSARD19_A::_0,
            true => PSARD19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARD19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARD19_A::_1
    }
}
#[doc = "Field `PSARD19` writer - DAC12 unit1 and the MSTPCRD.MSTPD19 bit security attribution"]
pub type PSARD19_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARD_SPEC, PSARD19_A, O>;
impl<'a, const O: u8> PSARD19_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARD19_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARD19_A::_1)
    }
}
#[doc = "Field `PSARD20` reader - DAC12 unit0 and the MSTPCRD.MSTPD20 bit security attribution"]
pub type PSARD20_R = crate::BitReader<PSARD20_A>;
#[doc = "DAC12 unit0 and the MSTPCRD.MSTPD20 bit security attribution\n\nValue on reset: 1"]
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
#[doc = "Field `PSARD20` writer - DAC12 unit0 and the MSTPCRD.MSTPD20 bit security attribution"]
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
#[doc = "Field `PSARD25` reader - ACMPHS3 and the MSTPCRD.MSTPD25 bit security attribution"]
pub type PSARD25_R = crate::BitReader<PSARD25_A>;
#[doc = "ACMPHS3 and the MSTPCRD.MSTPD25 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARD25_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARD25_A> for bool {
    #[inline(always)]
    fn from(variant: PSARD25_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARD25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARD25_A {
        match self.bits {
            false => PSARD25_A::_0,
            true => PSARD25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARD25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARD25_A::_1
    }
}
#[doc = "Field `PSARD25` writer - ACMPHS3 and the MSTPCRD.MSTPD25 bit security attribution"]
pub type PSARD25_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARD_SPEC, PSARD25_A, O>;
impl<'a, const O: u8> PSARD25_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARD25_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARD25_A::_1)
    }
}
#[doc = "Field `PSARD26` reader - ACMPHS2 and the MSTPCRD.MSTPD26 bit security attribution"]
pub type PSARD26_R = crate::BitReader<PSARD26_A>;
#[doc = "ACMPHS2 and the MSTPCRD.MSTPD26 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARD26_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARD26_A> for bool {
    #[inline(always)]
    fn from(variant: PSARD26_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARD26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARD26_A {
        match self.bits {
            false => PSARD26_A::_0,
            true => PSARD26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARD26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARD26_A::_1
    }
}
#[doc = "Field `PSARD26` writer - ACMPHS2 and the MSTPCRD.MSTPD26 bit security attribution"]
pub type PSARD26_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARD_SPEC, PSARD26_A, O>;
impl<'a, const O: u8> PSARD26_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARD26_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARD26_A::_1)
    }
}
#[doc = "Field `PSARD27` reader - ACMPHS1 and the MSTPCRD.MSTPD27 bit security attribution"]
pub type PSARD27_R = crate::BitReader<PSARD27_A>;
#[doc = "ACMPHS1 and the MSTPCRD.MSTPD27 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARD27_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARD27_A> for bool {
    #[inline(always)]
    fn from(variant: PSARD27_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARD27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARD27_A {
        match self.bits {
            false => PSARD27_A::_0,
            true => PSARD27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARD27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARD27_A::_1
    }
}
#[doc = "Field `PSARD27` writer - ACMPHS1 and the MSTPCRD.MSTPD27 bit security attribution"]
pub type PSARD27_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARD_SPEC, PSARD27_A, O>;
impl<'a, const O: u8> PSARD27_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARD27_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARD27_A::_1)
    }
}
#[doc = "Field `PSARD28` reader - ACMPHS0 and the MSTPCRD.MSTPD28 bit security attribution"]
pub type PSARD28_R = crate::BitReader<PSARD28_A>;
#[doc = "ACMPHS0 and the MSTPCRD.MSTPD28 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARD28_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARD28_A> for bool {
    #[inline(always)]
    fn from(variant: PSARD28_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARD28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARD28_A {
        match self.bits {
            false => PSARD28_A::_0,
            true => PSARD28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARD28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARD28_A::_1
    }
}
#[doc = "Field `PSARD28` writer - ACMPHS0 and the MSTPCRD.MSTPD28 bit security attribution"]
pub type PSARD28_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARD_SPEC, PSARD28_A, O>;
impl<'a, const O: u8> PSARD28_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARD28_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARD28_A::_1)
    }
}
impl R {
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
    #[doc = "Bit 16 - ADC and the MSTPCRD.MSTPD16 bit security attribution"]
    #[inline(always)]
    pub fn psard16(&self) -> PSARD16_R {
        PSARD16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - DAC12 unit1 and the MSTPCRD.MSTPD19 bit security attribution"]
    #[inline(always)]
    pub fn psard19(&self) -> PSARD19_R {
        PSARD19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DAC12 unit0 and the MSTPCRD.MSTPD20 bit security attribution"]
    #[inline(always)]
    pub fn psard20(&self) -> PSARD20_R {
        PSARD20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - TSN and the MSTPCRD.MSTPD22 bit security attribution"]
    #[inline(always)]
    pub fn psard22(&self) -> PSARD22_R {
        PSARD22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - ACMPHS3 and the MSTPCRD.MSTPD25 bit security attribution"]
    #[inline(always)]
    pub fn psard25(&self) -> PSARD25_R {
        PSARD25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - ACMPHS2 and the MSTPCRD.MSTPD26 bit security attribution"]
    #[inline(always)]
    pub fn psard26(&self) -> PSARD26_R {
        PSARD26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - ACMPHS1 and the MSTPCRD.MSTPD27 bit security attribution"]
    #[inline(always)]
    pub fn psard27(&self) -> PSARD27_R {
        PSARD27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - ACMPHS0 and the MSTPCRD.MSTPD28 bit security attribution"]
    #[inline(always)]
    pub fn psard28(&self) -> PSARD28_R {
        PSARD28_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
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
    #[doc = "Bit 16 - ADC and the MSTPCRD.MSTPD16 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psard16(&mut self) -> PSARD16_W<16> {
        PSARD16_W::new(self)
    }
    #[doc = "Bit 19 - DAC12 unit1 and the MSTPCRD.MSTPD19 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psard19(&mut self) -> PSARD19_W<19> {
        PSARD19_W::new(self)
    }
    #[doc = "Bit 20 - DAC12 unit0 and the MSTPCRD.MSTPD20 bit security attribution"]
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
    #[doc = "Bit 25 - ACMPHS3 and the MSTPCRD.MSTPD25 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psard25(&mut self) -> PSARD25_W<25> {
        PSARD25_W::new(self)
    }
    #[doc = "Bit 26 - ACMPHS2 and the MSTPCRD.MSTPD26 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psard26(&mut self) -> PSARD26_W<26> {
        PSARD26_W::new(self)
    }
    #[doc = "Bit 27 - ACMPHS1 and the MSTPCRD.MSTPD27 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psard27(&mut self) -> PSARD27_W<27> {
        PSARD27_W::new(self)
    }
    #[doc = "Bit 28 - ACMPHS0 and the MSTPCRD.MSTPD28 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psard28(&mut self) -> PSARD28_W<28> {
        PSARD28_W::new(self)
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
