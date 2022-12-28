#[doc = "Register `AMP0MS` reader"]
pub struct R(crate::R<AMP0MS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMP0MS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMP0MS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMP0MS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMP0MS` writer"]
pub struct W(crate::W<AMP0MS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMP0MS_SPEC>;
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
impl From<crate::W<AMP0MS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMP0MS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AMPMS0` reader - AMP0- pin select"]
pub type AMPMS0_R = crate::BitReader<AMPMS0_A>;
#[doc = "AMP0- pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPMS0_A {
    #[doc = "0: AMP0- pin is not connected to the AMP0 minus input"]
    _0 = 0,
    #[doc = "1: AMP0- pin is connected to the AMP0 minus input"]
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
#[doc = "Field `AMPMS0` writer - AMP0- pin select"]
pub type AMPMS0_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMP0MS_SPEC, AMPMS0_A, O>;
impl<'a, const O: u8> AMPMS0_W<'a, O> {
    #[doc = "AMP0- pin is not connected to the AMP0 minus input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPMS0_A::_0)
    }
    #[doc = "AMP0- pin is connected to the AMP0 minus input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPMS0_A::_1)
    }
}
#[doc = "Field `AMPMS1` reader - AMP0+ pin select"]
pub type AMPMS1_R = crate::BitReader<AMPMS1_A>;
#[doc = "AMP0+ pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPMS1_A {
    #[doc = "0: AMP0+ pin is not connected to the AMP0 minus input"]
    _0 = 0,
    #[doc = "1: AMP0+ pin is connected to the AMP0 minus input"]
    _1 = 1,
}
impl From<AMPMS1_A> for bool {
    #[inline(always)]
    fn from(variant: AMPMS1_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPMS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPMS1_A {
        match self.bits {
            false => AMPMS1_A::_0,
            true => AMPMS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPMS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPMS1_A::_1
    }
}
#[doc = "Field `AMPMS1` writer - AMP0+ pin select"]
pub type AMPMS1_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMP0MS_SPEC, AMPMS1_A, O>;
impl<'a, const O: u8> AMPMS1_W<'a, O> {
    #[doc = "AMP0+ pin is not connected to the AMP0 minus input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPMS1_A::_0)
    }
    #[doc = "AMP0+ pin is connected to the AMP0 minus input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPMS1_A::_1)
    }
}
#[doc = "Field `AMPMS2` reader - AMP1- pin select"]
pub type AMPMS2_R = crate::BitReader<AMPMS2_A>;
#[doc = "AMP1- pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPMS2_A {
    #[doc = "0: AMP1- pin is not connected to the AMP0 minus input"]
    _0 = 0,
    #[doc = "1: AMP1- pin is connected to the AMP0 minus input"]
    _1 = 1,
}
impl From<AMPMS2_A> for bool {
    #[inline(always)]
    fn from(variant: AMPMS2_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPMS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPMS2_A {
        match self.bits {
            false => AMPMS2_A::_0,
            true => AMPMS2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPMS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPMS2_A::_1
    }
}
#[doc = "Field `AMPMS2` writer - AMP1- pin select"]
pub type AMPMS2_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMP0MS_SPEC, AMPMS2_A, O>;
impl<'a, const O: u8> AMPMS2_W<'a, O> {
    #[doc = "AMP1- pin is not connected to the AMP0 minus input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPMS2_A::_0)
    }
    #[doc = "AMP1- pin is connected to the AMP0 minus input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPMS2_A::_1)
    }
}
#[doc = "Field `AMPMS3` reader - AMP1+ pin select"]
pub type AMPMS3_R = crate::BitReader<AMPMS3_A>;
#[doc = "AMP1+ pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPMS3_A {
    #[doc = "0: AMP1+ pin is not connected to the AMP0 minus input"]
    _0 = 0,
    #[doc = "1: AMP1+ pin is connected to the AMP0 minus input"]
    _1 = 1,
}
impl From<AMPMS3_A> for bool {
    #[inline(always)]
    fn from(variant: AMPMS3_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPMS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPMS3_A {
        match self.bits {
            false => AMPMS3_A::_0,
            true => AMPMS3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPMS3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPMS3_A::_1
    }
}
#[doc = "Field `AMPMS3` writer - AMP1+ pin select"]
pub type AMPMS3_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMP0MS_SPEC, AMPMS3_A, O>;
impl<'a, const O: u8> AMPMS3_W<'a, O> {
    #[doc = "AMP1+ pin is not connected to the AMP0 minus input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPMS3_A::_0)
    }
    #[doc = "AMP1+ pin is connected to the AMP0 minus input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPMS3_A::_1)
    }
}
#[doc = "Field `AMPMS4` reader - AMP2- pin select"]
pub type AMPMS4_R = crate::BitReader<AMPMS4_A>;
#[doc = "AMP2- pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPMS4_A {
    #[doc = "0: AMP2- pin is not connected to the AMP0 minus input"]
    _0 = 0,
    #[doc = "1: AMP2- pin is connected to the AMP0 minus input"]
    _1 = 1,
}
impl From<AMPMS4_A> for bool {
    #[inline(always)]
    fn from(variant: AMPMS4_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPMS4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPMS4_A {
        match self.bits {
            false => AMPMS4_A::_0,
            true => AMPMS4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPMS4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPMS4_A::_1
    }
}
#[doc = "Field `AMPMS4` writer - AMP2- pin select"]
pub type AMPMS4_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMP0MS_SPEC, AMPMS4_A, O>;
impl<'a, const O: u8> AMPMS4_W<'a, O> {
    #[doc = "AMP2- pin is not connected to the AMP0 minus input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPMS4_A::_0)
    }
    #[doc = "AMP2- pin is connected to the AMP0 minus input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPMS4_A::_1)
    }
}
#[doc = "Field `AMPMS7` reader - OPAMP0 output select"]
pub type AMPMS7_R = crate::BitReader<AMPMS7_A>;
#[doc = "OPAMP0 output select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPMS7_A {
    #[doc = "0: OPAMP0 output is not connected to the AMP0 minus input"]
    _0 = 0,
    #[doc = "1: OPAMP0 output is connected to the AMP0 minus input"]
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
#[doc = "Field `AMPMS7` writer - OPAMP0 output select"]
pub type AMPMS7_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMP0MS_SPEC, AMPMS7_A, O>;
impl<'a, const O: u8> AMPMS7_W<'a, O> {
    #[doc = "OPAMP0 output is not connected to the AMP0 minus input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPMS7_A::_0)
    }
    #[doc = "OPAMP0 output is connected to the AMP0 minus input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPMS7_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AMP0- pin select"]
    #[inline(always)]
    pub fn ampms0(&self) -> AMPMS0_R {
        AMPMS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AMP0+ pin select"]
    #[inline(always)]
    pub fn ampms1(&self) -> AMPMS1_R {
        AMPMS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AMP1- pin select"]
    #[inline(always)]
    pub fn ampms2(&self) -> AMPMS2_R {
        AMPMS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AMP1+ pin select"]
    #[inline(always)]
    pub fn ampms3(&self) -> AMPMS3_R {
        AMPMS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AMP2- pin select"]
    #[inline(always)]
    pub fn ampms4(&self) -> AMPMS4_R {
        AMPMS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - OPAMP0 output select"]
    #[inline(always)]
    pub fn ampms7(&self) -> AMPMS7_R {
        AMPMS7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AMP0- pin select"]
    #[inline(always)]
    #[must_use]
    pub fn ampms0(&mut self) -> AMPMS0_W<0> {
        AMPMS0_W::new(self)
    }
    #[doc = "Bit 1 - AMP0+ pin select"]
    #[inline(always)]
    #[must_use]
    pub fn ampms1(&mut self) -> AMPMS1_W<1> {
        AMPMS1_W::new(self)
    }
    #[doc = "Bit 2 - AMP1- pin select"]
    #[inline(always)]
    #[must_use]
    pub fn ampms2(&mut self) -> AMPMS2_W<2> {
        AMPMS2_W::new(self)
    }
    #[doc = "Bit 3 - AMP1+ pin select"]
    #[inline(always)]
    #[must_use]
    pub fn ampms3(&mut self) -> AMPMS3_W<3> {
        AMPMS3_W::new(self)
    }
    #[doc = "Bit 4 - AMP2- pin select"]
    #[inline(always)]
    #[must_use]
    pub fn ampms4(&mut self) -> AMPMS4_W<4> {
        AMPMS4_W::new(self)
    }
    #[doc = "Bit 7 - OPAMP0 output select"]
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
#[doc = "Operational Amplifier 0 Minus Input Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amp0ms](index.html) module"]
pub struct AMP0MS_SPEC;
impl crate::RegisterSpec for AMP0MS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [amp0ms::R](R) reader structure"]
impl crate::Readable for AMP0MS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [amp0ms::W](W) writer structure"]
impl crate::Writable for AMP0MS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMP0MS to value 0"]
impl crate::Resettable for AMP0MS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
