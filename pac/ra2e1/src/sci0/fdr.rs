#[doc = "Register `FDR` reader"]
pub struct R(crate::R<FDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `R` reader - Receive FIFO Data Count"]
pub type R_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T` reader - Transmit FIFO Data Count"]
pub type T_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - Receive FIFO Data Count"]
    #[inline(always)]
    pub fn r(&self) -> R_R {
        R_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Transmit FIFO Data Count"]
    #[inline(always)]
    pub fn t(&self) -> T_R {
        T_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
#[doc = "FIFO Data Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdr](index.html) module"]
pub struct FDR_SPEC;
impl crate::RegisterSpec for FDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fdr::R](R) reader structure"]
impl crate::Readable for FDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDR to value 0"]
impl crate::Resettable for FDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
