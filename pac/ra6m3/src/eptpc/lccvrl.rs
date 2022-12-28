#[doc = "Register `LCCVRL` reader"]
pub struct R(crate::R<LCCVRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCCVRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCCVRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCCVRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LCCVRL` reader - These bits are for reading the fractional portion of the local timer counter's value (in nanoseconds)."]
pub type LCCVRL_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - These bits are for reading the fractional portion of the local timer counter's value (in nanoseconds)."]
    #[inline(always)]
    pub fn lccvrl(&self) -> LCCVRL_R {
        LCCVRL_R::new(self.bits)
    }
}
#[doc = "Local Time Counters\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lccvrl](index.html) module"]
pub struct LCCVRL_SPEC;
impl crate::RegisterSpec for LCCVRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lccvrl::R](R) reader structure"]
impl crate::Readable for LCCVRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LCCVRL to value 0"]
impl crate::Resettable for LCCVRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
