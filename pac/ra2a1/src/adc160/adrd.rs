#[doc = "Register `ADRD` reader"]
pub struct R(crate::R<ADRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADRD` reader - The ADRD register is a 16-bit read-only register that holds the A/D conversion results based on the self-diagnosis of the ADC16."]
pub type ADRD_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - The ADRD register is a 16-bit read-only register that holds the A/D conversion results based on the self-diagnosis of the ADC16."]
    #[inline(always)]
    pub fn adrd(&self) -> ADRD_R {
        ADRD_R::new(self.bits)
    }
}
#[doc = "A/D Self-Diagnosis Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adrd](index.html) module"]
pub struct ADRD_SPEC;
impl crate::RegisterSpec for ADRD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adrd::R](R) reader structure"]
impl crate::Readable for ADRD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADRD to value 0"]
impl crate::Resettable for ADRD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
