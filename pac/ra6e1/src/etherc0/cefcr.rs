#[doc = "Register `CEFCR` reader"]
pub struct R(crate::R<CEFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CEFCR` writer"]
pub struct W(crate::W<CEFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEFCR_SPEC>;
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
impl From<crate::W<CEFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CEFCR` reader - CRC Error Frame Receive Counter"]
pub type CEFCR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CEFCR` writer - CRC Error Frame Receive Counter"]
pub type CEFCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CEFCR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CRC Error Frame Receive Counter"]
    #[inline(always)]
    pub fn cefcr(&self) -> CEFCR_R {
        CEFCR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Error Frame Receive Counter"]
    #[inline(always)]
    #[must_use]
    pub fn cefcr(&mut self) -> CEFCR_W<0> {
        CEFCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Error Frame Receive Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cefcr](index.html) module"]
pub struct CEFCR_SPEC;
impl crate::RegisterSpec for CEFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cefcr::R](R) reader structure"]
impl crate::Readable for CEFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cefcr::W](W) writer structure"]
impl crate::Writable for CEFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CEFCR to value 0"]
impl crate::Resettable for CEFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
