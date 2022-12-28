#[doc = "Register `PGTOSR` reader"]
pub struct R(crate::R<PGTOSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PGTOSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PGTOSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PGTOSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PGTOSR` writer"]
pub struct W(crate::W<PGTOSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PGTOSR_SPEC>;
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
impl From<crate::W<PGTOSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PGTOSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GETO` reader - PTP general Message TOS Field Value SettingThese bits hold the setting for the value of the TOS field within the IPv4 headers of PTP general messages."]
pub type GETO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GETO` writer - PTP general Message TOS Field Value SettingThese bits hold the setting for the value of the TOS field within the IPv4 headers of PTP general messages."]
pub type GETO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PGTOSR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - PTP general Message TOS Field Value SettingThese bits hold the setting for the value of the TOS field within the IPv4 headers of PTP general messages."]
    #[inline(always)]
    pub fn geto(&self) -> GETO_R {
        GETO_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PTP general Message TOS Field Value SettingThese bits hold the setting for the value of the TOS field within the IPv4 headers of PTP general messages."]
    #[inline(always)]
    #[must_use]
    pub fn geto(&mut self) -> GETO_W<0> {
        GETO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PTP general Message TOS Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgtosr](index.html) module"]
pub struct PGTOSR_SPEC;
impl crate::RegisterSpec for PGTOSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pgtosr::R](R) reader structure"]
impl crate::Readable for PGTOSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pgtosr::W](W) writer structure"]
impl crate::Writable for PGTOSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PGTOSR to value 0"]
impl crate::Resettable for PGTOSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
