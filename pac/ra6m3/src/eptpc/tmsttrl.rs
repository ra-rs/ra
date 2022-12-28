#[doc = "Register `TMSTTRL%s` reader"]
pub struct R(crate::R<TMSTTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMSTTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMSTTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMSTTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMSTTRL%s` writer"]
pub struct W(crate::W<TMSTTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMSTTRL_SPEC>;
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
impl From<crate::W<TMSTTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMSTTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMSTTRL` reader - These bits hold the setting for the lower-order 32 bits of the start time of the pulse output timer in nanoseconds."]
pub type TMSTTRL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TMSTTRL` writer - These bits hold the setting for the lower-order 32 bits of the start time of the pulse output timer in nanoseconds."]
pub type TMSTTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TMSTTRL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the start time of the pulse output timer in nanoseconds."]
    #[inline(always)]
    pub fn tmsttrl(&self) -> TMSTTRL_R {
        TMSTTRL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the start time of the pulse output timer in nanoseconds."]
    #[inline(always)]
    #[must_use]
    pub fn tmsttrl(&mut self) -> TMSTTRL_W<0> {
        TMSTTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Start Time Setting Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmsttrl](index.html) module"]
pub struct TMSTTRL_SPEC;
impl crate::RegisterSpec for TMSTTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmsttrl::R](R) reader structure"]
impl crate::Readable for TMSTTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmsttrl::W](W) writer structure"]
impl crate::Writable for TMSTTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMSTTRL%s to value 0"]
impl crate::Resettable for TMSTTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
