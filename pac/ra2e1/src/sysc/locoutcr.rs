#[doc = "Register `LOCOUTCR` reader"]
pub struct R(crate::R<LOCOUTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCOUTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCOUTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCOUTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOCOUTCR` writer"]
pub struct W(crate::W<LOCOUTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOCOUTCR_SPEC>;
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
impl From<crate::W<LOCOUTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOCOUTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCOUTRM` reader - LOCO User Trimming"]
pub type LOCOUTRM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOCOUTRM` writer - LOCO User Trimming"]
pub type LOCOUTRM_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LOCOUTCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - LOCO User Trimming"]
    #[inline(always)]
    pub fn locoutrm(&self) -> LOCOUTRM_R {
        LOCOUTRM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - LOCO User Trimming"]
    #[inline(always)]
    pub fn locoutrm(&mut self) -> LOCOUTRM_W<0> {
        LOCOUTRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LOCO User Trimming Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [locoutcr](index.html) module"]
pub struct LOCOUTCR_SPEC;
impl crate::RegisterSpec for LOCOUTCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [locoutcr::R](R) reader structure"]
impl crate::Readable for LOCOUTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [locoutcr::W](W) writer structure"]
impl crate::Writable for LOCOUTCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOCOUTCR to value 0"]
impl crate::Resettable for LOCOUTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
