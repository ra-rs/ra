#[doc = "Register `FRBL0` reader"]
pub struct R(crate::R<FRBL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRBL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRBL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRBL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDATA` reader - Flash Read Buffer L0"]
pub type RDATA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Flash Read Buffer L0"]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new(self.bits)
    }
}
#[doc = "Flash Read Buffer Register L0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frbl0](index.html) module"]
pub struct FRBL0_SPEC;
impl crate::RegisterSpec for FRBL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [frbl0::R](R) reader structure"]
impl crate::Readable for FRBL0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FRBL0 to value 0"]
impl crate::Resettable for FRBL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
