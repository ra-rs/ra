#[doc = "Register `CTSUSSC` reader"]
pub struct R(crate::R<CTSUSSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUSSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUSSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUSSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUSSC` writer"]
pub struct W(crate::W<CTSUSSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUSSC_SPEC>;
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
impl From<crate::W<CTSUSSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUSSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTSUSSDIV` reader - CTSU Spectrum Diffusion Frequency Division Setting"]
pub type CTSUSSDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTSUSSDIV` writer - CTSU Spectrum Diffusion Frequency Division Setting"]
pub type CTSUSSDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CTSUSSC_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 8:11 - CTSU Spectrum Diffusion Frequency Division Setting"]
    #[inline(always)]
    pub fn ctsussdiv(&self) -> CTSUSSDIV_R {
        CTSUSSDIV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - CTSU Spectrum Diffusion Frequency Division Setting"]
    #[inline(always)]
    #[must_use]
    pub fn ctsussdiv(&mut self) -> CTSUSSDIV_W<8> {
        CTSUSSDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU High-Pass Noise Reduction Spectrum Diffusion Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsussc](index.html) module"]
pub struct CTSUSSC_SPEC;
impl crate::RegisterSpec for CTSUSSC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctsussc::R](R) reader structure"]
impl crate::Readable for CTSUSSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsussc::W](W) writer structure"]
impl crate::Writable for CTSUSSC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUSSC to value 0"]
impl crate::Resettable for CTSUSSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
