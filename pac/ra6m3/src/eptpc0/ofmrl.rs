#[doc = "Register `OFMRL` reader"]
pub struct R(crate::R<OFMRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFMRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFMRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFMRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OFMRL` reader - These bits indicate the lower-order 32 bits of the calculated offsetFromMaster value."]
pub type OFMRL_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - These bits indicate the lower-order 32 bits of the calculated offsetFromMaster value."]
    #[inline(always)]
    pub fn ofmrl(&self) -> OFMRL_R {
        OFMRL_R::new(self.bits)
    }
}
#[doc = "offsetFromMaster Value Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofmrl](index.html) module"]
pub struct OFMRL_SPEC;
impl crate::RegisterSpec for OFMRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ofmrl::R](R) reader structure"]
impl crate::Readable for OFMRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OFMRL to value 0"]
impl crate::Resettable for OFMRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
