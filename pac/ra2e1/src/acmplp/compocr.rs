#[doc = "Register `COMPOCR` reader"]
pub struct R(crate::R<COMPOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMPOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMPOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMPOCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMPOCR` writer"]
pub struct W(crate::W<COMPOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMPOCR_SPEC>;
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
impl From<crate::W<COMPOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMPOCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C0OE` reader - ACMPLP0 VCOUT Pin Output Enable"]
pub type C0OE_R = crate::BitReader<C0OE_A>;
#[doc = "ACMPLP0 VCOUT Pin Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C0OE_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<C0OE_A> for bool {
    #[inline(always)]
    fn from(variant: C0OE_A) -> Self {
        variant as u8 != 0
    }
}
impl C0OE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C0OE_A {
        match self.bits {
            false => C0OE_A::_0,
            true => C0OE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0OE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0OE_A::_1
    }
}
#[doc = "Field `C0OE` writer - ACMPLP0 VCOUT Pin Output Enable"]
pub type C0OE_W<'a, const O: u8> = crate::BitWriter<'a, u8, COMPOCR_SPEC, C0OE_A, O>;
impl<'a, const O: u8> C0OE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(C0OE_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(C0OE_A::_1)
    }
}
#[doc = "Field `C0OP` reader - ACMPLP0 VCOUT Output Polarity Selection"]
pub type C0OP_R = crate::BitReader<C0OP_A>;
#[doc = "ACMPLP0 VCOUT Output Polarity Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C0OP_A {
    #[doc = "0: Non-inverted"]
    _0 = 0,
    #[doc = "1: Inverted"]
    _1 = 1,
}
impl From<C0OP_A> for bool {
    #[inline(always)]
    fn from(variant: C0OP_A) -> Self {
        variant as u8 != 0
    }
}
impl C0OP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C0OP_A {
        match self.bits {
            false => C0OP_A::_0,
            true => C0OP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0OP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0OP_A::_1
    }
}
#[doc = "Field `C0OP` writer - ACMPLP0 VCOUT Output Polarity Selection"]
pub type C0OP_W<'a, const O: u8> = crate::BitWriter<'a, u8, COMPOCR_SPEC, C0OP_A, O>;
impl<'a, const O: u8> C0OP_W<'a, O> {
    #[doc = "Non-inverted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(C0OP_A::_0)
    }
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(C0OP_A::_1)
    }
}
#[doc = "Field `C1OE` reader - ACMPLP1 VCOUT Pin Output Enable"]
pub type C1OE_R = crate::BitReader<C1OE_A>;
#[doc = "ACMPLP1 VCOUT Pin Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C1OE_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<C1OE_A> for bool {
    #[inline(always)]
    fn from(variant: C1OE_A) -> Self {
        variant as u8 != 0
    }
}
impl C1OE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C1OE_A {
        match self.bits {
            false => C1OE_A::_0,
            true => C1OE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1OE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1OE_A::_1
    }
}
#[doc = "Field `C1OE` writer - ACMPLP1 VCOUT Pin Output Enable"]
pub type C1OE_W<'a, const O: u8> = crate::BitWriter<'a, u8, COMPOCR_SPEC, C1OE_A, O>;
impl<'a, const O: u8> C1OE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(C1OE_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(C1OE_A::_1)
    }
}
#[doc = "Field `C1OP` reader - ACMPLP1 VCOUT Output Polarity Selection"]
pub type C1OP_R = crate::BitReader<C1OP_A>;
#[doc = "ACMPLP1 VCOUT Output Polarity Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C1OP_A {
    #[doc = "0: Non-inverted"]
    _0 = 0,
    #[doc = "1: Inverted"]
    _1 = 1,
}
impl From<C1OP_A> for bool {
    #[inline(always)]
    fn from(variant: C1OP_A) -> Self {
        variant as u8 != 0
    }
}
impl C1OP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C1OP_A {
        match self.bits {
            false => C1OP_A::_0,
            true => C1OP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1OP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1OP_A::_1
    }
}
#[doc = "Field `C1OP` writer - ACMPLP1 VCOUT Output Polarity Selection"]
pub type C1OP_W<'a, const O: u8> = crate::BitWriter<'a, u8, COMPOCR_SPEC, C1OP_A, O>;
impl<'a, const O: u8> C1OP_W<'a, O> {
    #[doc = "Non-inverted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(C1OP_A::_0)
    }
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(C1OP_A::_1)
    }
}
#[doc = "Field `SPDMD` reader - ACMPLP0/ACMPLP1 Speed Selection"]
pub type SPDMD_R = crate::BitReader<SPDMD_A>;
#[doc = "ACMPLP0/ACMPLP1 Speed Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPDMD_A {
    #[doc = "0: Low-speed mode"]
    _0 = 0,
    #[doc = "1: High-speed mode"]
    _1 = 1,
}
impl From<SPDMD_A> for bool {
    #[inline(always)]
    fn from(variant: SPDMD_A) -> Self {
        variant as u8 != 0
    }
}
impl SPDMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPDMD_A {
        match self.bits {
            false => SPDMD_A::_0,
            true => SPDMD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPDMD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPDMD_A::_1
    }
}
#[doc = "Field `SPDMD` writer - ACMPLP0/ACMPLP1 Speed Selection"]
pub type SPDMD_W<'a, const O: u8> = crate::BitWriter<'a, u8, COMPOCR_SPEC, SPDMD_A, O>;
impl<'a, const O: u8> SPDMD_W<'a, O> {
    #[doc = "Low-speed mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPDMD_A::_0)
    }
    #[doc = "High-speed mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPDMD_A::_1)
    }
}
impl R {
    #[doc = "Bit 1 - ACMPLP0 VCOUT Pin Output Enable"]
    #[inline(always)]
    pub fn c0oe(&self) -> C0OE_R {
        C0OE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ACMPLP0 VCOUT Output Polarity Selection"]
    #[inline(always)]
    pub fn c0op(&self) -> C0OP_R {
        C0OP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - ACMPLP1 VCOUT Pin Output Enable"]
    #[inline(always)]
    pub fn c1oe(&self) -> C1OE_R {
        C1OE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ACMPLP1 VCOUT Output Polarity Selection"]
    #[inline(always)]
    pub fn c1op(&self) -> C1OP_R {
        C1OP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ACMPLP0/ACMPLP1 Speed Selection"]
    #[inline(always)]
    pub fn spdmd(&self) -> SPDMD_R {
        SPDMD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - ACMPLP0 VCOUT Pin Output Enable"]
    #[inline(always)]
    pub fn c0oe(&mut self) -> C0OE_W<1> {
        C0OE_W::new(self)
    }
    #[doc = "Bit 2 - ACMPLP0 VCOUT Output Polarity Selection"]
    #[inline(always)]
    pub fn c0op(&mut self) -> C0OP_W<2> {
        C0OP_W::new(self)
    }
    #[doc = "Bit 5 - ACMPLP1 VCOUT Pin Output Enable"]
    #[inline(always)]
    pub fn c1oe(&mut self) -> C1OE_W<5> {
        C1OE_W::new(self)
    }
    #[doc = "Bit 6 - ACMPLP1 VCOUT Output Polarity Selection"]
    #[inline(always)]
    pub fn c1op(&mut self) -> C1OP_W<6> {
        C1OP_W::new(self)
    }
    #[doc = "Bit 7 - ACMPLP0/ACMPLP1 Speed Selection"]
    #[inline(always)]
    pub fn spdmd(&mut self) -> SPDMD_W<7> {
        SPDMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ACMPLP Output Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [compocr](index.html) module"]
pub struct COMPOCR_SPEC;
impl crate::RegisterSpec for COMPOCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [compocr::R](R) reader structure"]
impl crate::Readable for COMPOCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [compocr::W](W) writer structure"]
impl crate::Writable for COMPOCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMPOCR to value 0"]
impl crate::Resettable for COMPOCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
