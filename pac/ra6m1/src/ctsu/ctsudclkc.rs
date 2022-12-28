#[doc = "Register `CTSUDCLKC` reader"]
pub struct R(crate::R<CTSUDCLKC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUDCLKC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUDCLKC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUDCLKC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUDCLKC` writer"]
pub struct W(crate::W<CTSUDCLKC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUDCLKC_SPEC>;
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
impl From<crate::W<CTSUDCLKC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUDCLKC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTSUSSMOD` reader - CTSU Diffusion Clock Mode Select NOTE: This bit should be set to 00b."]
pub type CTSUSSMOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTSUSSMOD` writer - CTSU Diffusion Clock Mode Select NOTE: This bit should be set to 00b."]
pub type CTSUSSMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CTSUDCLKC_SPEC, u8, u8, 2, O>;
#[doc = "Field `CTSUSSCNT` reader - CTSU Diffusion Clock Mode Control NOTE: This bit should be set to 11b."]
pub type CTSUSSCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTSUSSCNT` writer - CTSU Diffusion Clock Mode Control NOTE: This bit should be set to 11b."]
pub type CTSUSSCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CTSUDCLKC_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - CTSU Diffusion Clock Mode Select NOTE: This bit should be set to 00b."]
    #[inline(always)]
    pub fn ctsussmod(&self) -> CTSUSSMOD_R {
        CTSUSSMOD_R::new(self.bits & 3)
    }
    #[doc = "Bits 4:5 - CTSU Diffusion Clock Mode Control NOTE: This bit should be set to 11b."]
    #[inline(always)]
    pub fn ctsusscnt(&self) -> CTSUSSCNT_R {
        CTSUSSCNT_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - CTSU Diffusion Clock Mode Select NOTE: This bit should be set to 00b."]
    #[inline(always)]
    #[must_use]
    pub fn ctsussmod(&mut self) -> CTSUSSMOD_W<0> {
        CTSUSSMOD_W::new(self)
    }
    #[doc = "Bits 4:5 - CTSU Diffusion Clock Mode Control NOTE: This bit should be set to 11b."]
    #[inline(always)]
    #[must_use]
    pub fn ctsusscnt(&mut self) -> CTSUSSCNT_W<4> {
        CTSUSSCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU High-Pass Noise Reduction Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsudclkc](index.html) module"]
pub struct CTSUDCLKC_SPEC;
impl crate::RegisterSpec for CTSUDCLKC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctsudclkc::R](R) reader structure"]
impl crate::Readable for CTSUDCLKC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsudclkc::W](W) writer structure"]
impl crate::Writable for CTSUDCLKC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUDCLKC to value 0"]
impl crate::Resettable for CTSUDCLKC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
