#[doc = "Register `FRBH0` reader"]
pub struct R(crate::R<FRBH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRBH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRBH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRBH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDATA` reader - Flash Read Buffer H0"]
pub type RDATA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Flash Read Buffer H0"]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new(self.bits)
    }
}
#[doc = "Flash Read Buffer Register H0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frbh0](index.html) module"]
pub struct FRBH0_SPEC;
impl crate::RegisterSpec for FRBH0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [frbh0::R](R) reader structure"]
impl crate::Readable for FRBH0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FRBH0 to value 0"]
impl crate::Resettable for FRBH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
