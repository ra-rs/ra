#[doc = "Register `ADCALSTCR` reader"]
pub struct R(crate::R<ADCALSTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCALSTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCALSTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCALSTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCALSTCR` writer"]
pub struct W(crate::W<ADCALSTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCALSTCR_SPEC>;
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
impl From<crate::W<ADCALSTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCALSTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALADSST` reader - A/D Converter Self-calibration Sampling Time Configuration"]
pub type CALADSST_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CALADSST` writer - A/D Converter Self-calibration Sampling Time Configuration"]
pub type CALADSST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCALSTCR_SPEC, u16, u16, 10, O>;
#[doc = "Field `CALADCST` reader - A/D Converter Self-calibration Successive Approximation Time Configuration."]
pub type CALADCST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALADCST` writer - A/D Converter Self-calibration Successive Approximation Time Configuration."]
pub type CALADCST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCALSTCR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:9 - A/D Converter Self-calibration Sampling Time Configuration"]
    #[inline(always)]
    pub fn caladsst(&self) -> CALADSST_R {
        CALADSST_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:21 - A/D Converter Self-calibration Successive Approximation Time Configuration."]
    #[inline(always)]
    pub fn caladcst(&self) -> CALADCST_R {
        CALADCST_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - A/D Converter Self-calibration Sampling Time Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn caladsst(&mut self) -> CALADSST_W<0> {
        CALADSST_W::new(self)
    }
    #[doc = "Bits 16:21 - A/D Converter Self-calibration Successive Approximation Time Configuration."]
    #[inline(always)]
    #[must_use]
    pub fn caladcst(&mut self) -> CALADCST_W<16> {
        CALADCST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Converter Self-calibration State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcalstcr](index.html) module"]
pub struct ADCALSTCR_SPEC;
impl crate::RegisterSpec for ADCALSTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcalstcr::R](R) reader structure"]
impl crate::Readable for ADCALSTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcalstcr::W](W) writer structure"]
impl crate::Writable for ADCALSTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCALSTCR to value 0x0003_0002"]
impl crate::Resettable for ADCALSTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0002;
}
