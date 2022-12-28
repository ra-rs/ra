#[doc = "Register `AMPMC` reader"]
pub struct R(crate::R<AMPMC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMPMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMPMC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMPMC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMPMC` writer"]
pub struct W(crate::W<AMPMC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMPMC_SPEC>;
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
impl From<crate::W<AMPMC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMPMC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AMPPC0` reader - Operational amplifier precharge control status"]
pub type AMPPC0_R = crate::BitReader<AMPPC0_A>;
#[doc = "Operational amplifier precharge control status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPPC0_A {
    #[doc = "0: Precharging is stopped."]
    _0 = 0,
    #[doc = "1: Precharging is enabled."]
    _1 = 1,
}
impl From<AMPPC0_A> for bool {
    #[inline(always)]
    fn from(variant: AMPPC0_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPPC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPPC0_A {
        match self.bits {
            false => AMPPC0_A::_0,
            true => AMPPC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPPC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPPC0_A::_1
    }
}
#[doc = "Field `AMPPC0` writer - Operational amplifier precharge control status"]
pub type AMPPC0_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMPMC_SPEC, AMPPC0_A, O>;
impl<'a, const O: u8> AMPPC0_W<'a, O> {
    #[doc = "Precharging is stopped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPPC0_A::_0)
    }
    #[doc = "Precharging is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPPC0_A::_1)
    }
}
#[doc = "Field `AMPPC1` reader - Operational amplifier precharge control status"]
pub type AMPPC1_R = crate::BitReader<AMPPC1_A>;
#[doc = "Operational amplifier precharge control status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPPC1_A {
    #[doc = "0: Precharging is stopped."]
    _0 = 0,
    #[doc = "1: Precharging is enabled."]
    _1 = 1,
}
impl From<AMPPC1_A> for bool {
    #[inline(always)]
    fn from(variant: AMPPC1_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPPC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPPC1_A {
        match self.bits {
            false => AMPPC1_A::_0,
            true => AMPPC1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPPC1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPPC1_A::_1
    }
}
#[doc = "Field `AMPPC1` writer - Operational amplifier precharge control status"]
pub type AMPPC1_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMPMC_SPEC, AMPPC1_A, O>;
impl<'a, const O: u8> AMPPC1_W<'a, O> {
    #[doc = "Precharging is stopped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPPC1_A::_0)
    }
    #[doc = "Precharging is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPPC1_A::_1)
    }
}
#[doc = "Field `AMPPC2` reader - Operational amplifier precharge control status"]
pub type AMPPC2_R = crate::BitReader<AMPPC2_A>;
#[doc = "Operational amplifier precharge control status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPPC2_A {
    #[doc = "0: Precharging is stopped."]
    _0 = 0,
    #[doc = "1: Precharging is enabled."]
    _1 = 1,
}
impl From<AMPPC2_A> for bool {
    #[inline(always)]
    fn from(variant: AMPPC2_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPPC2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPPC2_A {
        match self.bits {
            false => AMPPC2_A::_0,
            true => AMPPC2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPPC2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPPC2_A::_1
    }
}
#[doc = "Field `AMPPC2` writer - Operational amplifier precharge control status"]
pub type AMPPC2_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMPMC_SPEC, AMPPC2_A, O>;
impl<'a, const O: u8> AMPPC2_W<'a, O> {
    #[doc = "Precharging is stopped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPPC2_A::_0)
    }
    #[doc = "Precharging is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPPC2_A::_1)
    }
}
#[doc = "Field `AMPPC3` reader - Operational amplifier precharge control status"]
pub type AMPPC3_R = crate::BitReader<AMPPC3_A>;
#[doc = "Operational amplifier precharge control status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPPC3_A {
    #[doc = "0: Precharging is stopped."]
    _0 = 0,
    #[doc = "1: Precharging is enabled."]
    _1 = 1,
}
impl From<AMPPC3_A> for bool {
    #[inline(always)]
    fn from(variant: AMPPC3_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPPC3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPPC3_A {
        match self.bits {
            false => AMPPC3_A::_0,
            true => AMPPC3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPPC3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPPC3_A::_1
    }
}
#[doc = "Field `AMPPC3` writer - Operational amplifier precharge control status"]
pub type AMPPC3_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMPMC_SPEC, AMPPC3_A, O>;
impl<'a, const O: u8> AMPPC3_W<'a, O> {
    #[doc = "Precharging is stopped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPPC3_A::_0)
    }
    #[doc = "Precharging is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPPC3_A::_1)
    }
}
#[doc = "Field `AMPSP` reader - Operation mode selection"]
pub type AMPSP_R = crate::BitReader<AMPSP_A>;
#[doc = "Operation mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPSP_A {
    #[doc = "0: Low-power mode (low-speed)."]
    _0 = 0,
    #[doc = "1: High-speed mode."]
    _1 = 1,
}
impl From<AMPSP_A> for bool {
    #[inline(always)]
    fn from(variant: AMPSP_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPSP_A {
        match self.bits {
            false => AMPSP_A::_0,
            true => AMPSP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPSP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPSP_A::_1
    }
}
#[doc = "Field `AMPSP` writer - Operation mode selection"]
pub type AMPSP_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMPMC_SPEC, AMPSP_A, O>;
impl<'a, const O: u8> AMPSP_W<'a, O> {
    #[doc = "Low-power mode (low-speed)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPSP_A::_0)
    }
    #[doc = "High-speed mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPSP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Operational amplifier precharge control status"]
    #[inline(always)]
    pub fn amppc0(&self) -> AMPPC0_R {
        AMPPC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Operational amplifier precharge control status"]
    #[inline(always)]
    pub fn amppc1(&self) -> AMPPC1_R {
        AMPPC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Operational amplifier precharge control status"]
    #[inline(always)]
    pub fn amppc2(&self) -> AMPPC2_R {
        AMPPC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Operational amplifier precharge control status"]
    #[inline(always)]
    pub fn amppc3(&self) -> AMPPC3_R {
        AMPPC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Operation mode selection"]
    #[inline(always)]
    pub fn ampsp(&self) -> AMPSP_R {
        AMPSP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operational amplifier precharge control status"]
    #[inline(always)]
    #[must_use]
    pub fn amppc0(&mut self) -> AMPPC0_W<0> {
        AMPPC0_W::new(self)
    }
    #[doc = "Bit 1 - Operational amplifier precharge control status"]
    #[inline(always)]
    #[must_use]
    pub fn amppc1(&mut self) -> AMPPC1_W<1> {
        AMPPC1_W::new(self)
    }
    #[doc = "Bit 2 - Operational amplifier precharge control status"]
    #[inline(always)]
    #[must_use]
    pub fn amppc2(&mut self) -> AMPPC2_W<2> {
        AMPPC2_W::new(self)
    }
    #[doc = "Bit 3 - Operational amplifier precharge control status"]
    #[inline(always)]
    #[must_use]
    pub fn amppc3(&mut self) -> AMPPC3_W<3> {
        AMPPC3_W::new(self)
    }
    #[doc = "Bit 7 - Operation mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn ampsp(&mut self) -> AMPSP_W<7> {
        AMPSP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operational amplifier mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ampmc](index.html) module"]
pub struct AMPMC_SPEC;
impl crate::RegisterSpec for AMPMC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ampmc::R](R) reader structure"]
impl crate::Readable for AMPMC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ampmc::W](W) writer structure"]
impl crate::Writable for AMPMC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMPMC to value 0"]
impl crate::Resettable for AMPMC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
