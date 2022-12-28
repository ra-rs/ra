#[doc = "Register `ADSHCR0` reader"]
pub struct R(crate::R<ADSHCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADSHCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADSHCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADSHCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADSHCR0` writer"]
pub struct W(crate::W<ADSHCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSHCR0_SPEC>;
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
impl From<crate::W<ADSHCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSHCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHEN0` reader - Channel-dedicated Sample-and-hold Circuit Unit 0 Select"]
pub type SHEN0_R = crate::BitReader<SHEN0_A>;
#[doc = "Channel-dedicated Sample-and-hold Circuit Unit 0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHEN0_A {
    #[doc = "0: Bypass the circuit unit 0"]
    _0 = 0,
    #[doc = "1: Use the circuit unit 0"]
    _1 = 1,
}
impl From<SHEN0_A> for bool {
    #[inline(always)]
    fn from(variant: SHEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl SHEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHEN0_A {
        match self.bits {
            false => SHEN0_A::_0,
            true => SHEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHEN0_A::_1
    }
}
#[doc = "Field `SHEN0` writer - Channel-dedicated Sample-and-hold Circuit Unit 0 Select"]
pub type SHEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSHCR0_SPEC, SHEN0_A, O>;
impl<'a, const O: u8> SHEN0_W<'a, O> {
    #[doc = "Bypass the circuit unit 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHEN0_A::_0)
    }
    #[doc = "Use the circuit unit 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHEN0_A::_1)
    }
}
#[doc = "Field `SHEN1` reader - Channel-dedicated Sample-and-hold Circuit Unit 1 Select"]
pub type SHEN1_R = crate::BitReader<SHEN1_A>;
#[doc = "Channel-dedicated Sample-and-hold Circuit Unit 1 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHEN1_A {
    #[doc = "0: Bypass the circuit unit 1"]
    _0 = 0,
    #[doc = "1: Use the circuit unit 1"]
    _1 = 1,
}
impl From<SHEN1_A> for bool {
    #[inline(always)]
    fn from(variant: SHEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl SHEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHEN1_A {
        match self.bits {
            false => SHEN1_A::_0,
            true => SHEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHEN1_A::_1
    }
}
#[doc = "Field `SHEN1` writer - Channel-dedicated Sample-and-hold Circuit Unit 1 Select"]
pub type SHEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSHCR0_SPEC, SHEN1_A, O>;
impl<'a, const O: u8> SHEN1_W<'a, O> {
    #[doc = "Bypass the circuit unit 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHEN1_A::_0)
    }
    #[doc = "Use the circuit unit 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHEN1_A::_1)
    }
}
#[doc = "Field `SHEN2` reader - Channel-dedicated Sample-and-hold Circuit Unit 2 Select"]
pub type SHEN2_R = crate::BitReader<SHEN2_A>;
#[doc = "Channel-dedicated Sample-and-hold Circuit Unit 2 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHEN2_A {
    #[doc = "0: Bypass the circuit unit 2"]
    _0 = 0,
    #[doc = "1: Use the circuit unit 2"]
    _1 = 1,
}
impl From<SHEN2_A> for bool {
    #[inline(always)]
    fn from(variant: SHEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl SHEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHEN2_A {
        match self.bits {
            false => SHEN2_A::_0,
            true => SHEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHEN2_A::_1
    }
}
#[doc = "Field `SHEN2` writer - Channel-dedicated Sample-and-hold Circuit Unit 2 Select"]
pub type SHEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSHCR0_SPEC, SHEN2_A, O>;
impl<'a, const O: u8> SHEN2_W<'a, O> {
    #[doc = "Bypass the circuit unit 2"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHEN2_A::_0)
    }
    #[doc = "Use the circuit unit 2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHEN2_A::_1)
    }
}
#[doc = "Field `SHMD0` reader - Channel-dedicated Sample-and-hold Circuit Unit 0 Input Mode Select"]
pub type SHMD0_R = crate::BitReader<SHMD0_A>;
#[doc = "Channel-dedicated Sample-and-hold Circuit Unit 0 Input Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHMD0_A {
    #[doc = "0: Single-ended Input mode"]
    _0 = 0,
    #[doc = "1: Differential Input mode"]
    _1 = 1,
}
impl From<SHMD0_A> for bool {
    #[inline(always)]
    fn from(variant: SHMD0_A) -> Self {
        variant as u8 != 0
    }
}
impl SHMD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHMD0_A {
        match self.bits {
            false => SHMD0_A::_0,
            true => SHMD0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHMD0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHMD0_A::_1
    }
}
#[doc = "Field `SHMD0` writer - Channel-dedicated Sample-and-hold Circuit Unit 0 Input Mode Select"]
pub type SHMD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSHCR0_SPEC, SHMD0_A, O>;
impl<'a, const O: u8> SHMD0_W<'a, O> {
    #[doc = "Single-ended Input mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHMD0_A::_0)
    }
    #[doc = "Differential Input mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHMD0_A::_1)
    }
}
#[doc = "Field `SHMD1` reader - Channel-dedicated Sample-and-hold Circuit Unit 1 Input Mode Select"]
pub type SHMD1_R = crate::BitReader<SHMD1_A>;
#[doc = "Channel-dedicated Sample-and-hold Circuit Unit 1 Input Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHMD1_A {
    #[doc = "0: Single-ended Input mode"]
    _0 = 0,
    #[doc = "1: Differential Input mode"]
    _1 = 1,
}
impl From<SHMD1_A> for bool {
    #[inline(always)]
    fn from(variant: SHMD1_A) -> Self {
        variant as u8 != 0
    }
}
impl SHMD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHMD1_A {
        match self.bits {
            false => SHMD1_A::_0,
            true => SHMD1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHMD1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHMD1_A::_1
    }
}
#[doc = "Field `SHMD1` writer - Channel-dedicated Sample-and-hold Circuit Unit 1 Input Mode Select"]
pub type SHMD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSHCR0_SPEC, SHMD1_A, O>;
impl<'a, const O: u8> SHMD1_W<'a, O> {
    #[doc = "Single-ended Input mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHMD1_A::_0)
    }
    #[doc = "Differential Input mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHMD1_A::_1)
    }
}
#[doc = "Field `SHMD2` reader - Channel-dedicated Sample-and-hold Circuit Unit 2 Input Mode Select"]
pub type SHMD2_R = crate::BitReader<SHMD2_A>;
#[doc = "Channel-dedicated Sample-and-hold Circuit Unit 2 Input Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHMD2_A {
    #[doc = "0: Single-ended Input mode"]
    _0 = 0,
    #[doc = "1: Differential Input mode"]
    _1 = 1,
}
impl From<SHMD2_A> for bool {
    #[inline(always)]
    fn from(variant: SHMD2_A) -> Self {
        variant as u8 != 0
    }
}
impl SHMD2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHMD2_A {
        match self.bits {
            false => SHMD2_A::_0,
            true => SHMD2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHMD2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHMD2_A::_1
    }
}
#[doc = "Field `SHMD2` writer - Channel-dedicated Sample-and-hold Circuit Unit 2 Input Mode Select"]
pub type SHMD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSHCR0_SPEC, SHMD2_A, O>;
impl<'a, const O: u8> SHMD2_W<'a, O> {
    #[doc = "Single-ended Input mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHMD2_A::_0)
    }
    #[doc = "Differential Input mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHMD2_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel-dedicated Sample-and-hold Circuit Unit 0 Select"]
    #[inline(always)]
    pub fn shen0(&self) -> SHEN0_R {
        SHEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel-dedicated Sample-and-hold Circuit Unit 1 Select"]
    #[inline(always)]
    pub fn shen1(&self) -> SHEN1_R {
        SHEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel-dedicated Sample-and-hold Circuit Unit 2 Select"]
    #[inline(always)]
    pub fn shen2(&self) -> SHEN2_R {
        SHEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel-dedicated Sample-and-hold Circuit Unit 0 Input Mode Select"]
    #[inline(always)]
    pub fn shmd0(&self) -> SHMD0_R {
        SHMD0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel-dedicated Sample-and-hold Circuit Unit 1 Input Mode Select"]
    #[inline(always)]
    pub fn shmd1(&self) -> SHMD1_R {
        SHMD1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel-dedicated Sample-and-hold Circuit Unit 2 Input Mode Select"]
    #[inline(always)]
    pub fn shmd2(&self) -> SHMD2_R {
        SHMD2_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel-dedicated Sample-and-hold Circuit Unit 0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn shen0(&mut self) -> SHEN0_W<0> {
        SHEN0_W::new(self)
    }
    #[doc = "Bit 1 - Channel-dedicated Sample-and-hold Circuit Unit 1 Select"]
    #[inline(always)]
    #[must_use]
    pub fn shen1(&mut self) -> SHEN1_W<1> {
        SHEN1_W::new(self)
    }
    #[doc = "Bit 2 - Channel-dedicated Sample-and-hold Circuit Unit 2 Select"]
    #[inline(always)]
    #[must_use]
    pub fn shen2(&mut self) -> SHEN2_W<2> {
        SHEN2_W::new(self)
    }
    #[doc = "Bit 16 - Channel-dedicated Sample-and-hold Circuit Unit 0 Input Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn shmd0(&mut self) -> SHMD0_W<16> {
        SHMD0_W::new(self)
    }
    #[doc = "Bit 17 - Channel-dedicated Sample-and-hold Circuit Unit 1 Input Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn shmd1(&mut self) -> SHMD1_W<17> {
        SHMD1_W::new(self)
    }
    #[doc = "Bit 18 - Channel-dedicated Sample-and-hold Circuit Unit 2 Input Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn shmd2(&mut self) -> SHMD2_W<18> {
        SHMD2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel-dedicated Sample-and-hold Circuit Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adshcr0](index.html) module"]
pub struct ADSHCR0_SPEC;
impl crate::RegisterSpec for ADSHCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adshcr0::R](R) reader structure"]
impl crate::Readable for ADSHCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adshcr0::W](W) writer structure"]
impl crate::Writable for ADSHCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSHCR0 to value 0"]
impl crate::Resettable for ADSHCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
