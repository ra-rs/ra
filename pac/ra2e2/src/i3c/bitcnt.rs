#[doc = "Register `BITCNT` reader"]
pub struct R(crate::R<BITCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BITCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BITCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BITCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BCNT` reader - Bit Counter"]
pub type BCNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - Bit Counter"]
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Bit Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bitcnt](index.html) module"]
pub struct BITCNT_SPEC;
impl crate::RegisterSpec for BITCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bitcnt::R](R) reader structure"]
impl crate::Readable for BITCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BITCNT to value 0"]
impl crate::Resettable for BITCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
