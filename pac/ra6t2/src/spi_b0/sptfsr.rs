#[doc = "Register `SPTFSR` reader"]
pub struct R(crate::R<SPTFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPTFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPTFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPTFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TFDN` reader - Transmit FIFO data empty stage number"]
pub type TFDN_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Transmit FIFO data empty stage number"]
    #[inline(always)]
    pub fn tfdn(&self) -> TFDN_R {
        TFDN_R::new((self.bits & 7) as u8)
    }
}
#[doc = "SPI Transfer FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sptfsr](index.html) module"]
pub struct SPTFSR_SPEC;
impl crate::RegisterSpec for SPTFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sptfsr::R](R) reader structure"]
impl crate::Readable for SPTFSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPTFSR to value 0x04"]
impl crate::Resettable for SPTFSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
