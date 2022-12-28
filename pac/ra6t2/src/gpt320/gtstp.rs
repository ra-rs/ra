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
#[doc = "Field `CSTOP0` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP0_R = crate::BitReader<CSTOP0_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP0_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
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
#[doc = "Field `CSTOP0` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP0_A, O>;
impl<'a, const O: u8> CSTOP0_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP0_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP0_A::_1)
    }
}
#[doc = "Field `CSTOP1` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP1_R = crate::BitReader<CSTOP1_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP1_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
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
#[doc = "Field `CSTOP1` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP1_A, O>;
impl<'a, const O: u8> CSTOP1_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP1_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP1_A::_1)
    }
}
#[doc = "Field `CSTOP2` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP2_R = crate::BitReader<CSTOP2_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP2_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
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
#[doc = "Field `CSTOP2` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP2_A, O>;
impl<'a, const O: u8> CSTOP2_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP2_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP2_A::_1)
    }
}
#[doc = "Field `CSTOP3` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP3_R = crate::BitReader<CSTOP3_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP3_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
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
#[doc = "Field `CSTOP3` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP3_A, O>;
impl<'a, const O: u8> CSTOP3_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP3_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP3_A::_1)
    }
}
#[doc = "Field `CSTOP4` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP4_R = crate::BitReader<CSTOP4_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP4_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
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
#[doc = "Field `CSTOP4` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP4_A, O>;
impl<'a, const O: u8> CSTOP4_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP4_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP4_A::_1)
    }
}
#[doc = "Field `CSTOP5` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP5_R = crate::BitReader<CSTOP5_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP5_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
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
#[doc = "Field `CSTOP5` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP5_A, O>;
impl<'a, const O: u8> CSTOP5_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP5_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP5_A::_1)
    }
}
#[doc = "Field `CSTOP6` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP6_R = crate::BitReader<CSTOP6_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP6_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
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
#[doc = "Field `CSTOP6` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP6_A, O>;
impl<'a, const O: u8> CSTOP6_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP6_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP6_A::_1)
    }
}
#[doc = "Field `CSTOP7` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP7_R = crate::BitReader<CSTOP7_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP7_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
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
#[doc = "Field `CSTOP7` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP7_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP7_A, O>;
impl<'a, const O: u8> CSTOP7_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP7_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP7_A::_1)
    }
}
#[doc = "Field `CSTOP8` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP8_R = crate::BitReader<CSTOP8_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP8_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
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
#[doc = "Field `CSTOP8` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP8_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP8_A, O>;
impl<'a, const O: u8> CSTOP8_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP8_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP8_A::_1)
    }
}
#[doc = "Field `CSTOP9` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP9_R = crate::BitReader<CSTOP9_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP9_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
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
#[doc = "Field `CSTOP9` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP9_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP9_A, O>;
impl<'a, const O: u8> CSTOP9_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP9_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP9_A::_1)
    }
}
#[doc = "Field `CSTOP10` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP10_R = crate::BitReader<CSTOP10_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP10_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
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
#[doc = "Field `CSTOP10` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP10_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP10_A, O>;
impl<'a, const O: u8> CSTOP10_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP10_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP10_A::_1)
    }
}
#[doc = "Field `CSTOP11` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP11_R = crate::BitReader<CSTOP11_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP11_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
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
#[doc = "Field `CSTOP11` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP11_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP11_A, O>;
impl<'a, const O: u8> CSTOP11_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP11_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP11_A::_1)
    }
}
#[doc = "Field `CSTOP12` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP12_R = crate::BitReader<CSTOP12_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP12_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
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
#[doc = "Field `CSTOP12` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP12_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP12_A, O>;
impl<'a, const O: u8> CSTOP12_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP12_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP12_A::_1)
    }
}
#[doc = "Field `CSTOP13` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP13_R = crate::BitReader<CSTOP13_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP13_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
    _1 = 1,
}
impl From<CSTOP13_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP13_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP13_A {
        match self.bits {
            false => CSTOP13_A::_0,
            true => CSTOP13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP13_A::_1
    }
}
#[doc = "Field `CSTOP13` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP13_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP13_A, O>;
impl<'a, const O: u8> CSTOP13_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP13_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP13_A::_1)
    }
}
#[doc = "Field `CSTOP14` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP14_R = crate::BitReader<CSTOP14_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP14_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
    _1 = 1,
}
impl From<CSTOP14_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP14_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP14_A {
        match self.bits {
            false => CSTOP14_A::_0,
            true => CSTOP14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP14_A::_1
    }
}
#[doc = "Field `CSTOP14` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP14_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP14_A, O>;
impl<'a, const O: u8> CSTOP14_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP14_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP14_A::_1)
    }
}
#[doc = "Field `CSTOP15` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP15_R = crate::BitReader<CSTOP15_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP15_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
    _1 = 1,
}
impl From<CSTOP15_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP15_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP15_A {
        match self.bits {
            false => CSTOP15_A::_0,
            true => CSTOP15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP15_A::_1
    }
}
#[doc = "Field `CSTOP15` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP15_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP15_A, O>;
impl<'a, const O: u8> CSTOP15_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP15_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP15_A::_1)
    }
}
#[doc = "Field `CSTOP16` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP16_R = crate::BitReader<CSTOP16_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP16_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
    _1 = 1,
}
impl From<CSTOP16_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP16_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP16_A {
        match self.bits {
            false => CSTOP16_A::_0,
            true => CSTOP16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP16_A::_1
    }
}
#[doc = "Field `CSTOP16` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP16_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP16_A, O>;
impl<'a, const O: u8> CSTOP16_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP16_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP16_A::_1)
    }
}
#[doc = "Field `CSTOP17` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP17_R = crate::BitReader<CSTOP17_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP17_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
    _1 = 1,
}
impl From<CSTOP17_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP17_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP17_A {
        match self.bits {
            false => CSTOP17_A::_0,
            true => CSTOP17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP17_A::_1
    }
}
#[doc = "Field `CSTOP17` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP17_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP17_A, O>;
impl<'a, const O: u8> CSTOP17_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP17_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP17_A::_1)
    }
}
#[doc = "Field `CSTOP18` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP18_R = crate::BitReader<CSTOP18_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP18_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
    _1 = 1,
}
impl From<CSTOP18_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP18_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP18_A {
        match self.bits {
            false => CSTOP18_A::_0,
            true => CSTOP18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP18_A::_1
    }
}
#[doc = "Field `CSTOP18` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP18_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP18_A, O>;
impl<'a, const O: u8> CSTOP18_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP18_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP18_A::_1)
    }
}
#[doc = "Field `CSTOP19` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP19_R = crate::BitReader<CSTOP19_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP19_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
    _1 = 1,
}
impl From<CSTOP19_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP19_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP19_A {
        match self.bits {
            false => CSTOP19_A::_0,
            true => CSTOP19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP19_A::_1
    }
}
#[doc = "Field `CSTOP19` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP19_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP19_A, O>;
impl<'a, const O: u8> CSTOP19_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP19_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP19_A::_1)
    }
}
#[doc = "Field `CSTOP20` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP20_R = crate::BitReader<CSTOP20_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP20_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
    _1 = 1,
}
impl From<CSTOP20_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP20_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP20_A {
        match self.bits {
            false => CSTOP20_A::_0,
            true => CSTOP20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP20_A::_1
    }
}
#[doc = "Field `CSTOP20` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP20_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP20_A, O>;
impl<'a, const O: u8> CSTOP20_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP20_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP20_A::_1)
    }
}
#[doc = "Field `CSTOP21` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP21_R = crate::BitReader<CSTOP21_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP21_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
    _1 = 1,
}
impl From<CSTOP21_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP21_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP21_A {
        match self.bits {
            false => CSTOP21_A::_0,
            true => CSTOP21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP21_A::_1
    }
}
#[doc = "Field `CSTOP21` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP21_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP21_A, O>;
impl<'a, const O: u8> CSTOP21_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP21_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP21_A::_1)
    }
}
#[doc = "Field `CSTOP22` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP22_R = crate::BitReader<CSTOP22_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP22_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
    _1 = 1,
}
impl From<CSTOP22_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP22_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP22_A {
        match self.bits {
            false => CSTOP22_A::_0,
            true => CSTOP22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP22_A::_1
    }
}
#[doc = "Field `CSTOP22` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP22_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP22_A, O>;
impl<'a, const O: u8> CSTOP22_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP22_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP22_A::_1)
    }
}
#[doc = "Field `CSTOP23` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP23_R = crate::BitReader<CSTOP23_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP23_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
    _1 = 1,
}
impl From<CSTOP23_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP23_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP23_A {
        match self.bits {
            false => CSTOP23_A::_0,
            true => CSTOP23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP23_A::_1
    }
}
#[doc = "Field `CSTOP23` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP23_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP23_A, O>;
impl<'a, const O: u8> CSTOP23_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP23_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP23_A::_1)
    }
}
#[doc = "Field `CSTOP24` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP24_R = crate::BitReader<CSTOP24_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP24_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
    _1 = 1,
}
impl From<CSTOP24_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP24_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP24_A {
        match self.bits {
            false => CSTOP24_A::_0,
            true => CSTOP24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP24_A::_1
    }
}
#[doc = "Field `CSTOP24` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP24_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP24_A, O>;
impl<'a, const O: u8> CSTOP24_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP24_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP24_A::_1)
    }
}
#[doc = "Field `CSTOP25` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP25_R = crate::BitReader<CSTOP25_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP25_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
    _1 = 1,
}
impl From<CSTOP25_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP25_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP25_A {
        match self.bits {
            false => CSTOP25_A::_0,
            true => CSTOP25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP25_A::_1
    }
}
#[doc = "Field `CSTOP25` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP25_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP25_A, O>;
impl<'a, const O: u8> CSTOP25_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP25_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP25_A::_1)
    }
}
#[doc = "Field `CSTOP26` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP26_R = crate::BitReader<CSTOP26_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP26_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
    _1 = 1,
}
impl From<CSTOP26_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP26_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP26_A {
        match self.bits {
            false => CSTOP26_A::_0,
            true => CSTOP26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP26_A::_1
    }
}
#[doc = "Field `CSTOP26` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP26_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP26_A, O>;
impl<'a, const O: u8> CSTOP26_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP26_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP26_A::_1)
    }
}
#[doc = "Field `CSTOP27` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP27_R = crate::BitReader<CSTOP27_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP27_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
    _1 = 1,
}
impl From<CSTOP27_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP27_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP27_A {
        match self.bits {
            false => CSTOP27_A::_0,
            true => CSTOP27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP27_A::_1
    }
}
#[doc = "Field `CSTOP27` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP27_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP27_A, O>;
impl<'a, const O: u8> CSTOP27_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP27_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP27_A::_1)
    }
}
#[doc = "Field `CSTOP28` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP28_R = crate::BitReader<CSTOP28_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP28_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
    _1 = 1,
}
impl From<CSTOP28_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP28_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP28_A {
        match self.bits {
            false => CSTOP28_A::_0,
            true => CSTOP28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP28_A::_1
    }
}
#[doc = "Field `CSTOP28` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP28_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP28_A, O>;
impl<'a, const O: u8> CSTOP28_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP28_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP28_A::_1)
    }
}
#[doc = "Field `CSTOP29` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP29_R = crate::BitReader<CSTOP29_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP29_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
    _1 = 1,
}
impl From<CSTOP29_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP29_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP29_A {
        match self.bits {
            false => CSTOP29_A::_0,
            true => CSTOP29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP29_A::_1
    }
}
#[doc = "Field `CSTOP29` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP29_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP29_A, O>;
impl<'a, const O: u8> CSTOP29_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP29_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP29_A::_1)
    }
}
#[doc = "Field `CSTOP30` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP30_R = crate::BitReader<CSTOP30_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP30_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
    _1 = 1,
}
impl From<CSTOP30_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP30_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP30_A {
        match self.bits {
            false => CSTOP30_A::_0,
            true => CSTOP30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP30_A::_1
    }
}
#[doc = "Field `CSTOP30` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP30_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP30_A, O>;
impl<'a, const O: u8> CSTOP30_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP30_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP30_A::_1)
    }
}
#[doc = "Field `CSTOP31` reader - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP31_R = crate::BitReader<CSTOP31_A>;
#[doc = "Channel n GTCNT Count Stop (n is the same as the bit position value)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP31_A {
    #[doc = "0: GTCNT counter is not stopped"]
    _0 = 0,
    #[doc = "1: GTCNT counter stopped"]
    _1 = 1,
}
impl From<CSTOP31_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP31_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOP31_A {
        match self.bits {
            false => CSTOP31_A::_0,
            true => CSTOP31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP31_A::_1
    }
}
#[doc = "Field `CSTOP31` writer - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
pub type CSTOP31_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTP_SPEC, CSTOP31_A, O>;
impl<'a, const O: u8> CSTOP31_W<'a, O> {
    #[doc = "GTCNT counter is not stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTOP31_A::_0)
    }
    #[doc = "GTCNT counter stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOP31_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop0(&self) -> CSTOP0_R {
        CSTOP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop1(&self) -> CSTOP1_R {
        CSTOP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop2(&self) -> CSTOP2_R {
        CSTOP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop3(&self) -> CSTOP3_R {
        CSTOP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop4(&self) -> CSTOP4_R {
        CSTOP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop5(&self) -> CSTOP5_R {
        CSTOP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop6(&self) -> CSTOP6_R {
        CSTOP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop7(&self) -> CSTOP7_R {
        CSTOP7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop8(&self) -> CSTOP8_R {
        CSTOP8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop9(&self) -> CSTOP9_R {
        CSTOP9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop10(&self) -> CSTOP10_R {
        CSTOP10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop11(&self) -> CSTOP11_R {
        CSTOP11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop12(&self) -> CSTOP12_R {
        CSTOP12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop13(&self) -> CSTOP13_R {
        CSTOP13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop14(&self) -> CSTOP14_R {
        CSTOP14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop15(&self) -> CSTOP15_R {
        CSTOP15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop16(&self) -> CSTOP16_R {
        CSTOP16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop17(&self) -> CSTOP17_R {
        CSTOP17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop18(&self) -> CSTOP18_R {
        CSTOP18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop19(&self) -> CSTOP19_R {
        CSTOP19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop20(&self) -> CSTOP20_R {
        CSTOP20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop21(&self) -> CSTOP21_R {
        CSTOP21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop22(&self) -> CSTOP22_R {
        CSTOP22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop23(&self) -> CSTOP23_R {
        CSTOP23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop24(&self) -> CSTOP24_R {
        CSTOP24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop25(&self) -> CSTOP25_R {
        CSTOP25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop26(&self) -> CSTOP26_R {
        CSTOP26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop27(&self) -> CSTOP27_R {
        CSTOP27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop28(&self) -> CSTOP28_R {
        CSTOP28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop29(&self) -> CSTOP29_R {
        CSTOP29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop30(&self) -> CSTOP30_R {
        CSTOP30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstop31(&self) -> CSTOP31_R {
        CSTOP31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop0(&mut self) -> CSTOP0_W<0> {
        CSTOP0_W::new(self)
    }
    #[doc = "Bit 1 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop1(&mut self) -> CSTOP1_W<1> {
        CSTOP1_W::new(self)
    }
    #[doc = "Bit 2 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop2(&mut self) -> CSTOP2_W<2> {
        CSTOP2_W::new(self)
    }
    #[doc = "Bit 3 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop3(&mut self) -> CSTOP3_W<3> {
        CSTOP3_W::new(self)
    }
    #[doc = "Bit 4 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop4(&mut self) -> CSTOP4_W<4> {
        CSTOP4_W::new(self)
    }
    #[doc = "Bit 5 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop5(&mut self) -> CSTOP5_W<5> {
        CSTOP5_W::new(self)
    }
    #[doc = "Bit 6 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop6(&mut self) -> CSTOP6_W<6> {
        CSTOP6_W::new(self)
    }
    #[doc = "Bit 7 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop7(&mut self) -> CSTOP7_W<7> {
        CSTOP7_W::new(self)
    }
    #[doc = "Bit 8 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop8(&mut self) -> CSTOP8_W<8> {
        CSTOP8_W::new(self)
    }
    #[doc = "Bit 9 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop9(&mut self) -> CSTOP9_W<9> {
        CSTOP9_W::new(self)
    }
    #[doc = "Bit 10 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop10(&mut self) -> CSTOP10_W<10> {
        CSTOP10_W::new(self)
    }
    #[doc = "Bit 11 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop11(&mut self) -> CSTOP11_W<11> {
        CSTOP11_W::new(self)
    }
    #[doc = "Bit 12 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop12(&mut self) -> CSTOP12_W<12> {
        CSTOP12_W::new(self)
    }
    #[doc = "Bit 13 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop13(&mut self) -> CSTOP13_W<13> {
        CSTOP13_W::new(self)
    }
    #[doc = "Bit 14 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop14(&mut self) -> CSTOP14_W<14> {
        CSTOP14_W::new(self)
    }
    #[doc = "Bit 15 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop15(&mut self) -> CSTOP15_W<15> {
        CSTOP15_W::new(self)
    }
    #[doc = "Bit 16 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop16(&mut self) -> CSTOP16_W<16> {
        CSTOP16_W::new(self)
    }
    #[doc = "Bit 17 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop17(&mut self) -> CSTOP17_W<17> {
        CSTOP17_W::new(self)
    }
    #[doc = "Bit 18 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop18(&mut self) -> CSTOP18_W<18> {
        CSTOP18_W::new(self)
    }
    #[doc = "Bit 19 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop19(&mut self) -> CSTOP19_W<19> {
        CSTOP19_W::new(self)
    }
    #[doc = "Bit 20 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop20(&mut self) -> CSTOP20_W<20> {
        CSTOP20_W::new(self)
    }
    #[doc = "Bit 21 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop21(&mut self) -> CSTOP21_W<21> {
        CSTOP21_W::new(self)
    }
    #[doc = "Bit 22 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop22(&mut self) -> CSTOP22_W<22> {
        CSTOP22_W::new(self)
    }
    #[doc = "Bit 23 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop23(&mut self) -> CSTOP23_W<23> {
        CSTOP23_W::new(self)
    }
    #[doc = "Bit 24 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop24(&mut self) -> CSTOP24_W<24> {
        CSTOP24_W::new(self)
    }
    #[doc = "Bit 25 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop25(&mut self) -> CSTOP25_W<25> {
        CSTOP25_W::new(self)
    }
    #[doc = "Bit 26 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop26(&mut self) -> CSTOP26_W<26> {
        CSTOP26_W::new(self)
    }
    #[doc = "Bit 27 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop27(&mut self) -> CSTOP27_W<27> {
        CSTOP27_W::new(self)
    }
    #[doc = "Bit 28 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop28(&mut self) -> CSTOP28_W<28> {
        CSTOP28_W::new(self)
    }
    #[doc = "Bit 29 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop29(&mut self) -> CSTOP29_W<29> {
        CSTOP29_W::new(self)
    }
    #[doc = "Bit 30 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop30(&mut self) -> CSTOP30_W<30> {
        CSTOP30_W::new(self)
    }
    #[doc = "Bit 31 - Channel n GTCNT Count Stop (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstop31(&mut self) -> CSTOP31_W<31> {
        CSTOP31_W::new(self)
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
