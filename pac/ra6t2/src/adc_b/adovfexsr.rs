#[doc = "Register `ADOVFEXSR` reader"]
pub struct R(crate::R<ADOVFEXSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADOVFEXSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADOVFEXSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADOVFEXSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OVFEXF0` reader - Self-diagnosis Channel: Overflow Flag"]
pub type OVFEXF0_R = crate::BitReader<OVFEXF0_A>;
#[doc = "Self-diagnosis Channel: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFEXF0_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OVFEXF0_A> for bool {
    #[inline(always)]
    fn from(variant: OVFEXF0_A) -> Self {
        variant as u8 != 0
    }
}
impl OVFEXF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVFEXF0_A {
        match self.bits {
            false => OVFEXF0_A::_0,
            true => OVFEXF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVFEXF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVFEXF0_A::_1
    }
}
#[doc = "Field `OVFEXF1` reader - Temperature Sensor Channel: Overflow Flag"]
pub type OVFEXF1_R = crate::BitReader<OVFEXF1_A>;
#[doc = "Temperature Sensor Channel: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFEXF1_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OVFEXF1_A> for bool {
    #[inline(always)]
    fn from(variant: OVFEXF1_A) -> Self {
        variant as u8 != 0
    }
}
impl OVFEXF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVFEXF1_A {
        match self.bits {
            false => OVFEXF1_A::_0,
            true => OVFEXF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVFEXF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVFEXF1_A::_1
    }
}
#[doc = "Field `OVFEXF2` reader - Internal Reference Voltage Channel: Overflow Flag"]
pub type OVFEXF2_R = crate::BitReader<OVFEXF2_A>;
#[doc = "Internal Reference Voltage Channel: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFEXF2_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OVFEXF2_A> for bool {
    #[inline(always)]
    fn from(variant: OVFEXF2_A) -> Self {
        variant as u8 != 0
    }
}
impl OVFEXF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVFEXF2_A {
        match self.bits {
            false => OVFEXF2_A::_0,
            true => OVFEXF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVFEXF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVFEXF2_A::_1
    }
}
#[doc = "Field `OVFEXF5` reader - D/A Converter 0 Channel: Overflow Flag"]
pub type OVFEXF5_R = crate::BitReader<OVFEXF5_A>;
#[doc = "D/A Converter 0 Channel: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFEXF5_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OVFEXF5_A> for bool {
    #[inline(always)]
    fn from(variant: OVFEXF5_A) -> Self {
        variant as u8 != 0
    }
}
impl OVFEXF5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVFEXF5_A {
        match self.bits {
            false => OVFEXF5_A::_0,
            true => OVFEXF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVFEXF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVFEXF5_A::_1
    }
}
#[doc = "Field `OVFEXF6` reader - D/A Converter 1 Channel: Overflow Flag"]
pub type OVFEXF6_R = crate::BitReader<OVFEXF6_A>;
#[doc = "D/A Converter 1 Channel: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFEXF6_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OVFEXF6_A> for bool {
    #[inline(always)]
    fn from(variant: OVFEXF6_A) -> Self {
        variant as u8 != 0
    }
}
impl OVFEXF6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVFEXF6_A {
        match self.bits {
            false => OVFEXF6_A::_0,
            true => OVFEXF6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVFEXF6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVFEXF6_A::_1
    }
}
#[doc = "Field `OVFEXF7` reader - D/A Converter 2 Channel: Overflow Flag"]
pub type OVFEXF7_R = crate::BitReader<OVFEXF7_A>;
#[doc = "D/A Converter 2 Channel: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFEXF7_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OVFEXF7_A> for bool {
    #[inline(always)]
    fn from(variant: OVFEXF7_A) -> Self {
        variant as u8 != 0
    }
}
impl OVFEXF7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVFEXF7_A {
        match self.bits {
            false => OVFEXF7_A::_0,
            true => OVFEXF7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVFEXF7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVFEXF7_A::_1
    }
}
#[doc = "Field `OVFEXF8` reader - D/A Converter 3 Channel: Overflow Flag"]
pub type OVFEXF8_R = crate::BitReader<OVFEXF8_A>;
#[doc = "D/A Converter 3 Channel: Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFEXF8_A {
    #[doc = "0: Overflow is not detected"]
    _0 = 0,
    #[doc = "1: Overflow is detected"]
    _1 = 1,
}
impl From<OVFEXF8_A> for bool {
    #[inline(always)]
    fn from(variant: OVFEXF8_A) -> Self {
        variant as u8 != 0
    }
}
impl OVFEXF8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVFEXF8_A {
        match self.bits {
            false => OVFEXF8_A::_0,
            true => OVFEXF8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVFEXF8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVFEXF8_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Self-diagnosis Channel: Overflow Flag"]
    #[inline(always)]
    pub fn ovfexf0(&self) -> OVFEXF0_R {
        OVFEXF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Temperature Sensor Channel: Overflow Flag"]
    #[inline(always)]
    pub fn ovfexf1(&self) -> OVFEXF1_R {
        OVFEXF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Internal Reference Voltage Channel: Overflow Flag"]
    #[inline(always)]
    pub fn ovfexf2(&self) -> OVFEXF2_R {
        OVFEXF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - D/A Converter 0 Channel: Overflow Flag"]
    #[inline(always)]
    pub fn ovfexf5(&self) -> OVFEXF5_R {
        OVFEXF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - D/A Converter 1 Channel: Overflow Flag"]
    #[inline(always)]
    pub fn ovfexf6(&self) -> OVFEXF6_R {
        OVFEXF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - D/A Converter 2 Channel: Overflow Flag"]
    #[inline(always)]
    pub fn ovfexf7(&self) -> OVFEXF7_R {
        OVFEXF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - D/A Converter 3 Channel: Overflow Flag"]
    #[inline(always)]
    pub fn ovfexf8(&self) -> OVFEXF8_R {
        OVFEXF8_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Extended Analog A/D Conversion Overflow Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adovfexsr](index.html) module"]
pub struct ADOVFEXSR_SPEC;
impl crate::RegisterSpec for ADOVFEXSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adovfexsr::R](R) reader structure"]
impl crate::Readable for ADOVFEXSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADOVFEXSR to value 0"]
impl crate::Resettable for ADOVFEXSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
