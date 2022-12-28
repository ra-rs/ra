#[doc = "Register `ADMDR` reader"]
pub struct R(crate::R<ADMDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADMDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADMDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADMDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADMDR` writer"]
pub struct W(crate::W<ADMDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADMDR_SPEC>;
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
impl From<crate::W<ADMDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADMDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADMD0` reader - ADC0 Mode Selection"]
pub type ADMD0_R = crate::FieldReader<u8, ADMD0_A>;
#[doc = "ADC0 Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADMD0_A {
    #[doc = "0: SAR mode â\u{80}\u{93} Single scan mode"]
    _0X0 = 0,
    #[doc = "1: SAR mode â\u{80}\u{93} Continuous scan mode"]
    _0X1 = 1,
    #[doc = "4: Oversampling mode â\u{80}\u{93} Single scan mode"]
    _0X4 = 4,
    #[doc = "5: Oversampling mode â\u{80}\u{93} Continuous scan mode"]
    _0X5 = 5,
    #[doc = "8: Hybrid mode â\u{80}\u{93} Single scan mode"]
    _0X8 = 8,
    #[doc = "9: Hybrid mode â\u{80}\u{93} Continuous scan mode"]
    _0X9 = 9,
    #[doc = "10: Hybrid mode â\u{80}\u{93} Background continuous scan mode"]
    _0X_A = 10,
}
impl From<ADMD0_A> for u8 {
    #[inline(always)]
    fn from(variant: ADMD0_A) -> Self {
        variant as _
    }
}
impl ADMD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADMD0_A> {
        match self.bits {
            0 => Some(ADMD0_A::_0X0),
            1 => Some(ADMD0_A::_0X1),
            4 => Some(ADMD0_A::_0X4),
            5 => Some(ADMD0_A::_0X5),
            8 => Some(ADMD0_A::_0X8),
            9 => Some(ADMD0_A::_0X9),
            10 => Some(ADMD0_A::_0X_A),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == ADMD0_A::_0X0
    }
    #[doc = "Checks if the value of the field is `_0X1`"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == ADMD0_A::_0X1
    }
    #[doc = "Checks if the value of the field is `_0X4`"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == ADMD0_A::_0X4
    }
    #[doc = "Checks if the value of the field is `_0X5`"]
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == ADMD0_A::_0X5
    }
    #[doc = "Checks if the value of the field is `_0X8`"]
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == ADMD0_A::_0X8
    }
    #[doc = "Checks if the value of the field is `_0X9`"]
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == ADMD0_A::_0X9
    }
    #[doc = "Checks if the value of the field is `_0X_A`"]
    #[inline(always)]
    pub fn is_0x_a(&self) -> bool {
        *self == ADMD0_A::_0X_A
    }
}
#[doc = "Field `ADMD0` writer - ADC0 Mode Selection"]
pub type ADMD0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADMDR_SPEC, u8, ADMD0_A, 4, O>;
impl<'a, const O: u8> ADMD0_W<'a, O> {
    #[doc = "SAR mode â\u{80}\u{93} Single scan mode"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(ADMD0_A::_0X0)
    }
    #[doc = "SAR mode â\u{80}\u{93} Continuous scan mode"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut W {
        self.variant(ADMD0_A::_0X1)
    }
    #[doc = "Oversampling mode â\u{80}\u{93} Single scan mode"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut W {
        self.variant(ADMD0_A::_0X4)
    }
    #[doc = "Oversampling mode â\u{80}\u{93} Continuous scan mode"]
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut W {
        self.variant(ADMD0_A::_0X5)
    }
    #[doc = "Hybrid mode â\u{80}\u{93} Single scan mode"]
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut W {
        self.variant(ADMD0_A::_0X8)
    }
    #[doc = "Hybrid mode â\u{80}\u{93} Continuous scan mode"]
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut W {
        self.variant(ADMD0_A::_0X9)
    }
    #[doc = "Hybrid mode â\u{80}\u{93} Background continuous scan mode"]
    #[inline(always)]
    pub fn _0x_a(self) -> &'a mut W {
        self.variant(ADMD0_A::_0X_A)
    }
}
#[doc = "Field `ADMD1` reader - ADC1 Mode Selection"]
pub type ADMD1_R = crate::FieldReader<u8, ADMD1_A>;
#[doc = "ADC1 Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADMD1_A {
    #[doc = "0: SAR mode â\u{80}\u{93} Single scan mode"]
    _0X0 = 0,
    #[doc = "1: SAR mode â\u{80}\u{93} Continuous scan mode"]
    _0X1 = 1,
    #[doc = "4: Oversampling mode â\u{80}\u{93} Single scan mode"]
    _0X4 = 4,
    #[doc = "5: Oversampling mode â\u{80}\u{93} Continuous scan mode"]
    _0X5 = 5,
    #[doc = "8: Hybrid mode â\u{80}\u{93} Single scan mode"]
    _0X8 = 8,
    #[doc = "9: Hybrid mode â\u{80}\u{93} Continuous scan mode"]
    _0X9 = 9,
    #[doc = "10: Hybrid mode â\u{80}\u{93} Background continuous scan mode"]
    _0X_A = 10,
}
impl From<ADMD1_A> for u8 {
    #[inline(always)]
    fn from(variant: ADMD1_A) -> Self {
        variant as _
    }
}
impl ADMD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADMD1_A> {
        match self.bits {
            0 => Some(ADMD1_A::_0X0),
            1 => Some(ADMD1_A::_0X1),
            4 => Some(ADMD1_A::_0X4),
            5 => Some(ADMD1_A::_0X5),
            8 => Some(ADMD1_A::_0X8),
            9 => Some(ADMD1_A::_0X9),
            10 => Some(ADMD1_A::_0X_A),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == ADMD1_A::_0X0
    }
    #[doc = "Checks if the value of the field is `_0X1`"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == ADMD1_A::_0X1
    }
    #[doc = "Checks if the value of the field is `_0X4`"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == ADMD1_A::_0X4
    }
    #[doc = "Checks if the value of the field is `_0X5`"]
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == ADMD1_A::_0X5
    }
    #[doc = "Checks if the value of the field is `_0X8`"]
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == ADMD1_A::_0X8
    }
    #[doc = "Checks if the value of the field is `_0X9`"]
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == ADMD1_A::_0X9
    }
    #[doc = "Checks if the value of the field is `_0X_A`"]
    #[inline(always)]
    pub fn is_0x_a(&self) -> bool {
        *self == ADMD1_A::_0X_A
    }
}
#[doc = "Field `ADMD1` writer - ADC1 Mode Selection"]
pub type ADMD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADMDR_SPEC, u8, ADMD1_A, 4, O>;
impl<'a, const O: u8> ADMD1_W<'a, O> {
    #[doc = "SAR mode â\u{80}\u{93} Single scan mode"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(ADMD1_A::_0X0)
    }
    #[doc = "SAR mode â\u{80}\u{93} Continuous scan mode"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut W {
        self.variant(ADMD1_A::_0X1)
    }
    #[doc = "Oversampling mode â\u{80}\u{93} Single scan mode"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut W {
        self.variant(ADMD1_A::_0X4)
    }
    #[doc = "Oversampling mode â\u{80}\u{93} Continuous scan mode"]
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut W {
        self.variant(ADMD1_A::_0X5)
    }
    #[doc = "Hybrid mode â\u{80}\u{93} Single scan mode"]
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut W {
        self.variant(ADMD1_A::_0X8)
    }
    #[doc = "Hybrid mode â\u{80}\u{93} Continuous scan mode"]
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut W {
        self.variant(ADMD1_A::_0X9)
    }
    #[doc = "Hybrid mode â\u{80}\u{93} Background continuous scan mode"]
    #[inline(always)]
    pub fn _0x_a(self) -> &'a mut W {
        self.variant(ADMD1_A::_0X_A)
    }
}
impl R {
    #[doc = "Bits 0:3 - ADC0 Mode Selection"]
    #[inline(always)]
    pub fn admd0(&self) -> ADMD0_R {
        ADMD0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ADC1 Mode Selection"]
    #[inline(always)]
    pub fn admd1(&self) -> ADMD1_R {
        ADMD1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC0 Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn admd0(&mut self) -> ADMD0_W<0> {
        ADMD0_W::new(self)
    }
    #[doc = "Bits 8:11 - ADC1 Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn admd1(&mut self) -> ADMD1_W<8> {
        ADMD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Converter Mode Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [admdr](index.html) module"]
pub struct ADMDR_SPEC;
impl crate::RegisterSpec for ADMDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [admdr::R](R) reader structure"]
impl crate::Readable for ADMDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [admdr::W](W) writer structure"]
impl crate::Writable for ADMDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADMDR to value 0"]
impl crate::Resettable for ADMDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
