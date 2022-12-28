#[doc = "Register `COMPSEL0` reader"]
pub struct R(crate::R<COMPSEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMPSEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMPSEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMPSEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMPSEL0` writer"]
pub struct W(crate::W<COMPSEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMPSEL0_SPEC>;
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
impl From<crate::W<COMPSEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMPSEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPSEL10` reader - ACMPLP0 Input (IVCMP0) Selection"]
pub type CMPSEL10_R = crate::FieldReader<u8, CMPSEL10_A>;
#[doc = "ACMPLP0 Input (IVCMP0) Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPSEL10_A {
    #[doc = "0: No input"]
    _00 = 0,
    #[doc = "1: CMPIN0 input selected"]
    _01 = 1,
    #[doc = "2: AMP0O output selected"]
    _10 = 2,
}
impl From<CMPSEL10_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPSEL10_A) -> Self {
        variant as _
    }
}
impl CMPSEL10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMPSEL10_A> {
        match self.bits {
            0 => Some(CMPSEL10_A::_00),
            1 => Some(CMPSEL10_A::_01),
            2 => Some(CMPSEL10_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CMPSEL10_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CMPSEL10_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CMPSEL10_A::_10
    }
}
#[doc = "Field `CMPSEL10` writer - ACMPLP0 Input (IVCMP0) Selection"]
pub type CMPSEL10_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, COMPSEL0_SPEC, u8, CMPSEL10_A, 2, O>;
impl<'a, const O: u8> CMPSEL10_W<'a, O> {
    #[doc = "No input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CMPSEL10_A::_00)
    }
    #[doc = "CMPIN0 input selected"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CMPSEL10_A::_01)
    }
    #[doc = "AMP0O output selected"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMPSEL10_A::_10)
    }
}
#[doc = "Field `CMPSEL54` reader - ACMPLP1 Input (IVCMP1) Selection"]
pub type CMPSEL54_R = crate::FieldReader<u8, CMPSEL54_A>;
#[doc = "ACMPLP1 Input (IVCMP1) Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPSEL54_A {
    #[doc = "0: No input"]
    _00 = 0,
    #[doc = "1: CMPIN1 input selected"]
    _01 = 1,
    #[doc = "2: AMP1O output selected"]
    _10 = 2,
}
impl From<CMPSEL54_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPSEL54_A) -> Self {
        variant as _
    }
}
impl CMPSEL54_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMPSEL54_A> {
        match self.bits {
            0 => Some(CMPSEL54_A::_00),
            1 => Some(CMPSEL54_A::_01),
            2 => Some(CMPSEL54_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CMPSEL54_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CMPSEL54_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CMPSEL54_A::_10
    }
}
#[doc = "Field `CMPSEL54` writer - ACMPLP1 Input (IVCMP1) Selection"]
pub type CMPSEL54_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, COMPSEL0_SPEC, u8, CMPSEL54_A, 2, O>;
impl<'a, const O: u8> CMPSEL54_W<'a, O> {
    #[doc = "No input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CMPSEL54_A::_00)
    }
    #[doc = "CMPIN1 input selected"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CMPSEL54_A::_01)
    }
    #[doc = "AMP1O output selected"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMPSEL54_A::_10)
    }
}
impl R {
    #[doc = "Bits 0:1 - ACMPLP0 Input (IVCMP0) Selection"]
    #[inline(always)]
    pub fn cmpsel10(&self) -> CMPSEL10_R {
        CMPSEL10_R::new(self.bits & 3)
    }
    #[doc = "Bits 4:5 - ACMPLP1 Input (IVCMP1) Selection"]
    #[inline(always)]
    pub fn cmpsel54(&self) -> CMPSEL54_R {
        CMPSEL54_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - ACMPLP0 Input (IVCMP0) Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmpsel10(&mut self) -> CMPSEL10_W<0> {
        CMPSEL10_W::new(self)
    }
    #[doc = "Bits 4:5 - ACMPLP1 Input (IVCMP1) Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmpsel54(&mut self) -> CMPSEL54_W<4> {
        CMPSEL54_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator Input Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [compsel0](index.html) module"]
pub struct COMPSEL0_SPEC;
impl crate::RegisterSpec for COMPSEL0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [compsel0::R](R) reader structure"]
impl crate::Readable for COMPSEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [compsel0::W](W) writer structure"]
impl crate::Writable for COMPSEL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMPSEL0 to value 0x11"]
impl crate::Resettable for COMPSEL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x11;
}
