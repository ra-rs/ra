#[doc = "Register `PDTTLR` reader"]
pub struct R(crate::R<PDTTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDTTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDTTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDTTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDTTLR` writer"]
pub struct W(crate::W<PDTTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDTTLR_SPEC>;
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
impl From<crate::W<PDTTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDTTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDTL` reader - PTP-pdelay Message TTL Field ValueThese bits hold the setting for the value of the TTL field within the IPv4 headers of PTP-pdelay messages."]
pub type PDTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PDTL` writer - PTP-pdelay Message TTL Field ValueThese bits hold the setting for the value of the TTL field within the IPv4 headers of PTP-pdelay messages."]
pub type PDTL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDTTLR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - PTP-pdelay Message TTL Field ValueThese bits hold the setting for the value of the TTL field within the IPv4 headers of PTP-pdelay messages."]
    #[inline(always)]
    pub fn pdtl(&self) -> PDTL_R {
        PDTL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PTP-pdelay Message TTL Field ValueThese bits hold the setting for the value of the TTL field within the IPv4 headers of PTP-pdelay messages."]
    #[inline(always)]
    #[must_use]
    pub fn pdtl(&mut self) -> PDTL_W<0> {
        PDTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PTP-pdelay Message TTL Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdttlr](index.html) module"]
pub struct PDTTLR_SPEC;
impl crate::RegisterSpec for PDTTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdttlr::R](R) reader structure"]
impl crate::Readable for PDTTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdttlr::W](W) writer structure"]
impl crate::Writable for PDTTLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDTTLR to value 0x01"]
impl crate::Resettable for PDTTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
