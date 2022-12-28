#[doc = "Register `ADSGCR1` reader"]
pub struct R(crate::R<ADSGCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADSGCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADSGCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADSGCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADSGCR1` writer"]
pub struct W(crate::W<ADSGCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSGCR1_SPEC>;
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
impl From<crate::W<ADSGCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSGCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SGADS4` reader - Scan Group 4 A/D Converter Selection"]
pub type SGADS4_R = crate::FieldReader<u8, SGADS4_A>;
#[doc = "Scan Group 4 A/D Converter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SGADS4_A {
    #[doc = "0: Select ADC0"]
    _00 = 0,
    #[doc = "1: Select ADC1"]
    _01 = 1,
}
impl From<SGADS4_A> for u8 {
    #[inline(always)]
    fn from(variant: SGADS4_A) -> Self {
        variant as _
    }
}
impl SGADS4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SGADS4_A> {
        match self.bits {
            0 => Some(SGADS4_A::_00),
            1 => Some(SGADS4_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SGADS4_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SGADS4_A::_01
    }
}
#[doc = "Field `SGADS4` writer - Scan Group 4 A/D Converter Selection"]
pub type SGADS4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSGCR1_SPEC, u8, SGADS4_A, 2, O>;
impl<'a, const O: u8> SGADS4_W<'a, O> {
    #[doc = "Select ADC0"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SGADS4_A::_00)
    }
    #[doc = "Select ADC1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SGADS4_A::_01)
    }
}
#[doc = "Field `SGADS5` reader - Scan Group 5 A/D Converter Selection"]
pub type SGADS5_R = crate::FieldReader<u8, SGADS5_A>;
#[doc = "Scan Group 5 A/D Converter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SGADS5_A {
    #[doc = "0: Select ADC0"]
    _00 = 0,
    #[doc = "1: Select ADC1"]
    _01 = 1,
}
impl From<SGADS5_A> for u8 {
    #[inline(always)]
    fn from(variant: SGADS5_A) -> Self {
        variant as _
    }
}
impl SGADS5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SGADS5_A> {
        match self.bits {
            0 => Some(SGADS5_A::_00),
            1 => Some(SGADS5_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SGADS5_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SGADS5_A::_01
    }
}
#[doc = "Field `SGADS5` writer - Scan Group 5 A/D Converter Selection"]
pub type SGADS5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSGCR1_SPEC, u8, SGADS5_A, 2, O>;
impl<'a, const O: u8> SGADS5_W<'a, O> {
    #[doc = "Select ADC0"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SGADS5_A::_00)
    }
    #[doc = "Select ADC1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SGADS5_A::_01)
    }
}
#[doc = "Field `SGADS6` reader - Scan Group 6 A/D Converter Selection"]
pub type SGADS6_R = crate::FieldReader<u8, SGADS6_A>;
#[doc = "Scan Group 6 A/D Converter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SGADS6_A {
    #[doc = "0: Select ADC0"]
    _00 = 0,
    #[doc = "1: Select ADC1"]
    _01 = 1,
}
impl From<SGADS6_A> for u8 {
    #[inline(always)]
    fn from(variant: SGADS6_A) -> Self {
        variant as _
    }
}
impl SGADS6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SGADS6_A> {
        match self.bits {
            0 => Some(SGADS6_A::_00),
            1 => Some(SGADS6_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SGADS6_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SGADS6_A::_01
    }
}
#[doc = "Field `SGADS6` writer - Scan Group 6 A/D Converter Selection"]
pub type SGADS6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSGCR1_SPEC, u8, SGADS6_A, 2, O>;
impl<'a, const O: u8> SGADS6_W<'a, O> {
    #[doc = "Select ADC0"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SGADS6_A::_00)
    }
    #[doc = "Select ADC1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SGADS6_A::_01)
    }
}
#[doc = "Field `SGADS7` reader - Scan Group 7 A/D Converter Selection"]
pub type SGADS7_R = crate::FieldReader<u8, SGADS7_A>;
#[doc = "Scan Group 7 A/D Converter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SGADS7_A {
    #[doc = "0: Select ADC0"]
    _00 = 0,
    #[doc = "1: Select ADC1"]
    _01 = 1,
}
impl From<SGADS7_A> for u8 {
    #[inline(always)]
    fn from(variant: SGADS7_A) -> Self {
        variant as _
    }
}
impl SGADS7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SGADS7_A> {
        match self.bits {
            0 => Some(SGADS7_A::_00),
            1 => Some(SGADS7_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SGADS7_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SGADS7_A::_01
    }
}
#[doc = "Field `SGADS7` writer - Scan Group 7 A/D Converter Selection"]
pub type SGADS7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSGCR1_SPEC, u8, SGADS7_A, 2, O>;
impl<'a, const O: u8> SGADS7_W<'a, O> {
    #[doc = "Select ADC0"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SGADS7_A::_00)
    }
    #[doc = "Select ADC1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SGADS7_A::_01)
    }
}
impl R {
    #[doc = "Bits 0:1 - Scan Group 4 A/D Converter Selection"]
    #[inline(always)]
    pub fn sgads4(&self) -> SGADS4_R {
        SGADS4_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Scan Group 5 A/D Converter Selection"]
    #[inline(always)]
    pub fn sgads5(&self) -> SGADS5_R {
        SGADS5_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Scan Group 6 A/D Converter Selection"]
    #[inline(always)]
    pub fn sgads6(&self) -> SGADS6_R {
        SGADS6_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Scan Group 7 A/D Converter Selection"]
    #[inline(always)]
    pub fn sgads7(&self) -> SGADS7_R {
        SGADS7_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Scan Group 4 A/D Converter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn sgads4(&mut self) -> SGADS4_W<0> {
        SGADS4_W::new(self)
    }
    #[doc = "Bits 8:9 - Scan Group 5 A/D Converter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn sgads5(&mut self) -> SGADS5_W<8> {
        SGADS5_W::new(self)
    }
    #[doc = "Bits 16:17 - Scan Group 6 A/D Converter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn sgads6(&mut self) -> SGADS6_W<16> {
        SGADS6_W::new(self)
    }
    #[doc = "Bits 24:25 - Scan Group 7 A/D Converter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn sgads7(&mut self) -> SGADS7_W<24> {
        SGADS7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scan Group Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adsgcr1](index.html) module"]
pub struct ADSGCR1_SPEC;
impl crate::RegisterSpec for ADSGCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adsgcr1::R](R) reader structure"]
impl crate::Readable for ADSGCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adsgcr1::W](W) writer structure"]
impl crate::Writable for ADSGCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSGCR1 to value 0"]
impl crate::Resettable for ADSGCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
