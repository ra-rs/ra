#[doc = "Register `DMDAR` reader"]
pub struct R(crate::R<DMDAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMDAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMDAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMDAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMDAR` writer"]
pub struct W(crate::W<DMDAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMDAR_SPEC>;
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
impl From<crate::W<DMDAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMDAR_SPEC>) -> Self {
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
#[doc = "DMA Destination Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmdar](index.html) module"]
pub struct DMDAR_SPEC;
impl crate::RegisterSpec for DMDAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmdar::R](R) reader structure"]
impl crate::Readable for DMDAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmdar::W](W) writer structure"]
impl crate::Writable for DMDAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMDAR to value 0"]
impl crate::Resettable for DMDAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
