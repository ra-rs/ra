#[doc = "Register `SYRFL1R` reader"]
pub struct R(crate::R<SYRFL1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYRFL1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYRFL1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYRFL1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYRFL1R` writer"]
pub struct W(crate::W<SYRFL1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYRFL1R_SPEC>;
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
impl From<crate::W<SYRFL1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYRFL1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANCE0` reader - Announce Message Processing"]
pub type ANCE0_R = crate::BitReader<ANCE0_A>;
#[doc = "Announce Message Processing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANCE0_A {
    #[doc = "0: Messages are not transferred to the PTPEDMAC."]
    _0 = 0,
    #[doc = "1: Messages are transferred to the PTPEDMAC."]
    _1 = 1,
}
impl From<ANCE0_A> for bool {
    #[inline(always)]
    fn from(variant: ANCE0_A) -> Self {
        variant as u8 != 0
    }
}
impl ANCE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANCE0_A {
        match self.bits {
            false => ANCE0_A::_0,
            true => ANCE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANCE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANCE0_A::_1
    }
}
#[doc = "Field `ANCE0` writer - Announce Message Processing"]
pub type ANCE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYRFL1R_SPEC, ANCE0_A, O>;
impl<'a, const O: u8> ANCE0_W<'a, O> {
    #[doc = "Messages are not transferred to the PTPEDMAC."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANCE0_A::_0)
    }
    #[doc = "Messages are transferred to the PTPEDMAC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANCE0_A::_1)
    }
}
#[doc = "Field `SYNC0` reader - Sync Message Processing"]
pub type SYNC0_R = crate::BitReader<SYNC0_A>;
#[doc = "Sync Message Processing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNC0_A {
    #[doc = "0: Messages are not transferred to the PTPEDMAC."]
    _0 = 0,
    #[doc = "1: Messages are transferred to the PTPEDMAC."]
    _1 = 1,
}
impl From<SYNC0_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC0_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC0_A {
        match self.bits {
            false => SYNC0_A::_0,
            true => SYNC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNC0_A::_1
    }
}
#[doc = "Field `SYNC0` writer - Sync Message Processing"]
pub type SYNC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYRFL1R_SPEC, SYNC0_A, O>;
impl<'a, const O: u8> SYNC0_W<'a, O> {
    #[doc = "Messages are not transferred to the PTPEDMAC."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNC0_A::_0)
    }
    #[doc = "Messages are transferred to the PTPEDMAC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNC0_A::_1)
    }
}
#[doc = "Field `SYNC2` reader - Sync Message Processing"]
pub type SYNC2_R = crate::BitReader<SYNC2_A>;
#[doc = "Sync Message Processing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNC2_A {
    #[doc = "0: The SYNFP does not process messages."]
    _0 = 0,
    #[doc = "1: The SYNFP processes messages."]
    _1 = 1,
}
impl From<SYNC2_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC2_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNC2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC2_A {
        match self.bits {
            false => SYNC2_A::_0,
            true => SYNC2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNC2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNC2_A::_1
    }
}
#[doc = "Field `SYNC2` writer - Sync Message Processing"]
pub type SYNC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYRFL1R_SPEC, SYNC2_A, O>;
impl<'a, const O: u8> SYNC2_W<'a, O> {
    #[doc = "The SYNFP does not process messages."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNC2_A::_0)
    }
    #[doc = "The SYNFP processes messages."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNC2_A::_1)
    }
}
#[doc = "Field `FUP0` reader - Follow_Up Message Processing"]
pub type FUP0_R = crate::BitReader<FUP0_A>;
#[doc = "Follow_Up Message Processing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FUP0_A {
    #[doc = "0: Messages are not transferred to the PTPEDMAC."]
    _0 = 0,
    #[doc = "1: Messages are transferred to the PTPEDMAC."]
    _1 = 1,
}
impl From<FUP0_A> for bool {
    #[inline(always)]
    fn from(variant: FUP0_A) -> Self {
        variant as u8 != 0
    }
}
impl FUP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FUP0_A {
        match self.bits {
            false => FUP0_A::_0,
            true => FUP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FUP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FUP0_A::_1
    }
}
#[doc = "Field `FUP0` writer - Follow_Up Message Processing"]
pub type FUP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYRFL1R_SPEC, FUP0_A, O>;
impl<'a, const O: u8> FUP0_W<'a, O> {
    #[doc = "Messages are not transferred to the PTPEDMAC."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FUP0_A::_0)
    }
    #[doc = "Messages are transferred to the PTPEDMAC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FUP0_A::_1)
    }
}
#[doc = "Field `FUP2` reader - Follow_Up Message Processing"]
pub type FUP2_R = crate::BitReader<FUP2_A>;
#[doc = "Follow_Up Message Processing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FUP2_A {
    #[doc = "0: The SYNFP does not process messages."]
    _0 = 0,
    #[doc = "1: The SYNFP processes messages."]
    _1 = 1,
}
impl From<FUP2_A> for bool {
    #[inline(always)]
    fn from(variant: FUP2_A) -> Self {
        variant as u8 != 0
    }
}
impl FUP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FUP2_A {
        match self.bits {
            false => FUP2_A::_0,
            true => FUP2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FUP2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FUP2_A::_1
    }
}
#[doc = "Field `FUP2` writer - Follow_Up Message Processing"]
pub type FUP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYRFL1R_SPEC, FUP2_A, O>;
impl<'a, const O: u8> FUP2_W<'a, O> {
    #[doc = "The SYNFP does not process messages."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FUP2_A::_0)
    }
    #[doc = "The SYNFP processes messages."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FUP2_A::_1)
    }
}
#[doc = "Field `DRQ0` reader - Delay_Req Message Processing"]
pub type DRQ0_R = crate::BitReader<DRQ0_A>;
#[doc = "Delay_Req Message Processing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRQ0_A {
    #[doc = "0: Messages are not transferred to the PTPEDMAC."]
    _0 = 0,
    #[doc = "1: Messages are transferred to the PTPEDMAC."]
    _1 = 1,
}
impl From<DRQ0_A> for bool {
    #[inline(always)]
    fn from(variant: DRQ0_A) -> Self {
        variant as u8 != 0
    }
}
impl DRQ0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRQ0_A {
        match self.bits {
            false => DRQ0_A::_0,
            true => DRQ0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRQ0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRQ0_A::_1
    }
}
#[doc = "Field `DRQ0` writer - Delay_Req Message Processing"]
pub type DRQ0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYRFL1R_SPEC, DRQ0_A, O>;
impl<'a, const O: u8> DRQ0_W<'a, O> {
    #[doc = "Messages are not transferred to the PTPEDMAC."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRQ0_A::_0)
    }
    #[doc = "Messages are transferred to the PTPEDMAC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRQ0_A::_1)
    }
}
#[doc = "Field `DRQ2` reader - Delay_Req Message Processing"]
pub type DRQ2_R = crate::BitReader<DRQ2_A>;
#[doc = "Delay_Req Message Processing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRQ2_A {
    #[doc = "0: The SYNFP does not process messages."]
    _0 = 0,
    #[doc = "1: The SYNFP processes messages."]
    _1 = 1,
}
impl From<DRQ2_A> for bool {
    #[inline(always)]
    fn from(variant: DRQ2_A) -> Self {
        variant as u8 != 0
    }
}
impl DRQ2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRQ2_A {
        match self.bits {
            false => DRQ2_A::_0,
            true => DRQ2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRQ2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRQ2_A::_1
    }
}
#[doc = "Field `DRQ2` writer - Delay_Req Message Processing"]
pub type DRQ2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYRFL1R_SPEC, DRQ2_A, O>;
impl<'a, const O: u8> DRQ2_W<'a, O> {
    #[doc = "The SYNFP does not process messages."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRQ2_A::_0)
    }
    #[doc = "The SYNFP processes messages."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRQ2_A::_1)
    }
}
#[doc = "Field `DRP0` reader - Delay_Resp Message Processing"]
pub type DRP0_R = crate::BitReader<DRP0_A>;
#[doc = "Delay_Resp Message Processing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRP0_A {
    #[doc = "0: Messages are not transferred to the PTPEDMAC."]
    _0 = 0,
    #[doc = "1: Messages are transferred to the PTPEDMAC."]
    _1 = 1,
}
impl From<DRP0_A> for bool {
    #[inline(always)]
    fn from(variant: DRP0_A) -> Self {
        variant as u8 != 0
    }
}
impl DRP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRP0_A {
        match self.bits {
            false => DRP0_A::_0,
            true => DRP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRP0_A::_1
    }
}
#[doc = "Field `DRP0` writer - Delay_Resp Message Processing"]
pub type DRP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYRFL1R_SPEC, DRP0_A, O>;
impl<'a, const O: u8> DRP0_W<'a, O> {
    #[doc = "Messages are not transferred to the PTPEDMAC."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRP0_A::_0)
    }
    #[doc = "Messages are transferred to the PTPEDMAC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRP0_A::_1)
    }
}
#[doc = "Field `DRP2` reader - Delay_Resp Message Processing"]
pub type DRP2_R = crate::BitReader<DRP2_A>;
#[doc = "Delay_Resp Message Processing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRP2_A {
    #[doc = "0: The SYNFP does not process messages."]
    _0 = 0,
    #[doc = "1: The SYNFP processes messages."]
    _1 = 1,
}
impl From<DRP2_A> for bool {
    #[inline(always)]
    fn from(variant: DRP2_A) -> Self {
        variant as u8 != 0
    }
}
impl DRP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRP2_A {
        match self.bits {
            false => DRP2_A::_0,
            true => DRP2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRP2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRP2_A::_1
    }
}
#[doc = "Field `DRP2` writer - Delay_Resp Message Processing"]
pub type DRP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYRFL1R_SPEC, DRP2_A, O>;
impl<'a, const O: u8> DRP2_W<'a, O> {
    #[doc = "The SYNFP does not process messages."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRP2_A::_0)
    }
    #[doc = "The SYNFP processes messages."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRP2_A::_1)
    }
}
#[doc = "Field `PDRQ0` reader - Pdelay_Req Message Processing"]
pub type PDRQ0_R = crate::BitReader<PDRQ0_A>;
#[doc = "Pdelay_Req Message Processing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDRQ0_A {
    #[doc = "0: Messages are not transferred to the PTPEDMAC."]
    _0 = 0,
    #[doc = "1: Messages are transferred to the PTPEDMAC."]
    _1 = 1,
}
impl From<PDRQ0_A> for bool {
    #[inline(always)]
    fn from(variant: PDRQ0_A) -> Self {
        variant as u8 != 0
    }
}
impl PDRQ0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDRQ0_A {
        match self.bits {
            false => PDRQ0_A::_0,
            true => PDRQ0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDRQ0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDRQ0_A::_1
    }
}
#[doc = "Field `PDRQ0` writer - Pdelay_Req Message Processing"]
pub type PDRQ0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYRFL1R_SPEC, PDRQ0_A, O>;
impl<'a, const O: u8> PDRQ0_W<'a, O> {
    #[doc = "Messages are not transferred to the PTPEDMAC."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDRQ0_A::_0)
    }
    #[doc = "Messages are transferred to the PTPEDMAC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDRQ0_A::_1)
    }
}
#[doc = "Field `PDRQ2` reader - Pdelay_Req Message Processing"]
pub type PDRQ2_R = crate::BitReader<PDRQ2_A>;
#[doc = "Pdelay_Req Message Processing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDRQ2_A {
    #[doc = "0: The SYNFP does not process messages."]
    _0 = 0,
    #[doc = "1: The SYNFP processes messages."]
    _1 = 1,
}
impl From<PDRQ2_A> for bool {
    #[inline(always)]
    fn from(variant: PDRQ2_A) -> Self {
        variant as u8 != 0
    }
}
impl PDRQ2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDRQ2_A {
        match self.bits {
            false => PDRQ2_A::_0,
            true => PDRQ2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDRQ2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDRQ2_A::_1
    }
}
#[doc = "Field `PDRQ2` writer - Pdelay_Req Message Processing"]
pub type PDRQ2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYRFL1R_SPEC, PDRQ2_A, O>;
impl<'a, const O: u8> PDRQ2_W<'a, O> {
    #[doc = "The SYNFP does not process messages."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDRQ2_A::_0)
    }
    #[doc = "The SYNFP processes messages."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDRQ2_A::_1)
    }
}
#[doc = "Field `PDRP0` reader - Pdelay_Resp Message Processing"]
pub type PDRP0_R = crate::BitReader<PDRP0_A>;
#[doc = "Pdelay_Resp Message Processing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDRP0_A {
    #[doc = "0: Messages are not transferred to the PTPEDMAC."]
    _0 = 0,
    #[doc = "1: Messages are transferred to the PTPEDMAC."]
    _1 = 1,
}
impl From<PDRP0_A> for bool {
    #[inline(always)]
    fn from(variant: PDRP0_A) -> Self {
        variant as u8 != 0
    }
}
impl PDRP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDRP0_A {
        match self.bits {
            false => PDRP0_A::_0,
            true => PDRP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDRP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDRP0_A::_1
    }
}
#[doc = "Field `PDRP0` writer - Pdelay_Resp Message Processing"]
pub type PDRP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYRFL1R_SPEC, PDRP0_A, O>;
impl<'a, const O: u8> PDRP0_W<'a, O> {
    #[doc = "Messages are not transferred to the PTPEDMAC."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDRP0_A::_0)
    }
    #[doc = "Messages are transferred to the PTPEDMAC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDRP0_A::_1)
    }
}
#[doc = "Field `PDRP2` reader - Pdelay_Resp Message Processing"]
pub type PDRP2_R = crate::BitReader<PDRP2_A>;
#[doc = "Pdelay_Resp Message Processing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDRP2_A {
    #[doc = "0: The SYNFP does not process messages."]
    _0 = 0,
    #[doc = "1: The SYNFP processes messages."]
    _1 = 1,
}
impl From<PDRP2_A> for bool {
    #[inline(always)]
    fn from(variant: PDRP2_A) -> Self {
        variant as u8 != 0
    }
}
impl PDRP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDRP2_A {
        match self.bits {
            false => PDRP2_A::_0,
            true => PDRP2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDRP2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDRP2_A::_1
    }
}
#[doc = "Field `PDRP2` writer - Pdelay_Resp Message Processing"]
pub type PDRP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYRFL1R_SPEC, PDRP2_A, O>;
impl<'a, const O: u8> PDRP2_W<'a, O> {
    #[doc = "The SYNFP does not process messages."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDRP2_A::_0)
    }
    #[doc = "The SYNFP processes messages."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDRP2_A::_1)
    }
}
#[doc = "Field `PDFUP0` reader - Pdelay_Resp_Follow_Up Message Processing"]
pub type PDFUP0_R = crate::BitReader<PDFUP0_A>;
#[doc = "Pdelay_Resp_Follow_Up Message Processing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDFUP0_A {
    #[doc = "0: Messages are not transferred to the PTPEDMAC."]
    _0 = 0,
    #[doc = "1: Messages are transferred to the PTPEDMAC."]
    _1 = 1,
}
impl From<PDFUP0_A> for bool {
    #[inline(always)]
    fn from(variant: PDFUP0_A) -> Self {
        variant as u8 != 0
    }
}
impl PDFUP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDFUP0_A {
        match self.bits {
            false => PDFUP0_A::_0,
            true => PDFUP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDFUP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDFUP0_A::_1
    }
}
#[doc = "Field `PDFUP0` writer - Pdelay_Resp_Follow_Up Message Processing"]
pub type PDFUP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYRFL1R_SPEC, PDFUP0_A, O>;
impl<'a, const O: u8> PDFUP0_W<'a, O> {
    #[doc = "Messages are not transferred to the PTPEDMAC."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDFUP0_A::_0)
    }
    #[doc = "Messages are transferred to the PTPEDMAC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDFUP0_A::_1)
    }
}
#[doc = "Field `PDFUP2` reader - Pdelay_Resp_Follow_Up Message Processing"]
pub type PDFUP2_R = crate::BitReader<PDFUP2_A>;
#[doc = "Pdelay_Resp_Follow_Up Message Processing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDFUP2_A {
    #[doc = "0: The SYNFP does not process messages."]
    _0 = 0,
    #[doc = "1: The SYNFP does not process messages."]
    _1 = 1,
}
impl From<PDFUP2_A> for bool {
    #[inline(always)]
    fn from(variant: PDFUP2_A) -> Self {
        variant as u8 != 0
    }
}
impl PDFUP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDFUP2_A {
        match self.bits {
            false => PDFUP2_A::_0,
            true => PDFUP2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDFUP2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDFUP2_A::_1
    }
}
#[doc = "Field `PDFUP2` writer - Pdelay_Resp_Follow_Up Message Processing"]
pub type PDFUP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYRFL1R_SPEC, PDFUP2_A, O>;
impl<'a, const O: u8> PDFUP2_W<'a, O> {
    #[doc = "The SYNFP does not process messages."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDFUP2_A::_0)
    }
    #[doc = "The SYNFP does not process messages."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDFUP2_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Announce Message Processing"]
    #[inline(always)]
    pub fn ance0(&self) -> ANCE0_R {
        ANCE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Sync Message Processing"]
    #[inline(always)]
    pub fn sync0(&self) -> SYNC0_R {
        SYNC0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Sync Message Processing"]
    #[inline(always)]
    pub fn sync2(&self) -> SYNC2_R {
        SYNC2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Follow_Up Message Processing"]
    #[inline(always)]
    pub fn fup0(&self) -> FUP0_R {
        FUP0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Follow_Up Message Processing"]
    #[inline(always)]
    pub fn fup2(&self) -> FUP2_R {
        FUP2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Delay_Req Message Processing"]
    #[inline(always)]
    pub fn drq0(&self) -> DRQ0_R {
        DRQ0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Delay_Req Message Processing"]
    #[inline(always)]
    pub fn drq2(&self) -> DRQ2_R {
        DRQ2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Delay_Resp Message Processing"]
    #[inline(always)]
    pub fn drp0(&self) -> DRP0_R {
        DRP0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Delay_Resp Message Processing"]
    #[inline(always)]
    pub fn drp2(&self) -> DRP2_R {
        DRP2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Pdelay_Req Message Processing"]
    #[inline(always)]
    pub fn pdrq0(&self) -> PDRQ0_R {
        PDRQ0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Pdelay_Req Message Processing"]
    #[inline(always)]
    pub fn pdrq2(&self) -> PDRQ2_R {
        PDRQ2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Pdelay_Resp Message Processing"]
    #[inline(always)]
    pub fn pdrp0(&self) -> PDRP0_R {
        PDRP0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Pdelay_Resp Message Processing"]
    #[inline(always)]
    pub fn pdrp2(&self) -> PDRP2_R {
        PDRP2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Pdelay_Resp_Follow_Up Message Processing"]
    #[inline(always)]
    pub fn pdfup0(&self) -> PDFUP0_R {
        PDFUP0_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Pdelay_Resp_Follow_Up Message Processing"]
    #[inline(always)]
    pub fn pdfup2(&self) -> PDFUP2_R {
        PDFUP2_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Announce Message Processing"]
    #[inline(always)]
    #[must_use]
    pub fn ance0(&mut self) -> ANCE0_W<0> {
        ANCE0_W::new(self)
    }
    #[doc = "Bit 4 - Sync Message Processing"]
    #[inline(always)]
    #[must_use]
    pub fn sync0(&mut self) -> SYNC0_W<4> {
        SYNC0_W::new(self)
    }
    #[doc = "Bit 6 - Sync Message Processing"]
    #[inline(always)]
    #[must_use]
    pub fn sync2(&mut self) -> SYNC2_W<6> {
        SYNC2_W::new(self)
    }
    #[doc = "Bit 8 - Follow_Up Message Processing"]
    #[inline(always)]
    #[must_use]
    pub fn fup0(&mut self) -> FUP0_W<8> {
        FUP0_W::new(self)
    }
    #[doc = "Bit 10 - Follow_Up Message Processing"]
    #[inline(always)]
    #[must_use]
    pub fn fup2(&mut self) -> FUP2_W<10> {
        FUP2_W::new(self)
    }
    #[doc = "Bit 12 - Delay_Req Message Processing"]
    #[inline(always)]
    #[must_use]
    pub fn drq0(&mut self) -> DRQ0_W<12> {
        DRQ0_W::new(self)
    }
    #[doc = "Bit 14 - Delay_Req Message Processing"]
    #[inline(always)]
    #[must_use]
    pub fn drq2(&mut self) -> DRQ2_W<14> {
        DRQ2_W::new(self)
    }
    #[doc = "Bit 16 - Delay_Resp Message Processing"]
    #[inline(always)]
    #[must_use]
    pub fn drp0(&mut self) -> DRP0_W<16> {
        DRP0_W::new(self)
    }
    #[doc = "Bit 18 - Delay_Resp Message Processing"]
    #[inline(always)]
    #[must_use]
    pub fn drp2(&mut self) -> DRP2_W<18> {
        DRP2_W::new(self)
    }
    #[doc = "Bit 20 - Pdelay_Req Message Processing"]
    #[inline(always)]
    #[must_use]
    pub fn pdrq0(&mut self) -> PDRQ0_W<20> {
        PDRQ0_W::new(self)
    }
    #[doc = "Bit 22 - Pdelay_Req Message Processing"]
    #[inline(always)]
    #[must_use]
    pub fn pdrq2(&mut self) -> PDRQ2_W<22> {
        PDRQ2_W::new(self)
    }
    #[doc = "Bit 24 - Pdelay_Resp Message Processing"]
    #[inline(always)]
    #[must_use]
    pub fn pdrp0(&mut self) -> PDRP0_W<24> {
        PDRP0_W::new(self)
    }
    #[doc = "Bit 26 - Pdelay_Resp Message Processing"]
    #[inline(always)]
    #[must_use]
    pub fn pdrp2(&mut self) -> PDRP2_W<26> {
        PDRP2_W::new(self)
    }
    #[doc = "Bit 28 - Pdelay_Resp_Follow_Up Message Processing"]
    #[inline(always)]
    #[must_use]
    pub fn pdfup0(&mut self) -> PDFUP0_W<28> {
        PDFUP0_W::new(self)
    }
    #[doc = "Bit 30 - Pdelay_Resp_Follow_Up Message Processing"]
    #[inline(always)]
    #[must_use]
    pub fn pdfup2(&mut self) -> PDFUP2_W<30> {
        PDFUP2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYNFP Reception Filter Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syrfl1r](index.html) module"]
pub struct SYRFL1R_SPEC;
impl crate::RegisterSpec for SYRFL1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syrfl1r::R](R) reader structure"]
impl crate::Readable for SYRFL1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syrfl1r::W](W) writer structure"]
impl crate::Writable for SYRFL1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYRFL1R to value 0"]
impl crate::Resettable for SYRFL1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
