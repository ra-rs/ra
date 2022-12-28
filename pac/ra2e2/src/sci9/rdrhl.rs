#[doc = "Register `RDRHL` reader"]
pub struct R(crate::R<RDRHL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDRHL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDRHL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDRHL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDAT` reader - Serial Receive Data"]
pub type RDAT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:8 - Serial Receive Data"]
    #[inline(always)]
    pub fn rdat(&self) -> RDAT_R {
        RDAT_R::new(self.bits & 0x01ff)
    }
}
#[doc = "Receive Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdrhl](index.html) module"]
pub struct RDRHL_SPEC;
impl crate::RegisterSpec for RDRHL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rdrhl::R](R) reader structure"]
impl crate::Readable for RDRHL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RDRHL to value 0"]
impl crate::Resettable for RDRHL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
