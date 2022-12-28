#[doc = "Register `MSTPCRB` reader"]
pub struct R(crate::R<MSTPCRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSTPCRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSTPCRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSTPCRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSTPCRB` writer"]
pub struct W(crate::W<MSTPCRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSTPCRB_SPEC>;
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
impl From<crate::W<MSTPCRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSTPCRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSTPB2` reader - Controller Area Network Module Stop"]
pub type MSTPB2_R = crate::BitReader<MSTPB2_A>;
#[doc = "Controller Area Network Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB2_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPB2_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB2_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPB2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPB2_A {
        match self.bits {
            false => MSTPB2_A::_0,
            true => MSTPB2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB2_A::_1
    }
}
#[doc = "Field `MSTPB2` writer - Controller Area Network Module Stop"]
pub type MSTPB2_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRB_SPEC, MSTPB2_A, O>;
impl<'a, const O: u8> MSTPB2_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPB2_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPB2_A::_1)
    }
}
#[doc = "Field `MSTPB6` reader - Queued Serial Peripheral Interface Module Stop"]
pub type MSTPB6_R = crate::BitReader<MSTPB6_A>;
#[doc = "Queued Serial Peripheral Interface Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB6_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPB6_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB6_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPB6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPB6_A {
        match self.bits {
            false => MSTPB6_A::_0,
            true => MSTPB6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB6_A::_1
    }
}
#[doc = "Field `MSTPB6` writer - Queued Serial Peripheral Interface Module Stop"]
pub type MSTPB6_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRB_SPEC, MSTPB6_A, O>;
impl<'a, const O: u8> MSTPB6_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPB6_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPB6_A::_1)
    }
}
#[doc = "Field `MSTPB7` reader - I2C Bus Interface 2 Module Stop"]
pub type MSTPB7_R = crate::BitReader<MSTPB7_A>;
#[doc = "I2C Bus Interface 2 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB7_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPB7_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB7_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPB7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPB7_A {
        match self.bits {
            false => MSTPB7_A::_0,
            true => MSTPB7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB7_A::_1
    }
}
#[doc = "Field `MSTPB7` writer - I2C Bus Interface 2 Module Stop"]
pub type MSTPB7_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRB_SPEC, MSTPB7_A, O>;
impl<'a, const O: u8> MSTPB7_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPB7_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPB7_A::_1)
    }
}
#[doc = "Field `MSTPB8` reader - I2C Bus Interface 1 Module Stop"]
pub type MSTPB8_R = crate::BitReader<MSTPB8_A>;
#[doc = "I2C Bus Interface 1 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB8_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPB8_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB8_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPB8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPB8_A {
        match self.bits {
            false => MSTPB8_A::_0,
            true => MSTPB8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB8_A::_1
    }
}
#[doc = "Field `MSTPB8` writer - I2C Bus Interface 1 Module Stop"]
pub type MSTPB8_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRB_SPEC, MSTPB8_A, O>;
impl<'a, const O: u8> MSTPB8_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPB8_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPB8_A::_1)
    }
}
#[doc = "Field `MSTPB9` reader - I2C Bus Interface 0 Module Stop"]
pub type MSTPB9_R = crate::BitReader<MSTPB9_A>;
#[doc = "I2C Bus Interface 0 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB9_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPB9_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB9_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPB9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPB9_A {
        match self.bits {
            false => MSTPB9_A::_0,
            true => MSTPB9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB9_A::_1
    }
}
#[doc = "Field `MSTPB9` writer - I2C Bus Interface 0 Module Stop"]
pub type MSTPB9_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRB_SPEC, MSTPB9_A, O>;
impl<'a, const O: u8> MSTPB9_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPB9_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPB9_A::_1)
    }
}
#[doc = "Field `MSTPB11` reader - Universal Serial Bus 2.0 FS Interface Module Stop"]
pub type MSTPB11_R = crate::BitReader<MSTPB11_A>;
#[doc = "Universal Serial Bus 2.0 FS Interface Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB11_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPB11_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB11_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPB11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPB11_A {
        match self.bits {
            false => MSTPB11_A::_0,
            true => MSTPB11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB11_A::_1
    }
}
#[doc = "Field `MSTPB11` writer - Universal Serial Bus 2.0 FS Interface Module Stop"]
pub type MSTPB11_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRB_SPEC, MSTPB11_A, O>;
impl<'a, const O: u8> MSTPB11_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPB11_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPB11_A::_1)
    }
}
#[doc = "Field `MSTPB18` reader - Serial Peripheral Interface 1 Module Stop"]
pub type MSTPB18_R = crate::BitReader<MSTPB18_A>;
#[doc = "Serial Peripheral Interface 1 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB18_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPB18_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB18_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPB18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPB18_A {
        match self.bits {
            false => MSTPB18_A::_0,
            true => MSTPB18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB18_A::_1
    }
}
#[doc = "Field `MSTPB18` writer - Serial Peripheral Interface 1 Module Stop"]
pub type MSTPB18_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRB_SPEC, MSTPB18_A, O>;
impl<'a, const O: u8> MSTPB18_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPB18_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPB18_A::_1)
    }
}
#[doc = "Field `MSTPB19` reader - Serial Peripheral Interface 0 Module Stop"]
pub type MSTPB19_R = crate::BitReader<MSTPB19_A>;
#[doc = "Serial Peripheral Interface 0 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB19_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPB19_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB19_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPB19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPB19_A {
        match self.bits {
            false => MSTPB19_A::_0,
            true => MSTPB19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB19_A::_1
    }
}
#[doc = "Field `MSTPB19` writer - Serial Peripheral Interface 0 Module Stop"]
pub type MSTPB19_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRB_SPEC, MSTPB19_A, O>;
impl<'a, const O: u8> MSTPB19_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPB19_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPB19_A::_1)
    }
}
#[doc = "Field `MSTPB22` reader - Serial Communication Interface 9 Module Stop"]
pub type MSTPB22_R = crate::BitReader<MSTPB22_A>;
#[doc = "Serial Communication Interface 9 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB22_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPB22_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB22_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPB22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPB22_A {
        match self.bits {
            false => MSTPB22_A::_0,
            true => MSTPB22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB22_A::_1
    }
}
#[doc = "Field `MSTPB22` writer - Serial Communication Interface 9 Module Stop"]
pub type MSTPB22_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRB_SPEC, MSTPB22_A, O>;
impl<'a, const O: u8> MSTPB22_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPB22_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPB22_A::_1)
    }
}
#[doc = "Field `MSTPB27` reader - Serial Communication Interface 4 Module Stop"]
pub type MSTPB27_R = crate::BitReader<MSTPB27_A>;
#[doc = "Serial Communication Interface 4 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB27_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPB27_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB27_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPB27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPB27_A {
        match self.bits {
            false => MSTPB27_A::_0,
            true => MSTPB27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB27_A::_1
    }
}
#[doc = "Field `MSTPB27` writer - Serial Communication Interface 4 Module Stop"]
pub type MSTPB27_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRB_SPEC, MSTPB27_A, O>;
impl<'a, const O: u8> MSTPB27_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPB27_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPB27_A::_1)
    }
}
#[doc = "Field `MSTPB28` reader - Serial Communication Interface 3 Module Stop"]
pub type MSTPB28_R = crate::BitReader<MSTPB28_A>;
#[doc = "Serial Communication Interface 3 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB28_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPB28_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB28_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPB28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPB28_A {
        match self.bits {
            false => MSTPB28_A::_0,
            true => MSTPB28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB28_A::_1
    }
}
#[doc = "Field `MSTPB28` writer - Serial Communication Interface 3 Module Stop"]
pub type MSTPB28_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRB_SPEC, MSTPB28_A, O>;
impl<'a, const O: u8> MSTPB28_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPB28_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPB28_A::_1)
    }
}
#[doc = "Field `MSTPB29` reader - Serial Communication Interface 2 Module Stop"]
pub type MSTPB29_R = crate::BitReader<MSTPB29_A>;
#[doc = "Serial Communication Interface 2 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB29_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPB29_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB29_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPB29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPB29_A {
        match self.bits {
            false => MSTPB29_A::_0,
            true => MSTPB29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB29_A::_1
    }
}
#[doc = "Field `MSTPB29` writer - Serial Communication Interface 2 Module Stop"]
pub type MSTPB29_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRB_SPEC, MSTPB29_A, O>;
impl<'a, const O: u8> MSTPB29_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPB29_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPB29_A::_1)
    }
}
#[doc = "Field `MSTPB30` reader - Serial Communication Interface 1 Module Stop"]
pub type MSTPB30_R = crate::BitReader<MSTPB30_A>;
#[doc = "Serial Communication Interface 1 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB30_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPB30_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB30_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPB30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPB30_A {
        match self.bits {
            false => MSTPB30_A::_0,
            true => MSTPB30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB30_A::_1
    }
}
#[doc = "Field `MSTPB30` writer - Serial Communication Interface 1 Module Stop"]
pub type MSTPB30_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRB_SPEC, MSTPB30_A, O>;
impl<'a, const O: u8> MSTPB30_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPB30_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPB30_A::_1)
    }
}
#[doc = "Field `MSTPB31` reader - Serial Communication Interface 0 Module Stop"]
pub type MSTPB31_R = crate::BitReader<MSTPB31_A>;
#[doc = "Serial Communication Interface 0 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB31_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPB31_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB31_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPB31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPB31_A {
        match self.bits {
            false => MSTPB31_A::_0,
            true => MSTPB31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB31_A::_1
    }
}
#[doc = "Field `MSTPB31` writer - Serial Communication Interface 0 Module Stop"]
pub type MSTPB31_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRB_SPEC, MSTPB31_A, O>;
impl<'a, const O: u8> MSTPB31_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPB31_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPB31_A::_1)
    }
}
impl R {
    #[doc = "Bit 2 - Controller Area Network Module Stop"]
    #[inline(always)]
    pub fn mstpb2(&self) -> MSTPB2_R {
        MSTPB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Queued Serial Peripheral Interface Module Stop"]
    #[inline(always)]
    pub fn mstpb6(&self) -> MSTPB6_R {
        MSTPB6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Bus Interface 2 Module Stop"]
    #[inline(always)]
    pub fn mstpb7(&self) -> MSTPB7_R {
        MSTPB7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C Bus Interface 1 Module Stop"]
    #[inline(always)]
    pub fn mstpb8(&self) -> MSTPB8_R {
        MSTPB8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C Bus Interface 0 Module Stop"]
    #[inline(always)]
    pub fn mstpb9(&self) -> MSTPB9_R {
        MSTPB9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Universal Serial Bus 2.0 FS Interface Module Stop"]
    #[inline(always)]
    pub fn mstpb11(&self) -> MSTPB11_R {
        MSTPB11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 18 - Serial Peripheral Interface 1 Module Stop"]
    #[inline(always)]
    pub fn mstpb18(&self) -> MSTPB18_R {
        MSTPB18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Serial Peripheral Interface 0 Module Stop"]
    #[inline(always)]
    pub fn mstpb19(&self) -> MSTPB19_R {
        MSTPB19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - Serial Communication Interface 9 Module Stop"]
    #[inline(always)]
    pub fn mstpb22(&self) -> MSTPB22_R {
        MSTPB22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 27 - Serial Communication Interface 4 Module Stop"]
    #[inline(always)]
    pub fn mstpb27(&self) -> MSTPB27_R {
        MSTPB27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Serial Communication Interface 3 Module Stop"]
    #[inline(always)]
    pub fn mstpb28(&self) -> MSTPB28_R {
        MSTPB28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Serial Communication Interface 2 Module Stop"]
    #[inline(always)]
    pub fn mstpb29(&self) -> MSTPB29_R {
        MSTPB29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Serial Communication Interface 1 Module Stop"]
    #[inline(always)]
    pub fn mstpb30(&self) -> MSTPB30_R {
        MSTPB30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Serial Communication Interface 0 Module Stop"]
    #[inline(always)]
    pub fn mstpb31(&self) -> MSTPB31_R {
        MSTPB31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Controller Area Network Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpb2(&mut self) -> MSTPB2_W<2> {
        MSTPB2_W::new(self)
    }
    #[doc = "Bit 6 - Queued Serial Peripheral Interface Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpb6(&mut self) -> MSTPB6_W<6> {
        MSTPB6_W::new(self)
    }
    #[doc = "Bit 7 - I2C Bus Interface 2 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpb7(&mut self) -> MSTPB7_W<7> {
        MSTPB7_W::new(self)
    }
    #[doc = "Bit 8 - I2C Bus Interface 1 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpb8(&mut self) -> MSTPB8_W<8> {
        MSTPB8_W::new(self)
    }
    #[doc = "Bit 9 - I2C Bus Interface 0 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpb9(&mut self) -> MSTPB9_W<9> {
        MSTPB9_W::new(self)
    }
    #[doc = "Bit 11 - Universal Serial Bus 2.0 FS Interface Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpb11(&mut self) -> MSTPB11_W<11> {
        MSTPB11_W::new(self)
    }
    #[doc = "Bit 18 - Serial Peripheral Interface 1 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpb18(&mut self) -> MSTPB18_W<18> {
        MSTPB18_W::new(self)
    }
    #[doc = "Bit 19 - Serial Peripheral Interface 0 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpb19(&mut self) -> MSTPB19_W<19> {
        MSTPB19_W::new(self)
    }
    #[doc = "Bit 22 - Serial Communication Interface 9 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpb22(&mut self) -> MSTPB22_W<22> {
        MSTPB22_W::new(self)
    }
    #[doc = "Bit 27 - Serial Communication Interface 4 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpb27(&mut self) -> MSTPB27_W<27> {
        MSTPB27_W::new(self)
    }
    #[doc = "Bit 28 - Serial Communication Interface 3 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpb28(&mut self) -> MSTPB28_W<28> {
        MSTPB28_W::new(self)
    }
    #[doc = "Bit 29 - Serial Communication Interface 2 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpb29(&mut self) -> MSTPB29_W<29> {
        MSTPB29_W::new(self)
    }
    #[doc = "Bit 30 - Serial Communication Interface 1 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpb30(&mut self) -> MSTPB30_W<30> {
        MSTPB30_W::new(self)
    }
    #[doc = "Bit 31 - Serial Communication Interface 0 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpb31(&mut self) -> MSTPB31_W<31> {
        MSTPB31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Stop Control Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mstpcrb](index.html) module"]
pub struct MSTPCRB_SPEC;
impl crate::RegisterSpec for MSTPCRB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mstpcrb::R](R) reader structure"]
impl crate::Readable for MSTPCRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mstpcrb::W](W) writer structure"]
impl crate::Writable for MSTPCRB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSTPCRB to value 0xffff_ffff"]
impl crate::Resettable for MSTPCRB_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
