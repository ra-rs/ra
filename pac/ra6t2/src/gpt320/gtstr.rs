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
#[doc = "Field `CSTRT0` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT0_R = crate::BitReader<CSTRT0_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT0_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
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
#[doc = "Field `CSTRT0` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT0_A, O>;
impl<'a, const O: u8> CSTRT0_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT0_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT0_A::_1)
    }
}
#[doc = "Field `CSTRT1` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT1_R = crate::BitReader<CSTRT1_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT1_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
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
#[doc = "Field `CSTRT1` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT1_A, O>;
impl<'a, const O: u8> CSTRT1_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT1_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT1_A::_1)
    }
}
#[doc = "Field `CSTRT2` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT2_R = crate::BitReader<CSTRT2_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT2_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
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
#[doc = "Field `CSTRT2` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT2_A, O>;
impl<'a, const O: u8> CSTRT2_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT2_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT2_A::_1)
    }
}
#[doc = "Field `CSTRT3` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT3_R = crate::BitReader<CSTRT3_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT3_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
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
#[doc = "Field `CSTRT3` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT3_A, O>;
impl<'a, const O: u8> CSTRT3_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT3_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT3_A::_1)
    }
}
#[doc = "Field `CSTRT4` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT4_R = crate::BitReader<CSTRT4_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT4_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
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
#[doc = "Field `CSTRT4` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT4_A, O>;
impl<'a, const O: u8> CSTRT4_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT4_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT4_A::_1)
    }
}
#[doc = "Field `CSTRT5` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT5_R = crate::BitReader<CSTRT5_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT5_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
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
#[doc = "Field `CSTRT5` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT5_A, O>;
impl<'a, const O: u8> CSTRT5_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT5_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT5_A::_1)
    }
}
#[doc = "Field `CSTRT6` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT6_R = crate::BitReader<CSTRT6_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT6_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
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
#[doc = "Field `CSTRT6` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT6_A, O>;
impl<'a, const O: u8> CSTRT6_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT6_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT6_A::_1)
    }
}
#[doc = "Field `CSTRT7` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT7_R = crate::BitReader<CSTRT7_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT7_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
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
#[doc = "Field `CSTRT7` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT7_A, O>;
impl<'a, const O: u8> CSTRT7_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT7_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT7_A::_1)
    }
}
#[doc = "Field `CSTRT8` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT8_R = crate::BitReader<CSTRT8_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT8_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
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
#[doc = "Field `CSTRT8` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT8_A, O>;
impl<'a, const O: u8> CSTRT8_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT8_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT8_A::_1)
    }
}
#[doc = "Field `CSTRT9` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT9_R = crate::BitReader<CSTRT9_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT9_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
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
#[doc = "Field `CSTRT9` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT9_A, O>;
impl<'a, const O: u8> CSTRT9_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT9_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT9_A::_1)
    }
}
#[doc = "Field `CSTRT10` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT10_R = crate::BitReader<CSTRT10_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT10_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
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
#[doc = "Field `CSTRT10` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT10_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT10_A, O>;
impl<'a, const O: u8> CSTRT10_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT10_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT10_A::_1)
    }
}
#[doc = "Field `CSTRT11` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT11_R = crate::BitReader<CSTRT11_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT11_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
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
#[doc = "Field `CSTRT11` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT11_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT11_A, O>;
impl<'a, const O: u8> CSTRT11_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT11_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT11_A::_1)
    }
}
#[doc = "Field `CSTRT12` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT12_R = crate::BitReader<CSTRT12_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT12_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
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
#[doc = "Field `CSTRT12` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT12_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT12_A, O>;
impl<'a, const O: u8> CSTRT12_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT12_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT12_A::_1)
    }
}
#[doc = "Field `CSTRT13` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT13_R = crate::BitReader<CSTRT13_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT13_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
    _1 = 1,
}
impl From<CSTRT13_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT13_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT13_A {
        match self.bits {
            false => CSTRT13_A::_0,
            true => CSTRT13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT13_A::_1
    }
}
#[doc = "Field `CSTRT13` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT13_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT13_A, O>;
impl<'a, const O: u8> CSTRT13_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT13_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT13_A::_1)
    }
}
#[doc = "Field `CSTRT14` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT14_R = crate::BitReader<CSTRT14_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT14_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
    _1 = 1,
}
impl From<CSTRT14_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT14_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT14_A {
        match self.bits {
            false => CSTRT14_A::_0,
            true => CSTRT14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT14_A::_1
    }
}
#[doc = "Field `CSTRT14` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT14_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT14_A, O>;
impl<'a, const O: u8> CSTRT14_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT14_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT14_A::_1)
    }
}
#[doc = "Field `CSTRT15` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT15_R = crate::BitReader<CSTRT15_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT15_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
    _1 = 1,
}
impl From<CSTRT15_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT15_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT15_A {
        match self.bits {
            false => CSTRT15_A::_0,
            true => CSTRT15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT15_A::_1
    }
}
#[doc = "Field `CSTRT15` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT15_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT15_A, O>;
impl<'a, const O: u8> CSTRT15_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT15_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT15_A::_1)
    }
}
#[doc = "Field `CSTRT16` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT16_R = crate::BitReader<CSTRT16_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT16_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
    _1 = 1,
}
impl From<CSTRT16_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT16_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT16_A {
        match self.bits {
            false => CSTRT16_A::_0,
            true => CSTRT16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT16_A::_1
    }
}
#[doc = "Field `CSTRT16` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT16_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT16_A, O>;
impl<'a, const O: u8> CSTRT16_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT16_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT16_A::_1)
    }
}
#[doc = "Field `CSTRT17` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT17_R = crate::BitReader<CSTRT17_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT17_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
    _1 = 1,
}
impl From<CSTRT17_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT17_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT17_A {
        match self.bits {
            false => CSTRT17_A::_0,
            true => CSTRT17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT17_A::_1
    }
}
#[doc = "Field `CSTRT17` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT17_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT17_A, O>;
impl<'a, const O: u8> CSTRT17_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT17_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT17_A::_1)
    }
}
#[doc = "Field `CSTRT18` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT18_R = crate::BitReader<CSTRT18_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT18_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
    _1 = 1,
}
impl From<CSTRT18_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT18_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT18_A {
        match self.bits {
            false => CSTRT18_A::_0,
            true => CSTRT18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT18_A::_1
    }
}
#[doc = "Field `CSTRT18` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT18_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT18_A, O>;
impl<'a, const O: u8> CSTRT18_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT18_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT18_A::_1)
    }
}
#[doc = "Field `CSTRT19` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT19_R = crate::BitReader<CSTRT19_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT19_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
    _1 = 1,
}
impl From<CSTRT19_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT19_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT19_A {
        match self.bits {
            false => CSTRT19_A::_0,
            true => CSTRT19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT19_A::_1
    }
}
#[doc = "Field `CSTRT19` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT19_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT19_A, O>;
impl<'a, const O: u8> CSTRT19_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT19_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT19_A::_1)
    }
}
#[doc = "Field `CSTRT20` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT20_R = crate::BitReader<CSTRT20_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT20_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
    _1 = 1,
}
impl From<CSTRT20_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT20_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT20_A {
        match self.bits {
            false => CSTRT20_A::_0,
            true => CSTRT20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT20_A::_1
    }
}
#[doc = "Field `CSTRT20` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT20_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT20_A, O>;
impl<'a, const O: u8> CSTRT20_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT20_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT20_A::_1)
    }
}
#[doc = "Field `CSTRT21` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT21_R = crate::BitReader<CSTRT21_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT21_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
    _1 = 1,
}
impl From<CSTRT21_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT21_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT21_A {
        match self.bits {
            false => CSTRT21_A::_0,
            true => CSTRT21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT21_A::_1
    }
}
#[doc = "Field `CSTRT21` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT21_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT21_A, O>;
impl<'a, const O: u8> CSTRT21_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT21_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT21_A::_1)
    }
}
#[doc = "Field `CSTRT22` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT22_R = crate::BitReader<CSTRT22_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT22_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
    _1 = 1,
}
impl From<CSTRT22_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT22_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT22_A {
        match self.bits {
            false => CSTRT22_A::_0,
            true => CSTRT22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT22_A::_1
    }
}
#[doc = "Field `CSTRT22` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT22_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT22_A, O>;
impl<'a, const O: u8> CSTRT22_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT22_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT22_A::_1)
    }
}
#[doc = "Field `CSTRT23` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT23_R = crate::BitReader<CSTRT23_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT23_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
    _1 = 1,
}
impl From<CSTRT23_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT23_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT23_A {
        match self.bits {
            false => CSTRT23_A::_0,
            true => CSTRT23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT23_A::_1
    }
}
#[doc = "Field `CSTRT23` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT23_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT23_A, O>;
impl<'a, const O: u8> CSTRT23_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT23_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT23_A::_1)
    }
}
#[doc = "Field `CSTRT24` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT24_R = crate::BitReader<CSTRT24_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT24_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
    _1 = 1,
}
impl From<CSTRT24_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT24_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT24_A {
        match self.bits {
            false => CSTRT24_A::_0,
            true => CSTRT24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT24_A::_1
    }
}
#[doc = "Field `CSTRT24` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT24_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT24_A, O>;
impl<'a, const O: u8> CSTRT24_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT24_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT24_A::_1)
    }
}
#[doc = "Field `CSTRT25` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT25_R = crate::BitReader<CSTRT25_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT25_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
    _1 = 1,
}
impl From<CSTRT25_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT25_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT25_A {
        match self.bits {
            false => CSTRT25_A::_0,
            true => CSTRT25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT25_A::_1
    }
}
#[doc = "Field `CSTRT25` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT25_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT25_A, O>;
impl<'a, const O: u8> CSTRT25_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT25_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT25_A::_1)
    }
}
#[doc = "Field `CSTRT26` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT26_R = crate::BitReader<CSTRT26_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT26_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
    _1 = 1,
}
impl From<CSTRT26_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT26_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT26_A {
        match self.bits {
            false => CSTRT26_A::_0,
            true => CSTRT26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT26_A::_1
    }
}
#[doc = "Field `CSTRT26` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT26_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT26_A, O>;
impl<'a, const O: u8> CSTRT26_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT26_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT26_A::_1)
    }
}
#[doc = "Field `CSTRT27` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT27_R = crate::BitReader<CSTRT27_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT27_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
    _1 = 1,
}
impl From<CSTRT27_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT27_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT27_A {
        match self.bits {
            false => CSTRT27_A::_0,
            true => CSTRT27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT27_A::_1
    }
}
#[doc = "Field `CSTRT27` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT27_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT27_A, O>;
impl<'a, const O: u8> CSTRT27_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT27_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT27_A::_1)
    }
}
#[doc = "Field `CSTRT28` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT28_R = crate::BitReader<CSTRT28_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT28_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
    _1 = 1,
}
impl From<CSTRT28_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT28_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT28_A {
        match self.bits {
            false => CSTRT28_A::_0,
            true => CSTRT28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT28_A::_1
    }
}
#[doc = "Field `CSTRT28` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT28_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT28_A, O>;
impl<'a, const O: u8> CSTRT28_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT28_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT28_A::_1)
    }
}
#[doc = "Field `CSTRT29` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT29_R = crate::BitReader<CSTRT29_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT29_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
    _1 = 1,
}
impl From<CSTRT29_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT29_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT29_A {
        match self.bits {
            false => CSTRT29_A::_0,
            true => CSTRT29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT29_A::_1
    }
}
#[doc = "Field `CSTRT29` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT29_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT29_A, O>;
impl<'a, const O: u8> CSTRT29_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT29_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT29_A::_1)
    }
}
#[doc = "Field `CSTRT30` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT30_R = crate::BitReader<CSTRT30_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT30_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
    _1 = 1,
}
impl From<CSTRT30_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT30_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT30_A {
        match self.bits {
            false => CSTRT30_A::_0,
            true => CSTRT30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT30_A::_1
    }
}
#[doc = "Field `CSTRT30` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT30_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT30_A, O>;
impl<'a, const O: u8> CSTRT30_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT30_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT30_A::_1)
    }
}
#[doc = "Field `CSTRT31` reader - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT31_R = crate::BitReader<CSTRT31_A>;
#[doc = "Channel n GTCNT Count Start (n is the same as the bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT31_A {
    #[doc = "0: GTCNT counter is not started"]
    _0 = 0,
    #[doc = "1: GTCNT counter is started"]
    _1 = 1,
}
impl From<CSTRT31_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT31_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTRT31_A {
        match self.bits {
            false => CSTRT31_A::_0,
            true => CSTRT31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT31_A::_1
    }
}
#[doc = "Field `CSTRT31` writer - Channel n GTCNT Count Start (n is the same as the bit position value)"]
pub type CSTRT31_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSTR_SPEC, CSTRT31_A, O>;
impl<'a, const O: u8> CSTRT31_W<'a, O> {
    #[doc = "GTCNT counter is not started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTRT31_A::_0)
    }
    #[doc = "GTCNT counter is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTRT31_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt0(&self) -> CSTRT0_R {
        CSTRT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt1(&self) -> CSTRT1_R {
        CSTRT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt2(&self) -> CSTRT2_R {
        CSTRT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt3(&self) -> CSTRT3_R {
        CSTRT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt4(&self) -> CSTRT4_R {
        CSTRT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt5(&self) -> CSTRT5_R {
        CSTRT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt6(&self) -> CSTRT6_R {
        CSTRT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt7(&self) -> CSTRT7_R {
        CSTRT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt8(&self) -> CSTRT8_R {
        CSTRT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt9(&self) -> CSTRT9_R {
        CSTRT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt10(&self) -> CSTRT10_R {
        CSTRT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt11(&self) -> CSTRT11_R {
        CSTRT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt12(&self) -> CSTRT12_R {
        CSTRT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt13(&self) -> CSTRT13_R {
        CSTRT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt14(&self) -> CSTRT14_R {
        CSTRT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt15(&self) -> CSTRT15_R {
        CSTRT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt16(&self) -> CSTRT16_R {
        CSTRT16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt17(&self) -> CSTRT17_R {
        CSTRT17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt18(&self) -> CSTRT18_R {
        CSTRT18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt19(&self) -> CSTRT19_R {
        CSTRT19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt20(&self) -> CSTRT20_R {
        CSTRT20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt21(&self) -> CSTRT21_R {
        CSTRT21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt22(&self) -> CSTRT22_R {
        CSTRT22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt23(&self) -> CSTRT23_R {
        CSTRT23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt24(&self) -> CSTRT24_R {
        CSTRT24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt25(&self) -> CSTRT25_R {
        CSTRT25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt26(&self) -> CSTRT26_R {
        CSTRT26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt27(&self) -> CSTRT27_R {
        CSTRT27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt28(&self) -> CSTRT28_R {
        CSTRT28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt29(&self) -> CSTRT29_R {
        CSTRT29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt30(&self) -> CSTRT30_R {
        CSTRT30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    pub fn cstrt31(&self) -> CSTRT31_R {
        CSTRT31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt0(&mut self) -> CSTRT0_W<0> {
        CSTRT0_W::new(self)
    }
    #[doc = "Bit 1 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt1(&mut self) -> CSTRT1_W<1> {
        CSTRT1_W::new(self)
    }
    #[doc = "Bit 2 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt2(&mut self) -> CSTRT2_W<2> {
        CSTRT2_W::new(self)
    }
    #[doc = "Bit 3 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt3(&mut self) -> CSTRT3_W<3> {
        CSTRT3_W::new(self)
    }
    #[doc = "Bit 4 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt4(&mut self) -> CSTRT4_W<4> {
        CSTRT4_W::new(self)
    }
    #[doc = "Bit 5 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt5(&mut self) -> CSTRT5_W<5> {
        CSTRT5_W::new(self)
    }
    #[doc = "Bit 6 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt6(&mut self) -> CSTRT6_W<6> {
        CSTRT6_W::new(self)
    }
    #[doc = "Bit 7 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt7(&mut self) -> CSTRT7_W<7> {
        CSTRT7_W::new(self)
    }
    #[doc = "Bit 8 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt8(&mut self) -> CSTRT8_W<8> {
        CSTRT8_W::new(self)
    }
    #[doc = "Bit 9 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt9(&mut self) -> CSTRT9_W<9> {
        CSTRT9_W::new(self)
    }
    #[doc = "Bit 10 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt10(&mut self) -> CSTRT10_W<10> {
        CSTRT10_W::new(self)
    }
    #[doc = "Bit 11 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt11(&mut self) -> CSTRT11_W<11> {
        CSTRT11_W::new(self)
    }
    #[doc = "Bit 12 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt12(&mut self) -> CSTRT12_W<12> {
        CSTRT12_W::new(self)
    }
    #[doc = "Bit 13 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt13(&mut self) -> CSTRT13_W<13> {
        CSTRT13_W::new(self)
    }
    #[doc = "Bit 14 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt14(&mut self) -> CSTRT14_W<14> {
        CSTRT14_W::new(self)
    }
    #[doc = "Bit 15 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt15(&mut self) -> CSTRT15_W<15> {
        CSTRT15_W::new(self)
    }
    #[doc = "Bit 16 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt16(&mut self) -> CSTRT16_W<16> {
        CSTRT16_W::new(self)
    }
    #[doc = "Bit 17 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt17(&mut self) -> CSTRT17_W<17> {
        CSTRT17_W::new(self)
    }
    #[doc = "Bit 18 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt18(&mut self) -> CSTRT18_W<18> {
        CSTRT18_W::new(self)
    }
    #[doc = "Bit 19 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt19(&mut self) -> CSTRT19_W<19> {
        CSTRT19_W::new(self)
    }
    #[doc = "Bit 20 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt20(&mut self) -> CSTRT20_W<20> {
        CSTRT20_W::new(self)
    }
    #[doc = "Bit 21 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt21(&mut self) -> CSTRT21_W<21> {
        CSTRT21_W::new(self)
    }
    #[doc = "Bit 22 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt22(&mut self) -> CSTRT22_W<22> {
        CSTRT22_W::new(self)
    }
    #[doc = "Bit 23 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt23(&mut self) -> CSTRT23_W<23> {
        CSTRT23_W::new(self)
    }
    #[doc = "Bit 24 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt24(&mut self) -> CSTRT24_W<24> {
        CSTRT24_W::new(self)
    }
    #[doc = "Bit 25 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt25(&mut self) -> CSTRT25_W<25> {
        CSTRT25_W::new(self)
    }
    #[doc = "Bit 26 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt26(&mut self) -> CSTRT26_W<26> {
        CSTRT26_W::new(self)
    }
    #[doc = "Bit 27 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt27(&mut self) -> CSTRT27_W<27> {
        CSTRT27_W::new(self)
    }
    #[doc = "Bit 28 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt28(&mut self) -> CSTRT28_W<28> {
        CSTRT28_W::new(self)
    }
    #[doc = "Bit 29 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt29(&mut self) -> CSTRT29_W<29> {
        CSTRT29_W::new(self)
    }
    #[doc = "Bit 30 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt30(&mut self) -> CSTRT30_W<30> {
        CSTRT30_W::new(self)
    }
    #[doc = "Bit 31 - Channel n GTCNT Count Start (n is the same as the bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt31(&mut self) -> CSTRT31_W<31> {
        CSTRT31_W::new(self)
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
