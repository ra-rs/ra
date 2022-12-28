#[doc = "Register `RMONCP0` reader"]
pub struct R(crate::R<RMONCP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMONCP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMONCP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMONCP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MON1` reader - 1-Month Capture"]
pub type MON1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MON10` reader - 10-Month Capture"]
pub type MON10_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - 1-Month Capture"]
    #[inline(always)]
    pub fn mon1(&self) -> MON1_R {
        MON1_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - 10-Month Capture"]
    #[inline(always)]
    pub fn mon10(&self) -> MON10_R {
        MON10_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Month Capture Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmoncp0](index.html) module"]
pub struct RMONCP0_SPEC;
impl crate::RegisterSpec for RMONCP0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rmoncp0::R](R) reader structure"]
impl crate::Readable for RMONCP0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RMONCP0 to value 0"]
impl crate::Resettable for RMONCP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
