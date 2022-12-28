#[doc = "Register `RMINCP%s` reader"]
pub struct R(crate::R<RMINCP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMINCP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMINCP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMINCP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MIN1` reader - 1-Minute Capture"]
pub type MIN1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIN10` reader - 10-Minute Capture"]
pub type MIN10_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - 1-Minute Capture"]
    #[inline(always)]
    pub fn min1(&self) -> MIN1_R {
        MIN1_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - 10-Minute Capture"]
    #[inline(always)]
    pub fn min10(&self) -> MIN10_R {
        MIN10_R::new((self.bits >> 4) & 7)
    }
}
#[doc = "Minute Capture Register %s\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmincp](index.html) module"]
pub struct RMINCP_SPEC;
impl crate::RegisterSpec for RMINCP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rmincp::R](R) reader structure"]
impl crate::Readable for RMINCP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RMINCP%s to value 0"]
impl crate::Resettable for RMINCP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
