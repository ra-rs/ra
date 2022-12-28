#[doc = "Register `CDCR` reader"]
pub struct R(crate::R<CDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CDCR` writer"]
pub struct W(crate::W<CDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDCR_SPEC>;
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
impl From<crate::W<CDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CDCR` reader - Late Collision Detect Counter"]
pub type CDCR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CDCR` writer - Late Collision Detect Counter"]
pub type CDCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CDCR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Late Collision Detect Counter"]
    #[inline(always)]
    pub fn cdcr(&self) -> CDCR_R {
        CDCR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Late Collision Detect Counter"]
    #[inline(always)]
    #[must_use]
    pub fn cdcr(&mut self) -> CDCR_W<0> {
        CDCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Late Collision Detect Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdcr](index.html) module"]
pub struct CDCR_SPEC;
impl crate::RegisterSpec for CDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdcr::R](R) reader structure"]
impl crate::Readable for CDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cdcr::W](W) writer structure"]
impl crate::Writable for CDCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDCR to value 0"]
impl crate::Resettable for CDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
