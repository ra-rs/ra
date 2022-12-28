#[doc = "Register `AMP1MS` reader"]
pub struct R(crate::R<AMP1MS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMP1MS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMP1MS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMP1MS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMP1MS` writer"]
pub struct W(crate::W<AMP1MS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMP1MS_SPEC>;
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
impl From<crate::W<AMP1MS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMP1MS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AMPMS0` reader - AMP1- pin select"]
pub type AMPMS0_R = crate::BitReader<AMPMS0_A>;
#[doc = "AMP1- pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPMS0_A {
    #[doc = "0: AMP1- pin is not connected to the AMP1 minus input"]
    _0 = 0,
    #[doc = "1: AMP1- pin is connected to the AMP1 minus input"]
    _1 = 1,
}
impl From<AMPMS0_A> for bool {
    #[inline(always)]
    fn from(variant: AMPMS0_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPMS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPMS0_A {
        match self.bits {
            false => AMPMS0_A::_0,
            true => AMPMS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPMS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPMS0_A::_1
    }
}
#[doc = "Field `AMPMS0` writer - AMP1- pin select"]
pub type AMPMS0_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMP1MS_SPEC, AMPMS0_A, O>;
impl<'a, const O: u8> AMPMS0_W<'a, O> {
    #[doc = "AMP1- pin is not connected to the AMP1 minus input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPMS0_A::_0)
    }
    #[doc = "AMP1- pin is connected to the AMP1 minus input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPMS0_A::_1)
    }
}
#[doc = "Field `AMPMS7` reader - OPAMP1 output select"]
pub type AMPMS7_R = crate::BitReader<AMPMS7_A>;
#[doc = "OPAMP1 output select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPMS7_A {
    #[doc = "0: OPAMP1 output is not connected to the AMP1 minus input"]
    _0 = 0,
    #[doc = "1: OPAMP1 output is connected to the AMP1 minus input"]
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
#[doc = "Field `AMPMS7` writer - OPAMP1 output select"]
pub type AMPMS7_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMP1MS_SPEC, AMPMS7_A, O>;
impl<'a, const O: u8> AMPMS7_W<'a, O> {
    #[doc = "OPAMP1 output is not connected to the AMP1 minus input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPMS7_A::_0)
    }
    #[doc = "OPAMP1 output is connected to the AMP1 minus input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPMS7_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AMP1- pin select"]
    #[inline(always)]
    pub fn ampms0(&self) -> AMPMS0_R {
        AMPMS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - OPAMP1 output select"]
    #[inline(always)]
    pub fn ampms7(&self) -> AMPMS7_R {
        AMPMS7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AMP1- pin select"]
    #[inline(always)]
    #[must_use]
    pub fn ampms0(&mut self) -> AMPMS0_W<0> {
        AMPMS0_W::new(self)
    }
    #[doc = "Bit 7 - OPAMP1 output select"]
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
#[doc = "Operational Amplifier 1 Minus Input Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amp1ms](index.html) module"]
pub struct AMP1MS_SPEC;
impl crate::RegisterSpec for AMP1MS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [amp1ms::R](R) reader structure"]
impl crate::Readable for AMP1MS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [amp1ms::W](W) writer structure"]
impl crate::Writable for AMP1MS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMP1MS to value 0"]
impl crate::Resettable for AMP1MS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
