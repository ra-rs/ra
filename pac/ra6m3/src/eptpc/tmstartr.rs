#[doc = "Register `TMSTARTR` reader"]
pub struct R(crate::R<TMSTARTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMSTARTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMSTARTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMSTARTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMSTARTR` writer"]
pub struct W(crate::W<TMSTARTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMSTARTR_SPEC>;
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
impl From<crate::W<TMSTARTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMSTARTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN0` reader - Pulse Output Timer 0 Start"]
pub type EN0_R = crate::BitReader<EN0_A>;
#[doc = "Pulse Output Timer 0 Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN0_A {
    #[doc = "0: Stops pulse output timer 0."]
    _0 = 0,
    #[doc = "1: Starts pulse output timer 0."]
    _1 = 1,
}
impl From<EN0_A> for bool {
    #[inline(always)]
    fn from(variant: EN0_A) -> Self {
        variant as u8 != 0
    }
}
impl EN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN0_A {
        match self.bits {
            false => EN0_A::_0,
            true => EN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN0_A::_1
    }
}
#[doc = "Field `EN0` writer - Pulse Output Timer 0 Start"]
pub type EN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMSTARTR_SPEC, EN0_A, O>;
impl<'a, const O: u8> EN0_W<'a, O> {
    #[doc = "Stops pulse output timer 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN0_A::_0)
    }
    #[doc = "Starts pulse output timer 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN0_A::_1)
    }
}
#[doc = "Field `EN1` reader - Pulse Output Timer 1 Start"]
pub type EN1_R = crate::BitReader<EN1_A>;
#[doc = "Pulse Output Timer 1 Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN1_A {
    #[doc = "0: Stops pulse output timer 1."]
    _0 = 0,
    #[doc = "1: Starts pulse output timer 1."]
    _1 = 1,
}
impl From<EN1_A> for bool {
    #[inline(always)]
    fn from(variant: EN1_A) -> Self {
        variant as u8 != 0
    }
}
impl EN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN1_A {
        match self.bits {
            false => EN1_A::_0,
            true => EN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN1_A::_1
    }
}
#[doc = "Field `EN1` writer - Pulse Output Timer 1 Start"]
pub type EN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMSTARTR_SPEC, EN1_A, O>;
impl<'a, const O: u8> EN1_W<'a, O> {
    #[doc = "Stops pulse output timer 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN1_A::_0)
    }
    #[doc = "Starts pulse output timer 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN1_A::_1)
    }
}
#[doc = "Field `EN2` reader - Pulse Output Timer 2 Start"]
pub type EN2_R = crate::BitReader<EN2_A>;
#[doc = "Pulse Output Timer 2 Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN2_A {
    #[doc = "0: Stops pulse output timer 2."]
    _0 = 0,
    #[doc = "1: Starts pulse output timer 2."]
    _1 = 1,
}
impl From<EN2_A> for bool {
    #[inline(always)]
    fn from(variant: EN2_A) -> Self {
        variant as u8 != 0
    }
}
impl EN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN2_A {
        match self.bits {
            false => EN2_A::_0,
            true => EN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN2_A::_1
    }
}
#[doc = "Field `EN2` writer - Pulse Output Timer 2 Start"]
pub type EN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMSTARTR_SPEC, EN2_A, O>;
impl<'a, const O: u8> EN2_W<'a, O> {
    #[doc = "Stops pulse output timer 2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN2_A::_0)
    }
    #[doc = "Starts pulse output timer 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN2_A::_1)
    }
}
#[doc = "Field `EN3` reader - Pulse Output Timer 3 Start"]
pub type EN3_R = crate::BitReader<EN3_A>;
#[doc = "Pulse Output Timer 3 Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN3_A {
    #[doc = "0: Stops pulse output timer 3."]
    _0 = 0,
    #[doc = "1: Starts pulse output timer 3."]
    _1 = 1,
}
impl From<EN3_A> for bool {
    #[inline(always)]
    fn from(variant: EN3_A) -> Self {
        variant as u8 != 0
    }
}
impl EN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN3_A {
        match self.bits {
            false => EN3_A::_0,
            true => EN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN3_A::_1
    }
}
#[doc = "Field `EN3` writer - Pulse Output Timer 3 Start"]
pub type EN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMSTARTR_SPEC, EN3_A, O>;
impl<'a, const O: u8> EN3_W<'a, O> {
    #[doc = "Stops pulse output timer 3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN3_A::_0)
    }
    #[doc = "Starts pulse output timer 3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN3_A::_1)
    }
}
#[doc = "Field `EN4` reader - Pulse Output Timer 4 Start"]
pub type EN4_R = crate::BitReader<EN4_A>;
#[doc = "Pulse Output Timer 4 Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN4_A {
    #[doc = "0: Stops pulse output timer 4."]
    _0 = 0,
    #[doc = "1: Starts pulse output timer 4."]
    _1 = 1,
}
impl From<EN4_A> for bool {
    #[inline(always)]
    fn from(variant: EN4_A) -> Self {
        variant as u8 != 0
    }
}
impl EN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN4_A {
        match self.bits {
            false => EN4_A::_0,
            true => EN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN4_A::_1
    }
}
#[doc = "Field `EN4` writer - Pulse Output Timer 4 Start"]
pub type EN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMSTARTR_SPEC, EN4_A, O>;
impl<'a, const O: u8> EN4_W<'a, O> {
    #[doc = "Stops pulse output timer 4."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN4_A::_0)
    }
    #[doc = "Starts pulse output timer 4."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN4_A::_1)
    }
}
#[doc = "Field `EN5` reader - Pulse Output Timer 5 Start"]
pub type EN5_R = crate::BitReader<EN5_A>;
#[doc = "Pulse Output Timer 5 Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN5_A {
    #[doc = "0: Stops pulse output timer 5."]
    _0 = 0,
    #[doc = "1: Starts pulse output timer 5."]
    _1 = 1,
}
impl From<EN5_A> for bool {
    #[inline(always)]
    fn from(variant: EN5_A) -> Self {
        variant as u8 != 0
    }
}
impl EN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN5_A {
        match self.bits {
            false => EN5_A::_0,
            true => EN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN5_A::_1
    }
}
#[doc = "Field `EN5` writer - Pulse Output Timer 5 Start"]
pub type EN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMSTARTR_SPEC, EN5_A, O>;
impl<'a, const O: u8> EN5_W<'a, O> {
    #[doc = "Stops pulse output timer 5."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN5_A::_0)
    }
    #[doc = "Starts pulse output timer 5."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN5_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Pulse Output Timer 0 Start"]
    #[inline(always)]
    pub fn en0(&self) -> EN0_R {
        EN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pulse Output Timer 1 Start"]
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pulse Output Timer 2 Start"]
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pulse Output Timer 3 Start"]
    #[inline(always)]
    pub fn en3(&self) -> EN3_R {
        EN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pulse Output Timer 4 Start"]
    #[inline(always)]
    pub fn en4(&self) -> EN4_R {
        EN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pulse Output Timer 5 Start"]
    #[inline(always)]
    pub fn en5(&self) -> EN5_R {
        EN5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pulse Output Timer 0 Start"]
    #[inline(always)]
    #[must_use]
    pub fn en0(&mut self) -> EN0_W<0> {
        EN0_W::new(self)
    }
    #[doc = "Bit 1 - Pulse Output Timer 1 Start"]
    #[inline(always)]
    #[must_use]
    pub fn en1(&mut self) -> EN1_W<1> {
        EN1_W::new(self)
    }
    #[doc = "Bit 2 - Pulse Output Timer 2 Start"]
    #[inline(always)]
    #[must_use]
    pub fn en2(&mut self) -> EN2_W<2> {
        EN2_W::new(self)
    }
    #[doc = "Bit 3 - Pulse Output Timer 3 Start"]
    #[inline(always)]
    #[must_use]
    pub fn en3(&mut self) -> EN3_W<3> {
        EN3_W::new(self)
    }
    #[doc = "Bit 4 - Pulse Output Timer 4 Start"]
    #[inline(always)]
    #[must_use]
    pub fn en4(&mut self) -> EN4_W<4> {
        EN4_W::new(self)
    }
    #[doc = "Bit 5 - Pulse Output Timer 5 Start"]
    #[inline(always)]
    #[must_use]
    pub fn en5(&mut self) -> EN5_W<5> {
        EN5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Start Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmstartr](index.html) module"]
pub struct TMSTARTR_SPEC;
impl crate::RegisterSpec for TMSTARTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmstartr::R](R) reader structure"]
impl crate::Readable for TMSTARTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmstartr::W](W) writer structure"]
impl crate::Writable for TMSTARTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMSTARTR to value 0"]
impl crate::Resettable for TMSTARTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
