#[doc = "Register `COMPFIR` reader"]
pub struct R(crate::R<COMPFIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMPFIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMPFIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMPFIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMPFIR` writer"]
pub struct W(crate::W<COMPFIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMPFIR_SPEC>;
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
impl From<crate::W<COMPFIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMPFIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C0FCK` reader - ACMPLP0 Filter Select"]
pub type C0FCK_R = crate::FieldReader<u8, C0FCK_A>;
#[doc = "ACMPLP0 Filter Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C0FCK_A {
    #[doc = "0: No Sampling (bypass)"]
    _00 = 0,
    #[doc = "1: Sampling at PCLK"]
    _01 = 1,
    #[doc = "2: Sampling at PCLK/8"]
    _10 = 2,
    #[doc = "3: Sampling at PCLK/32"]
    _11 = 3,
}
impl From<C0FCK_A> for u8 {
    #[inline(always)]
    fn from(variant: C0FCK_A) -> Self {
        variant as _
    }
}
impl C0FCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C0FCK_A {
        match self.bits {
            0 => C0FCK_A::_00,
            1 => C0FCK_A::_01,
            2 => C0FCK_A::_10,
            3 => C0FCK_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == C0FCK_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == C0FCK_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == C0FCK_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == C0FCK_A::_11
    }
}
#[doc = "Field `C0FCK` writer - ACMPLP0 Filter Select"]
pub type C0FCK_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, COMPFIR_SPEC, u8, C0FCK_A, 2, O>;
impl<'a, const O: u8> C0FCK_W<'a, O> {
    #[doc = "No Sampling (bypass)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(C0FCK_A::_00)
    }
    #[doc = "Sampling at PCLK"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(C0FCK_A::_01)
    }
    #[doc = "Sampling at PCLK/8"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(C0FCK_A::_10)
    }
    #[doc = "Sampling at PCLK/32"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(C0FCK_A::_11)
    }
}
#[doc = "Field `C0EPO` reader - ACMPLP0 Edge Polarity Switching"]
pub type C0EPO_R = crate::BitReader<C0EPO_A>;
#[doc = "ACMPLP0 Edge Polarity Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0EPO_A {
    #[doc = "0: Interrupt and ELC event request at rising edge"]
    _0 = 0,
    #[doc = "1: Interrupt and ELC event request at falling edge"]
    _1 = 1,
}
impl From<C0EPO_A> for bool {
    #[inline(always)]
    fn from(variant: C0EPO_A) -> Self {
        variant as u8 != 0
    }
}
impl C0EPO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C0EPO_A {
        match self.bits {
            false => C0EPO_A::_0,
            true => C0EPO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0EPO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0EPO_A::_1
    }
}
#[doc = "Field `C0EPO` writer - ACMPLP0 Edge Polarity Switching"]
pub type C0EPO_W<'a, const O: u8> = crate::BitWriter<'a, u8, COMPFIR_SPEC, C0EPO_A, O>;
impl<'a, const O: u8> C0EPO_W<'a, O> {
    #[doc = "Interrupt and ELC event request at rising edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(C0EPO_A::_0)
    }
    #[doc = "Interrupt and ELC event request at falling edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(C0EPO_A::_1)
    }
}
#[doc = "Field `C0EDG` reader - ACMPLP0 Edge Detection Selection"]
pub type C0EDG_R = crate::BitReader<C0EDG_A>;
#[doc = "ACMPLP0 Edge Detection Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0EDG_A {
    #[doc = "0: Interrupt and ELC event request by one-edge detection"]
    _0 = 0,
    #[doc = "1: Interrupt and ELC event request by both-edge detection"]
    _1 = 1,
}
impl From<C0EDG_A> for bool {
    #[inline(always)]
    fn from(variant: C0EDG_A) -> Self {
        variant as u8 != 0
    }
}
impl C0EDG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C0EDG_A {
        match self.bits {
            false => C0EDG_A::_0,
            true => C0EDG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0EDG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0EDG_A::_1
    }
}
#[doc = "Field `C0EDG` writer - ACMPLP0 Edge Detection Selection"]
pub type C0EDG_W<'a, const O: u8> = crate::BitWriter<'a, u8, COMPFIR_SPEC, C0EDG_A, O>;
impl<'a, const O: u8> C0EDG_W<'a, O> {
    #[doc = "Interrupt and ELC event request by one-edge detection"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(C0EDG_A::_0)
    }
    #[doc = "Interrupt and ELC event request by both-edge detection"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(C0EDG_A::_1)
    }
}
#[doc = "Field `C1FCK` reader - ACMPLP1 Filter Select"]
pub type C1FCK_R = crate::FieldReader<u8, C1FCK_A>;
#[doc = "ACMPLP1 Filter Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C1FCK_A {
    #[doc = "0: No Sampling (bypass)"]
    _00 = 0,
    #[doc = "1: Sampling at PCLK"]
    _01 = 1,
    #[doc = "2: Sampling at PCLK/8"]
    _10 = 2,
    #[doc = "3: Sampling at PCLK/32"]
    _11 = 3,
}
impl From<C1FCK_A> for u8 {
    #[inline(always)]
    fn from(variant: C1FCK_A) -> Self {
        variant as _
    }
}
impl C1FCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C1FCK_A {
        match self.bits {
            0 => C1FCK_A::_00,
            1 => C1FCK_A::_01,
            2 => C1FCK_A::_10,
            3 => C1FCK_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == C1FCK_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == C1FCK_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == C1FCK_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == C1FCK_A::_11
    }
}
#[doc = "Field `C1FCK` writer - ACMPLP1 Filter Select"]
pub type C1FCK_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, COMPFIR_SPEC, u8, C1FCK_A, 2, O>;
impl<'a, const O: u8> C1FCK_W<'a, O> {
    #[doc = "No Sampling (bypass)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(C1FCK_A::_00)
    }
    #[doc = "Sampling at PCLK"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(C1FCK_A::_01)
    }
    #[doc = "Sampling at PCLK/8"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(C1FCK_A::_10)
    }
    #[doc = "Sampling at PCLK/32"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(C1FCK_A::_11)
    }
}
#[doc = "Field `C1EPO` reader - ACMPLP1 Edge Polarity Switching"]
pub type C1EPO_R = crate::BitReader<C1EPO_A>;
#[doc = "ACMPLP1 Edge Polarity Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1EPO_A {
    #[doc = "0: Interrupt and ELC event request at rising edge"]
    _0 = 0,
    #[doc = "1: Interrupt and ELC event request at falling edge"]
    _1 = 1,
}
impl From<C1EPO_A> for bool {
    #[inline(always)]
    fn from(variant: C1EPO_A) -> Self {
        variant as u8 != 0
    }
}
impl C1EPO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C1EPO_A {
        match self.bits {
            false => C1EPO_A::_0,
            true => C1EPO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1EPO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1EPO_A::_1
    }
}
#[doc = "Field `C1EPO` writer - ACMPLP1 Edge Polarity Switching"]
pub type C1EPO_W<'a, const O: u8> = crate::BitWriter<'a, u8, COMPFIR_SPEC, C1EPO_A, O>;
impl<'a, const O: u8> C1EPO_W<'a, O> {
    #[doc = "Interrupt and ELC event request at rising edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(C1EPO_A::_0)
    }
    #[doc = "Interrupt and ELC event request at falling edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(C1EPO_A::_1)
    }
}
#[doc = "Field `C1EDG` reader - ACMPLP1 Edge Detection Selection"]
pub type C1EDG_R = crate::BitReader<C1EDG_A>;
#[doc = "ACMPLP1 Edge Detection Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1EDG_A {
    #[doc = "0: Interrupt and ELC event request by one-edge detection"]
    _0 = 0,
    #[doc = "1: Interrupt and ELC event request by both-edge detection"]
    _1 = 1,
}
impl From<C1EDG_A> for bool {
    #[inline(always)]
    fn from(variant: C1EDG_A) -> Self {
        variant as u8 != 0
    }
}
impl C1EDG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C1EDG_A {
        match self.bits {
            false => C1EDG_A::_0,
            true => C1EDG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1EDG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1EDG_A::_1
    }
}
#[doc = "Field `C1EDG` writer - ACMPLP1 Edge Detection Selection"]
pub type C1EDG_W<'a, const O: u8> = crate::BitWriter<'a, u8, COMPFIR_SPEC, C1EDG_A, O>;
impl<'a, const O: u8> C1EDG_W<'a, O> {
    #[doc = "Interrupt and ELC event request by one-edge detection"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(C1EDG_A::_0)
    }
    #[doc = "Interrupt and ELC event request by both-edge detection"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(C1EDG_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - ACMPLP0 Filter Select"]
    #[inline(always)]
    pub fn c0fck(&self) -> C0FCK_R {
        C0FCK_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - ACMPLP0 Edge Polarity Switching"]
    #[inline(always)]
    pub fn c0epo(&self) -> C0EPO_R {
        C0EPO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ACMPLP0 Edge Detection Selection"]
    #[inline(always)]
    pub fn c0edg(&self) -> C0EDG_R {
        C0EDG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - ACMPLP1 Filter Select"]
    #[inline(always)]
    pub fn c1fck(&self) -> C1FCK_R {
        C1FCK_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - ACMPLP1 Edge Polarity Switching"]
    #[inline(always)]
    pub fn c1epo(&self) -> C1EPO_R {
        C1EPO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ACMPLP1 Edge Detection Selection"]
    #[inline(always)]
    pub fn c1edg(&self) -> C1EDG_R {
        C1EDG_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ACMPLP0 Filter Select"]
    #[inline(always)]
    #[must_use]
    pub fn c0fck(&mut self) -> C0FCK_W<0> {
        C0FCK_W::new(self)
    }
    #[doc = "Bit 2 - ACMPLP0 Edge Polarity Switching"]
    #[inline(always)]
    #[must_use]
    pub fn c0epo(&mut self) -> C0EPO_W<2> {
        C0EPO_W::new(self)
    }
    #[doc = "Bit 3 - ACMPLP0 Edge Detection Selection"]
    #[inline(always)]
    #[must_use]
    pub fn c0edg(&mut self) -> C0EDG_W<3> {
        C0EDG_W::new(self)
    }
    #[doc = "Bits 4:5 - ACMPLP1 Filter Select"]
    #[inline(always)]
    #[must_use]
    pub fn c1fck(&mut self) -> C1FCK_W<4> {
        C1FCK_W::new(self)
    }
    #[doc = "Bit 6 - ACMPLP1 Edge Polarity Switching"]
    #[inline(always)]
    #[must_use]
    pub fn c1epo(&mut self) -> C1EPO_W<6> {
        C1EPO_W::new(self)
    }
    #[doc = "Bit 7 - ACMPLP1 Edge Detection Selection"]
    #[inline(always)]
    #[must_use]
    pub fn c1edg(&mut self) -> C1EDG_W<7> {
        C1EDG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ACMPLP Filter Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [compfir](index.html) module"]
pub struct COMPFIR_SPEC;
impl crate::RegisterSpec for COMPFIR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [compfir::R](R) reader structure"]
impl crate::Readable for COMPFIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [compfir::W](W) writer structure"]
impl crate::Writable for COMPFIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMPFIR to value 0"]
impl crate::Resettable for COMPFIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
