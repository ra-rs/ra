#[doc = "Register `ADCMPMDR0` reader"]
pub struct R(crate::R<ADCMPMDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPMDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPMDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPMDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPMDR0` writer"]
pub struct W(crate::W<ADCMPMDR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPMDR0_SPEC>;
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
impl From<crate::W<ADCMPMDR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPMDR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPMD0` reader - Compare Match 0 : Match Mode Selection"]
pub type CMPMD0_R = crate::FieldReader<u8, CMPMD0_A>;
#[doc = "Compare Match 0 : Match Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPMD0_A {
    #[doc = "0: Generate the match event when high-side level or more"]
    _00 = 0,
    #[doc = "1: Generate the match event when low-side level or less"]
    _01 = 1,
    #[doc = "2: Generate the match event when high-side level or more, or low-side level or less"]
    _10 = 2,
    #[doc = "3: Generate the match event when low-side level or more and high-side level or less"]
    _11 = 3,
}
impl From<CMPMD0_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPMD0_A) -> Self {
        variant as _
    }
}
impl CMPMD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPMD0_A {
        match self.bits {
            0 => CMPMD0_A::_00,
            1 => CMPMD0_A::_01,
            2 => CMPMD0_A::_10,
            3 => CMPMD0_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CMPMD0_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CMPMD0_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CMPMD0_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CMPMD0_A::_11
    }
}
#[doc = "Field `CMPMD0` writer - Compare Match 0 : Match Mode Selection"]
pub type CMPMD0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADCMPMDR0_SPEC, u8, CMPMD0_A, 2, O>;
impl<'a, const O: u8> CMPMD0_W<'a, O> {
    #[doc = "Generate the match event when high-side level or more"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CMPMD0_A::_00)
    }
    #[doc = "Generate the match event when low-side level or less"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CMPMD0_A::_01)
    }
    #[doc = "Generate the match event when high-side level or more, or low-side level or less"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMPMD0_A::_10)
    }
    #[doc = "Generate the match event when low-side level or more and high-side level or less"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CMPMD0_A::_11)
    }
}
#[doc = "Field `CMPMD1` reader - Compare Match 1 : Match Mode Selection"]
pub type CMPMD1_R = crate::FieldReader<u8, CMPMD1_A>;
#[doc = "Compare Match 1 : Match Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPMD1_A {
    #[doc = "0: Generate the match event when high-side level or more"]
    _00 = 0,
    #[doc = "1: Generate the match event when low-side level or less"]
    _01 = 1,
    #[doc = "2: Generate the match event when high-side level or more, or low-side level or less"]
    _10 = 2,
    #[doc = "3: Generate the match event when low-side level or more and high-side level or less"]
    _11 = 3,
}
impl From<CMPMD1_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPMD1_A) -> Self {
        variant as _
    }
}
impl CMPMD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPMD1_A {
        match self.bits {
            0 => CMPMD1_A::_00,
            1 => CMPMD1_A::_01,
            2 => CMPMD1_A::_10,
            3 => CMPMD1_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CMPMD1_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CMPMD1_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CMPMD1_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CMPMD1_A::_11
    }
}
#[doc = "Field `CMPMD1` writer - Compare Match 1 : Match Mode Selection"]
pub type CMPMD1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADCMPMDR0_SPEC, u8, CMPMD1_A, 2, O>;
impl<'a, const O: u8> CMPMD1_W<'a, O> {
    #[doc = "Generate the match event when high-side level or more"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CMPMD1_A::_00)
    }
    #[doc = "Generate the match event when low-side level or less"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CMPMD1_A::_01)
    }
    #[doc = "Generate the match event when high-side level or more, or low-side level or less"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMPMD1_A::_10)
    }
    #[doc = "Generate the match event when low-side level or more and high-side level or less"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CMPMD1_A::_11)
    }
}
#[doc = "Field `CMPMD2` reader - Compare Match 2 : Match Mode Selection"]
pub type CMPMD2_R = crate::FieldReader<u8, CMPMD2_A>;
#[doc = "Compare Match 2 : Match Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPMD2_A {
    #[doc = "0: Generate the match event when high-side level or more"]
    _00 = 0,
    #[doc = "1: Generate the match event when low-side level or less"]
    _01 = 1,
    #[doc = "2: Generate the match event when high-side level or more, or low-side level or less"]
    _10 = 2,
    #[doc = "3: Generate the match event when low-side level or more and high-side level or less"]
    _11 = 3,
}
impl From<CMPMD2_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPMD2_A) -> Self {
        variant as _
    }
}
impl CMPMD2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPMD2_A {
        match self.bits {
            0 => CMPMD2_A::_00,
            1 => CMPMD2_A::_01,
            2 => CMPMD2_A::_10,
            3 => CMPMD2_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CMPMD2_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CMPMD2_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CMPMD2_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CMPMD2_A::_11
    }
}
#[doc = "Field `CMPMD2` writer - Compare Match 2 : Match Mode Selection"]
pub type CMPMD2_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADCMPMDR0_SPEC, u8, CMPMD2_A, 2, O>;
impl<'a, const O: u8> CMPMD2_W<'a, O> {
    #[doc = "Generate the match event when high-side level or more"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CMPMD2_A::_00)
    }
    #[doc = "Generate the match event when low-side level or less"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CMPMD2_A::_01)
    }
    #[doc = "Generate the match event when high-side level or more, or low-side level or less"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMPMD2_A::_10)
    }
    #[doc = "Generate the match event when low-side level or more and high-side level or less"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CMPMD2_A::_11)
    }
}
#[doc = "Field `CMPMD3` reader - Compare Match 3 : Match Mode Selection"]
pub type CMPMD3_R = crate::FieldReader<u8, CMPMD3_A>;
#[doc = "Compare Match 3 : Match Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPMD3_A {
    #[doc = "0: Generate the match event when high-side level or more"]
    _00 = 0,
    #[doc = "1: Generate the match event when low-side level or less"]
    _01 = 1,
    #[doc = "2: Generate the match event when high-side level or more, or low-side level or less"]
    _10 = 2,
    #[doc = "3: Generate the match event when low-side level or more and high-side level or less"]
    _11 = 3,
}
impl From<CMPMD3_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPMD3_A) -> Self {
        variant as _
    }
}
impl CMPMD3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPMD3_A {
        match self.bits {
            0 => CMPMD3_A::_00,
            1 => CMPMD3_A::_01,
            2 => CMPMD3_A::_10,
            3 => CMPMD3_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CMPMD3_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CMPMD3_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CMPMD3_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CMPMD3_A::_11
    }
}
#[doc = "Field `CMPMD3` writer - Compare Match 3 : Match Mode Selection"]
pub type CMPMD3_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADCMPMDR0_SPEC, u8, CMPMD3_A, 2, O>;
impl<'a, const O: u8> CMPMD3_W<'a, O> {
    #[doc = "Generate the match event when high-side level or more"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CMPMD3_A::_00)
    }
    #[doc = "Generate the match event when low-side level or less"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CMPMD3_A::_01)
    }
    #[doc = "Generate the match event when high-side level or more, or low-side level or less"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMPMD3_A::_10)
    }
    #[doc = "Generate the match event when low-side level or more and high-side level or less"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CMPMD3_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Compare Match 0 : Match Mode Selection"]
    #[inline(always)]
    pub fn cmpmd0(&self) -> CMPMD0_R {
        CMPMD0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Compare Match 1 : Match Mode Selection"]
    #[inline(always)]
    pub fn cmpmd1(&self) -> CMPMD1_R {
        CMPMD1_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Compare Match 2 : Match Mode Selection"]
    #[inline(always)]
    pub fn cmpmd2(&self) -> CMPMD2_R {
        CMPMD2_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Compare Match 3 : Match Mode Selection"]
    #[inline(always)]
    pub fn cmpmd3(&self) -> CMPMD3_R {
        CMPMD3_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Compare Match 0 : Match Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmpmd0(&mut self) -> CMPMD0_W<0> {
        CMPMD0_W::new(self)
    }
    #[doc = "Bits 8:9 - Compare Match 1 : Match Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmpmd1(&mut self) -> CMPMD1_W<8> {
        CMPMD1_W::new(self)
    }
    #[doc = "Bits 16:17 - Compare Match 2 : Match Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmpmd2(&mut self) -> CMPMD2_W<16> {
        CMPMD2_W::new(self)
    }
    #[doc = "Bits 24:25 - Compare Match 3 : Match Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmpmd3(&mut self) -> CMPMD3_W<24> {
        CMPMD3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare Match Mode Selection Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpmdr0](index.html) module"]
pub struct ADCMPMDR0_SPEC;
impl crate::RegisterSpec for ADCMPMDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcmpmdr0::R](R) reader structure"]
impl crate::Readable for ADCMPMDR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmpmdr0::W](W) writer structure"]
impl crate::Writable for ADCMPMDR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPMDR0 to value 0"]
impl crate::Resettable for ADCMPMDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
