#[doc = "Register `GR%s_AB6` reader"]
pub struct R(crate::R<GR_AB6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GR_AB6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GR_AB6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GR_AB6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GR%s_AB6` writer"]
pub struct W(crate::W<GR_AB6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GR_AB6_SPEC>;
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
impl From<crate::W<GR_AB6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GR_AB6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARCRATE` reader - Frame rate for alpha blending in rectangular area."]
pub type ARCRATE_R = crate::FieldReader<u8, ARCRATE_A>;
#[doc = "Frame rate for alpha blending in rectangular area.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ARCRATE_A(u8);
impl From<ARCRATE_A> for u8 {
    #[inline(always)]
    fn from(val: ARCRATE_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `ARCRATE` writer - Frame rate for alpha blending in rectangular area."]
pub type ARCRATE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GR_AB6_SPEC, u8, ARCRATE_A, 8, O>;
#[doc = "Field `ARCCOEF` reader - Alpha coefficient for alpha blending in rectangular area (-255 to 255).\\[8\\]: Sign (0: addition, 1: subtraction)\\[7:0\\]: Variation (absolute value)"]
pub type ARCCOEF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ARCCOEF` writer - Alpha coefficient for alpha blending in rectangular area (-255 to 255).\\[8\\]: Sign (0: addition, 1: subtraction)\\[7:0\\]: Variation (absolute value)"]
pub type ARCCOEF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GR_AB6_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:7 - Frame rate for alpha blending in rectangular area."]
    #[inline(always)]
    pub fn arcrate(&self) -> ARCRATE_R {
        ARCRATE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:24 - Alpha coefficient for alpha blending in rectangular area (-255 to 255).\\[8\\]: Sign (0: addition, 1: subtraction)\\[7:0\\]: Variation (absolute value)"]
    #[inline(always)]
    pub fn arccoef(&self) -> ARCCOEF_R {
        ARCCOEF_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame rate for alpha blending in rectangular area."]
    #[inline(always)]
    #[must_use]
    pub fn arcrate(&mut self) -> ARCRATE_W<0> {
        ARCRATE_W::new(self)
    }
    #[doc = "Bits 16:24 - Alpha coefficient for alpha blending in rectangular area (-255 to 255).\\[8\\]: Sign (0: addition, 1: subtraction)\\[7:0\\]: Variation (absolute value)"]
    #[inline(always)]
    #[must_use]
    pub fn arccoef(&mut self) -> ARCCOEF_W<16> {
        ARCCOEF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Graphics %s Alpha Blending Control Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gr_ab6](index.html) module"]
pub struct GR_AB6_SPEC;
impl crate::RegisterSpec for GR_AB6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gr_ab6::R](R) reader structure"]
impl crate::Readable for GR_AB6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gr_ab6::W](W) writer structure"]
impl crate::Writable for GR_AB6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GR%s_AB6 to value 0"]
impl crate::Resettable for GR_AB6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
