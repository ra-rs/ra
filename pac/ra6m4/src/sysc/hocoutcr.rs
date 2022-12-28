#[doc = "Register `HOCOUTCR` reader"]
pub struct R(crate::R<HOCOUTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOCOUTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOCOUTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOCOUTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOCOUTCR` writer"]
pub struct W(crate::W<HOCOUTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOCOUTCR_SPEC>;
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
impl From<crate::W<HOCOUTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOCOUTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOCOUTRM` reader - HOCO User Trimming"]
pub type HOCOUTRM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOCOUTRM` writer - HOCO User Trimming"]
pub type HOCOUTRM_W<'a, const O: u8> = crate::FieldWriter<'a, u8, HOCOUTCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - HOCO User Trimming"]
    #[inline(always)]
    pub fn hocoutrm(&self) -> HOCOUTRM_R {
        HOCOUTRM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - HOCO User Trimming"]
    #[inline(always)]
    #[must_use]
    pub fn hocoutrm(&mut self) -> HOCOUTRM_W<0> {
        HOCOUTRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HOCO User Trimming Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hocoutcr](index.html) module"]
pub struct HOCOUTCR_SPEC;
impl crate::RegisterSpec for HOCOUTCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [hocoutcr::R](R) reader structure"]
impl crate::Readable for HOCOUTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hocoutcr::W](W) writer structure"]
impl crate::Writable for HOCOUTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOCOUTCR to value 0"]
impl crate::Resettable for HOCOUTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
