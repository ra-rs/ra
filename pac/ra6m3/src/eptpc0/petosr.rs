#[doc = "Register `PETOSR` reader"]
pub struct R(crate::R<PETOSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PETOSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PETOSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PETOSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PETOSR` writer"]
pub struct W(crate::W<PETOSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PETOSR_SPEC>;
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
impl From<crate::W<PETOSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PETOSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVTO` reader - PTP Event Message TOS Field Value SettingThese bits hold the setting for the value of the TOS field within the IPv4 headers of PTP event messages."]
pub type EVTO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EVTO` writer - PTP Event Message TOS Field Value SettingThese bits hold the setting for the value of the TOS field within the IPv4 headers of PTP event messages."]
pub type EVTO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PETOSR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - PTP Event Message TOS Field Value SettingThese bits hold the setting for the value of the TOS field within the IPv4 headers of PTP event messages."]
    #[inline(always)]
    pub fn evto(&self) -> EVTO_R {
        EVTO_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PTP Event Message TOS Field Value SettingThese bits hold the setting for the value of the TOS field within the IPv4 headers of PTP event messages."]
    #[inline(always)]
    #[must_use]
    pub fn evto(&mut self) -> EVTO_W<0> {
        EVTO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PTP Event Message TOS Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [petosr](index.html) module"]
pub struct PETOSR_SPEC;
impl crate::RegisterSpec for PETOSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [petosr::R](R) reader structure"]
impl crate::Readable for PETOSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [petosr::W](W) writer structure"]
impl crate::Writable for PETOSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PETOSR to value 0"]
impl crate::Resettable for PETOSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
