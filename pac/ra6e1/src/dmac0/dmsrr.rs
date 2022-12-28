#[doc = "Register `DMSRR` reader"]
pub struct R(crate::R<DMSRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMSRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMSRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMSRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMSRR` writer"]
pub struct W(crate::W<DMSRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMSRR_SPEC>;
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
impl From<crate::W<DMSRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMSRR_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Source Reload Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmsrr](index.html) module"]
pub struct DMSRR_SPEC;
impl crate::RegisterSpec for DMSRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmsrr::R](R) reader structure"]
impl crate::Readable for DMSRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmsrr::W](W) writer structure"]
impl crate::Writable for DMSRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMSRR to value 0"]
impl crate::Resettable for DMSRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
