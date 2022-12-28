#[doc = "Register `FRECR` reader"]
pub struct R(crate::R<FRECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRECR` writer"]
pub struct W(crate::W<FRECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRECR_SPEC>;
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
impl From<crate::W<FRECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRECR` reader - Frame Receive Error Counter"]
pub type FRECR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FRECR` writer - Frame Receive Error Counter"]
pub type FRECR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRECR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Frame Receive Error Counter"]
    #[inline(always)]
    pub fn frecr(&self) -> FRECR_R {
        FRECR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Frame Receive Error Counter"]
    #[inline(always)]
    #[must_use]
    pub fn frecr(&mut self) -> FRECR_W<0> {
        FRECR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame Receive Error Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frecr](index.html) module"]
pub struct FRECR_SPEC;
impl crate::RegisterSpec for FRECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frecr::R](R) reader structure"]
impl crate::Readable for FRECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frecr::W](W) writer structure"]
impl crate::Writable for FRECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRECR to value 0"]
impl crate::Resettable for FRECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
