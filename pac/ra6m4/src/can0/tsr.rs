#[doc = "Register `TSR` reader"]
pub struct R(crate::R<TSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Time Stamp Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsr](index.html) module"]
pub struct TSR_SPEC;
impl crate::RegisterSpec for TSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tsr::R](R) reader structure"]
impl crate::Readable for TSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TSR to value 0"]
impl crate::Resettable for TSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
