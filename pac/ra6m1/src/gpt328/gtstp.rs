#[doc = "Register `GTSTP` reader"]
pub struct R(crate::R<GTSTP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTSTP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTSTP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTSTP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTSTP` writer"]
pub struct W(crate::W<GTSTP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTSTP_SPEC>;
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
impl From<crate::W<GTSTP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTSTP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSTOP0` reader - Channel 0 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP0_R = crate::BitReader<CSTOP0_A>;
#[doc = "Channel 0 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP0_A {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT32EH0.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<CSTOP0_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP0_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP0_A {
        match self.bits {
            false => CSTOP0_A::_0,
            true => CSTOP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP0_A::_1
    }
}
#[doc = "Field `CSTOP0` writer - Channel 0 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP0_A, O>;
impl<'a, const O: u8> CSTOP0_W<'a, O> {
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP0_A::_0)
    }
    #[doc = "GPT32EH0.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP0_A::_1)
    }
}
#[doc = "Field `CSTOP1` reader - Channel 1 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP1_R = crate::BitReader<CSTOP1_A>;
#[doc = "Channel 1 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP1_A {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT32EH1.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<CSTOP1_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP1_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP1_A {
        match self.bits {
            false => CSTOP1_A::_0,
            true => CSTOP1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP1_A::_1
    }
}
#[doc = "Field `CSTOP1` writer - Channel 1 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP1_A, O>;
impl<'a, const O: u8> CSTOP1_W<'a, O> {
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP1_A::_0)
    }
    #[doc = "GPT32EH1.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP1_A::_1)
    }
}
#[doc = "Field `CSTOP2` reader - Channel 2 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP2_R = crate::BitReader<CSTOP2_A>;
#[doc = "Channel 2 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP2_A {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT32EH2.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<CSTOP2_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP2_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP2_A {
        match self.bits {
            false => CSTOP2_A::_0,
            true => CSTOP2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP2_A::_1
    }
}
#[doc = "Field `CSTOP2` writer - Channel 2 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP2_A, O>;
impl<'a, const O: u8> CSTOP2_W<'a, O> {
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP2_A::_0)
    }
    #[doc = "GPT32EH2.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP2_A::_1)
    }
}
#[doc = "Field `CSTOP3` reader - Channel 3 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP3_R = crate::BitReader<CSTOP3_A>;
#[doc = "Channel 3 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP3_A {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT32EH3.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<CSTOP3_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP3_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP3_A {
        match self.bits {
            false => CSTOP3_A::_0,
            true => CSTOP3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP3_A::_1
    }
}
#[doc = "Field `CSTOP3` writer - Channel 3 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP3_A, O>;
impl<'a, const O: u8> CSTOP3_W<'a, O> {
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP3_A::_0)
    }
    #[doc = "GPT32EH3.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP3_A::_1)
    }
}
#[doc = "Field `CSTOP4` reader - Channel 4 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP4_R = crate::BitReader<CSTOP4_A>;
#[doc = "Channel 4 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP4_A {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT32E4.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<CSTOP4_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP4_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP4_A {
        match self.bits {
            false => CSTOP4_A::_0,
            true => CSTOP4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP4_A::_1
    }
}
#[doc = "Field `CSTOP4` writer - Channel 4 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP4_A, O>;
impl<'a, const O: u8> CSTOP4_W<'a, O> {
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP4_A::_0)
    }
    #[doc = "GPT32E4.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP4_A::_1)
    }
}
#[doc = "Field `CSTOP5` reader - Channel 5 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP5_R = crate::BitReader<CSTOP5_A>;
#[doc = "Channel 5 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP5_A {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT32E5.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<CSTOP5_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP5_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP5_A {
        match self.bits {
            false => CSTOP5_A::_0,
            true => CSTOP5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP5_A::_1
    }
}
#[doc = "Field `CSTOP5` writer - Channel 5 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP5_A, O>;
impl<'a, const O: u8> CSTOP5_W<'a, O> {
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP5_A::_0)
    }
    #[doc = "GPT32E5.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP5_A::_1)
    }
}
#[doc = "Field `CSTOP6` reader - Channel 6 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP6_R = crate::BitReader<CSTOP6_A>;
#[doc = "Channel 6 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP6_A {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT32E6.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<CSTOP6_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP6_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP6_A {
        match self.bits {
            false => CSTOP6_A::_0,
            true => CSTOP6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP6_A::_1
    }
}
#[doc = "Field `CSTOP6` writer - Channel 6 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP6_A, O>;
impl<'a, const O: u8> CSTOP6_W<'a, O> {
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP6_A::_0)
    }
    #[doc = "GPT32E6.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP6_A::_1)
    }
}
#[doc = "Field `CSTOP7` reader - Channel 7 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP7_R = crate::BitReader<CSTOP7_A>;
#[doc = "Channel 7 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP7_A {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT32E7.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<CSTOP7_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP7_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP7_A {
        match self.bits {
            false => CSTOP7_A::_0,
            true => CSTOP7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP7_A::_1
    }
}
#[doc = "Field `CSTOP7` writer - Channel 7 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP7_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP7_A, O>;
impl<'a, const O: u8> CSTOP7_W<'a, O> {
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP7_A::_0)
    }
    #[doc = "GPT32E7.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP7_A::_1)
    }
}
#[doc = "Field `CSTOP8` reader - Channel 8 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP8_R = crate::BitReader<CSTOP8_A>;
#[doc = "Channel 8 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP8_A {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT328.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<CSTOP8_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP8_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP8_A {
        match self.bits {
            false => CSTOP8_A::_0,
            true => CSTOP8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP8_A::_1
    }
}
#[doc = "Field `CSTOP8` writer - Channel 8 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP8_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP8_A, O>;
impl<'a, const O: u8> CSTOP8_W<'a, O> {
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP8_A::_0)
    }
    #[doc = "GPT328.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP8_A::_1)
    }
}
#[doc = "Field `CSTOP9` reader - Channel 9 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP9_R = crate::BitReader<CSTOP9_A>;
#[doc = "Channel 9 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP9_A {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT329.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<CSTOP9_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP9_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP9_A {
        match self.bits {
            false => CSTOP9_A::_0,
            true => CSTOP9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP9_A::_1
    }
}
#[doc = "Field `CSTOP9` writer - Channel 9 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP9_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP9_A, O>;
impl<'a, const O: u8> CSTOP9_W<'a, O> {
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP9_A::_0)
    }
    #[doc = "GPT329.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP9_A::_1)
    }
}
#[doc = "Field `CSTOP10` reader - Channel 10 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP10_R = crate::BitReader<CSTOP10_A>;
#[doc = "Channel 10 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP10_A {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT3210.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<CSTOP10_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP10_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP10_A {
        match self.bits {
            false => CSTOP10_A::_0,
            true => CSTOP10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP10_A::_1
    }
}
#[doc = "Field `CSTOP10` writer - Channel 10 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP10_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP10_A, O>;
impl<'a, const O: u8> CSTOP10_W<'a, O> {
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP10_A::_0)
    }
    #[doc = "GPT3210.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP10_A::_1)
    }
}
#[doc = "Field `CSTOP11` reader - Channel 11 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP11_R = crate::BitReader<CSTOP11_A>;
#[doc = "Channel 11 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP11_A {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT3211.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<CSTOP11_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP11_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP11_A {
        match self.bits {
            false => CSTOP11_A::_0,
            true => CSTOP11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP11_A::_1
    }
}
#[doc = "Field `CSTOP11` writer - Channel 11 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP11_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP11_A, O>;
impl<'a, const O: u8> CSTOP11_W<'a, O> {
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP11_A::_0)
    }
    #[doc = "GPT3211.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP11_A::_1)
    }
}
#[doc = "Field `CSTOP12` reader - Channel 12 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP12_R = crate::BitReader<CSTOP12_A>;
#[doc = "Channel 12 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP12_A {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT3212.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<CSTOP12_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP12_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP12_A {
        match self.bits {
            false => CSTOP12_A::_0,
            true => CSTOP12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP12_A::_1
    }
}
#[doc = "Field `CSTOP12` writer - Channel 12 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP12_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP12_A, O>;
impl<'a, const O: u8> CSTOP12_W<'a, O> {
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP12_A::_0)
    }
    #[doc = "GPT3212.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP12_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop0(&self) -> CSTOP0_R {
        CSTOP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop1(&self) -> CSTOP1_R {
        CSTOP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop2(&self) -> CSTOP2_R {
        CSTOP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop3(&self) -> CSTOP3_R {
        CSTOP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop4(&self) -> CSTOP4_R {
        CSTOP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop5(&self) -> CSTOP5_R {
        CSTOP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop6(&self) -> CSTOP6_R {
        CSTOP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop7(&self) -> CSTOP7_R {
        CSTOP7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 8 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop8(&self) -> CSTOP8_R {
        CSTOP8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 9 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop9(&self) -> CSTOP9_R {
        CSTOP9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 10 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop10(&self) -> CSTOP10_R {
        CSTOP10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 11 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop11(&self) -> CSTOP11_R {
        CSTOP11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 12 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop12(&self) -> CSTOP12_R {
        CSTOP12_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    #[must_use]
    pub fn cstop0(&mut self) -> CSTOP0_W<0> {
        CSTOP0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    #[must_use]
    pub fn cstop1(&mut self) -> CSTOP1_W<1> {
        CSTOP1_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    #[must_use]
    pub fn cstop2(&mut self) -> CSTOP2_W<2> {
        CSTOP2_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    #[must_use]
    pub fn cstop3(&mut self) -> CSTOP3_W<3> {
        CSTOP3_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    #[must_use]
    pub fn cstop4(&mut self) -> CSTOP4_W<4> {
        CSTOP4_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    #[must_use]
    pub fn cstop5(&mut self) -> CSTOP5_W<5> {
        CSTOP5_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    #[must_use]
    pub fn cstop6(&mut self) -> CSTOP6_W<6> {
        CSTOP6_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    #[must_use]
    pub fn cstop7(&mut self) -> CSTOP7_W<7> {
        CSTOP7_W::new(self)
    }
    #[doc = "Bit 8 - Channel 8 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    #[must_use]
    pub fn cstop8(&mut self) -> CSTOP8_W<8> {
        CSTOP8_W::new(self)
    }
    #[doc = "Bit 9 - Channel 9 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    #[must_use]
    pub fn cstop9(&mut self) -> CSTOP9_W<9> {
        CSTOP9_W::new(self)
    }
    #[doc = "Bit 10 - Channel 10 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    #[must_use]
    pub fn cstop10(&mut self) -> CSTOP10_W<10> {
        CSTOP10_W::new(self)
    }
    #[doc = "Bit 11 - Channel 11 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    #[must_use]
    pub fn cstop11(&mut self) -> CSTOP11_W<11> {
        CSTOP11_W::new(self)
    }
    #[doc = "Bit 12 - Channel 12 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    #[must_use]
    pub fn cstop12(&mut self) -> CSTOP12_W<12> {
        CSTOP12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Software Stop Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtstp](index.html) module"]
pub struct GTSTP_SPEC;
impl crate::RegisterSpec for GTSTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtstp::R](R) reader structure"]
impl crate::Readable for GTSTP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtstp::W](W) writer structure"]
impl crate::Writable for GTSTP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTSTP to value 0xffff_ffff"]
impl crate::Resettable for GTSTP_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
