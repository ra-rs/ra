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
#[doc = "Field `TSCDR` reader - Temperature Sensor Calibration Data"]
pub type TSCDR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Temperature Sensor Calibration Data"]
    #[inline(always)]
    pub fn tscdr(&self) -> TSCDR_R {
        TSCDR_R::new((self.bits & 0xffff) as u16)
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
