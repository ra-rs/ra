#[doc = "Register `PPMACRU` reader"]
pub struct R(crate::R<PPMACRU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPMACRU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPMACRU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPMACRU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPMACRU` writer"]
pub struct W(crate::W<PPMACRU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPMACRU_SPEC>;
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
impl From<crate::W<PPMACRU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPMACRU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PPMACRU` reader - These bits hold the setting for the higher-order 24 bits of the destination MAC address for PTP-primary messages."]
pub type PPMACRU_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PPMACRU` writer - These bits hold the setting for the higher-order 24 bits of the destination MAC address for PTP-primary messages."]
pub type PPMACRU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PPMACRU_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - These bits hold the setting for the higher-order 24 bits of the destination MAC address for PTP-primary messages."]
    #[inline(always)]
    pub fn ppmacru(&self) -> PPMACRU_R {
        PPMACRU_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - These bits hold the setting for the higher-order 24 bits of the destination MAC address for PTP-primary messages."]
    #[inline(always)]
    #[must_use]
    pub fn ppmacru(&mut self) -> PPMACRU_W<0> {
        PPMACRU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PTP-primary Message Destination MAC Address Setting Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppmacru](index.html) module"]
pub struct PPMACRU_SPEC;
impl crate::RegisterSpec for PPMACRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppmacru::R](R) reader structure"]
impl crate::Readable for PPMACRU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppmacru::W](W) writer structure"]
impl crate::Writable for PPMACRU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PPMACRU to value 0x0001_1b19"]
impl crate::Resettable for PPMACRU_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_1b19;
}
