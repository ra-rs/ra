#[doc = "Register `SNZCR` reader"]
pub struct R(crate::R<SNZCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SNZCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SNZCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SNZCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SNZCR` writer"]
pub struct W(crate::W<SNZCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SNZCR_SPEC>;
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
impl From<crate::W<SNZCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SNZCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXDREQEN` reader - RXD0 Snooze Request Enable"]
pub type RXDREQEN_R = crate::BitReader<RXDREQEN_A>;
#[doc = "RXD0 Snooze Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDREQEN_A {
    #[doc = "0: Ignore RXD0 falling edge in Software Standby mode"]
    _0 = 0,
    #[doc = "1: Detect RXD0 falling edge in Software Standby mode"]
    _1 = 1,
}
impl From<RXDREQEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXDREQEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RXDREQEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDREQEN_A {
        match self.bits {
            false => RXDREQEN_A::_0,
            true => RXDREQEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXDREQEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXDREQEN_A::_1
    }
}
#[doc = "Field `RXDREQEN` writer - RXD0 Snooze Request Enable"]
pub type RXDREQEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, SNZCR_SPEC, RXDREQEN_A, O>;
impl<'a, const O: u8> RXDREQEN_W<'a, O> {
    #[doc = "Ignore RXD0 falling edge in Software Standby mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXDREQEN_A::_0)
    }
    #[doc = "Detect RXD0 falling edge in Software Standby mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXDREQEN_A::_1)
    }
}
#[doc = "Field `SNZDTCEN` reader - DTC Enable in Snooze mode"]
pub type SNZDTCEN_R = crate::BitReader<SNZDTCEN_A>;
#[doc = "DTC Enable in Snooze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZDTCEN_A {
    #[doc = "0: Disable DTC operation"]
    _0 = 0,
    #[doc = "1: Enable DTC operation"]
    _1 = 1,
}
impl From<SNZDTCEN_A> for bool {
    #[inline(always)]
    fn from(variant: SNZDTCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZDTCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNZDTCEN_A {
        match self.bits {
            false => SNZDTCEN_A::_0,
            true => SNZDTCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZDTCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZDTCEN_A::_1
    }
}
#[doc = "Field `SNZDTCEN` writer - DTC Enable in Snooze mode"]
pub type SNZDTCEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, SNZCR_SPEC, SNZDTCEN_A, O>;
impl<'a, const O: u8> SNZDTCEN_W<'a, O> {
    #[doc = "Disable DTC operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNZDTCEN_A::_0)
    }
    #[doc = "Enable DTC operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNZDTCEN_A::_1)
    }
}
#[doc = "Field `SNZE` reader - Snooze mode Enable"]
pub type SNZE_R = crate::BitReader<SNZE_A>;
#[doc = "Snooze mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZE_A {
    #[doc = "0: Disable Snooze mode"]
    _0 = 0,
    #[doc = "1: Enable Snooze mode"]
    _1 = 1,
}
impl From<SNZE_A> for bool {
    #[inline(always)]
    fn from(variant: SNZE_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNZE_A {
        match self.bits {
            false => SNZE_A::_0,
            true => SNZE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZE_A::_1
    }
}
#[doc = "Field `SNZE` writer - Snooze mode Enable"]
pub type SNZE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SNZCR_SPEC, SNZE_A, O>;
impl<'a, const O: u8> SNZE_W<'a, O> {
    #[doc = "Disable Snooze mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNZE_A::_0)
    }
    #[doc = "Enable Snooze mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNZE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - RXD0 Snooze Request Enable"]
    #[inline(always)]
    pub fn rxdreqen(&self) -> RXDREQEN_R {
        RXDREQEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DTC Enable in Snooze mode"]
    #[inline(always)]
    pub fn snzdtcen(&self) -> SNZDTCEN_R {
        SNZDTCEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - Snooze mode Enable"]
    #[inline(always)]
    pub fn snze(&self) -> SNZE_R {
        SNZE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXD0 Snooze Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdreqen(&mut self) -> RXDREQEN_W<0> {
        RXDREQEN_W::new(self)
    }
    #[doc = "Bit 1 - DTC Enable in Snooze mode"]
    #[inline(always)]
    #[must_use]
    pub fn snzdtcen(&mut self) -> SNZDTCEN_W<1> {
        SNZDTCEN_W::new(self)
    }
    #[doc = "Bit 7 - Snooze mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn snze(&mut self) -> SNZE_W<7> {
        SNZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Snooze Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snzcr](index.html) module"]
pub struct SNZCR_SPEC;
impl crate::RegisterSpec for SNZCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [snzcr::R](R) reader structure"]
impl crate::Readable for SNZCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [snzcr::W](W) writer structure"]
impl crate::Writable for SNZCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SNZCR to value 0"]
impl crate::Resettable for SNZCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
