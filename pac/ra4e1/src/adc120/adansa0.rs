#[doc = "Register `ADANSA0` reader"]
pub struct R(crate::R<ADANSA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADANSA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADANSA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADANSA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADANSA0` writer"]
pub struct W(crate::W<ADANSA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADANSA0_SPEC>;
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
impl From<crate::W<ADANSA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADANSA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANSA0` reader - A/D Conversion Channels Select n"]
pub type ANSA0_R = crate::BitReader<ANSA0_A>;
#[doc = "A/D Conversion Channels Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA0_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSA0_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA0_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA0_A {
        match self.bits {
            false => ANSA0_A::_0,
            true => ANSA0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA0_A::_1
    }
}
#[doc = "Field `ANSA0` writer - A/D Conversion Channels Select n"]
pub type ANSA0_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA0_SPEC, ANSA0_A, O>;
impl<'a, const O: u8> ANSA0_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA0_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA0_A::_1)
    }
}
#[doc = "Field `ANSA1` reader - A/D Conversion Channels Select n"]
pub type ANSA1_R = crate::BitReader<ANSA1_A>;
#[doc = "A/D Conversion Channels Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA1_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSA1_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA1_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA1_A {
        match self.bits {
            false => ANSA1_A::_0,
            true => ANSA1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA1_A::_1
    }
}
#[doc = "Field `ANSA1` writer - A/D Conversion Channels Select n"]
pub type ANSA1_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA0_SPEC, ANSA1_A, O>;
impl<'a, const O: u8> ANSA1_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA1_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA1_A::_1)
    }
}
#[doc = "Field `ANSA2` reader - A/D Conversion Channels Select n"]
pub type ANSA2_R = crate::BitReader<ANSA2_A>;
#[doc = "A/D Conversion Channels Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA2_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSA2_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA2_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA2_A {
        match self.bits {
            false => ANSA2_A::_0,
            true => ANSA2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA2_A::_1
    }
}
#[doc = "Field `ANSA2` writer - A/D Conversion Channels Select n"]
pub type ANSA2_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA0_SPEC, ANSA2_A, O>;
impl<'a, const O: u8> ANSA2_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA2_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA2_A::_1)
    }
}
#[doc = "Field `ANSA3` reader - A/D Conversion Channels Select n"]
pub type ANSA3_R = crate::BitReader<ANSA3_A>;
#[doc = "A/D Conversion Channels Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA3_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSA3_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA3_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA3_A {
        match self.bits {
            false => ANSA3_A::_0,
            true => ANSA3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA3_A::_1
    }
}
#[doc = "Field `ANSA3` writer - A/D Conversion Channels Select n"]
pub type ANSA3_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA0_SPEC, ANSA3_A, O>;
impl<'a, const O: u8> ANSA3_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA3_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA3_A::_1)
    }
}
#[doc = "Field `ANSA4` reader - A/D Conversion Channels Select n"]
pub type ANSA4_R = crate::BitReader<ANSA4_A>;
#[doc = "A/D Conversion Channels Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA4_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSA4_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA4_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA4_A {
        match self.bits {
            false => ANSA4_A::_0,
            true => ANSA4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA4_A::_1
    }
}
#[doc = "Field `ANSA4` writer - A/D Conversion Channels Select n"]
pub type ANSA4_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA0_SPEC, ANSA4_A, O>;
impl<'a, const O: u8> ANSA4_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA4_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA4_A::_1)
    }
}
#[doc = "Field `ANSA11` reader - A/D Conversion Channels Select n"]
pub type ANSA11_R = crate::BitReader<ANSA11_A>;
#[doc = "A/D Conversion Channels Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA11_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSA11_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA11_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA11_A {
        match self.bits {
            false => ANSA11_A::_0,
            true => ANSA11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA11_A::_1
    }
}
#[doc = "Field `ANSA11` writer - A/D Conversion Channels Select n"]
pub type ANSA11_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA0_SPEC, ANSA11_A, O>;
impl<'a, const O: u8> ANSA11_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA11_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA11_A::_1)
    }
}
#[doc = "Field `ANSA12` reader - A/D Conversion Channels Select n"]
pub type ANSA12_R = crate::BitReader<ANSA12_A>;
#[doc = "A/D Conversion Channels Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA12_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSA12_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA12_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA12_A {
        match self.bits {
            false => ANSA12_A::_0,
            true => ANSA12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA12_A::_1
    }
}
#[doc = "Field `ANSA12` writer - A/D Conversion Channels Select n"]
pub type ANSA12_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA0_SPEC, ANSA12_A, O>;
impl<'a, const O: u8> ANSA12_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA12_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA12_A::_1)
    }
}
#[doc = "Field `ANSA13` reader - A/D Conversion Channels Select n"]
pub type ANSA13_R = crate::BitReader<ANSA13_A>;
#[doc = "A/D Conversion Channels Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA13_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSA13_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA13_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA13_A {
        match self.bits {
            false => ANSA13_A::_0,
            true => ANSA13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA13_A::_1
    }
}
#[doc = "Field `ANSA13` writer - A/D Conversion Channels Select n"]
pub type ANSA13_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA0_SPEC, ANSA13_A, O>;
impl<'a, const O: u8> ANSA13_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA13_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA13_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansa0(&self) -> ANSA0_R {
        ANSA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansa1(&self) -> ANSA1_R {
        ANSA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansa2(&self) -> ANSA2_R {
        ANSA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansa3(&self) -> ANSA3_R {
        ANSA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansa4(&self) -> ANSA4_R {
        ANSA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 11 - A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansa11(&self) -> ANSA11_R {
        ANSA11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansa12(&self) -> ANSA12_R {
        ANSA12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansa13(&self) -> ANSA13_R {
        ANSA13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A/D Conversion Channels Select n"]
    #[inline(always)]
    #[must_use]
    pub fn ansa0(&mut self) -> ANSA0_W<0> {
        ANSA0_W::new(self)
    }
    #[doc = "Bit 1 - A/D Conversion Channels Select n"]
    #[inline(always)]
    #[must_use]
    pub fn ansa1(&mut self) -> ANSA1_W<1> {
        ANSA1_W::new(self)
    }
    #[doc = "Bit 2 - A/D Conversion Channels Select n"]
    #[inline(always)]
    #[must_use]
    pub fn ansa2(&mut self) -> ANSA2_W<2> {
        ANSA2_W::new(self)
    }
    #[doc = "Bit 3 - A/D Conversion Channels Select n"]
    #[inline(always)]
    #[must_use]
    pub fn ansa3(&mut self) -> ANSA3_W<3> {
        ANSA3_W::new(self)
    }
    #[doc = "Bit 4 - A/D Conversion Channels Select n"]
    #[inline(always)]
    #[must_use]
    pub fn ansa4(&mut self) -> ANSA4_W<4> {
        ANSA4_W::new(self)
    }
    #[doc = "Bit 11 - A/D Conversion Channels Select n"]
    #[inline(always)]
    #[must_use]
    pub fn ansa11(&mut self) -> ANSA11_W<11> {
        ANSA11_W::new(self)
    }
    #[doc = "Bit 12 - A/D Conversion Channels Select n"]
    #[inline(always)]
    #[must_use]
    pub fn ansa12(&mut self) -> ANSA12_W<12> {
        ANSA12_W::new(self)
    }
    #[doc = "Bit 13 - A/D Conversion Channels Select n"]
    #[inline(always)]
    #[must_use]
    pub fn ansa13(&mut self) -> ANSA13_W<13> {
        ANSA13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Channel Select Register A0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adansa0](index.html) module"]
pub struct ADANSA0_SPEC;
impl crate::RegisterSpec for ADANSA0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adansa0::R](R) reader structure"]
impl crate::Readable for ADANSA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adansa0::W](W) writer structure"]
impl crate::Writable for ADANSA0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADANSA0 to value 0"]
impl crate::Resettable for ADANSA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
