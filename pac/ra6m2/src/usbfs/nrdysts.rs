#[doc = "Register `NRDYSTS` reader"]
pub struct R(crate::R<NRDYSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NRDYSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NRDYSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NRDYSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NRDYSTS` writer"]
pub struct W(crate::W<NRDYSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NRDYSTS_SPEC>;
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
impl From<crate::W<NRDYSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NRDYSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIPE0NRDY` reader - NRDY Interrupt Status for PIPE0\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE0NRDY_R = crate::BitReader<PIPE0NRDY_A>;
#[doc = "NRDY Interrupt Status for PIPE0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE0NRDY_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE0NRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE0NRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE0NRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE0NRDY_A {
        match self.bits {
            false => PIPE0NRDY_A::_0,
            true => PIPE0NRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE0NRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE0NRDY_A::_1
    }
}
#[doc = "Field `PIPE0NRDY` writer - NRDY Interrupt Status for PIPE0"]
pub type PIPE0NRDY_W<'a, const O: u8> = crate::BitWriter0C<'a, u16, NRDYSTS_SPEC, PIPE0NRDY_A, O>;
impl<'a, const O: u8> PIPE0NRDY_W<'a, O> {
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE0NRDY_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE0NRDY_A::_1)
    }
}
#[doc = "Field `PIPE1NRDY` reader - NRDY Interrupt Status for PIPE1\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE1NRDY_R = crate::BitReader<PIPE1NRDY_A>;
#[doc = "NRDY Interrupt Status for PIPE1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE1NRDY_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE1NRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE1NRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE1NRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE1NRDY_A {
        match self.bits {
            false => PIPE1NRDY_A::_0,
            true => PIPE1NRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE1NRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE1NRDY_A::_1
    }
}
#[doc = "Field `PIPE1NRDY` writer - NRDY Interrupt Status for PIPE1"]
pub type PIPE1NRDY_W<'a, const O: u8> = crate::BitWriter0C<'a, u16, NRDYSTS_SPEC, PIPE1NRDY_A, O>;
impl<'a, const O: u8> PIPE1NRDY_W<'a, O> {
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE1NRDY_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE1NRDY_A::_1)
    }
}
#[doc = "Field `PIPE2NRDY` reader - NRDY Interrupt Status for PIPE2\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE2NRDY_R = crate::BitReader<PIPE2NRDY_A>;
#[doc = "NRDY Interrupt Status for PIPE2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE2NRDY_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE2NRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE2NRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE2NRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE2NRDY_A {
        match self.bits {
            false => PIPE2NRDY_A::_0,
            true => PIPE2NRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE2NRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE2NRDY_A::_1
    }
}
#[doc = "Field `PIPE2NRDY` writer - NRDY Interrupt Status for PIPE2"]
pub type PIPE2NRDY_W<'a, const O: u8> = crate::BitWriter0C<'a, u16, NRDYSTS_SPEC, PIPE2NRDY_A, O>;
impl<'a, const O: u8> PIPE2NRDY_W<'a, O> {
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE2NRDY_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE2NRDY_A::_1)
    }
}
#[doc = "Field `PIPE3NRDY` reader - NRDY Interrupt Status for PIPE3\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE3NRDY_R = crate::BitReader<PIPE3NRDY_A>;
#[doc = "NRDY Interrupt Status for PIPE3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE3NRDY_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE3NRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE3NRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE3NRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE3NRDY_A {
        match self.bits {
            false => PIPE3NRDY_A::_0,
            true => PIPE3NRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE3NRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE3NRDY_A::_1
    }
}
#[doc = "Field `PIPE3NRDY` writer - NRDY Interrupt Status for PIPE3"]
pub type PIPE3NRDY_W<'a, const O: u8> = crate::BitWriter0C<'a, u16, NRDYSTS_SPEC, PIPE3NRDY_A, O>;
impl<'a, const O: u8> PIPE3NRDY_W<'a, O> {
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE3NRDY_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE3NRDY_A::_1)
    }
}
#[doc = "Field `PIPE4NRDY` reader - NRDY Interrupt Status for PIPE4\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE4NRDY_R = crate::BitReader<PIPE4NRDY_A>;
#[doc = "NRDY Interrupt Status for PIPE4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE4NRDY_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE4NRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE4NRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE4NRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE4NRDY_A {
        match self.bits {
            false => PIPE4NRDY_A::_0,
            true => PIPE4NRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE4NRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE4NRDY_A::_1
    }
}
#[doc = "Field `PIPE4NRDY` writer - NRDY Interrupt Status for PIPE4"]
pub type PIPE4NRDY_W<'a, const O: u8> = crate::BitWriter0C<'a, u16, NRDYSTS_SPEC, PIPE4NRDY_A, O>;
impl<'a, const O: u8> PIPE4NRDY_W<'a, O> {
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE4NRDY_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE4NRDY_A::_1)
    }
}
#[doc = "Field `PIPE5NRDY` reader - NRDY Interrupt Status for PIPE5\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE5NRDY_R = crate::BitReader<PIPE5NRDY_A>;
#[doc = "NRDY Interrupt Status for PIPE5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE5NRDY_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE5NRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE5NRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE5NRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE5NRDY_A {
        match self.bits {
            false => PIPE5NRDY_A::_0,
            true => PIPE5NRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE5NRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE5NRDY_A::_1
    }
}
#[doc = "Field `PIPE5NRDY` writer - NRDY Interrupt Status for PIPE5"]
pub type PIPE5NRDY_W<'a, const O: u8> = crate::BitWriter0C<'a, u16, NRDYSTS_SPEC, PIPE5NRDY_A, O>;
impl<'a, const O: u8> PIPE5NRDY_W<'a, O> {
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE5NRDY_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE5NRDY_A::_1)
    }
}
#[doc = "Field `PIPE6NRDY` reader - NRDY Interrupt Status for PIPE6\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE6NRDY_R = crate::BitReader<PIPE6NRDY_A>;
#[doc = "NRDY Interrupt Status for PIPE6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE6NRDY_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE6NRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE6NRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE6NRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE6NRDY_A {
        match self.bits {
            false => PIPE6NRDY_A::_0,
            true => PIPE6NRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE6NRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE6NRDY_A::_1
    }
}
#[doc = "Field `PIPE6NRDY` writer - NRDY Interrupt Status for PIPE6"]
pub type PIPE6NRDY_W<'a, const O: u8> = crate::BitWriter0C<'a, u16, NRDYSTS_SPEC, PIPE6NRDY_A, O>;
impl<'a, const O: u8> PIPE6NRDY_W<'a, O> {
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE6NRDY_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE6NRDY_A::_1)
    }
}
#[doc = "Field `PIPE7NRDY` reader - NRDY Interrupt Status for PIPE7\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE7NRDY_R = crate::BitReader<PIPE7NRDY_A>;
#[doc = "NRDY Interrupt Status for PIPE7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE7NRDY_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE7NRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE7NRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE7NRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE7NRDY_A {
        match self.bits {
            false => PIPE7NRDY_A::_0,
            true => PIPE7NRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE7NRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE7NRDY_A::_1
    }
}
#[doc = "Field `PIPE7NRDY` writer - NRDY Interrupt Status for PIPE7"]
pub type PIPE7NRDY_W<'a, const O: u8> = crate::BitWriter0C<'a, u16, NRDYSTS_SPEC, PIPE7NRDY_A, O>;
impl<'a, const O: u8> PIPE7NRDY_W<'a, O> {
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE7NRDY_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE7NRDY_A::_1)
    }
}
#[doc = "Field `PIPE8NRDY` reader - NRDY Interrupt Status for PIPE8\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE8NRDY_R = crate::BitReader<PIPE8NRDY_A>;
#[doc = "NRDY Interrupt Status for PIPE8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE8NRDY_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE8NRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE8NRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE8NRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE8NRDY_A {
        match self.bits {
            false => PIPE8NRDY_A::_0,
            true => PIPE8NRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE8NRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE8NRDY_A::_1
    }
}
#[doc = "Field `PIPE8NRDY` writer - NRDY Interrupt Status for PIPE8"]
pub type PIPE8NRDY_W<'a, const O: u8> = crate::BitWriter0C<'a, u16, NRDYSTS_SPEC, PIPE8NRDY_A, O>;
impl<'a, const O: u8> PIPE8NRDY_W<'a, O> {
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE8NRDY_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE8NRDY_A::_1)
    }
}
#[doc = "Field `PIPE9NRDY` reader - NRDY Interrupt Status for PIPE9\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE9NRDY_R = crate::BitReader<PIPE9NRDY_A>;
#[doc = "NRDY Interrupt Status for PIPE9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE9NRDY_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE9NRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE9NRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE9NRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE9NRDY_A {
        match self.bits {
            false => PIPE9NRDY_A::_0,
            true => PIPE9NRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE9NRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE9NRDY_A::_1
    }
}
#[doc = "Field `PIPE9NRDY` writer - NRDY Interrupt Status for PIPE9"]
pub type PIPE9NRDY_W<'a, const O: u8> = crate::BitWriter0C<'a, u16, NRDYSTS_SPEC, PIPE9NRDY_A, O>;
impl<'a, const O: u8> PIPE9NRDY_W<'a, O> {
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE9NRDY_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE9NRDY_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - NRDY Interrupt Status for PIPE0"]
    #[inline(always)]
    pub fn pipe0nrdy(&self) -> PIPE0NRDY_R {
        PIPE0NRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NRDY Interrupt Status for PIPE1"]
    #[inline(always)]
    pub fn pipe1nrdy(&self) -> PIPE1NRDY_R {
        PIPE1NRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NRDY Interrupt Status for PIPE2"]
    #[inline(always)]
    pub fn pipe2nrdy(&self) -> PIPE2NRDY_R {
        PIPE2NRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NRDY Interrupt Status for PIPE3"]
    #[inline(always)]
    pub fn pipe3nrdy(&self) -> PIPE3NRDY_R {
        PIPE3NRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NRDY Interrupt Status for PIPE4"]
    #[inline(always)]
    pub fn pipe4nrdy(&self) -> PIPE4NRDY_R {
        PIPE4NRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NRDY Interrupt Status for PIPE5"]
    #[inline(always)]
    pub fn pipe5nrdy(&self) -> PIPE5NRDY_R {
        PIPE5NRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NRDY Interrupt Status for PIPE6"]
    #[inline(always)]
    pub fn pipe6nrdy(&self) -> PIPE6NRDY_R {
        PIPE6NRDY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NRDY Interrupt Status for PIPE7"]
    #[inline(always)]
    pub fn pipe7nrdy(&self) -> PIPE7NRDY_R {
        PIPE7NRDY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NRDY Interrupt Status for PIPE8"]
    #[inline(always)]
    pub fn pipe8nrdy(&self) -> PIPE8NRDY_R {
        PIPE8NRDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NRDY Interrupt Status for PIPE9"]
    #[inline(always)]
    pub fn pipe9nrdy(&self) -> PIPE9NRDY_R {
        PIPE9NRDY_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NRDY Interrupt Status for PIPE0"]
    #[inline(always)]
    #[must_use]
    pub fn pipe0nrdy(&mut self) -> PIPE0NRDY_W<0> {
        PIPE0NRDY_W::new(self)
    }
    #[doc = "Bit 1 - NRDY Interrupt Status for PIPE1"]
    #[inline(always)]
    #[must_use]
    pub fn pipe1nrdy(&mut self) -> PIPE1NRDY_W<1> {
        PIPE1NRDY_W::new(self)
    }
    #[doc = "Bit 2 - NRDY Interrupt Status for PIPE2"]
    #[inline(always)]
    #[must_use]
    pub fn pipe2nrdy(&mut self) -> PIPE2NRDY_W<2> {
        PIPE2NRDY_W::new(self)
    }
    #[doc = "Bit 3 - NRDY Interrupt Status for PIPE3"]
    #[inline(always)]
    #[must_use]
    pub fn pipe3nrdy(&mut self) -> PIPE3NRDY_W<3> {
        PIPE3NRDY_W::new(self)
    }
    #[doc = "Bit 4 - NRDY Interrupt Status for PIPE4"]
    #[inline(always)]
    #[must_use]
    pub fn pipe4nrdy(&mut self) -> PIPE4NRDY_W<4> {
        PIPE4NRDY_W::new(self)
    }
    #[doc = "Bit 5 - NRDY Interrupt Status for PIPE5"]
    #[inline(always)]
    #[must_use]
    pub fn pipe5nrdy(&mut self) -> PIPE5NRDY_W<5> {
        PIPE5NRDY_W::new(self)
    }
    #[doc = "Bit 6 - NRDY Interrupt Status for PIPE6"]
    #[inline(always)]
    #[must_use]
    pub fn pipe6nrdy(&mut self) -> PIPE6NRDY_W<6> {
        PIPE6NRDY_W::new(self)
    }
    #[doc = "Bit 7 - NRDY Interrupt Status for PIPE7"]
    #[inline(always)]
    #[must_use]
    pub fn pipe7nrdy(&mut self) -> PIPE7NRDY_W<7> {
        PIPE7NRDY_W::new(self)
    }
    #[doc = "Bit 8 - NRDY Interrupt Status for PIPE8"]
    #[inline(always)]
    #[must_use]
    pub fn pipe8nrdy(&mut self) -> PIPE8NRDY_W<8> {
        PIPE8NRDY_W::new(self)
    }
    #[doc = "Bit 9 - NRDY Interrupt Status for PIPE9"]
    #[inline(always)]
    #[must_use]
    pub fn pipe9nrdy(&mut self) -> PIPE9NRDY_W<9> {
        PIPE9NRDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NRDY Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nrdysts](index.html) module"]
pub struct NRDYSTS_SPEC;
impl crate::RegisterSpec for NRDYSTS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [nrdysts::R](R) reader structure"]
impl crate::Readable for NRDYSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nrdysts::W](W) writer structure"]
impl crate::Writable for NRDYSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x03ff;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NRDYSTS to value 0"]
impl crate::Resettable for NRDYSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
