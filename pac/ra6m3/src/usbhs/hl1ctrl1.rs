#[doc = "Register `HL1CTRL1` reader"]
pub struct R(crate::R<HL1CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HL1CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HL1CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HL1CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HL1CTRL1` writer"]
pub struct W(crate::W<HL1CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HL1CTRL1_SPEC>;
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
impl From<crate::W<HL1CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HL1CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L1REQ` reader - L1 Transition Request"]
pub type L1REQ_R = crate::BitReader<L1REQ_A>;
#[doc = "L1 Transition Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1REQ_A {
    #[doc = "0: This bit is cleared to 0 by hardware when the LPM transaction is completed."]
    _0 = 0,
    #[doc = "1: Set this bit to 1 when requesting a transition to the L1 state."]
    _1 = 1,
}
impl From<L1REQ_A> for bool {
    #[inline(always)]
    fn from(variant: L1REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl L1REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L1REQ_A {
        match self.bits {
            false => L1REQ_A::_0,
            true => L1REQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L1REQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L1REQ_A::_1
    }
}
#[doc = "Field `L1REQ` writer - L1 Transition Request"]
pub type L1REQ_W<'a, const O: u8> = crate::BitWriter<'a, u16, HL1CTRL1_SPEC, L1REQ_A, O>;
impl<'a, const O: u8> L1REQ_W<'a, O> {
    #[doc = "This bit is cleared to 0 by hardware when the LPM transaction is completed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(L1REQ_A::_0)
    }
    #[doc = "Set this bit to 1 when requesting a transition to the L1 state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(L1REQ_A::_1)
    }
}
#[doc = "Field `L1STATUS` reader - L1 Request Completion Status"]
pub type L1STATUS_R = crate::FieldReader<u8, L1STATUS_A>;
#[doc = "L1 Request Completion Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum L1STATUS_A {
    #[doc = "0: ACK received"]
    _00 = 0,
    #[doc = "1: NYET received"]
    _01 = 1,
    #[doc = "2: STALL received"]
    _10 = 2,
    #[doc = "3: Transaction error"]
    _11 = 3,
}
impl From<L1STATUS_A> for u8 {
    #[inline(always)]
    fn from(variant: L1STATUS_A) -> Self {
        variant as _
    }
}
impl L1STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L1STATUS_A {
        match self.bits {
            0 => L1STATUS_A::_00,
            1 => L1STATUS_A::_01,
            2 => L1STATUS_A::_10,
            3 => L1STATUS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == L1STATUS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == L1STATUS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == L1STATUS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == L1STATUS_A::_11
    }
}
impl R {
    #[doc = "Bit 0 - L1 Transition Request"]
    #[inline(always)]
    pub fn l1req(&self) -> L1REQ_R {
        L1REQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - L1 Request Completion Status"]
    #[inline(always)]
    pub fn l1status(&self) -> L1STATUS_R {
        L1STATUS_R::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - L1 Transition Request"]
    #[inline(always)]
    #[must_use]
    pub fn l1req(&mut self) -> L1REQ_W<0> {
        L1REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host L1 Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hl1ctrl1](index.html) module"]
pub struct HL1CTRL1_SPEC;
impl crate::RegisterSpec for HL1CTRL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [hl1ctrl1::R](R) reader structure"]
impl crate::Readable for HL1CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hl1ctrl1::W](W) writer structure"]
impl crate::Writable for HL1CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HL1CTRL1 to value 0"]
impl crate::Resettable for HL1CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
