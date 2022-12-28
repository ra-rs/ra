#[doc = "Register `PDMACRU` reader"]
pub struct R(crate::R<PDMACRU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMACRU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMACRU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMACRU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMACRU` writer"]
pub struct W(crate::W<PDMACRU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMACRU_SPEC>;
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
impl From<crate::W<PDMACRU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMACRU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDMACRU` reader - These bits hold the setting for the higher-order 24 bits of the destination MAC address for PTP-pdelay messages."]
pub type PDMACRU_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PDMACRU` writer - These bits hold the setting for the higher-order 24 bits of the destination MAC address for PTP-pdelay messages."]
pub type PDMACRU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDMACRU_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - These bits hold the setting for the higher-order 24 bits of the destination MAC address for PTP-pdelay messages."]
    #[inline(always)]
    pub fn pdmacru(&self) -> PDMACRU_R {
        PDMACRU_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - These bits hold the setting for the higher-order 24 bits of the destination MAC address for PTP-pdelay messages."]
    #[inline(always)]
    #[must_use]
    pub fn pdmacru(&mut self) -> PDMACRU_W<0> {
        PDMACRU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PTP-pdelay Message MAC Address Setting Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdmacru](index.html) module"]
pub struct PDMACRU_SPEC;
impl crate::RegisterSpec for PDMACRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdmacru::R](R) reader structure"]
impl crate::Readable for PDMACRU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdmacru::W](W) writer structure"]
impl crate::Writable for PDMACRU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDMACRU to value 0x0001_80c2"]
impl crate::Resettable for PDMACRU_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_80c2;
}
