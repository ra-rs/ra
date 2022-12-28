#[doc = "Register `SRCOD` reader"]
pub struct R(crate::R<SRCOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRCOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRCOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRCOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SRCOD` reader - SRCOD is a 32-bit read-only register used to output the data after sampling rate conversion. The data in the 16-stage output data FIFO is read through SRCOD. When the number of data in the output data FIFO is zero after the start of conversion, the value previously read is read again."]
pub type SRCOD_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - SRCOD is a 32-bit read-only register used to output the data after sampling rate conversion. The data in the 16-stage output data FIFO is read through SRCOD. When the number of data in the output data FIFO is zero after the start of conversion, the value previously read is read again."]
    #[inline(always)]
    pub fn srcod(&self) -> SRCOD_R {
        SRCOD_R::new(self.bits)
    }
}
#[doc = "Output Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcod](index.html) module"]
pub struct SRCOD_SPEC;
impl crate::RegisterSpec for SRCOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srcod::R](R) reader structure"]
impl crate::Readable for SRCOD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SRCOD to value 0"]
impl crate::Resettable for SRCOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
