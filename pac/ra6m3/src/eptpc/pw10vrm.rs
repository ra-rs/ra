#[doc = "Register `PW10VRM` reader"]
pub struct R(crate::R<PW10VRM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PW10VRM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PW10VRM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PW10VRM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PW10VRM` reader - These bits are for reading the middle-order 32 bits of the positive gradient value."]
pub type PW10VRM_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - These bits are for reading the middle-order 32 bits of the positive gradient value."]
    #[inline(always)]
    pub fn pw10vrm(&self) -> PW10VRM_R {
        PW10VRM_R::new(self.bits)
    }
}
#[doc = "Positive Gradient Worst 10 Value Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pw10vrm](index.html) module"]
pub struct PW10VRM_SPEC;
impl crate::RegisterSpec for PW10VRM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pw10vrm::R](R) reader structure"]
impl crate::Readable for PW10VRM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PW10VRM to value 0"]
impl crate::Resettable for PW10VRM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
