#[doc = "Register `PLIMITRU` reader"]
pub struct R(crate::R<PLIMITRU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLIMITRU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLIMITRU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLIMITRU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLIMITRU` writer"]
pub struct W(crate::W<PLIMITRU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLIMITRU_SPEC>;
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
impl From<crate::W<PLIMITRU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLIMITRU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLIMITRU` reader - These bits hold the setting for the higher-order 31 bits of the limit for the positive gradient."]
pub type PLIMITRU_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PLIMITRU` writer - These bits hold the setting for the higher-order 31 bits of the limit for the positive gradient."]
pub type PLIMITRU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLIMITRU_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bits 0:30 - These bits hold the setting for the higher-order 31 bits of the limit for the positive gradient."]
    #[inline(always)]
    pub fn plimitru(&self) -> PLIMITRU_R {
        PLIMITRU_R::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - These bits hold the setting for the higher-order 31 bits of the limit for the positive gradient."]
    #[inline(always)]
    #[must_use]
    pub fn plimitru(&mut self) -> PLIMITRU_W<0> {
        PLIMITRU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Positive Gradient Limit Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plimitru](index.html) module"]
pub struct PLIMITRU_SPEC;
impl crate::RegisterSpec for PLIMITRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [plimitru::R](R) reader structure"]
impl crate::Readable for PLIMITRU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [plimitru::W](W) writer structure"]
impl crate::Writable for PLIMITRU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLIMITRU to value 0"]
impl crate::Resettable for PLIMITRU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
