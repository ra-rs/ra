#[doc = "Register `CFSAMONB` reader"]
pub struct R(crate::R<CFSAMONB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFSAMONB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFSAMONB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFSAMONB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CFS1` reader - Code Flash Secure area 1"]
pub type CFS1_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 10:23 - Code Flash Secure area 1"]
    #[inline(always)]
    pub fn cfs1(&self) -> CFS1_R {
        CFS1_R::new(((self.bits >> 10) & 0x3fff) as u16)
    }
}
#[doc = "Code Flash Security Attribution Monitor Register B\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfsamonb](index.html) module"]
pub struct CFSAMONB_SPEC;
impl crate::RegisterSpec for CFSAMONB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfsamonb::R](R) reader structure"]
impl crate::Readable for CFSAMONB_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFSAMONB to value 0"]
impl crate::Resettable for CFSAMONB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
