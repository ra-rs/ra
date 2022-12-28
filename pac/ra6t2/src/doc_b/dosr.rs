#[doc = "Register `DOSR` reader"]
pub struct R(crate::R<DOSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DOPCF` reader - Data Operation Circuit Flag"]
pub type DOPCF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Data Operation Circuit Flag"]
    #[inline(always)]
    pub fn dopcf(&self) -> DOPCF_R {
        DOPCF_R::new((self.bits & 1) != 0)
    }
}
#[doc = "DOC Flag Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dosr](index.html) module"]
pub struct DOSR_SPEC;
impl crate::RegisterSpec for DOSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dosr::R](R) reader structure"]
impl crate::Readable for DOSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DOSR to value 0"]
impl crate::Resettable for DOSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
