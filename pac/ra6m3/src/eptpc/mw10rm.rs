#[doc = "Register `MW10RM` reader"]
pub struct R(crate::R<MW10RM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MW10RM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MW10RM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MW10RM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MW10RM` reader - These bits are for reading the middle-order 32 bits of the negative gradient value."]
pub type MW10RM_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - These bits are for reading the middle-order 32 bits of the negative gradient value."]
    #[inline(always)]
    pub fn mw10rm(&self) -> MW10RM_R {
        MW10RM_R::new(self.bits)
    }
}
#[doc = "Negative Gradient Worst 10 Value Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mw10rm](index.html) module"]
pub struct MW10RM_SPEC;
impl crate::RegisterSpec for MW10RM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mw10rm::R](R) reader structure"]
impl crate::Readable for MW10RM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MW10RM to value 0"]
impl crate::Resettable for MW10RM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
