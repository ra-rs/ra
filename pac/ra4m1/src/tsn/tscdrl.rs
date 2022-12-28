#[doc = "Register `TSCDRL` reader"]
pub struct R(crate::R<TSCDRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSCDRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSCDRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSCDRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TSCDRL` reader - The calibration data stores the lower 8 bits of the converted value."]
pub type TSCDRL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - The calibration data stores the lower 8 bits of the converted value."]
    #[inline(always)]
    pub fn tscdrl(&self) -> TSCDRL_R {
        TSCDRL_R::new(self.bits)
    }
}
#[doc = "Temperature Sensor Calibration Data Register L\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tscdrl](index.html) module"]
pub struct TSCDRL_SPEC;
impl crate::RegisterSpec for TSCDRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tscdrl::R](R) reader structure"]
impl crate::Readable for TSCDRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TSCDRL to value 0"]
impl crate::Resettable for TSCDRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
