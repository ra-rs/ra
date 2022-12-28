#[doc = "Register `SMPUMBIU` reader"]
pub struct R(crate::R<SMPUMBIU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPUMBIU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPUMBIU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPUMBIU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPUMBIU` writer"]
pub struct W(crate::W<SMPUMBIU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPUMBIU_SPEC>;
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
impl From<crate::W<SMPUMBIU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPUMBIU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPGRPA` reader - Master Group A Read protection"]
pub type RPGRPA_R = crate::BitReader<RPGRPA_A>;
#[doc = "Master Group A Read protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPGRPA_A {
    #[doc = "0: Memory protection for master group A reads disabled"]
    _0 = 0,
    #[doc = "1: Memory protection for master group A reads enabled."]
    _1 = 1,
}
impl From<RPGRPA_A> for bool {
    #[inline(always)]
    fn from(variant: RPGRPA_A) -> Self {
        variant as u8 != 0
    }
}
impl RPGRPA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPGRPA_A {
        match self.bits {
            false => RPGRPA_A::_0,
            true => RPGRPA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPGRPA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPGRPA_A::_1
    }
}
#[doc = "Field `RPGRPA` writer - Master Group A Read protection"]
pub type RPGRPA_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUMBIU_SPEC, RPGRPA_A, O>;
impl<'a, const O: u8> RPGRPA_W<'a, O> {
    #[doc = "Memory protection for master group A reads disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RPGRPA_A::_0)
    }
    #[doc = "Memory protection for master group A reads enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RPGRPA_A::_1)
    }
}
#[doc = "Field `WPGRPA` reader - Master Group A Write protection"]
pub type WPGRPA_R = crate::BitReader<WPGRPA_A>;
#[doc = "Master Group A Write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPGRPA_A {
    #[doc = "0: Memory protection for master group A writes disabled"]
    _0 = 0,
    #[doc = "1: Memory protection for master group A writes enabled."]
    _1 = 1,
}
impl From<WPGRPA_A> for bool {
    #[inline(always)]
    fn from(variant: WPGRPA_A) -> Self {
        variant as u8 != 0
    }
}
impl WPGRPA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPGRPA_A {
        match self.bits {
            false => WPGRPA_A::_0,
            true => WPGRPA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WPGRPA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WPGRPA_A::_1
    }
}
#[doc = "Field `WPGRPA` writer - Master Group A Write protection"]
pub type WPGRPA_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUMBIU_SPEC, WPGRPA_A, O>;
impl<'a, const O: u8> WPGRPA_W<'a, O> {
    #[doc = "Memory protection for master group A writes disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WPGRPA_A::_0)
    }
    #[doc = "Memory protection for master group A writes enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WPGRPA_A::_1)
    }
}
#[doc = "Field `RPGRPB` reader - Master Group B Read protection"]
pub type RPGRPB_R = crate::BitReader<RPGRPB_A>;
#[doc = "Master Group B Read protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPGRPB_A {
    #[doc = "0: Memory protection for master group B reads disabled"]
    _0 = 0,
    #[doc = "1: Memory protection for master group B reads enabled."]
    _1 = 1,
}
impl From<RPGRPB_A> for bool {
    #[inline(always)]
    fn from(variant: RPGRPB_A) -> Self {
        variant as u8 != 0
    }
}
impl RPGRPB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPGRPB_A {
        match self.bits {
            false => RPGRPB_A::_0,
            true => RPGRPB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPGRPB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPGRPB_A::_1
    }
}
#[doc = "Field `RPGRPB` writer - Master Group B Read protection"]
pub type RPGRPB_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUMBIU_SPEC, RPGRPB_A, O>;
impl<'a, const O: u8> RPGRPB_W<'a, O> {
    #[doc = "Memory protection for master group B reads disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RPGRPB_A::_0)
    }
    #[doc = "Memory protection for master group B reads enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RPGRPB_A::_1)
    }
}
#[doc = "Field `WPGRPB` reader - Master Group B Write protection"]
pub type WPGRPB_R = crate::BitReader<WPGRPB_A>;
#[doc = "Master Group B Write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPGRPB_A {
    #[doc = "0: Memory protection for master group B writes disabled"]
    _0 = 0,
    #[doc = "1: Memory protection for master group B writes enabled."]
    _1 = 1,
}
impl From<WPGRPB_A> for bool {
    #[inline(always)]
    fn from(variant: WPGRPB_A) -> Self {
        variant as u8 != 0
    }
}
impl WPGRPB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPGRPB_A {
        match self.bits {
            false => WPGRPB_A::_0,
            true => WPGRPB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WPGRPB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WPGRPB_A::_1
    }
}
#[doc = "Field `WPGRPB` writer - Master Group B Write protection"]
pub type WPGRPB_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUMBIU_SPEC, WPGRPB_A, O>;
impl<'a, const O: u8> WPGRPB_W<'a, O> {
    #[doc = "Memory protection for master group B writes disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WPGRPB_A::_0)
    }
    #[doc = "Memory protection for master group B writes enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WPGRPB_A::_1)
    }
}
#[doc = "Field `RPGRPC` reader - Master Group C Read protection"]
pub type RPGRPC_R = crate::BitReader<RPGRPC_A>;
#[doc = "Master Group C Read protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPGRPC_A {
    #[doc = "0: Memory protection for master group C reads disabled"]
    _0 = 0,
    #[doc = "1: Memory protection for master group C reads enabled."]
    _1 = 1,
}
impl From<RPGRPC_A> for bool {
    #[inline(always)]
    fn from(variant: RPGRPC_A) -> Self {
        variant as u8 != 0
    }
}
impl RPGRPC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPGRPC_A {
        match self.bits {
            false => RPGRPC_A::_0,
            true => RPGRPC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPGRPC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPGRPC_A::_1
    }
}
#[doc = "Field `RPGRPC` writer - Master Group C Read protection"]
pub type RPGRPC_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUMBIU_SPEC, RPGRPC_A, O>;
impl<'a, const O: u8> RPGRPC_W<'a, O> {
    #[doc = "Memory protection for master group C reads disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RPGRPC_A::_0)
    }
    #[doc = "Memory protection for master group C reads enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RPGRPC_A::_1)
    }
}
#[doc = "Field `WPGRPC` reader - Master Group C Write protection"]
pub type WPGRPC_R = crate::BitReader<WPGRPC_A>;
#[doc = "Master Group C Write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPGRPC_A {
    #[doc = "0: Memory protection for master group C writes disabled"]
    _0 = 0,
    #[doc = "1: Memory protection for master group C writes enabled."]
    _1 = 1,
}
impl From<WPGRPC_A> for bool {
    #[inline(always)]
    fn from(variant: WPGRPC_A) -> Self {
        variant as u8 != 0
    }
}
impl WPGRPC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPGRPC_A {
        match self.bits {
            false => WPGRPC_A::_0,
            true => WPGRPC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WPGRPC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WPGRPC_A::_1
    }
}
#[doc = "Field `WPGRPC` writer - Master Group C Write protection"]
pub type WPGRPC_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUMBIU_SPEC, WPGRPC_A, O>;
impl<'a, const O: u8> WPGRPC_W<'a, O> {
    #[doc = "Memory protection for master group C writes disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WPGRPC_A::_0)
    }
    #[doc = "Memory protection for master group C writes enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WPGRPC_A::_1)
    }
}
#[doc = "Field `RPFLI` reader - Code Flash Memory Read Protection"]
pub type RPFLI_R = crate::BitReader<RPFLI_A>;
#[doc = "Code Flash Memory Read Protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPFLI_A {
    #[doc = "0: Memory protection for code flash memory reads from master group A, B, and C disabled"]
    _0 = 0,
    #[doc = "1: Memory protection for code flash memory reads from master group A, B, and C enabled."]
    _1 = 1,
}
impl From<RPFLI_A> for bool {
    #[inline(always)]
    fn from(variant: RPFLI_A) -> Self {
        variant as u8 != 0
    }
}
impl RPFLI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPFLI_A {
        match self.bits {
            false => RPFLI_A::_0,
            true => RPFLI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPFLI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPFLI_A::_1
    }
}
#[doc = "Field `RPFLI` writer - Code Flash Memory Read Protection"]
pub type RPFLI_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUMBIU_SPEC, RPFLI_A, O>;
impl<'a, const O: u8> RPFLI_W<'a, O> {
    #[doc = "Memory protection for code flash memory reads from master group A, B, and C disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RPFLI_A::_0)
    }
    #[doc = "Memory protection for code flash memory reads from master group A, B, and C enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RPFLI_A::_1)
    }
}
#[doc = "Field `WPFLI` reader - Code Flash Memory Write Protection (Note: This bit is read as 1. The write value should be 1.)"]
pub type WPFLI_R = crate::BitReader<WPFLI_A>;
#[doc = "Code Flash Memory Write Protection (Note: This bit is read as 1. The write value should be 1.)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPFLI_A {
    #[doc = "0: Setting prohibited"]
    _0 = 0,
    #[doc = "1: Memory protection for code flash memory writes from master group A, B, and C enabled."]
    _1 = 1,
}
impl From<WPFLI_A> for bool {
    #[inline(always)]
    fn from(variant: WPFLI_A) -> Self {
        variant as u8 != 0
    }
}
impl WPFLI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPFLI_A {
        match self.bits {
            false => WPFLI_A::_0,
            true => WPFLI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WPFLI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WPFLI_A::_1
    }
}
#[doc = "Field `WPFLI` writer - Code Flash Memory Write Protection (Note: This bit is read as 1. The write value should be 1.)"]
pub type WPFLI_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUMBIU_SPEC, WPFLI_A, O>;
impl<'a, const O: u8> WPFLI_W<'a, O> {
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WPFLI_A::_0)
    }
    #[doc = "Memory protection for code flash memory writes from master group A, B, and C enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WPFLI_A::_1)
    }
}
#[doc = "Field `RPSRAMHS` reader - SRAMHS Read Protection"]
pub type RPSRAMHS_R = crate::BitReader<RPSRAMHS_A>;
#[doc = "SRAMHS Read Protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPSRAMHS_A {
    #[doc = "0: Memory protection for SRAMHS reads from master group A, B, and C disabled"]
    _0 = 0,
    #[doc = "1: Memory protection for SRAMHS reads from master group A, B, and C enabled."]
    _1 = 1,
}
impl From<RPSRAMHS_A> for bool {
    #[inline(always)]
    fn from(variant: RPSRAMHS_A) -> Self {
        variant as u8 != 0
    }
}
impl RPSRAMHS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPSRAMHS_A {
        match self.bits {
            false => RPSRAMHS_A::_0,
            true => RPSRAMHS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPSRAMHS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPSRAMHS_A::_1
    }
}
#[doc = "Field `RPSRAMHS` writer - SRAMHS Read Protection"]
pub type RPSRAMHS_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUMBIU_SPEC, RPSRAMHS_A, O>;
impl<'a, const O: u8> RPSRAMHS_W<'a, O> {
    #[doc = "Memory protection for SRAMHS reads from master group A, B, and C disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RPSRAMHS_A::_0)
    }
    #[doc = "Memory protection for SRAMHS reads from master group A, B, and C enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RPSRAMHS_A::_1)
    }
}
#[doc = "Field `WPSRAMHS` reader - SRAMHS Write Protection"]
pub type WPSRAMHS_R = crate::BitReader<WPSRAMHS_A>;
#[doc = "SRAMHS Write Protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPSRAMHS_A {
    #[doc = "0: Memory protection for SRAMHS writes from master group A, B, and C disabled"]
    _0 = 0,
    #[doc = "1: Memory protection for SRAMHS writes from master group A, B, and C enabled."]
    _1 = 1,
}
impl From<WPSRAMHS_A> for bool {
    #[inline(always)]
    fn from(variant: WPSRAMHS_A) -> Self {
        variant as u8 != 0
    }
}
impl WPSRAMHS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPSRAMHS_A {
        match self.bits {
            false => WPSRAMHS_A::_0,
            true => WPSRAMHS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WPSRAMHS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WPSRAMHS_A::_1
    }
}
#[doc = "Field `WPSRAMHS` writer - SRAMHS Write Protection"]
pub type WPSRAMHS_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUMBIU_SPEC, WPSRAMHS_A, O>;
impl<'a, const O: u8> WPSRAMHS_W<'a, O> {
    #[doc = "Memory protection for SRAMHS writes from master group A, B, and C disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WPSRAMHS_A::_0)
    }
    #[doc = "Memory protection for SRAMHS writes from master group A, B, and C enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WPSRAMHS_A::_1)
    }
}
impl R {
    #[doc = "Bit 2 - Master Group A Read protection"]
    #[inline(always)]
    pub fn rpgrpa(&self) -> RPGRPA_R {
        RPGRPA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master Group A Write protection"]
    #[inline(always)]
    pub fn wpgrpa(&self) -> WPGRPA_R {
        WPGRPA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master Group B Read protection"]
    #[inline(always)]
    pub fn rpgrpb(&self) -> RPGRPB_R {
        RPGRPB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master Group B Write protection"]
    #[inline(always)]
    pub fn wpgrpb(&self) -> WPGRPB_R {
        WPGRPB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Master Group C Read protection"]
    #[inline(always)]
    pub fn rpgrpc(&self) -> RPGRPC_R {
        RPGRPC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Master Group C Write protection"]
    #[inline(always)]
    pub fn wpgrpc(&self) -> WPGRPC_R {
        WPGRPC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - Code Flash Memory Read Protection"]
    #[inline(always)]
    pub fn rpfli(&self) -> RPFLI_R {
        RPFLI_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Code Flash Memory Write Protection (Note: This bit is read as 1. The write value should be 1.)"]
    #[inline(always)]
    pub fn wpfli(&self) -> WPFLI_R {
        WPFLI_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SRAMHS Read Protection"]
    #[inline(always)]
    pub fn rpsramhs(&self) -> RPSRAMHS_R {
        RPSRAMHS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SRAMHS Write Protection"]
    #[inline(always)]
    pub fn wpsramhs(&self) -> WPSRAMHS_R {
        WPSRAMHS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Master Group A Read protection"]
    #[inline(always)]
    #[must_use]
    pub fn rpgrpa(&mut self) -> RPGRPA_W<2> {
        RPGRPA_W::new(self)
    }
    #[doc = "Bit 3 - Master Group A Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn wpgrpa(&mut self) -> WPGRPA_W<3> {
        WPGRPA_W::new(self)
    }
    #[doc = "Bit 4 - Master Group B Read protection"]
    #[inline(always)]
    #[must_use]
    pub fn rpgrpb(&mut self) -> RPGRPB_W<4> {
        RPGRPB_W::new(self)
    }
    #[doc = "Bit 5 - Master Group B Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn wpgrpb(&mut self) -> WPGRPB_W<5> {
        WPGRPB_W::new(self)
    }
    #[doc = "Bit 6 - Master Group C Read protection"]
    #[inline(always)]
    #[must_use]
    pub fn rpgrpc(&mut self) -> RPGRPC_W<6> {
        RPGRPC_W::new(self)
    }
    #[doc = "Bit 7 - Master Group C Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn wpgrpc(&mut self) -> WPGRPC_W<7> {
        WPGRPC_W::new(self)
    }
    #[doc = "Bit 12 - Code Flash Memory Read Protection"]
    #[inline(always)]
    #[must_use]
    pub fn rpfli(&mut self) -> RPFLI_W<12> {
        RPFLI_W::new(self)
    }
    #[doc = "Bit 13 - Code Flash Memory Write Protection (Note: This bit is read as 1. The write value should be 1.)"]
    #[inline(always)]
    #[must_use]
    pub fn wpfli(&mut self) -> WPFLI_W<13> {
        WPFLI_W::new(self)
    }
    #[doc = "Bit 14 - SRAMHS Read Protection"]
    #[inline(always)]
    #[must_use]
    pub fn rpsramhs(&mut self) -> RPSRAMHS_W<14> {
        RPSRAMHS_W::new(self)
    }
    #[doc = "Bit 15 - SRAMHS Write Protection"]
    #[inline(always)]
    #[must_use]
    pub fn wpsramhs(&mut self) -> WPSRAMHS_W<15> {
        WPSRAMHS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Access Control Register for MBIU\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpumbiu](index.html) module"]
pub struct SMPUMBIU_SPEC;
impl crate::RegisterSpec for SMPUMBIU_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smpumbiu::R](R) reader structure"]
impl crate::Readable for SMPUMBIU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpumbiu::W](W) writer structure"]
impl crate::Writable for SMPUMBIU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMPUMBIU to value 0x2000"]
impl crate::Resettable for SMPUMBIU_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000;
}
