#[doc = "Register `GAM%s_LUT2` reader"]
pub struct R(crate::R<GAM_LUT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAM_LUT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAM_LUT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAM_LUT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAM%s_LUT2` writer"]
pub struct W(crate::W<GAM_LUT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAM_LUT2_SPEC>;
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
impl From<crate::W<GAM_LUT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAM_LUT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GAIN03` reader - Gain value of area 3Unsigned 11-bit fixed point"]
pub type GAIN03_R = crate::FieldReader<u16, GAIN03_A>;
#[doc = "Gain value of area 3Unsigned 11-bit fixed point\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GAIN03_A(u16);
impl From<GAIN03_A> for u16 {
    #[inline(always)]
    fn from(val: GAIN03_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `GAIN03` writer - Gain value of area 3Unsigned 11-bit fixed point"]
pub type GAIN03_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GAM_LUT2_SPEC, u16, GAIN03_A, 11, O>;
#[doc = "Field `GAIN02` reader - Gain value of area 2Unsigned 11-bit fixed point"]
pub type GAIN02_R = crate::FieldReader<u16, GAIN02_A>;
#[doc = "Gain value of area 2Unsigned 11-bit fixed point\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GAIN02_A(u16);
impl From<GAIN02_A> for u16 {
    #[inline(always)]
    fn from(val: GAIN02_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `GAIN02` writer - Gain value of area 2Unsigned 11-bit fixed point"]
pub type GAIN02_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GAM_LUT2_SPEC, u16, GAIN02_A, 11, O>;
impl R {
    #[doc = "Bits 0:10 - Gain value of area 3Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain03(&self) -> GAIN03_R {
        GAIN03_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Gain value of area 2Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain02(&self) -> GAIN02_R {
        GAIN02_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Gain value of area 3Unsigned 11-bit fixed point"]
    #[inline(always)]
    #[must_use]
    pub fn gain03(&mut self) -> GAIN03_W<0> {
        GAIN03_W::new(self)
    }
    #[doc = "Bits 16:26 - Gain value of area 2Unsigned 11-bit fixed point"]
    #[inline(always)]
    #[must_use]
    pub fn gain02(&mut self) -> GAIN02_W<16> {
        GAIN02_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Gamma %s Correction Block Table Setting Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gam_lut2](index.html) module"]
pub struct GAM_LUT2_SPEC;
impl crate::RegisterSpec for GAM_LUT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gam_lut2::R](R) reader structure"]
impl crate::Readable for GAM_LUT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gam_lut2::W](W) writer structure"]
impl crate::Writable for GAM_LUT2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GAM%s_LUT2 to value 0"]
impl crate::Resettable for GAM_LUT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
