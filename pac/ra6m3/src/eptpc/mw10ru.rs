#[doc = "Register `MW10RU` reader"]
pub struct R(crate::R<MW10RU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MW10RU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MW10RU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MW10RU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MW10RU` reader - These bits are for reading the higher-order 32 bits of the negative gradient value."]
pub type MW10RU_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - These bits are for reading the higher-order 32 bits of the negative gradient value."]
    #[inline(always)]
    pub fn mw10ru(&self) -> MW10RU_R {
        MW10RU_R::new(self.bits)
    }
}
#[doc = "Negative Gradient Worst 10 Value Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mw10ru](index.html) module"]
pub struct MW10RU_SPEC;
impl crate::RegisterSpec for MW10RU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mw10ru::R](R) reader structure"]
impl crate::Readable for MW10RU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MW10RU to value 0"]
impl crate::Resettable for MW10RU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
