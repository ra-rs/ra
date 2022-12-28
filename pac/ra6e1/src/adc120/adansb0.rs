#[doc = "Register `ADANSB0` reader"]
pub struct R(crate::R<ADANSB0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADANSB0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADANSB0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADANSB0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADANSB0` writer"]
pub struct W(crate::W<ADANSB0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADANSB0_SPEC>;
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
impl From<crate::W<ADANSB0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADANSB0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANSB0` reader - A/D Conversion Channels Select n"]
pub type ANSB0_R = crate::BitReader<ANSB0_A>;
#[doc = "A/D Conversion Channels Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB0_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSB0_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB0_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB0_A {
        match self.bits {
            false => ANSB0_A::_0,
            true => ANSB0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB0_A::_1
    }
}
#[doc = "Field `ANSB0` writer - A/D Conversion Channels Select n"]
pub type ANSB0_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB0_A, O>;
impl<'a, const O: u8> ANSB0_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB0_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB0_A::_1)
    }
}
#[doc = "Field `ANSB1` reader - A/D Conversion Channels Select n"]
pub type ANSB1_R = crate::BitReader<ANSB1_A>;
#[doc = "A/D Conversion Channels Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB1_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSB1_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB1_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB1_A {
        match self.bits {
            false => ANSB1_A::_0,
            true => ANSB1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB1_A::_1
    }
}
#[doc = "Field `ANSB1` writer - A/D Conversion Channels Select n"]
pub type ANSB1_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB1_A, O>;
impl<'a, const O: u8> ANSB1_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB1_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB1_A::_1)
    }
}
#[doc = "Field `ANSB2` reader - A/D Conversion Channels Select n"]
pub type ANSB2_R = crate::BitReader<ANSB2_A>;
#[doc = "A/D Conversion Channels Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB2_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSB2_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB2_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB2_A {
        match self.bits {
            false => ANSB2_A::_0,
            true => ANSB2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB2_A::_1
    }
}
#[doc = "Field `ANSB2` writer - A/D Conversion Channels Select n"]
pub type ANSB2_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB2_A, O>;
impl<'a, const O: u8> ANSB2_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB2_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB2_A::_1)
    }
}
#[doc = "Field `ANSB3` reader - A/D Conversion Channels Select n"]
pub type ANSB3_R = crate::BitReader<ANSB3_A>;
#[doc = "A/D Conversion Channels Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB3_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSB3_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB3_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB3_A {
        match self.bits {
            false => ANSB3_A::_0,
            true => ANSB3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB3_A::_1
    }
}
#[doc = "Field `ANSB3` writer - A/D Conversion Channels Select n"]
pub type ANSB3_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB3_A, O>;
impl<'a, const O: u8> ANSB3_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB3_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB3_A::_1)
    }
}
#[doc = "Field `ANSB4` reader - A/D Conversion Channels Select n"]
pub type ANSB4_R = crate::BitReader<ANSB4_A>;
#[doc = "A/D Conversion Channels Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB4_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSB4_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB4_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB4_A {
        match self.bits {
            false => ANSB4_A::_0,
            true => ANSB4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB4_A::_1
    }
}
#[doc = "Field `ANSB4` writer - A/D Conversion Channels Select n"]
pub type ANSB4_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB4_A, O>;
impl<'a, const O: u8> ANSB4_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB4_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB4_A::_1)
    }
}
#[doc = "Field `ANSB5` reader - A/D Conversion Channels Select n"]
pub type ANSB5_R = crate::BitReader<ANSB5_A>;
#[doc = "A/D Conversion Channels Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB5_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSB5_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB5_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB5_A {
        match self.bits {
            false => ANSB5_A::_0,
            true => ANSB5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB5_A::_1
    }
}
#[doc = "Field `ANSB5` writer - A/D Conversion Channels Select n"]
pub type ANSB5_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB5_A, O>;
impl<'a, const O: u8> ANSB5_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB5_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB5_A::_1)
    }
}
#[doc = "Field `ANSB6` reader - A/D Conversion Channels Select n"]
pub type ANSB6_R = crate::BitReader<ANSB6_A>;
#[doc = "A/D Conversion Channels Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB6_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSB6_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB6_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB6_A {
        match self.bits {
            false => ANSB6_A::_0,
            true => ANSB6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB6_A::_1
    }
}
#[doc = "Field `ANSB6` writer - A/D Conversion Channels Select n"]
pub type ANSB6_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB6_A, O>;
impl<'a, const O: u8> ANSB6_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB6_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB6_A::_1)
    }
}
#[doc = "Field `ANSB7` reader - A/D Conversion Channels Select n"]
pub type ANSB7_R = crate::BitReader<ANSB7_A>;
#[doc = "A/D Conversion Channels Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB7_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSB7_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB7_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB7_A {
        match self.bits {
            false => ANSB7_A::_0,
            true => ANSB7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB7_A::_1
    }
}
#[doc = "Field `ANSB7` writer - A/D Conversion Channels Select n"]
pub type ANSB7_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB7_A, O>;
impl<'a, const O: u8> ANSB7_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB7_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB7_A::_1)
    }
}
#[doc = "Field `ANSB8` reader - A/D Conversion Channels Select n"]
pub type ANSB8_R = crate::BitReader<ANSB8_A>;
#[doc = "A/D Conversion Channels Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB8_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSB8_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB8_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB8_A {
        match self.bits {
            false => ANSB8_A::_0,
            true => ANSB8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB8_A::_1
    }
}
#[doc = "Field `ANSB8` writer - A/D Conversion Channels Select n"]
pub type ANSB8_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB8_A, O>;
impl<'a, const O: u8> ANSB8_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB8_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB8_A::_1)
    }
}
#[doc = "Field `ANSB12` reader - A/D Conversion Channels Select n"]
pub type ANSB12_R = crate::BitReader<ANSB12_A>;
#[doc = "A/D Conversion Channels Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB12_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSB12_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB12_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB12_A {
        match self.bits {
            false => ANSB12_A::_0,
            true => ANSB12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB12_A::_1
    }
}
#[doc = "Field `ANSB12` writer - A/D Conversion Channels Select n"]
pub type ANSB12_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB12_A, O>;
impl<'a, const O: u8> ANSB12_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB12_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB12_A::_1)
    }
}
#[doc = "Field `ANSB13` reader - A/D Conversion Channels Select n"]
pub type ANSB13_R = crate::BitReader<ANSB13_A>;
#[doc = "A/D Conversion Channels Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB13_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSB13_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB13_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB13_A {
        match self.bits {
            false => ANSB13_A::_0,
            true => ANSB13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB13_A::_1
    }
}
#[doc = "Field `ANSB13` writer - A/D Conversion Channels Select n"]
pub type ANSB13_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB13_A, O>;
impl<'a, const O: u8> ANSB13_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB13_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB13_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansb0(&self) -> ANSB0_R {
        ANSB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansb1(&self) -> ANSB1_R {
        ANSB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansb2(&self) -> ANSB2_R {
        ANSB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansb3(&self) -> ANSB3_R {
        ANSB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansb4(&self) -> ANSB4_R {
        ANSB4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansb5(&self) -> ANSB5_R {
        ANSB5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansb6(&self) -> ANSB6_R {
        ANSB6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansb7(&self) -> ANSB7_R {
        ANSB7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansb8(&self) -> ANSB8_R {
        ANSB8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansb12(&self) -> ANSB12_R {
        ANSB12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansb13(&self) -> ANSB13_R {
        ANSB13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A/D Conversion Channels Select n"]
    #[inline(always)]
    #[must_use]
    pub fn ansb0(&mut self) -> ANSB0_W<0> {
        ANSB0_W::new(self)
    }
    #[doc = "Bit 1 - A/D Conversion Channels Select n"]
    #[inline(always)]
    #[must_use]
    pub fn ansb1(&mut self) -> ANSB1_W<1> {
        ANSB1_W::new(self)
    }
    #[doc = "Bit 2 - A/D Conversion Channels Select n"]
    #[inline(always)]
    #[must_use]
    pub fn ansb2(&mut self) -> ANSB2_W<2> {
        ANSB2_W::new(self)
    }
    #[doc = "Bit 3 - A/D Conversion Channels Select n"]
    #[inline(always)]
    #[must_use]
    pub fn ansb3(&mut self) -> ANSB3_W<3> {
        ANSB3_W::new(self)
    }
    #[doc = "Bit 4 - A/D Conversion Channels Select n"]
    #[inline(always)]
    #[must_use]
    pub fn ansb4(&mut self) -> ANSB4_W<4> {
        ANSB4_W::new(self)
    }
    #[doc = "Bit 5 - A/D Conversion Channels Select n"]
    #[inline(always)]
    #[must_use]
    pub fn ansb5(&mut self) -> ANSB5_W<5> {
        ANSB5_W::new(self)
    }
    #[doc = "Bit 6 - A/D Conversion Channels Select n"]
    #[inline(always)]
    #[must_use]
    pub fn ansb6(&mut self) -> ANSB6_W<6> {
        ANSB6_W::new(self)
    }
    #[doc = "Bit 7 - A/D Conversion Channels Select n"]
    #[inline(always)]
    #[must_use]
    pub fn ansb7(&mut self) -> ANSB7_W<7> {
        ANSB7_W::new(self)
    }
    #[doc = "Bit 8 - A/D Conversion Channels Select n"]
    #[inline(always)]
    #[must_use]
    pub fn ansb8(&mut self) -> ANSB8_W<8> {
        ANSB8_W::new(self)
    }
    #[doc = "Bit 12 - A/D Conversion Channels Select n"]
    #[inline(always)]
    #[must_use]
    pub fn ansb12(&mut self) -> ANSB12_W<12> {
        ANSB12_W::new(self)
    }
    #[doc = "Bit 13 - A/D Conversion Channels Select n"]
    #[inline(always)]
    #[must_use]
    pub fn ansb13(&mut self) -> ANSB13_W<13> {
        ANSB13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Channel Select Register B0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adansb0](index.html) module"]
pub struct ADANSB0_SPEC;
impl crate::RegisterSpec for ADANSB0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adansb0::R](R) reader structure"]
impl crate::Readable for ADANSB0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adansb0::W](W) writer structure"]
impl crate::Writable for ADANSB0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADANSB0 to value 0"]
impl crate::Resettable for ADANSB0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
