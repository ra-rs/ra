#[doc = "Register `TMPLSR%s` reader"]
pub struct R(crate::R<TMPLSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMPLSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMPLSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMPLSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMPLSR%s` writer"]
pub struct W(crate::W<TMPLSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMPLSR_SPEC>;
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
impl From<crate::W<TMPLSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMPLSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMPLSR` reader - These bits set the width at high level of the pulse signal from the timer in nanoseconds. Set a value that is equivalent to at least two cycles of the STCA clock."]
pub type TMPLSR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TMPLSR` writer - These bits set the width at high level of the pulse signal from the timer in nanoseconds. Set a value that is equivalent to at least two cycles of the STCA clock."]
pub type TMPLSR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TMPLSR_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:28 - These bits set the width at high level of the pulse signal from the timer in nanoseconds. Set a value that is equivalent to at least two cycles of the STCA clock."]
    #[inline(always)]
    pub fn tmplsr(&self) -> TMPLSR_R {
        TMPLSR_R::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - These bits set the width at high level of the pulse signal from the timer in nanoseconds. Set a value that is equivalent to at least two cycles of the STCA clock."]
    #[inline(always)]
    #[must_use]
    pub fn tmplsr(&mut self) -> TMPLSR_W<0> {
        TMPLSR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Pulse Width Setting Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmplsr](index.html) module"]
pub struct TMPLSR_SPEC;
impl crate::RegisterSpec for TMPLSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmplsr::R](R) reader structure"]
impl crate::Readable for TMPLSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmplsr::W](W) writer structure"]
impl crate::Writable for TMPLSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMPLSR%s to value 0"]
impl crate::Resettable for TMPLSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
