#[doc = "Register `CUOTSR` reader"]
pub struct R(crate::R<CUOTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CUOTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CUOTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CUOTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CUOTSR` writer"]
pub struct W(crate::W<CUOTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CUOTSR_SPEC>;
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
impl From<crate::W<CUOTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CUOTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSRC` reader - timeSource Field SettingThese bits set the value of the timeSource fields of Announce messages."]
pub type TSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSRC` writer - timeSource Field SettingThese bits set the value of the timeSource fields of Announce messages."]
pub type TSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CUOTSR_SPEC, u8, u8, 8, O>;
#[doc = "Field `CUTO` reader - currentUtcOffset Field SettingThese bits set the value of the currentUtcOffset fields of Announce messages."]
pub type CUTO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CUTO` writer - currentUtcOffset Field SettingThese bits set the value of the currentUtcOffset fields of Announce messages."]
pub type CUTO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CUOTSR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:7 - timeSource Field SettingThese bits set the value of the timeSource fields of Announce messages."]
    #[inline(always)]
    pub fn tsrc(&self) -> TSRC_R {
        TSRC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - currentUtcOffset Field SettingThese bits set the value of the currentUtcOffset fields of Announce messages."]
    #[inline(always)]
    pub fn cuto(&self) -> CUTO_R {
        CUTO_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - timeSource Field SettingThese bits set the value of the timeSource fields of Announce messages."]
    #[inline(always)]
    #[must_use]
    pub fn tsrc(&mut self) -> TSRC_W<0> {
        TSRC_W::new(self)
    }
    #[doc = "Bits 16:31 - currentUtcOffset Field SettingThese bits set the value of the currentUtcOffset fields of Announce messages."]
    #[inline(always)]
    #[must_use]
    pub fn cuto(&mut self) -> CUTO_W<16> {
        CUTO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "currentUtcOffset/timeSource Field Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cuotsr](index.html) module"]
pub struct CUOTSR_SPEC;
impl crate::RegisterSpec for CUOTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cuotsr::R](R) reader structure"]
impl crate::Readable for CUOTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cuotsr::W](W) writer structure"]
impl crate::Writable for CUOTSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CUOTSR to value 0"]
impl crate::Resettable for CUOTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
