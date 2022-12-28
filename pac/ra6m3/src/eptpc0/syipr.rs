#[doc = "Register `SYIPR` reader"]
pub struct R(crate::R<SYIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYIPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYIPR` writer"]
pub struct W(crate::W<SYIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYIPR_SPEC>;
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
impl From<crate::W<SYIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYIPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFMUD` reader - SYSR.OFMUD Status Notification Permission"]
pub type OFMUD_R = crate::BitReader<OFMUD_A>;
#[doc = "SYSR.OFMUD Status Notification Permission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFMUD_A {
    #[doc = "0: Prohibits notification of the state of SYSR.OFMUD."]
    _0 = 0,
    #[doc = "1: Permits notification of the state of SYSR.OFMUD."]
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
#[doc = "Field `OFMUD` writer - SYSR.OFMUD Status Notification Permission"]
pub type OFMUD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYIPR_SPEC, OFMUD_A, O>;
impl<'a, const O: u8> OFMUD_W<'a, O> {
    #[doc = "Prohibits notification of the state of SYSR.OFMUD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OFMUD_A::_0)
    }
    #[doc = "Permits notification of the state of SYSR.OFMUD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OFMUD_A::_1)
    }
}
#[doc = "Field `INTCHG` reader - SYSR.INTCHG Status Notification Permission"]
pub type INTCHG_R = crate::BitReader<INTCHG_A>;
#[doc = "SYSR.INTCHG Status Notification Permission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTCHG_A {
    #[doc = "0: Prohibits notification of the state of SYSR.INTCHG."]
    _0 = 0,
    #[doc = "1: Permits notification of the state of SYSR.INTCHG."]
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
#[doc = "Field `INTCHG` writer - SYSR.INTCHG Status Notification Permission"]
pub type INTCHG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYIPR_SPEC, INTCHG_A, O>;
impl<'a, const O: u8> INTCHG_W<'a, O> {
    #[doc = "Prohibits notification of the state of SYSR.INTCHG."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTCHG_A::_0)
    }
    #[doc = "Permits notification of the state of SYSR.INTCHG."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTCHG_A::_1)
    }
}
#[doc = "Field `MPDUD` reader - SYSR.MPDUD Status Notification Permission"]
pub type MPDUD_R = crate::BitReader<MPDUD_A>;
#[doc = "SYSR.MPDUD Status Notification Permission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPDUD_A {
    #[doc = "0: Prohibits notification of the state of SYSR.MPDUD."]
    _0 = 0,
    #[doc = "1: Permits notification of the state of SYSR.MPDUD."]
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
#[doc = "Field `MPDUD` writer - SYSR.MPDUD Status Notification Permission"]
pub type MPDUD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYIPR_SPEC, MPDUD_A, O>;
impl<'a, const O: u8> MPDUD_W<'a, O> {
    #[doc = "Prohibits notification of the state of SYSR.MPDUD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPDUD_A::_0)
    }
    #[doc = "Permits notification of the state of SYSR.MPDUD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPDUD_A::_1)
    }
}
#[doc = "Field `DRPTO` reader - SYSR.DRPTO Status Notification Permission"]
pub type DRPTO_R = crate::BitReader<DRPTO_A>;
#[doc = "SYSR.DRPTO Status Notification Permission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRPTO_A {
    #[doc = "0: Prohibits notification of the state of SYSR.DRPTO."]
    _0 = 0,
    #[doc = "1: Permits notification of the state of SYSR.DRPTO."]
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
#[doc = "Field `DRPTO` writer - SYSR.DRPTO Status Notification Permission"]
pub type DRPTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYIPR_SPEC, DRPTO_A, O>;
impl<'a, const O: u8> DRPTO_W<'a, O> {
    #[doc = "Prohibits notification of the state of SYSR.DRPTO."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRPTO_A::_0)
    }
    #[doc = "Permits notification of the state of SYSR.DRPTO."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRPTO_A::_1)
    }
}
#[doc = "Field `INTDEV` reader - SYSR.INTDEV Status Notification Permission"]
pub type INTDEV_R = crate::BitReader<INTDEV_A>;
#[doc = "SYSR.INTDEV Status Notification Permission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTDEV_A {
    #[doc = "0: Prohibits notification of the state of SYSR.INTDEV."]
    _0 = 0,
    #[doc = "1: Permits notification of the state of SYSR.INTDEV."]
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
#[doc = "Field `INTDEV` writer - SYSR.INTDEV Status Notification Permission"]
pub type INTDEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYIPR_SPEC, INTDEV_A, O>;
impl<'a, const O: u8> INTDEV_W<'a, O> {
    #[doc = "Prohibits notification of the state of SYSR.INTDEV."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTDEV_A::_0)
    }
    #[doc = "Permits notification of the state of SYSR.INTDEV."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTDEV_A::_1)
    }
}
#[doc = "Field `DRQOVR` reader - SYSR.DRQOVR Status Notification Permission"]
pub type DRQOVR_R = crate::BitReader<DRQOVR_A>;
#[doc = "SYSR.DRQOVR Status Notification Permission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRQOVR_A {
    #[doc = "0: Prohibits notification of the state of SYSR.DRQOVR."]
    _0 = 0,
    #[doc = "1: Permits notification of the state of SYSR.DRQOVR."]
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
#[doc = "Field `DRQOVR` writer - SYSR.DRQOVR Status Notification Permission"]
pub type DRQOVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYIPR_SPEC, DRQOVR_A, O>;
impl<'a, const O: u8> DRQOVR_W<'a, O> {
    #[doc = "Prohibits notification of the state of SYSR.DRQOVR."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRQOVR_A::_0)
    }
    #[doc = "Permits notification of the state of SYSR.DRQOVR."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRQOVR_A::_1)
    }
}
#[doc = "Field `RECLP` reader - SYSR.RECLP Status Notification Permission"]
pub type RECLP_R = crate::BitReader<RECLP_A>;
#[doc = "SYSR.RECLP Status Notification Permission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECLP_A {
    #[doc = "0: Prohibits notification of the state of SYSR.RECLP."]
    _0 = 0,
    #[doc = "1: Permits notification of the state of SYSR.RECLP."]
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
#[doc = "Field `RECLP` writer - SYSR.RECLP Status Notification Permission"]
pub type RECLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYIPR_SPEC, RECLP_A, O>;
impl<'a, const O: u8> RECLP_W<'a, O> {
    #[doc = "Prohibits notification of the state of SYSR.RECLP."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RECLP_A::_0)
    }
    #[doc = "Permits notification of the state of SYSR.RECLP."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RECLP_A::_1)
    }
}
#[doc = "Field `INFABT` reader - SYSR.INFABT Status Notification Permission"]
pub type INFABT_R = crate::BitReader<INFABT_A>;
#[doc = "SYSR.INFABT Status Notification Permission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INFABT_A {
    #[doc = "0: Prohibits notification of the state of SYSR.INFABT."]
    _0 = 0,
    #[doc = "1: Permits notification of the state of SYSR.INFABT."]
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
#[doc = "Field `INFABT` writer - SYSR.INFABT Status Notification Permission"]
pub type INFABT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYIPR_SPEC, INFABT_A, O>;
impl<'a, const O: u8> INFABT_W<'a, O> {
    #[doc = "Prohibits notification of the state of SYSR.INFABT."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INFABT_A::_0)
    }
    #[doc = "Permits notification of the state of SYSR.INFABT."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INFABT_A::_1)
    }
}
#[doc = "Field `RESDN` reader - SYSR.RESDN Status Notification Permission"]
pub type RESDN_R = crate::BitReader<RESDN_A>;
#[doc = "SYSR.RESDN Status Notification Permission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESDN_A {
    #[doc = "0: Prohibits notification of the state of SYSR.RESDN."]
    _0 = 0,
    #[doc = "1: Permits notification of the state of SYSR.RESDN."]
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
#[doc = "Field `RESDN` writer - SYSR.RESDN Status Notification Permission"]
pub type RESDN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYIPR_SPEC, RESDN_A, O>;
impl<'a, const O: u8> RESDN_W<'a, O> {
    #[doc = "Prohibits notification of the state of SYSR.RESDN."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RESDN_A::_0)
    }
    #[doc = "Permits notification of the state of SYSR.RESDN."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RESDN_A::_1)
    }
}
#[doc = "Field `GENDN` reader - SYSR.GENDN Status Notification Permission"]
pub type GENDN_R = crate::BitReader<GENDN_A>;
#[doc = "SYSR.GENDN Status Notification Permission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GENDN_A {
    #[doc = "0: Prohibits notification of the state of SYSR.GENDN."]
    _0 = 0,
    #[doc = "1: Permits notification of the state of SYSR.GENDN."]
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
#[doc = "Field `GENDN` writer - SYSR.GENDN Status Notification Permission"]
pub type GENDN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYIPR_SPEC, GENDN_A, O>;
impl<'a, const O: u8> GENDN_W<'a, O> {
    #[doc = "Prohibits notification of the state of SYSR.GENDN."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GENDN_A::_0)
    }
    #[doc = "Permits notification of the state of SYSR.GENDN."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GENDN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - SYSR.OFMUD Status Notification Permission"]
    #[inline(always)]
    pub fn ofmud(&self) -> OFMUD_R {
        OFMUD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYSR.INTCHG Status Notification Permission"]
    #[inline(always)]
    pub fn intchg(&self) -> INTCHG_R {
        INTCHG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SYSR.MPDUD Status Notification Permission"]
    #[inline(always)]
    pub fn mpdud(&self) -> MPDUD_R {
        MPDUD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - SYSR.DRPTO Status Notification Permission"]
    #[inline(always)]
    pub fn drpto(&self) -> DRPTO_R {
        DRPTO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SYSR.INTDEV Status Notification Permission"]
    #[inline(always)]
    pub fn intdev(&self) -> INTDEV_R {
        INTDEV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SYSR.DRQOVR Status Notification Permission"]
    #[inline(always)]
    pub fn drqovr(&self) -> DRQOVR_R {
        DRQOVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - SYSR.RECLP Status Notification Permission"]
    #[inline(always)]
    pub fn reclp(&self) -> RECLP_R {
        RECLP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - SYSR.INFABT Status Notification Permission"]
    #[inline(always)]
    pub fn infabt(&self) -> INFABT_R {
        INFABT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - SYSR.RESDN Status Notification Permission"]
    #[inline(always)]
    pub fn resdn(&self) -> RESDN_R {
        RESDN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SYSR.GENDN Status Notification Permission"]
    #[inline(always)]
    pub fn gendn(&self) -> GENDN_R {
        GENDN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSR.OFMUD Status Notification Permission"]
    #[inline(always)]
    #[must_use]
    pub fn ofmud(&mut self) -> OFMUD_W<0> {
        OFMUD_W::new(self)
    }
    #[doc = "Bit 1 - SYSR.INTCHG Status Notification Permission"]
    #[inline(always)]
    #[must_use]
    pub fn intchg(&mut self) -> INTCHG_W<1> {
        INTCHG_W::new(self)
    }
    #[doc = "Bit 2 - SYSR.MPDUD Status Notification Permission"]
    #[inline(always)]
    #[must_use]
    pub fn mpdud(&mut self) -> MPDUD_W<2> {
        MPDUD_W::new(self)
    }
    #[doc = "Bit 4 - SYSR.DRPTO Status Notification Permission"]
    #[inline(always)]
    #[must_use]
    pub fn drpto(&mut self) -> DRPTO_W<4> {
        DRPTO_W::new(self)
    }
    #[doc = "Bit 5 - SYSR.INTDEV Status Notification Permission"]
    #[inline(always)]
    #[must_use]
    pub fn intdev(&mut self) -> INTDEV_W<5> {
        INTDEV_W::new(self)
    }
    #[doc = "Bit 6 - SYSR.DRQOVR Status Notification Permission"]
    #[inline(always)]
    #[must_use]
    pub fn drqovr(&mut self) -> DRQOVR_W<6> {
        DRQOVR_W::new(self)
    }
    #[doc = "Bit 12 - SYSR.RECLP Status Notification Permission"]
    #[inline(always)]
    #[must_use]
    pub fn reclp(&mut self) -> RECLP_W<12> {
        RECLP_W::new(self)
    }
    #[doc = "Bit 14 - SYSR.INFABT Status Notification Permission"]
    #[inline(always)]
    #[must_use]
    pub fn infabt(&mut self) -> INFABT_W<14> {
        INFABT_W::new(self)
    }
    #[doc = "Bit 16 - SYSR.RESDN Status Notification Permission"]
    #[inline(always)]
    #[must_use]
    pub fn resdn(&mut self) -> RESDN_W<16> {
        RESDN_W::new(self)
    }
    #[doc = "Bit 17 - SYSR.GENDN Status Notification Permission"]
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
#[doc = "SYNFP Status Notification Permission Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syipr](index.html) module"]
pub struct SYIPR_SPEC;
impl crate::RegisterSpec for SYIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syipr::R](R) reader structure"]
impl crate::Readable for SYIPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syipr::W](W) writer structure"]
impl crate::Writable for SYIPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYIPR to value 0"]
impl crate::Resettable for SYIPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
