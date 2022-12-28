#[doc = "Register `SYSR` reader"]
pub struct R(crate::R<SYSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSR` writer"]
pub struct W(crate::W<SYSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSR_SPEC>;
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
impl From<crate::W<SYSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFMUD` reader - offsetFromMaster Value Update Flag\n\nThe field is **modified** in some way after a read operation."]
pub type OFMUD_R = crate::BitReader<OFMUD_A>;
#[doc = "offsetFromMaster Value Update Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFMUD_A {
    #[doc = "0: The offsetFromMaster value has not been updated."]
    _0 = 0,
    #[doc = "1: The offsetFromMaster value has been updated."]
    _1 = 1,
}
impl From<OFMUD_A> for bool {
    #[inline(always)]
    fn from(variant: OFMUD_A) -> Self {
        variant as u8 != 0
    }
}
impl OFMUD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFMUD_A {
        match self.bits {
            false => OFMUD_A::_0,
            true => OFMUD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFMUD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFMUD_A::_1
    }
}
#[doc = "Field `OFMUD` writer - offsetFromMaster Value Update Flag"]
pub type OFMUD_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SYSR_SPEC, OFMUD_A, O>;
impl<'a, const O: u8> OFMUD_W<'a, O> {
    #[doc = "The offsetFromMaster value has not been updated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OFMUD_A::_0)
    }
    #[doc = "The offsetFromMaster value has been updated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OFMUD_A::_1)
    }
}
#[doc = "Field `INTCHG` reader - Receive logMessageInterval Value Change Detection Flag\n\nThe field is **modified** in some way after a read operation."]
pub type INTCHG_R = crate::BitReader<INTCHG_A>;
#[doc = "Receive logMessageInterval Value Change Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTCHG_A {
    #[doc = "0: No change in the received logMessageInterval value."]
    _0 = 0,
    #[doc = "1: A change in the received logMessageInterval value."]
    _1 = 1,
}
impl From<INTCHG_A> for bool {
    #[inline(always)]
    fn from(variant: INTCHG_A) -> Self {
        variant as u8 != 0
    }
}
impl INTCHG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTCHG_A {
        match self.bits {
            false => INTCHG_A::_0,
            true => INTCHG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INTCHG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INTCHG_A::_1
    }
}
#[doc = "Field `INTCHG` writer - Receive logMessageInterval Value Change Detection Flag"]
pub type INTCHG_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SYSR_SPEC, INTCHG_A, O>;
impl<'a, const O: u8> INTCHG_W<'a, O> {
    #[doc = "No change in the received logMessageInterval value."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTCHG_A::_0)
    }
    #[doc = "A change in the received logMessageInterval value."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTCHG_A::_1)
    }
}
#[doc = "Field `MPDUD` reader - meanPathDelay Value Update Flag\n\nThe field is **modified** in some way after a read operation."]
pub type MPDUD_R = crate::BitReader<MPDUD_A>;
#[doc = "meanPathDelay Value Update Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPDUD_A {
    #[doc = "0: The meanPathDelay value has not been updated."]
    _0 = 0,
    #[doc = "1: The meanPathDelay value has been updated."]
    _1 = 1,
}
impl From<MPDUD_A> for bool {
    #[inline(always)]
    fn from(variant: MPDUD_A) -> Self {
        variant as u8 != 0
    }
}
impl MPDUD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPDUD_A {
        match self.bits {
            false => MPDUD_A::_0,
            true => MPDUD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPDUD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPDUD_A::_1
    }
}
#[doc = "Field `MPDUD` writer - meanPathDelay Value Update Flag"]
pub type MPDUD_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SYSR_SPEC, MPDUD_A, O>;
impl<'a, const O: u8> MPDUD_W<'a, O> {
    #[doc = "The meanPathDelay value has not been updated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPDUD_A::_0)
    }
    #[doc = "The meanPathDelay value has been updated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPDUD_A::_1)
    }
}
#[doc = "Field `DRPTO` reader - Delay_Resp/Pdelay_Resp Reception Timeout Detection Flag\n\nThe field is **modified** in some way after a read operation."]
pub type DRPTO_R = crate::BitReader<DRPTO_A>;
#[doc = "Delay_Resp/Pdelay_Resp Reception Timeout Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRPTO_A {
    #[doc = "0: A Delay_Resp/Pdelay_Resp timeout has not occurred."]
    _0 = 0,
    #[doc = "1: A Delay_Resp/Pdelay_Resp timeout has occurred."]
    _1 = 1,
}
impl From<DRPTO_A> for bool {
    #[inline(always)]
    fn from(variant: DRPTO_A) -> Self {
        variant as u8 != 0
    }
}
impl DRPTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRPTO_A {
        match self.bits {
            false => DRPTO_A::_0,
            true => DRPTO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRPTO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRPTO_A::_1
    }
}
#[doc = "Field `DRPTO` writer - Delay_Resp/Pdelay_Resp Reception Timeout Detection Flag"]
pub type DRPTO_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SYSR_SPEC, DRPTO_A, O>;
impl<'a, const O: u8> DRPTO_W<'a, O> {
    #[doc = "A Delay_Resp/Pdelay_Resp timeout has not occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRPTO_A::_0)
    }
    #[doc = "A Delay_Resp/Pdelay_Resp timeout has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRPTO_A::_1)
    }
}
#[doc = "Field `INTDEV` reader - Receive logMessageInterval Value Out-of-Range Flag\n\nThe field is **modified** in some way after a read operation."]
pub type INTDEV_R = crate::BitReader<INTDEV_A>;
#[doc = "Receive logMessageInterval Value Out-of-Range Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTDEV_A {
    #[doc = "0: The received logMessageInterval value is within the range."]
    _0 = 0,
    #[doc = "1: The received logMessageInterval value is out of the range."]
    _1 = 1,
}
impl From<INTDEV_A> for bool {
    #[inline(always)]
    fn from(variant: INTDEV_A) -> Self {
        variant as u8 != 0
    }
}
impl INTDEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTDEV_A {
        match self.bits {
            false => INTDEV_A::_0,
            true => INTDEV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INTDEV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INTDEV_A::_1
    }
}
#[doc = "Field `INTDEV` writer - Receive logMessageInterval Value Out-of-Range Flag"]
pub type INTDEV_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SYSR_SPEC, INTDEV_A, O>;
impl<'a, const O: u8> INTDEV_W<'a, O> {
    #[doc = "The received logMessageInterval value is within the range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTDEV_A::_0)
    }
    #[doc = "The received logMessageInterval value is out of the range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTDEV_A::_1)
    }
}
#[doc = "Field `DRQOVR` reader - Delay_Req Reception FIFO Overflow Detection Flag\n\nThe field is **modified** in some way after a read operation."]
pub type DRQOVR_R = crate::BitReader<DRQOVR_A>;
#[doc = "Delay_Req Reception FIFO Overflow Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRQOVR_A {
    #[doc = "0: The received Delay_Req has not caused the reception FIFO to overflow."]
    _0 = 0,
    #[doc = "1: The received Delay_Req has caused the reception FIFO to overflow."]
    _1 = 1,
}
impl From<DRQOVR_A> for bool {
    #[inline(always)]
    fn from(variant: DRQOVR_A) -> Self {
        variant as u8 != 0
    }
}
impl DRQOVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRQOVR_A {
        match self.bits {
            false => DRQOVR_A::_0,
            true => DRQOVR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRQOVR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRQOVR_A::_1
    }
}
#[doc = "Field `DRQOVR` writer - Delay_Req Reception FIFO Overflow Detection Flag"]
pub type DRQOVR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SYSR_SPEC, DRQOVR_A, O>;
impl<'a, const O: u8> DRQOVR_W<'a, O> {
    #[doc = "The received Delay_Req has not caused the reception FIFO to overflow."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRQOVR_A::_0)
    }
    #[doc = "The received Delay_Req has caused the reception FIFO to overflow."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRQOVR_A::_1)
    }
}
#[doc = "Field `RECLP` reader - Loop Reception Detection Flag\n\nThe field is **modified** in some way after a read operation."]
pub type RECLP_R = crate::BitReader<RECLP_A>;
#[doc = "Loop Reception Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECLP_A {
    #[doc = "0: A received message has not returned through a loop."]
    _0 = 0,
    #[doc = "1: A received message has returned through a loop."]
    _1 = 1,
}
impl From<RECLP_A> for bool {
    #[inline(always)]
    fn from(variant: RECLP_A) -> Self {
        variant as u8 != 0
    }
}
impl RECLP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECLP_A {
        match self.bits {
            false => RECLP_A::_0,
            true => RECLP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RECLP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RECLP_A::_1
    }
}
#[doc = "Field `RECLP` writer - Loop Reception Detection Flag"]
pub type RECLP_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SYSR_SPEC, RECLP_A, O>;
impl<'a, const O: u8> RECLP_W<'a, O> {
    #[doc = "A received message has not returned through a loop."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RECLP_A::_0)
    }
    #[doc = "A received message has returned through a loop."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RECLP_A::_1)
    }
}
#[doc = "Field `INFABT` reader - Control Information Abnormality Detection Flag\n\nThe field is **modified** in some way after a read operation."]
pub type INFABT_R = crate::BitReader<INFABT_A>;
#[doc = "Control Information Abnormality Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INFABT_A {
    #[doc = "0: No abnormality in control information"]
    _0 = 0,
    #[doc = "1: Abnormality in control information"]
    _1 = 1,
}
impl From<INFABT_A> for bool {
    #[inline(always)]
    fn from(variant: INFABT_A) -> Self {
        variant as u8 != 0
    }
}
impl INFABT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INFABT_A {
        match self.bits {
            false => INFABT_A::_0,
            true => INFABT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INFABT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INFABT_A::_1
    }
}
#[doc = "Field `INFABT` writer - Control Information Abnormality Detection Flag"]
pub type INFABT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SYSR_SPEC, INFABT_A, O>;
impl<'a, const O: u8> INFABT_W<'a, O> {
    #[doc = "No abnormality in control information"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INFABT_A::_0)
    }
    #[doc = "Abnormality in control information"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INFABT_A::_1)
    }
}
#[doc = "Field `RESDN` reader - Response Stop Completion Detection Flag\n\nThe field is **modified** in some way after a read operation."]
pub type RESDN_R = crate::BitReader<RESDN_A>;
#[doc = "Response Stop Completion Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESDN_A {
    #[doc = "0: Stopping responses has not been completed."]
    _0 = 0,
    #[doc = "1: Stopping responses has been completed."]
    _1 = 1,
}
impl From<RESDN_A> for bool {
    #[inline(always)]
    fn from(variant: RESDN_A) -> Self {
        variant as u8 != 0
    }
}
impl RESDN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESDN_A {
        match self.bits {
            false => RESDN_A::_0,
            true => RESDN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RESDN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RESDN_A::_1
    }
}
#[doc = "Field `RESDN` writer - Response Stop Completion Detection Flag"]
pub type RESDN_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SYSR_SPEC, RESDN_A, O>;
impl<'a, const O: u8> RESDN_W<'a, O> {
    #[doc = "Stopping responses has not been completed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RESDN_A::_0)
    }
    #[doc = "Stopping responses has been completed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RESDN_A::_1)
    }
}
#[doc = "Field `GENDN` reader - Generation Stop Completion Detection Flag\n\nThe field is **modified** in some way after a read operation."]
pub type GENDN_R = crate::BitReader<GENDN_A>;
#[doc = "Generation Stop Completion Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GENDN_A {
    #[doc = "0: Stopping generation has not been completed."]
    _0 = 0,
    #[doc = "1: Stopping generation has been completed."]
    _1 = 1,
}
impl From<GENDN_A> for bool {
    #[inline(always)]
    fn from(variant: GENDN_A) -> Self {
        variant as u8 != 0
    }
}
impl GENDN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GENDN_A {
        match self.bits {
            false => GENDN_A::_0,
            true => GENDN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GENDN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GENDN_A::_1
    }
}
#[doc = "Field `GENDN` writer - Generation Stop Completion Detection Flag"]
pub type GENDN_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SYSR_SPEC, GENDN_A, O>;
impl<'a, const O: u8> GENDN_W<'a, O> {
    #[doc = "Stopping generation has not been completed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GENDN_A::_0)
    }
    #[doc = "Stopping generation has been completed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GENDN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - offsetFromMaster Value Update Flag"]
    #[inline(always)]
    pub fn ofmud(&self) -> OFMUD_R {
        OFMUD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive logMessageInterval Value Change Detection Flag"]
    #[inline(always)]
    pub fn intchg(&self) -> INTCHG_R {
        INTCHG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - meanPathDelay Value Update Flag"]
    #[inline(always)]
    pub fn mpdud(&self) -> MPDUD_R {
        MPDUD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Delay_Resp/Pdelay_Resp Reception Timeout Detection Flag"]
    #[inline(always)]
    pub fn drpto(&self) -> DRPTO_R {
        DRPTO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive logMessageInterval Value Out-of-Range Flag"]
    #[inline(always)]
    pub fn intdev(&self) -> INTDEV_R {
        INTDEV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Delay_Req Reception FIFO Overflow Detection Flag"]
    #[inline(always)]
    pub fn drqovr(&self) -> DRQOVR_R {
        DRQOVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - Loop Reception Detection Flag"]
    #[inline(always)]
    pub fn reclp(&self) -> RECLP_R {
        RECLP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Control Information Abnormality Detection Flag"]
    #[inline(always)]
    pub fn infabt(&self) -> INFABT_R {
        INFABT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Response Stop Completion Detection Flag"]
    #[inline(always)]
    pub fn resdn(&self) -> RESDN_R {
        RESDN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Generation Stop Completion Detection Flag"]
    #[inline(always)]
    pub fn gendn(&self) -> GENDN_R {
        GENDN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - offsetFromMaster Value Update Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ofmud(&mut self) -> OFMUD_W<0> {
        OFMUD_W::new(self)
    }
    #[doc = "Bit 1 - Receive logMessageInterval Value Change Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn intchg(&mut self) -> INTCHG_W<1> {
        INTCHG_W::new(self)
    }
    #[doc = "Bit 2 - meanPathDelay Value Update Flag"]
    #[inline(always)]
    #[must_use]
    pub fn mpdud(&mut self) -> MPDUD_W<2> {
        MPDUD_W::new(self)
    }
    #[doc = "Bit 4 - Delay_Resp/Pdelay_Resp Reception Timeout Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn drpto(&mut self) -> DRPTO_W<4> {
        DRPTO_W::new(self)
    }
    #[doc = "Bit 5 - Receive logMessageInterval Value Out-of-Range Flag"]
    #[inline(always)]
    #[must_use]
    pub fn intdev(&mut self) -> INTDEV_W<5> {
        INTDEV_W::new(self)
    }
    #[doc = "Bit 6 - Delay_Req Reception FIFO Overflow Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn drqovr(&mut self) -> DRQOVR_W<6> {
        DRQOVR_W::new(self)
    }
    #[doc = "Bit 12 - Loop Reception Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn reclp(&mut self) -> RECLP_W<12> {
        RECLP_W::new(self)
    }
    #[doc = "Bit 14 - Control Information Abnormality Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn infabt(&mut self) -> INFABT_W<14> {
        INFABT_W::new(self)
    }
    #[doc = "Bit 16 - Response Stop Completion Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn resdn(&mut self) -> RESDN_W<16> {
        RESDN_W::new(self)
    }
    #[doc = "Bit 17 - Generation Stop Completion Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn gendn(&mut self) -> GENDN_W<17> {
        GENDN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYNFP Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysr](index.html) module"]
pub struct SYSR_SPEC;
impl crate::RegisterSpec for SYSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysr::R](R) reader structure"]
impl crate::Readable for SYSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysr::W](W) writer structure"]
impl crate::Writable for SYSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0003_5077;
}
#[doc = "`reset()` method sets SYSR to value 0"]
impl crate::Resettable for SYSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
