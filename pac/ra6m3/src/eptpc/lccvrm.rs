#[doc = "Register `LCCVRM` reader"]
pub struct R(crate::R<LCCVRM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCCVRM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCCVRM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCCVRM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LCCVRM` reader - These bits are for reading the lower-order 32 bits of the integer portion of the local timer counter's value."]
pub type LCCVRM_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - These bits are for reading the lower-order 32 bits of the integer portion of the local timer counter's value."]
    #[inline(always)]
    pub fn lccvrm(&self) -> LCCVRM_R {
        LCCVRM_R::new(self.bits)
    }
}
#[doc = "Local Time Counters\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lccvrm](index.html) module"]
pub struct LCCVRM_SPEC;
impl crate::RegisterSpec for LCCVRM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lccvrm::R](R) reader structure"]
impl crate::Readable for LCCVRM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LCCVRM to value 0"]
impl crate::Resettable for LCCVRM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
