#[doc = "Register `BCNT3CP%s` reader"]
pub struct R(crate::R<BCNT3CP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCNT3CP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCNT3CP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCNT3CP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BCNT3CP` reader - BCNT3CP is a read-only register that captures the BCNT3 value when a time capture event is detected."]
pub type BCNT3CP_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - BCNT3CP is a read-only register that captures the BCNT3 value when a time capture event is detected."]
    #[inline(always)]
    pub fn bcnt3cp(&self) -> BCNT3CP_R {
        BCNT3CP_R::new(self.bits)
    }
}
#[doc = "BCNT3 Capture Register %s\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcnt3cp](index.html) module"]
pub struct BCNT3CP_SPEC;
impl crate::RegisterSpec for BCNT3CP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bcnt3cp::R](R) reader structure"]
impl crate::Readable for BCNT3CP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BCNT3CP%s to value 0"]
impl crate::Resettable for BCNT3CP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
