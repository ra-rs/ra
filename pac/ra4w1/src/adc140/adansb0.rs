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
#[doc = "Field `ANSB00` reader - AN000 Select"]
pub type ANSB00_R = crate::BitReader<ANSB00_A>;
#[doc = "AN000 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB00_A {
    #[doc = "0: AN000 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN000 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB00_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB00_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB00_A {
        match self.bits {
            false => ANSB00_A::_0,
            true => ANSB00_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB00_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB00_A::_1
    }
}
#[doc = "Field `ANSB00` writer - AN000 Select"]
pub type ANSB00_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB00_A, O>;
impl<'a, const O: u8> ANSB00_W<'a, O> {
    #[doc = "AN000 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB00_A::_0)
    }
    #[doc = "AN000 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB00_A::_1)
    }
}
#[doc = "Field `ANSB01` reader - AN001 Select"]
pub type ANSB01_R = crate::BitReader<ANSB01_A>;
#[doc = "AN001 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB01_A {
    #[doc = "0: AN001 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN001 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB01_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB01_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB01_A {
        match self.bits {
            false => ANSB01_A::_0,
            true => ANSB01_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB01_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB01_A::_1
    }
}
#[doc = "Field `ANSB01` writer - AN001 Select"]
pub type ANSB01_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB01_A, O>;
impl<'a, const O: u8> ANSB01_W<'a, O> {
    #[doc = "AN001 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB01_A::_0)
    }
    #[doc = "AN001 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB01_A::_1)
    }
}
#[doc = "Field `ANSB02` reader - AN002 Select"]
pub type ANSB02_R = crate::BitReader<ANSB02_A>;
#[doc = "AN002 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB02_A {
    #[doc = "0: AN002 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN002 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB02_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB02_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB02_A {
        match self.bits {
            false => ANSB02_A::_0,
            true => ANSB02_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB02_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB02_A::_1
    }
}
#[doc = "Field `ANSB02` writer - AN002 Select"]
pub type ANSB02_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB02_A, O>;
impl<'a, const O: u8> ANSB02_W<'a, O> {
    #[doc = "AN002 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB02_A::_0)
    }
    #[doc = "AN002 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB02_A::_1)
    }
}
#[doc = "Field `ANSB03` reader - AN003 Select"]
pub type ANSB03_R = crate::BitReader<ANSB03_A>;
#[doc = "AN003 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB03_A {
    #[doc = "0: AN003 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN003 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB03_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB03_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB03_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB03_A {
        match self.bits {
            false => ANSB03_A::_0,
            true => ANSB03_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB03_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB03_A::_1
    }
}
#[doc = "Field `ANSB03` writer - AN003 Select"]
pub type ANSB03_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB03_A, O>;
impl<'a, const O: u8> ANSB03_W<'a, O> {
    #[doc = "AN003 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB03_A::_0)
    }
    #[doc = "AN003 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB03_A::_1)
    }
}
#[doc = "Field `ANSB04` reader - AN004 Select"]
pub type ANSB04_R = crate::BitReader<ANSB04_A>;
#[doc = "AN004 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB04_A {
    #[doc = "0: AN004 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN004 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB04_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB04_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB04_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB04_A {
        match self.bits {
            false => ANSB04_A::_0,
            true => ANSB04_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB04_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB04_A::_1
    }
}
#[doc = "Field `ANSB04` writer - AN004 Select"]
pub type ANSB04_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB04_A, O>;
impl<'a, const O: u8> ANSB04_W<'a, O> {
    #[doc = "AN004 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB04_A::_0)
    }
    #[doc = "AN004 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB04_A::_1)
    }
}
#[doc = "Field `ANSB05` reader - AN005 Select"]
pub type ANSB05_R = crate::BitReader<ANSB05_A>;
#[doc = "AN005 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB05_A {
    #[doc = "0: AN005 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN005 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB05_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB05_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB05_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB05_A {
        match self.bits {
            false => ANSB05_A::_0,
            true => ANSB05_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB05_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB05_A::_1
    }
}
#[doc = "Field `ANSB05` writer - AN005 Select"]
pub type ANSB05_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB05_A, O>;
impl<'a, const O: u8> ANSB05_W<'a, O> {
    #[doc = "AN005 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB05_A::_0)
    }
    #[doc = "AN005 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB05_A::_1)
    }
}
#[doc = "Field `ANSB06` reader - AN006 Select"]
pub type ANSB06_R = crate::BitReader<ANSB06_A>;
#[doc = "AN006 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB06_A {
    #[doc = "0: AN006 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN006 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB06_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB06_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB06_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB06_A {
        match self.bits {
            false => ANSB06_A::_0,
            true => ANSB06_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB06_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB06_A::_1
    }
}
#[doc = "Field `ANSB06` writer - AN006 Select"]
pub type ANSB06_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB06_A, O>;
impl<'a, const O: u8> ANSB06_W<'a, O> {
    #[doc = "AN006 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB06_A::_0)
    }
    #[doc = "AN006 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB06_A::_1)
    }
}
#[doc = "Field `ANSB07` reader - AN007 Select"]
pub type ANSB07_R = crate::BitReader<ANSB07_A>;
#[doc = "AN007 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB07_A {
    #[doc = "0: AN007 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN007 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB07_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB07_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB07_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB07_A {
        match self.bits {
            false => ANSB07_A::_0,
            true => ANSB07_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB07_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB07_A::_1
    }
}
#[doc = "Field `ANSB07` writer - AN007 Select"]
pub type ANSB07_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB07_A, O>;
impl<'a, const O: u8> ANSB07_W<'a, O> {
    #[doc = "AN007 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB07_A::_0)
    }
    #[doc = "AN007 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB07_A::_1)
    }
}
#[doc = "Field `ANSB08` reader - AN008 Select"]
pub type ANSB08_R = crate::BitReader<ANSB08_A>;
#[doc = "AN008 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB08_A {
    #[doc = "0: AN008 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN008 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB08_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB08_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB08_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB08_A {
        match self.bits {
            false => ANSB08_A::_0,
            true => ANSB08_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB08_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB08_A::_1
    }
}
#[doc = "Field `ANSB08` writer - AN008 Select"]
pub type ANSB08_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB08_A, O>;
impl<'a, const O: u8> ANSB08_W<'a, O> {
    #[doc = "AN008 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB08_A::_0)
    }
    #[doc = "AN008 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB08_A::_1)
    }
}
#[doc = "Field `ANSB09` reader - AN009 Select"]
pub type ANSB09_R = crate::BitReader<ANSB09_A>;
#[doc = "AN009 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB09_A {
    #[doc = "0: AN009 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN009 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB09_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB09_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB09_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB09_A {
        match self.bits {
            false => ANSB09_A::_0,
            true => ANSB09_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB09_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB09_A::_1
    }
}
#[doc = "Field `ANSB09` writer - AN009 Select"]
pub type ANSB09_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB09_A, O>;
impl<'a, const O: u8> ANSB09_W<'a, O> {
    #[doc = "AN009 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB09_A::_0)
    }
    #[doc = "AN009 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB09_A::_1)
    }
}
#[doc = "Field `ANSB10` reader - AN010 Select"]
pub type ANSB10_R = crate::BitReader<ANSB10_A>;
#[doc = "AN010 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB10_A {
    #[doc = "0: AN010 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN010 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB10_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB10_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB10_A {
        match self.bits {
            false => ANSB10_A::_0,
            true => ANSB10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB10_A::_1
    }
}
#[doc = "Field `ANSB10` writer - AN010 Select"]
pub type ANSB10_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB10_A, O>;
impl<'a, const O: u8> ANSB10_W<'a, O> {
    #[doc = "AN010 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB10_A::_0)
    }
    #[doc = "AN010 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB10_A::_1)
    }
}
#[doc = "Field `ANSB11` reader - AN011 Select"]
pub type ANSB11_R = crate::BitReader<ANSB11_A>;
#[doc = "AN011 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB11_A {
    #[doc = "0: AN011 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN011 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB11_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB11_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB11_A {
        match self.bits {
            false => ANSB11_A::_0,
            true => ANSB11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB11_A::_1
    }
}
#[doc = "Field `ANSB11` writer - AN011 Select"]
pub type ANSB11_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB11_A, O>;
impl<'a, const O: u8> ANSB11_W<'a, O> {
    #[doc = "AN011 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB11_A::_0)
    }
    #[doc = "AN011 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB11_A::_1)
    }
}
#[doc = "Field `ANSB12` reader - AN012 Select"]
pub type ANSB12_R = crate::BitReader<ANSB12_A>;
#[doc = "AN012 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB12_A {
    #[doc = "0: AN012 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN012 is subjected to conversion."]
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
#[doc = "Field `ANSB12` writer - AN012 Select"]
pub type ANSB12_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB12_A, O>;
impl<'a, const O: u8> ANSB12_W<'a, O> {
    #[doc = "AN012 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB12_A::_0)
    }
    #[doc = "AN012 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB12_A::_1)
    }
}
#[doc = "Field `ANSB13` reader - AN013 Select"]
pub type ANSB13_R = crate::BitReader<ANSB13_A>;
#[doc = "AN013 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB13_A {
    #[doc = "0: AN013 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN013 is subjected to conversion."]
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
#[doc = "Field `ANSB13` writer - AN013 Select"]
pub type ANSB13_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB13_A, O>;
impl<'a, const O: u8> ANSB13_W<'a, O> {
    #[doc = "AN013 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB13_A::_0)
    }
    #[doc = "AN013 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB13_A::_1)
    }
}
#[doc = "Field `ANSB14` reader - AN014 Select"]
pub type ANSB14_R = crate::BitReader<ANSB14_A>;
#[doc = "AN014 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB14_A {
    #[doc = "0: AN014 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN014 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB14_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB14_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB14_A {
        match self.bits {
            false => ANSB14_A::_0,
            true => ANSB14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB14_A::_1
    }
}
#[doc = "Field `ANSB14` writer - AN014 Select"]
pub type ANSB14_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB14_A, O>;
impl<'a, const O: u8> ANSB14_W<'a, O> {
    #[doc = "AN014 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB14_A::_0)
    }
    #[doc = "AN014 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB14_A::_1)
    }
}
#[doc = "Field `ANSB15` reader - AN015 Select"]
pub type ANSB15_R = crate::BitReader<ANSB15_A>;
#[doc = "AN015 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB15_A {
    #[doc = "0: AN015 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN015 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB15_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB15_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB15_A {
        match self.bits {
            false => ANSB15_A::_0,
            true => ANSB15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB15_A::_1
    }
}
#[doc = "Field `ANSB15` writer - AN015 Select"]
pub type ANSB15_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB15_A, O>;
impl<'a, const O: u8> ANSB15_W<'a, O> {
    #[doc = "AN015 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB15_A::_0)
    }
    #[doc = "AN015 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB15_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AN000 Select"]
    #[inline(always)]
    pub fn ansb00(&self) -> ANSB00_R {
        ANSB00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AN001 Select"]
    #[inline(always)]
    pub fn ansb01(&self) -> ANSB01_R {
        ANSB01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AN002 Select"]
    #[inline(always)]
    pub fn ansb02(&self) -> ANSB02_R {
        ANSB02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AN003 Select"]
    #[inline(always)]
    pub fn ansb03(&self) -> ANSB03_R {
        ANSB03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AN004 Select"]
    #[inline(always)]
    pub fn ansb04(&self) -> ANSB04_R {
        ANSB04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AN005 Select"]
    #[inline(always)]
    pub fn ansb05(&self) -> ANSB05_R {
        ANSB05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AN006 Select"]
    #[inline(always)]
    pub fn ansb06(&self) -> ANSB06_R {
        ANSB06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AN007 Select"]
    #[inline(always)]
    pub fn ansb07(&self) -> ANSB07_R {
        ANSB07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AN008 Select"]
    #[inline(always)]
    pub fn ansb08(&self) -> ANSB08_R {
        ANSB08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AN009 Select"]
    #[inline(always)]
    pub fn ansb09(&self) -> ANSB09_R {
        ANSB09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AN010 Select"]
    #[inline(always)]
    pub fn ansb10(&self) -> ANSB10_R {
        ANSB10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - AN011 Select"]
    #[inline(always)]
    pub fn ansb11(&self) -> ANSB11_R {
        ANSB11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AN012 Select"]
    #[inline(always)]
    pub fn ansb12(&self) -> ANSB12_R {
        ANSB12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AN013 Select"]
    #[inline(always)]
    pub fn ansb13(&self) -> ANSB13_R {
        ANSB13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AN014 Select"]
    #[inline(always)]
    pub fn ansb14(&self) -> ANSB14_R {
        ANSB14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AN015 Select"]
    #[inline(always)]
    pub fn ansb15(&self) -> ANSB15_R {
        ANSB15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AN000 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb00(&mut self) -> ANSB00_W<0> {
        ANSB00_W::new(self)
    }
    #[doc = "Bit 1 - AN001 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb01(&mut self) -> ANSB01_W<1> {
        ANSB01_W::new(self)
    }
    #[doc = "Bit 2 - AN002 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb02(&mut self) -> ANSB02_W<2> {
        ANSB02_W::new(self)
    }
    #[doc = "Bit 3 - AN003 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb03(&mut self) -> ANSB03_W<3> {
        ANSB03_W::new(self)
    }
    #[doc = "Bit 4 - AN004 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb04(&mut self) -> ANSB04_W<4> {
        ANSB04_W::new(self)
    }
    #[doc = "Bit 5 - AN005 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb05(&mut self) -> ANSB05_W<5> {
        ANSB05_W::new(self)
    }
    #[doc = "Bit 6 - AN006 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb06(&mut self) -> ANSB06_W<6> {
        ANSB06_W::new(self)
    }
    #[doc = "Bit 7 - AN007 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb07(&mut self) -> ANSB07_W<7> {
        ANSB07_W::new(self)
    }
    #[doc = "Bit 8 - AN008 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb08(&mut self) -> ANSB08_W<8> {
        ANSB08_W::new(self)
    }
    #[doc = "Bit 9 - AN009 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb09(&mut self) -> ANSB09_W<9> {
        ANSB09_W::new(self)
    }
    #[doc = "Bit 10 - AN010 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb10(&mut self) -> ANSB10_W<10> {
        ANSB10_W::new(self)
    }
    #[doc = "Bit 11 - AN011 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb11(&mut self) -> ANSB11_W<11> {
        ANSB11_W::new(self)
    }
    #[doc = "Bit 12 - AN012 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb12(&mut self) -> ANSB12_W<12> {
        ANSB12_W::new(self)
    }
    #[doc = "Bit 13 - AN013 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb13(&mut self) -> ANSB13_W<13> {
        ANSB13_W::new(self)
    }
    #[doc = "Bit 14 - AN014 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb14(&mut self) -> ANSB14_W<14> {
        ANSB14_W::new(self)
    }
    #[doc = "Bit 15 - AN015 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb15(&mut self) -> ANSB15_W<15> {
        ANSB15_W::new(self)
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
