#[doc = "Register `CTSURC` reader"]
pub struct R(crate::R<CTSURC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSURC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSURC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSURC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CTSURC` reader - CTSU Reference Counter These bits indicate the measurement result of the reference ICO. These bits indicate FFFFh when an overflow occurs."]
pub type CTSURC_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - CTSU Reference Counter These bits indicate the measurement result of the reference ICO. These bits indicate FFFFh when an overflow occurs."]
    #[inline(always)]
    pub fn ctsurc(&self) -> CTSURC_R {
        CTSURC_R::new(self.bits)
    }
}
#[doc = "CTSU Reference Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsurc](index.html) module"]
pub struct CTSURC_SPEC;
impl crate::RegisterSpec for CTSURC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctsurc::R](R) reader structure"]
impl crate::Readable for CTSURC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CTSURC to value 0"]
impl crate::Resettable for CTSURC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
