#[doc = "Register `ADSGCR2` reader"]
pub struct R(crate::R<ADSGCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADSGCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADSGCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADSGCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADSGCR2` writer"]
pub struct W(crate::W<ADSGCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSGCR2_SPEC>;
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
impl From<crate::W<ADSGCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSGCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SGADS8` reader - Scan Group 8 A/D Converter Selection"]
pub type SGADS8_R = crate::FieldReader<u8, SGADS8_A>;
#[doc = "Scan Group 8 A/D Converter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SGADS8_A {
    #[doc = "0: Select ADC0"]
    _00 = 0,
    #[doc = "1: Select ADC1"]
    _01 = 1,
}
impl From<SGADS8_A> for u8 {
    #[inline(always)]
    fn from(variant: SGADS8_A) -> Self {
        variant as _
    }
}
impl SGADS8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SGADS8_A> {
        match self.bits {
            0 => Some(SGADS8_A::_00),
            1 => Some(SGADS8_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SGADS8_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SGADS8_A::_01
    }
}
#[doc = "Field `SGADS8` writer - Scan Group 8 A/D Converter Selection"]
pub type SGADS8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSGCR2_SPEC, u8, SGADS8_A, 2, O>;
impl<'a, const O: u8> SGADS8_W<'a, O> {
    #[doc = "Select ADC0"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SGADS8_A::_00)
    }
    #[doc = "Select ADC1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SGADS8_A::_01)
    }
}
impl R {
    #[doc = "Bits 0:1 - Scan Group 8 A/D Converter Selection"]
    #[inline(always)]
    pub fn sgads8(&self) -> SGADS8_R {
        SGADS8_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Scan Group 8 A/D Converter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn sgads8(&mut self) -> SGADS8_W<0> {
        SGADS8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scan Group Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adsgcr2](index.html) module"]
pub struct ADSGCR2_SPEC;
impl crate::RegisterSpec for ADSGCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adsgcr2::R](R) reader structure"]
impl crate::Readable for ADSGCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adsgcr2::W](W) writer structure"]
impl crate::Writable for ADSGCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSGCR2 to value 0"]
impl crate::Resettable for ADSGCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
