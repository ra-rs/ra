#[doc = "Register `CANFDCKCR` reader"]
pub struct R(crate::R<CANFDCKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CANFDCKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CANFDCKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CANFDCKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CANFDCKCR` writer"]
pub struct W(crate::W<CANFDCKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CANFDCKCR_SPEC>;
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
impl From<crate::W<CANFDCKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CANFDCKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CANFDCKSEL` reader - CANFD clock (CANFDCLK) Source Select"]
pub type CANFDCKSEL_R = crate::FieldReader<u8, CANFDCKSEL_A>;
#[doc = "CANFD clock (CANFDCLK) Source Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CANFDCKSEL_A {
    #[doc = "5: PLL"]
    _101 = 5,
    #[doc = "6: PLL2"]
    _110 = 6,
}
impl From<CANFDCKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CANFDCKSEL_A) -> Self {
        variant as _
    }
}
impl CANFDCKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CANFDCKSEL_A> {
        match self.bits {
            5 => Some(CANFDCKSEL_A::_101),
            6 => Some(CANFDCKSEL_A::_110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CANFDCKSEL_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == CANFDCKSEL_A::_110
    }
}
#[doc = "Field `CANFDCKSEL` writer - CANFD clock (CANFDCLK) Source Select"]
pub type CANFDCKSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, CANFDCKCR_SPEC, u8, CANFDCKSEL_A, 3, O>;
impl<'a, const O: u8> CANFDCKSEL_W<'a, O> {
    #[doc = "PLL"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(CANFDCKSEL_A::_101)
    }
    #[doc = "PLL2"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(CANFDCKSEL_A::_110)
    }
}
#[doc = "Field `CANFDCKSREQ` reader - CANFD clock (CANFDCLK) Switching Request"]
pub type CANFDCKSREQ_R = crate::BitReader<CANFDCKSREQ_A>;
#[doc = "CANFD clock (CANFDCLK) Switching Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CANFDCKSREQ_A {
    #[doc = "0: No request"]
    _0 = 0,
    #[doc = "1: Request switching"]
    _1 = 1,
}
impl From<CANFDCKSREQ_A> for bool {
    #[inline(always)]
    fn from(variant: CANFDCKSREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl CANFDCKSREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CANFDCKSREQ_A {
        match self.bits {
            false => CANFDCKSREQ_A::_0,
            true => CANFDCKSREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CANFDCKSREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CANFDCKSREQ_A::_1
    }
}
#[doc = "Field `CANFDCKSREQ` writer - CANFD clock (CANFDCLK) Switching Request"]
pub type CANFDCKSREQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, CANFDCKCR_SPEC, CANFDCKSREQ_A, O>;
impl<'a, const O: u8> CANFDCKSREQ_W<'a, O> {
    #[doc = "No request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CANFDCKSREQ_A::_0)
    }
    #[doc = "Request switching"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CANFDCKSREQ_A::_1)
    }
}
#[doc = "Field `CANFDCKSRDY` reader - CANFD clock (CANFDCLK) Switching Ready state flag"]
pub type CANFDCKSRDY_R = crate::BitReader<CANFDCKSRDY_A>;
#[doc = "CANFD clock (CANFDCLK) Switching Ready state flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CANFDCKSRDY_A {
    #[doc = "0: Impossible to Switch"]
    _0 = 0,
    #[doc = "1: Possible to Switch"]
    _1 = 1,
}
impl From<CANFDCKSRDY_A> for bool {
    #[inline(always)]
    fn from(variant: CANFDCKSRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl CANFDCKSRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CANFDCKSRDY_A {
        match self.bits {
            false => CANFDCKSRDY_A::_0,
            true => CANFDCKSRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CANFDCKSRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CANFDCKSRDY_A::_1
    }
}
impl R {
    #[doc = "Bits 0:2 - CANFD clock (CANFDCLK) Source Select"]
    #[inline(always)]
    pub fn canfdcksel(&self) -> CANFDCKSEL_R {
        CANFDCKSEL_R::new(self.bits & 7)
    }
    #[doc = "Bit 6 - CANFD clock (CANFDCLK) Switching Request"]
    #[inline(always)]
    pub fn canfdcksreq(&self) -> CANFDCKSREQ_R {
        CANFDCKSREQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CANFD clock (CANFDCLK) Switching Ready state flag"]
    #[inline(always)]
    pub fn canfdcksrdy(&self) -> CANFDCKSRDY_R {
        CANFDCKSRDY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CANFD clock (CANFDCLK) Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn canfdcksel(&mut self) -> CANFDCKSEL_W<0> {
        CANFDCKSEL_W::new(self)
    }
    #[doc = "Bit 6 - CANFD clock (CANFDCLK) Switching Request"]
    #[inline(always)]
    #[must_use]
    pub fn canfdcksreq(&mut self) -> CANFDCKSREQ_W<6> {
        CANFDCKSREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CANFD Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canfdckcr](index.html) module"]
pub struct CANFDCKCR_SPEC;
impl crate::RegisterSpec for CANFDCKCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [canfdckcr::R](R) reader structure"]
impl crate::Readable for CANFDCKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [canfdckcr::W](W) writer structure"]
impl crate::Writable for CANFDCKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CANFDCKCR to value 0x01"]
impl crate::Resettable for CANFDCKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
