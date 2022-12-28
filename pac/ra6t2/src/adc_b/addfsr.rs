#[doc = "Register `ADDFSR%s` reader"]
pub struct R(crate::R<ADDFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDFSR%s` writer"]
pub struct W(crate::W<ADDFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDFSR_SPEC>;
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
impl From<crate::W<ADDFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDFSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFSEL0` reader - A/D Converter Unit n the 1st Digital Filter Characteristic Selection"]
pub type DFSEL0_R = crate::FieldReader<u8, DFSEL0_A>;
#[doc = "A/D Converter Unit n the 1st Digital Filter Characteristic Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DFSEL0_A {
    #[doc = "1: Sinc filter"]
    _01 = 1,
    #[doc = "2: Minimum phase filter"]
    _10 = 2,
}
impl From<DFSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: DFSEL0_A) -> Self {
        variant as _
    }
}
impl DFSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DFSEL0_A> {
        match self.bits {
            1 => Some(DFSEL0_A::_01),
            2 => Some(DFSEL0_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DFSEL0_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DFSEL0_A::_10
    }
}
#[doc = "Field `DFSEL0` writer - A/D Converter Unit n the 1st Digital Filter Characteristic Selection"]
pub type DFSEL0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDFSR_SPEC, u8, DFSEL0_A, 2, O>;
impl<'a, const O: u8> DFSEL0_W<'a, O> {
    #[doc = "Sinc filter"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DFSEL0_A::_01)
    }
    #[doc = "Minimum phase filter"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DFSEL0_A::_10)
    }
}
#[doc = "Field `DFSEL1` reader - A/D Converter Unit n the 2nd Digital Filter Characteristic Selection"]
pub type DFSEL1_R = crate::FieldReader<u8, DFSEL1_A>;
#[doc = "A/D Converter Unit n the 2nd Digital Filter Characteristic Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DFSEL1_A {
    #[doc = "1: Sinc filter"]
    _01 = 1,
    #[doc = "2: Minimum phase filter"]
    _10 = 2,
}
impl From<DFSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: DFSEL1_A) -> Self {
        variant as _
    }
}
impl DFSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DFSEL1_A> {
        match self.bits {
            1 => Some(DFSEL1_A::_01),
            2 => Some(DFSEL1_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DFSEL1_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DFSEL1_A::_10
    }
}
#[doc = "Field `DFSEL1` writer - A/D Converter Unit n the 2nd Digital Filter Characteristic Selection"]
pub type DFSEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDFSR_SPEC, u8, DFSEL1_A, 2, O>;
impl<'a, const O: u8> DFSEL1_W<'a, O> {
    #[doc = "Sinc filter"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DFSEL1_A::_01)
    }
    #[doc = "Minimum phase filter"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DFSEL1_A::_10)
    }
}
#[doc = "Field `DFSEL2` reader - A/D Converter Unit n the 3rd Digital Filter Characteristic Selection"]
pub type DFSEL2_R = crate::FieldReader<u8, DFSEL2_A>;
#[doc = "A/D Converter Unit n the 3rd Digital Filter Characteristic Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DFSEL2_A {
    #[doc = "1: Sinc filter"]
    _01 = 1,
    #[doc = "2: Minimum phase filter"]
    _10 = 2,
}
impl From<DFSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: DFSEL2_A) -> Self {
        variant as _
    }
}
impl DFSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DFSEL2_A> {
        match self.bits {
            1 => Some(DFSEL2_A::_01),
            2 => Some(DFSEL2_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DFSEL2_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DFSEL2_A::_10
    }
}
#[doc = "Field `DFSEL2` writer - A/D Converter Unit n the 3rd Digital Filter Characteristic Selection"]
pub type DFSEL2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDFSR_SPEC, u8, DFSEL2_A, 2, O>;
impl<'a, const O: u8> DFSEL2_W<'a, O> {
    #[doc = "Sinc filter"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DFSEL2_A::_01)
    }
    #[doc = "Minimum phase filter"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DFSEL2_A::_10)
    }
}
#[doc = "Field `DFSEL3` reader - A/D Converter Unit n the 4th Digital Filter Characteristic Selection"]
pub type DFSEL3_R = crate::FieldReader<u8, DFSEL3_A>;
#[doc = "A/D Converter Unit n the 4th Digital Filter Characteristic Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DFSEL3_A {
    #[doc = "1: Sinc filter"]
    _01 = 1,
    #[doc = "2: Minimum phase filter"]
    _10 = 2,
}
impl From<DFSEL3_A> for u8 {
    #[inline(always)]
    fn from(variant: DFSEL3_A) -> Self {
        variant as _
    }
}
impl DFSEL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DFSEL3_A> {
        match self.bits {
            1 => Some(DFSEL3_A::_01),
            2 => Some(DFSEL3_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DFSEL3_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DFSEL3_A::_10
    }
}
#[doc = "Field `DFSEL3` writer - A/D Converter Unit n the 4th Digital Filter Characteristic Selection"]
pub type DFSEL3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDFSR_SPEC, u8, DFSEL3_A, 2, O>;
impl<'a, const O: u8> DFSEL3_W<'a, O> {
    #[doc = "Sinc filter"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DFSEL3_A::_01)
    }
    #[doc = "Minimum phase filter"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DFSEL3_A::_10)
    }
}
impl R {
    #[doc = "Bits 0:1 - A/D Converter Unit n the 1st Digital Filter Characteristic Selection"]
    #[inline(always)]
    pub fn dfsel0(&self) -> DFSEL0_R {
        DFSEL0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - A/D Converter Unit n the 2nd Digital Filter Characteristic Selection"]
    #[inline(always)]
    pub fn dfsel1(&self) -> DFSEL1_R {
        DFSEL1_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - A/D Converter Unit n the 3rd Digital Filter Characteristic Selection"]
    #[inline(always)]
    pub fn dfsel2(&self) -> DFSEL2_R {
        DFSEL2_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - A/D Converter Unit n the 4th Digital Filter Characteristic Selection"]
    #[inline(always)]
    pub fn dfsel3(&self) -> DFSEL3_R {
        DFSEL3_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - A/D Converter Unit n the 1st Digital Filter Characteristic Selection"]
    #[inline(always)]
    #[must_use]
    pub fn dfsel0(&mut self) -> DFSEL0_W<0> {
        DFSEL0_W::new(self)
    }
    #[doc = "Bits 8:9 - A/D Converter Unit n the 2nd Digital Filter Characteristic Selection"]
    #[inline(always)]
    #[must_use]
    pub fn dfsel1(&mut self) -> DFSEL1_W<8> {
        DFSEL1_W::new(self)
    }
    #[doc = "Bits 16:17 - A/D Converter Unit n the 3rd Digital Filter Characteristic Selection"]
    #[inline(always)]
    #[must_use]
    pub fn dfsel2(&mut self) -> DFSEL2_W<16> {
        DFSEL2_W::new(self)
    }
    #[doc = "Bits 24:25 - A/D Converter Unit n the 4th Digital Filter Characteristic Selection"]
    #[inline(always)]
    #[must_use]
    pub fn dfsel3(&mut self) -> DFSEL3_W<24> {
        DFSEL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Converter Digital Filter Selection Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addfsr](index.html) module"]
pub struct ADDFSR_SPEC;
impl crate::RegisterSpec for ADDFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addfsr::R](R) reader structure"]
impl crate::Readable for ADDFSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addfsr::W](W) writer structure"]
impl crate::Writable for ADDFSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDFSR%s to value 0"]
impl crate::Resettable for ADDFSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
