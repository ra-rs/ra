#[doc = "Register `FTSR` reader"]
pub struct R(crate::R<FTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `T` reader - Transmit-FIFO Data Count"]
pub type T_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Transmit-FIFO Data Count"]
    #[inline(always)]
    pub fn t(&self) -> T_R {
        T_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "FIFO Transmit Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftsr](index.html) module"]
pub struct FTSR_SPEC;
impl crate::RegisterSpec for FTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ftsr::R](R) reader structure"]
impl crate::Readable for FTSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FTSR to value 0"]
impl crate::Resettable for FTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
