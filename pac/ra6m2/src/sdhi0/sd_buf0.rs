#[doc = "Register `SD_BUF0` reader"]
pub struct R(crate::R<SD_BUF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_BUF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_BUF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_BUF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SD_BUF0` writer"]
pub struct W(crate::W<SD_BUF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_BUF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SD_BUF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_BUF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SD_BUF` reader - SD Buffer RegisterWhen writing to the SD card, the write data is written to this register. When reading from the SD card, the read data is read from this register. This register is internally connected to two 512-byte buffers.If both buffers are not empty when executing multiple block read, SD/MMC clock is stopped to suspend receiving data. When one of buffers is empty, SD/MMC clock is supplied to resume receiving data."]
pub type SD_BUF_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SD_BUF` writer - SD Buffer RegisterWhen writing to the SD card, the write data is written to this register. When reading from the SD card, the read data is read from this register. This register is internally connected to two 512-byte buffers.If both buffers are not empty when executing multiple block read, SD/MMC clock is stopped to suspend receiving data. When one of buffers is empty, SD/MMC clock is supplied to resume receiving data."]
pub type SD_BUF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SD_BUF0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - SD Buffer RegisterWhen writing to the SD card, the write data is written to this register. When reading from the SD card, the read data is read from this register. This register is internally connected to two 512-byte buffers.If both buffers are not empty when executing multiple block read, SD/MMC clock is stopped to suspend receiving data. When one of buffers is empty, SD/MMC clock is supplied to resume receiving data."]
    #[inline(always)]
    pub fn sd_buf(&self) -> SD_BUF_R {
        SD_BUF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SD Buffer RegisterWhen writing to the SD card, the write data is written to this register. When reading from the SD card, the read data is read from this register. This register is internally connected to two 512-byte buffers.If both buffers are not empty when executing multiple block read, SD/MMC clock is stopped to suspend receiving data. When one of buffers is empty, SD/MMC clock is supplied to resume receiving data."]
    #[inline(always)]
    #[must_use]
    pub fn sd_buf(&mut self) -> SD_BUF_W<0> {
        SD_BUF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_buf0](index.html) module"]
pub struct SD_BUF0_SPEC;
impl crate::RegisterSpec for SD_BUF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_buf0::R](R) reader structure"]
impl crate::Readable for SD_BUF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_buf0::W](W) writer structure"]
impl crate::Writable for SD_BUF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SD_BUF0 to value 0"]
impl crate::Resettable for SD_BUF0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
