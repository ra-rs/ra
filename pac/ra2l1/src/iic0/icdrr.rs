#[doc = "Register `ICDRR` reader"]
pub struct R(crate::R<ICDRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICDRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICDRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICDRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "I2C Bus Receive Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icdrr](index.html) module"]
pub struct ICDRR_SPEC;
impl crate::RegisterSpec for ICDRR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [icdrr::R](R) reader structure"]
impl crate::Readable for ICDRR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ICDRR to value 0"]
impl crate::Resettable for ICDRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
