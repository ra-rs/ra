#[doc = "Register `SD_RSP3` reader"]
pub struct R(crate::R<SD_RSP3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_RSP3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_RSP3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_RSP3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "SD Card Response Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_rsp3](index.html) module"]
pub struct SD_RSP3_SPEC;
impl crate::RegisterSpec for SD_RSP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_rsp3::R](R) reader structure"]
impl crate::Readable for SD_RSP3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SD_RSP3 to value 0"]
impl crate::Resettable for SD_RSP3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
