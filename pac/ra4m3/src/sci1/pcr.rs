#[doc = "Register `PCR` reader"]
pub struct R(crate::R<PCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCR` writer"]
pub struct W(crate::W<PCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCR_SPEC>;
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
impl From<crate::W<PCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDXPS` reader - TXDXn Signal Polarity Select"]
pub type TXDXPS_R = crate::BitReader<TXDXPS_A>;
#[doc = "TXDXn Signal Polarity Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDXPS_A {
    #[doc = "0: The polarity of TXDXn signal is not inverted for output."]
    _0 = 0,
    #[doc = "1: The polarity of TXDXn signal is inverted for output."]
    _1 = 1,
}
impl From<TXDXPS_A> for bool {
    #[inline(always)]
    fn from(variant: TXDXPS_A) -> Self {
        variant as u8 != 0
    }
}
impl TXDXPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDXPS_A {
        match self.bits {
            false => TXDXPS_A::_0,
            true => TXDXPS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXDXPS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXDXPS_A::_1
    }
}
#[doc = "Field `TXDXPS` writer - TXDXn Signal Polarity Select"]
pub type TXDXPS_W<'a, const O: u8> = crate::BitWriter<'a, u8, PCR_SPEC, TXDXPS_A, O>;
impl<'a, const O: u8> TXDXPS_W<'a, O> {
    #[doc = "The polarity of TXDXn signal is not inverted for output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXDXPS_A::_0)
    }
    #[doc = "The polarity of TXDXn signal is inverted for output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXDXPS_A::_1)
    }
}
#[doc = "Field `RXDXPS` reader - RXDXn Signal Polarity Select"]
pub type RXDXPS_R = crate::BitReader<RXDXPS_A>;
#[doc = "RXDXn Signal Polarity Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDXPS_A {
    #[doc = "0: The polarity of RXDXn signal is not inverted for input."]
    _0 = 0,
    #[doc = "1: The polarity of RXDXn signal is inverted for input."]
    _1 = 1,
}
impl From<RXDXPS_A> for bool {
    #[inline(always)]
    fn from(variant: RXDXPS_A) -> Self {
        variant as u8 != 0
    }
}
impl RXDXPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDXPS_A {
        match self.bits {
            false => RXDXPS_A::_0,
            true => RXDXPS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXDXPS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXDXPS_A::_1
    }
}
#[doc = "Field `RXDXPS` writer - RXDXn Signal Polarity Select"]
pub type RXDXPS_W<'a, const O: u8> = crate::BitWriter<'a, u8, PCR_SPEC, RXDXPS_A, O>;
impl<'a, const O: u8> RXDXPS_W<'a, O> {
    #[doc = "The polarity of RXDXn signal is not inverted for input."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXDXPS_A::_0)
    }
    #[doc = "The polarity of RXDXn signal is inverted for input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXDXPS_A::_1)
    }
}
#[doc = "Field `SHARPS` reader - TXDXn/RXDXn Pin Multiplexing Select"]
pub type SHARPS_R = crate::BitReader<SHARPS_A>;
#[doc = "TXDXn/RXDXn Pin Multiplexing Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHARPS_A {
    #[doc = "0: The TXDXn and RXDXn pins are independent."]
    _0 = 0,
    #[doc = "1: The TXDXn and RXDXn signals are multiplexed on the same pin."]
    _1 = 1,
}
impl From<SHARPS_A> for bool {
    #[inline(always)]
    fn from(variant: SHARPS_A) -> Self {
        variant as u8 != 0
    }
}
impl SHARPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHARPS_A {
        match self.bits {
            false => SHARPS_A::_0,
            true => SHARPS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHARPS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHARPS_A::_1
    }
}
#[doc = "Field `SHARPS` writer - TXDXn/RXDXn Pin Multiplexing Select"]
pub type SHARPS_W<'a, const O: u8> = crate::BitWriter<'a, u8, PCR_SPEC, SHARPS_A, O>;
impl<'a, const O: u8> SHARPS_W<'a, O> {
    #[doc = "The TXDXn and RXDXn pins are independent."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHARPS_A::_0)
    }
    #[doc = "The TXDXn and RXDXn signals are multiplexed on the same pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHARPS_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - TXDXn Signal Polarity Select"]
    #[inline(always)]
    pub fn txdxps(&self) -> TXDXPS_R {
        TXDXPS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXDXn Signal Polarity Select"]
    #[inline(always)]
    pub fn rxdxps(&self) -> RXDXPS_R {
        RXDXPS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - TXDXn/RXDXn Pin Multiplexing Select"]
    #[inline(always)]
    pub fn sharps(&self) -> SHARPS_R {
        SHARPS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TXDXn Signal Polarity Select"]
    #[inline(always)]
    #[must_use]
    pub fn txdxps(&mut self) -> TXDXPS_W<0> {
        TXDXPS_W::new(self)
    }
    #[doc = "Bit 1 - RXDXn Signal Polarity Select"]
    #[inline(always)]
    #[must_use]
    pub fn rxdxps(&mut self) -> RXDXPS_W<1> {
        RXDXPS_W::new(self)
    }
    #[doc = "Bit 4 - TXDXn/RXDXn Pin Multiplexing Select"]
    #[inline(always)]
    #[must_use]
    pub fn sharps(&mut self) -> SHARPS_W<4> {
        SHARPS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr](index.html) module"]
pub struct PCR_SPEC;
impl crate::RegisterSpec for PCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pcr::R](R) reader structure"]
impl crate::Readable for PCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcr::W](W) writer structure"]
impl crate::Writable for PCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCR to value 0"]
impl crate::Resettable for PCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
