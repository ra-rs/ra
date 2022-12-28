#[doc = "Register `FAWSMR` reader"]
pub struct R(crate::R<FAWSMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FAWSMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FAWSMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FAWSMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FAWS` reader - Access Window Start Address"]
pub type FAWS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FSPR` reader - Access Window Protection Flag"]
pub type FSPR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:10 - Access Window Start Address"]
    #[inline(always)]
    pub fn faws(&self) -> FAWS_R {
        FAWS_R::new(self.bits & 0x07ff)
    }
    #[doc = "Bit 15 - Access Window Protection Flag"]
    #[inline(always)]
    pub fn fspr(&self) -> FSPR_R {
        FSPR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Flash Access Window Start Address Monitor Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fawsmr](index.html) module"]
pub struct FAWSMR_SPEC;
impl crate::RegisterSpec for FAWSMR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fawsmr::R](R) reader structure"]
impl crate::Readable for FAWSMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FAWSMR to value 0"]
impl crate::Resettable for FAWSMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
