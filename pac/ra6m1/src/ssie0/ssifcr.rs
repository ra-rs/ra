#[doc = "Register `SSIFCR` reader"]
pub struct R(crate::R<SSIFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSIFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSIFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSIFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSIFCR` writer"]
pub struct W(crate::W<SSIFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSIFCR_SPEC>;
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
impl From<crate::W<SSIFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSIFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFRST` reader - Receive FIFO Data Register Reset"]
pub type RFRST_R = crate::BitReader<RFRST_A>;
#[doc = "Receive FIFO Data Register Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFRST_A {
    #[doc = "0: Clears the receive data FIFO reset."]
    _0 = 0,
    #[doc = "1: Initiates the receive data FIFO reset."]
    _1 = 1,
}
impl From<RFRST_A> for bool {
    #[inline(always)]
    fn from(variant: RFRST_A) -> Self {
        variant as u8 != 0
    }
}
impl RFRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFRST_A {
        match self.bits {
            false => RFRST_A::_0,
            true => RFRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFRST_A::_1
    }
}
#[doc = "Field `RFRST` writer - Receive FIFO Data Register Reset"]
pub type RFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSIFCR_SPEC, RFRST_A, O>;
impl<'a, const O: u8> RFRST_W<'a, O> {
    #[doc = "Clears the receive data FIFO reset."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFRST_A::_0)
    }
    #[doc = "Initiates the receive data FIFO reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFRST_A::_1)
    }
}
#[doc = "Field `TFRST` reader - Transmit FIFO Data Register Reset"]
pub type TFRST_R = crate::BitReader<TFRST_A>;
#[doc = "Transmit FIFO Data Register Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFRST_A {
    #[doc = "0: Clears the transmit data FIFO reset."]
    _0 = 0,
    #[doc = "1: Initiates the transmit data FIFO reset."]
    _1 = 1,
}
impl From<TFRST_A> for bool {
    #[inline(always)]
    fn from(variant: TFRST_A) -> Self {
        variant as u8 != 0
    }
}
impl TFRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFRST_A {
        match self.bits {
            false => TFRST_A::_0,
            true => TFRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFRST_A::_1
    }
}
#[doc = "Field `TFRST` writer - Transmit FIFO Data Register Reset"]
pub type TFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSIFCR_SPEC, TFRST_A, O>;
impl<'a, const O: u8> TFRST_W<'a, O> {
    #[doc = "Clears the transmit data FIFO reset."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFRST_A::_0)
    }
    #[doc = "Initiates the transmit data FIFO reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFRST_A::_1)
    }
}
#[doc = "Field `RIE` reader - Receive Interrupt Enable NOTE: RXI can be cleared by clearing either the RDF flag (see the description of the RDF bit for details) or RIE bit."]
pub type RIE_R = crate::BitReader<RIE_A>;
#[doc = "Receive Interrupt Enable NOTE: RXI can be cleared by clearing either the RDF flag (see the description of the RDF bit for details) or RIE bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RIE_A {
    #[doc = "0: Receive data full interrupt (RXI) request is disabled"]
    _0 = 0,
    #[doc = "1: Receive data full interrupt (RXI) request is enabled"]
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
#[doc = "Field `RIE` writer - Receive Interrupt Enable NOTE: RXI can be cleared by clearing either the RDF flag (see the description of the RDF bit for details) or RIE bit."]
pub type RIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSIFCR_SPEC, RIE_A, O>;
impl<'a, const O: u8> RIE_W<'a, O> {
    #[doc = "Receive data full interrupt (RXI) request is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RIE_A::_0)
    }
    #[doc = "Receive data full interrupt (RXI) request is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RIE_A::_1)
    }
}
#[doc = "Field `TIE` reader - Transmit Interrupt Enable NOTE: TXI can be cleared by clearing either the TDE flag (see the description of the TDE bit for details) or TIE bit."]
pub type TIE_R = crate::BitReader<TIE_A>;
#[doc = "Transmit Interrupt Enable NOTE: TXI can be cleared by clearing either the TDE flag (see the description of the TDE bit for details) or TIE bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIE_A {
    #[doc = "0: Transmit data empty interrupt (TXI) request is disabled"]
    _0 = 0,
    #[doc = "1: Transmit data empty interrupt (TXI) request is enabled"]
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
#[doc = "Field `TIE` writer - Transmit Interrupt Enable NOTE: TXI can be cleared by clearing either the TDE flag (see the description of the TDE bit for details) or TIE bit."]
pub type TIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSIFCR_SPEC, TIE_A, O>;
impl<'a, const O: u8> TIE_W<'a, O> {
    #[doc = "Transmit data empty interrupt (TXI) request is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIE_A::_0)
    }
    #[doc = "Transmit data empty interrupt (TXI) request is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIE_A::_1)
    }
}
#[doc = "Field `RTRG` reader - Receive Data Trigger Number"]
pub type RTRG_R = crate::FieldReader<u8, RTRG_A>;
#[doc = "Receive Data Trigger Number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTRG_A {
    #[doc = "0: 1"]
    _00 = 0,
    #[doc = "1: 2"]
    _01 = 1,
    #[doc = "2: 4"]
    _10 = 2,
    #[doc = "3: 6"]
    _11 = 3,
}
impl From<RTRG_A> for u8 {
    #[inline(always)]
    fn from(variant: RTRG_A) -> Self {
        variant as _
    }
}
impl RTRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTRG_A {
        match self.bits {
            0 => RTRG_A::_00,
            1 => RTRG_A::_01,
            2 => RTRG_A::_10,
            3 => RTRG_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RTRG_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RTRG_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RTRG_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == RTRG_A::_11
    }
}
#[doc = "Field `RTRG` writer - Receive Data Trigger Number"]
pub type RTRG_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SSIFCR_SPEC, u8, RTRG_A, 2, O>;
impl<'a, const O: u8> RTRG_W<'a, O> {
    #[doc = "1"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RTRG_A::_00)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RTRG_A::_01)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RTRG_A::_10)
    }
    #[doc = "6"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(RTRG_A::_11)
    }
}
#[doc = "Field `TTRG` reader - Transmit Data Trigger Number NOTE: The values in parenthesis are the number of empty stages in SSIFTDR at which the TDE flag is set."]
pub type TTRG_R = crate::FieldReader<u8, TTRG_A>;
#[doc = "Transmit Data Trigger Number NOTE: The values in parenthesis are the number of empty stages in SSIFTDR at which the TDE flag is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TTRG_A {
    #[doc = "0: 7 (1)"]
    _00 = 0,
    #[doc = "1: 6 (2)"]
    _01 = 1,
    #[doc = "2: 4 (4)"]
    _10 = 2,
    #[doc = "3: 2 (6)"]
    _11 = 3,
}
impl From<TTRG_A> for u8 {
    #[inline(always)]
    fn from(variant: TTRG_A) -> Self {
        variant as _
    }
}
impl TTRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TTRG_A {
        match self.bits {
            0 => TTRG_A::_00,
            1 => TTRG_A::_01,
            2 => TTRG_A::_10,
            3 => TTRG_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TTRG_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TTRG_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TTRG_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TTRG_A::_11
    }
}
#[doc = "Field `TTRG` writer - Transmit Data Trigger Number NOTE: The values in parenthesis are the number of empty stages in SSIFTDR at which the TDE flag is set."]
pub type TTRG_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SSIFCR_SPEC, u8, TTRG_A, 2, O>;
impl<'a, const O: u8> TTRG_W<'a, O> {
    #[doc = "7 (1)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TTRG_A::_00)
    }
    #[doc = "6 (2)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TTRG_A::_01)
    }
    #[doc = "4 (4)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TTRG_A::_10)
    }
    #[doc = "2 (6)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TTRG_A::_11)
    }
}
#[doc = "Field `SSIRST` reader - SSI soft ware reset"]
pub type SSIRST_R = crate::BitReader<SSIRST_A>;
#[doc = "SSI soft ware reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSIRST_A {
    #[doc = "0: Clears the SSI software reset."]
    _0 = 0,
    #[doc = "1: initiates the SSI software reset."]
    _1 = 1,
}
impl From<SSIRST_A> for bool {
    #[inline(always)]
    fn from(variant: SSIRST_A) -> Self {
        variant as u8 != 0
    }
}
impl SSIRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSIRST_A {
        match self.bits {
            false => SSIRST_A::_0,
            true => SSIRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSIRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSIRST_A::_1
    }
}
#[doc = "Field `SSIRST` writer - SSI soft ware reset"]
pub type SSIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSIFCR_SPEC, SSIRST_A, O>;
impl<'a, const O: u8> SSIRST_W<'a, O> {
    #[doc = "Clears the SSI software reset."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSIRST_A::_0)
    }
    #[doc = "initiates the SSI software reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSIRST_A::_1)
    }
}
#[doc = "Field `AUCKE` reader - Oversampling Clock Enable"]
pub type AUCKE_R = crate::BitReader<AUCKE_A>;
#[doc = "Oversampling Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUCKE_A {
    #[doc = "0: The oversampling clock is disabled."]
    _0 = 0,
    #[doc = "1: The oversampling clock is enabled."]
    _1 = 1,
}
impl From<AUCKE_A> for bool {
    #[inline(always)]
    fn from(variant: AUCKE_A) -> Self {
        variant as u8 != 0
    }
}
impl AUCKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUCKE_A {
        match self.bits {
            false => AUCKE_A::_0,
            true => AUCKE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AUCKE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AUCKE_A::_1
    }
}
#[doc = "Field `AUCKE` writer - Oversampling Clock Enable"]
pub type AUCKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSIFCR_SPEC, AUCKE_A, O>;
impl<'a, const O: u8> AUCKE_W<'a, O> {
    #[doc = "The oversampling clock is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AUCKE_A::_0)
    }
    #[doc = "The oversampling clock is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AUCKE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Receive FIFO Data Register Reset"]
    #[inline(always)]
    pub fn rfrst(&self) -> RFRST_R {
        RFRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Data Register Reset"]
    #[inline(always)]
    pub fn tfrst(&self) -> TFRST_R {
        TFRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Interrupt Enable NOTE: RXI can be cleared by clearing either the RDF flag (see the description of the RDF bit for details) or RIE bit."]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Interrupt Enable NOTE: TXI can be cleared by clearing either the TDE flag (see the description of the TDE bit for details) or TIE bit."]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Receive Data Trigger Number"]
    #[inline(always)]
    pub fn rtrg(&self) -> RTRG_R {
        RTRG_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Transmit Data Trigger Number NOTE: The values in parenthesis are the number of empty stages in SSIFTDR at which the TDE flag is set."]
    #[inline(always)]
    pub fn ttrg(&self) -> TTRG_R {
        TTRG_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 16 - SSI soft ware reset"]
    #[inline(always)]
    pub fn ssirst(&self) -> SSIRST_R {
        SSIRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - Oversampling Clock Enable"]
    #[inline(always)]
    pub fn aucke(&self) -> AUCKE_R {
        AUCKE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive FIFO Data Register Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rfrst(&mut self) -> RFRST_W<0> {
        RFRST_W::new(self)
    }
    #[doc = "Bit 1 - Transmit FIFO Data Register Reset"]
    #[inline(always)]
    #[must_use]
    pub fn tfrst(&mut self) -> TFRST_W<1> {
        TFRST_W::new(self)
    }
    #[doc = "Bit 2 - Receive Interrupt Enable NOTE: RXI can be cleared by clearing either the RDF flag (see the description of the RDF bit for details) or RIE bit."]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<2> {
        RIE_W::new(self)
    }
    #[doc = "Bit 3 - Transmit Interrupt Enable NOTE: TXI can be cleared by clearing either the TDE flag (see the description of the TDE bit for details) or TIE bit."]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<3> {
        TIE_W::new(self)
    }
    #[doc = "Bits 4:5 - Receive Data Trigger Number"]
    #[inline(always)]
    #[must_use]
    pub fn rtrg(&mut self) -> RTRG_W<4> {
        RTRG_W::new(self)
    }
    #[doc = "Bits 6:7 - Transmit Data Trigger Number NOTE: The values in parenthesis are the number of empty stages in SSIFTDR at which the TDE flag is set."]
    #[inline(always)]
    #[must_use]
    pub fn ttrg(&mut self) -> TTRG_W<6> {
        TTRG_W::new(self)
    }
    #[doc = "Bit 16 - SSI soft ware reset"]
    #[inline(always)]
    #[must_use]
    pub fn ssirst(&mut self) -> SSIRST_W<16> {
        SSIRST_W::new(self)
    }
    #[doc = "Bit 31 - Oversampling Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aucke(&mut self) -> AUCKE_W<31> {
        AUCKE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssifcr](index.html) module"]
pub struct SSIFCR_SPEC;
impl crate::RegisterSpec for SSIFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssifcr::R](R) reader structure"]
impl crate::Readable for SSIFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssifcr::W](W) writer structure"]
impl crate::Writable for SSIFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSIFCR to value 0"]
impl crate::Resettable for SSIFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
