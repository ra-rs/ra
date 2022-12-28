#[doc = "Register `BCNT0CP%s` reader"]
pub struct R(crate::R<BCNT0CP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCNT0CP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCNT0CP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCNT0CP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BCNT0CP` reader - BCNT0CP is a read-only register that captures the BCNT0 value when a time capture event is detected."]
pub type BCNT0CP_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - BCNT0CP is a read-only register that captures the BCNT0 value when a time capture event is detected."]
    #[inline(always)]
    pub fn bcnt0cp(&self) -> BCNT0CP_R {
        BCNT0CP_R::new(self.bits)
    }
}
#[doc = "BCNT0 Capture Register %s\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcnt0cp](index.html) module"]
pub struct BCNT0CP_SPEC;
impl crate::RegisterSpec for BCNT0CP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bcnt0cp::R](R) reader structure"]
impl crate::Readable for BCNT0CP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BCNT0CP%s to value 0"]
impl crate::Resettable for BCNT0CP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
