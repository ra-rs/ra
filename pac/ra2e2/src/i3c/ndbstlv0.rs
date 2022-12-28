#[doc = "Register `NDBSTLV0` reader"]
pub struct R(crate::R<NDBSTLV0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NDBSTLV0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NDBSTLV0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NDBSTLV0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TDBFLV` reader - Normal Transmit Data Buffer Free Level"]
pub type TDBFLV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDBLV` reader - Normal Receive Data Buffer Level"]
pub type RDBLV_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Normal Transmit Data Buffer Free Level"]
    #[inline(always)]
    pub fn tdbflv(&self) -> TDBFLV_R {
        TDBFLV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Normal Receive Data Buffer Level"]
    #[inline(always)]
    pub fn rdblv(&self) -> RDBLV_R {
        RDBLV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Normal Data Buffer Status Level Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ndbstlv0](index.html) module"]
pub struct NDBSTLV0_SPEC;
impl crate::RegisterSpec for NDBSTLV0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ndbstlv0::R](R) reader structure"]
impl crate::Readable for NDBSTLV0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NDBSTLV0 to value 0x01"]
impl crate::Resettable for NDBSTLV0_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
