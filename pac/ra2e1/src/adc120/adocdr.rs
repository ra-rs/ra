#[doc = "Register `ADOCDR` reader"]
pub struct R(crate::R<ADOCDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADOCDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADOCDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADOCDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADOCDR` reader - Converted Value 15 to 0"]
pub type ADOCDR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Converted Value 15 to 0"]
    #[inline(always)]
    pub fn adocdr(&self) -> ADOCDR_R {
        ADOCDR_R::new(self.bits)
    }
}
#[doc = "A/D Internal Reference Voltage Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adocdr](index.html) module"]
pub struct ADOCDR_SPEC;
impl crate::RegisterSpec for ADOCDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adocdr::R](R) reader structure"]
impl crate::Readable for ADOCDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADOCDR to value 0"]
impl crate::Resettable for ADOCDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
