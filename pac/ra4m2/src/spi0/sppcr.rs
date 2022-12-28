#[doc = "Register `SPPCR` reader"]
pub struct R(crate::R<SPPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPPCR` writer"]
pub struct W(crate::W<SPPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPPCR_SPEC>;
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
impl From<crate::W<SPPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPLP` reader - SPI Loopback"]
pub type SPLP_R = crate::BitReader<SPLP_A>;
#[doc = "SPI Loopback\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPLP_A {
    #[doc = "0: Normal mode"]
    _0 = 0,
    #[doc = "1: Loopback mode (receive data = inverted transmit data)"]
    _1 = 1,
}
impl From<SPLP_A> for bool {
    #[inline(always)]
    fn from(variant: SPLP_A) -> Self {
        variant as u8 != 0
    }
}
impl SPLP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLP_A {
        match self.bits {
            false => SPLP_A::_0,
            true => SPLP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPLP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPLP_A::_1
    }
}
#[doc = "Field `SPLP` writer - SPI Loopback"]
pub type SPLP_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPPCR_SPEC, SPLP_A, O>;
impl<'a, const O: u8> SPLP_W<'a, O> {
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLP_A::_0)
    }
    #[doc = "Loopback mode (receive data = inverted transmit data)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLP_A::_1)
    }
}
#[doc = "Field `SPLP2` reader - SPI Loopback 2"]
pub type SPLP2_R = crate::BitReader<SPLP2_A>;
#[doc = "SPI Loopback 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPLP2_A {
    #[doc = "0: Normal mode"]
    _0 = 0,
    #[doc = "1: Loopback mode (receive data = transmit data)"]
    _1 = 1,
}
impl From<SPLP2_A> for bool {
    #[inline(always)]
    fn from(variant: SPLP2_A) -> Self {
        variant as u8 != 0
    }
}
impl SPLP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLP2_A {
        match self.bits {
            false => SPLP2_A::_0,
            true => SPLP2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPLP2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPLP2_A::_1
    }
}
#[doc = "Field `SPLP2` writer - SPI Loopback 2"]
pub type SPLP2_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPPCR_SPEC, SPLP2_A, O>;
impl<'a, const O: u8> SPLP2_W<'a, O> {
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLP2_A::_0)
    }
    #[doc = "Loopback mode (receive data = transmit data)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLP2_A::_1)
    }
}
#[doc = "Field `MOIFV` reader - MOSI Idle Fixed Value"]
pub type MOIFV_R = crate::BitReader<MOIFV_A>;
#[doc = "MOSI Idle Fixed Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOIFV_A {
    #[doc = "0: Set level output on MOSIn pin during MOSI idling to low"]
    _0 = 0,
    #[doc = "1: Set level output on MOSIn pin during MOSI idling to high"]
    _1 = 1,
}
impl From<MOIFV_A> for bool {
    #[inline(always)]
    fn from(variant: MOIFV_A) -> Self {
        variant as u8 != 0
    }
}
impl MOIFV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOIFV_A {
        match self.bits {
            false => MOIFV_A::_0,
            true => MOIFV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MOIFV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MOIFV_A::_1
    }
}
#[doc = "Field `MOIFV` writer - MOSI Idle Fixed Value"]
pub type MOIFV_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPPCR_SPEC, MOIFV_A, O>;
impl<'a, const O: u8> MOIFV_W<'a, O> {
    #[doc = "Set level output on MOSIn pin during MOSI idling to low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MOIFV_A::_0)
    }
    #[doc = "Set level output on MOSIn pin during MOSI idling to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MOIFV_A::_1)
    }
}
#[doc = "Field `MOIFE` reader - MOSI Idle Value Fixing Enable"]
pub type MOIFE_R = crate::BitReader<MOIFE_A>;
#[doc = "MOSI Idle Value Fixing Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOIFE_A {
    #[doc = "0: Set MOSI output value to equal final data from previous transfer"]
    _0 = 0,
    #[doc = "1: Set MOSI output value to equal value set in the MOIFV bit"]
    _1 = 1,
}
impl From<MOIFE_A> for bool {
    #[inline(always)]
    fn from(variant: MOIFE_A) -> Self {
        variant as u8 != 0
    }
}
impl MOIFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOIFE_A {
        match self.bits {
            false => MOIFE_A::_0,
            true => MOIFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MOIFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MOIFE_A::_1
    }
}
#[doc = "Field `MOIFE` writer - MOSI Idle Value Fixing Enable"]
pub type MOIFE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPPCR_SPEC, MOIFE_A, O>;
impl<'a, const O: u8> MOIFE_W<'a, O> {
    #[doc = "Set MOSI output value to equal final data from previous transfer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MOIFE_A::_0)
    }
    #[doc = "Set MOSI output value to equal value set in the MOIFV bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MOIFE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - SPI Loopback"]
    #[inline(always)]
    pub fn splp(&self) -> SPLP_R {
        SPLP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI Loopback 2"]
    #[inline(always)]
    pub fn splp2(&self) -> SPLP2_R {
        SPLP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - MOSI Idle Fixed Value"]
    #[inline(always)]
    pub fn moifv(&self) -> MOIFV_R {
        MOIFV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MOSI Idle Value Fixing Enable"]
    #[inline(always)]
    pub fn moife(&self) -> MOIFE_R {
        MOIFE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Loopback"]
    #[inline(always)]
    #[must_use]
    pub fn splp(&mut self) -> SPLP_W<0> {
        SPLP_W::new(self)
    }
    #[doc = "Bit 1 - SPI Loopback 2"]
    #[inline(always)]
    #[must_use]
    pub fn splp2(&mut self) -> SPLP2_W<1> {
        SPLP2_W::new(self)
    }
    #[doc = "Bit 4 - MOSI Idle Fixed Value"]
    #[inline(always)]
    #[must_use]
    pub fn moifv(&mut self) -> MOIFV_W<4> {
        MOIFV_W::new(self)
    }
    #[doc = "Bit 5 - MOSI Idle Value Fixing Enable"]
    #[inline(always)]
    #[must_use]
    pub fn moife(&mut self) -> MOIFE_W<5> {
        MOIFE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Pin Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sppcr](index.html) module"]
pub struct SPPCR_SPEC;
impl crate::RegisterSpec for SPPCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sppcr::R](R) reader structure"]
impl crate::Readable for SPPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sppcr::W](W) writer structure"]
impl crate::Writable for SPPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPPCR to value 0"]
impl crate::Resettable for SPPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
