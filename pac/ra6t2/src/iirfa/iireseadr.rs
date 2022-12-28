#[doc = "Register `IIRESEADR` reader"]
pub struct R(crate::R<IIRESEADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIRESEADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIRESEADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIRESEADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEADR` reader - Error address"]
pub type SEADR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - Error address"]
    #[inline(always)]
    pub fn seadr(&self) -> SEADR_R {
        SEADR_R::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "ECC 1-bit Error Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iireseadr](index.html) module"]
pub struct IIRESEADR_SPEC;
impl crate::RegisterSpec for IIRESEADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iireseadr::R](R) reader structure"]
impl crate::Readable for IIRESEADR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IIRESEADR to value 0"]
impl crate::Resettable for IIRESEADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
