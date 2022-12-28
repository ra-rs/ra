#[doc = "Register `PW10VRL` reader"]
pub struct R(crate::R<PW10VRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PW10VRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PW10VRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PW10VRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PW10VRL` reader - These bits are for reading the lower-order 32 bits of the positive gradient value."]
pub type PW10VRL_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - These bits are for reading the lower-order 32 bits of the positive gradient value."]
    #[inline(always)]
    pub fn pw10vrl(&self) -> PW10VRL_R {
        PW10VRL_R::new(self.bits)
    }
}
#[doc = "Positive Gradient Worst 10 Value Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pw10vrl](index.html) module"]
pub struct PW10VRL_SPEC;
impl crate::RegisterSpec for PW10VRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pw10vrl::R](R) reader structure"]
impl crate::Readable for PW10VRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PW10VRL to value 0"]
impl crate::Resettable for PW10VRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
