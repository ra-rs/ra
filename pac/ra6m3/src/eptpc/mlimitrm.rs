#[doc = "Register `MLIMITRM` reader"]
pub struct R(crate::R<MLIMITRM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MLIMITRM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MLIMITRM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MLIMITRM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MLIMITRM` writer"]
pub struct W(crate::W<MLIMITRM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MLIMITRM_SPEC>;
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
impl From<crate::W<MLIMITRM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MLIMITRM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MLIMITRM` reader - These bits hold the setting for the middle-order 32 bits of the limit for the negative gradient."]
pub type MLIMITRM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MLIMITRM` writer - These bits hold the setting for the middle-order 32 bits of the limit for the negative gradient."]
pub type MLIMITRM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MLIMITRM_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - These bits hold the setting for the middle-order 32 bits of the limit for the negative gradient."]
    #[inline(always)]
    pub fn mlimitrm(&self) -> MLIMITRM_R {
        MLIMITRM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits hold the setting for the middle-order 32 bits of the limit for the negative gradient."]
    #[inline(always)]
    #[must_use]
    pub fn mlimitrm(&mut self) -> MLIMITRM_W<0> {
        MLIMITRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Negative Gradient Limit Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mlimitrm](index.html) module"]
pub struct MLIMITRM_SPEC;
impl crate::RegisterSpec for MLIMITRM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mlimitrm::R](R) reader structure"]
impl crate::Readable for MLIMITRM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mlimitrm::W](W) writer structure"]
impl crate::Writable for MLIMITRM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MLIMITRM to value 0"]
impl crate::Resettable for MLIMITRM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
