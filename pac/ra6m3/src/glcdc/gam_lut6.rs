#[doc = "Register `GAM%s_LUT6` reader"]
pub struct R(crate::R<GAM_LUT6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAM_LUT6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAM_LUT6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAM_LUT6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAM%s_LUT6` writer"]
pub struct W(crate::W<GAM_LUT6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAM_LUT6_SPEC>;
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
impl From<crate::W<GAM_LUT6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAM_LUT6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GAIN11` reader - Gain value of area 11Unsigned 11-bit fixed point"]
pub type GAIN11_R = crate::FieldReader<u16, GAIN11_A>;
#[doc = "Gain value of area 11Unsigned 11-bit fixed point\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GAIN11_A(u16);
impl From<GAIN11_A> for u16 {
    #[inline(always)]
    fn from(val: GAIN11_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `GAIN11` writer - Gain value of area 11Unsigned 11-bit fixed point"]
pub type GAIN11_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GAM_LUT6_SPEC, u16, GAIN11_A, 11, O>;
#[doc = "Field `GAIN10` reader - Gain value of area 10Unsigned 11-bit fixed point"]
pub type GAIN10_R = crate::FieldReader<u16, GAIN10_A>;
#[doc = "Gain value of area 10Unsigned 11-bit fixed point\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GAIN10_A(u16);
impl From<GAIN10_A> for u16 {
    #[inline(always)]
    fn from(val: GAIN10_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `GAIN10` writer - Gain value of area 10Unsigned 11-bit fixed point"]
pub type GAIN10_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GAM_LUT6_SPEC, u16, GAIN10_A, 11, O>;
impl R {
    #[doc = "Bits 0:10 - Gain value of area 11Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain11(&self) -> GAIN11_R {
        GAIN11_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Gain value of area 10Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain10(&self) -> GAIN10_R {
        GAIN10_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Gain value of area 11Unsigned 11-bit fixed point"]
    #[inline(always)]
    #[must_use]
    pub fn gain11(&mut self) -> GAIN11_W<0> {
        GAIN11_W::new(self)
    }
    #[doc = "Bits 16:26 - Gain value of area 10Unsigned 11-bit fixed point"]
    #[inline(always)]
    #[must_use]
    pub fn gain10(&mut self) -> GAIN10_W<16> {
        GAIN10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Gamma %s Correction Block Table Setting Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gam_lut6](index.html) module"]
pub struct GAM_LUT6_SPEC;
impl crate::RegisterSpec for GAM_LUT6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gam_lut6::R](R) reader structure"]
impl crate::Readable for GAM_LUT6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gam_lut6::W](W) writer structure"]
impl crate::Writable for GAM_LUT6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GAM%s_LUT6 to value 0"]
impl crate::Resettable for GAM_LUT6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
