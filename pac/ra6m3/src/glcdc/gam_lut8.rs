#[doc = "Register `GAM%s_LUT8` reader"]
pub struct R(crate::R<GAM_LUT8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAM_LUT8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAM_LUT8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAM_LUT8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAM%s_LUT8` writer"]
pub struct W(crate::W<GAM_LUT8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAM_LUT8_SPEC>;
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
impl From<crate::W<GAM_LUT8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAM_LUT8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GAIN15` reader - Gain value of area 15Unsigned 11-bit fixed point"]
pub type GAIN15_R = crate::FieldReader<u16, GAIN15_A>;
#[doc = "Gain value of area 15Unsigned 11-bit fixed point\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GAIN15_A(u16);
impl From<GAIN15_A> for u16 {
    #[inline(always)]
    fn from(val: GAIN15_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `GAIN15` writer - Gain value of area 15Unsigned 11-bit fixed point"]
pub type GAIN15_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GAM_LUT8_SPEC, u16, GAIN15_A, 11, O>;
#[doc = "Field `GAIN14` reader - Gain value of area 14Unsigned 11-bit fixed point"]
pub type GAIN14_R = crate::FieldReader<u16, GAIN14_A>;
#[doc = "Gain value of area 14Unsigned 11-bit fixed point\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GAIN14_A(u16);
impl From<GAIN14_A> for u16 {
    #[inline(always)]
    fn from(val: GAIN14_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `GAIN14` writer - Gain value of area 14Unsigned 11-bit fixed point"]
pub type GAIN14_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GAM_LUT8_SPEC, u16, GAIN14_A, 11, O>;
impl R {
    #[doc = "Bits 0:10 - Gain value of area 15Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain15(&self) -> GAIN15_R {
        GAIN15_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Gain value of area 14Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain14(&self) -> GAIN14_R {
        GAIN14_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Gain value of area 15Unsigned 11-bit fixed point"]
    #[inline(always)]
    #[must_use]
    pub fn gain15(&mut self) -> GAIN15_W<0> {
        GAIN15_W::new(self)
    }
    #[doc = "Bits 16:26 - Gain value of area 14Unsigned 11-bit fixed point"]
    #[inline(always)]
    #[must_use]
    pub fn gain14(&mut self) -> GAIN14_W<16> {
        GAIN14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Gamma %s Correction Block Table Setting Register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gam_lut8](index.html) module"]
pub struct GAM_LUT8_SPEC;
impl crate::RegisterSpec for GAM_LUT8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gam_lut8::R](R) reader structure"]
impl crate::Readable for GAM_LUT8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gam_lut8::W](W) writer structure"]
impl crate::Writable for GAM_LUT8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GAM%s_LUT8 to value 0"]
impl crate::Resettable for GAM_LUT8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
