#[doc = "Register `PDR` reader"]
pub struct R(crate::R<PDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDR` writer"]
pub struct W(crate::W<PDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDR_SPEC>;
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
impl From<crate::W<PDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDR00` reader - Pmn Direction"]
pub type PDR00_R = crate::BitReader<PDR00_A>;
#[doc = "Pmn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDR00_A {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<PDR00_A> for bool {
    #[inline(always)]
    fn from(variant: PDR00_A) -> Self {
        variant as u8 != 0
    }
}
impl PDR00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDR00_A {
        match self.bits {
            false => PDR00_A::_0,
            true => PDR00_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDR00_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDR00_A::_1
    }
}
#[doc = "Field `PDR00` writer - Pmn Direction"]
pub type PDR00_W<'a, const O: u8> = crate::BitWriter<'a, u16, PDR_SPEC, PDR00_A, O>;
impl<'a, const O: u8> PDR00_W<'a, O> {
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDR00_A::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDR00_A::_1)
    }
}
#[doc = "Field `PDR01` reader - Pmn Direction"]
pub type PDR01_R = crate::BitReader<PDR01_A>;
#[doc = "Pmn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDR01_A {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<PDR01_A> for bool {
    #[inline(always)]
    fn from(variant: PDR01_A) -> Self {
        variant as u8 != 0
    }
}
impl PDR01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDR01_A {
        match self.bits {
            false => PDR01_A::_0,
            true => PDR01_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDR01_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDR01_A::_1
    }
}
#[doc = "Field `PDR01` writer - Pmn Direction"]
pub type PDR01_W<'a, const O: u8> = crate::BitWriter<'a, u16, PDR_SPEC, PDR01_A, O>;
impl<'a, const O: u8> PDR01_W<'a, O> {
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDR01_A::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDR01_A::_1)
    }
}
#[doc = "Field `PDR02` reader - Pmn Direction"]
pub type PDR02_R = crate::BitReader<PDR02_A>;
#[doc = "Pmn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDR02_A {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<PDR02_A> for bool {
    #[inline(always)]
    fn from(variant: PDR02_A) -> Self {
        variant as u8 != 0
    }
}
impl PDR02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDR02_A {
        match self.bits {
            false => PDR02_A::_0,
            true => PDR02_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDR02_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDR02_A::_1
    }
}
#[doc = "Field `PDR02` writer - Pmn Direction"]
pub type PDR02_W<'a, const O: u8> = crate::BitWriter<'a, u16, PDR_SPEC, PDR02_A, O>;
impl<'a, const O: u8> PDR02_W<'a, O> {
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDR02_A::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDR02_A::_1)
    }
}
#[doc = "Field `PDR03` reader - Pmn Direction"]
pub type PDR03_R = crate::BitReader<PDR03_A>;
#[doc = "Pmn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDR03_A {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<PDR03_A> for bool {
    #[inline(always)]
    fn from(variant: PDR03_A) -> Self {
        variant as u8 != 0
    }
}
impl PDR03_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDR03_A {
        match self.bits {
            false => PDR03_A::_0,
            true => PDR03_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDR03_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDR03_A::_1
    }
}
#[doc = "Field `PDR03` writer - Pmn Direction"]
pub type PDR03_W<'a, const O: u8> = crate::BitWriter<'a, u16, PDR_SPEC, PDR03_A, O>;
impl<'a, const O: u8> PDR03_W<'a, O> {
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDR03_A::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDR03_A::_1)
    }
}
#[doc = "Field `PDR04` reader - Pmn Direction"]
pub type PDR04_R = crate::BitReader<PDR04_A>;
#[doc = "Pmn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDR04_A {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<PDR04_A> for bool {
    #[inline(always)]
    fn from(variant: PDR04_A) -> Self {
        variant as u8 != 0
    }
}
impl PDR04_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDR04_A {
        match self.bits {
            false => PDR04_A::_0,
            true => PDR04_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDR04_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDR04_A::_1
    }
}
#[doc = "Field `PDR04` writer - Pmn Direction"]
pub type PDR04_W<'a, const O: u8> = crate::BitWriter<'a, u16, PDR_SPEC, PDR04_A, O>;
impl<'a, const O: u8> PDR04_W<'a, O> {
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDR04_A::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDR04_A::_1)
    }
}
#[doc = "Field `PDR05` reader - Pmn Direction"]
pub type PDR05_R = crate::BitReader<PDR05_A>;
#[doc = "Pmn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDR05_A {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<PDR05_A> for bool {
    #[inline(always)]
    fn from(variant: PDR05_A) -> Self {
        variant as u8 != 0
    }
}
impl PDR05_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDR05_A {
        match self.bits {
            false => PDR05_A::_0,
            true => PDR05_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDR05_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDR05_A::_1
    }
}
#[doc = "Field `PDR05` writer - Pmn Direction"]
pub type PDR05_W<'a, const O: u8> = crate::BitWriter<'a, u16, PDR_SPEC, PDR05_A, O>;
impl<'a, const O: u8> PDR05_W<'a, O> {
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDR05_A::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDR05_A::_1)
    }
}
#[doc = "Field `PDR06` reader - Pmn Direction"]
pub type PDR06_R = crate::BitReader<PDR06_A>;
#[doc = "Pmn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDR06_A {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<PDR06_A> for bool {
    #[inline(always)]
    fn from(variant: PDR06_A) -> Self {
        variant as u8 != 0
    }
}
impl PDR06_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDR06_A {
        match self.bits {
            false => PDR06_A::_0,
            true => PDR06_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDR06_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDR06_A::_1
    }
}
#[doc = "Field `PDR06` writer - Pmn Direction"]
pub type PDR06_W<'a, const O: u8> = crate::BitWriter<'a, u16, PDR_SPEC, PDR06_A, O>;
impl<'a, const O: u8> PDR06_W<'a, O> {
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDR06_A::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDR06_A::_1)
    }
}
#[doc = "Field `PDR07` reader - Pmn Direction"]
pub type PDR07_R = crate::BitReader<PDR07_A>;
#[doc = "Pmn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDR07_A {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<PDR07_A> for bool {
    #[inline(always)]
    fn from(variant: PDR07_A) -> Self {
        variant as u8 != 0
    }
}
impl PDR07_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDR07_A {
        match self.bits {
            false => PDR07_A::_0,
            true => PDR07_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDR07_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDR07_A::_1
    }
}
#[doc = "Field `PDR07` writer - Pmn Direction"]
pub type PDR07_W<'a, const O: u8> = crate::BitWriter<'a, u16, PDR_SPEC, PDR07_A, O>;
impl<'a, const O: u8> PDR07_W<'a, O> {
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDR07_A::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDR07_A::_1)
    }
}
#[doc = "Field `PDR08` reader - Pmn Direction"]
pub type PDR08_R = crate::BitReader<PDR08_A>;
#[doc = "Pmn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDR08_A {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<PDR08_A> for bool {
    #[inline(always)]
    fn from(variant: PDR08_A) -> Self {
        variant as u8 != 0
    }
}
impl PDR08_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDR08_A {
        match self.bits {
            false => PDR08_A::_0,
            true => PDR08_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDR08_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDR08_A::_1
    }
}
#[doc = "Field `PDR08` writer - Pmn Direction"]
pub type PDR08_W<'a, const O: u8> = crate::BitWriter<'a, u16, PDR_SPEC, PDR08_A, O>;
impl<'a, const O: u8> PDR08_W<'a, O> {
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDR08_A::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDR08_A::_1)
    }
}
#[doc = "Field `PDR09` reader - Pmn Direction"]
pub type PDR09_R = crate::BitReader<PDR09_A>;
#[doc = "Pmn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDR09_A {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<PDR09_A> for bool {
    #[inline(always)]
    fn from(variant: PDR09_A) -> Self {
        variant as u8 != 0
    }
}
impl PDR09_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDR09_A {
        match self.bits {
            false => PDR09_A::_0,
            true => PDR09_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDR09_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDR09_A::_1
    }
}
#[doc = "Field `PDR09` writer - Pmn Direction"]
pub type PDR09_W<'a, const O: u8> = crate::BitWriter<'a, u16, PDR_SPEC, PDR09_A, O>;
impl<'a, const O: u8> PDR09_W<'a, O> {
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDR09_A::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDR09_A::_1)
    }
}
#[doc = "Field `PDR10` reader - Pmn Direction"]
pub type PDR10_R = crate::BitReader<PDR10_A>;
#[doc = "Pmn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDR10_A {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<PDR10_A> for bool {
    #[inline(always)]
    fn from(variant: PDR10_A) -> Self {
        variant as u8 != 0
    }
}
impl PDR10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDR10_A {
        match self.bits {
            false => PDR10_A::_0,
            true => PDR10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDR10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDR10_A::_1
    }
}
#[doc = "Field `PDR10` writer - Pmn Direction"]
pub type PDR10_W<'a, const O: u8> = crate::BitWriter<'a, u16, PDR_SPEC, PDR10_A, O>;
impl<'a, const O: u8> PDR10_W<'a, O> {
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDR10_A::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDR10_A::_1)
    }
}
#[doc = "Field `PDR11` reader - Pmn Direction"]
pub type PDR11_R = crate::BitReader<PDR11_A>;
#[doc = "Pmn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDR11_A {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<PDR11_A> for bool {
    #[inline(always)]
    fn from(variant: PDR11_A) -> Self {
        variant as u8 != 0
    }
}
impl PDR11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDR11_A {
        match self.bits {
            false => PDR11_A::_0,
            true => PDR11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDR11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDR11_A::_1
    }
}
#[doc = "Field `PDR11` writer - Pmn Direction"]
pub type PDR11_W<'a, const O: u8> = crate::BitWriter<'a, u16, PDR_SPEC, PDR11_A, O>;
impl<'a, const O: u8> PDR11_W<'a, O> {
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDR11_A::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDR11_A::_1)
    }
}
#[doc = "Field `PDR12` reader - Pmn Direction"]
pub type PDR12_R = crate::BitReader<PDR12_A>;
#[doc = "Pmn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDR12_A {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<PDR12_A> for bool {
    #[inline(always)]
    fn from(variant: PDR12_A) -> Self {
        variant as u8 != 0
    }
}
impl PDR12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDR12_A {
        match self.bits {
            false => PDR12_A::_0,
            true => PDR12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDR12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDR12_A::_1
    }
}
#[doc = "Field `PDR12` writer - Pmn Direction"]
pub type PDR12_W<'a, const O: u8> = crate::BitWriter<'a, u16, PDR_SPEC, PDR12_A, O>;
impl<'a, const O: u8> PDR12_W<'a, O> {
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDR12_A::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDR12_A::_1)
    }
}
#[doc = "Field `PDR13` reader - Pmn Direction"]
pub type PDR13_R = crate::BitReader<PDR13_A>;
#[doc = "Pmn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDR13_A {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<PDR13_A> for bool {
    #[inline(always)]
    fn from(variant: PDR13_A) -> Self {
        variant as u8 != 0
    }
}
impl PDR13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDR13_A {
        match self.bits {
            false => PDR13_A::_0,
            true => PDR13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDR13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDR13_A::_1
    }
}
#[doc = "Field `PDR13` writer - Pmn Direction"]
pub type PDR13_W<'a, const O: u8> = crate::BitWriter<'a, u16, PDR_SPEC, PDR13_A, O>;
impl<'a, const O: u8> PDR13_W<'a, O> {
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDR13_A::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDR13_A::_1)
    }
}
#[doc = "Field `PDR14` reader - Pmn Direction"]
pub type PDR14_R = crate::BitReader<PDR14_A>;
#[doc = "Pmn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDR14_A {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<PDR14_A> for bool {
    #[inline(always)]
    fn from(variant: PDR14_A) -> Self {
        variant as u8 != 0
    }
}
impl PDR14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDR14_A {
        match self.bits {
            false => PDR14_A::_0,
            true => PDR14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDR14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDR14_A::_1
    }
}
#[doc = "Field `PDR14` writer - Pmn Direction"]
pub type PDR14_W<'a, const O: u8> = crate::BitWriter<'a, u16, PDR_SPEC, PDR14_A, O>;
impl<'a, const O: u8> PDR14_W<'a, O> {
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDR14_A::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDR14_A::_1)
    }
}
#[doc = "Field `PDR15` reader - Pmn Direction"]
pub type PDR15_R = crate::BitReader<PDR15_A>;
#[doc = "Pmn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDR15_A {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<PDR15_A> for bool {
    #[inline(always)]
    fn from(variant: PDR15_A) -> Self {
        variant as u8 != 0
    }
}
impl PDR15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDR15_A {
        match self.bits {
            false => PDR15_A::_0,
            true => PDR15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDR15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDR15_A::_1
    }
}
#[doc = "Field `PDR15` writer - Pmn Direction"]
pub type PDR15_W<'a, const O: u8> = crate::BitWriter<'a, u16, PDR_SPEC, PDR15_A, O>;
impl<'a, const O: u8> PDR15_W<'a, O> {
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDR15_A::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDR15_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Pmn Direction"]
    #[inline(always)]
    pub fn pdr00(&self) -> PDR00_R {
        PDR00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pmn Direction"]
    #[inline(always)]
    pub fn pdr01(&self) -> PDR01_R {
        PDR01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pmn Direction"]
    #[inline(always)]
    pub fn pdr02(&self) -> PDR02_R {
        PDR02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pmn Direction"]
    #[inline(always)]
    pub fn pdr03(&self) -> PDR03_R {
        PDR03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pmn Direction"]
    #[inline(always)]
    pub fn pdr04(&self) -> PDR04_R {
        PDR04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pmn Direction"]
    #[inline(always)]
    pub fn pdr05(&self) -> PDR05_R {
        PDR05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pmn Direction"]
    #[inline(always)]
    pub fn pdr06(&self) -> PDR06_R {
        PDR06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pmn Direction"]
    #[inline(always)]
    pub fn pdr07(&self) -> PDR07_R {
        PDR07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pmn Direction"]
    #[inline(always)]
    pub fn pdr08(&self) -> PDR08_R {
        PDR08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pmn Direction"]
    #[inline(always)]
    pub fn pdr09(&self) -> PDR09_R {
        PDR09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pmn Direction"]
    #[inline(always)]
    pub fn pdr10(&self) -> PDR10_R {
        PDR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pmn Direction"]
    #[inline(always)]
    pub fn pdr11(&self) -> PDR11_R {
        PDR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pmn Direction"]
    #[inline(always)]
    pub fn pdr12(&self) -> PDR12_R {
        PDR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pmn Direction"]
    #[inline(always)]
    pub fn pdr13(&self) -> PDR13_R {
        PDR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pmn Direction"]
    #[inline(always)]
    pub fn pdr14(&self) -> PDR14_R {
        PDR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pmn Direction"]
    #[inline(always)]
    pub fn pdr15(&self) -> PDR15_R {
        PDR15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pmn Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdr00(&mut self) -> PDR00_W<0> {
        PDR00_W::new(self)
    }
    #[doc = "Bit 1 - Pmn Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdr01(&mut self) -> PDR01_W<1> {
        PDR01_W::new(self)
    }
    #[doc = "Bit 2 - Pmn Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdr02(&mut self) -> PDR02_W<2> {
        PDR02_W::new(self)
    }
    #[doc = "Bit 3 - Pmn Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdr03(&mut self) -> PDR03_W<3> {
        PDR03_W::new(self)
    }
    #[doc = "Bit 4 - Pmn Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdr04(&mut self) -> PDR04_W<4> {
        PDR04_W::new(self)
    }
    #[doc = "Bit 5 - Pmn Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdr05(&mut self) -> PDR05_W<5> {
        PDR05_W::new(self)
    }
    #[doc = "Bit 6 - Pmn Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdr06(&mut self) -> PDR06_W<6> {
        PDR06_W::new(self)
    }
    #[doc = "Bit 7 - Pmn Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdr07(&mut self) -> PDR07_W<7> {
        PDR07_W::new(self)
    }
    #[doc = "Bit 8 - Pmn Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdr08(&mut self) -> PDR08_W<8> {
        PDR08_W::new(self)
    }
    #[doc = "Bit 9 - Pmn Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdr09(&mut self) -> PDR09_W<9> {
        PDR09_W::new(self)
    }
    #[doc = "Bit 10 - Pmn Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdr10(&mut self) -> PDR10_W<10> {
        PDR10_W::new(self)
    }
    #[doc = "Bit 11 - Pmn Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdr11(&mut self) -> PDR11_W<11> {
        PDR11_W::new(self)
    }
    #[doc = "Bit 12 - Pmn Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdr12(&mut self) -> PDR12_W<12> {
        PDR12_W::new(self)
    }
    #[doc = "Bit 13 - Pmn Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdr13(&mut self) -> PDR13_W<13> {
        PDR13_W::new(self)
    }
    #[doc = "Bit 14 - Pmn Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdr14(&mut self) -> PDR14_W<14> {
        PDR14_W::new(self)
    }
    #[doc = "Bit 15 - Pmn Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdr15(&mut self) -> PDR15_W<15> {
        PDR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdr](index.html) module"]
pub struct PDR_SPEC;
impl crate::RegisterSpec for PDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pdr::R](R) reader structure"]
impl crate::Readable for PDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdr::W](W) writer structure"]
impl crate::Writable for PDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDR to value 0"]
impl crate::Resettable for PDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
