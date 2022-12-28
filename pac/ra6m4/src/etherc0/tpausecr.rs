#[doc = "Register `TPAUSECR` reader"]
pub struct R(crate::R<TPAUSECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPAUSECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPAUSECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPAUSECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXP` reader - PAUSE Frame Retransmit Count"]
pub type TXP_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - PAUSE Frame Retransmit Count"]
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "PAUSE Frame Retransmit Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpausecr](index.html) module"]
pub struct TPAUSECR_SPEC;
impl crate::RegisterSpec for TPAUSECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpausecr::R](R) reader structure"]
impl crate::Readable for TPAUSECR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TPAUSECR to value 0"]
impl crate::Resettable for TPAUSECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
