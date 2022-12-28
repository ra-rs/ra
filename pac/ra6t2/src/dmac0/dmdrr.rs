#[doc = "Register `DMDRR` reader"]
pub struct R(crate::R<DMDRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMDRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMDRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMDRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMDRR` writer"]
pub struct W(crate::W<DMDRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMDRR_SPEC>;
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
impl From<crate::W<DMDRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMDRR_SPEC>) -> Self {
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
#[doc = "DMA Destination Reload Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmdrr](index.html) module"]
pub struct DMDRR_SPEC;
impl crate::RegisterSpec for DMDRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmdrr::R](R) reader structure"]
impl crate::Readable for DMDRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmdrr::W](W) writer structure"]
impl crate::Writable for DMDRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMDRR to value 0"]
impl crate::Resettable for DMDRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
