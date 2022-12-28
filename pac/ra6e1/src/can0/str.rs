#[doc = "Register `STR` reader"]
pub struct R(crate::R<STR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NDST` reader - NEWDATA Status Flag"]
pub type NDST_R = crate::BitReader<NDST_A>;
#[doc = "NEWDATA Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NDST_A {
    #[doc = "0: No mailbox with NEWDATA = 1"]
    _0 = 0,
    #[doc = "1: One or more mailboxes with NEWDATA = 1"]
    _1 = 1,
}
impl From<NDST_A> for bool {
    #[inline(always)]
    fn from(variant: NDST_A) -> Self {
        variant as u8 != 0
    }
}
impl NDST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NDST_A {
        match self.bits {
            false => NDST_A::_0,
            true => NDST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NDST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NDST_A::_1
    }
}
#[doc = "Field `SDST` reader - SENTDATA Status Flag"]
pub type SDST_R = crate::BitReader<SDST_A>;
#[doc = "SENTDATA Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDST_A {
    #[doc = "0: No mailbox with SENTDATA = 1"]
    _0 = 0,
    #[doc = "1: One or more mailboxes with SENTDATA = 1"]
    _1 = 1,
}
impl From<SDST_A> for bool {
    #[inline(always)]
    fn from(variant: SDST_A) -> Self {
        variant as u8 != 0
    }
}
impl SDST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDST_A {
        match self.bits {
            false => SDST_A::_0,
            true => SDST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDST_A::_1
    }
}
#[doc = "Field `RFST` reader - Receive FIFO Status Flag"]
pub type RFST_R = crate::BitReader<RFST_A>;
#[doc = "Receive FIFO Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFST_A {
    #[doc = "0: Receive FIFO empty"]
    _0 = 0,
    #[doc = "1: Message in receive FIFO"]
    _1 = 1,
}
impl From<RFST_A> for bool {
    #[inline(always)]
    fn from(variant: RFST_A) -> Self {
        variant as u8 != 0
    }
}
impl RFST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFST_A {
        match self.bits {
            false => RFST_A::_0,
            true => RFST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFST_A::_1
    }
}
#[doc = "Field `TFST` reader - Transmit FIFO Status Flag"]
pub type TFST_R = crate::BitReader<TFST_A>;
#[doc = "Transmit FIFO Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFST_A {
    #[doc = "0: Transmit FIFO is full"]
    _0 = 0,
    #[doc = "1: Transmit FIFO is not full"]
    _1 = 1,
}
impl From<TFST_A> for bool {
    #[inline(always)]
    fn from(variant: TFST_A) -> Self {
        variant as u8 != 0
    }
}
impl TFST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFST_A {
        match self.bits {
            false => TFST_A::_0,
            true => TFST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFST_A::_1
    }
}
#[doc = "Field `NMLST` reader - Normal Mailbox Message Lost Status Flag"]
pub type NMLST_R = crate::BitReader<NMLST_A>;
#[doc = "Normal Mailbox Message Lost Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NMLST_A {
    #[doc = "0: No mailbox with MSGLOST = 1"]
    _0 = 0,
    #[doc = "1: One or more mailboxes with MSGLOST = 1"]
    _1 = 1,
}
impl From<NMLST_A> for bool {
    #[inline(always)]
    fn from(variant: NMLST_A) -> Self {
        variant as u8 != 0
    }
}
impl NMLST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMLST_A {
        match self.bits {
            false => NMLST_A::_0,
            true => NMLST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NMLST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NMLST_A::_1
    }
}
#[doc = "Field `FMLST` reader - FIFO Mailbox Message Lost Status Flag"]
pub type FMLST_R = crate::BitReader<FMLST_A>;
#[doc = "FIFO Mailbox Message Lost Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMLST_A {
    #[doc = "0: RFMLF = 0"]
    _0 = 0,
    #[doc = "1: RFMLF = 1"]
    _1 = 1,
}
impl From<FMLST_A> for bool {
    #[inline(always)]
    fn from(variant: FMLST_A) -> Self {
        variant as u8 != 0
    }
}
impl FMLST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMLST_A {
        match self.bits {
            false => FMLST_A::_0,
            true => FMLST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FMLST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FMLST_A::_1
    }
}
#[doc = "Field `TABST` reader - Transmission Abort Status Flag"]
pub type TABST_R = crate::BitReader<TABST_A>;
#[doc = "Transmission Abort Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TABST_A {
    #[doc = "0: No mailbox with TRMABT = 1"]
    _0 = 0,
    #[doc = "1: One or more mailboxes with TRMABT = 1"]
    _1 = 1,
}
impl From<TABST_A> for bool {
    #[inline(always)]
    fn from(variant: TABST_A) -> Self {
        variant as u8 != 0
    }
}
impl TABST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TABST_A {
        match self.bits {
            false => TABST_A::_0,
            true => TABST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TABST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TABST_A::_1
    }
}
#[doc = "Field `EST` reader - Error Status Flag"]
pub type EST_R = crate::BitReader<EST_A>;
#[doc = "Error Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EST_A {
    #[doc = "0: No error occurred"]
    _0 = 0,
    #[doc = "1: Error occurred"]
    _1 = 1,
}
impl From<EST_A> for bool {
    #[inline(always)]
    fn from(variant: EST_A) -> Self {
        variant as u8 != 0
    }
}
impl EST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EST_A {
        match self.bits {
            false => EST_A::_0,
            true => EST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EST_A::_1
    }
}
#[doc = "Field `RSTST` reader - CAN Reset Status Flag"]
pub type RSTST_R = crate::BitReader<RSTST_A>;
#[doc = "CAN Reset Status Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTST_A {
    #[doc = "0: Not in CAN reset mode"]
    _0 = 0,
    #[doc = "1: In CAN reset mode"]
    _1 = 1,
}
impl From<RSTST_A> for bool {
    #[inline(always)]
    fn from(variant: RSTST_A) -> Self {
        variant as u8 != 0
    }
}
impl RSTST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTST_A {
        match self.bits {
            false => RSTST_A::_0,
            true => RSTST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSTST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSTST_A::_1
    }
}
#[doc = "Field `HLTST` reader - CAN Halt Status Flag"]
pub type HLTST_R = crate::BitReader<HLTST_A>;
#[doc = "CAN Halt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HLTST_A {
    #[doc = "0: Not in CAN halt mode"]
    _0 = 0,
    #[doc = "1: In CAN halt mode"]
    _1 = 1,
}
impl From<HLTST_A> for bool {
    #[inline(always)]
    fn from(variant: HLTST_A) -> Self {
        variant as u8 != 0
    }
}
impl HLTST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HLTST_A {
        match self.bits {
            false => HLTST_A::_0,
            true => HLTST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HLTST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HLTST_A::_1
    }
}
#[doc = "Field `SLPST` reader - CAN Sleep Status Flag"]
pub type SLPST_R = crate::BitReader<SLPST_A>;
#[doc = "CAN Sleep Status Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLPST_A {
    #[doc = "0: Not in CAN sleep mode"]
    _0 = 0,
    #[doc = "1: In CAN sleep mode"]
    _1 = 1,
}
impl From<SLPST_A> for bool {
    #[inline(always)]
    fn from(variant: SLPST_A) -> Self {
        variant as u8 != 0
    }
}
impl SLPST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLPST_A {
        match self.bits {
            false => SLPST_A::_0,
            true => SLPST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLPST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLPST_A::_1
    }
}
#[doc = "Field `EPST` reader - Error-Passive Status Flag"]
pub type EPST_R = crate::BitReader<EPST_A>;
#[doc = "Error-Passive Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPST_A {
    #[doc = "0: Not in error-passive state"]
    _0 = 0,
    #[doc = "1: In error-passive state"]
    _1 = 1,
}
impl From<EPST_A> for bool {
    #[inline(always)]
    fn from(variant: EPST_A) -> Self {
        variant as u8 != 0
    }
}
impl EPST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPST_A {
        match self.bits {
            false => EPST_A::_0,
            true => EPST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EPST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EPST_A::_1
    }
}
#[doc = "Field `BOST` reader - Bus-Off Status Flag"]
pub type BOST_R = crate::BitReader<BOST_A>;
#[doc = "Bus-Off Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOST_A {
    #[doc = "0: Not in bus-off state"]
    _0 = 0,
    #[doc = "1: In bus-off state"]
    _1 = 1,
}
impl From<BOST_A> for bool {
    #[inline(always)]
    fn from(variant: BOST_A) -> Self {
        variant as u8 != 0
    }
}
impl BOST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOST_A {
        match self.bits {
            false => BOST_A::_0,
            true => BOST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BOST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BOST_A::_1
    }
}
#[doc = "Field `TRMST` reader - Transmit Status Flag"]
pub type TRMST_R = crate::BitReader<TRMST_A>;
#[doc = "Transmit Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRMST_A {
    #[doc = "0: Bus idle or reception in progress"]
    _0 = 0,
    #[doc = "1: Transmission in progress or module in bus-off state"]
    _1 = 1,
}
impl From<TRMST_A> for bool {
    #[inline(always)]
    fn from(variant: TRMST_A) -> Self {
        variant as u8 != 0
    }
}
impl TRMST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRMST_A {
        match self.bits {
            false => TRMST_A::_0,
            true => TRMST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRMST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRMST_A::_1
    }
}
#[doc = "Field `RECST` reader - Receive Status Flag"]
pub type RECST_R = crate::BitReader<RECST_A>;
#[doc = "Receive Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECST_A {
    #[doc = "0: Bus idle or transmission in progress"]
    _0 = 0,
    #[doc = "1: Reception in progress"]
    _1 = 1,
}
impl From<RECST_A> for bool {
    #[inline(always)]
    fn from(variant: RECST_A) -> Self {
        variant as u8 != 0
    }
}
impl RECST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECST_A {
        match self.bits {
            false => RECST_A::_0,
            true => RECST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RECST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RECST_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - NEWDATA Status Flag"]
    #[inline(always)]
    pub fn ndst(&self) -> NDST_R {
        NDST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SENTDATA Status Flag"]
    #[inline(always)]
    pub fn sdst(&self) -> SDST_R {
        SDST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO Status Flag"]
    #[inline(always)]
    pub fn rfst(&self) -> RFST_R {
        RFST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit FIFO Status Flag"]
    #[inline(always)]
    pub fn tfst(&self) -> TFST_R {
        TFST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Normal Mailbox Message Lost Status Flag"]
    #[inline(always)]
    pub fn nmlst(&self) -> NMLST_R {
        NMLST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FIFO Mailbox Message Lost Status Flag"]
    #[inline(always)]
    pub fn fmlst(&self) -> FMLST_R {
        FMLST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission Abort Status Flag"]
    #[inline(always)]
    pub fn tabst(&self) -> TABST_R {
        TABST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error Status Flag"]
    #[inline(always)]
    pub fn est(&self) -> EST_R {
        EST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CAN Reset Status Flag"]
    #[inline(always)]
    pub fn rstst(&self) -> RSTST_R {
        RSTST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CAN Halt Status Flag"]
    #[inline(always)]
    pub fn hltst(&self) -> HLTST_R {
        HLTST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CAN Sleep Status Flag"]
    #[inline(always)]
    pub fn slpst(&self) -> SLPST_R {
        SLPST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Error-Passive Status Flag"]
    #[inline(always)]
    pub fn epst(&self) -> EPST_R {
        EPST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Bus-Off Status Flag"]
    #[inline(always)]
    pub fn bost(&self) -> BOST_R {
        BOST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit Status Flag"]
    #[inline(always)]
    pub fn trmst(&self) -> TRMST_R {
        TRMST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive Status Flag"]
    #[inline(always)]
    pub fn recst(&self) -> RECST_R {
        RECST_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [str](index.html) module"]
pub struct STR_SPEC;
impl crate::RegisterSpec for STR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [str::R](R) reader structure"]
impl crate::Readable for STR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STR to value 0x0500"]
impl crate::Resettable for STR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0500;
}
