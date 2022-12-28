#[doc = "Register `BCNT1CP%s` reader"]
pub struct R(crate::R<BCNT1CP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCNT1CP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCNT1CP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCNT1CP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BCNT1CP` reader - BCNT1CP is a read-only register that captures the BCNT1 value when a time capture event is detected."]
pub type BCNT1CP_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - BCNT1CP is a read-only register that captures the BCNT1 value when a time capture event is detected."]
    #[inline(always)]
    pub fn bcnt1cp(&self) -> BCNT1CP_R {
        BCNT1CP_R::new(self.bits)
    }
}
#[doc = "BCNT1 Capture Register %s\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcnt1cp](index.html) module"]
pub struct BCNT1CP_SPEC;
impl crate::RegisterSpec for BCNT1CP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bcnt1cp::R](R) reader structure"]
impl crate::Readable for BCNT1CP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BCNT1CP%s to value 0"]
impl crate::Resettable for BCNT1CP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
