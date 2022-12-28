#[doc = "Register `EIDR` reader"]
pub struct R(crate::R<EIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Event input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eidr](index.html) module"]
pub struct EIDR_SPEC;
impl crate::RegisterSpec for EIDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [eidr::R](R) reader structure"]
impl crate::Readable for EIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EIDR to value 0"]
impl crate::Resettable for EIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
