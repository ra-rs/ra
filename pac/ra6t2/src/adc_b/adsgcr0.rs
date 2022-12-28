#[doc = "Register `ADSGCR0` reader"]
pub struct R(crate::R<ADSGCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADSGCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADSGCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADSGCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADSGCR0` writer"]
pub struct W(crate::W<ADSGCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSGCR0_SPEC>;
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
impl From<crate::W<ADSGCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSGCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SGADS0` reader - Scan Group 0 A/D Converter Selection"]
pub type SGADS0_R = crate::FieldReader<u8, SGADS0_A>;
#[doc = "Scan Group 0 A/D Converter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SGADS0_A {
    #[doc = "0: Select ADC0"]
    _00 = 0,
    #[doc = "1: Select ADC1"]
    _01 = 1,
}
impl From<SGADS0_A> for u8 {
    #[inline(always)]
    fn from(variant: SGADS0_A) -> Self {
        variant as _
    }
}
impl SGADS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SGADS0_A> {
        match self.bits {
            0 => Some(SGADS0_A::_00),
            1 => Some(SGADS0_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SGADS0_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SGADS0_A::_01
    }
}
#[doc = "Field `SGADS0` writer - Scan Group 0 A/D Converter Selection"]
pub type SGADS0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSGCR0_SPEC, u8, SGADS0_A, 2, O>;
impl<'a, const O: u8> SGADS0_W<'a, O> {
    #[doc = "Select ADC0"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SGADS0_A::_00)
    }
    #[doc = "Select ADC1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SGADS0_A::_01)
    }
}
#[doc = "Field `SGADS1` reader - Scan Group 1 A/D Converter Selection"]
pub type SGADS1_R = crate::FieldReader<u8, SGADS1_A>;
#[doc = "Scan Group 1 A/D Converter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SGADS1_A {
    #[doc = "0: Select ADC0"]
    _00 = 0,
    #[doc = "1: Select ADC1"]
    _01 = 1,
}
impl From<SGADS1_A> for u8 {
    #[inline(always)]
    fn from(variant: SGADS1_A) -> Self {
        variant as _
    }
}
impl SGADS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SGADS1_A> {
        match self.bits {
            0 => Some(SGADS1_A::_00),
            1 => Some(SGADS1_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SGADS1_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SGADS1_A::_01
    }
}
#[doc = "Field `SGADS1` writer - Scan Group 1 A/D Converter Selection"]
pub type SGADS1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSGCR0_SPEC, u8, SGADS1_A, 2, O>;
impl<'a, const O: u8> SGADS1_W<'a, O> {
    #[doc = "Select ADC0"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SGADS1_A::_00)
    }
    #[doc = "Select ADC1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SGADS1_A::_01)
    }
}
#[doc = "Field `SGADS2` reader - Scan Group 2 A/D Converter Selection"]
pub type SGADS2_R = crate::FieldReader<u8, SGADS2_A>;
#[doc = "Scan Group 2 A/D Converter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SGADS2_A {
    #[doc = "0: Select ADC0"]
    _00 = 0,
    #[doc = "1: Select ADC1"]
    _01 = 1,
}
impl From<SGADS2_A> for u8 {
    #[inline(always)]
    fn from(variant: SGADS2_A) -> Self {
        variant as _
    }
}
impl SGADS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SGADS2_A> {
        match self.bits {
            0 => Some(SGADS2_A::_00),
            1 => Some(SGADS2_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SGADS2_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SGADS2_A::_01
    }
}
#[doc = "Field `SGADS2` writer - Scan Group 2 A/D Converter Selection"]
pub type SGADS2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSGCR0_SPEC, u8, SGADS2_A, 2, O>;
impl<'a, const O: u8> SGADS2_W<'a, O> {
    #[doc = "Select ADC0"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SGADS2_A::_00)
    }
    #[doc = "Select ADC1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SGADS2_A::_01)
    }
}
#[doc = "Field `SGADS3` reader - Scan Group 3 A/D Converter Selection"]
pub type SGADS3_R = crate::FieldReader<u8, SGADS3_A>;
#[doc = "Scan Group 3 A/D Converter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SGADS3_A {
    #[doc = "0: Select ADC0"]
    _00 = 0,
    #[doc = "1: Select ADC1"]
    _01 = 1,
}
impl From<SGADS3_A> for u8 {
    #[inline(always)]
    fn from(variant: SGADS3_A) -> Self {
        variant as _
    }
}
impl SGADS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SGADS3_A> {
        match self.bits {
            0 => Some(SGADS3_A::_00),
            1 => Some(SGADS3_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SGADS3_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SGADS3_A::_01
    }
}
#[doc = "Field `SGADS3` writer - Scan Group 3 A/D Converter Selection"]
pub type SGADS3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSGCR0_SPEC, u8, SGADS3_A, 2, O>;
impl<'a, const O: u8> SGADS3_W<'a, O> {
    #[doc = "Select ADC0"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SGADS3_A::_00)
    }
    #[doc = "Select ADC1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SGADS3_A::_01)
    }
}
impl R {
    #[doc = "Bits 0:1 - Scan Group 0 A/D Converter Selection"]
    #[inline(always)]
    pub fn sgads0(&self) -> SGADS0_R {
        SGADS0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Scan Group 1 A/D Converter Selection"]
    #[inline(always)]
    pub fn sgads1(&self) -> SGADS1_R {
        SGADS1_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Scan Group 2 A/D Converter Selection"]
    #[inline(always)]
    pub fn sgads2(&self) -> SGADS2_R {
        SGADS2_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Scan Group 3 A/D Converter Selection"]
    #[inline(always)]
    pub fn sgads3(&self) -> SGADS3_R {
        SGADS3_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Scan Group 0 A/D Converter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn sgads0(&mut self) -> SGADS0_W<0> {
        SGADS0_W::new(self)
    }
    #[doc = "Bits 8:9 - Scan Group 1 A/D Converter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn sgads1(&mut self) -> SGADS1_W<8> {
        SGADS1_W::new(self)
    }
    #[doc = "Bits 16:17 - Scan Group 2 A/D Converter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn sgads2(&mut self) -> SGADS2_W<16> {
        SGADS2_W::new(self)
    }
    #[doc = "Bits 24:25 - Scan Group 3 A/D Converter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn sgads3(&mut self) -> SGADS3_W<24> {
        SGADS3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scan Group Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adsgcr0](index.html) module"]
pub struct ADSGCR0_SPEC;
impl crate::RegisterSpec for ADSGCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adsgcr0::R](R) reader structure"]
impl crate::Readable for ADSGCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adsgcr0::W](W) writer structure"]
impl crate::Writable for ADSGCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSGCR0 to value 0"]
impl crate::Resettable for ADSGCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
