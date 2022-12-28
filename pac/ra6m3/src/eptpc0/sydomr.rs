#[doc = "Register `SYDOMR` reader"]
pub struct R(crate::R<SYDOMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYDOMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYDOMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYDOMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYDOMR` writer"]
pub struct W(crate::W<SYDOMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYDOMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SYDOMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYDOMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DNUM` reader - domainNumber Field Value SettingThese bits are used to set the domainNumber field value of the PTP v2 header.When a message is received, this value is compared with the domainNumber field of the received frame as a condition for PTP reception processing.In generating messages, the value is used for the domainNumber field of the frame for transmission."]
pub type DNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DNUM` writer - domainNumber Field Value SettingThese bits are used to set the domainNumber field value of the PTP v2 header.When a message is received, this value is compared with the domainNumber field of the received frame as a condition for PTP reception processing.In generating messages, the value is used for the domainNumber field of the frame for transmission."]
pub type DNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYDOMR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - domainNumber Field Value SettingThese bits are used to set the domainNumber field value of the PTP v2 header.When a message is received, this value is compared with the domainNumber field of the received frame as a condition for PTP reception processing.In generating messages, the value is used for the domainNumber field of the frame for transmission."]
    #[inline(always)]
    pub fn dnum(&self) -> DNUM_R {
        DNUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - domainNumber Field Value SettingThese bits are used to set the domainNumber field value of the PTP v2 header.When a message is received, this value is compared with the domainNumber field of the received frame as a condition for PTP reception processing.In generating messages, the value is used for the domainNumber field of the frame for transmission."]
    #[inline(always)]
    #[must_use]
    pub fn dnum(&mut self) -> DNUM_W<0> {
        DNUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYNFP Domain Number Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sydomr](index.html) module"]
pub struct SYDOMR_SPEC;
impl crate::RegisterSpec for SYDOMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sydomr::R](R) reader structure"]
impl crate::Readable for SYDOMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sydomr::W](W) writer structure"]
impl crate::Writable for SYDOMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYDOMR to value 0"]
impl crate::Resettable for SYDOMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
