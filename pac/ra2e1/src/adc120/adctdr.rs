#[doc = "Register `ADCTDR` reader"]
pub struct R(crate::R<ADCTDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCTDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCTDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCTDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADCTDR` reader - Converted Value 15 to 0"]
pub type ADCTDR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Converted Value 15 to 0"]
    #[inline(always)]
    pub fn adctdr(&self) -> ADCTDR_R {
        ADCTDR_R::new(self.bits)
    }
}
#[doc = "A/D CTSU TSCAP Voltage Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adctdr](index.html) module"]
pub struct ADCTDR_SPEC;
impl crate::RegisterSpec for ADCTDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adctdr::R](R) reader structure"]
impl crate::Readable for ADCTDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADCTDR to value 0"]
impl crate::Resettable for ADCTDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
