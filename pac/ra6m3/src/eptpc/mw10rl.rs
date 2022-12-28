#[doc = "Register `MW10RL` reader"]
pub struct R(crate::R<MW10RL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MW10RL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MW10RL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MW10RL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MW10RL` reader - These bits are for reading the lower-order 32 bits of the negative gradient value."]
pub type MW10RL_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - These bits are for reading the lower-order 32 bits of the negative gradient value."]
    #[inline(always)]
    pub fn mw10rl(&self) -> MW10RL_R {
        MW10RL_R::new(self.bits)
    }
}
#[doc = "Negative Gradient Worst 10 Value Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mw10rl](index.html) module"]
pub struct MW10RL_SPEC;
impl crate::RegisterSpec for MW10RL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mw10rl::R](R) reader structure"]
impl crate::Readable for MW10RL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MW10RL to value 0"]
impl crate::Resettable for MW10RL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
