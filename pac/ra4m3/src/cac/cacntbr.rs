#[doc = "Register `CACNTBR` reader"]
pub struct R(crate::R<CACNTBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACNTBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACNTBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACNTBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "CAC Counter Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cacntbr](index.html) module"]
pub struct CACNTBR_SPEC;
impl crate::RegisterSpec for CACNTBR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cacntbr::R](R) reader structure"]
impl crate::Readable for CACNTBR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CACNTBR to value 0"]
impl crate::Resettable for CACNTBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
