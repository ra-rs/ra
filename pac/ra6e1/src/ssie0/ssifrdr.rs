#[doc = "Register `SSIFRDR` reader"]
pub struct R(crate::R<SSIFRDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSIFRDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSIFRDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSIFRDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SSIFRDR` reader - Receive FIFO Data"]
pub type SSIFRDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive FIFO Data"]
    #[inline(always)]
    pub fn ssifrdr(&self) -> SSIFRDR_R {
        SSIFRDR_R::new(self.bits)
    }
}
#[doc = "Receive FIFO Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssifrdr](index.html) module"]
pub struct SSIFRDR_SPEC;
impl crate::RegisterSpec for SSIFRDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssifrdr::R](R) reader structure"]
impl crate::Readable for SSIFRDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SSIFRDR to value 0"]
impl crate::Resettable for SSIFRDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
