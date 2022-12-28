#[doc = "Register `NRSPQP` reader"]
pub struct R(crate::R<NRSPQP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NRSPQP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NRSPQP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NRSPQP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Normal Response Queue Port Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nrspqp](index.html) module"]
pub struct NRSPQP_SPEC;
impl crate::RegisterSpec for NRSPQP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nrspqp::R](R) reader structure"]
impl crate::Readable for NRSPQP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NRSPQP to value 0"]
impl crate::Resettable for NRSPQP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
