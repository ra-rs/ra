#[doc = "Register `TDFAR` reader"]
pub struct R(crate::R<TDFAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TDFAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TDFAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TDFAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Transmit Descriptor Fetch Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdfar](index.html) module"]
pub struct TDFAR_SPEC;
impl crate::RegisterSpec for TDFAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tdfar::R](R) reader structure"]
impl crate::Readable for TDFAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TDFAR to value 0"]
impl crate::Resettable for TDFAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
