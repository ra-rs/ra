#[doc = "Register `ADCMPANSR0` reader"]
pub struct R(crate::R<ADCMPANSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPANSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPANSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPANSR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPANSR0` writer"]
pub struct W(crate::W<ADCMPANSR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPANSR0_SPEC>;
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
impl From<crate::W<ADCMPANSR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPANSR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPCHA00` reader - AN000 Select"]
pub type CMPCHA00_R = crate::BitReader<CMPCHA00_A>;
#[doc = "AN000 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA00_A {
    #[doc = "0: Excludes AN000 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN000 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA00_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA00_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA00_A {
        match self.bits {
            false => CMPCHA00_A::_0,
            true => CMPCHA00_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA00_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA00_A::_1
    }
}
#[doc = "Field `CMPCHA00` writer - AN000 Select"]
pub type CMPCHA00_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA00_A, O>;
impl<'a, const O: u8> CMPCHA00_W<'a, O> {
    #[doc = "Excludes AN000 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA00_A::_0)
    }
    #[doc = "Includes AN000 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA00_A::_1)
    }
}
#[doc = "Field `CMPCHA01` reader - AN001 Select"]
pub type CMPCHA01_R = crate::BitReader<CMPCHA01_A>;
#[doc = "AN001 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA01_A {
    #[doc = "0: Excludes AN001 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN001 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA01_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA01_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA01_A {
        match self.bits {
            false => CMPCHA01_A::_0,
            true => CMPCHA01_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA01_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA01_A::_1
    }
}
#[doc = "Field `CMPCHA01` writer - AN001 Select"]
pub type CMPCHA01_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA01_A, O>;
impl<'a, const O: u8> CMPCHA01_W<'a, O> {
    #[doc = "Excludes AN001 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA01_A::_0)
    }
    #[doc = "Includes AN001 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA01_A::_1)
    }
}
#[doc = "Field `CMPCHA02` reader - AN002 Select"]
pub type CMPCHA02_R = crate::BitReader<CMPCHA02_A>;
#[doc = "AN002 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA02_A {
    #[doc = "0: Excludes AN002 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN002 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA02_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA02_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA02_A {
        match self.bits {
            false => CMPCHA02_A::_0,
            true => CMPCHA02_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA02_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA02_A::_1
    }
}
#[doc = "Field `CMPCHA02` writer - AN002 Select"]
pub type CMPCHA02_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA02_A, O>;
impl<'a, const O: u8> CMPCHA02_W<'a, O> {
    #[doc = "Excludes AN002 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA02_A::_0)
    }
    #[doc = "Includes AN002 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA02_A::_1)
    }
}
#[doc = "Field `CMPCHA03` reader - AN003 Select"]
pub type CMPCHA03_R = crate::BitReader<CMPCHA03_A>;
#[doc = "AN003 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA03_A {
    #[doc = "0: Excludes AN003 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN003 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA03_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA03_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA03_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA03_A {
        match self.bits {
            false => CMPCHA03_A::_0,
            true => CMPCHA03_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA03_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA03_A::_1
    }
}
#[doc = "Field `CMPCHA03` writer - AN003 Select"]
pub type CMPCHA03_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA03_A, O>;
impl<'a, const O: u8> CMPCHA03_W<'a, O> {
    #[doc = "Excludes AN003 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA03_A::_0)
    }
    #[doc = "Includes AN003 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA03_A::_1)
    }
}
#[doc = "Field `CMPCHA04` reader - AN004 Select"]
pub type CMPCHA04_R = crate::BitReader<CMPCHA04_A>;
#[doc = "AN004 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA04_A {
    #[doc = "0: Excludes AN004 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN004 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA04_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA04_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA04_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA04_A {
        match self.bits {
            false => CMPCHA04_A::_0,
            true => CMPCHA04_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA04_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA04_A::_1
    }
}
#[doc = "Field `CMPCHA04` writer - AN004 Select"]
pub type CMPCHA04_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA04_A, O>;
impl<'a, const O: u8> CMPCHA04_W<'a, O> {
    #[doc = "Excludes AN004 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA04_A::_0)
    }
    #[doc = "Includes AN004 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA04_A::_1)
    }
}
#[doc = "Field `CMPCHA05` reader - AN005 Select"]
pub type CMPCHA05_R = crate::BitReader<CMPCHA05_A>;
#[doc = "AN005 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA05_A {
    #[doc = "0: Excludes AN005 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN005 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA05_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA05_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA05_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA05_A {
        match self.bits {
            false => CMPCHA05_A::_0,
            true => CMPCHA05_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA05_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA05_A::_1
    }
}
#[doc = "Field `CMPCHA05` writer - AN005 Select"]
pub type CMPCHA05_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA05_A, O>;
impl<'a, const O: u8> CMPCHA05_W<'a, O> {
    #[doc = "Excludes AN005 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA05_A::_0)
    }
    #[doc = "Includes AN005 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA05_A::_1)
    }
}
#[doc = "Field `CMPCHA06` reader - AN006 Select"]
pub type CMPCHA06_R = crate::BitReader<CMPCHA06_A>;
#[doc = "AN006 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA06_A {
    #[doc = "0: Excludes AN006 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN006 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA06_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA06_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA06_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA06_A {
        match self.bits {
            false => CMPCHA06_A::_0,
            true => CMPCHA06_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA06_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA06_A::_1
    }
}
#[doc = "Field `CMPCHA06` writer - AN006 Select"]
pub type CMPCHA06_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA06_A, O>;
impl<'a, const O: u8> CMPCHA06_W<'a, O> {
    #[doc = "Excludes AN006 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA06_A::_0)
    }
    #[doc = "Includes AN006 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA06_A::_1)
    }
}
#[doc = "Field `CMPCHA07` reader - AN007 Select"]
pub type CMPCHA07_R = crate::BitReader<CMPCHA07_A>;
#[doc = "AN007 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA07_A {
    #[doc = "0: Excludes AN007 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN007 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA07_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA07_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA07_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA07_A {
        match self.bits {
            false => CMPCHA07_A::_0,
            true => CMPCHA07_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA07_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA07_A::_1
    }
}
#[doc = "Field `CMPCHA07` writer - AN007 Select"]
pub type CMPCHA07_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA07_A, O>;
impl<'a, const O: u8> CMPCHA07_W<'a, O> {
    #[doc = "Excludes AN007 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA07_A::_0)
    }
    #[doc = "Includes AN007 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA07_A::_1)
    }
}
#[doc = "Field `CMPCHA08` reader - AN008 Select"]
pub type CMPCHA08_R = crate::BitReader<CMPCHA08_A>;
#[doc = "AN008 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA08_A {
    #[doc = "0: Excludes AN008 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN008 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA08_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA08_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA08_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA08_A {
        match self.bits {
            false => CMPCHA08_A::_0,
            true => CMPCHA08_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA08_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA08_A::_1
    }
}
#[doc = "Field `CMPCHA08` writer - AN008 Select"]
pub type CMPCHA08_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA08_A, O>;
impl<'a, const O: u8> CMPCHA08_W<'a, O> {
    #[doc = "Excludes AN008 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA08_A::_0)
    }
    #[doc = "Includes AN008 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA08_A::_1)
    }
}
#[doc = "Field `CMPCHA09` reader - AN009 Select"]
pub type CMPCHA09_R = crate::BitReader<CMPCHA09_A>;
#[doc = "AN009 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA09_A {
    #[doc = "0: Excludes AN009 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN009 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA09_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA09_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA09_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA09_A {
        match self.bits {
            false => CMPCHA09_A::_0,
            true => CMPCHA09_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA09_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA09_A::_1
    }
}
#[doc = "Field `CMPCHA09` writer - AN009 Select"]
pub type CMPCHA09_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA09_A, O>;
impl<'a, const O: u8> CMPCHA09_W<'a, O> {
    #[doc = "Excludes AN009 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA09_A::_0)
    }
    #[doc = "Includes AN009 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA09_A::_1)
    }
}
#[doc = "Field `CMPCHA10` reader - AN010 Select"]
pub type CMPCHA10_R = crate::BitReader<CMPCHA10_A>;
#[doc = "AN010 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA10_A {
    #[doc = "0: Excludes AN010 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN010 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA10_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA10_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA10_A {
        match self.bits {
            false => CMPCHA10_A::_0,
            true => CMPCHA10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA10_A::_1
    }
}
#[doc = "Field `CMPCHA10` writer - AN010 Select"]
pub type CMPCHA10_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA10_A, O>;
impl<'a, const O: u8> CMPCHA10_W<'a, O> {
    #[doc = "Excludes AN010 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA10_A::_0)
    }
    #[doc = "Includes AN010 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA10_A::_1)
    }
}
#[doc = "Field `CMPCHA11` reader - AN011 Select"]
pub type CMPCHA11_R = crate::BitReader<CMPCHA11_A>;
#[doc = "AN011 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA11_A {
    #[doc = "0: Excludes AN011 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN011 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA11_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA11_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA11_A {
        match self.bits {
            false => CMPCHA11_A::_0,
            true => CMPCHA11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA11_A::_1
    }
}
#[doc = "Field `CMPCHA11` writer - AN011 Select"]
pub type CMPCHA11_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA11_A, O>;
impl<'a, const O: u8> CMPCHA11_W<'a, O> {
    #[doc = "Excludes AN011 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA11_A::_0)
    }
    #[doc = "Includes AN011 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA11_A::_1)
    }
}
#[doc = "Field `CMPCHA12` reader - AN012 Select"]
pub type CMPCHA12_R = crate::BitReader<CMPCHA12_A>;
#[doc = "AN012 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA12_A {
    #[doc = "0: Excludes AN012 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN012 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA12_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA12_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA12_A {
        match self.bits {
            false => CMPCHA12_A::_0,
            true => CMPCHA12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA12_A::_1
    }
}
#[doc = "Field `CMPCHA12` writer - AN012 Select"]
pub type CMPCHA12_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA12_A, O>;
impl<'a, const O: u8> CMPCHA12_W<'a, O> {
    #[doc = "Excludes AN012 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA12_A::_0)
    }
    #[doc = "Includes AN012 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA12_A::_1)
    }
}
#[doc = "Field `CMPCHA13` reader - AN013 Select"]
pub type CMPCHA13_R = crate::BitReader<CMPCHA13_A>;
#[doc = "AN013 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA13_A {
    #[doc = "0: Excludes AN013 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN013 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA13_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA13_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA13_A {
        match self.bits {
            false => CMPCHA13_A::_0,
            true => CMPCHA13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA13_A::_1
    }
}
#[doc = "Field `CMPCHA13` writer - AN013 Select"]
pub type CMPCHA13_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA13_A, O>;
impl<'a, const O: u8> CMPCHA13_W<'a, O> {
    #[doc = "Excludes AN013 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA13_A::_0)
    }
    #[doc = "Includes AN013 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA13_A::_1)
    }
}
#[doc = "Field `CMPCHA14` reader - AN014 Select"]
pub type CMPCHA14_R = crate::BitReader<CMPCHA14_A>;
#[doc = "AN014 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA14_A {
    #[doc = "0: Excludes AN014 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN014 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA14_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA14_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA14_A {
        match self.bits {
            false => CMPCHA14_A::_0,
            true => CMPCHA14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA14_A::_1
    }
}
#[doc = "Field `CMPCHA14` writer - AN014 Select"]
pub type CMPCHA14_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA14_A, O>;
impl<'a, const O: u8> CMPCHA14_W<'a, O> {
    #[doc = "Excludes AN014 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA14_A::_0)
    }
    #[doc = "Includes AN014 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA14_A::_1)
    }
}
#[doc = "Field `CMPCHA15` reader - AN015 Select"]
pub type CMPCHA15_R = crate::BitReader<CMPCHA15_A>;
#[doc = "AN015 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA15_A {
    #[doc = "0: Excludes AN015 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN015 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA15_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA15_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA15_A {
        match self.bits {
            false => CMPCHA15_A::_0,
            true => CMPCHA15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA15_A::_1
    }
}
#[doc = "Field `CMPCHA15` writer - AN015 Select"]
pub type CMPCHA15_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA15_A, O>;
impl<'a, const O: u8> CMPCHA15_W<'a, O> {
    #[doc = "Excludes AN015 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA15_A::_0)
    }
    #[doc = "Includes AN015 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA15_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AN000 Select"]
    #[inline(always)]
    pub fn cmpcha00(&self) -> CMPCHA00_R {
        CMPCHA00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AN001 Select"]
    #[inline(always)]
    pub fn cmpcha01(&self) -> CMPCHA01_R {
        CMPCHA01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AN002 Select"]
    #[inline(always)]
    pub fn cmpcha02(&self) -> CMPCHA02_R {
        CMPCHA02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AN003 Select"]
    #[inline(always)]
    pub fn cmpcha03(&self) -> CMPCHA03_R {
        CMPCHA03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AN004 Select"]
    #[inline(always)]
    pub fn cmpcha04(&self) -> CMPCHA04_R {
        CMPCHA04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AN005 Select"]
    #[inline(always)]
    pub fn cmpcha05(&self) -> CMPCHA05_R {
        CMPCHA05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AN006 Select"]
    #[inline(always)]
    pub fn cmpcha06(&self) -> CMPCHA06_R {
        CMPCHA06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AN007 Select"]
    #[inline(always)]
    pub fn cmpcha07(&self) -> CMPCHA07_R {
        CMPCHA07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AN008 Select"]
    #[inline(always)]
    pub fn cmpcha08(&self) -> CMPCHA08_R {
        CMPCHA08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AN009 Select"]
    #[inline(always)]
    pub fn cmpcha09(&self) -> CMPCHA09_R {
        CMPCHA09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AN010 Select"]
    #[inline(always)]
    pub fn cmpcha10(&self) -> CMPCHA10_R {
        CMPCHA10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - AN011 Select"]
    #[inline(always)]
    pub fn cmpcha11(&self) -> CMPCHA11_R {
        CMPCHA11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AN012 Select"]
    #[inline(always)]
    pub fn cmpcha12(&self) -> CMPCHA12_R {
        CMPCHA12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AN013 Select"]
    #[inline(always)]
    pub fn cmpcha13(&self) -> CMPCHA13_R {
        CMPCHA13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AN014 Select"]
    #[inline(always)]
    pub fn cmpcha14(&self) -> CMPCHA14_R {
        CMPCHA14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AN015 Select"]
    #[inline(always)]
    pub fn cmpcha15(&self) -> CMPCHA15_R {
        CMPCHA15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AN000 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha00(&mut self) -> CMPCHA00_W<0> {
        CMPCHA00_W::new(self)
    }
    #[doc = "Bit 1 - AN001 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha01(&mut self) -> CMPCHA01_W<1> {
        CMPCHA01_W::new(self)
    }
    #[doc = "Bit 2 - AN002 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha02(&mut self) -> CMPCHA02_W<2> {
        CMPCHA02_W::new(self)
    }
    #[doc = "Bit 3 - AN003 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha03(&mut self) -> CMPCHA03_W<3> {
        CMPCHA03_W::new(self)
    }
    #[doc = "Bit 4 - AN004 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha04(&mut self) -> CMPCHA04_W<4> {
        CMPCHA04_W::new(self)
    }
    #[doc = "Bit 5 - AN005 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha05(&mut self) -> CMPCHA05_W<5> {
        CMPCHA05_W::new(self)
    }
    #[doc = "Bit 6 - AN006 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha06(&mut self) -> CMPCHA06_W<6> {
        CMPCHA06_W::new(self)
    }
    #[doc = "Bit 7 - AN007 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha07(&mut self) -> CMPCHA07_W<7> {
        CMPCHA07_W::new(self)
    }
    #[doc = "Bit 8 - AN008 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha08(&mut self) -> CMPCHA08_W<8> {
        CMPCHA08_W::new(self)
    }
    #[doc = "Bit 9 - AN009 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha09(&mut self) -> CMPCHA09_W<9> {
        CMPCHA09_W::new(self)
    }
    #[doc = "Bit 10 - AN010 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha10(&mut self) -> CMPCHA10_W<10> {
        CMPCHA10_W::new(self)
    }
    #[doc = "Bit 11 - AN011 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha11(&mut self) -> CMPCHA11_W<11> {
        CMPCHA11_W::new(self)
    }
    #[doc = "Bit 12 - AN012 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha12(&mut self) -> CMPCHA12_W<12> {
        CMPCHA12_W::new(self)
    }
    #[doc = "Bit 13 - AN013 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha13(&mut self) -> CMPCHA13_W<13> {
        CMPCHA13_W::new(self)
    }
    #[doc = "Bit 14 - AN014 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha14(&mut self) -> CMPCHA14_W<14> {
        CMPCHA14_W::new(self)
    }
    #[doc = "Bit 15 - AN015 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha15(&mut self) -> CMPCHA15_W<15> {
        CMPCHA15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Compare Function Window A Channel Select Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpansr0](index.html) module"]
pub struct ADCMPANSR0_SPEC;
impl crate::RegisterSpec for ADCMPANSR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcmpansr0::R](R) reader structure"]
impl crate::Readable for ADCMPANSR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmpansr0::W](W) writer structure"]
impl crate::Writable for ADCMPANSR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPANSR0 to value 0"]
impl crate::Resettable for ADCMPANSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
