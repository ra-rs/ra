#[doc = "Register `PSR` reader"]
pub struct R(crate::R<PSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LMON` reader - ETn_LINKSTA Pin Status FlagThe link status can be read by connecting the link signal output from the PHY-LSI to the ETn_LINKSTA pin. For details on the polarity, refer to the specifications of the connected PHY-LSI."]
pub type LMON_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - ETn_LINKSTA Pin Status FlagThe link status can be read by connecting the link signal output from the PHY-LSI to the ETn_LINKSTA pin. For details on the polarity, refer to the specifications of the connected PHY-LSI."]
    #[inline(always)]
    pub fn lmon(&self) -> LMON_R {
        LMON_R::new((self.bits & 1) != 0)
    }
}
#[doc = "PHY Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psr](index.html) module"]
pub struct PSR_SPEC;
impl crate::RegisterSpec for PSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psr::R](R) reader structure"]
impl crate::Readable for PSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PSR to value 0"]
impl crate::Resettable for PSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
