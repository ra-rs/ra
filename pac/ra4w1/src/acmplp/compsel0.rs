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
#[doc = "Field `CMPSEL20` reader - ACMPLP0 Input(IVCMP0) Selection"]
pub type CMPSEL20_R = crate::FieldReader<u8, CMPSEL20_A>;
#[doc = "ACMPLP0 Input(IVCMP0) Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPSEL20_A {
    #[doc = "0: No input"]
    _000 = 0,
    #[doc = "1: CMPIN0 (P100)"]
    _001 = 1,
    #[doc = "4: CMPIN0 (P503)"]
    _100 = 4,
}
impl From<CMPSEL20_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPSEL20_A) -> Self {
        variant as _
    }
}
impl CMPSEL20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMPSEL20_A> {
        match self.bits {
            0 => Some(CMPSEL20_A::_000),
            1 => Some(CMPSEL20_A::_001),
            4 => Some(CMPSEL20_A::_100),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CMPSEL20_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CMPSEL20_A::_001
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CMPSEL20_A::_100
    }
}
#[doc = "Field `CMPSEL20` writer - ACMPLP0 Input(IVCMP0) Selection"]
pub type CMPSEL20_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, COMPSEL0_SPEC, u8, CMPSEL20_A, 3, O>;
impl<'a, const O: u8> CMPSEL20_W<'a, O> {
    #[doc = "No input"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CMPSEL20_A::_000)
    }
    #[doc = "CMPIN0 (P100)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CMPSEL20_A::_001)
    }
    #[doc = "CMPIN0 (P503)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CMPSEL20_A::_100)
    }
}
#[doc = "Field `CMPSEL64` reader - ACMPLP1 Input (IVCMP1) Selection"]
pub type CMPSEL64_R = crate::FieldReader<u8, CMPSEL64_A>;
#[doc = "ACMPLP1 Input (IVCMP1) Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPSEL64_A {
    #[doc = "0: No input"]
    _000 = 0,
    #[doc = "1: CMPIN1 (P102)"]
    _001 = 1,
    #[doc = "4: CMPIN1 (P501)"]
    _100 = 4,
}
impl From<CMPSEL64_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPSEL64_A) -> Self {
        variant as _
    }
}
impl CMPSEL64_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMPSEL64_A> {
        match self.bits {
            0 => Some(CMPSEL64_A::_000),
            1 => Some(CMPSEL64_A::_001),
            4 => Some(CMPSEL64_A::_100),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CMPSEL64_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CMPSEL64_A::_001
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CMPSEL64_A::_100
    }
}
#[doc = "Field `CMPSEL64` writer - ACMPLP1 Input (IVCMP1) Selection"]
pub type CMPSEL64_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, COMPSEL0_SPEC, u8, CMPSEL64_A, 3, O>;
impl<'a, const O: u8> CMPSEL64_W<'a, O> {
    #[doc = "No input"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CMPSEL64_A::_000)
    }
    #[doc = "CMPIN1 (P102)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CMPSEL64_A::_001)
    }
    #[doc = "CMPIN1 (P501)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CMPSEL64_A::_100)
    }
}
impl R {
    #[doc = "Bits 0:2 - ACMPLP0 Input(IVCMP0) Selection"]
    #[inline(always)]
    pub fn cmpsel20(&self) -> CMPSEL20_R {
        CMPSEL20_R::new(self.bits & 7)
    }
    #[doc = "Bits 4:6 - ACMPLP1 Input (IVCMP1) Selection"]
    #[inline(always)]
    pub fn cmpsel64(&self) -> CMPSEL64_R {
        CMPSEL64_R::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - ACMPLP0 Input(IVCMP0) Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmpsel20(&mut self) -> CMPSEL20_W<0> {
        CMPSEL20_W::new(self)
    }
    #[doc = "Bits 4:6 - ACMPLP1 Input (IVCMP1) Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmpsel64(&mut self) -> CMPSEL64_W<4> {
        CMPSEL64_W::new(self)
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
