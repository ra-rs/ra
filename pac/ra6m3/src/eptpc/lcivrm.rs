#[doc = "Register `LCIVRM` reader"]
pub struct R(crate::R<LCIVRM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCIVRM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCIVRM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCIVRM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCIVRM` writer"]
pub struct W(crate::W<LCIVRM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCIVRM_SPEC>;
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
impl From<crate::W<LCIVRM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCIVRM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCIVRM` reader - These bits hold the setting for the lower-order 32 bits of the integer portion of the initial value for the local timer counter."]
pub type LCIVRM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LCIVRM` writer - These bits hold the setting for the lower-order 32 bits of the integer portion of the initial value for the local timer counter."]
pub type LCIVRM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCIVRM_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the integer portion of the initial value for the local timer counter."]
    #[inline(always)]
    pub fn lcivrm(&self) -> LCIVRM_R {
        LCIVRM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the integer portion of the initial value for the local timer counter."]
    #[inline(always)]
    #[must_use]
    pub fn lcivrm(&mut self) -> LCIVRM_W<0> {
        LCIVRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Local Time Counter Initial Value Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcivrm](index.html) module"]
pub struct LCIVRM_SPEC;
impl crate::RegisterSpec for LCIVRM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcivrm::R](R) reader structure"]
impl crate::Readable for LCIVRM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcivrm::W](W) writer structure"]
impl crate::Writable for LCIVRM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCIVRM to value 0"]
impl crate::Resettable for LCIVRM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
