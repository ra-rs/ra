#[doc = "Register `ADANSB1` reader"]
pub struct R(crate::R<ADANSB1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADANSB1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADANSB1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADANSB1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADANSB1` writer"]
pub struct W(crate::W<ADANSB1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADANSB1_SPEC>;
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
impl From<crate::W<ADANSB1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADANSB1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANSB16` reader - AN016 Select"]
pub type ANSB16_R = crate::BitReader<ANSB16_A>;
#[doc = "AN016 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB16_A {
    #[doc = "0: AN016 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN016 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB16_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB16_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB16_A {
        match self.bits {
            false => ANSB16_A::_0,
            true => ANSB16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB16_A::_1
    }
}
#[doc = "Field `ANSB16` writer - AN016 Select"]
pub type ANSB16_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB16_A, O>;
impl<'a, const O: u8> ANSB16_W<'a, O> {
    #[doc = "AN016 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB16_A::_0)
    }
    #[doc = "AN016 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB16_A::_1)
    }
}
#[doc = "Field `ANSB17` reader - AN017 Select"]
pub type ANSB17_R = crate::BitReader<ANSB17_A>;
#[doc = "AN017 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB17_A {
    #[doc = "0: AN017 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN017 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB17_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB17_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB17_A {
        match self.bits {
            false => ANSB17_A::_0,
            true => ANSB17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB17_A::_1
    }
}
#[doc = "Field `ANSB17` writer - AN017 Select"]
pub type ANSB17_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB17_A, O>;
impl<'a, const O: u8> ANSB17_W<'a, O> {
    #[doc = "AN017 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB17_A::_0)
    }
    #[doc = "AN017 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB17_A::_1)
    }
}
#[doc = "Field `ANSB18` reader - AN018 Select"]
pub type ANSB18_R = crate::BitReader<ANSB18_A>;
#[doc = "AN018 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB18_A {
    #[doc = "0: AN018 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN018 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB18_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB18_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB18_A {
        match self.bits {
            false => ANSB18_A::_0,
            true => ANSB18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB18_A::_1
    }
}
#[doc = "Field `ANSB18` writer - AN018 Select"]
pub type ANSB18_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB18_A, O>;
impl<'a, const O: u8> ANSB18_W<'a, O> {
    #[doc = "AN018 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB18_A::_0)
    }
    #[doc = "AN018 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB18_A::_1)
    }
}
#[doc = "Field `ANSB20` reader - AN020 Select"]
pub type ANSB20_R = crate::BitReader<ANSB20_A>;
#[doc = "AN020 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB20_A {
    #[doc = "0: AN020 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN020 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB20_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB20_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB20_A {
        match self.bits {
            false => ANSB20_A::_0,
            true => ANSB20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB20_A::_1
    }
}
#[doc = "Field `ANSB20` writer - AN020 Select"]
pub type ANSB20_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB20_A, O>;
impl<'a, const O: u8> ANSB20_W<'a, O> {
    #[doc = "AN020 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB20_A::_0)
    }
    #[doc = "AN020 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB20_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AN016 Select"]
    #[inline(always)]
    pub fn ansb16(&self) -> ANSB16_R {
        ANSB16_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AN017 Select"]
    #[inline(always)]
    pub fn ansb17(&self) -> ANSB17_R {
        ANSB17_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AN018 Select"]
    #[inline(always)]
    pub fn ansb18(&self) -> ANSB18_R {
        ANSB18_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - AN020 Select"]
    #[inline(always)]
    pub fn ansb20(&self) -> ANSB20_R {
        ANSB20_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AN016 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb16(&mut self) -> ANSB16_W<0> {
        ANSB16_W::new(self)
    }
    #[doc = "Bit 1 - AN017 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb17(&mut self) -> ANSB17_W<1> {
        ANSB17_W::new(self)
    }
    #[doc = "Bit 2 - AN018 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb18(&mut self) -> ANSB18_W<2> {
        ANSB18_W::new(self)
    }
    #[doc = "Bit 4 - AN020 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb20(&mut self) -> ANSB20_W<4> {
        ANSB20_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Channel Select Register B1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adansb1](index.html) module"]
pub struct ADANSB1_SPEC;
impl crate::RegisterSpec for ADANSB1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adansb1::R](R) reader structure"]
impl crate::Readable for ADANSB1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adansb1::W](W) writer structure"]
impl crate::Writable for ADANSB1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADANSB1 to value 0"]
impl crate::Resettable for ADANSB1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
