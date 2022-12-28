#[doc = "Register `AMP0OS` reader"]
pub struct R(crate::R<AMP0OS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMP0OS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMP0OS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMP0OS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMP0OS` writer"]
pub struct W(crate::W<AMP0OS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMP0OS_SPEC>;
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
impl From<crate::W<AMP0OS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMP0OS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AMPOS0` reader - AMP1- pin select"]
pub type AMPOS0_R = crate::BitReader<AMPOS0_A>;
#[doc = "AMP1- pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPOS0_A {
    #[doc = "0: AMP1- pin is not connected to the OPAMP0 output"]
    _0 = 0,
    #[doc = "1: AMP1- pin is connected to the OPAMP0 output"]
    _1 = 1,
}
impl From<AMPOS0_A> for bool {
    #[inline(always)]
    fn from(variant: AMPOS0_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPOS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPOS0_A {
        match self.bits {
            false => AMPOS0_A::_0,
            true => AMPOS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPOS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPOS0_A::_1
    }
}
#[doc = "Field `AMPOS0` writer - AMP1- pin select"]
pub type AMPOS0_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMP0OS_SPEC, AMPOS0_A, O>;
impl<'a, const O: u8> AMPOS0_W<'a, O> {
    #[doc = "AMP1- pin is not connected to the OPAMP0 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPOS0_A::_0)
    }
    #[doc = "AMP1- pin is connected to the OPAMP0 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPOS0_A::_1)
    }
}
#[doc = "Field `AMPOS1` reader - AMP1+ pin select"]
pub type AMPOS1_R = crate::BitReader<AMPOS1_A>;
#[doc = "AMP1+ pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPOS1_A {
    #[doc = "0: AMP1+ pin is not connected to the OPAMP0 output"]
    _0 = 0,
    #[doc = "1: AMP1+ pin is connected to the OPAMP0 output"]
    _1 = 1,
}
impl From<AMPOS1_A> for bool {
    #[inline(always)]
    fn from(variant: AMPOS1_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPOS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPOS1_A {
        match self.bits {
            false => AMPOS1_A::_0,
            true => AMPOS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPOS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPOS1_A::_1
    }
}
#[doc = "Field `AMPOS1` writer - AMP1+ pin select"]
pub type AMPOS1_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMP0OS_SPEC, AMPOS1_A, O>;
impl<'a, const O: u8> AMPOS1_W<'a, O> {
    #[doc = "AMP1+ pin is not connected to the OPAMP0 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPOS1_A::_0)
    }
    #[doc = "AMP1+ pin is connected to the OPAMP0 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPOS1_A::_1)
    }
}
#[doc = "Field `AMPOS2` reader - AMP2- pin select"]
pub type AMPOS2_R = crate::BitReader<AMPOS2_A>;
#[doc = "AMP2- pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPOS2_A {
    #[doc = "0: AMP2- pin is not connected to the OPAMP0 output"]
    _0 = 0,
    #[doc = "1: AMP2- pin is connected to the OPAMP0 output"]
    _1 = 1,
}
impl From<AMPOS2_A> for bool {
    #[inline(always)]
    fn from(variant: AMPOS2_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPOS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPOS2_A {
        match self.bits {
            false => AMPOS2_A::_0,
            true => AMPOS2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPOS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPOS2_A::_1
    }
}
#[doc = "Field `AMPOS2` writer - AMP2- pin select"]
pub type AMPOS2_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMP0OS_SPEC, AMPOS2_A, O>;
impl<'a, const O: u8> AMPOS2_W<'a, O> {
    #[doc = "AMP2- pin is not connected to the OPAMP0 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPOS2_A::_0)
    }
    #[doc = "AMP2- pin is connected to the OPAMP0 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPOS2_A::_1)
    }
}
#[doc = "Field `AMPOS3` reader - AMP2+ pin select"]
pub type AMPOS3_R = crate::BitReader<AMPOS3_A>;
#[doc = "AMP2+ pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPOS3_A {
    #[doc = "0: AMP2+ pin is not connected to the OPAMP0 output"]
    _0 = 0,
    #[doc = "1: AMP2+ pin is connected to the OPAMP0 output"]
    _1 = 1,
}
impl From<AMPOS3_A> for bool {
    #[inline(always)]
    fn from(variant: AMPOS3_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPOS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPOS3_A {
        match self.bits {
            false => AMPOS3_A::_0,
            true => AMPOS3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPOS3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPOS3_A::_1
    }
}
#[doc = "Field `AMPOS3` writer - AMP2+ pin select"]
pub type AMPOS3_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMP0OS_SPEC, AMPOS3_A, O>;
impl<'a, const O: u8> AMPOS3_W<'a, O> {
    #[doc = "AMP2+ pin is not connected to the OPAMP0 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPOS3_A::_0)
    }
    #[doc = "AMP2+ pin is connected to the OPAMP0 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPOS3_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AMP1- pin select"]
    #[inline(always)]
    pub fn ampos0(&self) -> AMPOS0_R {
        AMPOS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AMP1+ pin select"]
    #[inline(always)]
    pub fn ampos1(&self) -> AMPOS1_R {
        AMPOS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AMP2- pin select"]
    #[inline(always)]
    pub fn ampos2(&self) -> AMPOS2_R {
        AMPOS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AMP2+ pin select"]
    #[inline(always)]
    pub fn ampos3(&self) -> AMPOS3_R {
        AMPOS3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AMP1- pin select"]
    #[inline(always)]
    #[must_use]
    pub fn ampos0(&mut self) -> AMPOS0_W<0> {
        AMPOS0_W::new(self)
    }
    #[doc = "Bit 1 - AMP1+ pin select"]
    #[inline(always)]
    #[must_use]
    pub fn ampos1(&mut self) -> AMPOS1_W<1> {
        AMPOS1_W::new(self)
    }
    #[doc = "Bit 2 - AMP2- pin select"]
    #[inline(always)]
    #[must_use]
    pub fn ampos2(&mut self) -> AMPOS2_W<2> {
        AMPOS2_W::new(self)
    }
    #[doc = "Bit 3 - AMP2+ pin select"]
    #[inline(always)]
    #[must_use]
    pub fn ampos3(&mut self) -> AMPOS3_W<3> {
        AMPOS3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operational Amplifier 0 Output Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amp0os](index.html) module"]
pub struct AMP0OS_SPEC;
impl crate::RegisterSpec for AMP0OS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [amp0os::R](R) reader structure"]
impl crate::Readable for AMP0OS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [amp0os::W](W) writer structure"]
impl crate::Writable for AMP0OS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMP0OS to value 0"]
impl crate::Resettable for AMP0OS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
