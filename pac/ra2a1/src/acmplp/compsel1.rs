#[doc = "Register `COMPSEL1` reader"]
pub struct R(crate::R<COMPSEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMPSEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMPSEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMPSEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMPSEL1` writer"]
pub struct W(crate::W<COMPSEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMPSEL1_SPEC>;
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
impl From<crate::W<COMPSEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMPSEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRVS10` reader - ACMPLP0 Reference Voltage (IVREF0) Selection"]
pub type CRVS10_R = crate::FieldReader<u8, CRVS10_A>;
#[doc = "ACMPLP0 Reference Voltage (IVREF0) Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CRVS10_A {
    #[doc = "0: No reference voltage"]
    _00 = 0,
    #[doc = "1: CMPREF0 selected"]
    _01 = 1,
    #[doc = "2: DA8_0 output selected"]
    _10 = 2,
}
impl From<CRVS10_A> for u8 {
    #[inline(always)]
    fn from(variant: CRVS10_A) -> Self {
        variant as _
    }
}
impl CRVS10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CRVS10_A> {
        match self.bits {
            0 => Some(CRVS10_A::_00),
            1 => Some(CRVS10_A::_01),
            2 => Some(CRVS10_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CRVS10_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CRVS10_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CRVS10_A::_10
    }
}
#[doc = "Field `CRVS10` writer - ACMPLP0 Reference Voltage (IVREF0) Selection"]
pub type CRVS10_W<'a, const O: u8> = crate::FieldWriter<'a, u8, COMPSEL1_SPEC, u8, CRVS10_A, 2, O>;
impl<'a, const O: u8> CRVS10_W<'a, O> {
    #[doc = "No reference voltage"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CRVS10_A::_00)
    }
    #[doc = "CMPREF0 selected"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CRVS10_A::_01)
    }
    #[doc = "DA8_0 output selected"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CRVS10_A::_10)
    }
}
#[doc = "Field `CRVS54` reader - ACMPLP1 Reference Voltage(IVREF1) Selection"]
pub type CRVS54_R = crate::FieldReader<u8, CRVS54_A>;
#[doc = "ACMPLP1 Reference Voltage(IVREF1) Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CRVS54_A {
    #[doc = "0: No reference voltage"]
    _00 = 0,
    #[doc = "1: CMPREF1 selected"]
    _01 = 1,
    #[doc = "2: DA8_1 output selected"]
    _10 = 2,
}
impl From<CRVS54_A> for u8 {
    #[inline(always)]
    fn from(variant: CRVS54_A) -> Self {
        variant as _
    }
}
impl CRVS54_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CRVS54_A> {
        match self.bits {
            0 => Some(CRVS54_A::_00),
            1 => Some(CRVS54_A::_01),
            2 => Some(CRVS54_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CRVS54_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CRVS54_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CRVS54_A::_10
    }
}
#[doc = "Field `CRVS54` writer - ACMPLP1 Reference Voltage(IVREF1) Selection"]
pub type CRVS54_W<'a, const O: u8> = crate::FieldWriter<'a, u8, COMPSEL1_SPEC, u8, CRVS54_A, 2, O>;
impl<'a, const O: u8> CRVS54_W<'a, O> {
    #[doc = "No reference voltage"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CRVS54_A::_00)
    }
    #[doc = "CMPREF1 selected"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CRVS54_A::_01)
    }
    #[doc = "DA8_1 output selected"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CRVS54_A::_10)
    }
}
#[doc = "Field `C1VRF2` reader - ACMPLP1 Reference Voltage Selection"]
pub type C1VRF2_R = crate::BitReader<C1VRF2_A>;
#[doc = "ACMPLP1 Reference Voltage Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1VRF2_A {
    #[doc = "0: IVREF0 selected"]
    _0 = 0,
    #[doc = "1: IVREF1 selected"]
    _1 = 1,
}
impl From<C1VRF2_A> for bool {
    #[inline(always)]
    fn from(variant: C1VRF2_A) -> Self {
        variant as u8 != 0
    }
}
impl C1VRF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C1VRF2_A {
        match self.bits {
            false => C1VRF2_A::_0,
            true => C1VRF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1VRF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1VRF2_A::_1
    }
}
#[doc = "Field `C1VRF2` writer - ACMPLP1 Reference Voltage Selection"]
pub type C1VRF2_W<'a, const O: u8> = crate::BitWriter<'a, u8, COMPSEL1_SPEC, C1VRF2_A, O>;
impl<'a, const O: u8> C1VRF2_W<'a, O> {
    #[doc = "IVREF0 selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(C1VRF2_A::_0)
    }
    #[doc = "IVREF1 selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(C1VRF2_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - ACMPLP0 Reference Voltage (IVREF0) Selection"]
    #[inline(always)]
    pub fn crvs10(&self) -> CRVS10_R {
        CRVS10_R::new(self.bits & 3)
    }
    #[doc = "Bits 4:5 - ACMPLP1 Reference Voltage(IVREF1) Selection"]
    #[inline(always)]
    pub fn crvs54(&self) -> CRVS54_R {
        CRVS54_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 7 - ACMPLP1 Reference Voltage Selection"]
    #[inline(always)]
    pub fn c1vrf2(&self) -> C1VRF2_R {
        C1VRF2_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ACMPLP0 Reference Voltage (IVREF0) Selection"]
    #[inline(always)]
    #[must_use]
    pub fn crvs10(&mut self) -> CRVS10_W<0> {
        CRVS10_W::new(self)
    }
    #[doc = "Bits 4:5 - ACMPLP1 Reference Voltage(IVREF1) Selection"]
    #[inline(always)]
    #[must_use]
    pub fn crvs54(&mut self) -> CRVS54_W<4> {
        CRVS54_W::new(self)
    }
    #[doc = "Bit 7 - ACMPLP1 Reference Voltage Selection"]
    #[inline(always)]
    #[must_use]
    pub fn c1vrf2(&mut self) -> C1VRF2_W<7> {
        C1VRF2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator Reference voltage Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [compsel1](index.html) module"]
pub struct COMPSEL1_SPEC;
impl crate::RegisterSpec for COMPSEL1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [compsel1::R](R) reader structure"]
impl crate::Readable for COMPSEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [compsel1::W](W) writer structure"]
impl crate::Writable for COMPSEL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMPSEL1 to value 0x91"]
impl crate::Resettable for COMPSEL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x91;
}
