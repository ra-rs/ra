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
#[doc = "Field `CRVS20` reader - ACMPLP0 Reference Voltage(IVREF0) Selection*"]
pub type CRVS20_R = crate::FieldReader<u8, CRVS20_A>;
#[doc = "ACMPLP0 Reference Voltage(IVREF0) Selection*\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CRVS20_A {
    #[doc = "0: No input"]
    _000 = 0,
    #[doc = "1: CMPREF0 (P101)"]
    _001 = 1,
    #[doc = "2: DAC8 (ch0) output"]
    _010 = 2,
    #[doc = "4: CMPREF0 (P502)"]
    _100 = 4,
}
impl From<CRVS20_A> for u8 {
    #[inline(always)]
    fn from(variant: CRVS20_A) -> Self {
        variant as _
    }
}
impl CRVS20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CRVS20_A> {
        match self.bits {
            0 => Some(CRVS20_A::_000),
            1 => Some(CRVS20_A::_001),
            2 => Some(CRVS20_A::_010),
            4 => Some(CRVS20_A::_100),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CRVS20_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CRVS20_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CRVS20_A::_010
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CRVS20_A::_100
    }
}
#[doc = "Field `CRVS20` writer - ACMPLP0 Reference Voltage(IVREF0) Selection*"]
pub type CRVS20_W<'a, const O: u8> = crate::FieldWriter<'a, u8, COMPSEL1_SPEC, u8, CRVS20_A, 3, O>;
impl<'a, const O: u8> CRVS20_W<'a, O> {
    #[doc = "No input"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CRVS20_A::_000)
    }
    #[doc = "CMPREF0 (P101)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CRVS20_A::_001)
    }
    #[doc = "DAC8 (ch0) output"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CRVS20_A::_010)
    }
    #[doc = "CMPREF0 (P502)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CRVS20_A::_100)
    }
}
#[doc = "Field `CRVS64` reader - ACMPLP1 Reference Voltage(IVREF1) Selection"]
pub type CRVS64_R = crate::FieldReader<u8, CRVS64_A>;
#[doc = "ACMPLP1 Reference Voltage(IVREF1) Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CRVS64_A {
    #[doc = "0: No input"]
    _000 = 0,
    #[doc = "1: CMPREF1 (P103)"]
    _001 = 1,
    #[doc = "2: DAC8 (ch1) output"]
    _010 = 2,
    #[doc = "4: CMPREF1 (P500)"]
    _100 = 4,
}
impl From<CRVS64_A> for u8 {
    #[inline(always)]
    fn from(variant: CRVS64_A) -> Self {
        variant as _
    }
}
impl CRVS64_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CRVS64_A> {
        match self.bits {
            0 => Some(CRVS64_A::_000),
            1 => Some(CRVS64_A::_001),
            2 => Some(CRVS64_A::_010),
            4 => Some(CRVS64_A::_100),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CRVS64_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CRVS64_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CRVS64_A::_010
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CRVS64_A::_100
    }
}
#[doc = "Field `CRVS64` writer - ACMPLP1 Reference Voltage(IVREF1) Selection"]
pub type CRVS64_W<'a, const O: u8> = crate::FieldWriter<'a, u8, COMPSEL1_SPEC, u8, CRVS64_A, 3, O>;
impl<'a, const O: u8> CRVS64_W<'a, O> {
    #[doc = "No input"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CRVS64_A::_000)
    }
    #[doc = "CMPREF1 (P103)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CRVS64_A::_001)
    }
    #[doc = "DAC8 (ch1) output"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CRVS64_A::_010)
    }
    #[doc = "CMPREF1 (P500)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CRVS64_A::_100)
    }
}
#[doc = "Field `C1VRF2` reader - ACMPLP1 Reference Voltage Selection"]
pub type C1VRF2_R = crate::BitReader<C1VRF2_A>;
#[doc = "ACMPLP1 Reference Voltage Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1VRF2_A {
    #[doc = "0: IVREF0 selected"]
    _0 = 0,
    #[doc = "1: IVREF1 selected."]
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
    #[doc = "IVREF1 selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(C1VRF2_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - ACMPLP0 Reference Voltage(IVREF0) Selection*"]
    #[inline(always)]
    pub fn crvs20(&self) -> CRVS20_R {
        CRVS20_R::new(self.bits & 7)
    }
    #[doc = "Bits 4:6 - ACMPLP1 Reference Voltage(IVREF1) Selection"]
    #[inline(always)]
    pub fn crvs64(&self) -> CRVS64_R {
        CRVS64_R::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - ACMPLP1 Reference Voltage Selection"]
    #[inline(always)]
    pub fn c1vrf2(&self) -> C1VRF2_R {
        C1VRF2_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - ACMPLP0 Reference Voltage(IVREF0) Selection*"]
    #[inline(always)]
    #[must_use]
    pub fn crvs20(&mut self) -> CRVS20_W<0> {
        CRVS20_W::new(self)
    }
    #[doc = "Bits 4:6 - ACMPLP1 Reference Voltage(IVREF1) Selection"]
    #[inline(always)]
    #[must_use]
    pub fn crvs64(&mut self) -> CRVS64_W<4> {
        CRVS64_W::new(self)
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
#[doc = "Comparator Reference Voltage Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [compsel1](index.html) module"]
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
