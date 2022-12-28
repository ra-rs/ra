#[doc = "Register `ADCCMPCR%s` reader"]
pub struct R(crate::R<ADCCMPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCCMPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCCMPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCCMPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCCMPCR%s` writer"]
pub struct W(crate::W<ADCCMPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCCMPCR_SPEC>;
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
impl From<crate::W<ADCCMPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCCMPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCMPCND` reader - Composite Compare Match Condition Selection"]
pub type CCMPCND_R = crate::FieldReader<u8, CCMPCND_A>;
#[doc = "Composite Compare Match Condition Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CCMPCND_A {
    #[doc = "0: Logical disjunction (OR) conditions"]
    _00 = 0,
    #[doc = "1: Logical conjunction (AND) conditions"]
    _01 = 1,
    #[doc = "2: Logical exclusive disjunction (EXOR) conditions"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<CCMPCND_A> for u8 {
    #[inline(always)]
    fn from(variant: CCMPCND_A) -> Self {
        variant as _
    }
}
impl CCMPCND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCMPCND_A {
        match self.bits {
            0 => CCMPCND_A::_00,
            1 => CCMPCND_A::_01,
            2 => CCMPCND_A::_10,
            3 => CCMPCND_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CCMPCND_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CCMPCND_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CCMPCND_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CCMPCND_A::_11
    }
}
#[doc = "Field `CCMPCND` writer - Composite Compare Match Condition Selection"]
pub type CCMPCND_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADCCMPCR_SPEC, u8, CCMPCND_A, 2, O>;
impl<'a, const O: u8> CCMPCND_W<'a, O> {
    #[doc = "Logical disjunction (OR) conditions"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CCMPCND_A::_00)
    }
    #[doc = "Logical conjunction (AND) conditions"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CCMPCND_A::_01)
    }
    #[doc = "Logical exclusive disjunction (EXOR) conditions"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CCMPCND_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CCMPCND_A::_11)
    }
}
#[doc = "Field `CCMPTBL0` reader - Composite Compare Match Condition Table Selection"]
pub type CCMPTBL0_R = crate::BitReader<CCMPTBL0_A>;
#[doc = "Composite Compare Match Condition Table Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCMPTBL0_A {
    #[doc = "0: Not use the compare match table m"]
    _0 = 0,
    #[doc = "1: Use the compare match table m"]
    _1 = 1,
}
impl From<CCMPTBL0_A> for bool {
    #[inline(always)]
    fn from(variant: CCMPTBL0_A) -> Self {
        variant as u8 != 0
    }
}
impl CCMPTBL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCMPTBL0_A {
        match self.bits {
            false => CCMPTBL0_A::_0,
            true => CCMPTBL0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCMPTBL0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCMPTBL0_A::_1
    }
}
#[doc = "Field `CCMPTBL0` writer - Composite Compare Match Condition Table Selection"]
pub type CCMPTBL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCMPCR_SPEC, CCMPTBL0_A, O>;
impl<'a, const O: u8> CCMPTBL0_W<'a, O> {
    #[doc = "Not use the compare match table m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCMPTBL0_A::_0)
    }
    #[doc = "Use the compare match table m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCMPTBL0_A::_1)
    }
}
#[doc = "Field `CCMPTBL1` reader - Composite Compare Match Condition Table Selection"]
pub type CCMPTBL1_R = crate::BitReader<CCMPTBL1_A>;
#[doc = "Composite Compare Match Condition Table Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCMPTBL1_A {
    #[doc = "0: Not use the compare match table m"]
    _0 = 0,
    #[doc = "1: Use the compare match table m"]
    _1 = 1,
}
impl From<CCMPTBL1_A> for bool {
    #[inline(always)]
    fn from(variant: CCMPTBL1_A) -> Self {
        variant as u8 != 0
    }
}
impl CCMPTBL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCMPTBL1_A {
        match self.bits {
            false => CCMPTBL1_A::_0,
            true => CCMPTBL1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCMPTBL1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCMPTBL1_A::_1
    }
}
#[doc = "Field `CCMPTBL1` writer - Composite Compare Match Condition Table Selection"]
pub type CCMPTBL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCMPCR_SPEC, CCMPTBL1_A, O>;
impl<'a, const O: u8> CCMPTBL1_W<'a, O> {
    #[doc = "Not use the compare match table m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCMPTBL1_A::_0)
    }
    #[doc = "Use the compare match table m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCMPTBL1_A::_1)
    }
}
#[doc = "Field `CCMPTBL2` reader - Composite Compare Match Condition Table Selection"]
pub type CCMPTBL2_R = crate::BitReader<CCMPTBL2_A>;
#[doc = "Composite Compare Match Condition Table Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCMPTBL2_A {
    #[doc = "0: Not use the compare match table m"]
    _0 = 0,
    #[doc = "1: Use the compare match table m"]
    _1 = 1,
}
impl From<CCMPTBL2_A> for bool {
    #[inline(always)]
    fn from(variant: CCMPTBL2_A) -> Self {
        variant as u8 != 0
    }
}
impl CCMPTBL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCMPTBL2_A {
        match self.bits {
            false => CCMPTBL2_A::_0,
            true => CCMPTBL2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCMPTBL2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCMPTBL2_A::_1
    }
}
#[doc = "Field `CCMPTBL2` writer - Composite Compare Match Condition Table Selection"]
pub type CCMPTBL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCMPCR_SPEC, CCMPTBL2_A, O>;
impl<'a, const O: u8> CCMPTBL2_W<'a, O> {
    #[doc = "Not use the compare match table m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCMPTBL2_A::_0)
    }
    #[doc = "Use the compare match table m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCMPTBL2_A::_1)
    }
}
#[doc = "Field `CCMPTBL3` reader - Composite Compare Match Condition Table Selection"]
pub type CCMPTBL3_R = crate::BitReader<CCMPTBL3_A>;
#[doc = "Composite Compare Match Condition Table Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCMPTBL3_A {
    #[doc = "0: Not use the compare match table m"]
    _0 = 0,
    #[doc = "1: Use the compare match table m"]
    _1 = 1,
}
impl From<CCMPTBL3_A> for bool {
    #[inline(always)]
    fn from(variant: CCMPTBL3_A) -> Self {
        variant as u8 != 0
    }
}
impl CCMPTBL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCMPTBL3_A {
        match self.bits {
            false => CCMPTBL3_A::_0,
            true => CCMPTBL3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCMPTBL3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCMPTBL3_A::_1
    }
}
#[doc = "Field `CCMPTBL3` writer - Composite Compare Match Condition Table Selection"]
pub type CCMPTBL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCMPCR_SPEC, CCMPTBL3_A, O>;
impl<'a, const O: u8> CCMPTBL3_W<'a, O> {
    #[doc = "Not use the compare match table m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCMPTBL3_A::_0)
    }
    #[doc = "Use the compare match table m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCMPTBL3_A::_1)
    }
}
#[doc = "Field `CCMPTBL4` reader - Composite Compare Match Condition Table Selection"]
pub type CCMPTBL4_R = crate::BitReader<CCMPTBL4_A>;
#[doc = "Composite Compare Match Condition Table Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCMPTBL4_A {
    #[doc = "0: Not use the compare match table m"]
    _0 = 0,
    #[doc = "1: Use the compare match table m"]
    _1 = 1,
}
impl From<CCMPTBL4_A> for bool {
    #[inline(always)]
    fn from(variant: CCMPTBL4_A) -> Self {
        variant as u8 != 0
    }
}
impl CCMPTBL4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCMPTBL4_A {
        match self.bits {
            false => CCMPTBL4_A::_0,
            true => CCMPTBL4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCMPTBL4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCMPTBL4_A::_1
    }
}
#[doc = "Field `CCMPTBL4` writer - Composite Compare Match Condition Table Selection"]
pub type CCMPTBL4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCMPCR_SPEC, CCMPTBL4_A, O>;
impl<'a, const O: u8> CCMPTBL4_W<'a, O> {
    #[doc = "Not use the compare match table m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCMPTBL4_A::_0)
    }
    #[doc = "Use the compare match table m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCMPTBL4_A::_1)
    }
}
#[doc = "Field `CCMPTBL5` reader - Composite Compare Match Condition Table Selection"]
pub type CCMPTBL5_R = crate::BitReader<CCMPTBL5_A>;
#[doc = "Composite Compare Match Condition Table Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCMPTBL5_A {
    #[doc = "0: Not use the compare match table m"]
    _0 = 0,
    #[doc = "1: Use the compare match table m"]
    _1 = 1,
}
impl From<CCMPTBL5_A> for bool {
    #[inline(always)]
    fn from(variant: CCMPTBL5_A) -> Self {
        variant as u8 != 0
    }
}
impl CCMPTBL5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCMPTBL5_A {
        match self.bits {
            false => CCMPTBL5_A::_0,
            true => CCMPTBL5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCMPTBL5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCMPTBL5_A::_1
    }
}
#[doc = "Field `CCMPTBL5` writer - Composite Compare Match Condition Table Selection"]
pub type CCMPTBL5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCMPCR_SPEC, CCMPTBL5_A, O>;
impl<'a, const O: u8> CCMPTBL5_W<'a, O> {
    #[doc = "Not use the compare match table m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCMPTBL5_A::_0)
    }
    #[doc = "Use the compare match table m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCMPTBL5_A::_1)
    }
}
#[doc = "Field `CCMPTBL6` reader - Composite Compare Match Condition Table Selection"]
pub type CCMPTBL6_R = crate::BitReader<CCMPTBL6_A>;
#[doc = "Composite Compare Match Condition Table Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCMPTBL6_A {
    #[doc = "0: Not use the compare match table m"]
    _0 = 0,
    #[doc = "1: Use the compare match table m"]
    _1 = 1,
}
impl From<CCMPTBL6_A> for bool {
    #[inline(always)]
    fn from(variant: CCMPTBL6_A) -> Self {
        variant as u8 != 0
    }
}
impl CCMPTBL6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCMPTBL6_A {
        match self.bits {
            false => CCMPTBL6_A::_0,
            true => CCMPTBL6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCMPTBL6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCMPTBL6_A::_1
    }
}
#[doc = "Field `CCMPTBL6` writer - Composite Compare Match Condition Table Selection"]
pub type CCMPTBL6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCMPCR_SPEC, CCMPTBL6_A, O>;
impl<'a, const O: u8> CCMPTBL6_W<'a, O> {
    #[doc = "Not use the compare match table m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCMPTBL6_A::_0)
    }
    #[doc = "Use the compare match table m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCMPTBL6_A::_1)
    }
}
#[doc = "Field `CCMPTBL7` reader - Composite Compare Match Condition Table Selection"]
pub type CCMPTBL7_R = crate::BitReader<CCMPTBL7_A>;
#[doc = "Composite Compare Match Condition Table Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCMPTBL7_A {
    #[doc = "0: Not use the compare match table m"]
    _0 = 0,
    #[doc = "1: Use the compare match table m"]
    _1 = 1,
}
impl From<CCMPTBL7_A> for bool {
    #[inline(always)]
    fn from(variant: CCMPTBL7_A) -> Self {
        variant as u8 != 0
    }
}
impl CCMPTBL7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCMPTBL7_A {
        match self.bits {
            false => CCMPTBL7_A::_0,
            true => CCMPTBL7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCMPTBL7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCMPTBL7_A::_1
    }
}
#[doc = "Field `CCMPTBL7` writer - Composite Compare Match Condition Table Selection"]
pub type CCMPTBL7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCMPCR_SPEC, CCMPTBL7_A, O>;
impl<'a, const O: u8> CCMPTBL7_W<'a, O> {
    #[doc = "Not use the compare match table m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCMPTBL7_A::_0)
    }
    #[doc = "Use the compare match table m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCMPTBL7_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Composite Compare Match Condition Selection"]
    #[inline(always)]
    pub fn ccmpcnd(&self) -> CCMPCND_R {
        CCMPCND_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 16 - Composite Compare Match Condition Table Selection"]
    #[inline(always)]
    pub fn ccmptbl0(&self) -> CCMPTBL0_R {
        CCMPTBL0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Composite Compare Match Condition Table Selection"]
    #[inline(always)]
    pub fn ccmptbl1(&self) -> CCMPTBL1_R {
        CCMPTBL1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Composite Compare Match Condition Table Selection"]
    #[inline(always)]
    pub fn ccmptbl2(&self) -> CCMPTBL2_R {
        CCMPTBL2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Composite Compare Match Condition Table Selection"]
    #[inline(always)]
    pub fn ccmptbl3(&self) -> CCMPTBL3_R {
        CCMPTBL3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Composite Compare Match Condition Table Selection"]
    #[inline(always)]
    pub fn ccmptbl4(&self) -> CCMPTBL4_R {
        CCMPTBL4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Composite Compare Match Condition Table Selection"]
    #[inline(always)]
    pub fn ccmptbl5(&self) -> CCMPTBL5_R {
        CCMPTBL5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Composite Compare Match Condition Table Selection"]
    #[inline(always)]
    pub fn ccmptbl6(&self) -> CCMPTBL6_R {
        CCMPTBL6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Composite Compare Match Condition Table Selection"]
    #[inline(always)]
    pub fn ccmptbl7(&self) -> CCMPTBL7_R {
        CCMPTBL7_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Composite Compare Match Condition Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ccmpcnd(&mut self) -> CCMPCND_W<0> {
        CCMPCND_W::new(self)
    }
    #[doc = "Bit 16 - Composite Compare Match Condition Table Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ccmptbl0(&mut self) -> CCMPTBL0_W<16> {
        CCMPTBL0_W::new(self)
    }
    #[doc = "Bit 17 - Composite Compare Match Condition Table Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ccmptbl1(&mut self) -> CCMPTBL1_W<17> {
        CCMPTBL1_W::new(self)
    }
    #[doc = "Bit 18 - Composite Compare Match Condition Table Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ccmptbl2(&mut self) -> CCMPTBL2_W<18> {
        CCMPTBL2_W::new(self)
    }
    #[doc = "Bit 19 - Composite Compare Match Condition Table Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ccmptbl3(&mut self) -> CCMPTBL3_W<19> {
        CCMPTBL3_W::new(self)
    }
    #[doc = "Bit 20 - Composite Compare Match Condition Table Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ccmptbl4(&mut self) -> CCMPTBL4_W<20> {
        CCMPTBL4_W::new(self)
    }
    #[doc = "Bit 21 - Composite Compare Match Condition Table Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ccmptbl5(&mut self) -> CCMPTBL5_W<21> {
        CCMPTBL5_W::new(self)
    }
    #[doc = "Bit 22 - Composite Compare Match Condition Table Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ccmptbl6(&mut self) -> CCMPTBL6_W<22> {
        CCMPTBL6_W::new(self)
    }
    #[doc = "Bit 23 - Composite Compare Match Condition Table Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ccmptbl7(&mut self) -> CCMPTBL7_W<23> {
        CCMPTBL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Composite Compare Match Configuration Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adccmpcr](index.html) module"]
pub struct ADCCMPCR_SPEC;
impl crate::RegisterSpec for ADCCMPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adccmpcr::R](R) reader structure"]
impl crate::Readable for ADCCMPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adccmpcr::W](W) writer structure"]
impl crate::Writable for ADCCMPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCCMPCR%s to value 0"]
impl crate::Resettable for ADCCMPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
