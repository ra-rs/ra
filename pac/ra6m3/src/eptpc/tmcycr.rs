#[doc = "Register `TMCYCR%s` reader"]
pub struct R(crate::R<TMCYCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMCYCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMCYCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMCYCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMCYCR%s` writer"]
pub struct W(crate::W<TMCYCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMCYCR_SPEC>;
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
impl From<crate::W<TMCYCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMCYCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMCYCR` reader - These bits set the cycle of the pulse output timer in nanoseconds. Set a value that is equivalent to at least four cycles of the STCA clock."]
pub type TMCYCR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TMCYCR` writer - These bits set the cycle of the pulse output timer in nanoseconds. Set a value that is equivalent to at least four cycles of the STCA clock."]
pub type TMCYCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TMCYCR_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:29 - These bits set the cycle of the pulse output timer in nanoseconds. Set a value that is equivalent to at least four cycles of the STCA clock."]
    #[inline(always)]
    pub fn tmcycr(&self) -> TMCYCR_R {
        TMCYCR_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - These bits set the cycle of the pulse output timer in nanoseconds. Set a value that is equivalent to at least four cycles of the STCA clock."]
    #[inline(always)]
    #[must_use]
    pub fn tmcycr(&mut self) -> TMCYCR_W<0> {
        TMCYCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Cycle Setting Registers %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmcycr](index.html) module"]
pub struct TMCYCR_SPEC;
impl crate::RegisterSpec for TMCYCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmcycr::R](R) reader structure"]
impl crate::Readable for TMCYCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmcycr::W](W) writer structure"]
impl crate::Writable for TMCYCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMCYCR%s to value 0"]
impl crate::Resettable for TMCYCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
