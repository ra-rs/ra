#[doc = "Register `OCTACKCR` reader"]
pub struct R(crate::R<OCTACKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCTACKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCTACKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCTACKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCTACKCR` writer"]
pub struct W(crate::W<OCTACKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCTACKCR_SPEC>;
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
impl From<crate::W<OCTACKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCTACKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OCTACKSEL` reader - Octal-SPI Clock (OCTACLK) Source Select"]
pub type OCTACKSEL_R = crate::FieldReader<u8, OCTACKSEL_A>;
#[doc = "Octal-SPI Clock (OCTACLK) Source Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OCTACKSEL_A {
    #[doc = "0: HOCO"]
    _000 = 0,
    #[doc = "1: MOCO (value after reset)"]
    _001 = 1,
    #[doc = "2: LOCO"]
    _010 = 2,
    #[doc = "3: Main clock oscillator"]
    _011 = 3,
    #[doc = "4: Sub-clock oscillator"]
    _100 = 4,
    #[doc = "5: PLL"]
    _101 = 5,
    #[doc = "6: PLL2"]
    _110 = 6,
}
impl From<OCTACKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OCTACKSEL_A) -> Self {
        variant as _
    }
}
impl OCTACKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCTACKSEL_A {
        match self.bits {
            0 => OCTACKSEL_A::_000,
            1 => OCTACKSEL_A::_001,
            2 => OCTACKSEL_A::_010,
            3 => OCTACKSEL_A::_011,
            4 => OCTACKSEL_A::_100,
            5 => OCTACKSEL_A::_101,
            6 => OCTACKSEL_A::_110,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == OCTACKSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == OCTACKSEL_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == OCTACKSEL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == OCTACKSEL_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == OCTACKSEL_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == OCTACKSEL_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == OCTACKSEL_A::_110
    }
}
#[doc = "Field `OCTACKSEL` writer - Octal-SPI Clock (OCTACLK) Source Select"]
pub type OCTACKSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, OCTACKCR_SPEC, u8, OCTACKSEL_A, 3, O>;
impl<'a, const O: u8> OCTACKSEL_W<'a, O> {
    #[doc = "HOCO"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(OCTACKSEL_A::_000)
    }
    #[doc = "MOCO (value after reset)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(OCTACKSEL_A::_001)
    }
    #[doc = "LOCO"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(OCTACKSEL_A::_010)
    }
    #[doc = "Main clock oscillator"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(OCTACKSEL_A::_011)
    }
    #[doc = "Sub-clock oscillator"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(OCTACKSEL_A::_100)
    }
    #[doc = "PLL"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(OCTACKSEL_A::_101)
    }
    #[doc = "PLL2"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(OCTACKSEL_A::_110)
    }
}
#[doc = "Field `OCTACKSREQ` reader - Octal-SPI Clock (OCTACLK) Switching Request"]
pub type OCTACKSREQ_R = crate::BitReader<OCTACKSREQ_A>;
#[doc = "Octal-SPI Clock (OCTACLK) Switching Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCTACKSREQ_A {
    #[doc = "0: No request"]
    _0 = 0,
    #[doc = "1: Request switching."]
    _1 = 1,
}
impl From<OCTACKSREQ_A> for bool {
    #[inline(always)]
    fn from(variant: OCTACKSREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl OCTACKSREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCTACKSREQ_A {
        match self.bits {
            false => OCTACKSREQ_A::_0,
            true => OCTACKSREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OCTACKSREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OCTACKSREQ_A::_1
    }
}
#[doc = "Field `OCTACKSREQ` writer - Octal-SPI Clock (OCTACLK) Switching Request"]
pub type OCTACKSREQ_W<'a, const O: u8> = crate::BitWriter<'a, u8, OCTACKCR_SPEC, OCTACKSREQ_A, O>;
impl<'a, const O: u8> OCTACKSREQ_W<'a, O> {
    #[doc = "No request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OCTACKSREQ_A::_0)
    }
    #[doc = "Request switching."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OCTACKSREQ_A::_1)
    }
}
#[doc = "Field `OCTACKSRDY` reader - Octal-SPI Clock (OCTACLK) Switching Ready state flag"]
pub type OCTACKSRDY_R = crate::BitReader<OCTACKSRDY_A>;
#[doc = "Octal-SPI Clock (OCTACLK) Switching Ready state flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCTACKSRDY_A {
    #[doc = "0: Switching not possible"]
    _0 = 0,
    #[doc = "1: Switching possible."]
    _1 = 1,
}
impl From<OCTACKSRDY_A> for bool {
    #[inline(always)]
    fn from(variant: OCTACKSRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl OCTACKSRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCTACKSRDY_A {
        match self.bits {
            false => OCTACKSRDY_A::_0,
            true => OCTACKSRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OCTACKSRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OCTACKSRDY_A::_1
    }
}
impl R {
    #[doc = "Bits 0:2 - Octal-SPI Clock (OCTACLK) Source Select"]
    #[inline(always)]
    pub fn octacksel(&self) -> OCTACKSEL_R {
        OCTACKSEL_R::new(self.bits & 7)
    }
    #[doc = "Bit 6 - Octal-SPI Clock (OCTACLK) Switching Request"]
    #[inline(always)]
    pub fn octacksreq(&self) -> OCTACKSREQ_R {
        OCTACKSREQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Octal-SPI Clock (OCTACLK) Switching Ready state flag"]
    #[inline(always)]
    pub fn octacksrdy(&self) -> OCTACKSRDY_R {
        OCTACKSRDY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Octal-SPI Clock (OCTACLK) Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn octacksel(&mut self) -> OCTACKSEL_W<0> {
        OCTACKSEL_W::new(self)
    }
    #[doc = "Bit 6 - Octal-SPI Clock (OCTACLK) Switching Request"]
    #[inline(always)]
    #[must_use]
    pub fn octacksreq(&mut self) -> OCTACKSREQ_W<6> {
        OCTACKSREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Octal-SPI Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [octackcr](index.html) module"]
pub struct OCTACKCR_SPEC;
impl crate::RegisterSpec for OCTACKCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [octackcr::R](R) reader structure"]
impl crate::Readable for OCTACKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [octackcr::W](W) writer structure"]
impl crate::Writable for OCTACKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCTACKCR to value 0x01"]
impl crate::Resettable for OCTACKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
