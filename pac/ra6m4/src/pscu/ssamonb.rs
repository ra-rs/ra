#[doc = "Register `SSAMONB` reader"]
pub struct R(crate::R<SSAMONB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSAMONB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSAMONB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSAMONB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SS1` reader - SRAM secure area 1"]
pub type SS1_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 10:20 - SRAM secure area 1"]
    #[inline(always)]
    pub fn ss1(&self) -> SS1_R {
        SS1_R::new(((self.bits >> 10) & 0x07ff) as u16)
    }
}
#[doc = "SRAM Security Attribution Monitor Register B\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssamonb](index.html) module"]
pub struct SSAMONB_SPEC;
impl crate::RegisterSpec for SSAMONB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssamonb::R](R) reader structure"]
impl crate::Readable for SSAMONB_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SSAMONB to value 0"]
impl crate::Resettable for SSAMONB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
