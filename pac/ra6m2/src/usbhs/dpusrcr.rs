#[doc = "Register `DPUSRCR` reader"]
pub struct R(crate::R<DPUSRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPUSRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPUSRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPUSRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPUSRCR` writer"]
pub struct W(crate::W<DPUSRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPUSRCR_SPEC>;
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
impl From<crate::W<DPUSRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPUSRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIXPHY` reader - USB Transceiver Control Fix"]
pub type FIXPHY_R = crate::BitReader<FIXPHY_A>;
#[doc = "USB Transceiver Control Fix\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIXPHY_A {
    #[doc = "0: Normal mode"]
    _0 = 0,
    #[doc = "1: Go to/Return from deep software standby mode"]
    _1 = 1,
}
impl From<FIXPHY_A> for bool {
    #[inline(always)]
    fn from(variant: FIXPHY_A) -> Self {
        variant as u8 != 0
    }
}
impl FIXPHY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIXPHY_A {
        match self.bits {
            false => FIXPHY_A::_0,
            true => FIXPHY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIXPHY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIXPHY_A::_1
    }
}
#[doc = "Field `FIXPHY` writer - USB Transceiver Control Fix"]
pub type FIXPHY_W<'a, const O: u8> = crate::BitWriter<'a, u16, DPUSRCR_SPEC, FIXPHY_A, O>;
impl<'a, const O: u8> FIXPHY_W<'a, O> {
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIXPHY_A::_0)
    }
    #[doc = "Go to/Return from deep software standby mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIXPHY_A::_1)
    }
}
#[doc = "Field `FIXPHYPD` reader - USB Transceiver Control Fix for PLL"]
pub type FIXPHYPD_R = crate::BitReader<FIXPHYPD_A>;
#[doc = "USB Transceiver Control Fix for PLL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIXPHYPD_A {
    #[doc = "0: Normal mode"]
    _0 = 0,
    #[doc = "1: Go to/Return from deep software standby mode"]
    _1 = 1,
}
impl From<FIXPHYPD_A> for bool {
    #[inline(always)]
    fn from(variant: FIXPHYPD_A) -> Self {
        variant as u8 != 0
    }
}
impl FIXPHYPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIXPHYPD_A {
        match self.bits {
            false => FIXPHYPD_A::_0,
            true => FIXPHYPD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIXPHYPD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIXPHYPD_A::_1
    }
}
#[doc = "Field `FIXPHYPD` writer - USB Transceiver Control Fix for PLL"]
pub type FIXPHYPD_W<'a, const O: u8> = crate::BitWriter<'a, u16, DPUSRCR_SPEC, FIXPHYPD_A, O>;
impl<'a, const O: u8> FIXPHYPD_W<'a, O> {
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIXPHYPD_A::_0)
    }
    #[doc = "Go to/Return from deep software standby mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIXPHYPD_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - USB Transceiver Control Fix"]
    #[inline(always)]
    pub fn fixphy(&self) -> FIXPHY_R {
        FIXPHY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB Transceiver Control Fix for PLL"]
    #[inline(always)]
    pub fn fixphypd(&self) -> FIXPHYPD_R {
        FIXPHYPD_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Transceiver Control Fix"]
    #[inline(always)]
    #[must_use]
    pub fn fixphy(&mut self) -> FIXPHY_W<0> {
        FIXPHY_W::new(self)
    }
    #[doc = "Bit 1 - USB Transceiver Control Fix for PLL"]
    #[inline(always)]
    #[must_use]
    pub fn fixphypd(&mut self) -> FIXPHYPD_W<1> {
        FIXPHYPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep Standby USB Suspend/Resume Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpusrcr](index.html) module"]
pub struct DPUSRCR_SPEC;
impl crate::RegisterSpec for DPUSRCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dpusrcr::R](R) reader structure"]
impl crate::Readable for DPUSRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpusrcr::W](W) writer structure"]
impl crate::Writable for DPUSRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPUSRCR to value 0"]
impl crate::Resettable for DPUSRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
