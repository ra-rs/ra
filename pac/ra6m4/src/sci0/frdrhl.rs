#[doc = "Register `FRDRHL` reader"]
pub struct R(crate::R<FRDRHL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRDRHL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRDRHL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRDRHL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDAT` reader - Serial receive data"]
pub type RDAT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MPB` reader - Multi-Processor Bit Flag"]
pub type MPB_R = crate::BitReader<MPB_A>;
#[doc = "Multi-Processor Bit Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPB_A {
    #[doc = "0: Data transmission cycle"]
    _0 = 0,
    #[doc = "1: ID transmission cycle"]
    _1 = 1,
}
impl From<MPB_A> for bool {
    #[inline(always)]
    fn from(variant: MPB_A) -> Self {
        variant as u8 != 0
    }
}
impl MPB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPB_A {
        match self.bits {
            false => MPB_A::_0,
            true => MPB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPB_A::_1
    }
}
#[doc = "Field `DR` reader - Receive Data Ready Flag"]
pub type DR_R = crate::BitReader<DR_A>;
#[doc = "Receive Data Ready Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DR_A {
    #[doc = "0: Receiving is in progress, or no received data remains in the FRDRH and FRDRL registers after successfully completed reception"]
    _0 = 0,
    #[doc = "1: Next receive data is not received for a period after successfully completed reception"]
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
#[doc = "Field `PER` reader - Parity Error Flag"]
pub type PER_R = crate::BitReader<PER_A>;
#[doc = "Parity Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER_A {
    #[doc = "0: No parity error occurred in the first data of FRDRH and FRDRL"]
    _0 = 0,
    #[doc = "1: Parity error occurred in the first data of FRDRH and FRDRL"]
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
    #[doc = "0: No framing error occurred in the first data of FRDRH and FRDRL"]
    _0 = 0,
    #[doc = "1: Framing error occurred in the first data of FRDRH and FRDRL"]
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
#[doc = "Field `ORER` reader - Overrun Error Flag"]
pub type ORER_R = crate::BitReader<ORER_A>;
#[doc = "Overrun Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORER_A {
    #[doc = "0: No overrun error occurred"]
    _0 = 0,
    #[doc = "1: Overrun error occurred"]
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
#[doc = "Field `RDF` reader - Receive FIFO Data Full Flag"]
pub type RDF_R = crate::BitReader<RDF_A>;
#[doc = "Receive FIFO Data Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDF_A {
    #[doc = "0: The amount of receive data written in FRDRH and FRDRL is less than the specified receive triggering number"]
    _0 = 0,
    #[doc = "1: The amount of receive data written in FRDRH and FRDRL is equal to or greater than the specified receive triggering number"]
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
impl R {
    #[doc = "Bits 0:8 - Serial receive data"]
    #[inline(always)]
    pub fn rdat(&self) -> RDAT_R {
        RDAT_R::new(self.bits & 0x01ff)
    }
    #[doc = "Bit 9 - Multi-Processor Bit Flag"]
    #[inline(always)]
    pub fn mpb(&self) -> MPB_R {
        MPB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive Data Ready Flag"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Parity Error Flag"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Framing Error Flag"]
    #[inline(always)]
    pub fn fer(&self) -> FER_R {
        FER_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Overrun Error Flag"]
    #[inline(always)]
    pub fn orer(&self) -> ORER_R {
        ORER_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive FIFO Data Full Flag"]
    #[inline(always)]
    pub fn rdf(&self) -> RDF_R {
        RDF_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "Receive FIFO Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frdrhl](index.html) module"]
pub struct FRDRHL_SPEC;
impl crate::RegisterSpec for FRDRHL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [frdrhl::R](R) reader structure"]
impl crate::Readable for FRDRHL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FRDRHL to value 0"]
impl crate::Resettable for FRDRHL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
