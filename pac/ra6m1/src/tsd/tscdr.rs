#[doc = "Register `TSCDR` reader"]
pub struct R(crate::R<TSCDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSCDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSCDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSCDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TSCD` reader - Temperature sensor calibration data is a digital value obtained using the 12-bit A/D converter unit 0 to convert the voltage output by the temperature sensor under the condition Ta = Tj = 127°C and AVCC0 = 3.3 V."]
pub type TSCD_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Temperature sensor calibration data is a digital value obtained using the 12-bit A/D converter unit 0 to convert the voltage output by the temperature sensor under the condition Ta = Tj = 127°C and AVCC0 = 3.3 V."]
    #[inline(always)]
    pub fn tscd(&self) -> TSCD_R {
        TSCD_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Temperature Sensor Calibration Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tscdr](index.html) module"]
pub struct TSCDR_SPEC;
impl crate::RegisterSpec for TSCDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tscdr::R](R) reader structure"]
impl crate::Readable for TSCDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TSCDR to value 0"]
impl crate::Resettable for TSCDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
