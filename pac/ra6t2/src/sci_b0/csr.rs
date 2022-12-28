#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ERS` reader - Error Signal Status Flag"]
pub type ERS_R = crate::BitReader<ERS_A>;
#[doc = "Error Signal Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERS_A {
    #[doc = "0: Error signal Low not responded"]
    _0 = 0,
    #[doc = "1: Error signal Low responded"]
    _1 = 1,
}
impl From<ERS_A> for bool {
    #[inline(always)]
    fn from(variant: ERS_A) -> Self {
        variant as u8 != 0
    }
}
impl ERS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERS_A {
        match self.bits {
            false => ERS_A::_0,
            true => ERS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERS_A::_1
    }
}
#[doc = "Field `RXDMON` reader - Serial input data monitor bit"]
pub type RXDMON_R = crate::BitReader<RXDMON_A>;
#[doc = "Serial input data monitor bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDMON_A {
    #[doc = "0: When RINV is 0, RXDn pin is the Low level. When RINV is 1, RXDn pin is the High level."]
    _0 = 0,
    #[doc = "1: When RINV is 0, RXDn pin is the High level. When RINV is 1, RXDn pin is the Low level."]
    _1 = 1,
}
impl From<RXDMON_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMON_A) -> Self {
        variant as u8 != 0
    }
}
impl RXDMON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDMON_A {
        match self.bits {
            false => RXDMON_A::_0,
            true => RXDMON_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXDMON_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXDMON_A::_1
    }
}
#[doc = "Field `DCMF` reader - Data Compare Match Flag"]
pub type DCMF_R = crate::BitReader<DCMF_A>;
#[doc = "Data Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMF_A {
    #[doc = "0: No matched"]
    _0 = 0,
    #[doc = "1: Matched"]
    _1 = 1,
}
impl From<DCMF_A> for bool {
    #[inline(always)]
    fn from(variant: DCMF_A) -> Self {
        variant as u8 != 0
    }
}
impl DCMF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCMF_A {
        match self.bits {
            false => DCMF_A::_0,
            true => DCMF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCMF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCMF_A::_1
    }
}
#[doc = "Field `DPER` reader - Data Compare Match Parity Error Flag"]
pub type DPER_R = crate::BitReader<DPER_A>;
#[doc = "Data Compare Match Parity Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPER_A {
    #[doc = "0: No parity error occurred at address match detection"]
    _0 = 0,
    #[doc = "1: A parity error has occurred at address match detection"]
    _1 = 1,
}
impl From<DPER_A> for bool {
    #[inline(always)]
    fn from(variant: DPER_A) -> Self {
        variant as u8 != 0
    }
}
impl DPER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPER_A {
        match self.bits {
            false => DPER_A::_0,
            true => DPER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPER_A::_1
    }
}
#[doc = "Field `DFER` reader - Data Compare Match Framing Error Flag"]
pub type DFER_R = crate::BitReader<DFER_A>;
#[doc = "Data Compare Match Framing Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFER_A {
    #[doc = "0: No framing error occurred at address match detection"]
    _0 = 0,
    #[doc = "1: A framing error has occurred at address match detection"]
    _1 = 1,
}
impl From<DFER_A> for bool {
    #[inline(always)]
    fn from(variant: DFER_A) -> Self {
        variant as u8 != 0
    }
}
impl DFER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFER_A {
        match self.bits {
            false => DFER_A::_0,
            true => DFER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFER_A::_1
    }
}
#[doc = "Field `ORER` reader - Overrun Error Flag"]
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
#[doc = "Field `MFF` reader - Mode Fault Flag"]
pub type MFF_R = crate::BitReader<MFF_A>;
#[doc = "Mode Fault Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MFF_A {
    #[doc = "0: No mode fault error"]
    _0 = 0,
    #[doc = "1: Mode fault error"]
    _1 = 1,
}
impl From<MFF_A> for bool {
    #[inline(always)]
    fn from(variant: MFF_A) -> Self {
        variant as u8 != 0
    }
}
impl MFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MFF_A {
        match self.bits {
            false => MFF_A::_0,
            true => MFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MFF_A::_1
    }
}
#[doc = "Field `PER` reader - Parity Error Flag"]
pub type PER_R = crate::BitReader<PER_A>;
#[doc = "Parity Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER_A {
    #[doc = "0: Non-FIFO selected (CCR3.FM = 0): No parity error occurred FIFO selected (CCR3.FM = 1): No parity error in all received data in receive-FIFO"]
    _0 = 0,
    #[doc = "1: Non-FIFO selected (CCR3.FM = 0): A parity error has occurred FIFO selected (CCR3.FM = 1): One or more parity errors occurred in received data in receive-FIFO"]
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
#[doc = "Field `FER` reader - Framing Error Flag"]
pub type FER_R = crate::BitReader<FER_A>;
#[doc = "Framing Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FER_A {
    #[doc = "0: Non-FIFO selected (CCR3.FM = 0): No framing error occurred FIFO selected (CCR3.FM = 1): No framing error in all received data in receive-FIFO"]
    _0 = 0,
    #[doc = "1: Non-FIFO selected (CCR3.FM = 0): A framing error has occurred FIFO selected (CCR3.FM = 1): One or more framing errors occurred in received data in receive-FIFO"]
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
#[doc = "Field `TDRE` reader - Transmit Data Empty Flag"]
pub type TDRE_R = crate::BitReader<TDRE_A>;
#[doc = "Transmit Data Empty Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDRE_A {
    #[doc = "0: Non-FIFO selected (CCR3.FM = 0): Transmit data is in TDR register FIFO selected (CCR3.FM = 1): The quantity of transmit data written in transmit-FIFO exceeds the specified transmit triggering number."]
    _0 = 0,
    #[doc = "1: Non-FIFO selected (CCR3.FM = 0): No transmit data is in TDR register FIFO selected (CCR3.FM = 1): The quantity of transmit data written in transmit-FIFO is equal to or less than the specified transmit triggering number."]
    _1 = 1,
}
impl From<TDRE_A> for bool {
    #[inline(always)]
    fn from(variant: TDRE_A) -> Self {
        variant as u8 != 0
    }
}
impl TDRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDRE_A {
        match self.bits {
            false => TDRE_A::_0,
            true => TDRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDRE_A::_1
    }
}
#[doc = "Field `TEND` reader - Transmit End Flag"]
pub type TEND_R = crate::BitReader<TEND_A>;
#[doc = "Transmit End Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEND_A {
    #[doc = "0: A character is being transmitted or standing by for transmission."]
    _0 = 0,
    #[doc = "1: Character transfer has been completed, or sending Break Field."]
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
#[doc = "Field `RDRF` reader - Receive Data Full Flag"]
pub type RDRF_R = crate::BitReader<RDRF_A>;
#[doc = "Receive Data Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDRF_A {
    #[doc = "0: Non-FIFO selected (CCR3.FM = 0): No received data is in RDR register FIFO selected (CCR3.FM = 1): The quantity of receive data written in receive-FIFO falls below the specified receive triggering number."]
    _0 = 0,
    #[doc = "1: Non-FIFO selected (CCR3.FM = 0): Received data is in RDR register FIFO selected (CCR3.FM = 1): The quantity of receive data written in receive-FIFO is equal to or greater than the specified receive triggering number."]
    _1 = 1,
}
impl From<RDRF_A> for bool {
    #[inline(always)]
    fn from(variant: RDRF_A) -> Self {
        variant as u8 != 0
    }
}
impl RDRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDRF_A {
        match self.bits {
            false => RDRF_A::_0,
            true => RDRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDRF_A::_1
    }
}
impl R {
    #[doc = "Bit 4 - Error Signal Status Flag"]
    #[inline(always)]
    pub fn ers(&self) -> ERS_R {
        ERS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 15 - Serial input data monitor bit"]
    #[inline(always)]
    pub fn rxdmon(&self) -> RXDMON_R {
        RXDMON_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Data Compare Match Flag"]
    #[inline(always)]
    pub fn dcmf(&self) -> DCMF_R {
        DCMF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Data Compare Match Parity Error Flag"]
    #[inline(always)]
    pub fn dper(&self) -> DPER_R {
        DPER_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Data Compare Match Framing Error Flag"]
    #[inline(always)]
    pub fn dfer(&self) -> DFER_R {
        DFER_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Overrun Error Flag"]
    #[inline(always)]
    pub fn orer(&self) -> ORER_R {
        ORER_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Mode Fault Flag"]
    #[inline(always)]
    pub fn mff(&self) -> MFF_R {
        MFF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Parity Error Flag"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Framing Error Flag"]
    #[inline(always)]
    pub fn fer(&self) -> FER_R {
        FER_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Transmit Data Empty Flag"]
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Transmit End Flag"]
    #[inline(always)]
    pub fn tend(&self) -> TEND_R {
        TEND_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Receive Data Full Flag"]
    #[inline(always)]
    pub fn rdrf(&self) -> RDRF_R {
        RDRF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Common Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSR to value 0x6000_8000"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x6000_8000;
}
