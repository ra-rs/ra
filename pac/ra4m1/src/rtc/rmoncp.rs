#[doc = "Register `RMONCP%s` reader"]
pub struct R(crate::R<RMONCP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMONCP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMONCP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMONCP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MON1` reader - 1-Month Capture Capture value for the ones place of months"]
pub type MON1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MON10` reader - 10-Month Capture Capture value for the tens place of months"]
pub type MON10_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - 1-Month Capture Capture value for the ones place of months"]
    #[inline(always)]
    pub fn mon1(&self) -> MON1_R {
        MON1_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - 10-Month Capture Capture value for the tens place of months"]
    #[inline(always)]
    pub fn mon10(&self) -> MON10_R {
        MON10_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Month Capture Register %s\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmoncp](index.html) module"]
pub struct RMONCP_SPEC;
impl crate::RegisterSpec for RMONCP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rmoncp::R](R) reader structure"]
impl crate::Readable for RMONCP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RMONCP%s to value 0"]
impl crate::Resettable for RMONCP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
