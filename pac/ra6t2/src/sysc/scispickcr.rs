#[doc = "Register `SCISPICKCR` reader"]
pub struct R(crate::R<SCISPICKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCISPICKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCISPICKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCISPICKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCISPICKCR` writer"]
pub struct W(crate::W<SCISPICKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCISPICKCR_SPEC>;
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
impl From<crate::W<SCISPICKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCISPICKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCISPICKSEL` reader - SCI SPI Clock (SCISPICLK) Source Select"]
pub type SCISPICKSEL_R = crate::FieldReader<u8, SCISPICKSEL_A>;
#[doc = "SCI SPI Clock (SCISPICLK) Source Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCISPICKSEL_A {
    #[doc = "0: HOCO"]
    _000 = 0,
    #[doc = "1: MOCO (value after reset)"]
    _001 = 1,
    #[doc = "2: LOCO"]
    _010 = 2,
    #[doc = "3: Main clock oscillator"]
    _011 = 3,
    #[doc = "5: PLL"]
    _101 = 5,
    #[doc = "6: PLL2"]
    _110 = 6,
}
impl From<SCISPICKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SCISPICKSEL_A) -> Self {
        variant as _
    }
}
impl SCISPICKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SCISPICKSEL_A> {
        match self.bits {
            0 => Some(SCISPICKSEL_A::_000),
            1 => Some(SCISPICKSEL_A::_001),
            2 => Some(SCISPICKSEL_A::_010),
            3 => Some(SCISPICKSEL_A::_011),
            5 => Some(SCISPICKSEL_A::_101),
            6 => Some(SCISPICKSEL_A::_110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SCISPICKSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SCISPICKSEL_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SCISPICKSEL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SCISPICKSEL_A::_011
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SCISPICKSEL_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SCISPICKSEL_A::_110
    }
}
#[doc = "Field `SCISPICKSEL` writer - SCI SPI Clock (SCISPICLK) Source Select"]
pub type SCISPICKSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, SCISPICKCR_SPEC, u8, SCISPICKSEL_A, 3, O>;
impl<'a, const O: u8> SCISPICKSEL_W<'a, O> {
    #[doc = "HOCO"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(SCISPICKSEL_A::_000)
    }
    #[doc = "MOCO (value after reset)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(SCISPICKSEL_A::_001)
    }
    #[doc = "LOCO"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(SCISPICKSEL_A::_010)
    }
    #[doc = "Main clock oscillator"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(SCISPICKSEL_A::_011)
    }
    #[doc = "PLL"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(SCISPICKSEL_A::_101)
    }
    #[doc = "PLL2"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(SCISPICKSEL_A::_110)
    }
}
#[doc = "Field `SCISPICKSREQ` reader - SCI SPI Clock (SCISPICLK) Switching Request"]
pub type SCISPICKSREQ_R = crate::BitReader<SCISPICKSREQ_A>;
#[doc = "SCI SPI Clock (SCISPICLK) Switching Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCISPICKSREQ_A {
    #[doc = "0: No request"]
    _0 = 0,
    #[doc = "1: Request switching."]
    _1 = 1,
}
impl From<SCISPICKSREQ_A> for bool {
    #[inline(always)]
    fn from(variant: SCISPICKSREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl SCISPICKSREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCISPICKSREQ_A {
        match self.bits {
            false => SCISPICKSREQ_A::_0,
            true => SCISPICKSREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCISPICKSREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCISPICKSREQ_A::_1
    }
}
#[doc = "Field `SCISPICKSREQ` writer - SCI SPI Clock (SCISPICLK) Switching Request"]
pub type SCISPICKSREQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, SCISPICKCR_SPEC, SCISPICKSREQ_A, O>;
impl<'a, const O: u8> SCISPICKSREQ_W<'a, O> {
    #[doc = "No request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCISPICKSREQ_A::_0)
    }
    #[doc = "Request switching."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCISPICKSREQ_A::_1)
    }
}
#[doc = "Field `SCISPICKSRDY` reader - SCI SPI Clock (SCISPICLK) Switching Ready state flag"]
pub type SCISPICKSRDY_R = crate::BitReader<SCISPICKSRDY_A>;
#[doc = "SCI SPI Clock (SCISPICLK) Switching Ready state flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCISPICKSRDY_A {
    #[doc = "0: Switching not possible"]
    _0 = 0,
    #[doc = "1: Switching possible."]
    _1 = 1,
}
impl From<SCISPICKSRDY_A> for bool {
    #[inline(always)]
    fn from(variant: SCISPICKSRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl SCISPICKSRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCISPICKSRDY_A {
        match self.bits {
            false => SCISPICKSRDY_A::_0,
            true => SCISPICKSRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCISPICKSRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCISPICKSRDY_A::_1
    }
}
impl R {
    #[doc = "Bits 0:2 - SCI SPI Clock (SCISPICLK) Source Select"]
    #[inline(always)]
    pub fn scispicksel(&self) -> SCISPICKSEL_R {
        SCISPICKSEL_R::new(self.bits & 7)
    }
    #[doc = "Bit 6 - SCI SPI Clock (SCISPICLK) Switching Request"]
    #[inline(always)]
    pub fn scispicksreq(&self) -> SCISPICKSREQ_R {
        SCISPICKSREQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCI SPI Clock (SCISPICLK) Switching Ready state flag"]
    #[inline(always)]
    pub fn scispicksrdy(&self) -> SCISPICKSRDY_R {
        SCISPICKSRDY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCI SPI Clock (SCISPICLK) Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn scispicksel(&mut self) -> SCISPICKSEL_W<0> {
        SCISPICKSEL_W::new(self)
    }
    #[doc = "Bit 6 - SCI SPI Clock (SCISPICLK) Switching Request"]
    #[inline(always)]
    #[must_use]
    pub fn scispicksreq(&mut self) -> SCISPICKSREQ_W<6> {
        SCISPICKSREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCI SPI Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scispickcr](index.html) module"]
pub struct SCISPICKCR_SPEC;
impl crate::RegisterSpec for SCISPICKCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [scispickcr::R](R) reader structure"]
impl crate::Readable for SCISPICKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scispickcr::W](W) writer structure"]
impl crate::Writable for SCISPICKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCISPICKCR to value 0x01"]
impl crate::Resettable for SCISPICKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
