#[doc = "Register `AMP2PS` reader"]
pub struct R(crate::R<AMP2PS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMP2PS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMP2PS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMP2PS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMP2PS` writer"]
pub struct W(crate::W<AMP2PS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMP2PS_SPEC>;
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
impl From<crate::W<AMP2PS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMP2PS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AMPPS0` reader - AMP2- pin select"]
pub type AMPPS0_R = crate::BitReader<AMPPS0_A>;
#[doc = "AMP2- pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPPS0_A {
    #[doc = "0: AMP2- pin is not connected to the AMP2 plus input"]
    _0 = 0,
    #[doc = "1: AMP2- pin is connected to the AMP2 plus input"]
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
#[doc = "Field `AMPPS0` writer - AMP2- pin select"]
pub type AMPPS0_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMP2PS_SPEC, AMPPS0_A, O>;
impl<'a, const O: u8> AMPPS0_W<'a, O> {
    #[doc = "AMP2- pin is not connected to the AMP2 plus input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPPS0_A::_0)
    }
    #[doc = "AMP2- pin is connected to the AMP2 plus input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPPS0_A::_1)
    }
}
#[doc = "Field `AMPPS1` reader - AMP2+ pin select"]
pub type AMPPS1_R = crate::BitReader<AMPPS1_A>;
#[doc = "AMP2+ pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPPS1_A {
    #[doc = "0: AMP2+ pin is not connected to the AMP2 plus input"]
    _0 = 0,
    #[doc = "1: AMP2+ pin is connected to the AMP2 plus input"]
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
#[doc = "Field `AMPPS1` writer - AMP2+ pin select"]
pub type AMPPS1_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMP2PS_SPEC, AMPPS1_A, O>;
impl<'a, const O: u8> AMPPS1_W<'a, O> {
    #[doc = "AMP2+ pin is not connected to the AMP2 plus input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPPS1_A::_0)
    }
    #[doc = "AMP2+ pin is connected to the AMP2 plus input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPPS1_A::_1)
    }
}
#[doc = "Field `AMPPS7` reader - DAC8 channel 1output select"]
pub type AMPPS7_R = crate::BitReader<AMPPS7_A>;
#[doc = "DAC8 channel 1output select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPPS7_A {
    #[doc = "0: DAC8 channel 1 output is not connected to the AMP2 plus input"]
    _0 = 0,
    #[doc = "1: DAC8 channel 1 output is connected to the AMP2 plus input"]
    _1 = 1,
}
impl From<AMPPS7_A> for bool {
    #[inline(always)]
    fn from(variant: AMPPS7_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPPS7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPPS7_A {
        match self.bits {
            false => AMPPS7_A::_0,
            true => AMPPS7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPPS7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPPS7_A::_1
    }
}
#[doc = "Field `AMPPS7` writer - DAC8 channel 1output select"]
pub type AMPPS7_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMP2PS_SPEC, AMPPS7_A, O>;
impl<'a, const O: u8> AMPPS7_W<'a, O> {
    #[doc = "DAC8 channel 1 output is not connected to the AMP2 plus input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPPS7_A::_0)
    }
    #[doc = "DAC8 channel 1 output is connected to the AMP2 plus input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPPS7_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AMP2- pin select"]
    #[inline(always)]
    pub fn ampps0(&self) -> AMPPS0_R {
        AMPPS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AMP2+ pin select"]
    #[inline(always)]
    pub fn ampps1(&self) -> AMPPS1_R {
        AMPPS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - DAC8 channel 1output select"]
    #[inline(always)]
    pub fn ampps7(&self) -> AMPPS7_R {
        AMPPS7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AMP2- pin select"]
    #[inline(always)]
    #[must_use]
    pub fn ampps0(&mut self) -> AMPPS0_W<0> {
        AMPPS0_W::new(self)
    }
    #[doc = "Bit 1 - AMP2+ pin select"]
    #[inline(always)]
    #[must_use]
    pub fn ampps1(&mut self) -> AMPPS1_W<1> {
        AMPPS1_W::new(self)
    }
    #[doc = "Bit 7 - DAC8 channel 1output select"]
    #[inline(always)]
    #[must_use]
    pub fn ampps7(&mut self) -> AMPPS7_W<7> {
        AMPPS7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operational Amplifier 2 Plus Input Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amp2ps](index.html) module"]
pub struct AMP2PS_SPEC;
impl crate::RegisterSpec for AMP2PS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [amp2ps::R](R) reader structure"]
impl crate::Readable for AMP2PS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [amp2ps::W](W) writer structure"]
impl crate::Writable for AMP2PS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMP2PS to value 0"]
impl crate::Resettable for AMP2PS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
