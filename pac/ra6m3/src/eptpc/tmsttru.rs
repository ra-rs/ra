#[doc = "Register `TMSTTRU%s` reader"]
pub struct R(crate::R<TMSTTRU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMSTTRU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMSTTRU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMSTTRU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMSTTRU%s` writer"]
pub struct W(crate::W<TMSTTRU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMSTTRU_SPEC>;
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
impl From<crate::W<TMSTTRU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMSTTRU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMSTTRU` reader - These bits hold the setting for the higher-order 32 bits of the start time of the pulse output timer in nanoseconds."]
pub type TMSTTRU_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TMSTTRU` writer - These bits hold the setting for the higher-order 32 bits of the start time of the pulse output timer in nanoseconds."]
pub type TMSTTRU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TMSTTRU_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - These bits hold the setting for the higher-order 32 bits of the start time of the pulse output timer in nanoseconds."]
    #[inline(always)]
    pub fn tmsttru(&self) -> TMSTTRU_R {
        TMSTTRU_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits hold the setting for the higher-order 32 bits of the start time of the pulse output timer in nanoseconds."]
    #[inline(always)]
    #[must_use]
    pub fn tmsttru(&mut self) -> TMSTTRU_W<0> {
        TMSTTRU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Start Time Setting Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmsttru](index.html) module"]
pub struct TMSTTRU_SPEC;
impl crate::RegisterSpec for TMSTTRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmsttru::R](R) reader structure"]
impl crate::Readable for TMSTTRU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmsttru::W](W) writer structure"]
impl crate::Writable for TMSTTRU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMSTTRU%s to value 0"]
impl crate::Resettable for TMSTTRU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
