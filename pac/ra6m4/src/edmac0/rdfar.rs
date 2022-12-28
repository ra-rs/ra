#[doc = "Register `RDFAR` reader"]
pub struct R(crate::R<RDFAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDFAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDFAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDFAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Receive Descriptor Fetch Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdfar](index.html) module"]
pub struct RDFAR_SPEC;
impl crate::RegisterSpec for RDFAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdfar::R](R) reader structure"]
impl crate::Readable for RDFAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RDFAR to value 0"]
impl crate::Resettable for RDFAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
