#[doc = "Register `BRDYSTS` reader"]
pub struct R(crate::R<BRDYSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRDYSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRDYSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRDYSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRDYSTS` writer"]
pub struct W(crate::W<BRDYSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRDYSTS_SPEC>;
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
impl From<crate::W<BRDYSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRDYSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIPE0BRDY` reader - BRDY Interrupt Status for Pipe 0"]
pub type PIPE0BRDY_R = crate::BitReader<PIPE0BRDY_A>;
#[doc = "BRDY Interrupt Status for Pipe 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE0BRDY_A {
    #[doc = "0: No BRDY interrupt occurred"]
    _0 = 0,
    #[doc = "1: BRDY interrupt occurred"]
    _1 = 1,
}
impl From<PIPE0BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE0BRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE0BRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE0BRDY_A {
        match self.bits {
            false => PIPE0BRDY_A::_0,
            true => PIPE0BRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE0BRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE0BRDY_A::_1
    }
}
#[doc = "Field `PIPE0BRDY` writer - BRDY Interrupt Status for Pipe 0"]
pub type PIPE0BRDY_W<'a, const O: u8> = crate::BitWriter<'a, u16, BRDYSTS_SPEC, PIPE0BRDY_A, O>;
impl<'a, const O: u8> PIPE0BRDY_W<'a, O> {
    #[doc = "No BRDY interrupt occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE0BRDY_A::_0)
    }
    #[doc = "BRDY interrupt occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE0BRDY_A::_1)
    }
}
#[doc = "Field `PIPE1BRDY` reader - BRDY Interrupt Status for Pipe 1"]
pub type PIPE1BRDY_R = crate::BitReader<PIPE1BRDY_A>;
#[doc = "BRDY Interrupt Status for Pipe 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE1BRDY_A {
    #[doc = "0: No BRDY interrupt occurred"]
    _0 = 0,
    #[doc = "1: BRDY interrupt occurred"]
    _1 = 1,
}
impl From<PIPE1BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE1BRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE1BRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE1BRDY_A {
        match self.bits {
            false => PIPE1BRDY_A::_0,
            true => PIPE1BRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE1BRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE1BRDY_A::_1
    }
}
#[doc = "Field `PIPE1BRDY` writer - BRDY Interrupt Status for Pipe 1"]
pub type PIPE1BRDY_W<'a, const O: u8> = crate::BitWriter<'a, u16, BRDYSTS_SPEC, PIPE1BRDY_A, O>;
impl<'a, const O: u8> PIPE1BRDY_W<'a, O> {
    #[doc = "No BRDY interrupt occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE1BRDY_A::_0)
    }
    #[doc = "BRDY interrupt occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE1BRDY_A::_1)
    }
}
#[doc = "Field `PIPE2BRDY` reader - BRDY Interrupt Status for Pipe 2"]
pub type PIPE2BRDY_R = crate::BitReader<PIPE2BRDY_A>;
#[doc = "BRDY Interrupt Status for Pipe 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE2BRDY_A {
    #[doc = "0: No BRDY interrupt occurred"]
    _0 = 0,
    #[doc = "1: BRDY interrupt occurred"]
    _1 = 1,
}
impl From<PIPE2BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE2BRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE2BRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE2BRDY_A {
        match self.bits {
            false => PIPE2BRDY_A::_0,
            true => PIPE2BRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE2BRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE2BRDY_A::_1
    }
}
#[doc = "Field `PIPE2BRDY` writer - BRDY Interrupt Status for Pipe 2"]
pub type PIPE2BRDY_W<'a, const O: u8> = crate::BitWriter<'a, u16, BRDYSTS_SPEC, PIPE2BRDY_A, O>;
impl<'a, const O: u8> PIPE2BRDY_W<'a, O> {
    #[doc = "No BRDY interrupt occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE2BRDY_A::_0)
    }
    #[doc = "BRDY interrupt occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE2BRDY_A::_1)
    }
}
#[doc = "Field `PIPE3BRDY` reader - BRDY Interrupt Status for Pipe 3"]
pub type PIPE3BRDY_R = crate::BitReader<PIPE3BRDY_A>;
#[doc = "BRDY Interrupt Status for Pipe 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE3BRDY_A {
    #[doc = "0: No BRDY interrupt occurred"]
    _0 = 0,
    #[doc = "1: BRDY interrupt occurred"]
    _1 = 1,
}
impl From<PIPE3BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE3BRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE3BRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE3BRDY_A {
        match self.bits {
            false => PIPE3BRDY_A::_0,
            true => PIPE3BRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE3BRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE3BRDY_A::_1
    }
}
#[doc = "Field `PIPE3BRDY` writer - BRDY Interrupt Status for Pipe 3"]
pub type PIPE3BRDY_W<'a, const O: u8> = crate::BitWriter<'a, u16, BRDYSTS_SPEC, PIPE3BRDY_A, O>;
impl<'a, const O: u8> PIPE3BRDY_W<'a, O> {
    #[doc = "No BRDY interrupt occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE3BRDY_A::_0)
    }
    #[doc = "BRDY interrupt occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE3BRDY_A::_1)
    }
}
#[doc = "Field `PIPE4BRDY` reader - BRDY Interrupt Status for Pipe 4"]
pub type PIPE4BRDY_R = crate::BitReader<PIPE4BRDY_A>;
#[doc = "BRDY Interrupt Status for Pipe 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE4BRDY_A {
    #[doc = "0: No BRDY interrupt occurred"]
    _0 = 0,
    #[doc = "1: BRDY interrupt occurred"]
    _1 = 1,
}
impl From<PIPE4BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE4BRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE4BRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE4BRDY_A {
        match self.bits {
            false => PIPE4BRDY_A::_0,
            true => PIPE4BRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE4BRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE4BRDY_A::_1
    }
}
#[doc = "Field `PIPE4BRDY` writer - BRDY Interrupt Status for Pipe 4"]
pub type PIPE4BRDY_W<'a, const O: u8> = crate::BitWriter<'a, u16, BRDYSTS_SPEC, PIPE4BRDY_A, O>;
impl<'a, const O: u8> PIPE4BRDY_W<'a, O> {
    #[doc = "No BRDY interrupt occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE4BRDY_A::_0)
    }
    #[doc = "BRDY interrupt occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE4BRDY_A::_1)
    }
}
#[doc = "Field `PIPE5BRDY` reader - BRDY Interrupt Status for Pipe 5"]
pub type PIPE5BRDY_R = crate::BitReader<PIPE5BRDY_A>;
#[doc = "BRDY Interrupt Status for Pipe 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE5BRDY_A {
    #[doc = "0: No BRDY interrupt occurred"]
    _0 = 0,
    #[doc = "1: BRDY interrupt occurred"]
    _1 = 1,
}
impl From<PIPE5BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE5BRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE5BRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE5BRDY_A {
        match self.bits {
            false => PIPE5BRDY_A::_0,
            true => PIPE5BRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE5BRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE5BRDY_A::_1
    }
}
#[doc = "Field `PIPE5BRDY` writer - BRDY Interrupt Status for Pipe 5"]
pub type PIPE5BRDY_W<'a, const O: u8> = crate::BitWriter<'a, u16, BRDYSTS_SPEC, PIPE5BRDY_A, O>;
impl<'a, const O: u8> PIPE5BRDY_W<'a, O> {
    #[doc = "No BRDY interrupt occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE5BRDY_A::_0)
    }
    #[doc = "BRDY interrupt occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE5BRDY_A::_1)
    }
}
#[doc = "Field `PIPE6BRDY` reader - BRDY Interrupt Status for Pipe 6"]
pub type PIPE6BRDY_R = crate::BitReader<PIPE6BRDY_A>;
#[doc = "BRDY Interrupt Status for Pipe 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE6BRDY_A {
    #[doc = "0: No BRDY interrupt occurred"]
    _0 = 0,
    #[doc = "1: BRDY interrupt occurred"]
    _1 = 1,
}
impl From<PIPE6BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE6BRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE6BRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE6BRDY_A {
        match self.bits {
            false => PIPE6BRDY_A::_0,
            true => PIPE6BRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE6BRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE6BRDY_A::_1
    }
}
#[doc = "Field `PIPE6BRDY` writer - BRDY Interrupt Status for Pipe 6"]
pub type PIPE6BRDY_W<'a, const O: u8> = crate::BitWriter<'a, u16, BRDYSTS_SPEC, PIPE6BRDY_A, O>;
impl<'a, const O: u8> PIPE6BRDY_W<'a, O> {
    #[doc = "No BRDY interrupt occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE6BRDY_A::_0)
    }
    #[doc = "BRDY interrupt occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE6BRDY_A::_1)
    }
}
#[doc = "Field `PIPE7BRDY` reader - BRDY Interrupt Status for Pipe 7"]
pub type PIPE7BRDY_R = crate::BitReader<PIPE7BRDY_A>;
#[doc = "BRDY Interrupt Status for Pipe 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE7BRDY_A {
    #[doc = "0: No BRDY interrupt occurred"]
    _0 = 0,
    #[doc = "1: BRDY interrupt occurred"]
    _1 = 1,
}
impl From<PIPE7BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE7BRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE7BRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE7BRDY_A {
        match self.bits {
            false => PIPE7BRDY_A::_0,
            true => PIPE7BRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE7BRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE7BRDY_A::_1
    }
}
#[doc = "Field `PIPE7BRDY` writer - BRDY Interrupt Status for Pipe 7"]
pub type PIPE7BRDY_W<'a, const O: u8> = crate::BitWriter<'a, u16, BRDYSTS_SPEC, PIPE7BRDY_A, O>;
impl<'a, const O: u8> PIPE7BRDY_W<'a, O> {
    #[doc = "No BRDY interrupt occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE7BRDY_A::_0)
    }
    #[doc = "BRDY interrupt occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE7BRDY_A::_1)
    }
}
#[doc = "Field `PIPE8BRDY` reader - BRDY Interrupt Status for Pipe 8"]
pub type PIPE8BRDY_R = crate::BitReader<PIPE8BRDY_A>;
#[doc = "BRDY Interrupt Status for Pipe 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE8BRDY_A {
    #[doc = "0: No BRDY interrupt occurred"]
    _0 = 0,
    #[doc = "1: BRDY interrupt occurred"]
    _1 = 1,
}
impl From<PIPE8BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE8BRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE8BRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE8BRDY_A {
        match self.bits {
            false => PIPE8BRDY_A::_0,
            true => PIPE8BRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE8BRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE8BRDY_A::_1
    }
}
#[doc = "Field `PIPE8BRDY` writer - BRDY Interrupt Status for Pipe 8"]
pub type PIPE8BRDY_W<'a, const O: u8> = crate::BitWriter<'a, u16, BRDYSTS_SPEC, PIPE8BRDY_A, O>;
impl<'a, const O: u8> PIPE8BRDY_W<'a, O> {
    #[doc = "No BRDY interrupt occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE8BRDY_A::_0)
    }
    #[doc = "BRDY interrupt occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE8BRDY_A::_1)
    }
}
#[doc = "Field `PIPE9BRDY` reader - BRDY Interrupt Status for Pipe 9"]
pub type PIPE9BRDY_R = crate::BitReader<PIPE9BRDY_A>;
#[doc = "BRDY Interrupt Status for Pipe 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE9BRDY_A {
    #[doc = "0: No BRDY interrupt occurred"]
    _0 = 0,
    #[doc = "1: BRDY interrupt occurred"]
    _1 = 1,
}
impl From<PIPE9BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE9BRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE9BRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIPE9BRDY_A {
        match self.bits {
            false => PIPE9BRDY_A::_0,
            true => PIPE9BRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE9BRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE9BRDY_A::_1
    }
}
#[doc = "Field `PIPE9BRDY` writer - BRDY Interrupt Status for Pipe 9"]
pub type PIPE9BRDY_W<'a, const O: u8> = crate::BitWriter<'a, u16, BRDYSTS_SPEC, PIPE9BRDY_A, O>;
impl<'a, const O: u8> PIPE9BRDY_W<'a, O> {
    #[doc = "No BRDY interrupt occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPE9BRDY_A::_0)
    }
    #[doc = "BRDY interrupt occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPE9BRDY_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - BRDY Interrupt Status for Pipe 0"]
    #[inline(always)]
    pub fn pipe0brdy(&self) -> PIPE0BRDY_R {
        PIPE0BRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BRDY Interrupt Status for Pipe 1"]
    #[inline(always)]
    pub fn pipe1brdy(&self) -> PIPE1BRDY_R {
        PIPE1BRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BRDY Interrupt Status for Pipe 2"]
    #[inline(always)]
    pub fn pipe2brdy(&self) -> PIPE2BRDY_R {
        PIPE2BRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BRDY Interrupt Status for Pipe 3"]
    #[inline(always)]
    pub fn pipe3brdy(&self) -> PIPE3BRDY_R {
        PIPE3BRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BRDY Interrupt Status for Pipe 4"]
    #[inline(always)]
    pub fn pipe4brdy(&self) -> PIPE4BRDY_R {
        PIPE4BRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BRDY Interrupt Status for Pipe 5"]
    #[inline(always)]
    pub fn pipe5brdy(&self) -> PIPE5BRDY_R {
        PIPE5BRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BRDY Interrupt Status for Pipe 6"]
    #[inline(always)]
    pub fn pipe6brdy(&self) -> PIPE6BRDY_R {
        PIPE6BRDY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BRDY Interrupt Status for Pipe 7"]
    #[inline(always)]
    pub fn pipe7brdy(&self) -> PIPE7BRDY_R {
        PIPE7BRDY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BRDY Interrupt Status for Pipe 8"]
    #[inline(always)]
    pub fn pipe8brdy(&self) -> PIPE8BRDY_R {
        PIPE8BRDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BRDY Interrupt Status for Pipe 9"]
    #[inline(always)]
    pub fn pipe9brdy(&self) -> PIPE9BRDY_R {
        PIPE9BRDY_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BRDY Interrupt Status for Pipe 0"]
    #[inline(always)]
    #[must_use]
    pub fn pipe0brdy(&mut self) -> PIPE0BRDY_W<0> {
        PIPE0BRDY_W::new(self)
    }
    #[doc = "Bit 1 - BRDY Interrupt Status for Pipe 1"]
    #[inline(always)]
    #[must_use]
    pub fn pipe1brdy(&mut self) -> PIPE1BRDY_W<1> {
        PIPE1BRDY_W::new(self)
    }
    #[doc = "Bit 2 - BRDY Interrupt Status for Pipe 2"]
    #[inline(always)]
    #[must_use]
    pub fn pipe2brdy(&mut self) -> PIPE2BRDY_W<2> {
        PIPE2BRDY_W::new(self)
    }
    #[doc = "Bit 3 - BRDY Interrupt Status for Pipe 3"]
    #[inline(always)]
    #[must_use]
    pub fn pipe3brdy(&mut self) -> PIPE3BRDY_W<3> {
        PIPE3BRDY_W::new(self)
    }
    #[doc = "Bit 4 - BRDY Interrupt Status for Pipe 4"]
    #[inline(always)]
    #[must_use]
    pub fn pipe4brdy(&mut self) -> PIPE4BRDY_W<4> {
        PIPE4BRDY_W::new(self)
    }
    #[doc = "Bit 5 - BRDY Interrupt Status for Pipe 5"]
    #[inline(always)]
    #[must_use]
    pub fn pipe5brdy(&mut self) -> PIPE5BRDY_W<5> {
        PIPE5BRDY_W::new(self)
    }
    #[doc = "Bit 6 - BRDY Interrupt Status for Pipe 6"]
    #[inline(always)]
    #[must_use]
    pub fn pipe6brdy(&mut self) -> PIPE6BRDY_W<6> {
        PIPE6BRDY_W::new(self)
    }
    #[doc = "Bit 7 - BRDY Interrupt Status for Pipe 7"]
    #[inline(always)]
    #[must_use]
    pub fn pipe7brdy(&mut self) -> PIPE7BRDY_W<7> {
        PIPE7BRDY_W::new(self)
    }
    #[doc = "Bit 8 - BRDY Interrupt Status for Pipe 8"]
    #[inline(always)]
    #[must_use]
    pub fn pipe8brdy(&mut self) -> PIPE8BRDY_W<8> {
        PIPE8BRDY_W::new(self)
    }
    #[doc = "Bit 9 - BRDY Interrupt Status for Pipe 9"]
    #[inline(always)]
    #[must_use]
    pub fn pipe9brdy(&mut self) -> PIPE9BRDY_W<9> {
        PIPE9BRDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BRDY Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brdysts](index.html) module"]
pub struct BRDYSTS_SPEC;
impl crate::RegisterSpec for BRDYSTS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [brdysts::R](R) reader structure"]
impl crate::Readable for BRDYSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brdysts::W](W) writer structure"]
impl crate::Writable for BRDYSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BRDYSTS to value 0"]
impl crate::Resettable for BRDYSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
