#[doc = "Register `PSMCR` reader"]
pub struct R(crate::R<PSMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSMCR` writer"]
pub struct W(crate::W<PSMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSMCR_SPEC>;
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
impl From<crate::W<PSMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSMC` reader - Power save memory control."]
pub type PSMC_R = crate::FieldReader<u8, PSMC_A>;
#[doc = "Power save memory control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSMC_A {
    #[doc = "0: All RAM is on Software Standby mode."]
    _00 = 0,
    #[doc = "1: 48KB RAM is on in Software Standby mode."]
    _01 = 1,
}
impl From<PSMC_A> for u8 {
    #[inline(always)]
    fn from(variant: PSMC_A) -> Self {
        variant as _
    }
}
impl PSMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PSMC_A> {
        match self.bits {
            0 => Some(PSMC_A::_00),
            1 => Some(PSMC_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PSMC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PSMC_A::_01
    }
}
#[doc = "Field `PSMC` writer - Power save memory control."]
pub type PSMC_W<'a, const O: u8> = crate::FieldWriter<'a, u8, PSMCR_SPEC, u8, PSMC_A, 2, O>;
impl<'a, const O: u8> PSMC_W<'a, O> {
    #[doc = "All RAM is on Software Standby mode."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PSMC_A::_00)
    }
    #[doc = "48KB RAM is on in Software Standby mode."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PSMC_A::_01)
    }
}
impl R {
    #[doc = "Bits 0:1 - Power save memory control."]
    #[inline(always)]
    pub fn psmc(&self) -> PSMC_R {
        PSMC_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Power save memory control."]
    #[inline(always)]
    #[must_use]
    pub fn psmc(&mut self) -> PSMC_W<0> {
        PSMC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Save Memory Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psmcr](index.html) module"]
pub struct PSMCR_SPEC;
impl crate::RegisterSpec for PSMCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [psmcr::R](R) reader structure"]
impl crate::Readable for PSMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psmcr::W](W) writer structure"]
impl crate::Writable for PSMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSMCR to value 0"]
impl crate::Resettable for PSMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
