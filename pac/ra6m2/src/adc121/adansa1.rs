#[doc = "Register `ADANSA1` reader"]
pub struct R(crate::R<ADANSA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADANSA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADANSA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADANSA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADANSA1` writer"]
pub struct W(crate::W<ADANSA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADANSA1_SPEC>;
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
impl From<crate::W<ADANSA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADANSA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANSA16` reader - AN116 Select"]
pub type ANSA16_R = crate::BitReader<ANSA16_A>;
#[doc = "AN116 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA16_A {
    #[doc = "0: AN116 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN116 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSA16_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA16_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA16_A {
        match self.bits {
            false => ANSA16_A::_0,
            true => ANSA16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA16_A::_1
    }
}
#[doc = "Field `ANSA16` writer - AN116 Select"]
pub type ANSA16_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA1_SPEC, ANSA16_A, O>;
impl<'a, const O: u8> ANSA16_W<'a, O> {
    #[doc = "AN116 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA16_A::_0)
    }
    #[doc = "AN116 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA16_A::_1)
    }
}
#[doc = "Field `ANSA17` reader - AN117 Select"]
pub type ANSA17_R = crate::BitReader<ANSA17_A>;
#[doc = "AN117 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA17_A {
    #[doc = "0: AN117 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN117 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSA17_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA17_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA17_A {
        match self.bits {
            false => ANSA17_A::_0,
            true => ANSA17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA17_A::_1
    }
}
#[doc = "Field `ANSA17` writer - AN117 Select"]
pub type ANSA17_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA1_SPEC, ANSA17_A, O>;
impl<'a, const O: u8> ANSA17_W<'a, O> {
    #[doc = "AN117 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA17_A::_0)
    }
    #[doc = "AN117 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA17_A::_1)
    }
}
#[doc = "Field `ANSA18` reader - AN118 Select"]
pub type ANSA18_R = crate::BitReader<ANSA18_A>;
#[doc = "AN118 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA18_A {
    #[doc = "0: AN118 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN118 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSA18_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA18_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA18_A {
        match self.bits {
            false => ANSA18_A::_0,
            true => ANSA18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA18_A::_1
    }
}
#[doc = "Field `ANSA18` writer - AN118 Select"]
pub type ANSA18_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA1_SPEC, ANSA18_A, O>;
impl<'a, const O: u8> ANSA18_W<'a, O> {
    #[doc = "AN118 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA18_A::_0)
    }
    #[doc = "AN118 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA18_A::_1)
    }
}
#[doc = "Field `ANSA19` reader - AN119 Select"]
pub type ANSA19_R = crate::BitReader<ANSA19_A>;
#[doc = "AN119 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA19_A {
    #[doc = "0: AN119 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN119 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSA19_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA19_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA19_A {
        match self.bits {
            false => ANSA19_A::_0,
            true => ANSA19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA19_A::_1
    }
}
#[doc = "Field `ANSA19` writer - AN119 Select"]
pub type ANSA19_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA1_SPEC, ANSA19_A, O>;
impl<'a, const O: u8> ANSA19_W<'a, O> {
    #[doc = "AN119 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA19_A::_0)
    }
    #[doc = "AN119 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA19_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AN116 Select"]
    #[inline(always)]
    pub fn ansa16(&self) -> ANSA16_R {
        ANSA16_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AN117 Select"]
    #[inline(always)]
    pub fn ansa17(&self) -> ANSA17_R {
        ANSA17_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AN118 Select"]
    #[inline(always)]
    pub fn ansa18(&self) -> ANSA18_R {
        ANSA18_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AN119 Select"]
    #[inline(always)]
    pub fn ansa19(&self) -> ANSA19_R {
        ANSA19_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AN116 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa16(&mut self) -> ANSA16_W<0> {
        ANSA16_W::new(self)
    }
    #[doc = "Bit 1 - AN117 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa17(&mut self) -> ANSA17_W<1> {
        ANSA17_W::new(self)
    }
    #[doc = "Bit 2 - AN118 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa18(&mut self) -> ANSA18_W<2> {
        ANSA18_W::new(self)
    }
    #[doc = "Bit 3 - AN119 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa19(&mut self) -> ANSA19_W<3> {
        ANSA19_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Channel Select Register A1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adansa1](index.html) module"]
pub struct ADANSA1_SPEC;
impl crate::RegisterSpec for ADANSA1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adansa1::R](R) reader structure"]
impl crate::Readable for ADANSA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adansa1::W](W) writer structure"]
impl crate::Writable for ADANSA1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADANSA1 to value 0"]
impl crate::Resettable for ADANSA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
