#[doc = "Register `ADCMPMDR1` reader"]
pub struct R(crate::R<ADCMPMDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPMDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPMDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPMDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPMDR1` writer"]
pub struct W(crate::W<ADCMPMDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPMDR1_SPEC>;
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
impl From<crate::W<ADCMPMDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPMDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPMD4` reader - Compare Match 4 : Match Mode Selection"]
pub type CMPMD4_R = crate::FieldReader<u8, CMPMD4_A>;
#[doc = "Compare Match 4 : Match Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPMD4_A {
    #[doc = "0: Generate the match event when high-side level or more"]
    _00 = 0,
    #[doc = "1: Generate the match event when low-side level or less"]
    _01 = 1,
    #[doc = "2: Generate the match event when high-side level or more, or low-side level or less"]
    _10 = 2,
    #[doc = "3: Generate the match event when low-side level or more and high-side level or less"]
    _11 = 3,
}
impl From<CMPMD4_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPMD4_A) -> Self {
        variant as _
    }
}
impl CMPMD4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPMD4_A {
        match self.bits {
            0 => CMPMD4_A::_00,
            1 => CMPMD4_A::_01,
            2 => CMPMD4_A::_10,
            3 => CMPMD4_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CMPMD4_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CMPMD4_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CMPMD4_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CMPMD4_A::_11
    }
}
#[doc = "Field `CMPMD4` writer - Compare Match 4 : Match Mode Selection"]
pub type CMPMD4_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADCMPMDR1_SPEC, u8, CMPMD4_A, 2, O>;
impl<'a, const O: u8> CMPMD4_W<'a, O> {
    #[doc = "Generate the match event when high-side level or more"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CMPMD4_A::_00)
    }
    #[doc = "Generate the match event when low-side level or less"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CMPMD4_A::_01)
    }
    #[doc = "Generate the match event when high-side level or more, or low-side level or less"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMPMD4_A::_10)
    }
    #[doc = "Generate the match event when low-side level or more and high-side level or less"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CMPMD4_A::_11)
    }
}
#[doc = "Field `CMPMD5` reader - Compare Match 5 : Match Mode Selection"]
pub type CMPMD5_R = crate::FieldReader<u8, CMPMD5_A>;
#[doc = "Compare Match 5 : Match Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPMD5_A {
    #[doc = "0: Generate the match event when high-side level or more"]
    _00 = 0,
    #[doc = "1: Generate the match event when low-side level or less"]
    _01 = 1,
    #[doc = "2: Generate the match event when high-side level or more, or low-side level or less"]
    _10 = 2,
    #[doc = "3: Generate the match event when low-side level or more and high-side level or less"]
    _11 = 3,
}
impl From<CMPMD5_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPMD5_A) -> Self {
        variant as _
    }
}
impl CMPMD5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPMD5_A {
        match self.bits {
            0 => CMPMD5_A::_00,
            1 => CMPMD5_A::_01,
            2 => CMPMD5_A::_10,
            3 => CMPMD5_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CMPMD5_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CMPMD5_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CMPMD5_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CMPMD5_A::_11
    }
}
#[doc = "Field `CMPMD5` writer - Compare Match 5 : Match Mode Selection"]
pub type CMPMD5_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADCMPMDR1_SPEC, u8, CMPMD5_A, 2, O>;
impl<'a, const O: u8> CMPMD5_W<'a, O> {
    #[doc = "Generate the match event when high-side level or more"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CMPMD5_A::_00)
    }
    #[doc = "Generate the match event when low-side level or less"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CMPMD5_A::_01)
    }
    #[doc = "Generate the match event when high-side level or more, or low-side level or less"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMPMD5_A::_10)
    }
    #[doc = "Generate the match event when low-side level or more and high-side level or less"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CMPMD5_A::_11)
    }
}
#[doc = "Field `CMPMD6` reader - Compare Match 6 : Match Mode Selection"]
pub type CMPMD6_R = crate::FieldReader<u8, CMPMD6_A>;
#[doc = "Compare Match 6 : Match Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPMD6_A {
    #[doc = "0: Generate the match event when high-side level or more"]
    _00 = 0,
    #[doc = "1: Generate the match event when low-side level or less"]
    _01 = 1,
    #[doc = "2: Generate the match event when high-side level or more, or low-side level or less"]
    _10 = 2,
    #[doc = "3: Generate the match event when low-side level or more and high-side level or less"]
    _11 = 3,
}
impl From<CMPMD6_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPMD6_A) -> Self {
        variant as _
    }
}
impl CMPMD6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPMD6_A {
        match self.bits {
            0 => CMPMD6_A::_00,
            1 => CMPMD6_A::_01,
            2 => CMPMD6_A::_10,
            3 => CMPMD6_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CMPMD6_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CMPMD6_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CMPMD6_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CMPMD6_A::_11
    }
}
#[doc = "Field `CMPMD6` writer - Compare Match 6 : Match Mode Selection"]
pub type CMPMD6_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADCMPMDR1_SPEC, u8, CMPMD6_A, 2, O>;
impl<'a, const O: u8> CMPMD6_W<'a, O> {
    #[doc = "Generate the match event when high-side level or more"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CMPMD6_A::_00)
    }
    #[doc = "Generate the match event when low-side level or less"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CMPMD6_A::_01)
    }
    #[doc = "Generate the match event when high-side level or more, or low-side level or less"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMPMD6_A::_10)
    }
    #[doc = "Generate the match event when low-side level or more and high-side level or less"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CMPMD6_A::_11)
    }
}
#[doc = "Field `CMPMD7` reader - Compare Match 7 : Match Mode Selection"]
pub type CMPMD7_R = crate::FieldReader<u8, CMPMD7_A>;
#[doc = "Compare Match 7 : Match Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPMD7_A {
    #[doc = "0: Generate the match event when high-side level or more"]
    _00 = 0,
    #[doc = "1: Generate the match event when low-side level or less"]
    _01 = 1,
    #[doc = "2: Generate the match event when high-side level or more, or low-side level or less"]
    _10 = 2,
    #[doc = "3: Generate the match event when low-side level or more and high-side level or less"]
    _11 = 3,
}
impl From<CMPMD7_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPMD7_A) -> Self {
        variant as _
    }
}
impl CMPMD7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPMD7_A {
        match self.bits {
            0 => CMPMD7_A::_00,
            1 => CMPMD7_A::_01,
            2 => CMPMD7_A::_10,
            3 => CMPMD7_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CMPMD7_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CMPMD7_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CMPMD7_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CMPMD7_A::_11
    }
}
#[doc = "Field `CMPMD7` writer - Compare Match 7 : Match Mode Selection"]
pub type CMPMD7_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADCMPMDR1_SPEC, u8, CMPMD7_A, 2, O>;
impl<'a, const O: u8> CMPMD7_W<'a, O> {
    #[doc = "Generate the match event when high-side level or more"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CMPMD7_A::_00)
    }
    #[doc = "Generate the match event when low-side level or less"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CMPMD7_A::_01)
    }
    #[doc = "Generate the match event when high-side level or more, or low-side level or less"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMPMD7_A::_10)
    }
    #[doc = "Generate the match event when low-side level or more and high-side level or less"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CMPMD7_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Compare Match 4 : Match Mode Selection"]
    #[inline(always)]
    pub fn cmpmd4(&self) -> CMPMD4_R {
        CMPMD4_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Compare Match 5 : Match Mode Selection"]
    #[inline(always)]
    pub fn cmpmd5(&self) -> CMPMD5_R {
        CMPMD5_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Compare Match 6 : Match Mode Selection"]
    #[inline(always)]
    pub fn cmpmd6(&self) -> CMPMD6_R {
        CMPMD6_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Compare Match 7 : Match Mode Selection"]
    #[inline(always)]
    pub fn cmpmd7(&self) -> CMPMD7_R {
        CMPMD7_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Compare Match 4 : Match Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmpmd4(&mut self) -> CMPMD4_W<0> {
        CMPMD4_W::new(self)
    }
    #[doc = "Bits 8:9 - Compare Match 5 : Match Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmpmd5(&mut self) -> CMPMD5_W<8> {
        CMPMD5_W::new(self)
    }
    #[doc = "Bits 16:17 - Compare Match 6 : Match Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmpmd6(&mut self) -> CMPMD6_W<16> {
        CMPMD6_W::new(self)
    }
    #[doc = "Bits 24:25 - Compare Match 7 : Match Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmpmd7(&mut self) -> CMPMD7_W<24> {
        CMPMD7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare Match Mode Selection Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpmdr1](index.html) module"]
pub struct ADCMPMDR1_SPEC;
impl crate::RegisterSpec for ADCMPMDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcmpmdr1::R](R) reader structure"]
impl crate::Readable for ADCMPMDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmpmdr1::W](W) writer structure"]
impl crate::Writable for ADCMPMDR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPMDR1 to value 0"]
impl crate::Resettable for ADCMPMDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
