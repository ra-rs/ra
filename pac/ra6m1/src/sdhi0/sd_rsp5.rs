#[doc = "Register `SD_RSP5` reader"]
pub struct R(crate::R<SD_RSP5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_RSP5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_RSP5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_RSP5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SD_RSP5` reader - Store the response from the SD card/MMC"]
pub type SD_RSP5_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Store the response from the SD card/MMC"]
    #[inline(always)]
    pub fn sd_rsp5(&self) -> SD_RSP5_R {
        SD_RSP5_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "SD Card Response Register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_rsp5](index.html) module"]
pub struct SD_RSP5_SPEC;
impl crate::RegisterSpec for SD_RSP5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_rsp5::R](R) reader structure"]
impl crate::Readable for SD_RSP5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SD_RSP5 to value 0"]
impl crate::Resettable for SD_RSP5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
