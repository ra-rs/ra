#[doc = "Register `CTSUSC` reader"]
pub struct R(crate::R<CTSUSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CTSUSC` reader - CTSU Sensor Counter These bits indicate the measurement result of the CTSU. These bits indicate FFFFh when an overflow occurs."]
pub type CTSUSC_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - CTSU Sensor Counter These bits indicate the measurement result of the CTSU. These bits indicate FFFFh when an overflow occurs."]
    #[inline(always)]
    pub fn ctsusc(&self) -> CTSUSC_R {
        CTSUSC_R::new(self.bits)
    }
}
#[doc = "CTSU Sensor Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsusc](index.html) module"]
pub struct CTSUSC_SPEC;
impl crate::RegisterSpec for CTSUSC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctsusc::R](R) reader structure"]
impl crate::Readable for CTSUSC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CTSUSC to value 0"]
impl crate::Resettable for CTSUSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
