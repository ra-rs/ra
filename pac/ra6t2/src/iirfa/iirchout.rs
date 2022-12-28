#[doc = "Register `IIRCH%sOUT` reader"]
pub struct R(crate::R<IIRCHOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIRCHOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIRCHOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIRCHOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Channel %s Output Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iirchout](index.html) module"]
pub struct IIRCHOUT_SPEC;
impl crate::RegisterSpec for IIRCHOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iirchout::R](R) reader structure"]
impl crate::Readable for IIRCHOUT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IIRCH%sOUT to value 0"]
impl crate::Resettable for IIRCHOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
