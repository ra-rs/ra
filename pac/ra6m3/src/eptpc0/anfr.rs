#[doc = "Register `ANFR` reader"]
pub struct R(crate::R<ANFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANFR` writer"]
pub struct W(crate::W<ANFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANFR_SPEC>;
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
impl From<crate::W<ANFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLAG0` reader - leap61This bit is used to set the logical value of the leap61 member of timePropertiesDS."]
pub type FLAG0_R = crate::BitReader<FLAG0_A>;
#[doc = "leap61This bit is used to set the logical value of the leap61 member of timePropertiesDS.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG0_A {
    #[doc = "0: leap61 is set to FALSE."]
    _0 = 0,
    #[doc = "1: leap61 is set to TRUE."]
    _1 = 1,
}
impl From<FLAG0_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG0_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG0_A {
        match self.bits {
            false => FLAG0_A::_0,
            true => FLAG0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG0_A::_1
    }
}
#[doc = "Field `FLAG0` writer - leap61This bit is used to set the logical value of the leap61 member of timePropertiesDS."]
pub type FLAG0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANFR_SPEC, FLAG0_A, O>;
impl<'a, const O: u8> FLAG0_W<'a, O> {
    #[doc = "leap61 is set to FALSE."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLAG0_A::_0)
    }
    #[doc = "leap61 is set to TRUE."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLAG0_A::_1)
    }
}
#[doc = "Field `FLAG1` reader - leap59This bit is used to set the logical value of the leap59 member of timePropertiesDS."]
pub type FLAG1_R = crate::BitReader<FLAG1_A>;
#[doc = "leap59This bit is used to set the logical value of the leap59 member of timePropertiesDS.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG1_A {
    #[doc = "0: leap59 is set to FALSE."]
    _0 = 0,
    #[doc = "1: leap59 is set to TRUE."]
    _1 = 1,
}
impl From<FLAG1_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG1_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG1_A {
        match self.bits {
            false => FLAG1_A::_0,
            true => FLAG1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG1_A::_1
    }
}
#[doc = "Field `FLAG1` writer - leap59This bit is used to set the logical value of the leap59 member of timePropertiesDS."]
pub type FLAG1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANFR_SPEC, FLAG1_A, O>;
impl<'a, const O: u8> FLAG1_W<'a, O> {
    #[doc = "leap59 is set to FALSE."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLAG1_A::_0)
    }
    #[doc = "leap59 is set to TRUE."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLAG1_A::_1)
    }
}
#[doc = "Field `FLAG2` reader - currentUtcOffsetValidThis bit is used to set the logical value of the currentUtcOffsetValid member of timePropertiesDS."]
pub type FLAG2_R = crate::BitReader<FLAG2_A>;
#[doc = "currentUtcOffsetValidThis bit is used to set the logical value of the currentUtcOffsetValid member of timePropertiesDS.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG2_A {
    #[doc = "0: currentUtcOffsetValid is set to FALSE."]
    _0 = 0,
    #[doc = "1: currentUtcOffsetValid is set to TRUE."]
    _1 = 1,
}
impl From<FLAG2_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG2_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG2_A {
        match self.bits {
            false => FLAG2_A::_0,
            true => FLAG2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG2_A::_1
    }
}
#[doc = "Field `FLAG2` writer - currentUtcOffsetValidThis bit is used to set the logical value of the currentUtcOffsetValid member of timePropertiesDS."]
pub type FLAG2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANFR_SPEC, FLAG2_A, O>;
impl<'a, const O: u8> FLAG2_W<'a, O> {
    #[doc = "currentUtcOffsetValid is set to FALSE."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLAG2_A::_0)
    }
    #[doc = "currentUtcOffsetValid is set to TRUE."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLAG2_A::_1)
    }
}
#[doc = "Field `FLAG3` reader - ptpTimescaleThis bit is used to set the logical value of the ptpTimescale member of timePropertiesDS."]
pub type FLAG3_R = crate::BitReader<FLAG3_A>;
#[doc = "ptpTimescaleThis bit is used to set the logical value of the ptpTimescale member of timePropertiesDS.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG3_A {
    #[doc = "0: ptpTimescale is set to FALSE."]
    _0 = 0,
    #[doc = "1: ptpTimescale is set to TRUE."]
    _1 = 1,
}
impl From<FLAG3_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG3_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG3_A {
        match self.bits {
            false => FLAG3_A::_0,
            true => FLAG3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG3_A::_1
    }
}
#[doc = "Field `FLAG3` writer - ptpTimescaleThis bit is used to set the logical value of the ptpTimescale member of timePropertiesDS."]
pub type FLAG3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANFR_SPEC, FLAG3_A, O>;
impl<'a, const O: u8> FLAG3_W<'a, O> {
    #[doc = "ptpTimescale is set to FALSE."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLAG3_A::_0)
    }
    #[doc = "ptpTimescale is set to TRUE."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLAG3_A::_1)
    }
}
#[doc = "Field `FLAG4` reader - timeTraceableThis bit is used to set the logical value of the timeTraceable member of timePropertiesDS."]
pub type FLAG4_R = crate::BitReader<FLAG4_A>;
#[doc = "timeTraceableThis bit is used to set the logical value of the timeTraceable member of timePropertiesDS.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG4_A {
    #[doc = "0: timeTraceable is set to FALSE."]
    _0 = 0,
    #[doc = "1: timeTraceable is set to TRUE."]
    _1 = 1,
}
impl From<FLAG4_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG4_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG4_A {
        match self.bits {
            false => FLAG4_A::_0,
            true => FLAG4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG4_A::_1
    }
}
#[doc = "Field `FLAG4` writer - timeTraceableThis bit is used to set the logical value of the timeTraceable member of timePropertiesDS."]
pub type FLAG4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANFR_SPEC, FLAG4_A, O>;
impl<'a, const O: u8> FLAG4_W<'a, O> {
    #[doc = "timeTraceable is set to FALSE."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLAG4_A::_0)
    }
    #[doc = "timeTraceable is set to TRUE."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLAG4_A::_1)
    }
}
#[doc = "Field `FLAG5` reader - frequencyTraceableThis bit is used to set the logical value of the frequencyTraceable member of timePropertiesDS."]
pub type FLAG5_R = crate::BitReader<FLAG5_A>;
#[doc = "frequencyTraceableThis bit is used to set the logical value of the frequencyTraceable member of timePropertiesDS.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG5_A {
    #[doc = "0: frequencyTraceable is set to FALSE."]
    _0 = 0,
    #[doc = "1: frequencyTraceable is set to TRUE."]
    _1 = 1,
}
impl From<FLAG5_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG5_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG5_A {
        match self.bits {
            false => FLAG5_A::_0,
            true => FLAG5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG5_A::_1
    }
}
#[doc = "Field `FLAG5` writer - frequencyTraceableThis bit is used to set the logical value of the frequencyTraceable member of timePropertiesDS."]
pub type FLAG5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANFR_SPEC, FLAG5_A, O>;
impl<'a, const O: u8> FLAG5_W<'a, O> {
    #[doc = "frequencyTraceable is set to FALSE."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLAG5_A::_0)
    }
    #[doc = "frequencyTraceable is set to TRUE."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLAG5_A::_1)
    }
}
#[doc = "Field `FLAG8` reader - alternateMasterFlag"]
pub type FLAG8_R = crate::BitReader<FLAG8_A>;
#[doc = "alternateMasterFlag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG8_A {
    #[doc = "0: alternateMasterFlag is set to FALSE."]
    _0 = 0,
    #[doc = "1: alternateMasterFlag is set to TRUE."]
    _1 = 1,
}
impl From<FLAG8_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG8_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG8_A {
        match self.bits {
            false => FLAG8_A::_0,
            true => FLAG8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG8_A::_1
    }
}
#[doc = "Field `FLAG8` writer - alternateMasterFlag"]
pub type FLAG8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANFR_SPEC, FLAG8_A, O>;
impl<'a, const O: u8> FLAG8_W<'a, O> {
    #[doc = "alternateMasterFlag is set to FALSE."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLAG8_A::_0)
    }
    #[doc = "alternateMasterFlag is set to TRUE."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLAG8_A::_1)
    }
}
#[doc = "Field `FLAG10` reader - unicastFlag"]
pub type FLAG10_R = crate::BitReader<FLAG10_A>;
#[doc = "unicastFlag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG10_A {
    #[doc = "0: unicastFlag is set to FALSE."]
    _0 = 0,
    #[doc = "1: unicastFlag is set to TRUE."]
    _1 = 1,
}
impl From<FLAG10_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG10_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG10_A {
        match self.bits {
            false => FLAG10_A::_0,
            true => FLAG10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG10_A::_1
    }
}
#[doc = "Field `FLAG10` writer - unicastFlag"]
pub type FLAG10_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANFR_SPEC, FLAG10_A, O>;
impl<'a, const O: u8> FLAG10_W<'a, O> {
    #[doc = "unicastFlag is set to FALSE."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLAG10_A::_0)
    }
    #[doc = "unicastFlag is set to TRUE."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLAG10_A::_1)
    }
}
#[doc = "Field `FLAG13` reader - PTP profile Specific 1"]
pub type FLAG13_R = crate::BitReader<FLAG13_A>;
#[doc = "PTP profile Specific 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG13_A {
    #[doc = "0: PTP profile Specific 1 is set to FALSE."]
    _0 = 0,
    #[doc = "1: PTP profile Specific 1 is set to TRUE."]
    _1 = 1,
}
impl From<FLAG13_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG13_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG13_A {
        match self.bits {
            false => FLAG13_A::_0,
            true => FLAG13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG13_A::_1
    }
}
#[doc = "Field `FLAG13` writer - PTP profile Specific 1"]
pub type FLAG13_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANFR_SPEC, FLAG13_A, O>;
impl<'a, const O: u8> FLAG13_W<'a, O> {
    #[doc = "PTP profile Specific 1 is set to FALSE."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLAG13_A::_0)
    }
    #[doc = "PTP profile Specific 1 is set to TRUE."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLAG13_A::_1)
    }
}
#[doc = "Field `FLAG14` reader - PTP profile Specific 2"]
pub type FLAG14_R = crate::BitReader<FLAG14_A>;
#[doc = "PTP profile Specific 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG14_A {
    #[doc = "0: PTP profile Specific 2 is set to FALSE."]
    _0 = 0,
    #[doc = "1: PTP profile Specific 2 is set to TRUE."]
    _1 = 1,
}
impl From<FLAG14_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG14_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG14_A {
        match self.bits {
            false => FLAG14_A::_0,
            true => FLAG14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG14_A::_1
    }
}
#[doc = "Field `FLAG14` writer - PTP profile Specific 2"]
pub type FLAG14_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANFR_SPEC, FLAG14_A, O>;
impl<'a, const O: u8> FLAG14_W<'a, O> {
    #[doc = "PTP profile Specific 2 is set to FALSE."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLAG14_A::_0)
    }
    #[doc = "PTP profile Specific 2 is set to TRUE."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLAG14_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - leap61This bit is used to set the logical value of the leap61 member of timePropertiesDS."]
    #[inline(always)]
    pub fn flag0(&self) -> FLAG0_R {
        FLAG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - leap59This bit is used to set the logical value of the leap59 member of timePropertiesDS."]
    #[inline(always)]
    pub fn flag1(&self) -> FLAG1_R {
        FLAG1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - currentUtcOffsetValidThis bit is used to set the logical value of the currentUtcOffsetValid member of timePropertiesDS."]
    #[inline(always)]
    pub fn flag2(&self) -> FLAG2_R {
        FLAG2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ptpTimescaleThis bit is used to set the logical value of the ptpTimescale member of timePropertiesDS."]
    #[inline(always)]
    pub fn flag3(&self) -> FLAG3_R {
        FLAG3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - timeTraceableThis bit is used to set the logical value of the timeTraceable member of timePropertiesDS."]
    #[inline(always)]
    pub fn flag4(&self) -> FLAG4_R {
        FLAG4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - frequencyTraceableThis bit is used to set the logical value of the frequencyTraceable member of timePropertiesDS."]
    #[inline(always)]
    pub fn flag5(&self) -> FLAG5_R {
        FLAG5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - alternateMasterFlag"]
    #[inline(always)]
    pub fn flag8(&self) -> FLAG8_R {
        FLAG8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - unicastFlag"]
    #[inline(always)]
    pub fn flag10(&self) -> FLAG10_R {
        FLAG10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - PTP profile Specific 1"]
    #[inline(always)]
    pub fn flag13(&self) -> FLAG13_R {
        FLAG13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PTP profile Specific 2"]
    #[inline(always)]
    pub fn flag14(&self) -> FLAG14_R {
        FLAG14_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - leap61This bit is used to set the logical value of the leap61 member of timePropertiesDS."]
    #[inline(always)]
    #[must_use]
    pub fn flag0(&mut self) -> FLAG0_W<0> {
        FLAG0_W::new(self)
    }
    #[doc = "Bit 1 - leap59This bit is used to set the logical value of the leap59 member of timePropertiesDS."]
    #[inline(always)]
    #[must_use]
    pub fn flag1(&mut self) -> FLAG1_W<1> {
        FLAG1_W::new(self)
    }
    #[doc = "Bit 2 - currentUtcOffsetValidThis bit is used to set the logical value of the currentUtcOffsetValid member of timePropertiesDS."]
    #[inline(always)]
    #[must_use]
    pub fn flag2(&mut self) -> FLAG2_W<2> {
        FLAG2_W::new(self)
    }
    #[doc = "Bit 3 - ptpTimescaleThis bit is used to set the logical value of the ptpTimescale member of timePropertiesDS."]
    #[inline(always)]
    #[must_use]
    pub fn flag3(&mut self) -> FLAG3_W<3> {
        FLAG3_W::new(self)
    }
    #[doc = "Bit 4 - timeTraceableThis bit is used to set the logical value of the timeTraceable member of timePropertiesDS."]
    #[inline(always)]
    #[must_use]
    pub fn flag4(&mut self) -> FLAG4_W<4> {
        FLAG4_W::new(self)
    }
    #[doc = "Bit 5 - frequencyTraceableThis bit is used to set the logical value of the frequencyTraceable member of timePropertiesDS."]
    #[inline(always)]
    #[must_use]
    pub fn flag5(&mut self) -> FLAG5_W<5> {
        FLAG5_W::new(self)
    }
    #[doc = "Bit 8 - alternateMasterFlag"]
    #[inline(always)]
    #[must_use]
    pub fn flag8(&mut self) -> FLAG8_W<8> {
        FLAG8_W::new(self)
    }
    #[doc = "Bit 10 - unicastFlag"]
    #[inline(always)]
    #[must_use]
    pub fn flag10(&mut self) -> FLAG10_W<10> {
        FLAG10_W::new(self)
    }
    #[doc = "Bit 13 - PTP profile Specific 1"]
    #[inline(always)]
    #[must_use]
    pub fn flag13(&mut self) -> FLAG13_W<13> {
        FLAG13_W::new(self)
    }
    #[doc = "Bit 14 - PTP profile Specific 2"]
    #[inline(always)]
    #[must_use]
    pub fn flag14(&mut self) -> FLAG14_W<14> {
        FLAG14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Announce Message Flag Field Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [anfr](index.html) module"]
pub struct ANFR_SPEC;
impl crate::RegisterSpec for ANFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [anfr::R](R) reader structure"]
impl crate::Readable for ANFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [anfr::W](W) writer structure"]
impl crate::Writable for ANFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ANFR to value 0"]
impl crate::Resettable for ANFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
