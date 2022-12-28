#[doc = "Register `SCKDIVCR` reader"]
pub struct R(crate::R<SCKDIVCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCKDIVCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCKDIVCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCKDIVCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCKDIVCR` writer"]
pub struct W(crate::W<SCKDIVCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCKDIVCR_SPEC>;
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
impl From<crate::W<SCKDIVCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCKDIVCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCKD` reader - Peripheral Module Clock D (PCLKD) Select"]
pub type PCKD_R = crate::FieldReader<u8, PCKD_A>;
#[doc = "Peripheral Module Clock D (PCLKD) Select\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCKD_A {
    #[doc = "0: /1"]
    _000 = 0,
    #[doc = "1: /2"]
    _001 = 1,
    #[doc = "2: /4"]
    _010 = 2,
    #[doc = "3: /8"]
    _011 = 3,
    #[doc = "4: /16"]
    _100 = 4,
    #[doc = "5: /32"]
    _101 = 5,
    #[doc = "6: /64"]
    _110 = 6,
}
impl From<PCKD_A> for u8 {
    #[inline(always)]
    fn from(variant: PCKD_A) -> Self {
        variant as _
    }
}
impl PCKD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCKD_A {
        match self.bits {
            0 => PCKD_A::_000,
            1 => PCKD_A::_001,
            2 => PCKD_A::_010,
            3 => PCKD_A::_011,
            4 => PCKD_A::_100,
            5 => PCKD_A::_101,
            6 => PCKD_A::_110,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PCKD_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PCKD_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PCKD_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PCKD_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PCKD_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PCKD_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PCKD_A::_110
    }
}
#[doc = "Field `PCKD` writer - Peripheral Module Clock D (PCLKD) Select"]
pub type PCKD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCKDIVCR_SPEC, u8, PCKD_A, 3, O>;
impl<'a, const O: u8> PCKD_W<'a, O> {
    #[doc = "/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PCKD_A::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PCKD_A::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PCKD_A::_010)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PCKD_A::_011)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PCKD_A::_100)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PCKD_A::_101)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PCKD_A::_110)
    }
}
#[doc = "Field `PCKC` reader - Peripheral Module Clock C (PCLKC) Select"]
pub type PCKC_R = crate::FieldReader<u8, PCKC_A>;
#[doc = "Peripheral Module Clock C (PCLKC) Select\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCKC_A {
    #[doc = "0: /1"]
    _000 = 0,
    #[doc = "1: /2"]
    _001 = 1,
    #[doc = "2: /4"]
    _010 = 2,
    #[doc = "3: /8"]
    _011 = 3,
    #[doc = "4: /16"]
    _100 = 4,
    #[doc = "5: /32"]
    _101 = 5,
    #[doc = "6: /64"]
    _110 = 6,
}
impl From<PCKC_A> for u8 {
    #[inline(always)]
    fn from(variant: PCKC_A) -> Self {
        variant as _
    }
}
impl PCKC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCKC_A {
        match self.bits {
            0 => PCKC_A::_000,
            1 => PCKC_A::_001,
            2 => PCKC_A::_010,
            3 => PCKC_A::_011,
            4 => PCKC_A::_100,
            5 => PCKC_A::_101,
            6 => PCKC_A::_110,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PCKC_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PCKC_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PCKC_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PCKC_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PCKC_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PCKC_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PCKC_A::_110
    }
}
#[doc = "Field `PCKC` writer - Peripheral Module Clock C (PCLKC) Select"]
pub type PCKC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCKDIVCR_SPEC, u8, PCKC_A, 3, O>;
impl<'a, const O: u8> PCKC_W<'a, O> {
    #[doc = "/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PCKC_A::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PCKC_A::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PCKC_A::_010)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PCKC_A::_011)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PCKC_A::_100)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PCKC_A::_101)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PCKC_A::_110)
    }
}
#[doc = "Field `PCKB` reader - Peripheral Module Clock B (PCLKB) Select"]
pub type PCKB_R = crate::FieldReader<u8, PCKB_A>;
#[doc = "Peripheral Module Clock B (PCLKB) Select\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCKB_A {
    #[doc = "0: /1"]
    _000 = 0,
    #[doc = "1: /2"]
    _001 = 1,
    #[doc = "2: /4"]
    _010 = 2,
    #[doc = "3: /8"]
    _011 = 3,
    #[doc = "4: /16"]
    _100 = 4,
    #[doc = "5: /32"]
    _101 = 5,
    #[doc = "6: /64"]
    _110 = 6,
}
impl From<PCKB_A> for u8 {
    #[inline(always)]
    fn from(variant: PCKB_A) -> Self {
        variant as _
    }
}
impl PCKB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCKB_A {
        match self.bits {
            0 => PCKB_A::_000,
            1 => PCKB_A::_001,
            2 => PCKB_A::_010,
            3 => PCKB_A::_011,
            4 => PCKB_A::_100,
            5 => PCKB_A::_101,
            6 => PCKB_A::_110,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PCKB_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PCKB_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PCKB_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PCKB_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PCKB_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PCKB_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PCKB_A::_110
    }
}
#[doc = "Field `PCKB` writer - Peripheral Module Clock B (PCLKB) Select"]
pub type PCKB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCKDIVCR_SPEC, u8, PCKB_A, 3, O>;
impl<'a, const O: u8> PCKB_W<'a, O> {
    #[doc = "/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PCKB_A::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PCKB_A::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PCKB_A::_010)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PCKB_A::_011)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PCKB_A::_100)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PCKB_A::_101)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PCKB_A::_110)
    }
}
#[doc = "Field `PCKA` reader - Peripheral Module Clock A (PCLKA) Select"]
pub type PCKA_R = crate::FieldReader<u8, PCKA_A>;
#[doc = "Peripheral Module Clock A (PCLKA) Select\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCKA_A {
    #[doc = "0: /1"]
    _000 = 0,
    #[doc = "1: /2"]
    _001 = 1,
    #[doc = "2: /4"]
    _010 = 2,
    #[doc = "3: /8"]
    _011 = 3,
    #[doc = "4: /16"]
    _100 = 4,
    #[doc = "5: /32"]
    _101 = 5,
    #[doc = "6: /64"]
    _110 = 6,
}
impl From<PCKA_A> for u8 {
    #[inline(always)]
    fn from(variant: PCKA_A) -> Self {
        variant as _
    }
}
impl PCKA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCKA_A {
        match self.bits {
            0 => PCKA_A::_000,
            1 => PCKA_A::_001,
            2 => PCKA_A::_010,
            3 => PCKA_A::_011,
            4 => PCKA_A::_100,
            5 => PCKA_A::_101,
            6 => PCKA_A::_110,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PCKA_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PCKA_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PCKA_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PCKA_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PCKA_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PCKA_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PCKA_A::_110
    }
}
#[doc = "Field `PCKA` writer - Peripheral Module Clock A (PCLKA) Select"]
pub type PCKA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCKDIVCR_SPEC, u8, PCKA_A, 3, O>;
impl<'a, const O: u8> PCKA_W<'a, O> {
    #[doc = "/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PCKA_A::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PCKA_A::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PCKA_A::_010)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PCKA_A::_011)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PCKA_A::_100)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PCKA_A::_101)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PCKA_A::_110)
    }
}
#[doc = "Field `BCK` reader - External Bus Clock (BCLK) Select"]
pub type BCK_R = crate::FieldReader<u8, BCK_A>;
#[doc = "External Bus Clock (BCLK) Select\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BCK_A {
    #[doc = "0: /1"]
    _000 = 0,
    #[doc = "1: /2"]
    _001 = 1,
    #[doc = "2: /4"]
    _010 = 2,
    #[doc = "3: /8"]
    _011 = 3,
    #[doc = "4: /16"]
    _100 = 4,
    #[doc = "5: /32"]
    _101 = 5,
    #[doc = "6: /64"]
    _110 = 6,
}
impl From<BCK_A> for u8 {
    #[inline(always)]
    fn from(variant: BCK_A) -> Self {
        variant as _
    }
}
impl BCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCK_A {
        match self.bits {
            0 => BCK_A::_000,
            1 => BCK_A::_001,
            2 => BCK_A::_010,
            3 => BCK_A::_011,
            4 => BCK_A::_100,
            5 => BCK_A::_101,
            6 => BCK_A::_110,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == BCK_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == BCK_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == BCK_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == BCK_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == BCK_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == BCK_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == BCK_A::_110
    }
}
#[doc = "Field `BCK` writer - External Bus Clock (BCLK) Select"]
pub type BCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCKDIVCR_SPEC, u8, BCK_A, 3, O>;
impl<'a, const O: u8> BCK_W<'a, O> {
    #[doc = "/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(BCK_A::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(BCK_A::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(BCK_A::_010)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(BCK_A::_011)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(BCK_A::_100)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(BCK_A::_101)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(BCK_A::_110)
    }
}
#[doc = "Field `ICK` reader - System Clock (ICLK) Select"]
pub type ICK_R = crate::FieldReader<u8, ICK_A>;
#[doc = "System Clock (ICLK) Select\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICK_A {
    #[doc = "0: /1"]
    _000 = 0,
    #[doc = "1: /2"]
    _001 = 1,
    #[doc = "2: /4"]
    _010 = 2,
    #[doc = "3: /8"]
    _011 = 3,
    #[doc = "4: /16"]
    _100 = 4,
    #[doc = "5: /32"]
    _101 = 5,
    #[doc = "6: /64"]
    _110 = 6,
}
impl From<ICK_A> for u8 {
    #[inline(always)]
    fn from(variant: ICK_A) -> Self {
        variant as _
    }
}
impl ICK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICK_A {
        match self.bits {
            0 => ICK_A::_000,
            1 => ICK_A::_001,
            2 => ICK_A::_010,
            3 => ICK_A::_011,
            4 => ICK_A::_100,
            5 => ICK_A::_101,
            6 => ICK_A::_110,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == ICK_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == ICK_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == ICK_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == ICK_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == ICK_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == ICK_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == ICK_A::_110
    }
}
#[doc = "Field `ICK` writer - System Clock (ICLK) Select"]
pub type ICK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCKDIVCR_SPEC, u8, ICK_A, 3, O>;
impl<'a, const O: u8> ICK_W<'a, O> {
    #[doc = "/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(ICK_A::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(ICK_A::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(ICK_A::_010)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(ICK_A::_011)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(ICK_A::_100)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(ICK_A::_101)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(ICK_A::_110)
    }
}
#[doc = "Field `FCK` reader - Flash IF Clock (FCLK) Select"]
pub type FCK_R = crate::FieldReader<u8, FCK_A>;
#[doc = "Flash IF Clock (FCLK) Select\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FCK_A {
    #[doc = "0: /1"]
    _000 = 0,
    #[doc = "1: /2"]
    _001 = 1,
    #[doc = "2: /4"]
    _010 = 2,
    #[doc = "3: /8"]
    _011 = 3,
    #[doc = "4: /16"]
    _100 = 4,
    #[doc = "5: /32"]
    _101 = 5,
    #[doc = "6: /64"]
    _110 = 6,
}
impl From<FCK_A> for u8 {
    #[inline(always)]
    fn from(variant: FCK_A) -> Self {
        variant as _
    }
}
impl FCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCK_A {
        match self.bits {
            0 => FCK_A::_000,
            1 => FCK_A::_001,
            2 => FCK_A::_010,
            3 => FCK_A::_011,
            4 => FCK_A::_100,
            5 => FCK_A::_101,
            6 => FCK_A::_110,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == FCK_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == FCK_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == FCK_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == FCK_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == FCK_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == FCK_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == FCK_A::_110
    }
}
#[doc = "Field `FCK` writer - Flash IF Clock (FCLK) Select"]
pub type FCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCKDIVCR_SPEC, u8, FCK_A, 3, O>;
impl<'a, const O: u8> FCK_W<'a, O> {
    #[doc = "/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(FCK_A::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(FCK_A::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(FCK_A::_010)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(FCK_A::_011)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(FCK_A::_100)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(FCK_A::_101)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(FCK_A::_110)
    }
}
impl R {
    #[doc = "Bits 0:2 - Peripheral Module Clock D (PCLKD) Select"]
    #[inline(always)]
    pub fn pckd(&self) -> PCKD_R {
        PCKD_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Peripheral Module Clock C (PCLKC) Select"]
    #[inline(always)]
    pub fn pckc(&self) -> PCKC_R {
        PCKC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Peripheral Module Clock B (PCLKB) Select"]
    #[inline(always)]
    pub fn pckb(&self) -> PCKB_R {
        PCKB_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Peripheral Module Clock A (PCLKA) Select"]
    #[inline(always)]
    pub fn pcka(&self) -> PCKA_R {
        PCKA_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - External Bus Clock (BCLK) Select"]
    #[inline(always)]
    pub fn bck(&self) -> BCK_R {
        BCK_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - System Clock (ICLK) Select"]
    #[inline(always)]
    pub fn ick(&self) -> ICK_R {
        ICK_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Flash IF Clock (FCLK) Select"]
    #[inline(always)]
    pub fn fck(&self) -> FCK_R {
        FCK_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Peripheral Module Clock D (PCLKD) Select"]
    #[inline(always)]
    #[must_use]
    pub fn pckd(&mut self) -> PCKD_W<0> {
        PCKD_W::new(self)
    }
    #[doc = "Bits 4:6 - Peripheral Module Clock C (PCLKC) Select"]
    #[inline(always)]
    #[must_use]
    pub fn pckc(&mut self) -> PCKC_W<4> {
        PCKC_W::new(self)
    }
    #[doc = "Bits 8:10 - Peripheral Module Clock B (PCLKB) Select"]
    #[inline(always)]
    #[must_use]
    pub fn pckb(&mut self) -> PCKB_W<8> {
        PCKB_W::new(self)
    }
    #[doc = "Bits 12:14 - Peripheral Module Clock A (PCLKA) Select"]
    #[inline(always)]
    #[must_use]
    pub fn pcka(&mut self) -> PCKA_W<12> {
        PCKA_W::new(self)
    }
    #[doc = "Bits 16:18 - External Bus Clock (BCLK) Select"]
    #[inline(always)]
    #[must_use]
    pub fn bck(&mut self) -> BCK_W<16> {
        BCK_W::new(self)
    }
    #[doc = "Bits 24:26 - System Clock (ICLK) Select"]
    #[inline(always)]
    #[must_use]
    pub fn ick(&mut self) -> ICK_W<24> {
        ICK_W::new(self)
    }
    #[doc = "Bits 28:30 - Flash IF Clock (FCLK) Select"]
    #[inline(always)]
    #[must_use]
    pub fn fck(&mut self) -> FCK_W<28> {
        FCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Division Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sckdivcr](index.html) module"]
pub struct SCKDIVCR_SPEC;
impl crate::RegisterSpec for SCKDIVCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sckdivcr::R](R) reader structure"]
impl crate::Readable for SCKDIVCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sckdivcr::W](W) writer structure"]
impl crate::Writable for SCKDIVCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCKDIVCR to value 0x4404_4444"]
impl crate::Resettable for SCKDIVCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4404_4444;
}
