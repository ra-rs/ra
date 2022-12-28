#[doc = "Register `CFDRMPTR%s` reader"]
pub struct R(crate::R<CFDRMPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDRMPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDRMPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDRMPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RMTS` reader - RX Message Buffer Timestamp Field"]
pub type RMTS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RMDLC` reader - RX Message Buffer DLC Field"]
pub type RMDLC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - RX Message Buffer Timestamp Field"]
    #[inline(always)]
    pub fn rmts(&self) -> RMTS_R {
        RMTS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - RX Message Buffer DLC Field"]
    #[inline(always)]
    pub fn rmdlc(&self) -> RMDLC_R {
        RMDLC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "RX Message Buffer Pointer Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdrmptr](index.html) module"]
pub struct CFDRMPTR_SPEC;
impl crate::RegisterSpec for CFDRMPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdrmptr::R](R) reader structure"]
impl crate::Readable for CFDRMPTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFDRMPTR%s to value 0"]
impl crate::Resettable for CFDRMPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
