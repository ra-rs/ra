#[doc = "Register `GAM%s_LUT5` reader"]
pub struct R(crate::R<GAM_LUT5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAM_LUT5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAM_LUT5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAM_LUT5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAM%s_LUT5` writer"]
pub struct W(crate::W<GAM_LUT5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAM_LUT5_SPEC>;
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
impl From<crate::W<GAM_LUT5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAM_LUT5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GAIN09` reader - Gain value of area 9Unsigned 11-bit fixed point"]
pub type GAIN09_R = crate::FieldReader<u16, GAIN09_A>;
#[doc = "Gain value of area 9Unsigned 11-bit fixed point\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GAIN09_A(u16);
impl From<GAIN09_A> for u16 {
    #[inline(always)]
    fn from(val: GAIN09_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `GAIN09` writer - Gain value of area 9Unsigned 11-bit fixed point"]
pub type GAIN09_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GAM_LUT5_SPEC, u16, GAIN09_A, 11, O>;
#[doc = "Field `GAIN08` reader - Gain value of area 8Unsigned 11-bit fixed point"]
pub type GAIN08_R = crate::FieldReader<u16, GAIN08_A>;
#[doc = "Gain value of area 8Unsigned 11-bit fixed point\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GAIN08_A(u16);
impl From<GAIN08_A> for u16 {
    #[inline(always)]
    fn from(val: GAIN08_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `GAIN08` writer - Gain value of area 8Unsigned 11-bit fixed point"]
pub type GAIN08_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GAM_LUT5_SPEC, u16, GAIN08_A, 11, O>;
impl R {
    #[doc = "Bits 0:10 - Gain value of area 9Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain09(&self) -> GAIN09_R {
        GAIN09_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Gain value of area 8Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain08(&self) -> GAIN08_R {
        GAIN08_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Gain value of area 9Unsigned 11-bit fixed point"]
    #[inline(always)]
    #[must_use]
    pub fn gain09(&mut self) -> GAIN09_W<0> {
        GAIN09_W::new(self)
    }
    #[doc = "Bits 16:26 - Gain value of area 8Unsigned 11-bit fixed point"]
    #[inline(always)]
    #[must_use]
    pub fn gain08(&mut self) -> GAIN08_W<16> {
        GAIN08_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Gamma %s Correction Block Table Setting Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gam_lut5](index.html) module"]
pub struct GAM_LUT5_SPEC;
impl crate::RegisterSpec for GAM_LUT5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gam_lut5::R](R) reader structure"]
impl crate::Readable for GAM_LUT5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gam_lut5::W](W) writer structure"]
impl crate::Writable for GAM_LUT5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GAM%s_LUT5 to value 0"]
impl crate::Resettable for GAM_LUT5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
