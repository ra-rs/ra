#[doc = "Register `TSCDRH` reader"]
pub struct R(crate::R<TSCDRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSCDRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSCDRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSCDRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TSCDRH` reader - The calibration data stores the higher 8 bits of the converted value."]
pub type TSCDRH_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - The calibration data stores the higher 8 bits of the converted value."]
    #[inline(always)]
    pub fn tscdrh(&self) -> TSCDRH_R {
        TSCDRH_R::new(self.bits)
    }
}
#[doc = "Temperature Sensor Calibration Data Register H\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tscdrh](index.html) module"]
pub struct TSCDRH_SPEC;
impl crate::RegisterSpec for TSCDRH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tscdrh::R](R) reader structure"]
impl crate::Readable for TSCDRH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TSCDRH to value 0"]
impl crate::Resettable for TSCDRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
