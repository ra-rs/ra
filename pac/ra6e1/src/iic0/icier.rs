#[doc = "Register `ICIER` reader"]
pub struct R(crate::R<ICIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICIER` writer"]
pub struct W(crate::W<ICIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICIER_SPEC>;
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
impl From<crate::W<ICIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMOIE` reader - Timeout Interrupt Request Enable"]
pub type TMOIE_R = crate::BitReader<TMOIE_A>;
#[doc = "Timeout Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMOIE_A {
    #[doc = "0: Disable timeout interrupt (TMOI) request"]
    _0 = 0,
    #[doc = "1: Enable timeout interrupt (TMOI) request"]
    _1 = 1,
}
impl From<TMOIE_A> for bool {
    #[inline(always)]
    fn from(variant: TMOIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TMOIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMOIE_A {
        match self.bits {
            false => TMOIE_A::_0,
            true => TMOIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMOIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMOIE_A::_1
    }
}
#[doc = "Field `TMOIE` writer - Timeout Interrupt Request Enable"]
pub type TMOIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICIER_SPEC, TMOIE_A, O>;
impl<'a, const O: u8> TMOIE_W<'a, O> {
    #[doc = "Disable timeout interrupt (TMOI) request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMOIE_A::_0)
    }
    #[doc = "Enable timeout interrupt (TMOI) request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMOIE_A::_1)
    }
}
#[doc = "Field `ALIE` reader - Arbitration-Lost Interrupt Request Enable"]
pub type ALIE_R = crate::BitReader<ALIE_A>;
#[doc = "Arbitration-Lost Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALIE_A {
    #[doc = "0: Disable arbitration-lost interrupt (ALI) request"]
    _0 = 0,
    #[doc = "1: Enable arbitration-lost interrupt (ALI) request"]
    _1 = 1,
}
impl From<ALIE_A> for bool {
    #[inline(always)]
    fn from(variant: ALIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ALIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIE_A {
        match self.bits {
            false => ALIE_A::_0,
            true => ALIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALIE_A::_1
    }
}
#[doc = "Field `ALIE` writer - Arbitration-Lost Interrupt Request Enable"]
pub type ALIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICIER_SPEC, ALIE_A, O>;
impl<'a, const O: u8> ALIE_W<'a, O> {
    #[doc = "Disable arbitration-lost interrupt (ALI) request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALIE_A::_0)
    }
    #[doc = "Enable arbitration-lost interrupt (ALI) request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALIE_A::_1)
    }
}
#[doc = "Field `STIE` reader - Start Condition Detection Interrupt Request Enable"]
pub type STIE_R = crate::BitReader<STIE_A>;
#[doc = "Start Condition Detection Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STIE_A {
    #[doc = "0: Disable start condition detection interrupt (STI) request"]
    _0 = 0,
    #[doc = "1: Enable start condition detection interrupt (STI) request"]
    _1 = 1,
}
impl From<STIE_A> for bool {
    #[inline(always)]
    fn from(variant: STIE_A) -> Self {
        variant as u8 != 0
    }
}
impl STIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STIE_A {
        match self.bits {
            false => STIE_A::_0,
            true => STIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STIE_A::_1
    }
}
#[doc = "Field `STIE` writer - Start Condition Detection Interrupt Request Enable"]
pub type STIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICIER_SPEC, STIE_A, O>;
impl<'a, const O: u8> STIE_W<'a, O> {
    #[doc = "Disable start condition detection interrupt (STI) request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STIE_A::_0)
    }
    #[doc = "Enable start condition detection interrupt (STI) request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STIE_A::_1)
    }
}
#[doc = "Field `SPIE` reader - Stop Condition Detection Interrupt Request Enable"]
pub type SPIE_R = crate::BitReader<SPIE_A>;
#[doc = "Stop Condition Detection Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPIE_A {
    #[doc = "0: Disable stop condition detection interrupt (SPI) request"]
    _0 = 0,
    #[doc = "1: Enable stop condition detection interrupt (SPI) request"]
    _1 = 1,
}
impl From<SPIE_A> for bool {
    #[inline(always)]
    fn from(variant: SPIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIE_A {
        match self.bits {
            false => SPIE_A::_0,
            true => SPIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPIE_A::_1
    }
}
#[doc = "Field `SPIE` writer - Stop Condition Detection Interrupt Request Enable"]
pub type SPIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICIER_SPEC, SPIE_A, O>;
impl<'a, const O: u8> SPIE_W<'a, O> {
    #[doc = "Disable stop condition detection interrupt (SPI) request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPIE_A::_0)
    }
    #[doc = "Enable stop condition detection interrupt (SPI) request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPIE_A::_1)
    }
}
#[doc = "Field `NAKIE` reader - NACK Reception Interrupt Request Enable"]
pub type NAKIE_R = crate::BitReader<NAKIE_A>;
#[doc = "NACK Reception Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NAKIE_A {
    #[doc = "0: Disable NACK reception interrupt (NAKI) request"]
    _0 = 0,
    #[doc = "1: Enable NACK reception interrupt (NAKI) request"]
    _1 = 1,
}
impl From<NAKIE_A> for bool {
    #[inline(always)]
    fn from(variant: NAKIE_A) -> Self {
        variant as u8 != 0
    }
}
impl NAKIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NAKIE_A {
        match self.bits {
            false => NAKIE_A::_0,
            true => NAKIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NAKIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NAKIE_A::_1
    }
}
#[doc = "Field `NAKIE` writer - NACK Reception Interrupt Request Enable"]
pub type NAKIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICIER_SPEC, NAKIE_A, O>;
impl<'a, const O: u8> NAKIE_W<'a, O> {
    #[doc = "Disable NACK reception interrupt (NAKI) request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NAKIE_A::_0)
    }
    #[doc = "Enable NACK reception interrupt (NAKI) request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NAKIE_A::_1)
    }
}
#[doc = "Field `RIE` reader - Receive Data Full Interrupt Request Enable"]
pub type RIE_R = crate::BitReader<RIE_A>;
#[doc = "Receive Data Full Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RIE_A {
    #[doc = "0: Disable receive data full interrupt (IICn_RXI) request"]
    _0 = 0,
    #[doc = "1: Enable receive data full interrupt (IICn_RXI) request"]
    _1 = 1,
}
impl From<RIE_A> for bool {
    #[inline(always)]
    fn from(variant: RIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIE_A {
        match self.bits {
            false => RIE_A::_0,
            true => RIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RIE_A::_1
    }
}
#[doc = "Field `RIE` writer - Receive Data Full Interrupt Request Enable"]
pub type RIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICIER_SPEC, RIE_A, O>;
impl<'a, const O: u8> RIE_W<'a, O> {
    #[doc = "Disable receive data full interrupt (IICn_RXI) request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RIE_A::_0)
    }
    #[doc = "Enable receive data full interrupt (IICn_RXI) request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RIE_A::_1)
    }
}
#[doc = "Field `TEIE` reader - Transmit End Interrupt Request Enable"]
pub type TEIE_R = crate::BitReader<TEIE_A>;
#[doc = "Transmit End Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIE_A {
    #[doc = "0: Disable transmit end interrupt (IICn_TEI) request"]
    _0 = 0,
    #[doc = "1: Enable transmit end interrupt (IICn_TEI) request"]
    _1 = 1,
}
impl From<TEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIE_A {
        match self.bits {
            false => TEIE_A::_0,
            true => TEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEIE_A::_1
    }
}
#[doc = "Field `TEIE` writer - Transmit End Interrupt Request Enable"]
pub type TEIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICIER_SPEC, TEIE_A, O>;
impl<'a, const O: u8> TEIE_W<'a, O> {
    #[doc = "Disable transmit end interrupt (IICn_TEI) request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEIE_A::_0)
    }
    #[doc = "Enable transmit end interrupt (IICn_TEI) request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEIE_A::_1)
    }
}
#[doc = "Field `TIE` reader - Transmit Data Empty Interrupt Request Enable"]
pub type TIE_R = crate::BitReader<TIE_A>;
#[doc = "Transmit Data Empty Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIE_A {
    #[doc = "0: Disable transmit data empty interrupt (IICn_TXI) request"]
    _0 = 0,
    #[doc = "1: Enable transmit data empty interrupt (IICn_TXI) request"]
    _1 = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::_0,
            true => TIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIE_A::_1
    }
}
#[doc = "Field `TIE` writer - Transmit Data Empty Interrupt Request Enable"]
pub type TIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICIER_SPEC, TIE_A, O>;
impl<'a, const O: u8> TIE_W<'a, O> {
    #[doc = "Disable transmit data empty interrupt (IICn_TXI) request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIE_A::_0)
    }
    #[doc = "Enable transmit data empty interrupt (IICn_TXI) request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Timeout Interrupt Request Enable"]
    #[inline(always)]
    pub fn tmoie(&self) -> TMOIE_R {
        TMOIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Arbitration-Lost Interrupt Request Enable"]
    #[inline(always)]
    pub fn alie(&self) -> ALIE_R {
        ALIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start Condition Detection Interrupt Request Enable"]
    #[inline(always)]
    pub fn stie(&self) -> STIE_R {
        STIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stop Condition Detection Interrupt Request Enable"]
    #[inline(always)]
    pub fn spie(&self) -> SPIE_R {
        SPIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NACK Reception Interrupt Request Enable"]
    #[inline(always)]
    pub fn nakie(&self) -> NAKIE_R {
        NAKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Data Full Interrupt Request Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit End Interrupt Request Enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Data Empty Interrupt Request Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timeout Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmoie(&mut self) -> TMOIE_W<0> {
        TMOIE_W::new(self)
    }
    #[doc = "Bit 1 - Arbitration-Lost Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn alie(&mut self) -> ALIE_W<1> {
        ALIE_W::new(self)
    }
    #[doc = "Bit 2 - Start Condition Detection Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stie(&mut self) -> STIE_W<2> {
        STIE_W::new(self)
    }
    #[doc = "Bit 3 - Stop Condition Detection Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spie(&mut self) -> SPIE_W<3> {
        SPIE_W::new(self)
    }
    #[doc = "Bit 4 - NACK Reception Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nakie(&mut self) -> NAKIE_W<4> {
        NAKIE_W::new(self)
    }
    #[doc = "Bit 5 - Receive Data Full Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<5> {
        RIE_W::new(self)
    }
    #[doc = "Bit 6 - Transmit End Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<6> {
        TEIE_W::new(self)
    }
    #[doc = "Bit 7 - Transmit Data Empty Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<7> {
        TIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Bus Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icier](index.html) module"]
pub struct ICIER_SPEC;
impl crate::RegisterSpec for ICIER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [icier::R](R) reader structure"]
impl crate::Readable for ICIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icier::W](W) writer structure"]
impl crate::Writable for ICIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICIER to value 0"]
impl crate::Resettable for ICIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
