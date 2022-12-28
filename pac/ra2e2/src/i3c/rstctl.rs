#[doc = "Register `RSTCTL` reader"]
pub struct R(crate::R<RSTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTCTL` writer"]
pub struct W(crate::W<RSTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTCTL_SPEC>;
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
impl From<crate::W<RSTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RI3CRST` reader - I3C Software Reset"]
pub type RI3CRST_R = crate::BitReader<RI3CRST_A>;
#[doc = "I3C Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RI3CRST_A {
    #[doc = "0: Release I3C reset."]
    _0 = 0,
    #[doc = "1: Initiate I3C reset."]
    _1 = 1,
}
impl From<RI3CRST_A> for bool {
    #[inline(always)]
    fn from(variant: RI3CRST_A) -> Self {
        variant as u8 != 0
    }
}
impl RI3CRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RI3CRST_A {
        match self.bits {
            false => RI3CRST_A::_0,
            true => RI3CRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RI3CRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RI3CRST_A::_1
    }
}
#[doc = "Field `RI3CRST` writer - I3C Software Reset"]
pub type RI3CRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_SPEC, RI3CRST_A, O>;
impl<'a, const O: u8> RI3CRST_W<'a, O> {
    #[doc = "Release I3C reset."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RI3CRST_A::_0)
    }
    #[doc = "Initiate I3C reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RI3CRST_A::_1)
    }
}
#[doc = "Field `CMDQRST` reader - Command Queue Software Reset"]
pub type CMDQRST_R = crate::BitReader<CMDQRST_A>;
#[doc = "Command Queue Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDQRST_A {
    #[doc = "0: The Command Queues in I3C is not flushed."]
    _0 = 0,
    #[doc = "1: The Command Queues in I3C is flushed."]
    _1 = 1,
}
impl From<CMDQRST_A> for bool {
    #[inline(always)]
    fn from(variant: CMDQRST_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDQRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDQRST_A {
        match self.bits {
            false => CMDQRST_A::_0,
            true => CMDQRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMDQRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMDQRST_A::_1
    }
}
#[doc = "Field `CMDQRST` writer - Command Queue Software Reset"]
pub type CMDQRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_SPEC, CMDQRST_A, O>;
impl<'a, const O: u8> CMDQRST_W<'a, O> {
    #[doc = "The Command Queues in I3C is not flushed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMDQRST_A::_0)
    }
    #[doc = "The Command Queues in I3C is flushed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMDQRST_A::_1)
    }
}
#[doc = "Field `RSPQRST` reader - Response Queue Software Reset"]
pub type RSPQRST_R = crate::BitReader<RSPQRST_A>;
#[doc = "Response Queue Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPQRST_A {
    #[doc = "0: The Response Queues in I3C is not flushed."]
    _0 = 0,
    #[doc = "1: The Response Queues in I3C is flushed."]
    _1 = 1,
}
impl From<RSPQRST_A> for bool {
    #[inline(always)]
    fn from(variant: RSPQRST_A) -> Self {
        variant as u8 != 0
    }
}
impl RSPQRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSPQRST_A {
        match self.bits {
            false => RSPQRST_A::_0,
            true => RSPQRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSPQRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSPQRST_A::_1
    }
}
#[doc = "Field `RSPQRST` writer - Response Queue Software Reset"]
pub type RSPQRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_SPEC, RSPQRST_A, O>;
impl<'a, const O: u8> RSPQRST_W<'a, O> {
    #[doc = "The Response Queues in I3C is not flushed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSPQRST_A::_0)
    }
    #[doc = "The Response Queues in I3C is flushed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSPQRST_A::_1)
    }
}
#[doc = "Field `TDBRST` reader - Transmit Data Buffer Software Reset"]
pub type TDBRST_R = crate::BitReader<TDBRST_A>;
#[doc = "Transmit Data Buffer Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDBRST_A {
    #[doc = "0: The Transmit Queues in I3C is not flushed."]
    _0 = 0,
    #[doc = "1: The Transmit Queues in I3C is flushed."]
    _1 = 1,
}
impl From<TDBRST_A> for bool {
    #[inline(always)]
    fn from(variant: TDBRST_A) -> Self {
        variant as u8 != 0
    }
}
impl TDBRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDBRST_A {
        match self.bits {
            false => TDBRST_A::_0,
            true => TDBRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDBRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDBRST_A::_1
    }
}
#[doc = "Field `TDBRST` writer - Transmit Data Buffer Software Reset"]
pub type TDBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_SPEC, TDBRST_A, O>;
impl<'a, const O: u8> TDBRST_W<'a, O> {
    #[doc = "The Transmit Queues in I3C is not flushed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDBRST_A::_0)
    }
    #[doc = "The Transmit Queues in I3C is flushed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDBRST_A::_1)
    }
}
#[doc = "Field `RDBRST` reader - Receive Data Buffer Software Reset"]
pub type RDBRST_R = crate::BitReader<RDBRST_A>;
#[doc = "Receive Data Buffer Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDBRST_A {
    #[doc = "0: The Receive Queues in I3C is not flushed."]
    _0 = 0,
    #[doc = "1: The Receive Queues in I3C is flushed."]
    _1 = 1,
}
impl From<RDBRST_A> for bool {
    #[inline(always)]
    fn from(variant: RDBRST_A) -> Self {
        variant as u8 != 0
    }
}
impl RDBRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDBRST_A {
        match self.bits {
            false => RDBRST_A::_0,
            true => RDBRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDBRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDBRST_A::_1
    }
}
#[doc = "Field `RDBRST` writer - Receive Data Buffer Software Reset"]
pub type RDBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_SPEC, RDBRST_A, O>;
impl<'a, const O: u8> RDBRST_W<'a, O> {
    #[doc = "The Receive Queues in I3C is not flushed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDBRST_A::_0)
    }
    #[doc = "The Receive Queues in I3C is flushed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDBRST_A::_1)
    }
}
#[doc = "Field `IBIQRST` reader - IBI Queue Software Reset"]
pub type IBIQRST_R = crate::BitReader<IBIQRST_A>;
#[doc = "IBI Queue Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IBIQRST_A {
    #[doc = "0: The IBI Queues in I3C is not flushed."]
    _0 = 0,
    #[doc = "1: The IBI Queues in I3C is flushed."]
    _1 = 1,
}
impl From<IBIQRST_A> for bool {
    #[inline(always)]
    fn from(variant: IBIQRST_A) -> Self {
        variant as u8 != 0
    }
}
impl IBIQRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBIQRST_A {
        match self.bits {
            false => IBIQRST_A::_0,
            true => IBIQRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IBIQRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IBIQRST_A::_1
    }
}
#[doc = "Field `IBIQRST` writer - IBI Queue Software Reset"]
pub type IBIQRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_SPEC, IBIQRST_A, O>;
impl<'a, const O: u8> IBIQRST_W<'a, O> {
    #[doc = "The IBI Queues in I3C is not flushed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IBIQRST_A::_0)
    }
    #[doc = "The IBI Queues in I3C is flushed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IBIQRST_A::_1)
    }
}
#[doc = "Field `RSQRST` reader - Receive Status Queue Software Reset"]
pub type RSQRST_R = crate::BitReader<RSQRST_A>;
#[doc = "Receive Status Queue Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSQRST_A {
    #[doc = "0: The Receive Status Queue in I3C is not flushed."]
    _0 = 0,
    #[doc = "1: The Receive Status Queue in I3C is flushed."]
    _1 = 1,
}
impl From<RSQRST_A> for bool {
    #[inline(always)]
    fn from(variant: RSQRST_A) -> Self {
        variant as u8 != 0
    }
}
impl RSQRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSQRST_A {
        match self.bits {
            false => RSQRST_A::_0,
            true => RSQRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSQRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSQRST_A::_1
    }
}
#[doc = "Field `RSQRST` writer - Receive Status Queue Software Reset"]
pub type RSQRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_SPEC, RSQRST_A, O>;
impl<'a, const O: u8> RSQRST_W<'a, O> {
    #[doc = "The Receive Status Queue in I3C is not flushed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSQRST_A::_0)
    }
    #[doc = "The Receive Status Queue in I3C is flushed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSQRST_A::_1)
    }
}
#[doc = "Field `INTLRST` reader - Internal Software Reset"]
pub type INTLRST_R = crate::BitReader<INTLRST_A>;
#[doc = "Internal Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTLRST_A {
    #[doc = "0: Releases of some registers and internal state."]
    _0 = 0,
    #[doc = "1: Resets of some registers and internal state."]
    _1 = 1,
}
impl From<INTLRST_A> for bool {
    #[inline(always)]
    fn from(variant: INTLRST_A) -> Self {
        variant as u8 != 0
    }
}
impl INTLRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTLRST_A {
        match self.bits {
            false => INTLRST_A::_0,
            true => INTLRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INTLRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INTLRST_A::_1
    }
}
#[doc = "Field `INTLRST` writer - Internal Software Reset"]
pub type INTLRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_SPEC, INTLRST_A, O>;
impl<'a, const O: u8> INTLRST_W<'a, O> {
    #[doc = "Releases of some registers and internal state."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTLRST_A::_0)
    }
    #[doc = "Resets of some registers and internal state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTLRST_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - I3C Software Reset"]
    #[inline(always)]
    pub fn ri3crst(&self) -> RI3CRST_R {
        RI3CRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command Queue Software Reset"]
    #[inline(always)]
    pub fn cmdqrst(&self) -> CMDQRST_R {
        CMDQRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Response Queue Software Reset"]
    #[inline(always)]
    pub fn rspqrst(&self) -> RSPQRST_R {
        RSPQRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Data Buffer Software Reset"]
    #[inline(always)]
    pub fn tdbrst(&self) -> TDBRST_R {
        TDBRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Data Buffer Software Reset"]
    #[inline(always)]
    pub fn rdbrst(&self) -> RDBRST_R {
        RDBRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IBI Queue Software Reset"]
    #[inline(always)]
    pub fn ibiqrst(&self) -> IBIQRST_R {
        IBIQRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Status Queue Software Reset"]
    #[inline(always)]
    pub fn rsqrst(&self) -> RSQRST_R {
        RSQRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - Internal Software Reset"]
    #[inline(always)]
    pub fn intlrst(&self) -> INTLRST_R {
        INTLRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I3C Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn ri3crst(&mut self) -> RI3CRST_W<0> {
        RI3CRST_W::new(self)
    }
    #[doc = "Bit 1 - Command Queue Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn cmdqrst(&mut self) -> CMDQRST_W<1> {
        CMDQRST_W::new(self)
    }
    #[doc = "Bit 2 - Response Queue Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rspqrst(&mut self) -> RSPQRST_W<2> {
        RSPQRST_W::new(self)
    }
    #[doc = "Bit 3 - Transmit Data Buffer Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn tdbrst(&mut self) -> TDBRST_W<3> {
        TDBRST_W::new(self)
    }
    #[doc = "Bit 4 - Receive Data Buffer Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rdbrst(&mut self) -> RDBRST_W<4> {
        RDBRST_W::new(self)
    }
    #[doc = "Bit 5 - IBI Queue Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn ibiqrst(&mut self) -> IBIQRST_W<5> {
        IBIQRST_W::new(self)
    }
    #[doc = "Bit 6 - Receive Status Queue Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rsqrst(&mut self) -> RSQRST_W<6> {
        RSQRST_W::new(self)
    }
    #[doc = "Bit 16 - Internal Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn intlrst(&mut self) -> INTLRST_W<16> {
        INTLRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl](index.html) module"]
pub struct RSTCTL_SPEC;
impl crate::RegisterSpec for RSTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstctl::R](R) reader structure"]
impl crate::Readable for RSTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstctl::W](W) writer structure"]
impl crate::Writable for RSTCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSTCTL to value 0"]
impl crate::Resettable for RSTCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
