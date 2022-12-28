#[doc = "Register `ADANSA0` reader"]
pub struct R(crate::R<ADANSA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADANSA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADANSA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADANSA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADANSA0` writer"]
pub struct W(crate::W<ADANSA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADANSA0_SPEC>;
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
impl From<crate::W<ADANSA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADANSA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANSA00` reader - A/D Conversion Channels Select"]
pub type ANSA00_R = crate::BitReader<ANSA00_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA00_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSA00_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA00_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA00_A {
        match self.bits {
            false => ANSA00_A::_0,
            true => ANSA00_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA00_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA00_A::_1
    }
}
#[doc = "Field `ANSA00` writer - A/D Conversion Channels Select"]
pub type ANSA00_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA0_SPEC, ANSA00_A, O>;
impl<'a, const O: u8> ANSA00_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA00_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA00_A::_1)
    }
}
#[doc = "Field `ANSA01` reader - A/D Conversion Channels Select"]
pub type ANSA01_R = crate::BitReader<ANSA01_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA01_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSA01_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA01_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA01_A {
        match self.bits {
            false => ANSA01_A::_0,
            true => ANSA01_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA01_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA01_A::_1
    }
}
#[doc = "Field `ANSA01` writer - A/D Conversion Channels Select"]
pub type ANSA01_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA0_SPEC, ANSA01_A, O>;
impl<'a, const O: u8> ANSA01_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA01_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA01_A::_1)
    }
}
#[doc = "Field `ANSA02` reader - A/D Conversion Channels Select"]
pub type ANSA02_R = crate::BitReader<ANSA02_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA02_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSA02_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA02_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA02_A {
        match self.bits {
            false => ANSA02_A::_0,
            true => ANSA02_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA02_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA02_A::_1
    }
}
#[doc = "Field `ANSA02` writer - A/D Conversion Channels Select"]
pub type ANSA02_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA0_SPEC, ANSA02_A, O>;
impl<'a, const O: u8> ANSA02_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA02_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA02_A::_1)
    }
}
#[doc = "Field `ANSA03` reader - A/D Conversion Channels Select"]
pub type ANSA03_R = crate::BitReader<ANSA03_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA03_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSA03_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA03_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA03_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA03_A {
        match self.bits {
            false => ANSA03_A::_0,
            true => ANSA03_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA03_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA03_A::_1
    }
}
#[doc = "Field `ANSA03` writer - A/D Conversion Channels Select"]
pub type ANSA03_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA0_SPEC, ANSA03_A, O>;
impl<'a, const O: u8> ANSA03_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA03_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA03_A::_1)
    }
}
#[doc = "Field `ANSA04` reader - A/D Conversion Channels Select"]
pub type ANSA04_R = crate::BitReader<ANSA04_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA04_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSA04_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA04_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA04_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA04_A {
        match self.bits {
            false => ANSA04_A::_0,
            true => ANSA04_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA04_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA04_A::_1
    }
}
#[doc = "Field `ANSA04` writer - A/D Conversion Channels Select"]
pub type ANSA04_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA0_SPEC, ANSA04_A, O>;
impl<'a, const O: u8> ANSA04_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA04_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA04_A::_1)
    }
}
#[doc = "Field `ANSA05` reader - A/D Conversion Channels Select"]
pub type ANSA05_R = crate::BitReader<ANSA05_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA05_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSA05_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA05_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA05_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA05_A {
        match self.bits {
            false => ANSA05_A::_0,
            true => ANSA05_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA05_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA05_A::_1
    }
}
#[doc = "Field `ANSA05` writer - A/D Conversion Channels Select"]
pub type ANSA05_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA0_SPEC, ANSA05_A, O>;
impl<'a, const O: u8> ANSA05_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA05_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA05_A::_1)
    }
}
#[doc = "Field `ANSA06` reader - A/D Conversion Channels Select"]
pub type ANSA06_R = crate::BitReader<ANSA06_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA06_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSA06_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA06_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA06_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA06_A {
        match self.bits {
            false => ANSA06_A::_0,
            true => ANSA06_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA06_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA06_A::_1
    }
}
#[doc = "Field `ANSA06` writer - A/D Conversion Channels Select"]
pub type ANSA06_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA0_SPEC, ANSA06_A, O>;
impl<'a, const O: u8> ANSA06_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA06_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA06_A::_1)
    }
}
#[doc = "Field `ANSA07` reader - A/D Conversion Channels Select"]
pub type ANSA07_R = crate::BitReader<ANSA07_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA07_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSA07_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA07_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA07_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA07_A {
        match self.bits {
            false => ANSA07_A::_0,
            true => ANSA07_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA07_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA07_A::_1
    }
}
#[doc = "Field `ANSA07` writer - A/D Conversion Channels Select"]
pub type ANSA07_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA0_SPEC, ANSA07_A, O>;
impl<'a, const O: u8> ANSA07_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA07_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA07_A::_1)
    }
}
#[doc = "Field `ANSA08` reader - A/D Conversion Channels Select"]
pub type ANSA08_R = crate::BitReader<ANSA08_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA08_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSA08_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA08_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA08_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA08_A {
        match self.bits {
            false => ANSA08_A::_0,
            true => ANSA08_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA08_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA08_A::_1
    }
}
#[doc = "Field `ANSA08` writer - A/D Conversion Channels Select"]
pub type ANSA08_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA0_SPEC, ANSA08_A, O>;
impl<'a, const O: u8> ANSA08_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA08_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA08_A::_1)
    }
}
#[doc = "Field `ANSA09` reader - A/D Conversion Channels Select"]
pub type ANSA09_R = crate::BitReader<ANSA09_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA09_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSA09_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA09_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA09_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA09_A {
        match self.bits {
            false => ANSA09_A::_0,
            true => ANSA09_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA09_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA09_A::_1
    }
}
#[doc = "Field `ANSA09` writer - A/D Conversion Channels Select"]
pub type ANSA09_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA0_SPEC, ANSA09_A, O>;
impl<'a, const O: u8> ANSA09_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA09_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA09_A::_1)
    }
}
#[doc = "Field `ANSA10` reader - A/D Conversion Channels Select"]
pub type ANSA10_R = crate::BitReader<ANSA10_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA10_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSA10_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA10_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA10_A {
        match self.bits {
            false => ANSA10_A::_0,
            true => ANSA10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA10_A::_1
    }
}
#[doc = "Field `ANSA10` writer - A/D Conversion Channels Select"]
pub type ANSA10_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA0_SPEC, ANSA10_A, O>;
impl<'a, const O: u8> ANSA10_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA10_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA10_A::_1)
    }
}
#[doc = "Field `ANSA11` reader - A/D Conversion Channels Select"]
pub type ANSA11_R = crate::BitReader<ANSA11_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA11_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSA11_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA11_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA11_A {
        match self.bits {
            false => ANSA11_A::_0,
            true => ANSA11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA11_A::_1
    }
}
#[doc = "Field `ANSA11` writer - A/D Conversion Channels Select"]
pub type ANSA11_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA0_SPEC, ANSA11_A, O>;
impl<'a, const O: u8> ANSA11_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA11_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA11_A::_1)
    }
}
#[doc = "Field `ANSA12` reader - A/D Conversion Channels Select"]
pub type ANSA12_R = crate::BitReader<ANSA12_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA12_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSA12_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA12_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA12_A {
        match self.bits {
            false => ANSA12_A::_0,
            true => ANSA12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA12_A::_1
    }
}
#[doc = "Field `ANSA12` writer - A/D Conversion Channels Select"]
pub type ANSA12_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA0_SPEC, ANSA12_A, O>;
impl<'a, const O: u8> ANSA12_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA12_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA12_A::_1)
    }
}
#[doc = "Field `ANSA13` reader - A/D Conversion Channels Select"]
pub type ANSA13_R = crate::BitReader<ANSA13_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA13_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSA13_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA13_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA13_A {
        match self.bits {
            false => ANSA13_A::_0,
            true => ANSA13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA13_A::_1
    }
}
#[doc = "Field `ANSA13` writer - A/D Conversion Channels Select"]
pub type ANSA13_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA0_SPEC, ANSA13_A, O>;
impl<'a, const O: u8> ANSA13_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA13_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA13_A::_1)
    }
}
#[doc = "Field `ANSA14` reader - A/D Conversion Channels Select"]
pub type ANSA14_R = crate::BitReader<ANSA14_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA14_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSA14_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA14_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA14_A {
        match self.bits {
            false => ANSA14_A::_0,
            true => ANSA14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA14_A::_1
    }
}
#[doc = "Field `ANSA14` writer - A/D Conversion Channels Select"]
pub type ANSA14_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA0_SPEC, ANSA14_A, O>;
impl<'a, const O: u8> ANSA14_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA14_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA14_A::_1)
    }
}
#[doc = "Field `ANSA15` reader - A/D Conversion Channels Select"]
pub type ANSA15_R = crate::BitReader<ANSA15_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA15_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSA15_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA15_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSA15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSA15_A {
        match self.bits {
            false => ANSA15_A::_0,
            true => ANSA15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA15_A::_1
    }
}
#[doc = "Field `ANSA15` writer - A/D Conversion Channels Select"]
pub type ANSA15_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSA0_SPEC, ANSA15_A, O>;
impl<'a, const O: u8> ANSA15_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSA15_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSA15_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa00(&self) -> ANSA00_R {
        ANSA00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa01(&self) -> ANSA01_R {
        ANSA01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa02(&self) -> ANSA02_R {
        ANSA02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa03(&self) -> ANSA03_R {
        ANSA03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa04(&self) -> ANSA04_R {
        ANSA04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa05(&self) -> ANSA05_R {
        ANSA05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa06(&self) -> ANSA06_R {
        ANSA06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa07(&self) -> ANSA07_R {
        ANSA07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa08(&self) -> ANSA08_R {
        ANSA08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa09(&self) -> ANSA09_R {
        ANSA09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa10(&self) -> ANSA10_R {
        ANSA10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa11(&self) -> ANSA11_R {
        ANSA11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa12(&self) -> ANSA12_R {
        ANSA12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa13(&self) -> ANSA13_R {
        ANSA13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa14(&self) -> ANSA14_R {
        ANSA14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa15(&self) -> ANSA15_R {
        ANSA15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa00(&mut self) -> ANSA00_W<0> {
        ANSA00_W::new(self)
    }
    #[doc = "Bit 1 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa01(&mut self) -> ANSA01_W<1> {
        ANSA01_W::new(self)
    }
    #[doc = "Bit 2 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa02(&mut self) -> ANSA02_W<2> {
        ANSA02_W::new(self)
    }
    #[doc = "Bit 3 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa03(&mut self) -> ANSA03_W<3> {
        ANSA03_W::new(self)
    }
    #[doc = "Bit 4 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa04(&mut self) -> ANSA04_W<4> {
        ANSA04_W::new(self)
    }
    #[doc = "Bit 5 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa05(&mut self) -> ANSA05_W<5> {
        ANSA05_W::new(self)
    }
    #[doc = "Bit 6 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa06(&mut self) -> ANSA06_W<6> {
        ANSA06_W::new(self)
    }
    #[doc = "Bit 7 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa07(&mut self) -> ANSA07_W<7> {
        ANSA07_W::new(self)
    }
    #[doc = "Bit 8 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa08(&mut self) -> ANSA08_W<8> {
        ANSA08_W::new(self)
    }
    #[doc = "Bit 9 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa09(&mut self) -> ANSA09_W<9> {
        ANSA09_W::new(self)
    }
    #[doc = "Bit 10 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa10(&mut self) -> ANSA10_W<10> {
        ANSA10_W::new(self)
    }
    #[doc = "Bit 11 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa11(&mut self) -> ANSA11_W<11> {
        ANSA11_W::new(self)
    }
    #[doc = "Bit 12 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa12(&mut self) -> ANSA12_W<12> {
        ANSA12_W::new(self)
    }
    #[doc = "Bit 13 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa13(&mut self) -> ANSA13_W<13> {
        ANSA13_W::new(self)
    }
    #[doc = "Bit 14 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa14(&mut self) -> ANSA14_W<14> {
        ANSA14_W::new(self)
    }
    #[doc = "Bit 15 - A/D Conversion Channels Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansa15(&mut self) -> ANSA15_W<15> {
        ANSA15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Channel Select Register A0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adansa0](index.html) module"]
pub struct ADANSA0_SPEC;
impl crate::RegisterSpec for ADANSA0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adansa0::R](R) reader structure"]
impl crate::Readable for ADANSA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adansa0::W](W) writer structure"]
impl crate::Writable for ADANSA0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADANSA0 to value 0"]
impl crate::Resettable for ADANSA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
