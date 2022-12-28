#[doc = "Register `MLIMITRL` reader"]
pub struct R(crate::R<MLIMITRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MLIMITRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MLIMITRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MLIMITRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MLIMITRL` writer"]
pub struct W(crate::W<MLIMITRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MLIMITRL_SPEC>;
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
impl From<crate::W<MLIMITRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MLIMITRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MLIMITRL` reader - These bits hold the setting for the lower-order 32 bits of the limit for the negative gradient."]
pub type MLIMITRL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MLIMITRL` writer - These bits hold the setting for the lower-order 32 bits of the limit for the negative gradient."]
pub type MLIMITRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MLIMITRL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the limit for the negative gradient."]
    #[inline(always)]
    pub fn mlimitrl(&self) -> MLIMITRL_R {
        MLIMITRL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the limit for the negative gradient."]
    #[inline(always)]
    #[must_use]
    pub fn mlimitrl(&mut self) -> MLIMITRL_W<0> {
        MLIMITRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Negative Gradient Limit Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mlimitrl](index.html) module"]
pub struct MLIMITRL_SPEC;
impl crate::RegisterSpec for MLIMITRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mlimitrl::R](R) reader structure"]
impl crate::Readable for MLIMITRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mlimitrl::W](W) writer structure"]
impl crate::Writable for MLIMITRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MLIMITRL to value 0"]
impl crate::Resettable for MLIMITRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
