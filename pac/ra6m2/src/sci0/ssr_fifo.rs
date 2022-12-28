#[doc = "Register `SSR_FIFO` reader"]
pub struct R(crate::R<SSR_FIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSR_FIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSR_FIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSR_FIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSR_FIFO` writer"]
pub struct W(crate::W<SSR_FIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSR_FIFO_SPEC>;
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
impl From<crate::W<SSR_FIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSR_FIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DR` reader - Receive Data Ready flag(Valid only in asynchronous mode(including multi-processor) and FIFO selected)\n\nThe field is **modified** in some way after a read operation."]
pub type DR_R = crate::BitReader<DR_A>;
#[doc = "Receive Data Ready flag(Valid only in asynchronous mode(including multi-processor) and FIFO selected)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DR_A {
    #[doc = "0: Receiving is in progress, or no received data has remained in FRDR after normally completed receiving.(receive FIFO is empty)"]
    _0 = 0,
    #[doc = "1: Next receive data has not been received for a period after normal completed receiving, , when data is stored in FIFO to equal or less than receive triggering number."]
    _1 = 1,
}
impl From<DR_A> for bool {
    #[inline(always)]
    fn from(variant: DR_A) -> Self {
        variant as u8 != 0
    }
}
impl DR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DR_A {
        match self.bits {
            false => DR_A::_0,
            true => DR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DR_A::_1
    }
}
#[doc = "Field `DR` writer - Receive Data Ready flag(Valid only in asynchronous mode(including multi-processor) and FIFO selected)"]
pub type DR_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, SSR_FIFO_SPEC, DR_A, O>;
impl<'a, const O: u8> DR_W<'a, O> {
    #[doc = "Receiving is in progress, or no received data has remained in FRDR after normally completed receiving.(receive FIFO is empty)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DR_A::_0)
    }
    #[doc = "Next receive data has not been received for a period after normal completed receiving, , when data is stored in FIFO to equal or less than receive triggering number."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DR_A::_1)
    }
}
#[doc = "Field `TEND` reader - Transmit End Flag\n\nThe field is **modified** in some way after a read operation."]
pub type TEND_R = crate::BitReader<TEND_A>;
#[doc = "Transmit End Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEND_A {
    #[doc = "0: A character is being transmitted."]
    _0 = 0,
    #[doc = "1: Character transfer has been completed."]
    _1 = 1,
}
impl From<TEND_A> for bool {
    #[inline(always)]
    fn from(variant: TEND_A) -> Self {
        variant as u8 != 0
    }
}
impl TEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEND_A {
        match self.bits {
            false => TEND_A::_0,
            true => TEND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEND_A::_1
    }
}
#[doc = "Field `TEND` writer - Transmit End Flag"]
pub type TEND_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, SSR_FIFO_SPEC, TEND_A, O>;
impl<'a, const O: u8> TEND_W<'a, O> {
    #[doc = "A character is being transmitted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEND_A::_0)
    }
    #[doc = "Character transfer has been completed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEND_A::_1)
    }
}
#[doc = "Field `PER` reader - Parity Error Flag\n\nThe field is **modified** in some way after a read operation."]
pub type PER_R = crate::BitReader<PER_A>;
#[doc = "Parity Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER_A {
    #[doc = "0: No parity error occurred."]
    _0 = 0,
    #[doc = "1: A parity error has occurred."]
    _1 = 1,
}
impl From<PER_A> for bool {
    #[inline(always)]
    fn from(variant: PER_A) -> Self {
        variant as u8 != 0
    }
}
impl PER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PER_A {
        match self.bits {
            false => PER_A::_0,
            true => PER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PER_A::_1
    }
}
#[doc = "Field `PER` writer - Parity Error Flag"]
pub type PER_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, SSR_FIFO_SPEC, PER_A, O>;
impl<'a, const O: u8> PER_W<'a, O> {
    #[doc = "No parity error occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PER_A::_0)
    }
    #[doc = "A parity error has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PER_A::_1)
    }
}
#[doc = "Field `FER` reader - Framing Error Flag\n\nThe field is **modified** in some way after a read operation."]
pub type FER_R = crate::BitReader<FER_A>;
#[doc = "Framing Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FER_A {
    #[doc = "0: No framing error occurred."]
    _0 = 0,
    #[doc = "1: A framing error has occurred."]
    _1 = 1,
}
impl From<FER_A> for bool {
    #[inline(always)]
    fn from(variant: FER_A) -> Self {
        variant as u8 != 0
    }
}
impl FER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FER_A {
        match self.bits {
            false => FER_A::_0,
            true => FER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FER_A::_1
    }
}
#[doc = "Field `FER` writer - Framing Error Flag"]
pub type FER_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, SSR_FIFO_SPEC, FER_A, O>;
impl<'a, const O: u8> FER_W<'a, O> {
    #[doc = "No framing error occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FER_A::_0)
    }
    #[doc = "A framing error has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FER_A::_1)
    }
}
#[doc = "Field `ORER` reader - Overrun Error Flag\n\nThe field is **modified** in some way after a read operation."]
pub type ORER_R = crate::BitReader<ORER_A>;
#[doc = "Overrun Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORER_A {
    #[doc = "0: No overrun error occurred"]
    _0 = 0,
    #[doc = "1: An overrun error has occurred"]
    _1 = 1,
}
impl From<ORER_A> for bool {
    #[inline(always)]
    fn from(variant: ORER_A) -> Self {
        variant as u8 != 0
    }
}
impl ORER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ORER_A {
        match self.bits {
            false => ORER_A::_0,
            true => ORER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ORER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ORER_A::_1
    }
}
#[doc = "Field `ORER` writer - Overrun Error Flag"]
pub type ORER_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, SSR_FIFO_SPEC, ORER_A, O>;
impl<'a, const O: u8> ORER_W<'a, O> {
    #[doc = "No overrun error occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ORER_A::_0)
    }
    #[doc = "An overrun error has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ORER_A::_1)
    }
}
#[doc = "Field `RDF` reader - Receive FIFO data full flag\n\nThe field is **modified** in some way after a read operation."]
pub type RDF_R = crate::BitReader<RDF_A>;
#[doc = "Receive FIFO data full flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDF_A {
    #[doc = "0: The quantity of receive data written in FRDR falls below the specified receive triggering number."]
    _0 = 0,
    #[doc = "1: The quantity of receive data written in FRDR is equal to or greater than the specified receive triggering number."]
    _1 = 1,
}
impl From<RDF_A> for bool {
    #[inline(always)]
    fn from(variant: RDF_A) -> Self {
        variant as u8 != 0
    }
}
impl RDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDF_A {
        match self.bits {
            false => RDF_A::_0,
            true => RDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDF_A::_1
    }
}
#[doc = "Field `RDF` writer - Receive FIFO data full flag"]
pub type RDF_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, SSR_FIFO_SPEC, RDF_A, O>;
impl<'a, const O: u8> RDF_W<'a, O> {
    #[doc = "The quantity of receive data written in FRDR falls below the specified receive triggering number."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDF_A::_0)
    }
    #[doc = "The quantity of receive data written in FRDR is equal to or greater than the specified receive triggering number."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDF_A::_1)
    }
}
#[doc = "Field `TDFE` reader - Transmit FIFO data empty flag\n\nThe field is **modified** in some way after a read operation."]
pub type TDFE_R = crate::BitReader<TDFE_A>;
#[doc = "Transmit FIFO data empty flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDFE_A {
    #[doc = "0: The quantity of transmit data written in FTDR exceeds the specified transmit triggering number."]
    _0 = 0,
    #[doc = "1: The quantity of transmit data written in FTDR is equal to or less than the specified transmit triggering number"]
    _1 = 1,
}
impl From<TDFE_A> for bool {
    #[inline(always)]
    fn from(variant: TDFE_A) -> Self {
        variant as u8 != 0
    }
}
impl TDFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDFE_A {
        match self.bits {
            false => TDFE_A::_0,
            true => TDFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDFE_A::_1
    }
}
#[doc = "Field `TDFE` writer - Transmit FIFO data empty flag"]
pub type TDFE_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, SSR_FIFO_SPEC, TDFE_A, O>;
impl<'a, const O: u8> TDFE_W<'a, O> {
    #[doc = "The quantity of transmit data written in FTDR exceeds the specified transmit triggering number."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDFE_A::_0)
    }
    #[doc = "The quantity of transmit data written in FTDR is equal to or less than the specified transmit triggering number"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDFE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Receive Data Ready flag(Valid only in asynchronous mode(including multi-processor) and FIFO selected)"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit End Flag"]
    #[inline(always)]
    pub fn tend(&self) -> TEND_R {
        TEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity Error Flag"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Framing Error Flag"]
    #[inline(always)]
    pub fn fer(&self) -> FER_R {
        FER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error Flag"]
    #[inline(always)]
    pub fn orer(&self) -> ORER_R {
        ORER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO data full flag"]
    #[inline(always)]
    pub fn rdf(&self) -> RDF_R {
        RDF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO data empty flag"]
    #[inline(always)]
    pub fn tdfe(&self) -> TDFE_R {
        TDFE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Data Ready flag(Valid only in asynchronous mode(including multi-processor) and FIFO selected)"]
    #[inline(always)]
    #[must_use]
    pub fn dr(&mut self) -> DR_W<0> {
        DR_W::new(self)
    }
    #[doc = "Bit 2 - Transmit End Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tend(&mut self) -> TEND_W<2> {
        TEND_W::new(self)
    }
    #[doc = "Bit 3 - Parity Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<3> {
        PER_W::new(self)
    }
    #[doc = "Bit 4 - Framing Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fer(&mut self) -> FER_W<4> {
        FER_W::new(self)
    }
    #[doc = "Bit 5 - Overrun Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn orer(&mut self) -> ORER_W<5> {
        ORER_W::new(self)
    }
    #[doc = "Bit 6 - Receive FIFO data full flag"]
    #[inline(always)]
    #[must_use]
    pub fn rdf(&mut self) -> RDF_W<6> {
        RDF_W::new(self)
    }
    #[doc = "Bit 7 - Transmit FIFO data empty flag"]
    #[inline(always)]
    #[must_use]
    pub fn tdfe(&mut self) -> TDFE_W<7> {
        TDFE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Serial Status Register(SCMR.SMIF = 0 and FCR.FM=1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssr_fifo](index.html) module"]
pub struct SSR_FIFO_SPEC;
impl crate::RegisterSpec for SSR_FIFO_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ssr_fifo::R](R) reader structure"]
impl crate::Readable for SSR_FIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssr_fifo::W](W) writer structure"]
impl crate::Writable for SSR_FIFO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xfd;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSR_FIFO to value 0x80"]
impl crate::Resettable for SSR_FIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
