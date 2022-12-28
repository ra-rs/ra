#[doc = "Register `PPMACRL` reader"]
pub struct R(crate::R<PPMACRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPMACRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPMACRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPMACRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPMACRL` writer"]
pub struct W(crate::W<PPMACRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPMACRL_SPEC>;
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
impl From<crate::W<PPMACRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPMACRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PPMACRL` reader - These bits hold the setting for the lower-order 24 bits of the destination MAC address for PTP-primary messages."]
pub type PPMACRL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PPMACRL` writer - These bits hold the setting for the lower-order 24 bits of the destination MAC address for PTP-primary messages."]
pub type PPMACRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PPMACRL_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - These bits hold the setting for the lower-order 24 bits of the destination MAC address for PTP-primary messages."]
    #[inline(always)]
    pub fn ppmacrl(&self) -> PPMACRL_R {
        PPMACRL_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - These bits hold the setting for the lower-order 24 bits of the destination MAC address for PTP-primary messages."]
    #[inline(always)]
    #[must_use]
    pub fn ppmacrl(&mut self) -> PPMACRL_W<0> {
        PPMACRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PTP-primary Message Destination MAC Address Setting Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppmacrl](index.html) module"]
pub struct PPMACRL_SPEC;
impl crate::RegisterSpec for PPMACRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppmacrl::R](R) reader structure"]
impl crate::Readable for PPMACRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppmacrl::W](W) writer structure"]
impl crate::Writable for PPMACRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PPMACRL to value 0"]
impl crate::Resettable for PPMACRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
