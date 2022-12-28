#[doc = "Register `RDR` reader"]
pub struct R(crate::R<RDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDAT` reader - Serial receive data"]
pub type RDAT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MPB` reader - Multi-processor flag"]
pub type MPB_R = crate::BitReader<MPB_A>;
#[doc = "Multi-processor flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPB_A {
    #[doc = "0: Data transmission cycles"]
    _0 = 0,
    #[doc = "1: ID transmission cycles"]
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
#[doc = "Field `DR` reader - Receive data ready flag"]
pub type DR_R = crate::BitReader<bool>;
#[doc = "Field `FPER` reader - FIFO parity error flag"]
pub type FPER_R = crate::BitReader<FPER_A>;
#[doc = "FIFO parity error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPER_A {
    #[doc = "0: There is no parity error in the data read from the receive-FIFO"]
    _0 = 0,
    #[doc = "1: There is parity error in the data read from the receive-FIFO"]
    _1 = 1,
}
impl From<FPER_A> for bool {
    #[inline(always)]
    fn from(variant: FPER_A) -> Self {
        variant as u8 != 0
    }
}
impl FPER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPER_A {
        match self.bits {
            false => FPER_A::_0,
            true => FPER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FPER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FPER_A::_1
    }
}
#[doc = "Field `FFER` reader - FIFO framing error flag"]
pub type FFER_R = crate::BitReader<FFER_A>;
#[doc = "FIFO framing error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FFER_A {
    #[doc = "0: There is no framing error in the data read from the receive-FIFO"]
    _0 = 0,
    #[doc = "1: There is framing error in the data read from the receive-FIFO"]
    _1 = 1,
}
impl From<FFER_A> for bool {
    #[inline(always)]
    fn from(variant: FFER_A) -> Self {
        variant as u8 != 0
    }
}
impl FFER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFER_A {
        match self.bits {
            false => FFER_A::_0,
            true => FFER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FFER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FFER_A::_1
    }
}
#[doc = "Field `ORER` reader - Overrun Error flag"]
pub type ORER_R = crate::BitReader<bool>;
#[doc = "Field `PER` reader - Parity error flag"]
pub type PER_R = crate::BitReader<bool>;
#[doc = "Field `FER` reader - Framing error flag"]
pub type FER_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:8 - Serial receive data"]
    #[inline(always)]
    pub fn rdat(&self) -> RDAT_R {
        RDAT_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - Multi-processor flag"]
    #[inline(always)]
    pub fn mpb(&self) -> MPB_R {
        MPB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive data ready flag"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FIFO parity error flag"]
    #[inline(always)]
    pub fn fper(&self) -> FPER_R {
        FPER_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FIFO framing error flag"]
    #[inline(always)]
    pub fn ffer(&self) -> FFER_R {
        FFER_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 24 - Overrun Error flag"]
    #[inline(always)]
    pub fn orer(&self) -> ORER_R {
        ORER_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - Parity error flag"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Framing error flag"]
    #[inline(always)]
    pub fn fer(&self) -> FER_R {
        FER_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Receive Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdr](index.html) module"]
pub struct RDR_SPEC;
impl crate::RegisterSpec for RDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdr::R](R) reader structure"]
impl crate::Readable for RDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RDR to value 0"]
impl crate::Resettable for RDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
