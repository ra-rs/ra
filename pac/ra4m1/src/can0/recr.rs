#[doc = "Register `RECR` reader"]
pub struct R(crate::R<RECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RECR` reader - Receive error count function RECR increments or decrements the counter value according to the error status of the CAN module during reception."]
pub type RECR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Receive error count function RECR increments or decrements the counter value according to the error status of the CAN module during reception."]
    #[inline(always)]
    pub fn recr(&self) -> RECR_R {
        RECR_R::new(self.bits)
    }
}
#[doc = "Receive Error Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [recr](index.html) module"]
pub struct RECR_SPEC;
impl crate::RegisterSpec for RECR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [recr::R](R) reader structure"]
impl crate::Readable for RECR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RECR to value 0"]
impl crate::Resettable for RECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
