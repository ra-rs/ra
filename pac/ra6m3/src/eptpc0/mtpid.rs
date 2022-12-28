#[doc = "Register `MTPID` reader"]
pub struct R(crate::R<MTPID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTPID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTPID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTPID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTPID` writer"]
pub struct W(crate::W<MTPID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTPID_SPEC>;
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
impl From<crate::W<MTPID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTPID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PNUM` reader - Master Clock Port Number SettingThese bits hold the setting for the port number of the master clock."]
pub type PNUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PNUM` writer - Master Clock Port Number SettingThese bits hold the setting for the port number of the master clock."]
pub type PNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTPID_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Master Clock Port Number SettingThese bits hold the setting for the port number of the master clock."]
    #[inline(always)]
    pub fn pnum(&self) -> PNUM_R {
        PNUM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Master Clock Port Number SettingThese bits hold the setting for the port number of the master clock."]
    #[inline(always)]
    #[must_use]
    pub fn pnum(&mut self) -> PNUM_W<0> {
        PNUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master clock port number register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtpid](index.html) module"]
pub struct MTPID_SPEC;
impl crate::RegisterSpec for MTPID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtpid::R](R) reader structure"]
impl crate::Readable for MTPID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtpid::W](W) writer structure"]
impl crate::Writable for MTPID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTPID to value 0"]
impl crate::Resettable for MTPID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
