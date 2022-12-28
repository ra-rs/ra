#[doc = "Register `SRR` reader"]
pub struct R(crate::R<SRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRR` writer"]
pub struct W(crate::W<SRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRR_SPEC>;
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
impl From<crate::W<SRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRMV` reader - stepsRemoved Field Value SettingThese bits set the value of the stepsRemoved fields of Announce messages."]
pub type SRMV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SRMV` writer - stepsRemoved Field Value SettingThese bits set the value of the stepsRemoved fields of Announce messages."]
pub type SRMV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - stepsRemoved Field Value SettingThese bits set the value of the stepsRemoved fields of Announce messages."]
    #[inline(always)]
    pub fn srmv(&self) -> SRMV_R {
        SRMV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - stepsRemoved Field Value SettingThese bits set the value of the stepsRemoved fields of Announce messages."]
    #[inline(always)]
    #[must_use]
    pub fn srmv(&mut self) -> SRMV_W<0> {
        SRMV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "stepsRemoved Field Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srr](index.html) module"]
pub struct SRR_SPEC;
impl crate::RegisterSpec for SRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srr::R](R) reader structure"]
impl crate::Readable for SRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srr::W](W) writer structure"]
impl crate::Writable for SRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRR to value 0"]
impl crate::Resettable for SRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
