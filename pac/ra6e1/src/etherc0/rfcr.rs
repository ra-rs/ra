#[doc = "Register `RFCR` reader"]
pub struct R(crate::R<RFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFCR` writer"]
pub struct W(crate::W<RFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFCR_SPEC>;
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
impl From<crate::W<RFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFCR` reader - Received Alignment Error Frame Counter"]
pub type RFCR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RFCR` writer - Received Alignment Error Frame Counter"]
pub type RFCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RFCR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Received Alignment Error Frame Counter"]
    #[inline(always)]
    pub fn rfcr(&self) -> RFCR_R {
        RFCR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Received Alignment Error Frame Counter"]
    #[inline(always)]
    #[must_use]
    pub fn rfcr(&mut self) -> RFCR_W<0> {
        RFCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Received Alignment Error Frame Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcr](index.html) module"]
pub struct RFCR_SPEC;
impl crate::RegisterSpec for RFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfcr::R](R) reader structure"]
impl crate::Readable for RFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfcr::W](W) writer structure"]
impl crate::Writable for RFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFCR to value 0"]
impl crate::Resettable for RFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
