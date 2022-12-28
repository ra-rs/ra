#[doc = "Register `SPCR2` reader"]
pub struct R(crate::R<SPCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPCR2` writer"]
pub struct W(crate::W<SPCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPCR2_SPEC>;
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
impl From<crate::W<SPCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RMFM` reader - Frame processing count setting in Master Receive only"]
pub type RMFM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RMFM` writer - Frame processing count setting in Master Receive only"]
pub type RMFM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPCR2_SPEC, u8, u8, 5, O>;
#[doc = "End Trigger in Master Receive only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMEDTG_AW {
    #[doc = "1: Receive End (Writable only when Master Receive only) Reading value is always 0"]
    _1 = 1,
}
impl From<RMEDTG_AW> for bool {
    #[inline(always)]
    fn from(variant: RMEDTG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMEDTG` writer - End Trigger in Master Receive only"]
pub type RMEDTG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCR2_SPEC, RMEDTG_AW, O>;
impl<'a, const O: u8> RMEDTG_W<'a, O> {
    #[doc = "Receive End (Writable only when Master Receive only) Reading value is always 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RMEDTG_AW::_1)
    }
}
#[doc = "Start Trigger in Master Receive only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMSTTG_AW {
    #[doc = "1: Receive Start (Writable only when Master Receive only) Reading value is always 0"]
    _1 = 1,
}
impl From<RMSTTG_AW> for bool {
    #[inline(always)]
    fn from(variant: RMSTTG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMSTTG` writer - Start Trigger in Master Receive only"]
pub type RMSTTG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCR2_SPEC, RMSTTG_AW, O>;
impl<'a, const O: u8> RMSTTG_W<'a, O> {
    #[doc = "Receive Start (Writable only when Master Receive only) Reading value is always 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RMSTTG_AW::_1)
    }
}
#[doc = "Field `SPDRC` reader - SPI received data ready detect adjustment"]
pub type SPDRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPDRC` writer - SPI received data ready detect adjustment"]
pub type SPDRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPCR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `SPLP` reader - SPI Loopback"]
pub type SPLP_R = crate::BitReader<SPLP_A>;
#[doc = "SPI Loopback\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPLP_A {
    #[doc = "0: Normal mode"]
    _0 = 0,
    #[doc = "1: Loopback mode (inverted transmit data = receive data)"]
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
pub type SPLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCR2_SPEC, SPLP_A, O>;
impl<'a, const O: u8> SPLP_W<'a, O> {
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLP_A::_0)
    }
    #[doc = "Loopback mode (inverted transmit data = receive data)"]
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
    #[doc = "1: Loopback mode (transmit data = receive data)"]
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
pub type SPLP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCR2_SPEC, SPLP2_A, O>;
impl<'a, const O: u8> SPLP2_W<'a, O> {
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLP2_A::_0)
    }
    #[doc = "Loopback mode (transmit data = receive data)"]
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
    #[doc = "0: The fixed value of MOSI idle = 0."]
    _0 = 0,
    #[doc = "1: The fixed value of MOSI idle = 1."]
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
pub type MOIFV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCR2_SPEC, MOIFV_A, O>;
impl<'a, const O: u8> MOIFV_W<'a, O> {
    #[doc = "The fixed value of MOSI idle = 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MOIFV_A::_0)
    }
    #[doc = "The fixed value of MOSI idle = 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MOIFV_A::_1)
    }
}
#[doc = "Field `MOIFE` reader - MOSI Idle Fixed Value Enable"]
pub type MOIFE_R = crate::BitReader<MOIFE_A>;
#[doc = "MOSI Idle Fixed Value Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOIFE_A {
    #[doc = "0: The MOSI output value is the last data of previous transfer."]
    _0 = 0,
    #[doc = "1: The MOSI output value is the set MOIFV bit value."]
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
#[doc = "Field `MOIFE` writer - MOSI Idle Fixed Value Enable"]
pub type MOIFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCR2_SPEC, MOIFE_A, O>;
impl<'a, const O: u8> MOIFE_W<'a, O> {
    #[doc = "The MOSI output value is the last data of previous transfer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MOIFE_A::_0)
    }
    #[doc = "The MOSI output value is the set MOIFV bit value."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MOIFE_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:4 - Frame processing count setting in Master Receive only"]
    #[inline(always)]
    pub fn rmfm(&self) -> RMFM_R {
        RMFM_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - SPI received data ready detect adjustment"]
    #[inline(always)]
    pub fn spdrc(&self) -> SPDRC_R {
        SPDRC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - SPI Loopback"]
    #[inline(always)]
    pub fn splp(&self) -> SPLP_R {
        SPLP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SPI Loopback 2"]
    #[inline(always)]
    pub fn splp2(&self) -> SPLP2_R {
        SPLP2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - MOSI Idle Fixed Value"]
    #[inline(always)]
    pub fn moifv(&self) -> MOIFV_R {
        MOIFV_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MOSI Idle Fixed Value Enable"]
    #[inline(always)]
    pub fn moife(&self) -> MOIFE_R {
        MOIFE_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Frame processing count setting in Master Receive only"]
    #[inline(always)]
    #[must_use]
    pub fn rmfm(&mut self) -> RMFM_W<0> {
        RMFM_W::new(self)
    }
    #[doc = "Bit 6 - End Trigger in Master Receive only"]
    #[inline(always)]
    #[must_use]
    pub fn rmedtg(&mut self) -> RMEDTG_W<6> {
        RMEDTG_W::new(self)
    }
    #[doc = "Bit 7 - Start Trigger in Master Receive only"]
    #[inline(always)]
    #[must_use]
    pub fn rmsttg(&mut self) -> RMSTTG_W<7> {
        RMSTTG_W::new(self)
    }
    #[doc = "Bits 8:15 - SPI received data ready detect adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn spdrc(&mut self) -> SPDRC_W<8> {
        SPDRC_W::new(self)
    }
    #[doc = "Bit 16 - SPI Loopback"]
    #[inline(always)]
    #[must_use]
    pub fn splp(&mut self) -> SPLP_W<16> {
        SPLP_W::new(self)
    }
    #[doc = "Bit 17 - SPI Loopback 2"]
    #[inline(always)]
    #[must_use]
    pub fn splp2(&mut self) -> SPLP2_W<17> {
        SPLP2_W::new(self)
    }
    #[doc = "Bit 20 - MOSI Idle Fixed Value"]
    #[inline(always)]
    #[must_use]
    pub fn moifv(&mut self) -> MOIFV_W<20> {
        MOIFV_W::new(self)
    }
    #[doc = "Bit 21 - MOSI Idle Fixed Value Enable"]
    #[inline(always)]
    #[must_use]
    pub fn moife(&mut self) -> MOIFE_W<21> {
        MOIFE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spcr2](index.html) module"]
pub struct SPCR2_SPEC;
impl crate::RegisterSpec for SPCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spcr2::R](R) reader structure"]
impl crate::Readable for SPCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spcr2::W](W) writer structure"]
impl crate::Writable for SPCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPCR2 to value 0"]
impl crate::Resettable for SPCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
