#[doc = "Register `ADCNVSTR` reader"]
pub struct R(crate::R<ADCNVSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCNVSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCNVSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCNVSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCNVSTR` writer"]
pub struct W(crate::W<ADCNVSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCNVSTR_SPEC>;
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
impl From<crate::W<ADCNVSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCNVSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CST0` reader - A/D Converter Unit 0 (ADC0)"]
pub type CST0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CST0` writer - A/D Converter Unit 0 (ADC0)"]
pub type CST0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCNVSTR_SPEC, u8, u8, 6, O>;
#[doc = "Field `CST1` reader - A/D Converter Unit 1 (ADC1)"]
pub type CST1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CST1` writer - A/D Converter Unit 1 (ADC1)"]
pub type CST1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCNVSTR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - A/D Converter Unit 0 (ADC0)"]
    #[inline(always)]
    pub fn cst0(&self) -> CST0_R {
        CST0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - A/D Converter Unit 1 (ADC1)"]
    #[inline(always)]
    pub fn cst1(&self) -> CST1_R {
        CST1_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - A/D Converter Unit 0 (ADC0)"]
    #[inline(always)]
    #[must_use]
    pub fn cst0(&mut self) -> CST0_W<0> {
        CST0_W::new(self)
    }
    #[doc = "Bits 8:13 - A/D Converter Unit 1 (ADC1)"]
    #[inline(always)]
    #[must_use]
    pub fn cst1(&mut self) -> CST1_W<8> {
        CST1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Conversion State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcnvstr](index.html) module"]
pub struct ADCNVSTR_SPEC;
impl crate::RegisterSpec for ADCNVSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcnvstr::R](R) reader structure"]
impl crate::Readable for ADCNVSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcnvstr::W](W) writer structure"]
impl crate::Writable for ADCNVSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCNVSTR to value 0x0303"]
impl crate::Resettable for ADCNVSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0303;
}
