#[doc = "Register `MOCOUTCR` reader"]
pub struct R(crate::R<MOCOUTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOCOUTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOCOUTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOCOUTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MOCOUTCR` writer"]
pub struct W(crate::W<MOCOUTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOCOUTCR_SPEC>;
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
impl From<crate::W<MOCOUTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOCOUTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MOCOUTRM` reader - MOCO User Trimming"]
pub type MOCOUTRM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MOCOUTRM` writer - MOCO User Trimming"]
pub type MOCOUTRM_W<'a, const O: u8> = crate::FieldWriter<'a, u8, MOCOUTCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - MOCO User Trimming"]
    #[inline(always)]
    pub fn mocoutrm(&self) -> MOCOUTRM_R {
        MOCOUTRM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - MOCO User Trimming"]
    #[inline(always)]
    #[must_use]
    pub fn mocoutrm(&mut self) -> MOCOUTRM_W<0> {
        MOCOUTRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MOCO User Trimming Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mocoutcr](index.html) module"]
pub struct MOCOUTCR_SPEC;
impl crate::RegisterSpec for MOCOUTCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mocoutcr::R](R) reader structure"]
impl crate::Readable for MOCOUTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mocoutcr::W](W) writer structure"]
impl crate::Writable for MOCOUTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MOCOUTCR to value 0"]
impl crate::Resettable for MOCOUTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
