#[doc = "Register `SD_RSP10` reader"]
pub struct R(crate::R<SD_RSP10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_RSP10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_RSP10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_RSP10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SD_RSP10` reader - Store the response from the SD card/MMC"]
pub type SD_RSP10_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Store the response from the SD card/MMC"]
    #[inline(always)]
    pub fn sd_rsp10(&self) -> SD_RSP10_R {
        SD_RSP10_R::new(self.bits)
    }
}
#[doc = "SD Card Response Register 10\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_rsp10](index.html) module"]
pub struct SD_RSP10_SPEC;
impl crate::RegisterSpec for SD_RSP10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_rsp10::R](R) reader structure"]
impl crate::Readable for SD_RSP10_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SD_RSP10 to value 0"]
impl crate::Resettable for SD_RSP10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
