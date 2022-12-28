#[doc = "Register `MLIMITRU` reader"]
pub struct R(crate::R<MLIMITRU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MLIMITRU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MLIMITRU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MLIMITRU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MLIMITRU` writer"]
pub struct W(crate::W<MLIMITRU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MLIMITRU_SPEC>;
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
impl From<crate::W<MLIMITRU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MLIMITRU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MLIMITRU` reader - These bits hold the setting for the higher-order 31 bits of the limit for the negative gradient."]
pub type MLIMITRU_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MLIMITRU` writer - These bits hold the setting for the higher-order 31 bits of the limit for the negative gradient."]
pub type MLIMITRU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MLIMITRU_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bits 0:30 - These bits hold the setting for the higher-order 31 bits of the limit for the negative gradient."]
    #[inline(always)]
    pub fn mlimitru(&self) -> MLIMITRU_R {
        MLIMITRU_R::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - These bits hold the setting for the higher-order 31 bits of the limit for the negative gradient."]
    #[inline(always)]
    #[must_use]
    pub fn mlimitru(&mut self) -> MLIMITRU_W<0> {
        MLIMITRU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Negative Gradient Limit Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mlimitru](index.html) module"]
pub struct MLIMITRU_SPEC;
impl crate::RegisterSpec for MLIMITRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mlimitru::R](R) reader structure"]
impl crate::Readable for MLIMITRU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mlimitru::W](W) writer structure"]
impl crate::Writable for MLIMITRU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MLIMITRU to value 0"]
impl crate::Resettable for MLIMITRU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
