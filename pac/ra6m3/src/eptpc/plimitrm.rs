#[doc = "Register `PLIMITRM` reader"]
pub struct R(crate::R<PLIMITRM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLIMITRM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLIMITRM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLIMITRM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLIMITRM` writer"]
pub struct W(crate::W<PLIMITRM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLIMITRM_SPEC>;
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
impl From<crate::W<PLIMITRM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLIMITRM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLIMITRM` reader - These bits hold the setting for the middle-order 32 bits of the limit for the positive gradient."]
pub type PLIMITRM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PLIMITRM` writer - These bits hold the setting for the middle-order 32 bits of the limit for the positive gradient."]
pub type PLIMITRM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLIMITRM_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - These bits hold the setting for the middle-order 32 bits of the limit for the positive gradient."]
    #[inline(always)]
    pub fn plimitrm(&self) -> PLIMITRM_R {
        PLIMITRM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits hold the setting for the middle-order 32 bits of the limit for the positive gradient."]
    #[inline(always)]
    #[must_use]
    pub fn plimitrm(&mut self) -> PLIMITRM_W<0> {
        PLIMITRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Positive Gradient Limit Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plimitrm](index.html) module"]
pub struct PLIMITRM_SPEC;
impl crate::RegisterSpec for PLIMITRM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [plimitrm::R](R) reader structure"]
impl crate::Readable for PLIMITRM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [plimitrm::W](W) writer structure"]
impl crate::Writable for PLIMITRM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLIMITRM to value 0"]
impl crate::Resettable for PLIMITRM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
