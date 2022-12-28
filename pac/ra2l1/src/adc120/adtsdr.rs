#[doc = "Register `ADTSDR` reader"]
pub struct R(crate::R<ADTSDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADTSDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADTSDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADTSDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADTSDR` reader - Converted Value 15 to 0"]
pub type ADTSDR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Converted Value 15 to 0"]
    #[inline(always)]
    pub fn adtsdr(&self) -> ADTSDR_R {
        ADTSDR_R::new(self.bits)
    }
}
#[doc = "A/D Temperature Sensor Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adtsdr](index.html) module"]
pub struct ADTSDR_SPEC;
impl crate::RegisterSpec for ADTSDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adtsdr::R](R) reader structure"]
impl crate::Readable for ADTSDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADTSDR to value 0"]
impl crate::Resettable for ADTSDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
