#[doc = "Register `SD_RSP76` reader"]
pub struct R(crate::R<SD_RSP76_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_RSP76_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_RSP76_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_RSP76_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SD_RSP76` reader - These bits store the response from the SD card/MMC."]
pub type SD_RSP76_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - These bits store the response from the SD card/MMC."]
    #[inline(always)]
    pub fn sd_rsp76(&self) -> SD_RSP76_R {
        SD_RSP76_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "SD Card Response Register 76\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_rsp76](index.html) module"]
pub struct SD_RSP76_SPEC;
impl crate::RegisterSpec for SD_RSP76_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_rsp76::R](R) reader structure"]
impl crate::Readable for SD_RSP76_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SD_RSP76 to value 0"]
impl crate::Resettable for SD_RSP76_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
