#[doc = "Register `TSFRCR` reader"]
pub struct R(crate::R<TSFRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSFRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSFRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSFRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSFRCR` writer"]
pub struct W(crate::W<TSFRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSFRCR_SPEC>;
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
impl From<crate::W<TSFRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSFRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSFRCR` reader - Too-Short Frame Receive Counter"]
pub type TSFRCR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TSFRCR` writer - Too-Short Frame Receive Counter"]
pub type TSFRCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSFRCR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Too-Short Frame Receive Counter"]
    #[inline(always)]
    pub fn tsfrcr(&self) -> TSFRCR_R {
        TSFRCR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Too-Short Frame Receive Counter"]
    #[inline(always)]
    #[must_use]
    pub fn tsfrcr(&mut self) -> TSFRCR_W<0> {
        TSFRCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Too-Short Frame Receive Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsfrcr](index.html) module"]
pub struct TSFRCR_SPEC;
impl crate::RegisterSpec for TSFRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsfrcr::R](R) reader structure"]
impl crate::Readable for TSFRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsfrcr::W](W) writer structure"]
impl crate::Writable for TSFRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSFRCR to value 0"]
impl crate::Resettable for TSFRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
