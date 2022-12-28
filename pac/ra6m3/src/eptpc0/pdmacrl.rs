#[doc = "Register `PDMACRL` reader"]
pub struct R(crate::R<PDMACRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMACRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMACRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMACRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMACRL` writer"]
pub struct W(crate::W<PDMACRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMACRL_SPEC>;
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
impl From<crate::W<PDMACRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMACRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDMACRL` reader - These bits hold the setting for the lower-order 24 bits of the destination MAC address for PTP-pdelay messages."]
pub type PDMACRL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PDMACRL` writer - These bits hold the setting for the lower-order 24 bits of the destination MAC address for PTP-pdelay messages."]
pub type PDMACRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDMACRL_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - These bits hold the setting for the lower-order 24 bits of the destination MAC address for PTP-pdelay messages."]
    #[inline(always)]
    pub fn pdmacrl(&self) -> PDMACRL_R {
        PDMACRL_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - These bits hold the setting for the lower-order 24 bits of the destination MAC address for PTP-pdelay messages."]
    #[inline(always)]
    #[must_use]
    pub fn pdmacrl(&mut self) -> PDMACRL_W<0> {
        PDMACRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PTP-pdelay Message MAC Address Setting Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdmacrl](index.html) module"]
pub struct PDMACRL_SPEC;
impl crate::RegisterSpec for PDMACRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdmacrl::R](R) reader structure"]
impl crate::Readable for PDMACRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdmacrl::W](W) writer structure"]
impl crate::Writable for PDMACRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDMACRL to value 0x0e"]
impl crate::Resettable for PDMACRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0e;
}
