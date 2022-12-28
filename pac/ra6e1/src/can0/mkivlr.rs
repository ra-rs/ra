#[doc = "Register `MKIVLR` reader"]
pub struct R(crate::R<MKIVLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MKIVLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MKIVLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MKIVLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MKIVLR` writer"]
pub struct W(crate::W<MKIVLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MKIVLR_SPEC>;
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
impl From<crate::W<MKIVLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MKIVLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MB00` reader - Mask Invalid"]
pub type MB00_R = crate::BitReader<MB00_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB00_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB00_A> for bool {
    #[inline(always)]
    fn from(variant: MB00_A) -> Self {
        variant as u8 != 0
    }
}
impl MB00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB00_A {
        match self.bits {
            false => MB00_A::_0,
            true => MB00_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB00_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB00_A::_1
    }
}
#[doc = "Field `MB00` writer - Mask Invalid"]
pub type MB00_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB00_A, O>;
impl<'a, const O: u8> MB00_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB00_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB00_A::_1)
    }
}
#[doc = "Field `MB01` reader - Mask Invalid"]
pub type MB01_R = crate::BitReader<MB01_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB01_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB01_A> for bool {
    #[inline(always)]
    fn from(variant: MB01_A) -> Self {
        variant as u8 != 0
    }
}
impl MB01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB01_A {
        match self.bits {
            false => MB01_A::_0,
            true => MB01_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB01_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB01_A::_1
    }
}
#[doc = "Field `MB01` writer - Mask Invalid"]
pub type MB01_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB01_A, O>;
impl<'a, const O: u8> MB01_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB01_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB01_A::_1)
    }
}
#[doc = "Field `MB02` reader - Mask Invalid"]
pub type MB02_R = crate::BitReader<MB02_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB02_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB02_A> for bool {
    #[inline(always)]
    fn from(variant: MB02_A) -> Self {
        variant as u8 != 0
    }
}
impl MB02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB02_A {
        match self.bits {
            false => MB02_A::_0,
            true => MB02_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB02_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB02_A::_1
    }
}
#[doc = "Field `MB02` writer - Mask Invalid"]
pub type MB02_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB02_A, O>;
impl<'a, const O: u8> MB02_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB02_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB02_A::_1)
    }
}
#[doc = "Field `MB03` reader - Mask Invalid"]
pub type MB03_R = crate::BitReader<MB03_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB03_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB03_A> for bool {
    #[inline(always)]
    fn from(variant: MB03_A) -> Self {
        variant as u8 != 0
    }
}
impl MB03_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB03_A {
        match self.bits {
            false => MB03_A::_0,
            true => MB03_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB03_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB03_A::_1
    }
}
#[doc = "Field `MB03` writer - Mask Invalid"]
pub type MB03_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB03_A, O>;
impl<'a, const O: u8> MB03_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB03_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB03_A::_1)
    }
}
#[doc = "Field `MB04` reader - Mask Invalid"]
pub type MB04_R = crate::BitReader<MB04_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB04_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB04_A> for bool {
    #[inline(always)]
    fn from(variant: MB04_A) -> Self {
        variant as u8 != 0
    }
}
impl MB04_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB04_A {
        match self.bits {
            false => MB04_A::_0,
            true => MB04_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB04_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB04_A::_1
    }
}
#[doc = "Field `MB04` writer - Mask Invalid"]
pub type MB04_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB04_A, O>;
impl<'a, const O: u8> MB04_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB04_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB04_A::_1)
    }
}
#[doc = "Field `MB05` reader - Mask Invalid"]
pub type MB05_R = crate::BitReader<MB05_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB05_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB05_A> for bool {
    #[inline(always)]
    fn from(variant: MB05_A) -> Self {
        variant as u8 != 0
    }
}
impl MB05_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB05_A {
        match self.bits {
            false => MB05_A::_0,
            true => MB05_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB05_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB05_A::_1
    }
}
#[doc = "Field `MB05` writer - Mask Invalid"]
pub type MB05_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB05_A, O>;
impl<'a, const O: u8> MB05_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB05_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB05_A::_1)
    }
}
#[doc = "Field `MB06` reader - Mask Invalid"]
pub type MB06_R = crate::BitReader<MB06_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB06_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB06_A> for bool {
    #[inline(always)]
    fn from(variant: MB06_A) -> Self {
        variant as u8 != 0
    }
}
impl MB06_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB06_A {
        match self.bits {
            false => MB06_A::_0,
            true => MB06_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB06_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB06_A::_1
    }
}
#[doc = "Field `MB06` writer - Mask Invalid"]
pub type MB06_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB06_A, O>;
impl<'a, const O: u8> MB06_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB06_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB06_A::_1)
    }
}
#[doc = "Field `MB07` reader - Mask Invalid"]
pub type MB07_R = crate::BitReader<MB07_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB07_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB07_A> for bool {
    #[inline(always)]
    fn from(variant: MB07_A) -> Self {
        variant as u8 != 0
    }
}
impl MB07_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB07_A {
        match self.bits {
            false => MB07_A::_0,
            true => MB07_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB07_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB07_A::_1
    }
}
#[doc = "Field `MB07` writer - Mask Invalid"]
pub type MB07_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB07_A, O>;
impl<'a, const O: u8> MB07_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB07_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB07_A::_1)
    }
}
#[doc = "Field `MB08` reader - Mask Invalid"]
pub type MB08_R = crate::BitReader<MB08_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB08_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB08_A> for bool {
    #[inline(always)]
    fn from(variant: MB08_A) -> Self {
        variant as u8 != 0
    }
}
impl MB08_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB08_A {
        match self.bits {
            false => MB08_A::_0,
            true => MB08_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB08_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB08_A::_1
    }
}
#[doc = "Field `MB08` writer - Mask Invalid"]
pub type MB08_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB08_A, O>;
impl<'a, const O: u8> MB08_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB08_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB08_A::_1)
    }
}
#[doc = "Field `MB09` reader - Mask Invalid"]
pub type MB09_R = crate::BitReader<MB09_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB09_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB09_A> for bool {
    #[inline(always)]
    fn from(variant: MB09_A) -> Self {
        variant as u8 != 0
    }
}
impl MB09_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB09_A {
        match self.bits {
            false => MB09_A::_0,
            true => MB09_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB09_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB09_A::_1
    }
}
#[doc = "Field `MB09` writer - Mask Invalid"]
pub type MB09_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB09_A, O>;
impl<'a, const O: u8> MB09_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB09_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB09_A::_1)
    }
}
#[doc = "Field `MB10` reader - Mask Invalid"]
pub type MB10_R = crate::BitReader<MB10_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB10_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB10_A> for bool {
    #[inline(always)]
    fn from(variant: MB10_A) -> Self {
        variant as u8 != 0
    }
}
impl MB10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB10_A {
        match self.bits {
            false => MB10_A::_0,
            true => MB10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB10_A::_1
    }
}
#[doc = "Field `MB10` writer - Mask Invalid"]
pub type MB10_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB10_A, O>;
impl<'a, const O: u8> MB10_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB10_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB10_A::_1)
    }
}
#[doc = "Field `MB11` reader - Mask Invalid"]
pub type MB11_R = crate::BitReader<MB11_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB11_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB11_A> for bool {
    #[inline(always)]
    fn from(variant: MB11_A) -> Self {
        variant as u8 != 0
    }
}
impl MB11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB11_A {
        match self.bits {
            false => MB11_A::_0,
            true => MB11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB11_A::_1
    }
}
#[doc = "Field `MB11` writer - Mask Invalid"]
pub type MB11_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB11_A, O>;
impl<'a, const O: u8> MB11_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB11_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB11_A::_1)
    }
}
#[doc = "Field `MB12` reader - Mask Invalid"]
pub type MB12_R = crate::BitReader<MB12_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB12_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB12_A> for bool {
    #[inline(always)]
    fn from(variant: MB12_A) -> Self {
        variant as u8 != 0
    }
}
impl MB12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB12_A {
        match self.bits {
            false => MB12_A::_0,
            true => MB12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB12_A::_1
    }
}
#[doc = "Field `MB12` writer - Mask Invalid"]
pub type MB12_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB12_A, O>;
impl<'a, const O: u8> MB12_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB12_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB12_A::_1)
    }
}
#[doc = "Field `MB13` reader - Mask Invalid"]
pub type MB13_R = crate::BitReader<MB13_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB13_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB13_A> for bool {
    #[inline(always)]
    fn from(variant: MB13_A) -> Self {
        variant as u8 != 0
    }
}
impl MB13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB13_A {
        match self.bits {
            false => MB13_A::_0,
            true => MB13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB13_A::_1
    }
}
#[doc = "Field `MB13` writer - Mask Invalid"]
pub type MB13_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB13_A, O>;
impl<'a, const O: u8> MB13_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB13_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB13_A::_1)
    }
}
#[doc = "Field `MB14` reader - Mask Invalid"]
pub type MB14_R = crate::BitReader<MB14_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB14_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB14_A> for bool {
    #[inline(always)]
    fn from(variant: MB14_A) -> Self {
        variant as u8 != 0
    }
}
impl MB14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB14_A {
        match self.bits {
            false => MB14_A::_0,
            true => MB14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB14_A::_1
    }
}
#[doc = "Field `MB14` writer - Mask Invalid"]
pub type MB14_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB14_A, O>;
impl<'a, const O: u8> MB14_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB14_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB14_A::_1)
    }
}
#[doc = "Field `MB15` reader - Mask Invalid"]
pub type MB15_R = crate::BitReader<MB15_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB15_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB15_A> for bool {
    #[inline(always)]
    fn from(variant: MB15_A) -> Self {
        variant as u8 != 0
    }
}
impl MB15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB15_A {
        match self.bits {
            false => MB15_A::_0,
            true => MB15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB15_A::_1
    }
}
#[doc = "Field `MB15` writer - Mask Invalid"]
pub type MB15_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB15_A, O>;
impl<'a, const O: u8> MB15_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB15_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB15_A::_1)
    }
}
#[doc = "Field `MB16` reader - Mask Invalid"]
pub type MB16_R = crate::BitReader<MB16_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB16_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB16_A> for bool {
    #[inline(always)]
    fn from(variant: MB16_A) -> Self {
        variant as u8 != 0
    }
}
impl MB16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB16_A {
        match self.bits {
            false => MB16_A::_0,
            true => MB16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB16_A::_1
    }
}
#[doc = "Field `MB16` writer - Mask Invalid"]
pub type MB16_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB16_A, O>;
impl<'a, const O: u8> MB16_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB16_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB16_A::_1)
    }
}
#[doc = "Field `MB17` reader - Mask Invalid"]
pub type MB17_R = crate::BitReader<MB17_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB17_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB17_A> for bool {
    #[inline(always)]
    fn from(variant: MB17_A) -> Self {
        variant as u8 != 0
    }
}
impl MB17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB17_A {
        match self.bits {
            false => MB17_A::_0,
            true => MB17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB17_A::_1
    }
}
#[doc = "Field `MB17` writer - Mask Invalid"]
pub type MB17_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB17_A, O>;
impl<'a, const O: u8> MB17_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB17_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB17_A::_1)
    }
}
#[doc = "Field `MB18` reader - Mask Invalid"]
pub type MB18_R = crate::BitReader<MB18_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB18_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB18_A> for bool {
    #[inline(always)]
    fn from(variant: MB18_A) -> Self {
        variant as u8 != 0
    }
}
impl MB18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB18_A {
        match self.bits {
            false => MB18_A::_0,
            true => MB18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB18_A::_1
    }
}
#[doc = "Field `MB18` writer - Mask Invalid"]
pub type MB18_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB18_A, O>;
impl<'a, const O: u8> MB18_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB18_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB18_A::_1)
    }
}
#[doc = "Field `MB19` reader - Mask Invalid"]
pub type MB19_R = crate::BitReader<MB19_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB19_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB19_A> for bool {
    #[inline(always)]
    fn from(variant: MB19_A) -> Self {
        variant as u8 != 0
    }
}
impl MB19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB19_A {
        match self.bits {
            false => MB19_A::_0,
            true => MB19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB19_A::_1
    }
}
#[doc = "Field `MB19` writer - Mask Invalid"]
pub type MB19_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB19_A, O>;
impl<'a, const O: u8> MB19_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB19_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB19_A::_1)
    }
}
#[doc = "Field `MB20` reader - Mask Invalid"]
pub type MB20_R = crate::BitReader<MB20_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB20_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB20_A> for bool {
    #[inline(always)]
    fn from(variant: MB20_A) -> Self {
        variant as u8 != 0
    }
}
impl MB20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB20_A {
        match self.bits {
            false => MB20_A::_0,
            true => MB20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB20_A::_1
    }
}
#[doc = "Field `MB20` writer - Mask Invalid"]
pub type MB20_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB20_A, O>;
impl<'a, const O: u8> MB20_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB20_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB20_A::_1)
    }
}
#[doc = "Field `MB21` reader - Mask Invalid"]
pub type MB21_R = crate::BitReader<MB21_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB21_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB21_A> for bool {
    #[inline(always)]
    fn from(variant: MB21_A) -> Self {
        variant as u8 != 0
    }
}
impl MB21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB21_A {
        match self.bits {
            false => MB21_A::_0,
            true => MB21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB21_A::_1
    }
}
#[doc = "Field `MB21` writer - Mask Invalid"]
pub type MB21_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB21_A, O>;
impl<'a, const O: u8> MB21_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB21_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB21_A::_1)
    }
}
#[doc = "Field `MB22` reader - Mask Invalid"]
pub type MB22_R = crate::BitReader<MB22_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB22_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB22_A> for bool {
    #[inline(always)]
    fn from(variant: MB22_A) -> Self {
        variant as u8 != 0
    }
}
impl MB22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB22_A {
        match self.bits {
            false => MB22_A::_0,
            true => MB22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB22_A::_1
    }
}
#[doc = "Field `MB22` writer - Mask Invalid"]
pub type MB22_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB22_A, O>;
impl<'a, const O: u8> MB22_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB22_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB22_A::_1)
    }
}
#[doc = "Field `MB23` reader - Mask Invalid"]
pub type MB23_R = crate::BitReader<MB23_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB23_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB23_A> for bool {
    #[inline(always)]
    fn from(variant: MB23_A) -> Self {
        variant as u8 != 0
    }
}
impl MB23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB23_A {
        match self.bits {
            false => MB23_A::_0,
            true => MB23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB23_A::_1
    }
}
#[doc = "Field `MB23` writer - Mask Invalid"]
pub type MB23_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB23_A, O>;
impl<'a, const O: u8> MB23_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB23_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB23_A::_1)
    }
}
#[doc = "Field `MB24` reader - Mask Invalid"]
pub type MB24_R = crate::BitReader<MB24_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB24_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB24_A> for bool {
    #[inline(always)]
    fn from(variant: MB24_A) -> Self {
        variant as u8 != 0
    }
}
impl MB24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB24_A {
        match self.bits {
            false => MB24_A::_0,
            true => MB24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB24_A::_1
    }
}
#[doc = "Field `MB24` writer - Mask Invalid"]
pub type MB24_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB24_A, O>;
impl<'a, const O: u8> MB24_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB24_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB24_A::_1)
    }
}
#[doc = "Field `MB25` reader - Mask Invalid"]
pub type MB25_R = crate::BitReader<MB25_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB25_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB25_A> for bool {
    #[inline(always)]
    fn from(variant: MB25_A) -> Self {
        variant as u8 != 0
    }
}
impl MB25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB25_A {
        match self.bits {
            false => MB25_A::_0,
            true => MB25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB25_A::_1
    }
}
#[doc = "Field `MB25` writer - Mask Invalid"]
pub type MB25_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB25_A, O>;
impl<'a, const O: u8> MB25_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB25_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB25_A::_1)
    }
}
#[doc = "Field `MB26` reader - Mask Invalid"]
pub type MB26_R = crate::BitReader<MB26_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB26_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB26_A> for bool {
    #[inline(always)]
    fn from(variant: MB26_A) -> Self {
        variant as u8 != 0
    }
}
impl MB26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB26_A {
        match self.bits {
            false => MB26_A::_0,
            true => MB26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB26_A::_1
    }
}
#[doc = "Field `MB26` writer - Mask Invalid"]
pub type MB26_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB26_A, O>;
impl<'a, const O: u8> MB26_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB26_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB26_A::_1)
    }
}
#[doc = "Field `MB27` reader - Mask Invalid"]
pub type MB27_R = crate::BitReader<MB27_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB27_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB27_A> for bool {
    #[inline(always)]
    fn from(variant: MB27_A) -> Self {
        variant as u8 != 0
    }
}
impl MB27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB27_A {
        match self.bits {
            false => MB27_A::_0,
            true => MB27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB27_A::_1
    }
}
#[doc = "Field `MB27` writer - Mask Invalid"]
pub type MB27_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB27_A, O>;
impl<'a, const O: u8> MB27_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB27_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB27_A::_1)
    }
}
#[doc = "Field `MB28` reader - Mask Invalid"]
pub type MB28_R = crate::BitReader<MB28_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB28_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB28_A> for bool {
    #[inline(always)]
    fn from(variant: MB28_A) -> Self {
        variant as u8 != 0
    }
}
impl MB28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB28_A {
        match self.bits {
            false => MB28_A::_0,
            true => MB28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB28_A::_1
    }
}
#[doc = "Field `MB28` writer - Mask Invalid"]
pub type MB28_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB28_A, O>;
impl<'a, const O: u8> MB28_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB28_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB28_A::_1)
    }
}
#[doc = "Field `MB29` reader - Mask Invalid"]
pub type MB29_R = crate::BitReader<MB29_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB29_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB29_A> for bool {
    #[inline(always)]
    fn from(variant: MB29_A) -> Self {
        variant as u8 != 0
    }
}
impl MB29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB29_A {
        match self.bits {
            false => MB29_A::_0,
            true => MB29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB29_A::_1
    }
}
#[doc = "Field `MB29` writer - Mask Invalid"]
pub type MB29_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB29_A, O>;
impl<'a, const O: u8> MB29_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB29_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB29_A::_1)
    }
}
#[doc = "Field `MB30` reader - Mask Invalid"]
pub type MB30_R = crate::BitReader<MB30_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB30_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB30_A> for bool {
    #[inline(always)]
    fn from(variant: MB30_A) -> Self {
        variant as u8 != 0
    }
}
impl MB30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB30_A {
        match self.bits {
            false => MB30_A::_0,
            true => MB30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB30_A::_1
    }
}
#[doc = "Field `MB30` writer - Mask Invalid"]
pub type MB30_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB30_A, O>;
impl<'a, const O: u8> MB30_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB30_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB30_A::_1)
    }
}
#[doc = "Field `MB31` reader - Mask Invalid"]
pub type MB31_R = crate::BitReader<MB31_A>;
#[doc = "Mask Invalid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB31_A {
    #[doc = "0: Mask valid"]
    _0 = 0,
    #[doc = "1: Mask invalid"]
    _1 = 1,
}
impl From<MB31_A> for bool {
    #[inline(always)]
    fn from(variant: MB31_A) -> Self {
        variant as u8 != 0
    }
}
impl MB31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB31_A {
        match self.bits {
            false => MB31_A::_0,
            true => MB31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB31_A::_1
    }
}
#[doc = "Field `MB31` writer - Mask Invalid"]
pub type MB31_W<'a, const O: u8> = crate::BitWriter<'a, u32, MKIVLR_SPEC, MB31_A, O>;
impl<'a, const O: u8> MB31_W<'a, O> {
    #[doc = "Mask valid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MB31_A::_0)
    }
    #[doc = "Mask invalid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MB31_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Mask Invalid"]
    #[inline(always)]
    pub fn mb00(&self) -> MB00_R {
        MB00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask Invalid"]
    #[inline(always)]
    pub fn mb01(&self) -> MB01_R {
        MB01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask Invalid"]
    #[inline(always)]
    pub fn mb02(&self) -> MB02_R {
        MB02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask Invalid"]
    #[inline(always)]
    pub fn mb03(&self) -> MB03_R {
        MB03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask Invalid"]
    #[inline(always)]
    pub fn mb04(&self) -> MB04_R {
        MB04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask Invalid"]
    #[inline(always)]
    pub fn mb05(&self) -> MB05_R {
        MB05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask Invalid"]
    #[inline(always)]
    pub fn mb06(&self) -> MB06_R {
        MB06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mask Invalid"]
    #[inline(always)]
    pub fn mb07(&self) -> MB07_R {
        MB07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask Invalid"]
    #[inline(always)]
    pub fn mb08(&self) -> MB08_R {
        MB08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask Invalid"]
    #[inline(always)]
    pub fn mb09(&self) -> MB09_R {
        MB09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mask Invalid"]
    #[inline(always)]
    pub fn mb10(&self) -> MB10_R {
        MB10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mask Invalid"]
    #[inline(always)]
    pub fn mb11(&self) -> MB11_R {
        MB11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Mask Invalid"]
    #[inline(always)]
    pub fn mb12(&self) -> MB12_R {
        MB12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Mask Invalid"]
    #[inline(always)]
    pub fn mb13(&self) -> MB13_R {
        MB13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Mask Invalid"]
    #[inline(always)]
    pub fn mb14(&self) -> MB14_R {
        MB14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Mask Invalid"]
    #[inline(always)]
    pub fn mb15(&self) -> MB15_R {
        MB15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Mask Invalid"]
    #[inline(always)]
    pub fn mb16(&self) -> MB16_R {
        MB16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Mask Invalid"]
    #[inline(always)]
    pub fn mb17(&self) -> MB17_R {
        MB17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Mask Invalid"]
    #[inline(always)]
    pub fn mb18(&self) -> MB18_R {
        MB18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Mask Invalid"]
    #[inline(always)]
    pub fn mb19(&self) -> MB19_R {
        MB19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Mask Invalid"]
    #[inline(always)]
    pub fn mb20(&self) -> MB20_R {
        MB20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Mask Invalid"]
    #[inline(always)]
    pub fn mb21(&self) -> MB21_R {
        MB21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Mask Invalid"]
    #[inline(always)]
    pub fn mb22(&self) -> MB22_R {
        MB22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Mask Invalid"]
    #[inline(always)]
    pub fn mb23(&self) -> MB23_R {
        MB23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Mask Invalid"]
    #[inline(always)]
    pub fn mb24(&self) -> MB24_R {
        MB24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Mask Invalid"]
    #[inline(always)]
    pub fn mb25(&self) -> MB25_R {
        MB25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Mask Invalid"]
    #[inline(always)]
    pub fn mb26(&self) -> MB26_R {
        MB26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Mask Invalid"]
    #[inline(always)]
    pub fn mb27(&self) -> MB27_R {
        MB27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Mask Invalid"]
    #[inline(always)]
    pub fn mb28(&self) -> MB28_R {
        MB28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Mask Invalid"]
    #[inline(always)]
    pub fn mb29(&self) -> MB29_R {
        MB29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Mask Invalid"]
    #[inline(always)]
    pub fn mb30(&self) -> MB30_R {
        MB30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Mask Invalid"]
    #[inline(always)]
    pub fn mb31(&self) -> MB31_R {
        MB31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb00(&mut self) -> MB00_W<0> {
        MB00_W::new(self)
    }
    #[doc = "Bit 1 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb01(&mut self) -> MB01_W<1> {
        MB01_W::new(self)
    }
    #[doc = "Bit 2 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb02(&mut self) -> MB02_W<2> {
        MB02_W::new(self)
    }
    #[doc = "Bit 3 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb03(&mut self) -> MB03_W<3> {
        MB03_W::new(self)
    }
    #[doc = "Bit 4 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb04(&mut self) -> MB04_W<4> {
        MB04_W::new(self)
    }
    #[doc = "Bit 5 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb05(&mut self) -> MB05_W<5> {
        MB05_W::new(self)
    }
    #[doc = "Bit 6 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb06(&mut self) -> MB06_W<6> {
        MB06_W::new(self)
    }
    #[doc = "Bit 7 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb07(&mut self) -> MB07_W<7> {
        MB07_W::new(self)
    }
    #[doc = "Bit 8 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb08(&mut self) -> MB08_W<8> {
        MB08_W::new(self)
    }
    #[doc = "Bit 9 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb09(&mut self) -> MB09_W<9> {
        MB09_W::new(self)
    }
    #[doc = "Bit 10 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb10(&mut self) -> MB10_W<10> {
        MB10_W::new(self)
    }
    #[doc = "Bit 11 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb11(&mut self) -> MB11_W<11> {
        MB11_W::new(self)
    }
    #[doc = "Bit 12 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb12(&mut self) -> MB12_W<12> {
        MB12_W::new(self)
    }
    #[doc = "Bit 13 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb13(&mut self) -> MB13_W<13> {
        MB13_W::new(self)
    }
    #[doc = "Bit 14 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb14(&mut self) -> MB14_W<14> {
        MB14_W::new(self)
    }
    #[doc = "Bit 15 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb15(&mut self) -> MB15_W<15> {
        MB15_W::new(self)
    }
    #[doc = "Bit 16 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb16(&mut self) -> MB16_W<16> {
        MB16_W::new(self)
    }
    #[doc = "Bit 17 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb17(&mut self) -> MB17_W<17> {
        MB17_W::new(self)
    }
    #[doc = "Bit 18 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb18(&mut self) -> MB18_W<18> {
        MB18_W::new(self)
    }
    #[doc = "Bit 19 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb19(&mut self) -> MB19_W<19> {
        MB19_W::new(self)
    }
    #[doc = "Bit 20 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb20(&mut self) -> MB20_W<20> {
        MB20_W::new(self)
    }
    #[doc = "Bit 21 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb21(&mut self) -> MB21_W<21> {
        MB21_W::new(self)
    }
    #[doc = "Bit 22 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb22(&mut self) -> MB22_W<22> {
        MB22_W::new(self)
    }
    #[doc = "Bit 23 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb23(&mut self) -> MB23_W<23> {
        MB23_W::new(self)
    }
    #[doc = "Bit 24 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb24(&mut self) -> MB24_W<24> {
        MB24_W::new(self)
    }
    #[doc = "Bit 25 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb25(&mut self) -> MB25_W<25> {
        MB25_W::new(self)
    }
    #[doc = "Bit 26 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb26(&mut self) -> MB26_W<26> {
        MB26_W::new(self)
    }
    #[doc = "Bit 27 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb27(&mut self) -> MB27_W<27> {
        MB27_W::new(self)
    }
    #[doc = "Bit 28 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb28(&mut self) -> MB28_W<28> {
        MB28_W::new(self)
    }
    #[doc = "Bit 29 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb29(&mut self) -> MB29_W<29> {
        MB29_W::new(self)
    }
    #[doc = "Bit 30 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb30(&mut self) -> MB30_W<30> {
        MB30_W::new(self)
    }
    #[doc = "Bit 31 - Mask Invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mb31(&mut self) -> MB31_W<31> {
        MB31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mask Invalid Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mkivlr](index.html) module"]
pub struct MKIVLR_SPEC;
impl crate::RegisterSpec for MKIVLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mkivlr::R](R) reader structure"]
impl crate::Readable for MKIVLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mkivlr::W](W) writer structure"]
impl crate::Writable for MKIVLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MKIVLR to value 0"]
impl crate::Resettable for MKIVLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
