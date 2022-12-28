#[doc = "Register `DPUSR0R` reader"]
pub struct R(crate::R<DPUSR0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPUSR0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPUSR0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPUSR0R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DOVCAHM` reader - OVRCURA InputIndicates OVRCURA input signal on the HS side of USB port."]
pub type DOVCAHM_R = crate::BitReader<bool>;
#[doc = "Field `DOVCBHM` reader - OVRCURB InputIndicates OVRCURB input signal on the HS side of USB port."]
pub type DOVCBHM_R = crate::BitReader<bool>;
#[doc = "Field `DVBSTSHM` reader - VBUS InputIndicates VBUS input signal on the HS side of USB port."]
pub type DVBSTSHM_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 20 - OVRCURA InputIndicates OVRCURA input signal on the HS side of USB port."]
    #[inline(always)]
    pub fn dovcahm(&self) -> DOVCAHM_R {
        DOVCAHM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - OVRCURB InputIndicates OVRCURB input signal on the HS side of USB port."]
    #[inline(always)]
    pub fn dovcbhm(&self) -> DOVCBHM_R {
        DOVCBHM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - VBUS InputIndicates VBUS input signal on the HS side of USB port."]
    #[inline(always)]
    pub fn dvbstshm(&self) -> DVBSTSHM_R {
        DVBSTSHM_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "Deep Standby USB Transceiver Control/Pin Monitor Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpusr0r](index.html) module"]
pub struct DPUSR0R_SPEC;
impl crate::RegisterSpec for DPUSR0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dpusr0r::R](R) reader structure"]
impl crate::Readable for DPUSR0R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DPUSR0R to value 0"]
impl crate::Resettable for DPUSR0R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
