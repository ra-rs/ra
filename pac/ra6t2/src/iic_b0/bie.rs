#[doc = "Register `BIE` reader"]
pub struct R(crate::R<BIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BIE` writer"]
pub struct W(crate::W<BIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIE_SPEC>;
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
impl From<crate::W<BIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STCNDDIE` reader - START Condition Detection Interrupt Enable"]
pub type STCNDDIE_R = crate::BitReader<STCNDDIE_A>;
#[doc = "START Condition Detection Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STCNDDIE_A {
    #[doc = "0: Disables START condition Detection Interrupt Signal."]
    _0 = 0,
    #[doc = "1: Enables START condition Detection Interrupt Signal."]
    _1 = 1,
}
impl From<STCNDDIE_A> for bool {
    #[inline(always)]
    fn from(variant: STCNDDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl STCNDDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STCNDDIE_A {
        match self.bits {
            false => STCNDDIE_A::_0,
            true => STCNDDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STCNDDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STCNDDIE_A::_1
    }
}
#[doc = "Field `STCNDDIE` writer - START Condition Detection Interrupt Enable"]
pub type STCNDDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BIE_SPEC, STCNDDIE_A, O>;
impl<'a, const O: u8> STCNDDIE_W<'a, O> {
    #[doc = "Disables START condition Detection Interrupt Signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STCNDDIE_A::_0)
    }
    #[doc = "Enables START condition Detection Interrupt Signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STCNDDIE_A::_1)
    }
}
#[doc = "Field `SPCNDDIE` reader - STOP Condition Detection Interrupt Enable"]
pub type SPCNDDIE_R = crate::BitReader<SPCNDDIE_A>;
#[doc = "STOP Condition Detection Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPCNDDIE_A {
    #[doc = "0: Disables STOP condition Detection Interrupt Signal."]
    _0 = 0,
    #[doc = "1: Enables STOP condition Detection Interrupt Signal."]
    _1 = 1,
}
impl From<SPCNDDIE_A> for bool {
    #[inline(always)]
    fn from(variant: SPCNDDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPCNDDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPCNDDIE_A {
        match self.bits {
            false => SPCNDDIE_A::_0,
            true => SPCNDDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPCNDDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPCNDDIE_A::_1
    }
}
#[doc = "Field `SPCNDDIE` writer - STOP Condition Detection Interrupt Enable"]
pub type SPCNDDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BIE_SPEC, SPCNDDIE_A, O>;
impl<'a, const O: u8> SPCNDDIE_W<'a, O> {
    #[doc = "Disables STOP condition Detection Interrupt Signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPCNDDIE_A::_0)
    }
    #[doc = "Enables STOP condition Detection Interrupt Signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPCNDDIE_A::_1)
    }
}
#[doc = "Field `NACKDIE` reader - NACK Detection Interrupt Enable"]
pub type NACKDIE_R = crate::BitReader<NACKDIE_A>;
#[doc = "NACK Detection Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKDIE_A {
    #[doc = "0: Disables NACK Detection Interrupt Signal."]
    _0 = 0,
    #[doc = "1: Enables NACK Detection Interrupt Signal."]
    _1 = 1,
}
impl From<NACKDIE_A> for bool {
    #[inline(always)]
    fn from(variant: NACKDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl NACKDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NACKDIE_A {
        match self.bits {
            false => NACKDIE_A::_0,
            true => NACKDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NACKDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NACKDIE_A::_1
    }
}
#[doc = "Field `NACKDIE` writer - NACK Detection Interrupt Enable"]
pub type NACKDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BIE_SPEC, NACKDIE_A, O>;
impl<'a, const O: u8> NACKDIE_W<'a, O> {
    #[doc = "Disables NACK Detection Interrupt Signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NACKDIE_A::_0)
    }
    #[doc = "Enables NACK Detection Interrupt Signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NACKDIE_A::_1)
    }
}
#[doc = "Field `TENDIE` reader - Transmit End Interrupt Enable"]
pub type TENDIE_R = crate::BitReader<TENDIE_A>;
#[doc = "Transmit End Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TENDIE_A {
    #[doc = "0: Disables Transmit End Interrupt Signal."]
    _0 = 0,
    #[doc = "1: Enables Transmit End Interrupt Signal."]
    _1 = 1,
}
impl From<TENDIE_A> for bool {
    #[inline(always)]
    fn from(variant: TENDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TENDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TENDIE_A {
        match self.bits {
            false => TENDIE_A::_0,
            true => TENDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TENDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TENDIE_A::_1
    }
}
#[doc = "Field `TENDIE` writer - Transmit End Interrupt Enable"]
pub type TENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BIE_SPEC, TENDIE_A, O>;
impl<'a, const O: u8> TENDIE_W<'a, O> {
    #[doc = "Disables Transmit End Interrupt Signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TENDIE_A::_0)
    }
    #[doc = "Enables Transmit End Interrupt Signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TENDIE_A::_1)
    }
}
#[doc = "Field `ALIE` reader - Arbitration Lost Interrupt Enable"]
pub type ALIE_R = crate::BitReader<ALIE_A>;
#[doc = "Arbitration Lost Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALIE_A {
    #[doc = "0: Disables Arbitration Lost Interrupt Signal."]
    _0 = 0,
    #[doc = "1: Enables Arbitration Lost Interrupt Signal."]
    _1 = 1,
}
impl From<ALIE_A> for bool {
    #[inline(always)]
    fn from(variant: ALIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ALIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIE_A {
        match self.bits {
            false => ALIE_A::_0,
            true => ALIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALIE_A::_1
    }
}
#[doc = "Field `ALIE` writer - Arbitration Lost Interrupt Enable"]
pub type ALIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BIE_SPEC, ALIE_A, O>;
impl<'a, const O: u8> ALIE_W<'a, O> {
    #[doc = "Disables Arbitration Lost Interrupt Signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALIE_A::_0)
    }
    #[doc = "Enables Arbitration Lost Interrupt Signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALIE_A::_1)
    }
}
#[doc = "Field `TODIE` reader - Timeout Detection Interrupt Enable"]
pub type TODIE_R = crate::BitReader<TODIE_A>;
#[doc = "Timeout Detection Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TODIE_A {
    #[doc = "0: Disables Timeout Detection Interrupt Signal."]
    _0 = 0,
    #[doc = "1: Enables Timeout Detection Interrupt Signal."]
    _1 = 1,
}
impl From<TODIE_A> for bool {
    #[inline(always)]
    fn from(variant: TODIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TODIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TODIE_A {
        match self.bits {
            false => TODIE_A::_0,
            true => TODIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TODIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TODIE_A::_1
    }
}
#[doc = "Field `TODIE` writer - Timeout Detection Interrupt Enable"]
pub type TODIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BIE_SPEC, TODIE_A, O>;
impl<'a, const O: u8> TODIE_W<'a, O> {
    #[doc = "Disables Timeout Detection Interrupt Signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TODIE_A::_0)
    }
    #[doc = "Enables Timeout Detection Interrupt Signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TODIE_A::_1)
    }
}
#[doc = "Field `WUCNDDIE` reader - Wake-Up Condition Detection Interrupt Enable"]
pub type WUCNDDIE_R = crate::BitReader<WUCNDDIE_A>;
#[doc = "Wake-Up Condition Detection Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUCNDDIE_A {
    #[doc = "0: Disables Wake-Up Condition Detection Interrupt Signal."]
    _0 = 0,
    #[doc = "1: Enables Wake-Up Condition Detection Interrupt Signal."]
    _1 = 1,
}
impl From<WUCNDDIE_A> for bool {
    #[inline(always)]
    fn from(variant: WUCNDDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl WUCNDDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUCNDDIE_A {
        match self.bits {
            false => WUCNDDIE_A::_0,
            true => WUCNDDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUCNDDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUCNDDIE_A::_1
    }
}
#[doc = "Field `WUCNDDIE` writer - Wake-Up Condition Detection Interrupt Enable"]
pub type WUCNDDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BIE_SPEC, WUCNDDIE_A, O>;
impl<'a, const O: u8> WUCNDDIE_W<'a, O> {
    #[doc = "Disables Wake-Up Condition Detection Interrupt Signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUCNDDIE_A::_0)
    }
    #[doc = "Enables Wake-Up Condition Detection Interrupt Signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUCNDDIE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - START Condition Detection Interrupt Enable"]
    #[inline(always)]
    pub fn stcnddie(&self) -> STCNDDIE_R {
        STCNDDIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - STOP Condition Detection Interrupt Enable"]
    #[inline(always)]
    pub fn spcnddie(&self) -> SPCNDDIE_R {
        SPCNDDIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - NACK Detection Interrupt Enable"]
    #[inline(always)]
    pub fn nackdie(&self) -> NACKDIE_R {
        NACKDIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit End Interrupt Enable"]
    #[inline(always)]
    pub fn tendie(&self) -> TENDIE_R {
        TENDIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub fn alie(&self) -> ALIE_R {
        ALIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Timeout Detection Interrupt Enable"]
    #[inline(always)]
    pub fn todie(&self) -> TODIE_R {
        TODIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Wake-Up Condition Detection Interrupt Enable"]
    #[inline(always)]
    pub fn wucnddie(&self) -> WUCNDDIE_R {
        WUCNDDIE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - START Condition Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stcnddie(&mut self) -> STCNDDIE_W<0> {
        STCNDDIE_W::new(self)
    }
    #[doc = "Bit 1 - STOP Condition Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spcnddie(&mut self) -> SPCNDDIE_W<1> {
        SPCNDDIE_W::new(self)
    }
    #[doc = "Bit 4 - NACK Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nackdie(&mut self) -> NACKDIE_W<4> {
        NACKDIE_W::new(self)
    }
    #[doc = "Bit 8 - Transmit End Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tendie(&mut self) -> TENDIE_W<8> {
        TENDIE_W::new(self)
    }
    #[doc = "Bit 16 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn alie(&mut self) -> ALIE_W<16> {
        ALIE_W::new(self)
    }
    #[doc = "Bit 20 - Timeout Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn todie(&mut self) -> TODIE_W<20> {
        TODIE_W::new(self)
    }
    #[doc = "Bit 24 - Wake-Up Condition Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wucnddie(&mut self) -> WUCNDDIE_W<24> {
        WUCNDDIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bie](index.html) module"]
pub struct BIE_SPEC;
impl crate::RegisterSpec for BIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bie::R](R) reader structure"]
impl crate::Readable for BIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bie::W](W) writer structure"]
impl crate::Writable for BIE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BIE to value 0"]
impl crate::Resettable for BIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
