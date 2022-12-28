#[doc = "Register `PLIMITRL` reader"]
pub struct R(crate::R<PLIMITRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLIMITRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLIMITRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLIMITRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLIMITRL` writer"]
pub struct W(crate::W<PLIMITRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLIMITRL_SPEC>;
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
impl From<crate::W<PLIMITRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLIMITRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLIMITRL` reader - These bits hold the setting for the lower-order 32 bits of the limit for the positive gradient."]
pub type PLIMITRL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PLIMITRL` writer - These bits hold the setting for the lower-order 32 bits of the limit for the positive gradient."]
pub type PLIMITRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLIMITRL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the limit for the positive gradient."]
    #[inline(always)]
    pub fn plimitrl(&self) -> PLIMITRL_R {
        PLIMITRL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the limit for the positive gradient."]
    #[inline(always)]
    #[must_use]
    pub fn plimitrl(&mut self) -> PLIMITRL_W<0> {
        PLIMITRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Positive Gradient Limit Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plimitrl](index.html) module"]
pub struct PLIMITRL_SPEC;
impl crate::RegisterSpec for PLIMITRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [plimitrl::R](R) reader structure"]
impl crate::Readable for PLIMITRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [plimitrl::W](W) writer structure"]
impl crate::Writable for PLIMITRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLIMITRL to value 0"]
impl crate::Resettable for PLIMITRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
