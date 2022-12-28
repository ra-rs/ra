#[doc = "Register `AMP1PS` reader"]
pub struct R(crate::R<AMP1PS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMP1PS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMP1PS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMP1PS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMP1PS` writer"]
pub struct W(crate::W<AMP1PS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMP1PS_SPEC>;
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
impl From<crate::W<AMP1PS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMP1PS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AMPPS0` reader - AMP1- pin select"]
pub type AMPPS0_R = crate::BitReader<AMPPS0_A>;
#[doc = "AMP1- pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPPS0_A {
    #[doc = "0: AMP1- pin is not connected to the AMP1 plus input"]
    _0 = 0,
    #[doc = "1: AMP1- pin is connected to the AMP1 plus input"]
    _1 = 1,
}
impl From<AMPPS0_A> for bool {
    #[inline(always)]
    fn from(variant: AMPPS0_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPPS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPPS0_A {
        match self.bits {
            false => AMPPS0_A::_0,
            true => AMPPS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPPS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPPS0_A::_1
    }
}
#[doc = "Field `AMPPS0` writer - AMP1- pin select"]
pub type AMPPS0_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMP1PS_SPEC, AMPPS0_A, O>;
impl<'a, const O: u8> AMPPS0_W<'a, O> {
    #[doc = "AMP1- pin is not connected to the AMP1 plus input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPPS0_A::_0)
    }
    #[doc = "AMP1- pin is connected to the AMP1 plus input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPPS0_A::_1)
    }
}
#[doc = "Field `AMPPS1` reader - AMP1+ pin select"]
pub type AMPPS1_R = crate::BitReader<AMPPS1_A>;
#[doc = "AMP1+ pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPPS1_A {
    #[doc = "0: AMP1+ pin is not connected to the AMP1 plus input"]
    _0 = 0,
    #[doc = "1: AMP1+ pin is connected to the AMP1 plus input"]
    _1 = 1,
}
impl From<AMPPS1_A> for bool {
    #[inline(always)]
    fn from(variant: AMPPS1_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPPS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPPS1_A {
        match self.bits {
            false => AMPPS1_A::_0,
            true => AMPPS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPPS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPPS1_A::_1
    }
}
#[doc = "Field `AMPPS1` writer - AMP1+ pin select"]
pub type AMPPS1_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMP1PS_SPEC, AMPPS1_A, O>;
impl<'a, const O: u8> AMPPS1_W<'a, O> {
    #[doc = "AMP1+ pin is not connected to the AMP1 plus input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPPS1_A::_0)
    }
    #[doc = "AMP1+ pin is connected to the AMP1 plus input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPPS1_A::_1)
    }
}
#[doc = "Field `AMPPS2` reader - AMP2- pin select"]
pub type AMPPS2_R = crate::BitReader<AMPPS2_A>;
#[doc = "AMP2- pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPPS2_A {
    #[doc = "0: AMP2- pin is not connected to the AMP1 plus input"]
    _0 = 0,
    #[doc = "1: AMP2- pin is connected to the AMP1 plus input"]
    _1 = 1,
}
impl From<AMPPS2_A> for bool {
    #[inline(always)]
    fn from(variant: AMPPS2_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPPS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPPS2_A {
        match self.bits {
            false => AMPPS2_A::_0,
            true => AMPPS2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPPS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPPS2_A::_1
    }
}
#[doc = "Field `AMPPS2` writer - AMP2- pin select"]
pub type AMPPS2_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMP1PS_SPEC, AMPPS2_A, O>;
impl<'a, const O: u8> AMPPS2_W<'a, O> {
    #[doc = "AMP2- pin is not connected to the AMP1 plus input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPPS2_A::_0)
    }
    #[doc = "AMP2- pin is connected to the AMP1 plus input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPPS2_A::_1)
    }
}
#[doc = "Field `AMPPS3` reader - AMP2+ pin select"]
pub type AMPPS3_R = crate::BitReader<AMPPS3_A>;
#[doc = "AMP2+ pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPPS3_A {
    #[doc = "0: AMP2+ pin is not connected to the AMP1 plus input"]
    _0 = 0,
    #[doc = "1: AMP2+ pin is connected to the AMP1"]
    _1 = 1,
}
impl From<AMPPS3_A> for bool {
    #[inline(always)]
    fn from(variant: AMPPS3_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPPS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPPS3_A {
        match self.bits {
            false => AMPPS3_A::_0,
            true => AMPPS3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPPS3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPPS3_A::_1
    }
}
#[doc = "Field `AMPPS3` writer - AMP2+ pin select"]
pub type AMPPS3_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMP1PS_SPEC, AMPPS3_A, O>;
impl<'a, const O: u8> AMPPS3_W<'a, O> {
    #[doc = "AMP2+ pin is not connected to the AMP1 plus input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPPS3_A::_0)
    }
    #[doc = "AMP2+ pin is connected to the AMP1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPPS3_A::_1)
    }
}
#[doc = "Field `AMPMS7` reader - OPAMP2 output select"]
pub type AMPMS7_R = crate::BitReader<AMPMS7_A>;
#[doc = "OPAMP2 output select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPMS7_A {
    #[doc = "0: OPAMP2 output is not connected to the AMP2 minus input"]
    _0 = 0,
    #[doc = "1: OPAMP2 output is connected to the AMP2 minus input"]
    _1 = 1,
}
impl From<AMPMS7_A> for bool {
    #[inline(always)]
    fn from(variant: AMPMS7_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPMS7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPMS7_A {
        match self.bits {
            false => AMPMS7_A::_0,
            true => AMPMS7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPMS7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPMS7_A::_1
    }
}
#[doc = "Field `AMPMS7` writer - OPAMP2 output select"]
pub type AMPMS7_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMP1PS_SPEC, AMPMS7_A, O>;
impl<'a, const O: u8> AMPMS7_W<'a, O> {
    #[doc = "OPAMP2 output is not connected to the AMP2 minus input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPMS7_A::_0)
    }
    #[doc = "OPAMP2 output is connected to the AMP2 minus input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPMS7_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AMP1- pin select"]
    #[inline(always)]
    pub fn ampps0(&self) -> AMPPS0_R {
        AMPPS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AMP1+ pin select"]
    #[inline(always)]
    pub fn ampps1(&self) -> AMPPS1_R {
        AMPPS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AMP2- pin select"]
    #[inline(always)]
    pub fn ampps2(&self) -> AMPPS2_R {
        AMPPS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AMP2+ pin select"]
    #[inline(always)]
    pub fn ampps3(&self) -> AMPPS3_R {
        AMPPS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - OPAMP2 output select"]
    #[inline(always)]
    pub fn ampms7(&self) -> AMPMS7_R {
        AMPMS7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AMP1- pin select"]
    #[inline(always)]
    #[must_use]
    pub fn ampps0(&mut self) -> AMPPS0_W<0> {
        AMPPS0_W::new(self)
    }
    #[doc = "Bit 1 - AMP1+ pin select"]
    #[inline(always)]
    #[must_use]
    pub fn ampps1(&mut self) -> AMPPS1_W<1> {
        AMPPS1_W::new(self)
    }
    #[doc = "Bit 2 - AMP2- pin select"]
    #[inline(always)]
    #[must_use]
    pub fn ampps2(&mut self) -> AMPPS2_W<2> {
        AMPPS2_W::new(self)
    }
    #[doc = "Bit 3 - AMP2+ pin select"]
    #[inline(always)]
    #[must_use]
    pub fn ampps3(&mut self) -> AMPPS3_W<3> {
        AMPPS3_W::new(self)
    }
    #[doc = "Bit 7 - OPAMP2 output select"]
    #[inline(always)]
    #[must_use]
    pub fn ampms7(&mut self) -> AMPMS7_W<7> {
        AMPMS7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operational Amplifier 1 Plus Input Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amp1ps](index.html) module"]
pub struct AMP1PS_SPEC;
impl crate::RegisterSpec for AMP1PS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [amp1ps::R](R) reader structure"]
impl crate::Readable for AMP1PS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [amp1ps::W](W) writer structure"]
impl crate::Writable for AMP1PS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMP1PS to value 0"]
impl crate::Resettable for AMP1PS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
