#[doc = "Register `CGFSAR` reader"]
pub struct R(crate::R<CGFSAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CGFSAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CGFSAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CGFSAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CGFSAR` writer"]
pub struct W(crate::W<CGFSAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CGFSAR_SPEC>;
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
impl From<crate::W<CGFSAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CGFSAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NONSEC00` reader - Non Secure Attribute bit 00"]
pub type NONSEC00_R = crate::BitReader<NONSEC00_A>;
#[doc = "Non Secure Attribute bit 00\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC00_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC00_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC00_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC00_A {
        match self.bits {
            false => NONSEC00_A::_0,
            true => NONSEC00_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC00_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC00_A::_1
    }
}
#[doc = "Field `NONSEC00` writer - Non Secure Attribute bit 00"]
pub type NONSEC00_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGFSAR_SPEC, NONSEC00_A, O>;
impl<'a, const O: u8> NONSEC00_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC00_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC00_A::_1)
    }
}
#[doc = "Field `NONSEC02` reader - Non Secure Attribute bit 02"]
pub type NONSEC02_R = crate::BitReader<NONSEC02_A>;
#[doc = "Non Secure Attribute bit 02\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC02_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC02_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC02_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC02_A {
        match self.bits {
            false => NONSEC02_A::_0,
            true => NONSEC02_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC02_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC02_A::_1
    }
}
#[doc = "Field `NONSEC02` writer - Non Secure Attribute bit 02"]
pub type NONSEC02_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGFSAR_SPEC, NONSEC02_A, O>;
impl<'a, const O: u8> NONSEC02_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC02_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC02_A::_1)
    }
}
#[doc = "Field `NONSEC03` reader - Non Secure Attribute bit 03"]
pub type NONSEC03_R = crate::BitReader<NONSEC03_A>;
#[doc = "Non Secure Attribute bit 03\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC03_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC03_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC03_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC03_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC03_A {
        match self.bits {
            false => NONSEC03_A::_0,
            true => NONSEC03_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC03_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC03_A::_1
    }
}
#[doc = "Field `NONSEC03` writer - Non Secure Attribute bit 03"]
pub type NONSEC03_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGFSAR_SPEC, NONSEC03_A, O>;
impl<'a, const O: u8> NONSEC03_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC03_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC03_A::_1)
    }
}
#[doc = "Field `NONSEC04` reader - Non Secure Attribute bit 04"]
pub type NONSEC04_R = crate::BitReader<NONSEC04_A>;
#[doc = "Non Secure Attribute bit 04\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC04_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC04_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC04_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC04_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC04_A {
        match self.bits {
            false => NONSEC04_A::_0,
            true => NONSEC04_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC04_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC04_A::_1
    }
}
#[doc = "Field `NONSEC04` writer - Non Secure Attribute bit 04"]
pub type NONSEC04_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGFSAR_SPEC, NONSEC04_A, O>;
impl<'a, const O: u8> NONSEC04_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC04_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC04_A::_1)
    }
}
#[doc = "Field `NONSEC05` reader - Non Secure Attribute bit 05"]
pub type NONSEC05_R = crate::BitReader<NONSEC05_A>;
#[doc = "Non Secure Attribute bit 05\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC05_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC05_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC05_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC05_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC05_A {
        match self.bits {
            false => NONSEC05_A::_0,
            true => NONSEC05_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC05_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC05_A::_1
    }
}
#[doc = "Field `NONSEC05` writer - Non Secure Attribute bit 05"]
pub type NONSEC05_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGFSAR_SPEC, NONSEC05_A, O>;
impl<'a, const O: u8> NONSEC05_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC05_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC05_A::_1)
    }
}
#[doc = "Field `NONSEC06` reader - Non Secure Attribute bit 06"]
pub type NONSEC06_R = crate::BitReader<NONSEC06_A>;
#[doc = "Non Secure Attribute bit 06\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC06_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC06_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC06_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC06_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC06_A {
        match self.bits {
            false => NONSEC06_A::_0,
            true => NONSEC06_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC06_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC06_A::_1
    }
}
#[doc = "Field `NONSEC06` writer - Non Secure Attribute bit 06"]
pub type NONSEC06_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGFSAR_SPEC, NONSEC06_A, O>;
impl<'a, const O: u8> NONSEC06_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC06_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC06_A::_1)
    }
}
#[doc = "Field `NONSEC08` reader - Non Secure Attribute bit 08"]
pub type NONSEC08_R = crate::BitReader<NONSEC08_A>;
#[doc = "Non Secure Attribute bit 08\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC08_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC08_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC08_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC08_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC08_A {
        match self.bits {
            false => NONSEC08_A::_0,
            true => NONSEC08_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC08_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC08_A::_1
    }
}
#[doc = "Field `NONSEC08` writer - Non Secure Attribute bit 08"]
pub type NONSEC08_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGFSAR_SPEC, NONSEC08_A, O>;
impl<'a, const O: u8> NONSEC08_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC08_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC08_A::_1)
    }
}
#[doc = "Field `NONSEC09` reader - Non Secure Attribute bit 09"]
pub type NONSEC09_R = crate::BitReader<NONSEC09_A>;
#[doc = "Non Secure Attribute bit 09\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC09_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC09_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC09_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC09_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC09_A {
        match self.bits {
            false => NONSEC09_A::_0,
            true => NONSEC09_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC09_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC09_A::_1
    }
}
#[doc = "Field `NONSEC09` writer - Non Secure Attribute bit 09"]
pub type NONSEC09_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGFSAR_SPEC, NONSEC09_A, O>;
impl<'a, const O: u8> NONSEC09_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC09_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC09_A::_1)
    }
}
#[doc = "Field `NONSEC11` reader - Non Secure Attribute bit 11"]
pub type NONSEC11_R = crate::BitReader<NONSEC11_A>;
#[doc = "Non Secure Attribute bit 11\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC11_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC11_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC11_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC11_A {
        match self.bits {
            false => NONSEC11_A::_0,
            true => NONSEC11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC11_A::_1
    }
}
#[doc = "Field `NONSEC11` writer - Non Secure Attribute bit 11"]
pub type NONSEC11_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGFSAR_SPEC, NONSEC11_A, O>;
impl<'a, const O: u8> NONSEC11_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC11_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC11_A::_1)
    }
}
#[doc = "Field `NONSEC17` reader - Non Secure Attribute bit 17"]
pub type NONSEC17_R = crate::BitReader<NONSEC17_A>;
#[doc = "Non Secure Attribute bit 17\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC17_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC17_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC17_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC17_A {
        match self.bits {
            false => NONSEC17_A::_0,
            true => NONSEC17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC17_A::_1
    }
}
#[doc = "Field `NONSEC17` writer - Non Secure Attribute bit 17"]
pub type NONSEC17_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGFSAR_SPEC, NONSEC17_A, O>;
impl<'a, const O: u8> NONSEC17_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC17_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC17_A::_1)
    }
}
#[doc = "Field `NONSEC18` reader - Non Secure Attribute bit 18"]
pub type NONSEC18_R = crate::BitReader<NONSEC18_A>;
#[doc = "Non Secure Attribute bit 18\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC18_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC18_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC18_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC18_A {
        match self.bits {
            false => NONSEC18_A::_0,
            true => NONSEC18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC18_A::_1
    }
}
#[doc = "Field `NONSEC18` writer - Non Secure Attribute bit 18"]
pub type NONSEC18_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGFSAR_SPEC, NONSEC18_A, O>;
impl<'a, const O: u8> NONSEC18_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC18_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC18_A::_1)
    }
}
#[doc = "Field `NONSEC19` reader - Non Secure Attribute bit 19"]
pub type NONSEC19_R = crate::BitReader<NONSEC19_A>;
#[doc = "Non Secure Attribute bit 19\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC19_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC19_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC19_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC19_A {
        match self.bits {
            false => NONSEC19_A::_0,
            true => NONSEC19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC19_A::_1
    }
}
#[doc = "Field `NONSEC19` writer - Non Secure Attribute bit 19"]
pub type NONSEC19_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGFSAR_SPEC, NONSEC19_A, O>;
impl<'a, const O: u8> NONSEC19_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC19_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC19_A::_1)
    }
}
#[doc = "Field `NONSEC20` reader - Non Secure Attribute bit 20"]
pub type NONSEC20_R = crate::BitReader<NONSEC20_A>;
#[doc = "Non Secure Attribute bit 20\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC20_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC20_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC20_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC20_A {
        match self.bits {
            false => NONSEC20_A::_0,
            true => NONSEC20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC20_A::_1
    }
}
#[doc = "Field `NONSEC20` writer - Non Secure Attribute bit 20"]
pub type NONSEC20_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGFSAR_SPEC, NONSEC20_A, O>;
impl<'a, const O: u8> NONSEC20_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC20_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC20_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Non Secure Attribute bit 00"]
    #[inline(always)]
    pub fn nonsec00(&self) -> NONSEC00_R {
        NONSEC00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Non Secure Attribute bit 02"]
    #[inline(always)]
    pub fn nonsec02(&self) -> NONSEC02_R {
        NONSEC02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Non Secure Attribute bit 03"]
    #[inline(always)]
    pub fn nonsec03(&self) -> NONSEC03_R {
        NONSEC03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Non Secure Attribute bit 04"]
    #[inline(always)]
    pub fn nonsec04(&self) -> NONSEC04_R {
        NONSEC04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Non Secure Attribute bit 05"]
    #[inline(always)]
    pub fn nonsec05(&self) -> NONSEC05_R {
        NONSEC05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Non Secure Attribute bit 06"]
    #[inline(always)]
    pub fn nonsec06(&self) -> NONSEC06_R {
        NONSEC06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Non Secure Attribute bit 08"]
    #[inline(always)]
    pub fn nonsec08(&self) -> NONSEC08_R {
        NONSEC08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Non Secure Attribute bit 09"]
    #[inline(always)]
    pub fn nonsec09(&self) -> NONSEC09_R {
        NONSEC09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Non Secure Attribute bit 11"]
    #[inline(always)]
    pub fn nonsec11(&self) -> NONSEC11_R {
        NONSEC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 17 - Non Secure Attribute bit 17"]
    #[inline(always)]
    pub fn nonsec17(&self) -> NONSEC17_R {
        NONSEC17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Non Secure Attribute bit 18"]
    #[inline(always)]
    pub fn nonsec18(&self) -> NONSEC18_R {
        NONSEC18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Non Secure Attribute bit 19"]
    #[inline(always)]
    pub fn nonsec19(&self) -> NONSEC19_R {
        NONSEC19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Non Secure Attribute bit 20"]
    #[inline(always)]
    pub fn nonsec20(&self) -> NONSEC20_R {
        NONSEC20_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non Secure Attribute bit 00"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec00(&mut self) -> NONSEC00_W<0> {
        NONSEC00_W::new(self)
    }
    #[doc = "Bit 2 - Non Secure Attribute bit 02"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec02(&mut self) -> NONSEC02_W<2> {
        NONSEC02_W::new(self)
    }
    #[doc = "Bit 3 - Non Secure Attribute bit 03"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec03(&mut self) -> NONSEC03_W<3> {
        NONSEC03_W::new(self)
    }
    #[doc = "Bit 4 - Non Secure Attribute bit 04"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec04(&mut self) -> NONSEC04_W<4> {
        NONSEC04_W::new(self)
    }
    #[doc = "Bit 5 - Non Secure Attribute bit 05"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec05(&mut self) -> NONSEC05_W<5> {
        NONSEC05_W::new(self)
    }
    #[doc = "Bit 6 - Non Secure Attribute bit 06"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec06(&mut self) -> NONSEC06_W<6> {
        NONSEC06_W::new(self)
    }
    #[doc = "Bit 8 - Non Secure Attribute bit 08"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec08(&mut self) -> NONSEC08_W<8> {
        NONSEC08_W::new(self)
    }
    #[doc = "Bit 9 - Non Secure Attribute bit 09"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec09(&mut self) -> NONSEC09_W<9> {
        NONSEC09_W::new(self)
    }
    #[doc = "Bit 11 - Non Secure Attribute bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec11(&mut self) -> NONSEC11_W<11> {
        NONSEC11_W::new(self)
    }
    #[doc = "Bit 17 - Non Secure Attribute bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec17(&mut self) -> NONSEC17_W<17> {
        NONSEC17_W::new(self)
    }
    #[doc = "Bit 18 - Non Secure Attribute bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec18(&mut self) -> NONSEC18_W<18> {
        NONSEC18_W::new(self)
    }
    #[doc = "Bit 19 - Non Secure Attribute bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec19(&mut self) -> NONSEC19_W<19> {
        NONSEC19_W::new(self)
    }
    #[doc = "Bit 20 - Non Secure Attribute bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec20(&mut self) -> NONSEC20_W<20> {
        NONSEC20_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Generation Function Security Attribute Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgfsar](index.html) module"]
pub struct CGFSAR_SPEC;
impl crate::RegisterSpec for CGFSAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cgfsar::R](R) reader structure"]
impl crate::Readable for CGFSAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cgfsar::W](W) writer structure"]
impl crate::Writable for CGFSAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CGFSAR to value 0xffff_ffff"]
impl crate::Resettable for CGFSAR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
