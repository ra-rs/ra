#[doc = "Register `PETYPER` reader"]
pub struct R(crate::R<PETYPER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PETYPER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PETYPER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PETYPER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PETYPER` writer"]
pub struct W(crate::W<PETYPER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PETYPER_SPEC>;
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
impl From<crate::W<PETYPER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PETYPER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TYPE` reader - PTP Message EtherType Value SettingThese bits hold the setting for the EtherType field value for frames in the Ethernet II format."]
pub type TYPE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TYPE` writer - PTP Message EtherType Value SettingThese bits hold the setting for the EtherType field value for frames in the Ethernet II format."]
pub type TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PETYPER_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - PTP Message EtherType Value SettingThese bits hold the setting for the EtherType field value for frames in the Ethernet II format."]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PTP Message EtherType Value SettingThese bits hold the setting for the EtherType field value for frames in the Ethernet II format."]
    #[inline(always)]
    #[must_use]
    pub fn type_(&mut self) -> TYPE_W<0> {
        TYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PTP Message EtherType Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [petyper](index.html) module"]
pub struct PETYPER_SPEC;
impl crate::RegisterSpec for PETYPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [petyper::R](R) reader structure"]
impl crate::Readable for PETYPER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [petyper::W](W) writer structure"]
impl crate::Writable for PETYPER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PETYPER to value 0x88f7"]
impl crate::Resettable for PETYPER_SPEC {
    const RESET_VALUE: Self::Ux = 0x88f7;
}
