#[doc = "Register `CFDRFPTR%s` reader"]
pub struct R(crate::R<CFDRFPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDRFPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDRFPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDRFPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RFTS` reader - RX FIFO Timestamp Value"]
pub type RFTS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RFDLC` reader - RX FIFO Buffer DLC Field"]
pub type RFDLC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - RX FIFO Timestamp Value"]
    #[inline(always)]
    pub fn rfts(&self) -> RFTS_R {
        RFTS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - RX FIFO Buffer DLC Field"]
    #[inline(always)]
    pub fn rfdlc(&self) -> RFDLC_R {
        RFDLC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "RX FIFO Access Pointer Register %s\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdrfptr](index.html) module"]
pub struct CFDRFPTR_SPEC;
impl crate::RegisterSpec for CFDRFPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdrfptr::R](R) reader structure"]
impl crate::Readable for CFDRFPTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFDRFPTR%s to value 0"]
impl crate::Resettable for CFDRFPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
