#[doc = "Register `MPDRL` reader"]
pub struct R(crate::R<MPDRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPDRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPDRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPDRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MPDRL` reader - These bits indicate the lower-order 32 bits of the calculated meanPathDelay value."]
pub type MPDRL_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - These bits indicate the lower-order 32 bits of the calculated meanPathDelay value."]
    #[inline(always)]
    pub fn mpdrl(&self) -> MPDRL_R {
        MPDRL_R::new(self.bits)
    }
}
#[doc = "meanPathDelay Value Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpdrl](index.html) module"]
pub struct MPDRL_SPEC;
impl crate::RegisterSpec for MPDRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpdrl::R](R) reader structure"]
impl crate::Readable for MPDRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MPDRL to value 0"]
impl crate::Resettable for MPDRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
