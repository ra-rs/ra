#[doc = "Register `ADCALSTR` writer"]
pub struct W(crate::W<ADCALSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCALSTR_SPEC>;
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
impl From<crate::W<ADCALSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCALSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCALST0` writer - A/D Converter Unit 0 (ADC0) Self-calibration Start Control"]
pub type ADCALST0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCALSTR_SPEC, u8, u8, 3, O>;
#[doc = "Field `ADCALST1` writer - A/D Converter Unit 1 (ADC1) Self-calibration Start Control"]
pub type ADCALST1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCALSTR_SPEC, u8, u8, 3, O>;
impl W {
    #[doc = "Bits 0:2 - A/D Converter Unit 0 (ADC0) Self-calibration Start Control"]
    #[inline(always)]
    #[must_use]
    pub fn adcalst0(&mut self) -> ADCALST0_W<0> {
        ADCALST0_W::new(self)
    }
    #[doc = "Bits 8:10 - A/D Converter Unit 1 (ADC1) Self-calibration Start Control"]
    #[inline(always)]
    #[must_use]
    pub fn adcalst1(&mut self) -> ADCALST1_W<8> {
        ADCALST1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Converter Self-calibration Start Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcalstr](index.html) module"]
pub struct ADCALSTR_SPEC;
impl crate::RegisterSpec for ADCALSTR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [adcalstr::W](W) writer structure"]
impl crate::Writable for ADCALSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCALSTR to value 0"]
impl crate::Resettable for ADCALSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
