#[doc = "Register `ELSR%s` reader"]
pub struct R(crate::R<ELSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ELSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ELSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ELSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ELSR%s` writer"]
pub struct W(crate::W<ELSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ELSR_SPEC>;
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
impl From<crate::W<ELSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ELSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ELS` reader - Event Link Select"]
pub type ELS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ELS` writer - Event Link Select"]
pub type ELS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ELSR_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - Event Link Select"]
    #[inline(always)]
    pub fn els(&self) -> ELS_R {
        ELS_R::new(self.bits & 0x01ff)
    }
}
impl W {
    #[doc = "Bits 0:8 - Event Link Select"]
    #[inline(always)]
    #[must_use]
    pub fn els(&mut self) -> ELS_W<0> {
        ELS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Link Setting Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [elsr](index.html) module"]
pub struct ELSR_SPEC;
impl crate::RegisterSpec for ELSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [elsr::R](R) reader structure"]
impl crate::Readable for ELSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [elsr::W](W) writer structure"]
impl crate::Writable for ELSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ELSR%s to value 0"]
impl crate::Resettable for ELSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
