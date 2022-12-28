#[doc = "Register `MAFCR` reader"]
pub struct R(crate::R<MAFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAFCR` writer"]
pub struct W(crate::W<MAFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAFCR_SPEC>;
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
impl From<crate::W<MAFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAFCR` reader - Multicast Address Frame Receive Counter"]
pub type MAFCR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MAFCR` writer - Multicast Address Frame Receive Counter"]
pub type MAFCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAFCR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Multicast Address Frame Receive Counter"]
    #[inline(always)]
    pub fn mafcr(&self) -> MAFCR_R {
        MAFCR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Multicast Address Frame Receive Counter"]
    #[inline(always)]
    #[must_use]
    pub fn mafcr(&mut self) -> MAFCR_W<0> {
        MAFCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Multicast Address Frame Receive Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mafcr](index.html) module"]
pub struct MAFCR_SPEC;
impl crate::RegisterSpec for MAFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mafcr::R](R) reader structure"]
impl crate::Readable for MAFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mafcr::W](W) writer structure"]
impl crate::Writable for MAFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAFCR to value 0"]
impl crate::Resettable for MAFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
