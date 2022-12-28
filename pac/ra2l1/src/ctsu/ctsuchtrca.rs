#[doc = "Register `CTSUCHTRCA` reader"]
pub struct R(crate::R<CTSUCHTRCA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUCHTRCA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUCHTRCA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUCHTRCA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUCHTRCA` writer"]
pub struct W(crate::W<CTSUCHTRCA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUCHTRCA_SPEC>;
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
impl From<crate::W<CTSUCHTRCA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUCHTRCA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHTRC00` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC00_R = crate::BitReader<CHTRC00_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC00_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC00_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC00_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC00_A {
        match self.bits {
            false => CHTRC00_A::_0,
            true => CHTRC00_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC00_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC00_A::_1
    }
}
#[doc = "Field `CHTRC00` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC00_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC00_A, O>;
impl<'a, const O: u8> CHTRC00_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC00_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC00_A::_1)
    }
}
#[doc = "Field `CHTRC02` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC02_R = crate::BitReader<CHTRC02_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC02_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC02_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC02_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC02_A {
        match self.bits {
            false => CHTRC02_A::_0,
            true => CHTRC02_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC02_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC02_A::_1
    }
}
#[doc = "Field `CHTRC02` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC02_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC02_A, O>;
impl<'a, const O: u8> CHTRC02_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC02_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC02_A::_1)
    }
}
#[doc = "Field `CHTRC04` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC04_R = crate::BitReader<CHTRC04_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC04_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC04_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC04_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC04_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC04_A {
        match self.bits {
            false => CHTRC04_A::_0,
            true => CHTRC04_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC04_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC04_A::_1
    }
}
#[doc = "Field `CHTRC04` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC04_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC04_A, O>;
impl<'a, const O: u8> CHTRC04_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC04_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC04_A::_1)
    }
}
#[doc = "Field `CHTRC05` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC05_R = crate::BitReader<CHTRC05_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC05_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC05_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC05_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC05_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC05_A {
        match self.bits {
            false => CHTRC05_A::_0,
            true => CHTRC05_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC05_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC05_A::_1
    }
}
#[doc = "Field `CHTRC05` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC05_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC05_A, O>;
impl<'a, const O: u8> CHTRC05_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC05_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC05_A::_1)
    }
}
#[doc = "Field `CHTRC06` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC06_R = crate::BitReader<CHTRC06_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC06_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC06_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC06_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC06_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC06_A {
        match self.bits {
            false => CHTRC06_A::_0,
            true => CHTRC06_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC06_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC06_A::_1
    }
}
#[doc = "Field `CHTRC06` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC06_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC06_A, O>;
impl<'a, const O: u8> CHTRC06_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC06_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC06_A::_1)
    }
}
#[doc = "Field `CHTRC07` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC07_R = crate::BitReader<CHTRC07_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC07_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC07_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC07_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC07_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC07_A {
        match self.bits {
            false => CHTRC07_A::_0,
            true => CHTRC07_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC07_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC07_A::_1
    }
}
#[doc = "Field `CHTRC07` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC07_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC07_A, O>;
impl<'a, const O: u8> CHTRC07_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC07_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC07_A::_1)
    }
}
#[doc = "Field `CHTRC08` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC08_R = crate::BitReader<CHTRC08_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC08_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC08_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC08_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC08_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC08_A {
        match self.bits {
            false => CHTRC08_A::_0,
            true => CHTRC08_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC08_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC08_A::_1
    }
}
#[doc = "Field `CHTRC08` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC08_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC08_A, O>;
impl<'a, const O: u8> CHTRC08_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC08_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC08_A::_1)
    }
}
#[doc = "Field `CHTRC09` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC09_R = crate::BitReader<CHTRC09_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC09_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC09_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC09_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC09_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC09_A {
        match self.bits {
            false => CHTRC09_A::_0,
            true => CHTRC09_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC09_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC09_A::_1
    }
}
#[doc = "Field `CHTRC09` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC09_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC09_A, O>;
impl<'a, const O: u8> CHTRC09_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC09_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC09_A::_1)
    }
}
#[doc = "Field `CHTRC10` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC10_R = crate::BitReader<CHTRC10_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC10_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC10_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC10_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC10_A {
        match self.bits {
            false => CHTRC10_A::_0,
            true => CHTRC10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC10_A::_1
    }
}
#[doc = "Field `CHTRC10` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC10_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC10_A, O>;
impl<'a, const O: u8> CHTRC10_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC10_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC10_A::_1)
    }
}
#[doc = "Field `CHTRC11` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC11_R = crate::BitReader<CHTRC11_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC11_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC11_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC11_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC11_A {
        match self.bits {
            false => CHTRC11_A::_0,
            true => CHTRC11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC11_A::_1
    }
}
#[doc = "Field `CHTRC11` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC11_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC11_A, O>;
impl<'a, const O: u8> CHTRC11_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC11_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC11_A::_1)
    }
}
#[doc = "Field `CHTRC12` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC12_R = crate::BitReader<CHTRC12_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC12_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC12_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC12_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC12_A {
        match self.bits {
            false => CHTRC12_A::_0,
            true => CHTRC12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC12_A::_1
    }
}
#[doc = "Field `CHTRC12` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC12_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC12_A, O>;
impl<'a, const O: u8> CHTRC12_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC12_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC12_A::_1)
    }
}
#[doc = "Field `CHTRC13` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC13_R = crate::BitReader<CHTRC13_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC13_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC13_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC13_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC13_A {
        match self.bits {
            false => CHTRC13_A::_0,
            true => CHTRC13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC13_A::_1
    }
}
#[doc = "Field `CHTRC13` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC13_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC13_A, O>;
impl<'a, const O: u8> CHTRC13_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC13_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC13_A::_1)
    }
}
#[doc = "Field `CHTRC14` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC14_R = crate::BitReader<CHTRC14_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC14_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC14_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC14_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC14_A {
        match self.bits {
            false => CHTRC14_A::_0,
            true => CHTRC14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC14_A::_1
    }
}
#[doc = "Field `CHTRC14` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC14_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC14_A, O>;
impl<'a, const O: u8> CHTRC14_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC14_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC14_A::_1)
    }
}
#[doc = "Field `CHTRC15` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC15_R = crate::BitReader<CHTRC15_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC15_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC15_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC15_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC15_A {
        match self.bits {
            false => CHTRC15_A::_0,
            true => CHTRC15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC15_A::_1
    }
}
#[doc = "Field `CHTRC15` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC15_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC15_A, O>;
impl<'a, const O: u8> CHTRC15_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC15_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC15_A::_1)
    }
}
#[doc = "Field `CHTRC16` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC16_R = crate::BitReader<CHTRC16_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC16_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC16_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC16_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC16_A {
        match self.bits {
            false => CHTRC16_A::_0,
            true => CHTRC16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC16_A::_1
    }
}
#[doc = "Field `CHTRC16` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC16_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC16_A, O>;
impl<'a, const O: u8> CHTRC16_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC16_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC16_A::_1)
    }
}
#[doc = "Field `CHTRC17` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC17_R = crate::BitReader<CHTRC17_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC17_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC17_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC17_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC17_A {
        match self.bits {
            false => CHTRC17_A::_0,
            true => CHTRC17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC17_A::_1
    }
}
#[doc = "Field `CHTRC17` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC17_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC17_A, O>;
impl<'a, const O: u8> CHTRC17_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC17_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC17_A::_1)
    }
}
#[doc = "Field `CHTRC18` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC18_R = crate::BitReader<CHTRC18_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC18_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC18_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC18_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC18_A {
        match self.bits {
            false => CHTRC18_A::_0,
            true => CHTRC18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC18_A::_1
    }
}
#[doc = "Field `CHTRC18` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC18_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC18_A, O>;
impl<'a, const O: u8> CHTRC18_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC18_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC18_A::_1)
    }
}
#[doc = "Field `CHTRC21` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC21_R = crate::BitReader<CHTRC21_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC21_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC21_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC21_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC21_A {
        match self.bits {
            false => CHTRC21_A::_0,
            true => CHTRC21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC21_A::_1
    }
}
#[doc = "Field `CHTRC21` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC21_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC21_A, O>;
impl<'a, const O: u8> CHTRC21_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC21_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC21_A::_1)
    }
}
#[doc = "Field `CHTRC22` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC22_R = crate::BitReader<CHTRC22_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC22_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC22_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC22_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC22_A {
        match self.bits {
            false => CHTRC22_A::_0,
            true => CHTRC22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC22_A::_1
    }
}
#[doc = "Field `CHTRC22` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC22_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC22_A, O>;
impl<'a, const O: u8> CHTRC22_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC22_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC22_A::_1)
    }
}
#[doc = "Field `CHTRC23` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC23_R = crate::BitReader<CHTRC23_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC23_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC23_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC23_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC23_A {
        match self.bits {
            false => CHTRC23_A::_0,
            true => CHTRC23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC23_A::_1
    }
}
#[doc = "Field `CHTRC23` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC23_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC23_A, O>;
impl<'a, const O: u8> CHTRC23_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC23_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC23_A::_1)
    }
}
#[doc = "Field `CHTRC24` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC24_R = crate::BitReader<CHTRC24_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC24_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC24_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC24_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC24_A {
        match self.bits {
            false => CHTRC24_A::_0,
            true => CHTRC24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC24_A::_1
    }
}
#[doc = "Field `CHTRC24` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC24_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC24_A, O>;
impl<'a, const O: u8> CHTRC24_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC24_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC24_A::_1)
    }
}
#[doc = "Field `CHTRC25` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC25_R = crate::BitReader<CHTRC25_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC25_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC25_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC25_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC25_A {
        match self.bits {
            false => CHTRC25_A::_0,
            true => CHTRC25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC25_A::_1
    }
}
#[doc = "Field `CHTRC25` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC25_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC25_A, O>;
impl<'a, const O: u8> CHTRC25_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC25_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC25_A::_1)
    }
}
#[doc = "Field `CHTRC26` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC26_R = crate::BitReader<CHTRC26_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC26_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC26_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC26_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC26_A {
        match self.bits {
            false => CHTRC26_A::_0,
            true => CHTRC26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC26_A::_1
    }
}
#[doc = "Field `CHTRC26` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC26_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC26_A, O>;
impl<'a, const O: u8> CHTRC26_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC26_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC26_A::_1)
    }
}
#[doc = "Field `CHTRC27` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC27_R = crate::BitReader<CHTRC27_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC27_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC27_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC27_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC27_A {
        match self.bits {
            false => CHTRC27_A::_0,
            true => CHTRC27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC27_A::_1
    }
}
#[doc = "Field `CHTRC27` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC27_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC27_A, O>;
impl<'a, const O: u8> CHTRC27_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC27_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC27_A::_1)
    }
}
#[doc = "Field `CHTRC28` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC28_R = crate::BitReader<CHTRC28_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC28_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC28_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC28_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC28_A {
        match self.bits {
            false => CHTRC28_A::_0,
            true => CHTRC28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC28_A::_1
    }
}
#[doc = "Field `CHTRC28` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC28_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC28_A, O>;
impl<'a, const O: u8> CHTRC28_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC28_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC28_A::_1)
    }
}
#[doc = "Field `CHTRC29` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC29_R = crate::BitReader<CHTRC29_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC29_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC29_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC29_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC29_A {
        match self.bits {
            false => CHTRC29_A::_0,
            true => CHTRC29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC29_A::_1
    }
}
#[doc = "Field `CHTRC29` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC29_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC29_A, O>;
impl<'a, const O: u8> CHTRC29_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC29_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC29_A::_1)
    }
}
#[doc = "Field `CHTRC30` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC30_R = crate::BitReader<CHTRC30_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC30_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC30_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC30_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC30_A {
        match self.bits {
            false => CHTRC30_A::_0,
            true => CHTRC30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC30_A::_1
    }
}
#[doc = "Field `CHTRC30` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC30_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC30_A, O>;
impl<'a, const O: u8> CHTRC30_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC30_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC30_A::_1)
    }
}
#[doc = "Field `CHTRC31` reader - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC31_R = crate::BitReader<CHTRC31_A>;
#[doc = "CTSU Channel Transmit/Receive Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC31_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC31_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC31_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC31_A {
        match self.bits {
            false => CHTRC31_A::_0,
            true => CHTRC31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC31_A::_1
    }
}
#[doc = "Field `CHTRC31` writer - CTSU Channel Transmit/Receive Control A"]
pub type CHTRC31_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCA_SPEC, CHTRC31_A, O>;
impl<'a, const O: u8> CHTRC31_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC31_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC31_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc00(&self) -> CHTRC00_R {
        CHTRC00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc02(&self) -> CHTRC02_R {
        CHTRC02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc04(&self) -> CHTRC04_R {
        CHTRC04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc05(&self) -> CHTRC05_R {
        CHTRC05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc06(&self) -> CHTRC06_R {
        CHTRC06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc07(&self) -> CHTRC07_R {
        CHTRC07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc08(&self) -> CHTRC08_R {
        CHTRC08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc09(&self) -> CHTRC09_R {
        CHTRC09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc10(&self) -> CHTRC10_R {
        CHTRC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc11(&self) -> CHTRC11_R {
        CHTRC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc12(&self) -> CHTRC12_R {
        CHTRC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc13(&self) -> CHTRC13_R {
        CHTRC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc14(&self) -> CHTRC14_R {
        CHTRC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc15(&self) -> CHTRC15_R {
        CHTRC15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc16(&self) -> CHTRC16_R {
        CHTRC16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc17(&self) -> CHTRC17_R {
        CHTRC17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc18(&self) -> CHTRC18_R {
        CHTRC18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc21(&self) -> CHTRC21_R {
        CHTRC21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc22(&self) -> CHTRC22_R {
        CHTRC22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc23(&self) -> CHTRC23_R {
        CHTRC23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc24(&self) -> CHTRC24_R {
        CHTRC24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc25(&self) -> CHTRC25_R {
        CHTRC25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc26(&self) -> CHTRC26_R {
        CHTRC26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc27(&self) -> CHTRC27_R {
        CHTRC27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc28(&self) -> CHTRC28_R {
        CHTRC28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc29(&self) -> CHTRC29_R {
        CHTRC29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc30(&self) -> CHTRC30_R {
        CHTRC30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc31(&self) -> CHTRC31_R {
        CHTRC31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc00(&mut self) -> CHTRC00_W<0> {
        CHTRC00_W::new(self)
    }
    #[doc = "Bit 2 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc02(&mut self) -> CHTRC02_W<2> {
        CHTRC02_W::new(self)
    }
    #[doc = "Bit 4 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc04(&mut self) -> CHTRC04_W<4> {
        CHTRC04_W::new(self)
    }
    #[doc = "Bit 5 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc05(&mut self) -> CHTRC05_W<5> {
        CHTRC05_W::new(self)
    }
    #[doc = "Bit 6 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc06(&mut self) -> CHTRC06_W<6> {
        CHTRC06_W::new(self)
    }
    #[doc = "Bit 7 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc07(&mut self) -> CHTRC07_W<7> {
        CHTRC07_W::new(self)
    }
    #[doc = "Bit 8 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc08(&mut self) -> CHTRC08_W<8> {
        CHTRC08_W::new(self)
    }
    #[doc = "Bit 9 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc09(&mut self) -> CHTRC09_W<9> {
        CHTRC09_W::new(self)
    }
    #[doc = "Bit 10 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc10(&mut self) -> CHTRC10_W<10> {
        CHTRC10_W::new(self)
    }
    #[doc = "Bit 11 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc11(&mut self) -> CHTRC11_W<11> {
        CHTRC11_W::new(self)
    }
    #[doc = "Bit 12 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc12(&mut self) -> CHTRC12_W<12> {
        CHTRC12_W::new(self)
    }
    #[doc = "Bit 13 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc13(&mut self) -> CHTRC13_W<13> {
        CHTRC13_W::new(self)
    }
    #[doc = "Bit 14 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc14(&mut self) -> CHTRC14_W<14> {
        CHTRC14_W::new(self)
    }
    #[doc = "Bit 15 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc15(&mut self) -> CHTRC15_W<15> {
        CHTRC15_W::new(self)
    }
    #[doc = "Bit 16 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc16(&mut self) -> CHTRC16_W<16> {
        CHTRC16_W::new(self)
    }
    #[doc = "Bit 17 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc17(&mut self) -> CHTRC17_W<17> {
        CHTRC17_W::new(self)
    }
    #[doc = "Bit 18 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc18(&mut self) -> CHTRC18_W<18> {
        CHTRC18_W::new(self)
    }
    #[doc = "Bit 21 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc21(&mut self) -> CHTRC21_W<21> {
        CHTRC21_W::new(self)
    }
    #[doc = "Bit 22 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc22(&mut self) -> CHTRC22_W<22> {
        CHTRC22_W::new(self)
    }
    #[doc = "Bit 23 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc23(&mut self) -> CHTRC23_W<23> {
        CHTRC23_W::new(self)
    }
    #[doc = "Bit 24 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc24(&mut self) -> CHTRC24_W<24> {
        CHTRC24_W::new(self)
    }
    #[doc = "Bit 25 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc25(&mut self) -> CHTRC25_W<25> {
        CHTRC25_W::new(self)
    }
    #[doc = "Bit 26 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc26(&mut self) -> CHTRC26_W<26> {
        CHTRC26_W::new(self)
    }
    #[doc = "Bit 27 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc27(&mut self) -> CHTRC27_W<27> {
        CHTRC27_W::new(self)
    }
    #[doc = "Bit 28 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc28(&mut self) -> CHTRC28_W<28> {
        CHTRC28_W::new(self)
    }
    #[doc = "Bit 29 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc29(&mut self) -> CHTRC29_W<29> {
        CHTRC29_W::new(self)
    }
    #[doc = "Bit 30 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc30(&mut self) -> CHTRC30_W<30> {
        CHTRC30_W::new(self)
    }
    #[doc = "Bit 31 - CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc31(&mut self) -> CHTRC31_W<31> {
        CHTRC31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU Channel Transmit/Receive Control Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsuchtrca](index.html) module"]
pub struct CTSUCHTRCA_SPEC;
impl crate::RegisterSpec for CTSUCHTRCA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctsuchtrca::R](R) reader structure"]
impl crate::Readable for CTSUCHTRCA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsuchtrca::W](W) writer structure"]
impl crate::Writable for CTSUCHTRCA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUCHTRCA to value 0"]
impl crate::Resettable for CTSUCHTRCA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
