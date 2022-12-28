#[doc = "Register `PPTTLR` reader"]
pub struct R(crate::R<PPTTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPTTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPTTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPTTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPTTLR` writer"]
pub struct W(crate::W<PPTTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPTTLR_SPEC>;
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
impl From<crate::W<PPTTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPTTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRTL` reader - PTP-primary Message TTL Field Value SettingThese bits hold the setting for the value of the TTL field within the IPv4 headers of PTP-primary messages."]
pub type PRTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRTL` writer - PTP-primary Message TTL Field Value SettingThese bits hold the setting for the value of the TTL field within the IPv4 headers of PTP-primary messages."]
pub type PRTL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PPTTLR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - PTP-primary Message TTL Field Value SettingThese bits hold the setting for the value of the TTL field within the IPv4 headers of PTP-primary messages."]
    #[inline(always)]
    pub fn prtl(&self) -> PRTL_R {
        PRTL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PTP-primary Message TTL Field Value SettingThese bits hold the setting for the value of the TTL field within the IPv4 headers of PTP-primary messages."]
    #[inline(always)]
    #[must_use]
    pub fn prtl(&mut self) -> PRTL_W<0> {
        PRTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PTP-primary Message TTL Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppttlr](index.html) module"]
pub struct PPTTLR_SPEC;
impl crate::RegisterSpec for PPTTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppttlr::R](R) reader structure"]
impl crate::Readable for PPTTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppttlr::W](W) writer structure"]
impl crate::Writable for PPTTLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PPTTLR to value 0x80"]
impl crate::Resettable for PPTTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
