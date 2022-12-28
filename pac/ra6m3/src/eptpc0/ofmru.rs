#[doc = "Register `OFMRU` reader"]
pub struct R(crate::R<OFMRU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFMRU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFMRU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFMRU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OFMRU` reader - These bits indicate the higher-order 32 bits of the calculated offsetFromMaster value."]
pub type OFMRU_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - These bits indicate the higher-order 32 bits of the calculated offsetFromMaster value."]
    #[inline(always)]
    pub fn ofmru(&self) -> OFMRU_R {
        OFMRU_R::new(self.bits)
    }
}
#[doc = "offsetFromMaster Value Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofmru](index.html) module"]
pub struct OFMRU_SPEC;
impl crate::RegisterSpec for OFMRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ofmru::R](R) reader structure"]
impl crate::Readable for OFMRU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OFMRU to value 0"]
impl crate::Resettable for OFMRU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
