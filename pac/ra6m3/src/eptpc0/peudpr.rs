#[doc = "Register `PEUDPR` reader"]
pub struct R(crate::R<PEUDPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEUDPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEUDPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEUDPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEUDPR` writer"]
pub struct W(crate::W<PEUDPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEUDPR_SPEC>;
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
impl From<crate::W<PEUDPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEUDPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVUPT` reader - PTP Event Message Destination Port Number SettingThese bits hold the setting for the value of the destination port number field within the UDP headers of PTP event messages."]
pub type EVUPT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EVUPT` writer - PTP Event Message Destination Port Number SettingThese bits hold the setting for the value of the destination port number field within the UDP headers of PTP event messages."]
pub type EVUPT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PEUDPR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - PTP Event Message Destination Port Number SettingThese bits hold the setting for the value of the destination port number field within the UDP headers of PTP event messages."]
    #[inline(always)]
    pub fn evupt(&self) -> EVUPT_R {
        EVUPT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PTP Event Message Destination Port Number SettingThese bits hold the setting for the value of the destination port number field within the UDP headers of PTP event messages."]
    #[inline(always)]
    #[must_use]
    pub fn evupt(&mut self) -> EVUPT_W<0> {
        EVUPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PTP Event Message UDP Destination Port Number Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peudpr](index.html) module"]
pub struct PEUDPR_SPEC;
impl crate::RegisterSpec for PEUDPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peudpr::R](R) reader structure"]
impl crate::Readable for PEUDPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peudpr::W](W) writer structure"]
impl crate::Writable for PEUDPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PEUDPR to value 0x013f"]
impl crate::Resettable for PEUDPR_SPEC {
    const RESET_VALUE: Self::Ux = 0x013f;
}
