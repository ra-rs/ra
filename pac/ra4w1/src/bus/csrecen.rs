#[doc = "Register `CSRECEN` reader"]
pub struct R(crate::R<CSRECEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSRECEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSRECEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSRECEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSRECEN` writer"]
pub struct W(crate::W<CSRECEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSRECEN_SPEC>;
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
impl From<crate::W<CSRECEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSRECEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RECVEN0` reader - Separate Bus Recovery Cycle Insertion Enable 0"]
pub type RECVEN0_R = crate::BitReader<RECVEN0_A>;
#[doc = "Separate Bus Recovery Cycle Insertion Enable 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECVEN0_A {
    #[doc = "0: Disabled."]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<RECVEN0_A> for bool {
    #[inline(always)]
    fn from(variant: RECVEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl RECVEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECVEN0_A {
        match self.bits {
            false => RECVEN0_A::_0,
            true => RECVEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RECVEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RECVEN0_A::_1
    }
}
#[doc = "Field `RECVEN0` writer - Separate Bus Recovery Cycle Insertion Enable 0"]
pub type RECVEN0_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSRECEN_SPEC, RECVEN0_A, O>;
impl<'a, const O: u8> RECVEN0_W<'a, O> {
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RECVEN0_A::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RECVEN0_A::_1)
    }
}
#[doc = "Field `RECVEN1` reader - Separate Bus Recovery Cycle Insertion Enable 1"]
pub type RECVEN1_R = crate::BitReader<RECVEN1_A>;
#[doc = "Separate Bus Recovery Cycle Insertion Enable 1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECVEN1_A {
    #[doc = "0: Disabled."]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<RECVEN1_A> for bool {
    #[inline(always)]
    fn from(variant: RECVEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl RECVEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECVEN1_A {
        match self.bits {
            false => RECVEN1_A::_0,
            true => RECVEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RECVEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RECVEN1_A::_1
    }
}
#[doc = "Field `RECVEN1` writer - Separate Bus Recovery Cycle Insertion Enable 1"]
pub type RECVEN1_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSRECEN_SPEC, RECVEN1_A, O>;
impl<'a, const O: u8> RECVEN1_W<'a, O> {
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RECVEN1_A::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RECVEN1_A::_1)
    }
}
#[doc = "Field `RECVEN2` reader - Separate Bus Recovery Cycle Insertion Enable 2"]
pub type RECVEN2_R = crate::BitReader<RECVEN2_A>;
#[doc = "Separate Bus Recovery Cycle Insertion Enable 2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECVEN2_A {
    #[doc = "0: Disabled."]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<RECVEN2_A> for bool {
    #[inline(always)]
    fn from(variant: RECVEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl RECVEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECVEN2_A {
        match self.bits {
            false => RECVEN2_A::_0,
            true => RECVEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RECVEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RECVEN2_A::_1
    }
}
#[doc = "Field `RECVEN2` writer - Separate Bus Recovery Cycle Insertion Enable 2"]
pub type RECVEN2_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSRECEN_SPEC, RECVEN2_A, O>;
impl<'a, const O: u8> RECVEN2_W<'a, O> {
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RECVEN2_A::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RECVEN2_A::_1)
    }
}
#[doc = "Field `RECVEN3` reader - Separate Bus Recovery Cycle Insertion Enable 3"]
pub type RECVEN3_R = crate::BitReader<RECVEN3_A>;
#[doc = "Separate Bus Recovery Cycle Insertion Enable 3\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECVEN3_A {
    #[doc = "0: Disabled."]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<RECVEN3_A> for bool {
    #[inline(always)]
    fn from(variant: RECVEN3_A) -> Self {
        variant as u8 != 0
    }
}
impl RECVEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECVEN3_A {
        match self.bits {
            false => RECVEN3_A::_0,
            true => RECVEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RECVEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RECVEN3_A::_1
    }
}
#[doc = "Field `RECVEN3` writer - Separate Bus Recovery Cycle Insertion Enable 3"]
pub type RECVEN3_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSRECEN_SPEC, RECVEN3_A, O>;
impl<'a, const O: u8> RECVEN3_W<'a, O> {
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RECVEN3_A::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RECVEN3_A::_1)
    }
}
#[doc = "Field `RECVEN4` reader - Separate Bus Recovery Cycle Insertion Enable 4"]
pub type RECVEN4_R = crate::BitReader<RECVEN4_A>;
#[doc = "Separate Bus Recovery Cycle Insertion Enable 4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECVEN4_A {
    #[doc = "0: Disabled."]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<RECVEN4_A> for bool {
    #[inline(always)]
    fn from(variant: RECVEN4_A) -> Self {
        variant as u8 != 0
    }
}
impl RECVEN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECVEN4_A {
        match self.bits {
            false => RECVEN4_A::_0,
            true => RECVEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RECVEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RECVEN4_A::_1
    }
}
#[doc = "Field `RECVEN4` writer - Separate Bus Recovery Cycle Insertion Enable 4"]
pub type RECVEN4_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSRECEN_SPEC, RECVEN4_A, O>;
impl<'a, const O: u8> RECVEN4_W<'a, O> {
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RECVEN4_A::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RECVEN4_A::_1)
    }
}
#[doc = "Field `RECVEN5` reader - Separate Bus Recovery Cycle Insertion Enable 5"]
pub type RECVEN5_R = crate::BitReader<RECVEN5_A>;
#[doc = "Separate Bus Recovery Cycle Insertion Enable 5\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECVEN5_A {
    #[doc = "0: Disabled."]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<RECVEN5_A> for bool {
    #[inline(always)]
    fn from(variant: RECVEN5_A) -> Self {
        variant as u8 != 0
    }
}
impl RECVEN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECVEN5_A {
        match self.bits {
            false => RECVEN5_A::_0,
            true => RECVEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RECVEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RECVEN5_A::_1
    }
}
#[doc = "Field `RECVEN5` writer - Separate Bus Recovery Cycle Insertion Enable 5"]
pub type RECVEN5_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSRECEN_SPEC, RECVEN5_A, O>;
impl<'a, const O: u8> RECVEN5_W<'a, O> {
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RECVEN5_A::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RECVEN5_A::_1)
    }
}
#[doc = "Field `RECVEN6` reader - Separate Bus Recovery Cycle Insertion Enable 6"]
pub type RECVEN6_R = crate::BitReader<RECVEN6_A>;
#[doc = "Separate Bus Recovery Cycle Insertion Enable 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECVEN6_A {
    #[doc = "0: Disabled."]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<RECVEN6_A> for bool {
    #[inline(always)]
    fn from(variant: RECVEN6_A) -> Self {
        variant as u8 != 0
    }
}
impl RECVEN6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECVEN6_A {
        match self.bits {
            false => RECVEN6_A::_0,
            true => RECVEN6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RECVEN6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RECVEN6_A::_1
    }
}
#[doc = "Field `RECVEN6` writer - Separate Bus Recovery Cycle Insertion Enable 6"]
pub type RECVEN6_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSRECEN_SPEC, RECVEN6_A, O>;
impl<'a, const O: u8> RECVEN6_W<'a, O> {
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RECVEN6_A::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RECVEN6_A::_1)
    }
}
#[doc = "Field `RECVEN7` reader - Separate Bus Recovery Cycle Insertion Enable 7"]
pub type RECVEN7_R = crate::BitReader<RECVEN7_A>;
#[doc = "Separate Bus Recovery Cycle Insertion Enable 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECVEN7_A {
    #[doc = "0: Disabled."]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<RECVEN7_A> for bool {
    #[inline(always)]
    fn from(variant: RECVEN7_A) -> Self {
        variant as u8 != 0
    }
}
impl RECVEN7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECVEN7_A {
        match self.bits {
            false => RECVEN7_A::_0,
            true => RECVEN7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RECVEN7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RECVEN7_A::_1
    }
}
#[doc = "Field `RECVEN7` writer - Separate Bus Recovery Cycle Insertion Enable 7"]
pub type RECVEN7_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSRECEN_SPEC, RECVEN7_A, O>;
impl<'a, const O: u8> RECVEN7_W<'a, O> {
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RECVEN7_A::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RECVEN7_A::_1)
    }
}
#[doc = "Field `RECVENM0` reader - Multiplexed Bus Recovery Cycle Insertion Enable 0"]
pub type RECVENM0_R = crate::BitReader<RECVENM0_A>;
#[doc = "Multiplexed Bus Recovery Cycle Insertion Enable 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECVENM0_A {
    #[doc = "0: Recovery cycle insertion is disabled."]
    _0 = 0,
    #[doc = "1: Recovery cycle insertion is enabled."]
    _1 = 1,
}
impl From<RECVENM0_A> for bool {
    #[inline(always)]
    fn from(variant: RECVENM0_A) -> Self {
        variant as u8 != 0
    }
}
impl RECVENM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECVENM0_A {
        match self.bits {
            false => RECVENM0_A::_0,
            true => RECVENM0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RECVENM0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RECVENM0_A::_1
    }
}
#[doc = "Field `RECVENM0` writer - Multiplexed Bus Recovery Cycle Insertion Enable 0"]
pub type RECVENM0_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSRECEN_SPEC, RECVENM0_A, O>;
impl<'a, const O: u8> RECVENM0_W<'a, O> {
    #[doc = "Recovery cycle insertion is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RECVENM0_A::_0)
    }
    #[doc = "Recovery cycle insertion is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RECVENM0_A::_1)
    }
}
#[doc = "Field `RECVENM1` reader - Multiplexed Bus Recovery Cycle Insertion Enable 1"]
pub type RECVENM1_R = crate::BitReader<RECVENM1_A>;
#[doc = "Multiplexed Bus Recovery Cycle Insertion Enable 1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECVENM1_A {
    #[doc = "0: Recovery cycle insertion is disabled."]
    _0 = 0,
    #[doc = "1: Recovery cycle insertion is enabled."]
    _1 = 1,
}
impl From<RECVENM1_A> for bool {
    #[inline(always)]
    fn from(variant: RECVENM1_A) -> Self {
        variant as u8 != 0
    }
}
impl RECVENM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECVENM1_A {
        match self.bits {
            false => RECVENM1_A::_0,
            true => RECVENM1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RECVENM1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RECVENM1_A::_1
    }
}
#[doc = "Field `RECVENM1` writer - Multiplexed Bus Recovery Cycle Insertion Enable 1"]
pub type RECVENM1_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSRECEN_SPEC, RECVENM1_A, O>;
impl<'a, const O: u8> RECVENM1_W<'a, O> {
    #[doc = "Recovery cycle insertion is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RECVENM1_A::_0)
    }
    #[doc = "Recovery cycle insertion is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RECVENM1_A::_1)
    }
}
#[doc = "Field `RECVENM2` reader - Multiplexed Bus Recovery Cycle Insertion Enable 2"]
pub type RECVENM2_R = crate::BitReader<RECVENM2_A>;
#[doc = "Multiplexed Bus Recovery Cycle Insertion Enable 2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECVENM2_A {
    #[doc = "0: Recovery cycle insertion is disabled."]
    _0 = 0,
    #[doc = "1: Recovery cycle insertion is enabled."]
    _1 = 1,
}
impl From<RECVENM2_A> for bool {
    #[inline(always)]
    fn from(variant: RECVENM2_A) -> Self {
        variant as u8 != 0
    }
}
impl RECVENM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECVENM2_A {
        match self.bits {
            false => RECVENM2_A::_0,
            true => RECVENM2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RECVENM2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RECVENM2_A::_1
    }
}
#[doc = "Field `RECVENM2` writer - Multiplexed Bus Recovery Cycle Insertion Enable 2"]
pub type RECVENM2_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSRECEN_SPEC, RECVENM2_A, O>;
impl<'a, const O: u8> RECVENM2_W<'a, O> {
    #[doc = "Recovery cycle insertion is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RECVENM2_A::_0)
    }
    #[doc = "Recovery cycle insertion is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RECVENM2_A::_1)
    }
}
#[doc = "Field `RECVENM3` reader - Multiplexed Bus Recovery Cycle Insertion Enable 3"]
pub type RECVENM3_R = crate::BitReader<RECVENM3_A>;
#[doc = "Multiplexed Bus Recovery Cycle Insertion Enable 3\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECVENM3_A {
    #[doc = "0: Recovery cycle insertion is disabled."]
    _0 = 0,
    #[doc = "1: Recovery cycle insertion is enabled."]
    _1 = 1,
}
impl From<RECVENM3_A> for bool {
    #[inline(always)]
    fn from(variant: RECVENM3_A) -> Self {
        variant as u8 != 0
    }
}
impl RECVENM3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECVENM3_A {
        match self.bits {
            false => RECVENM3_A::_0,
            true => RECVENM3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RECVENM3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RECVENM3_A::_1
    }
}
#[doc = "Field `RECVENM3` writer - Multiplexed Bus Recovery Cycle Insertion Enable 3"]
pub type RECVENM3_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSRECEN_SPEC, RECVENM3_A, O>;
impl<'a, const O: u8> RECVENM3_W<'a, O> {
    #[doc = "Recovery cycle insertion is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RECVENM3_A::_0)
    }
    #[doc = "Recovery cycle insertion is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RECVENM3_A::_1)
    }
}
#[doc = "Field `RECVENM4` reader - Multiplexed Bus Recovery Cycle Insertion Enable 4"]
pub type RECVENM4_R = crate::BitReader<RECVENM4_A>;
#[doc = "Multiplexed Bus Recovery Cycle Insertion Enable 4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECVENM4_A {
    #[doc = "0: Recovery cycle insertion is disabled."]
    _0 = 0,
    #[doc = "1: Recovery cycle insertion is enabled."]
    _1 = 1,
}
impl From<RECVENM4_A> for bool {
    #[inline(always)]
    fn from(variant: RECVENM4_A) -> Self {
        variant as u8 != 0
    }
}
impl RECVENM4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECVENM4_A {
        match self.bits {
            false => RECVENM4_A::_0,
            true => RECVENM4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RECVENM4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RECVENM4_A::_1
    }
}
#[doc = "Field `RECVENM4` writer - Multiplexed Bus Recovery Cycle Insertion Enable 4"]
pub type RECVENM4_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSRECEN_SPEC, RECVENM4_A, O>;
impl<'a, const O: u8> RECVENM4_W<'a, O> {
    #[doc = "Recovery cycle insertion is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RECVENM4_A::_0)
    }
    #[doc = "Recovery cycle insertion is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RECVENM4_A::_1)
    }
}
#[doc = "Field `RECVENM5` reader - Multiplexed Bus Recovery Cycle Insertion Enable 5"]
pub type RECVENM5_R = crate::BitReader<RECVENM5_A>;
#[doc = "Multiplexed Bus Recovery Cycle Insertion Enable 5\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECVENM5_A {
    #[doc = "0: Recovery cycle insertion is disabled."]
    _0 = 0,
    #[doc = "1: Recovery cycle insertion is enabled."]
    _1 = 1,
}
impl From<RECVENM5_A> for bool {
    #[inline(always)]
    fn from(variant: RECVENM5_A) -> Self {
        variant as u8 != 0
    }
}
impl RECVENM5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECVENM5_A {
        match self.bits {
            false => RECVENM5_A::_0,
            true => RECVENM5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RECVENM5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RECVENM5_A::_1
    }
}
#[doc = "Field `RECVENM5` writer - Multiplexed Bus Recovery Cycle Insertion Enable 5"]
pub type RECVENM5_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSRECEN_SPEC, RECVENM5_A, O>;
impl<'a, const O: u8> RECVENM5_W<'a, O> {
    #[doc = "Recovery cycle insertion is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RECVENM5_A::_0)
    }
    #[doc = "Recovery cycle insertion is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RECVENM5_A::_1)
    }
}
#[doc = "Field `RECVENM6` reader - Multiplexed Bus Recovery Cycle Insertion Enable 6"]
pub type RECVENM6_R = crate::BitReader<RECVENM6_A>;
#[doc = "Multiplexed Bus Recovery Cycle Insertion Enable 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECVENM6_A {
    #[doc = "0: Recovery cycle insertion is disabled."]
    _0 = 0,
    #[doc = "1: Recovery cycle insertion is enabled."]
    _1 = 1,
}
impl From<RECVENM6_A> for bool {
    #[inline(always)]
    fn from(variant: RECVENM6_A) -> Self {
        variant as u8 != 0
    }
}
impl RECVENM6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECVENM6_A {
        match self.bits {
            false => RECVENM6_A::_0,
            true => RECVENM6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RECVENM6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RECVENM6_A::_1
    }
}
#[doc = "Field `RECVENM6` writer - Multiplexed Bus Recovery Cycle Insertion Enable 6"]
pub type RECVENM6_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSRECEN_SPEC, RECVENM6_A, O>;
impl<'a, const O: u8> RECVENM6_W<'a, O> {
    #[doc = "Recovery cycle insertion is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RECVENM6_A::_0)
    }
    #[doc = "Recovery cycle insertion is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RECVENM6_A::_1)
    }
}
#[doc = "Field `RECVENM7` reader - Multiplexed Bus Recovery Cycle Insertion Enable 7"]
pub type RECVENM7_R = crate::BitReader<RECVENM7_A>;
#[doc = "Multiplexed Bus Recovery Cycle Insertion Enable 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECVENM7_A {
    #[doc = "0: Recovery cycle insertion is disabled."]
    _0 = 0,
    #[doc = "1: Recovery cycle insertion is enabled."]
    _1 = 1,
}
impl From<RECVENM7_A> for bool {
    #[inline(always)]
    fn from(variant: RECVENM7_A) -> Self {
        variant as u8 != 0
    }
}
impl RECVENM7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECVENM7_A {
        match self.bits {
            false => RECVENM7_A::_0,
            true => RECVENM7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RECVENM7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RECVENM7_A::_1
    }
}
#[doc = "Field `RECVENM7` writer - Multiplexed Bus Recovery Cycle Insertion Enable 7"]
pub type RECVENM7_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSRECEN_SPEC, RECVENM7_A, O>;
impl<'a, const O: u8> RECVENM7_W<'a, O> {
    #[doc = "Recovery cycle insertion is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RECVENM7_A::_0)
    }
    #[doc = "Recovery cycle insertion is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RECVENM7_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Separate Bus Recovery Cycle Insertion Enable 0"]
    #[inline(always)]
    pub fn recven0(&self) -> RECVEN0_R {
        RECVEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Separate Bus Recovery Cycle Insertion Enable 1"]
    #[inline(always)]
    pub fn recven1(&self) -> RECVEN1_R {
        RECVEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Separate Bus Recovery Cycle Insertion Enable 2"]
    #[inline(always)]
    pub fn recven2(&self) -> RECVEN2_R {
        RECVEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Separate Bus Recovery Cycle Insertion Enable 3"]
    #[inline(always)]
    pub fn recven3(&self) -> RECVEN3_R {
        RECVEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Separate Bus Recovery Cycle Insertion Enable 4"]
    #[inline(always)]
    pub fn recven4(&self) -> RECVEN4_R {
        RECVEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Separate Bus Recovery Cycle Insertion Enable 5"]
    #[inline(always)]
    pub fn recven5(&self) -> RECVEN5_R {
        RECVEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Separate Bus Recovery Cycle Insertion Enable 6"]
    #[inline(always)]
    pub fn recven6(&self) -> RECVEN6_R {
        RECVEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Separate Bus Recovery Cycle Insertion Enable 7"]
    #[inline(always)]
    pub fn recven7(&self) -> RECVEN7_R {
        RECVEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Multiplexed Bus Recovery Cycle Insertion Enable 0"]
    #[inline(always)]
    pub fn recvenm0(&self) -> RECVENM0_R {
        RECVENM0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Multiplexed Bus Recovery Cycle Insertion Enable 1"]
    #[inline(always)]
    pub fn recvenm1(&self) -> RECVENM1_R {
        RECVENM1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Multiplexed Bus Recovery Cycle Insertion Enable 2"]
    #[inline(always)]
    pub fn recvenm2(&self) -> RECVENM2_R {
        RECVENM2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Multiplexed Bus Recovery Cycle Insertion Enable 3"]
    #[inline(always)]
    pub fn recvenm3(&self) -> RECVENM3_R {
        RECVENM3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Multiplexed Bus Recovery Cycle Insertion Enable 4"]
    #[inline(always)]
    pub fn recvenm4(&self) -> RECVENM4_R {
        RECVENM4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Multiplexed Bus Recovery Cycle Insertion Enable 5"]
    #[inline(always)]
    pub fn recvenm5(&self) -> RECVENM5_R {
        RECVENM5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Multiplexed Bus Recovery Cycle Insertion Enable 6"]
    #[inline(always)]
    pub fn recvenm6(&self) -> RECVENM6_R {
        RECVENM6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Multiplexed Bus Recovery Cycle Insertion Enable 7"]
    #[inline(always)]
    pub fn recvenm7(&self) -> RECVENM7_R {
        RECVENM7_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Separate Bus Recovery Cycle Insertion Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn recven0(&mut self) -> RECVEN0_W<0> {
        RECVEN0_W::new(self)
    }
    #[doc = "Bit 1 - Separate Bus Recovery Cycle Insertion Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn recven1(&mut self) -> RECVEN1_W<1> {
        RECVEN1_W::new(self)
    }
    #[doc = "Bit 2 - Separate Bus Recovery Cycle Insertion Enable 2"]
    #[inline(always)]
    #[must_use]
    pub fn recven2(&mut self) -> RECVEN2_W<2> {
        RECVEN2_W::new(self)
    }
    #[doc = "Bit 3 - Separate Bus Recovery Cycle Insertion Enable 3"]
    #[inline(always)]
    #[must_use]
    pub fn recven3(&mut self) -> RECVEN3_W<3> {
        RECVEN3_W::new(self)
    }
    #[doc = "Bit 4 - Separate Bus Recovery Cycle Insertion Enable 4"]
    #[inline(always)]
    #[must_use]
    pub fn recven4(&mut self) -> RECVEN4_W<4> {
        RECVEN4_W::new(self)
    }
    #[doc = "Bit 5 - Separate Bus Recovery Cycle Insertion Enable 5"]
    #[inline(always)]
    #[must_use]
    pub fn recven5(&mut self) -> RECVEN5_W<5> {
        RECVEN5_W::new(self)
    }
    #[doc = "Bit 6 - Separate Bus Recovery Cycle Insertion Enable 6"]
    #[inline(always)]
    #[must_use]
    pub fn recven6(&mut self) -> RECVEN6_W<6> {
        RECVEN6_W::new(self)
    }
    #[doc = "Bit 7 - Separate Bus Recovery Cycle Insertion Enable 7"]
    #[inline(always)]
    #[must_use]
    pub fn recven7(&mut self) -> RECVEN7_W<7> {
        RECVEN7_W::new(self)
    }
    #[doc = "Bit 8 - Multiplexed Bus Recovery Cycle Insertion Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn recvenm0(&mut self) -> RECVENM0_W<8> {
        RECVENM0_W::new(self)
    }
    #[doc = "Bit 9 - Multiplexed Bus Recovery Cycle Insertion Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn recvenm1(&mut self) -> RECVENM1_W<9> {
        RECVENM1_W::new(self)
    }
    #[doc = "Bit 10 - Multiplexed Bus Recovery Cycle Insertion Enable 2"]
    #[inline(always)]
    #[must_use]
    pub fn recvenm2(&mut self) -> RECVENM2_W<10> {
        RECVENM2_W::new(self)
    }
    #[doc = "Bit 11 - Multiplexed Bus Recovery Cycle Insertion Enable 3"]
    #[inline(always)]
    #[must_use]
    pub fn recvenm3(&mut self) -> RECVENM3_W<11> {
        RECVENM3_W::new(self)
    }
    #[doc = "Bit 12 - Multiplexed Bus Recovery Cycle Insertion Enable 4"]
    #[inline(always)]
    #[must_use]
    pub fn recvenm4(&mut self) -> RECVENM4_W<12> {
        RECVENM4_W::new(self)
    }
    #[doc = "Bit 13 - Multiplexed Bus Recovery Cycle Insertion Enable 5"]
    #[inline(always)]
    #[must_use]
    pub fn recvenm5(&mut self) -> RECVENM5_W<13> {
        RECVENM5_W::new(self)
    }
    #[doc = "Bit 14 - Multiplexed Bus Recovery Cycle Insertion Enable 6"]
    #[inline(always)]
    #[must_use]
    pub fn recvenm6(&mut self) -> RECVENM6_W<14> {
        RECVENM6_W::new(self)
    }
    #[doc = "Bit 15 - Multiplexed Bus Recovery Cycle Insertion Enable 7"]
    #[inline(always)]
    #[must_use]
    pub fn recvenm7(&mut self) -> RECVENM7_W<15> {
        RECVENM7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CS Recovery Cycle Insertion Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csrecen](index.html) module"]
pub struct CSRECEN_SPEC;
impl crate::RegisterSpec for CSRECEN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [csrecen::R](R) reader structure"]
impl crate::Readable for CSRECEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csrecen::W](W) writer structure"]
impl crate::Writable for CSRECEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSRECEN to value 0x3e3e"]
impl crate::Resettable for CSRECEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x3e3e;
}
