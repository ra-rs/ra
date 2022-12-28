#[doc = "Register `SD_RSP32` reader"]
pub struct R(crate::R<SD_RSP32_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_RSP32_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_RSP32_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_RSP32_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SD_RSP32` reader - Store the response from the SD card/MMC"]
pub type SD_RSP32_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Store the response from the SD card/MMC"]
    #[inline(always)]
    pub fn sd_rsp32(&self) -> SD_RSP32_R {
        SD_RSP32_R::new(self.bits)
    }
}
#[doc = "SD Card Response Register 32\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_rsp32](index.html) module"]
pub struct SD_RSP32_SPEC;
impl crate::RegisterSpec for SD_RSP32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_rsp32::R](R) reader structure"]
impl crate::Readable for SD_RSP32_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SD_RSP32 to value 0"]
impl crate::Resettable for SD_RSP32_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
