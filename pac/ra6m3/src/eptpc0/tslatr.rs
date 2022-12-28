#[doc = "Register `TSLATR` reader"]
pub struct R(crate::R<TSLATR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSLATR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSLATR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSLATR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSLATR` writer"]
pub struct W(crate::W<TSLATR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSLATR_SPEC>;
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
impl From<crate::W<TSLATR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSLATR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EGP` reader - Input Port Timestamp Latency SettingThese bits hold the setting for the time stamp latency (ns) for the input ports."]
pub type EGP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EGP` writer - Input Port Timestamp Latency SettingThese bits hold the setting for the time stamp latency (ns) for the input ports."]
pub type EGP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSLATR_SPEC, u16, u16, 16, O>;
#[doc = "Field `INGP` reader - Output Port Timestamp Latency SettingThese bits hold the setting for the time stamp latency (ns) for the output ports."]
pub type INGP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INGP` writer - Output Port Timestamp Latency SettingThese bits hold the setting for the time stamp latency (ns) for the output ports."]
pub type INGP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSLATR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Input Port Timestamp Latency SettingThese bits hold the setting for the time stamp latency (ns) for the input ports."]
    #[inline(always)]
    pub fn egp(&self) -> EGP_R {
        EGP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Output Port Timestamp Latency SettingThese bits hold the setting for the time stamp latency (ns) for the output ports."]
    #[inline(always)]
    pub fn ingp(&self) -> INGP_R {
        INGP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Input Port Timestamp Latency SettingThese bits hold the setting for the time stamp latency (ns) for the input ports."]
    #[inline(always)]
    #[must_use]
    pub fn egp(&mut self) -> EGP_W<0> {
        EGP_W::new(self)
    }
    #[doc = "Bits 16:31 - Output Port Timestamp Latency SettingThese bits hold the setting for the time stamp latency (ns) for the output ports."]
    #[inline(always)]
    #[must_use]
    pub fn ingp(&mut self) -> INGP_W<16> {
        INGP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timestamp Latency Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tslatr](index.html) module"]
pub struct TSLATR_SPEC;
impl crate::RegisterSpec for TSLATR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tslatr::R](R) reader structure"]
impl crate::Readable for TSLATR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tslatr::W](W) writer structure"]
impl crate::Writable for TSLATR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSLATR to value 0"]
impl crate::Resettable for TSLATR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
