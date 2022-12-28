#[doc = "Register `PW10VRU` reader"]
pub struct R(crate::R<PW10VRU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PW10VRU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PW10VRU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PW10VRU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PW10VRU` reader - These bits are for reading the higher-order 32 bits of the positive gradient value."]
pub type PW10VRU_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - These bits are for reading the higher-order 32 bits of the positive gradient value."]
    #[inline(always)]
    pub fn pw10vru(&self) -> PW10VRU_R {
        PW10VRU_R::new(self.bits)
    }
}
#[doc = "Positive Gradient Worst 10 Value Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pw10vru](index.html) module"]
pub struct PW10VRU_SPEC;
impl crate::RegisterSpec for PW10VRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pw10vru::R](R) reader structure"]
impl crate::Readable for PW10VRU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PW10VRU to value 0"]
impl crate::Resettable for PW10VRU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
