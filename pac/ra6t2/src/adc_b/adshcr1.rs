#[doc = "Register `ADSHCR1` reader"]
pub struct R(crate::R<ADSHCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADSHCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADSHCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADSHCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADSHCR1` writer"]
pub struct W(crate::W<ADSHCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSHCR1_SPEC>;
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
impl From<crate::W<ADSHCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSHCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHEN4` reader - Channel-dedicated Sample-and-hold Circuit Unit 4 Select"]
pub type SHEN4_R = crate::BitReader<SHEN4_A>;
#[doc = "Channel-dedicated Sample-and-hold Circuit Unit 4 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHEN4_A {
    #[doc = "0: Bypass the circuit unit 4"]
    _0 = 0,
    #[doc = "1: Use the circuit unit 4"]
    _1 = 1,
}
impl From<SHEN4_A> for bool {
    #[inline(always)]
    fn from(variant: SHEN4_A) -> Self {
        variant as u8 != 0
    }
}
impl SHEN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHEN4_A {
        match self.bits {
            false => SHEN4_A::_0,
            true => SHEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHEN4_A::_1
    }
}
#[doc = "Field `SHEN4` writer - Channel-dedicated Sample-and-hold Circuit Unit 4 Select"]
pub type SHEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSHCR1_SPEC, SHEN4_A, O>;
impl<'a, const O: u8> SHEN4_W<'a, O> {
    #[doc = "Bypass the circuit unit 4"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHEN4_A::_0)
    }
    #[doc = "Use the circuit unit 4"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHEN4_A::_1)
    }
}
#[doc = "Field `SHEN5` reader - Channel-dedicated Sample-and-hold Circuit Unit 5 Select"]
pub type SHEN5_R = crate::BitReader<SHEN5_A>;
#[doc = "Channel-dedicated Sample-and-hold Circuit Unit 5 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHEN5_A {
    #[doc = "0: Bypass the circuit unit 5"]
    _0 = 0,
    #[doc = "1: Use the circuit unit 5"]
    _1 = 1,
}
impl From<SHEN5_A> for bool {
    #[inline(always)]
    fn from(variant: SHEN5_A) -> Self {
        variant as u8 != 0
    }
}
impl SHEN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHEN5_A {
        match self.bits {
            false => SHEN5_A::_0,
            true => SHEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHEN5_A::_1
    }
}
#[doc = "Field `SHEN5` writer - Channel-dedicated Sample-and-hold Circuit Unit 5 Select"]
pub type SHEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSHCR1_SPEC, SHEN5_A, O>;
impl<'a, const O: u8> SHEN5_W<'a, O> {
    #[doc = "Bypass the circuit unit 5"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHEN5_A::_0)
    }
    #[doc = "Use the circuit unit 5"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHEN5_A::_1)
    }
}
#[doc = "Field `SHEN6` reader - Channel-dedicated Sample-and-hold Circuit Unit 6 Select"]
pub type SHEN6_R = crate::BitReader<SHEN6_A>;
#[doc = "Channel-dedicated Sample-and-hold Circuit Unit 6 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHEN6_A {
    #[doc = "0: Bypass the circuit unit 6"]
    _0 = 0,
    #[doc = "1: Use the circuit unit 6"]
    _1 = 1,
}
impl From<SHEN6_A> for bool {
    #[inline(always)]
    fn from(variant: SHEN6_A) -> Self {
        variant as u8 != 0
    }
}
impl SHEN6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHEN6_A {
        match self.bits {
            false => SHEN6_A::_0,
            true => SHEN6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHEN6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHEN6_A::_1
    }
}
#[doc = "Field `SHEN6` writer - Channel-dedicated Sample-and-hold Circuit Unit 6 Select"]
pub type SHEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSHCR1_SPEC, SHEN6_A, O>;
impl<'a, const O: u8> SHEN6_W<'a, O> {
    #[doc = "Bypass the circuit unit 6"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHEN6_A::_0)
    }
    #[doc = "Use the circuit unit 6"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHEN6_A::_1)
    }
}
#[doc = "Field `SHMD4` reader - Channel-dedicated Sample-and-hold Circuit Unit 4 Input Mode Select"]
pub type SHMD4_R = crate::BitReader<SHMD4_A>;
#[doc = "Channel-dedicated Sample-and-hold Circuit Unit 4 Input Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHMD4_A {
    #[doc = "0: Single-ended Input mode"]
    _0 = 0,
    #[doc = "1: Differential Input mode"]
    _1 = 1,
}
impl From<SHMD4_A> for bool {
    #[inline(always)]
    fn from(variant: SHMD4_A) -> Self {
        variant as u8 != 0
    }
}
impl SHMD4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHMD4_A {
        match self.bits {
            false => SHMD4_A::_0,
            true => SHMD4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHMD4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHMD4_A::_1
    }
}
#[doc = "Field `SHMD4` writer - Channel-dedicated Sample-and-hold Circuit Unit 4 Input Mode Select"]
pub type SHMD4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSHCR1_SPEC, SHMD4_A, O>;
impl<'a, const O: u8> SHMD4_W<'a, O> {
    #[doc = "Single-ended Input mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHMD4_A::_0)
    }
    #[doc = "Differential Input mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHMD4_A::_1)
    }
}
#[doc = "Field `SHMD5` reader - Channel-dedicated Sample-and-hold Circuit Unit 5 Input Mode Select"]
pub type SHMD5_R = crate::BitReader<SHMD5_A>;
#[doc = "Channel-dedicated Sample-and-hold Circuit Unit 5 Input Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHMD5_A {
    #[doc = "0: Single-ended Input mode"]
    _0 = 0,
    #[doc = "1: Differential Input mode"]
    _1 = 1,
}
impl From<SHMD5_A> for bool {
    #[inline(always)]
    fn from(variant: SHMD5_A) -> Self {
        variant as u8 != 0
    }
}
impl SHMD5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHMD5_A {
        match self.bits {
            false => SHMD5_A::_0,
            true => SHMD5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHMD5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHMD5_A::_1
    }
}
#[doc = "Field `SHMD5` writer - Channel-dedicated Sample-and-hold Circuit Unit 5 Input Mode Select"]
pub type SHMD5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSHCR1_SPEC, SHMD5_A, O>;
impl<'a, const O: u8> SHMD5_W<'a, O> {
    #[doc = "Single-ended Input mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHMD5_A::_0)
    }
    #[doc = "Differential Input mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHMD5_A::_1)
    }
}
#[doc = "Field `SHMD6` reader - Channel-dedicated Sample-and-hold Circuit Unit 6 Input Mode Select"]
pub type SHMD6_R = crate::BitReader<SHMD6_A>;
#[doc = "Channel-dedicated Sample-and-hold Circuit Unit 6 Input Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHMD6_A {
    #[doc = "0: Single-ended Input mode"]
    _0 = 0,
    #[doc = "1: Differential Input mode"]
    _1 = 1,
}
impl From<SHMD6_A> for bool {
    #[inline(always)]
    fn from(variant: SHMD6_A) -> Self {
        variant as u8 != 0
    }
}
impl SHMD6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHMD6_A {
        match self.bits {
            false => SHMD6_A::_0,
            true => SHMD6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHMD6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHMD6_A::_1
    }
}
#[doc = "Field `SHMD6` writer - Channel-dedicated Sample-and-hold Circuit Unit 6 Input Mode Select"]
pub type SHMD6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSHCR1_SPEC, SHMD6_A, O>;
impl<'a, const O: u8> SHMD6_W<'a, O> {
    #[doc = "Single-ended Input mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHMD6_A::_0)
    }
    #[doc = "Differential Input mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHMD6_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel-dedicated Sample-and-hold Circuit Unit 4 Select"]
    #[inline(always)]
    pub fn shen4(&self) -> SHEN4_R {
        SHEN4_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel-dedicated Sample-and-hold Circuit Unit 5 Select"]
    #[inline(always)]
    pub fn shen5(&self) -> SHEN5_R {
        SHEN5_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel-dedicated Sample-and-hold Circuit Unit 6 Select"]
    #[inline(always)]
    pub fn shen6(&self) -> SHEN6_R {
        SHEN6_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel-dedicated Sample-and-hold Circuit Unit 4 Input Mode Select"]
    #[inline(always)]
    pub fn shmd4(&self) -> SHMD4_R {
        SHMD4_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel-dedicated Sample-and-hold Circuit Unit 5 Input Mode Select"]
    #[inline(always)]
    pub fn shmd5(&self) -> SHMD5_R {
        SHMD5_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel-dedicated Sample-and-hold Circuit Unit 6 Input Mode Select"]
    #[inline(always)]
    pub fn shmd6(&self) -> SHMD6_R {
        SHMD6_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel-dedicated Sample-and-hold Circuit Unit 4 Select"]
    #[inline(always)]
    #[must_use]
    pub fn shen4(&mut self) -> SHEN4_W<0> {
        SHEN4_W::new(self)
    }
    #[doc = "Bit 1 - Channel-dedicated Sample-and-hold Circuit Unit 5 Select"]
    #[inline(always)]
    #[must_use]
    pub fn shen5(&mut self) -> SHEN5_W<1> {
        SHEN5_W::new(self)
    }
    #[doc = "Bit 2 - Channel-dedicated Sample-and-hold Circuit Unit 6 Select"]
    #[inline(always)]
    #[must_use]
    pub fn shen6(&mut self) -> SHEN6_W<2> {
        SHEN6_W::new(self)
    }
    #[doc = "Bit 16 - Channel-dedicated Sample-and-hold Circuit Unit 4 Input Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn shmd4(&mut self) -> SHMD4_W<16> {
        SHMD4_W::new(self)
    }
    #[doc = "Bit 17 - Channel-dedicated Sample-and-hold Circuit Unit 5 Input Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn shmd5(&mut self) -> SHMD5_W<17> {
        SHMD5_W::new(self)
    }
    #[doc = "Bit 18 - Channel-dedicated Sample-and-hold Circuit Unit 6 Input Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn shmd6(&mut self) -> SHMD6_W<18> {
        SHMD6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel-dedicated Sample-and-hold Circuit Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adshcr1](index.html) module"]
pub struct ADSHCR1_SPEC;
impl crate::RegisterSpec for ADSHCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adshcr1::R](R) reader structure"]
impl crate::Readable for ADSHCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adshcr1::W](W) writer structure"]
impl crate::Writable for ADSHCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSHCR1 to value 0"]
impl crate::Resettable for ADSHCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
