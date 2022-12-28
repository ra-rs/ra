#[doc = "Register `GTSTR` reader"]
pub struct R(crate::R<GTSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTSTR` writer"]
pub struct W(crate::W<GTSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTSTR_SPEC>;
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
impl From<crate::W<GTSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSTRT0` reader - Channel 0 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT0_R = crate::BitReader<CSTRT0_A>;
#[doc = "Channel 0 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT0_A {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT32EH0.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<CSTRT0_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT0_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT0_A {
        match self.bits {
            false => CSTRT0_A::_0,
            true => CSTRT0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT0_A::_1
    }
}
#[doc = "Field `CSTRT0` writer - Channel 0 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT0_A, O>;
impl<'a, const O: u8> CSTRT0_W<'a, O> {
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT0_A::_0)
    }
    #[doc = "GPT32EH0.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT0_A::_1)
    }
}
#[doc = "Field `CSTRT1` reader - Channel 1 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT1_R = crate::BitReader<CSTRT1_A>;
#[doc = "Channel 1 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT1_A {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT32EH1.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<CSTRT1_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT1_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT1_A {
        match self.bits {
            false => CSTRT1_A::_0,
            true => CSTRT1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT1_A::_1
    }
}
#[doc = "Field `CSTRT1` writer - Channel 1 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT1_A, O>;
impl<'a, const O: u8> CSTRT1_W<'a, O> {
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT1_A::_0)
    }
    #[doc = "GPT32EH1.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT1_A::_1)
    }
}
#[doc = "Field `CSTRT2` reader - Channel 2 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT2_R = crate::BitReader<CSTRT2_A>;
#[doc = "Channel 2 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT2_A {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT32EH2.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<CSTRT2_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT2_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT2_A {
        match self.bits {
            false => CSTRT2_A::_0,
            true => CSTRT2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT2_A::_1
    }
}
#[doc = "Field `CSTRT2` writer - Channel 2 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT2_A, O>;
impl<'a, const O: u8> CSTRT2_W<'a, O> {
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT2_A::_0)
    }
    #[doc = "GPT32EH2.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT2_A::_1)
    }
}
#[doc = "Field `CSTRT3` reader - Channel 3 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT3_R = crate::BitReader<CSTRT3_A>;
#[doc = "Channel 3 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT3_A {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT32EH3.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<CSTRT3_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT3_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT3_A {
        match self.bits {
            false => CSTRT3_A::_0,
            true => CSTRT3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT3_A::_1
    }
}
#[doc = "Field `CSTRT3` writer - Channel 3 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT3_A, O>;
impl<'a, const O: u8> CSTRT3_W<'a, O> {
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT3_A::_0)
    }
    #[doc = "GPT32EH3.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT3_A::_1)
    }
}
#[doc = "Field `CSTRT4` reader - Channel 4 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT4_R = crate::BitReader<CSTRT4_A>;
#[doc = "Channel 4 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT4_A {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT32E4.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<CSTRT4_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT4_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT4_A {
        match self.bits {
            false => CSTRT4_A::_0,
            true => CSTRT4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT4_A::_1
    }
}
#[doc = "Field `CSTRT4` writer - Channel 4 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT4_A, O>;
impl<'a, const O: u8> CSTRT4_W<'a, O> {
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT4_A::_0)
    }
    #[doc = "GPT32E4.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT4_A::_1)
    }
}
#[doc = "Field `CSTRT5` reader - Channel 5 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT5_R = crate::BitReader<CSTRT5_A>;
#[doc = "Channel 5 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT5_A {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT32E5.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<CSTRT5_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT5_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT5_A {
        match self.bits {
            false => CSTRT5_A::_0,
            true => CSTRT5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT5_A::_1
    }
}
#[doc = "Field `CSTRT5` writer - Channel 5 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT5_A, O>;
impl<'a, const O: u8> CSTRT5_W<'a, O> {
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT5_A::_0)
    }
    #[doc = "GPT32E5.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT5_A::_1)
    }
}
#[doc = "Field `CSTRT6` reader - Channel 6 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT6_R = crate::BitReader<CSTRT6_A>;
#[doc = "Channel 6 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT6_A {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT32E6.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<CSTRT6_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT6_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT6_A {
        match self.bits {
            false => CSTRT6_A::_0,
            true => CSTRT6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT6_A::_1
    }
}
#[doc = "Field `CSTRT6` writer - Channel 6 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT6_A, O>;
impl<'a, const O: u8> CSTRT6_W<'a, O> {
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT6_A::_0)
    }
    #[doc = "GPT32E6.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT6_A::_1)
    }
}
#[doc = "Field `CSTRT7` reader - Channel 7 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT7_R = crate::BitReader<CSTRT7_A>;
#[doc = "Channel 7 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT7_A {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT32E7.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<CSTRT7_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT7_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT7_A {
        match self.bits {
            false => CSTRT7_A::_0,
            true => CSTRT7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT7_A::_1
    }
}
#[doc = "Field `CSTRT7` writer - Channel 7 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT7_A, O>;
impl<'a, const O: u8> CSTRT7_W<'a, O> {
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT7_A::_0)
    }
    #[doc = "GPT32E7.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT7_A::_1)
    }
}
#[doc = "Field `CSTRT8` reader - Channel 8 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT8_R = crate::BitReader<CSTRT8_A>;
#[doc = "Channel 8 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT8_A {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT328.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<CSTRT8_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT8_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT8_A {
        match self.bits {
            false => CSTRT8_A::_0,
            true => CSTRT8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT8_A::_1
    }
}
#[doc = "Field `CSTRT8` writer - Channel 8 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT8_A, O>;
impl<'a, const O: u8> CSTRT8_W<'a, O> {
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT8_A::_0)
    }
    #[doc = "GPT328.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT8_A::_1)
    }
}
#[doc = "Field `CSTRT9` reader - Channel 9 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT9_R = crate::BitReader<CSTRT9_A>;
#[doc = "Channel 9 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT9_A {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT329.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<CSTRT9_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT9_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT9_A {
        match self.bits {
            false => CSTRT9_A::_0,
            true => CSTRT9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT9_A::_1
    }
}
#[doc = "Field `CSTRT9` writer - Channel 9 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT9_A, O>;
impl<'a, const O: u8> CSTRT9_W<'a, O> {
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT9_A::_0)
    }
    #[doc = "GPT329.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT9_A::_1)
    }
}
#[doc = "Field `CSTRT10` reader - Channel 10 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT10_R = crate::BitReader<CSTRT10_A>;
#[doc = "Channel 10 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT10_A {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT3210.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<CSTRT10_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT10_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT10_A {
        match self.bits {
            false => CSTRT10_A::_0,
            true => CSTRT10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT10_A::_1
    }
}
#[doc = "Field `CSTRT10` writer - Channel 10 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT10_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT10_A, O>;
impl<'a, const O: u8> CSTRT10_W<'a, O> {
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT10_A::_0)
    }
    #[doc = "GPT3210.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT10_A::_1)
    }
}
#[doc = "Field `CSTRT11` reader - Channel 11 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT11_R = crate::BitReader<CSTRT11_A>;
#[doc = "Channel 11 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT11_A {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT3211.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<CSTRT11_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT11_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT11_A {
        match self.bits {
            false => CSTRT11_A::_0,
            true => CSTRT11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT11_A::_1
    }
}
#[doc = "Field `CSTRT11` writer - Channel 11 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT11_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT11_A, O>;
impl<'a, const O: u8> CSTRT11_W<'a, O> {
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT11_A::_0)
    }
    #[doc = "GPT3211.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT11_A::_1)
    }
}
#[doc = "Field `CSTRT12` reader - Channel 12 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT12_R = crate::BitReader<CSTRT12_A>;
#[doc = "Channel 12 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT12_A {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT3212.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<CSTRT12_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT12_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT12_A {
        match self.bits {
            false => CSTRT12_A::_0,
            true => CSTRT12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT12_A::_1
    }
}
#[doc = "Field `CSTRT12` writer - Channel 12 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT12_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT12_A, O>;
impl<'a, const O: u8> CSTRT12_W<'a, O> {
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT12_A::_0)
    }
    #[doc = "GPT3212.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT12_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt0(&self) -> CSTRT0_R {
        CSTRT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt1(&self) -> CSTRT1_R {
        CSTRT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt2(&self) -> CSTRT2_R {
        CSTRT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt3(&self) -> CSTRT3_R {
        CSTRT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt4(&self) -> CSTRT4_R {
        CSTRT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt5(&self) -> CSTRT5_R {
        CSTRT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt6(&self) -> CSTRT6_R {
        CSTRT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt7(&self) -> CSTRT7_R {
        CSTRT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 8 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt8(&self) -> CSTRT8_R {
        CSTRT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 9 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt9(&self) -> CSTRT9_R {
        CSTRT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 10 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt10(&self) -> CSTRT10_R {
        CSTRT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 11 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt11(&self) -> CSTRT11_R {
        CSTRT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 12 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt12(&self) -> CSTRT12_R {
        CSTRT12_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    #[must_use]
    pub fn cstrt0(&mut self) -> CSTRT0_W<0> {
        CSTRT0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    #[must_use]
    pub fn cstrt1(&mut self) -> CSTRT1_W<1> {
        CSTRT1_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    #[must_use]
    pub fn cstrt2(&mut self) -> CSTRT2_W<2> {
        CSTRT2_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    #[must_use]
    pub fn cstrt3(&mut self) -> CSTRT3_W<3> {
        CSTRT3_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    #[must_use]
    pub fn cstrt4(&mut self) -> CSTRT4_W<4> {
        CSTRT4_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    #[must_use]
    pub fn cstrt5(&mut self) -> CSTRT5_W<5> {
        CSTRT5_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    #[must_use]
    pub fn cstrt6(&mut self) -> CSTRT6_W<6> {
        CSTRT6_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    #[must_use]
    pub fn cstrt7(&mut self) -> CSTRT7_W<7> {
        CSTRT7_W::new(self)
    }
    #[doc = "Bit 8 - Channel 8 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    #[must_use]
    pub fn cstrt8(&mut self) -> CSTRT8_W<8> {
        CSTRT8_W::new(self)
    }
    #[doc = "Bit 9 - Channel 9 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    #[must_use]
    pub fn cstrt9(&mut self) -> CSTRT9_W<9> {
        CSTRT9_W::new(self)
    }
    #[doc = "Bit 10 - Channel 10 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    #[must_use]
    pub fn cstrt10(&mut self) -> CSTRT10_W<10> {
        CSTRT10_W::new(self)
    }
    #[doc = "Bit 11 - Channel 11 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    #[must_use]
    pub fn cstrt11(&mut self) -> CSTRT11_W<11> {
        CSTRT11_W::new(self)
    }
    #[doc = "Bit 12 - Channel 12 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    #[must_use]
    pub fn cstrt12(&mut self) -> CSTRT12_W<12> {
        CSTRT12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Software Start Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtstr](index.html) module"]
pub struct GTSTR_SPEC;
impl crate::RegisterSpec for GTSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtstr::R](R) reader structure"]
impl crate::Readable for GTSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtstr::W](W) writer structure"]
impl crate::Writable for GTSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTSTR to value 0"]
impl crate::Resettable for GTSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
