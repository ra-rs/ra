#[doc = "Register `SCDT0` reader"]
pub struct R(crate::R<SCDT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCDT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCDT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCDT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCDT0` writer"]
pub struct W(crate::W<SCDT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCDT0_SPEC>;
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
impl From<crate::W<SCDT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCDT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCDT0` reader - Sine Cosine Data Register 0 (single-precision floating-point)"]
pub type SCDT0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SCDT0` writer - Sine Cosine Data Register 0 (single-precision floating-point)"]
pub type SCDT0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCDT0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Sine Cosine Data Register 0 (single-precision floating-point)"]
    #[inline(always)]
    pub fn scdt0(&self) -> SCDT0_R {
        SCDT0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Sine Cosine Data Register 0 (single-precision floating-point)"]
    #[inline(always)]
    #[must_use]
    pub fn scdt0(&mut self) -> SCDT0_W<0> {
        SCDT0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sine Cosine Data Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scdt0](index.html) module"]
pub struct SCDT0_SPEC;
impl crate::RegisterSpec for SCDT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scdt0::R](R) reader structure"]
impl crate::Readable for SCDT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scdt0::W](W) writer structure"]
impl crate::Writable for SCDT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCDT0 to value 0"]
impl crate::Resettable for SCDT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
