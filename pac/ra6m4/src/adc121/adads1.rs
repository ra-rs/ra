#[doc = "Register `ADADS1` reader"]
pub struct R(crate::R<ADADS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADADS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADADS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADADS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADADS1` writer"]
pub struct W(crate::W<ADADS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADADS1_SPEC>;
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
impl From<crate::W<ADADS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADADS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADS16` reader - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS16_R = crate::BitReader<ADS16_A>;
#[doc = "A/D-Converted Value Addition/Average Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS16_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ADS16_A> for bool {
    #[inline(always)]
    fn from(variant: ADS16_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS16_A {
        match self.bits {
            false => ADS16_A::_0,
            true => ADS16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS16_A::_1
    }
}
#[doc = "Field `ADS16` writer - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS16_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS1_SPEC, ADS16_A, O>;
impl<'a, const O: u8> ADS16_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS16_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS16_A::_1)
    }
}
#[doc = "Field `ADS17` reader - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS17_R = crate::BitReader<ADS17_A>;
#[doc = "A/D-Converted Value Addition/Average Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS17_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ADS17_A> for bool {
    #[inline(always)]
    fn from(variant: ADS17_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS17_A {
        match self.bits {
            false => ADS17_A::_0,
            true => ADS17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS17_A::_1
    }
}
#[doc = "Field `ADS17` writer - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS17_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS1_SPEC, ADS17_A, O>;
impl<'a, const O: u8> ADS17_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS17_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS17_A::_1)
    }
}
#[doc = "Field `ADS18` reader - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS18_R = crate::BitReader<ADS18_A>;
#[doc = "A/D-Converted Value Addition/Average Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS18_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ADS18_A> for bool {
    #[inline(always)]
    fn from(variant: ADS18_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS18_A {
        match self.bits {
            false => ADS18_A::_0,
            true => ADS18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS18_A::_1
    }
}
#[doc = "Field `ADS18` writer - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS18_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS1_SPEC, ADS18_A, O>;
impl<'a, const O: u8> ADS18_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS18_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS18_A::_1)
    }
}
#[doc = "Field `ADS19` reader - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS19_R = crate::BitReader<ADS19_A>;
#[doc = "A/D-Converted Value Addition/Average Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS19_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ADS19_A> for bool {
    #[inline(always)]
    fn from(variant: ADS19_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS19_A {
        match self.bits {
            false => ADS19_A::_0,
            true => ADS19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS19_A::_1
    }
}
#[doc = "Field `ADS19` writer - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS19_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS1_SPEC, ADS19_A, O>;
impl<'a, const O: u8> ADS19_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS19_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS19_A::_1)
    }
}
#[doc = "Field `ADS20` reader - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS20_R = crate::BitReader<ADS20_A>;
#[doc = "A/D-Converted Value Addition/Average Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS20_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ADS20_A> for bool {
    #[inline(always)]
    fn from(variant: ADS20_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS20_A {
        match self.bits {
            false => ADS20_A::_0,
            true => ADS20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS20_A::_1
    }
}
#[doc = "Field `ADS20` writer - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS20_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS1_SPEC, ADS20_A, O>;
impl<'a, const O: u8> ADS20_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS20_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS20_A::_1)
    }
}
#[doc = "Field `ADS21` reader - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS21_R = crate::BitReader<ADS21_A>;
#[doc = "A/D-Converted Value Addition/Average Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS21_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ADS21_A> for bool {
    #[inline(always)]
    fn from(variant: ADS21_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS21_A {
        match self.bits {
            false => ADS21_A::_0,
            true => ADS21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS21_A::_1
    }
}
#[doc = "Field `ADS21` writer - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS21_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS1_SPEC, ADS21_A, O>;
impl<'a, const O: u8> ADS21_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS21_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS21_A::_1)
    }
}
#[doc = "Field `ADS22` reader - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS22_R = crate::BitReader<ADS22_A>;
#[doc = "A/D-Converted Value Addition/Average Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS22_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ADS22_A> for bool {
    #[inline(always)]
    fn from(variant: ADS22_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS22_A {
        match self.bits {
            false => ADS22_A::_0,
            true => ADS22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS22_A::_1
    }
}
#[doc = "Field `ADS22` writer - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS22_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS1_SPEC, ADS22_A, O>;
impl<'a, const O: u8> ADS22_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS22_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS22_A::_1)
    }
}
#[doc = "Field `ADS23` reader - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS23_R = crate::BitReader<ADS23_A>;
#[doc = "A/D-Converted Value Addition/Average Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS23_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ADS23_A> for bool {
    #[inline(always)]
    fn from(variant: ADS23_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS23_A {
        match self.bits {
            false => ADS23_A::_0,
            true => ADS23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS23_A::_1
    }
}
#[doc = "Field `ADS23` writer - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS23_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS1_SPEC, ADS23_A, O>;
impl<'a, const O: u8> ADS23_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS23_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS23_A::_1)
    }
}
#[doc = "Field `ADS24` reader - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS24_R = crate::BitReader<ADS24_A>;
#[doc = "A/D-Converted Value Addition/Average Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS24_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ADS24_A> for bool {
    #[inline(always)]
    fn from(variant: ADS24_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS24_A {
        match self.bits {
            false => ADS24_A::_0,
            true => ADS24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS24_A::_1
    }
}
#[doc = "Field `ADS24` writer - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS24_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS1_SPEC, ADS24_A, O>;
impl<'a, const O: u8> ADS24_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS24_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS24_A::_1)
    }
}
#[doc = "Field `ADS25` reader - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS25_R = crate::BitReader<ADS25_A>;
#[doc = "A/D-Converted Value Addition/Average Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS25_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ADS25_A> for bool {
    #[inline(always)]
    fn from(variant: ADS25_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS25_A {
        match self.bits {
            false => ADS25_A::_0,
            true => ADS25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS25_A::_1
    }
}
#[doc = "Field `ADS25` writer - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS25_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS1_SPEC, ADS25_A, O>;
impl<'a, const O: u8> ADS25_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS25_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS25_A::_1)
    }
}
#[doc = "Field `ADS26` reader - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS26_R = crate::BitReader<ADS26_A>;
#[doc = "A/D-Converted Value Addition/Average Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS26_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ADS26_A> for bool {
    #[inline(always)]
    fn from(variant: ADS26_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS26_A {
        match self.bits {
            false => ADS26_A::_0,
            true => ADS26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS26_A::_1
    }
}
#[doc = "Field `ADS26` writer - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS26_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS1_SPEC, ADS26_A, O>;
impl<'a, const O: u8> ADS26_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS26_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS26_A::_1)
    }
}
#[doc = "Field `ADS27` reader - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS27_R = crate::BitReader<ADS27_A>;
#[doc = "A/D-Converted Value Addition/Average Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS27_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ADS27_A> for bool {
    #[inline(always)]
    fn from(variant: ADS27_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS27_A {
        match self.bits {
            false => ADS27_A::_0,
            true => ADS27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS27_A::_1
    }
}
#[doc = "Field `ADS27` writer - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS27_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS1_SPEC, ADS27_A, O>;
impl<'a, const O: u8> ADS27_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS27_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS27_A::_1)
    }
}
#[doc = "Field `ADS28` reader - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS28_R = crate::BitReader<ADS28_A>;
#[doc = "A/D-Converted Value Addition/Average Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS28_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ADS28_A> for bool {
    #[inline(always)]
    fn from(variant: ADS28_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS28_A {
        match self.bits {
            false => ADS28_A::_0,
            true => ADS28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS28_A::_1
    }
}
#[doc = "Field `ADS28` writer - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS28_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS1_SPEC, ADS28_A, O>;
impl<'a, const O: u8> ADS28_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS28_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS28_A::_1)
    }
}
#[doc = "Field `ADS29` reader - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS29_R = crate::BitReader<ADS29_A>;
#[doc = "A/D-Converted Value Addition/Average Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS29_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ADS29_A> for bool {
    #[inline(always)]
    fn from(variant: ADS29_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS29_A {
        match self.bits {
            false => ADS29_A::_0,
            true => ADS29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS29_A::_1
    }
}
#[doc = "Field `ADS29` writer - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS29_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS1_SPEC, ADS29_A, O>;
impl<'a, const O: u8> ADS29_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS29_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS29_A::_1)
    }
}
#[doc = "Field `ADS30` reader - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS30_R = crate::BitReader<ADS30_A>;
#[doc = "A/D-Converted Value Addition/Average Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS30_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ADS30_A> for bool {
    #[inline(always)]
    fn from(variant: ADS30_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS30_A {
        match self.bits {
            false => ADS30_A::_0,
            true => ADS30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS30_A::_1
    }
}
#[doc = "Field `ADS30` writer - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS30_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS1_SPEC, ADS30_A, O>;
impl<'a, const O: u8> ADS30_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS30_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS30_A::_1)
    }
}
#[doc = "Field `ADS31` reader - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS31_R = crate::BitReader<ADS31_A>;
#[doc = "A/D-Converted Value Addition/Average Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS31_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ADS31_A> for bool {
    #[inline(always)]
    fn from(variant: ADS31_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS31_A {
        match self.bits {
            false => ADS31_A::_0,
            true => ADS31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS31_A::_1
    }
}
#[doc = "Field `ADS31` writer - A/D-Converted Value Addition/Average Channel Select"]
pub type ADS31_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS1_SPEC, ADS31_A, O>;
impl<'a, const O: u8> ADS31_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS31_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS31_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads16(&self) -> ADS16_R {
        ADS16_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads17(&self) -> ADS17_R {
        ADS17_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads18(&self) -> ADS18_R {
        ADS18_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads19(&self) -> ADS19_R {
        ADS19_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads20(&self) -> ADS20_R {
        ADS20_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads21(&self) -> ADS21_R {
        ADS21_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads22(&self) -> ADS22_R {
        ADS22_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads23(&self) -> ADS23_R {
        ADS23_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads24(&self) -> ADS24_R {
        ADS24_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads25(&self) -> ADS25_R {
        ADS25_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads26(&self) -> ADS26_R {
        ADS26_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads27(&self) -> ADS27_R {
        ADS27_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads28(&self) -> ADS28_R {
        ADS28_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads29(&self) -> ADS29_R {
        ADS29_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads30(&self) -> ADS30_R {
        ADS30_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads31(&self) -> ADS31_R {
        ADS31_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads16(&mut self) -> ADS16_W<0> {
        ADS16_W::new(self)
    }
    #[doc = "Bit 1 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads17(&mut self) -> ADS17_W<1> {
        ADS17_W::new(self)
    }
    #[doc = "Bit 2 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads18(&mut self) -> ADS18_W<2> {
        ADS18_W::new(self)
    }
    #[doc = "Bit 3 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads19(&mut self) -> ADS19_W<3> {
        ADS19_W::new(self)
    }
    #[doc = "Bit 4 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads20(&mut self) -> ADS20_W<4> {
        ADS20_W::new(self)
    }
    #[doc = "Bit 5 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads21(&mut self) -> ADS21_W<5> {
        ADS21_W::new(self)
    }
    #[doc = "Bit 6 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads22(&mut self) -> ADS22_W<6> {
        ADS22_W::new(self)
    }
    #[doc = "Bit 7 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads23(&mut self) -> ADS23_W<7> {
        ADS23_W::new(self)
    }
    #[doc = "Bit 8 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads24(&mut self) -> ADS24_W<8> {
        ADS24_W::new(self)
    }
    #[doc = "Bit 9 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads25(&mut self) -> ADS25_W<9> {
        ADS25_W::new(self)
    }
    #[doc = "Bit 10 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads26(&mut self) -> ADS26_W<10> {
        ADS26_W::new(self)
    }
    #[doc = "Bit 11 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads27(&mut self) -> ADS27_W<11> {
        ADS27_W::new(self)
    }
    #[doc = "Bit 12 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads28(&mut self) -> ADS28_W<12> {
        ADS28_W::new(self)
    }
    #[doc = "Bit 13 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads29(&mut self) -> ADS29_W<13> {
        ADS29_W::new(self)
    }
    #[doc = "Bit 14 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads30(&mut self) -> ADS30_W<14> {
        ADS30_W::new(self)
    }
    #[doc = "Bit 15 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads31(&mut self) -> ADS31_W<15> {
        ADS31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D-Converted Value Addition/Average Channel Select Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adads1](index.html) module"]
pub struct ADADS1_SPEC;
impl crate::RegisterSpec for ADADS1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adads1::R](R) reader structure"]
impl crate::Readable for ADADS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adads1::W](W) writer structure"]
impl crate::Writable for ADADS1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADADS1 to value 0"]
impl crate::Resettable for ADADS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
