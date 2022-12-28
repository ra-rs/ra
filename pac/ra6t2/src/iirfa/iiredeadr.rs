#[doc = "Register `IIREDEADR` reader"]
pub struct R(crate::R<IIREDEADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIREDEADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIREDEADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIREDEADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DEADR` reader - Error address"]
pub type DEADR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - Error address"]
    #[inline(always)]
    pub fn deadr(&self) -> DEADR_R {
        DEADR_R::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "ECC 2-bit Error Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iiredeadr](index.html) module"]
pub struct IIREDEADR_SPEC;
impl crate::RegisterSpec for IIREDEADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iiredeadr::R](R) reader structure"]
impl crate::Readable for IIREDEADR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IIREDEADR to value 0"]
impl crate::Resettable for IIREDEADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
