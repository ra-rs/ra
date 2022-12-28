#[doc = "Register `DVCT` reader"]
pub struct R(crate::R<DVCT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVCT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVCT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVCT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IDX` reader - DCT Table Index"]
pub type IDX_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 19:23 - DCT Table Index"]
    #[inline(always)]
    pub fn idx(&self) -> IDX_R {
        IDX_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
#[doc = "Device Characteristic Table Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvct](index.html) module"]
pub struct DVCT_SPEC;
impl crate::RegisterSpec for DVCT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dvct::R](R) reader structure"]
impl crate::Readable for DVCT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DVCT to value 0"]
impl crate::Resettable for DVCT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
