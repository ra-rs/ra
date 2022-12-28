#[doc = "Register `NRDYENB` reader"]
pub struct R(crate::R<NRDYENB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NRDYENB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NRDYENB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NRDYENB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NRDYENB` writer"]
pub struct W(crate::W<NRDYENB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NRDYENB_SPEC>;
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
impl From<crate::W<NRDYENB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NRDYENB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIPE0NRDYE` reader - NRDY Interrupt Enable for Pipe 0"]
pub type PIPE0NRDYE_R = crate::BitReader<PIPE0NRDYE_A>;
#[doc = "NRDY Interrupt Enable for Pipe 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE0NRDYE_A {
    #[doc = "0: Disable interrupt request"]
    _0 = 0,
    #[doc = "1: Enable interrupt request"]
    _1 = 1,
}
impl From<PIPE0NRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE0NRDYE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE0NRDYE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE0NRDYE_A {
        match self.bits {
            false => PIPE0NRDYE_A::_0,
            true => PIPE0NRDYE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE0NRDYE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE0NRDYE_A::_1
    }
}
#[doc = "Field `PIPE0NRDYE` writer - NRDY Interrupt Enable for Pipe 0"]
pub type PIPE0NRDYE_W<'a, const O: u8> = crate::BitWriter<'a, u16, NRDYENB_SPEC, PIPE0NRDYE_A, O>;
impl<'a, const O: u8> PIPE0NRDYE_W<'a, O> {
    #[doc = "Disable interrupt request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE0NRDYE_A::_0)
    }
    #[doc = "Enable interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE0NRDYE_A::_1)
    }
}
#[doc = "Field `PIPE1NRDYE` reader - NRDY Interrupt Enable for Pipe 1"]
pub type PIPE1NRDYE_R = crate::BitReader<PIPE1NRDYE_A>;
#[doc = "NRDY Interrupt Enable for Pipe 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE1NRDYE_A {
    #[doc = "0: Disable interrupt request"]
    _0 = 0,
    #[doc = "1: Enable interrupt request"]
    _1 = 1,
}
impl From<PIPE1NRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE1NRDYE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE1NRDYE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE1NRDYE_A {
        match self.bits {
            false => PIPE1NRDYE_A::_0,
            true => PIPE1NRDYE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE1NRDYE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE1NRDYE_A::_1
    }
}
#[doc = "Field `PIPE1NRDYE` writer - NRDY Interrupt Enable for Pipe 1"]
pub type PIPE1NRDYE_W<'a, const O: u8> = crate::BitWriter<'a, u16, NRDYENB_SPEC, PIPE1NRDYE_A, O>;
impl<'a, const O: u8> PIPE1NRDYE_W<'a, O> {
    #[doc = "Disable interrupt request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE1NRDYE_A::_0)
    }
    #[doc = "Enable interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE1NRDYE_A::_1)
    }
}
#[doc = "Field `PIPE2NRDYE` reader - NRDY Interrupt Enable for Pipe 2"]
pub type PIPE2NRDYE_R = crate::BitReader<PIPE2NRDYE_A>;
#[doc = "NRDY Interrupt Enable for Pipe 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE2NRDYE_A {
    #[doc = "0: Disable interrupt request"]
    _0 = 0,
    #[doc = "1: Enable interrupt request"]
    _1 = 1,
}
impl From<PIPE2NRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE2NRDYE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE2NRDYE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE2NRDYE_A {
        match self.bits {
            false => PIPE2NRDYE_A::_0,
            true => PIPE2NRDYE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE2NRDYE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE2NRDYE_A::_1
    }
}
#[doc = "Field `PIPE2NRDYE` writer - NRDY Interrupt Enable for Pipe 2"]
pub type PIPE2NRDYE_W<'a, const O: u8> = crate::BitWriter<'a, u16, NRDYENB_SPEC, PIPE2NRDYE_A, O>;
impl<'a, const O: u8> PIPE2NRDYE_W<'a, O> {
    #[doc = "Disable interrupt request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE2NRDYE_A::_0)
    }
    #[doc = "Enable interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE2NRDYE_A::_1)
    }
}
#[doc = "Field `PIPE3NRDYE` reader - NRDY Interrupt Enable for Pipe 3"]
pub type PIPE3NRDYE_R = crate::BitReader<PIPE3NRDYE_A>;
#[doc = "NRDY Interrupt Enable for Pipe 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE3NRDYE_A {
    #[doc = "0: Disable interrupt request"]
    _0 = 0,
    #[doc = "1: Enable interrupt request"]
    _1 = 1,
}
impl From<PIPE3NRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE3NRDYE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE3NRDYE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE3NRDYE_A {
        match self.bits {
            false => PIPE3NRDYE_A::_0,
            true => PIPE3NRDYE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE3NRDYE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE3NRDYE_A::_1
    }
}
#[doc = "Field `PIPE3NRDYE` writer - NRDY Interrupt Enable for Pipe 3"]
pub type PIPE3NRDYE_W<'a, const O: u8> = crate::BitWriter<'a, u16, NRDYENB_SPEC, PIPE3NRDYE_A, O>;
impl<'a, const O: u8> PIPE3NRDYE_W<'a, O> {
    #[doc = "Disable interrupt request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE3NRDYE_A::_0)
    }
    #[doc = "Enable interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE3NRDYE_A::_1)
    }
}
#[doc = "Field `PIPE4NRDYE` reader - NRDY Interrupt Enable for Pipe 4"]
pub type PIPE4NRDYE_R = crate::BitReader<PIPE4NRDYE_A>;
#[doc = "NRDY Interrupt Enable for Pipe 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE4NRDYE_A {
    #[doc = "0: Disable interrupt request"]
    _0 = 0,
    #[doc = "1: Enable interrupt request"]
    _1 = 1,
}
impl From<PIPE4NRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE4NRDYE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE4NRDYE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE4NRDYE_A {
        match self.bits {
            false => PIPE4NRDYE_A::_0,
            true => PIPE4NRDYE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE4NRDYE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE4NRDYE_A::_1
    }
}
#[doc = "Field `PIPE4NRDYE` writer - NRDY Interrupt Enable for Pipe 4"]
pub type PIPE4NRDYE_W<'a, const O: u8> = crate::BitWriter<'a, u16, NRDYENB_SPEC, PIPE4NRDYE_A, O>;
impl<'a, const O: u8> PIPE4NRDYE_W<'a, O> {
    #[doc = "Disable interrupt request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE4NRDYE_A::_0)
    }
    #[doc = "Enable interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE4NRDYE_A::_1)
    }
}
#[doc = "Field `PIPE5NRDYE` reader - NRDY Interrupt Enable for Pipe 5"]
pub type PIPE5NRDYE_R = crate::BitReader<PIPE5NRDYE_A>;
#[doc = "NRDY Interrupt Enable for Pipe 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE5NRDYE_A {
    #[doc = "0: Disable interrupt request"]
    _0 = 0,
    #[doc = "1: Enable interrupt request"]
    _1 = 1,
}
impl From<PIPE5NRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE5NRDYE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE5NRDYE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE5NRDYE_A {
        match self.bits {
            false => PIPE5NRDYE_A::_0,
            true => PIPE5NRDYE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE5NRDYE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE5NRDYE_A::_1
    }
}
#[doc = "Field `PIPE5NRDYE` writer - NRDY Interrupt Enable for Pipe 5"]
pub type PIPE5NRDYE_W<'a, const O: u8> = crate::BitWriter<'a, u16, NRDYENB_SPEC, PIPE5NRDYE_A, O>;
impl<'a, const O: u8> PIPE5NRDYE_W<'a, O> {
    #[doc = "Disable interrupt request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE5NRDYE_A::_0)
    }
    #[doc = "Enable interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE5NRDYE_A::_1)
    }
}
#[doc = "Field `PIPE6NRDYE` reader - NRDY Interrupt Enable for Pipe 6"]
pub type PIPE6NRDYE_R = crate::BitReader<PIPE6NRDYE_A>;
#[doc = "NRDY Interrupt Enable for Pipe 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE6NRDYE_A {
    #[doc = "0: Disable interrupt request"]
    _0 = 0,
    #[doc = "1: Enable interrupt request"]
    _1 = 1,
}
impl From<PIPE6NRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE6NRDYE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE6NRDYE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE6NRDYE_A {
        match self.bits {
            false => PIPE6NRDYE_A::_0,
            true => PIPE6NRDYE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE6NRDYE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE6NRDYE_A::_1
    }
}
#[doc = "Field `PIPE6NRDYE` writer - NRDY Interrupt Enable for Pipe 6"]
pub type PIPE6NRDYE_W<'a, const O: u8> = crate::BitWriter<'a, u16, NRDYENB_SPEC, PIPE6NRDYE_A, O>;
impl<'a, const O: u8> PIPE6NRDYE_W<'a, O> {
    #[doc = "Disable interrupt request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE6NRDYE_A::_0)
    }
    #[doc = "Enable interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE6NRDYE_A::_1)
    }
}
#[doc = "Field `PIPE7NRDYE` reader - NRDY Interrupt Enable for Pipe 7"]
pub type PIPE7NRDYE_R = crate::BitReader<PIPE7NRDYE_A>;
#[doc = "NRDY Interrupt Enable for Pipe 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE7NRDYE_A {
    #[doc = "0: Disable interrupt request"]
    _0 = 0,
    #[doc = "1: Enable interrupt request"]
    _1 = 1,
}
impl From<PIPE7NRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE7NRDYE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE7NRDYE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE7NRDYE_A {
        match self.bits {
            false => PIPE7NRDYE_A::_0,
            true => PIPE7NRDYE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE7NRDYE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE7NRDYE_A::_1
    }
}
#[doc = "Field `PIPE7NRDYE` writer - NRDY Interrupt Enable for Pipe 7"]
pub type PIPE7NRDYE_W<'a, const O: u8> = crate::BitWriter<'a, u16, NRDYENB_SPEC, PIPE7NRDYE_A, O>;
impl<'a, const O: u8> PIPE7NRDYE_W<'a, O> {
    #[doc = "Disable interrupt request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE7NRDYE_A::_0)
    }
    #[doc = "Enable interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE7NRDYE_A::_1)
    }
}
#[doc = "Field `PIPE8NRDYE` reader - NRDY Interrupt Enable for Pipe 8"]
pub type PIPE8NRDYE_R = crate::BitReader<PIPE8NRDYE_A>;
#[doc = "NRDY Interrupt Enable for Pipe 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE8NRDYE_A {
    #[doc = "0: Disable interrupt request"]
    _0 = 0,
    #[doc = "1: Enable interrupt request"]
    _1 = 1,
}
impl From<PIPE8NRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE8NRDYE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE8NRDYE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE8NRDYE_A {
        match self.bits {
            false => PIPE8NRDYE_A::_0,
            true => PIPE8NRDYE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE8NRDYE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE8NRDYE_A::_1
    }
}
#[doc = "Field `PIPE8NRDYE` writer - NRDY Interrupt Enable for Pipe 8"]
pub type PIPE8NRDYE_W<'a, const O: u8> = crate::BitWriter<'a, u16, NRDYENB_SPEC, PIPE8NRDYE_A, O>;
impl<'a, const O: u8> PIPE8NRDYE_W<'a, O> {
    #[doc = "Disable interrupt request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE8NRDYE_A::_0)
    }
    #[doc = "Enable interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE8NRDYE_A::_1)
    }
}
#[doc = "Field `PIPE9NRDYE` reader - NRDY Interrupt Enable for Pipe 9"]
pub type PIPE9NRDYE_R = crate::BitReader<PIPE9NRDYE_A>;
#[doc = "NRDY Interrupt Enable for Pipe 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE9NRDYE_A {
    #[doc = "0: Disable interrupt request"]
    _0 = 0,
    #[doc = "1: Enable interrupt request"]
    _1 = 1,
}
impl From<PIPE9NRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE9NRDYE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE9NRDYE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE9NRDYE_A {
        match self.bits {
            false => PIPE9NRDYE_A::_0,
            true => PIPE9NRDYE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE9NRDYE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE9NRDYE_A::_1
    }
}
#[doc = "Field `PIPE9NRDYE` writer - NRDY Interrupt Enable for Pipe 9"]
pub type PIPE9NRDYE_W<'a, const O: u8> = crate::BitWriter<'a, u16, NRDYENB_SPEC, PIPE9NRDYE_A, O>;
impl<'a, const O: u8> PIPE9NRDYE_W<'a, O> {
    #[doc = "Disable interrupt request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE9NRDYE_A::_0)
    }
    #[doc = "Enable interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE9NRDYE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - NRDY Interrupt Enable for Pipe 0"]
    #[inline(always)]
    pub fn pipe0nrdye(&self) -> PIPE0NRDYE_R {
        PIPE0NRDYE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NRDY Interrupt Enable for Pipe 1"]
    #[inline(always)]
    pub fn pipe1nrdye(&self) -> PIPE1NRDYE_R {
        PIPE1NRDYE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NRDY Interrupt Enable for Pipe 2"]
    #[inline(always)]
    pub fn pipe2nrdye(&self) -> PIPE2NRDYE_R {
        PIPE2NRDYE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NRDY Interrupt Enable for Pipe 3"]
    #[inline(always)]
    pub fn pipe3nrdye(&self) -> PIPE3NRDYE_R {
        PIPE3NRDYE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NRDY Interrupt Enable for Pipe 4"]
    #[inline(always)]
    pub fn pipe4nrdye(&self) -> PIPE4NRDYE_R {
        PIPE4NRDYE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NRDY Interrupt Enable for Pipe 5"]
    #[inline(always)]
    pub fn pipe5nrdye(&self) -> PIPE5NRDYE_R {
        PIPE5NRDYE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NRDY Interrupt Enable for Pipe 6"]
    #[inline(always)]
    pub fn pipe6nrdye(&self) -> PIPE6NRDYE_R {
        PIPE6NRDYE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NRDY Interrupt Enable for Pipe 7"]
    #[inline(always)]
    pub fn pipe7nrdye(&self) -> PIPE7NRDYE_R {
        PIPE7NRDYE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NRDY Interrupt Enable for Pipe 8"]
    #[inline(always)]
    pub fn pipe8nrdye(&self) -> PIPE8NRDYE_R {
        PIPE8NRDYE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NRDY Interrupt Enable for Pipe 9"]
    #[inline(always)]
    pub fn pipe9nrdye(&self) -> PIPE9NRDYE_R {
        PIPE9NRDYE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NRDY Interrupt Enable for Pipe 0"]
    #[inline(always)]
    #[must_use]
    pub fn pipe0nrdye(&mut self) -> PIPE0NRDYE_W<0> {
        PIPE0NRDYE_W::new(self)
    }
    #[doc = "Bit 1 - NRDY Interrupt Enable for Pipe 1"]
    #[inline(always)]
    #[must_use]
    pub fn pipe1nrdye(&mut self) -> PIPE1NRDYE_W<1> {
        PIPE1NRDYE_W::new(self)
    }
    #[doc = "Bit 2 - NRDY Interrupt Enable for Pipe 2"]
    #[inline(always)]
    #[must_use]
    pub fn pipe2nrdye(&mut self) -> PIPE2NRDYE_W<2> {
        PIPE2NRDYE_W::new(self)
    }
    #[doc = "Bit 3 - NRDY Interrupt Enable for Pipe 3"]
    #[inline(always)]
    #[must_use]
    pub fn pipe3nrdye(&mut self) -> PIPE3NRDYE_W<3> {
        PIPE3NRDYE_W::new(self)
    }
    #[doc = "Bit 4 - NRDY Interrupt Enable for Pipe 4"]
    #[inline(always)]
    #[must_use]
    pub fn pipe4nrdye(&mut self) -> PIPE4NRDYE_W<4> {
        PIPE4NRDYE_W::new(self)
    }
    #[doc = "Bit 5 - NRDY Interrupt Enable for Pipe 5"]
    #[inline(always)]
    #[must_use]
    pub fn pipe5nrdye(&mut self) -> PIPE5NRDYE_W<5> {
        PIPE5NRDYE_W::new(self)
    }
    #[doc = "Bit 6 - NRDY Interrupt Enable for Pipe 6"]
    #[inline(always)]
    #[must_use]
    pub fn pipe6nrdye(&mut self) -> PIPE6NRDYE_W<6> {
        PIPE6NRDYE_W::new(self)
    }
    #[doc = "Bit 7 - NRDY Interrupt Enable for Pipe 7"]
    #[inline(always)]
    #[must_use]
    pub fn pipe7nrdye(&mut self) -> PIPE7NRDYE_W<7> {
        PIPE7NRDYE_W::new(self)
    }
    #[doc = "Bit 8 - NRDY Interrupt Enable for Pipe 8"]
    #[inline(always)]
    #[must_use]
    pub fn pipe8nrdye(&mut self) -> PIPE8NRDYE_W<8> {
        PIPE8NRDYE_W::new(self)
    }
    #[doc = "Bit 9 - NRDY Interrupt Enable for Pipe 9"]
    #[inline(always)]
    #[must_use]
    pub fn pipe9nrdye(&mut self) -> PIPE9NRDYE_W<9> {
        PIPE9NRDYE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NRDY Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nrdyenb](index.html) module"]
pub struct NRDYENB_SPEC;
impl crate::RegisterSpec for NRDYENB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [nrdyenb::R](R) reader structure"]
impl crate::Readable for NRDYENB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nrdyenb::W](W) writer structure"]
impl crate::Writable for NRDYENB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NRDYENB to value 0"]
impl crate::Resettable for NRDYENB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
