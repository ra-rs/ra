#[doc = "Register `CTSUTRIMA` reader"]
pub struct R(crate::R<CTSUTRIMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUTRIMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUTRIMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUTRIMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUTRIMA` writer"]
pub struct W(crate::W<CTSUTRIMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUTRIMA_SPEC>;
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
impl From<crate::W<CTSUTRIMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUTRIMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTRIM` reader - CTSU Reference Resistance Adjustment"]
pub type RTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTRIM` writer - CTSU Reference Resistance Adjustment"]
pub type RTRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTSUTRIMA_SPEC, u8, u8, 8, O>;
#[doc = "Field `DACTRIM` reader - Linearity Adjustment of Offset Current"]
pub type DACTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DACTRIM` writer - Linearity Adjustment of Offset Current"]
pub type DACTRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTSUTRIMA_SPEC, u8, u8, 8, O>;
#[doc = "Field `SUADJD` reader - CTSU SUCLK Frequency Adjustment"]
pub type SUADJD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUADJD` writer - CTSU SUCLK Frequency Adjustment"]
pub type SUADJD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTSUTRIMA_SPEC, u8, u8, 8, O>;
#[doc = "Field `SUADJTRIM` reader - Coefficient of variation for the reference load resistance"]
pub type SUADJTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUADJTRIM` writer - Coefficient of variation for the reference load resistance"]
pub type SUADJTRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTSUTRIMA_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - CTSU Reference Resistance Adjustment"]
    #[inline(always)]
    pub fn rtrim(&self) -> RTRIM_R {
        RTRIM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Linearity Adjustment of Offset Current"]
    #[inline(always)]
    pub fn dactrim(&self) -> DACTRIM_R {
        DACTRIM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CTSU SUCLK Frequency Adjustment"]
    #[inline(always)]
    pub fn suadjd(&self) -> SUADJD_R {
        SUADJD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Coefficient of variation for the reference load resistance"]
    #[inline(always)]
    pub fn suadjtrim(&self) -> SUADJTRIM_R {
        SUADJTRIM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CTSU Reference Resistance Adjustment"]
    #[inline(always)]
    pub fn rtrim(&mut self) -> RTRIM_W<0> {
        RTRIM_W::new(self)
    }
    #[doc = "Bits 8:15 - Linearity Adjustment of Offset Current"]
    #[inline(always)]
    pub fn dactrim(&mut self) -> DACTRIM_W<8> {
        DACTRIM_W::new(self)
    }
    #[doc = "Bits 16:23 - CTSU SUCLK Frequency Adjustment"]
    #[inline(always)]
    pub fn suadjd(&mut self) -> SUADJD_W<16> {
        SUADJD_W::new(self)
    }
    #[doc = "Bits 24:31 - Coefficient of variation for the reference load resistance"]
    #[inline(always)]
    pub fn suadjtrim(&mut self) -> SUADJTRIM_W<24> {
        SUADJTRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU Trimming Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsutrima](index.html) module"]
pub struct CTSUTRIMA_SPEC;
impl crate::RegisterSpec for CTSUTRIMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctsutrima::R](R) reader structure"]
impl crate::Readable for CTSUTRIMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsutrima::W](W) writer structure"]
impl crate::Writable for CTSUTRIMA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTSUTRIMA to value 0"]
impl crate::Resettable for CTSUTRIMA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
