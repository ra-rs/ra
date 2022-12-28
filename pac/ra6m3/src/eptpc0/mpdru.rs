#[doc = "Register `MPDRU` reader"]
pub struct R(crate::R<MPDRU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPDRU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPDRU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPDRU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MPDRU` reader - These bits indicate the higher-order 32 bits of the calculated meanPathDelay value."]
pub type MPDRU_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - These bits indicate the higher-order 32 bits of the calculated meanPathDelay value."]
    #[inline(always)]
    pub fn mpdru(&self) -> MPDRU_R {
        MPDRU_R::new(self.bits)
    }
}
#[doc = "meanPathDelay Value Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpdru](index.html) module"]
pub struct MPDRU_SPEC;
impl crate::RegisterSpec for MPDRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpdru::R](R) reader structure"]
impl crate::Readable for MPDRU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MPDRU to value 0"]
impl crate::Resettable for MPDRU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
