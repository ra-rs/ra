#[doc = "Register `MSTPCRD` reader"]
pub struct R(crate::R<MSTPCRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSTPCRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSTPCRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSTPCRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSTPCRD` writer"]
pub struct W(crate::W<MSTPCRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSTPCRD_SPEC>;
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
impl From<crate::W<MSTPCRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSTPCRD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSTPD2` reader - AGT1 Module Stop Note: AGT1 is in the module stop state when the count source is either of PCLKB, PCLKB/2 or PCLKB/8. In case the count source is sub-clock or LOCO, this bit should be set to 1 except when accessing the registers of AGT1."]
pub type MSTPD2_R = crate::BitReader<MSTPD2_A>;
#[doc = "AGT1 Module Stop Note: AGT1 is in the module stop state when the count source is either of PCLKB, PCLKB/2 or PCLKB/8. In case the count source is sub-clock or LOCO, this bit should be set to 1 except when accessing the registers of AGT1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD2_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPD2_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD2_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPD2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPD2_A {
        match self.bits {
            false => MSTPD2_A::_0,
            true => MSTPD2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD2_A::_1
    }
}
#[doc = "Field `MSTPD2` writer - AGT1 Module Stop Note: AGT1 is in the module stop state when the count source is either of PCLKB, PCLKB/2 or PCLKB/8. In case the count source is sub-clock or LOCO, this bit should be set to 1 except when accessing the registers of AGT1."]
pub type MSTPD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRD_SPEC, MSTPD2_A, O>;
impl<'a, const O: u8> MSTPD2_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPD2_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPD2_A::_1)
    }
}
#[doc = "Field `MSTPD3` reader - AGT0 Module Stop Note: AGT0 is in the module stop state when the count source is either of PCLKB, PCLKB/2 or PCLKB/8. In case the count source is sub-clock or LOCO, this bit should be set to 1 except when accessing the registers of AGT0."]
pub type MSTPD3_R = crate::BitReader<MSTPD3_A>;
#[doc = "AGT0 Module Stop Note: AGT0 is in the module stop state when the count source is either of PCLKB, PCLKB/2 or PCLKB/8. In case the count source is sub-clock or LOCO, this bit should be set to 1 except when accessing the registers of AGT0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD3_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPD3_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD3_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPD3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPD3_A {
        match self.bits {
            false => MSTPD3_A::_0,
            true => MSTPD3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD3_A::_1
    }
}
#[doc = "Field `MSTPD3` writer - AGT0 Module Stop Note: AGT0 is in the module stop state when the count source is either of PCLKB, PCLKB/2 or PCLKB/8. In case the count source is sub-clock or LOCO, this bit should be set to 1 except when accessing the registers of AGT0."]
pub type MSTPD3_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRD_SPEC, MSTPD3_A, O>;
impl<'a, const O: u8> MSTPD3_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPD3_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPD3_A::_1)
    }
}
#[doc = "Field `MSTPD5` reader - General PWM Timer 32EH0 to 32EH3 and 32E4 to 32E7 and PWM Delay Generation Circuit Module Stop"]
pub type MSTPD5_R = crate::BitReader<MSTPD5_A>;
#[doc = "General PWM Timer 32EH0 to 32EH3 and 32E4 to 32E7 and PWM Delay Generation Circuit Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD5_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPD5_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD5_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPD5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPD5_A {
        match self.bits {
            false => MSTPD5_A::_0,
            true => MSTPD5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD5_A::_1
    }
}
#[doc = "Field `MSTPD5` writer - General PWM Timer 32EH0 to 32EH3 and 32E4 to 32E7 and PWM Delay Generation Circuit Module Stop"]
pub type MSTPD5_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRD_SPEC, MSTPD5_A, O>;
impl<'a, const O: u8> MSTPD5_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPD5_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPD5_A::_1)
    }
}
#[doc = "Field `MSTPD6` reader - General PWM Timer 32_8 to 32_12 Module Stop"]
pub type MSTPD6_R = crate::BitReader<MSTPD6_A>;
#[doc = "General PWM Timer 32_8 to 32_12 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD6_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPD6_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD6_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPD6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPD6_A {
        match self.bits {
            false => MSTPD6_A::_0,
            true => MSTPD6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD6_A::_1
    }
}
#[doc = "Field `MSTPD6` writer - General PWM Timer 32_8 to 32_12 Module Stop"]
pub type MSTPD6_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRD_SPEC, MSTPD6_A, O>;
impl<'a, const O: u8> MSTPD6_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPD6_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPD6_A::_1)
    }
}
#[doc = "Field `MSTPD14` reader - Port Output Enable for GPT Module Stop"]
pub type MSTPD14_R = crate::BitReader<MSTPD14_A>;
#[doc = "Port Output Enable for GPT Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD14_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPD14_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD14_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPD14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPD14_A {
        match self.bits {
            false => MSTPD14_A::_0,
            true => MSTPD14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD14_A::_1
    }
}
#[doc = "Field `MSTPD14` writer - Port Output Enable for GPT Module Stop"]
pub type MSTPD14_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRD_SPEC, MSTPD14_A, O>;
impl<'a, const O: u8> MSTPD14_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPD14_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPD14_A::_1)
    }
}
#[doc = "Field `MSTPD15` reader - 12-Bit A/D Converter 1 Module Stop"]
pub type MSTPD15_R = crate::BitReader<MSTPD15_A>;
#[doc = "12-Bit A/D Converter 1 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD15_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPD15_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD15_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPD15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPD15_A {
        match self.bits {
            false => MSTPD15_A::_0,
            true => MSTPD15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD15_A::_1
    }
}
#[doc = "Field `MSTPD15` writer - 12-Bit A/D Converter 1 Module Stop"]
pub type MSTPD15_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRD_SPEC, MSTPD15_A, O>;
impl<'a, const O: u8> MSTPD15_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPD15_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPD15_A::_1)
    }
}
#[doc = "Field `MSTPD16` reader - 12-Bit A/D Converter 0 Module Stop"]
pub type MSTPD16_R = crate::BitReader<MSTPD16_A>;
#[doc = "12-Bit A/D Converter 0 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD16_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPD16_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD16_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPD16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPD16_A {
        match self.bits {
            false => MSTPD16_A::_0,
            true => MSTPD16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD16_A::_1
    }
}
#[doc = "Field `MSTPD16` writer - 12-Bit A/D Converter 0 Module Stop"]
pub type MSTPD16_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRD_SPEC, MSTPD16_A, O>;
impl<'a, const O: u8> MSTPD16_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPD16_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPD16_A::_1)
    }
}
#[doc = "Field `MSTPD20` reader - 12-Bit D/A Converter Module Stop"]
pub type MSTPD20_R = crate::BitReader<MSTPD20_A>;
#[doc = "12-Bit D/A Converter Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD20_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPD20_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD20_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPD20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPD20_A {
        match self.bits {
            false => MSTPD20_A::_0,
            true => MSTPD20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD20_A::_1
    }
}
#[doc = "Field `MSTPD20` writer - 12-Bit D/A Converter Module Stop"]
pub type MSTPD20_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRD_SPEC, MSTPD20_A, O>;
impl<'a, const O: u8> MSTPD20_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPD20_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPD20_A::_1)
    }
}
#[doc = "Field `MSTPD22` reader - Temperature Sensor Module Stop"]
pub type MSTPD22_R = crate::BitReader<MSTPD22_A>;
#[doc = "Temperature Sensor Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD22_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPD22_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD22_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPD22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPD22_A {
        match self.bits {
            false => MSTPD22_A::_0,
            true => MSTPD22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD22_A::_1
    }
}
#[doc = "Field `MSTPD22` writer - Temperature Sensor Module Stop"]
pub type MSTPD22_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRD_SPEC, MSTPD22_A, O>;
impl<'a, const O: u8> MSTPD22_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPD22_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPD22_A::_1)
    }
}
#[doc = "Field `MSTPD23` reader - High-Speed Analog Comparator 5 Module Stop"]
pub type MSTPD23_R = crate::BitReader<MSTPD23_A>;
#[doc = "High-Speed Analog Comparator 5 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD23_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPD23_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD23_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPD23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPD23_A {
        match self.bits {
            false => MSTPD23_A::_0,
            true => MSTPD23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD23_A::_1
    }
}
#[doc = "Field `MSTPD23` writer - High-Speed Analog Comparator 5 Module Stop"]
pub type MSTPD23_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRD_SPEC, MSTPD23_A, O>;
impl<'a, const O: u8> MSTPD23_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPD23_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPD23_A::_1)
    }
}
#[doc = "Field `MSTPD24` reader - High-Speed Analog Comparator 4 Module Stop"]
pub type MSTPD24_R = crate::BitReader<MSTPD24_A>;
#[doc = "High-Speed Analog Comparator 4 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD24_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPD24_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD24_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPD24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPD24_A {
        match self.bits {
            false => MSTPD24_A::_0,
            true => MSTPD24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD24_A::_1
    }
}
#[doc = "Field `MSTPD24` writer - High-Speed Analog Comparator 4 Module Stop"]
pub type MSTPD24_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRD_SPEC, MSTPD24_A, O>;
impl<'a, const O: u8> MSTPD24_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPD24_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPD24_A::_1)
    }
}
#[doc = "Field `MSTPD25` reader - High-Speed Analog Comparator 3 Module Stop"]
pub type MSTPD25_R = crate::BitReader<MSTPD25_A>;
#[doc = "High-Speed Analog Comparator 3 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD25_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPD25_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD25_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPD25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPD25_A {
        match self.bits {
            false => MSTPD25_A::_0,
            true => MSTPD25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD25_A::_1
    }
}
#[doc = "Field `MSTPD25` writer - High-Speed Analog Comparator 3 Module Stop"]
pub type MSTPD25_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRD_SPEC, MSTPD25_A, O>;
impl<'a, const O: u8> MSTPD25_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPD25_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPD25_A::_1)
    }
}
#[doc = "Field `MSTPD26` reader - High-Speed Analog Comparator 2 Module Stop"]
pub type MSTPD26_R = crate::BitReader<MSTPD26_A>;
#[doc = "High-Speed Analog Comparator 2 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD26_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPD26_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD26_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPD26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPD26_A {
        match self.bits {
            false => MSTPD26_A::_0,
            true => MSTPD26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD26_A::_1
    }
}
#[doc = "Field `MSTPD26` writer - High-Speed Analog Comparator 2 Module Stop"]
pub type MSTPD26_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRD_SPEC, MSTPD26_A, O>;
impl<'a, const O: u8> MSTPD26_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPD26_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPD26_A::_1)
    }
}
#[doc = "Field `MSTPD27` reader - High-Speed Analog Comparator 1 Module Stop"]
pub type MSTPD27_R = crate::BitReader<MSTPD27_A>;
#[doc = "High-Speed Analog Comparator 1 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD27_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPD27_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD27_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPD27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPD27_A {
        match self.bits {
            false => MSTPD27_A::_0,
            true => MSTPD27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD27_A::_1
    }
}
#[doc = "Field `MSTPD27` writer - High-Speed Analog Comparator 1 Module Stop"]
pub type MSTPD27_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRD_SPEC, MSTPD27_A, O>;
impl<'a, const O: u8> MSTPD27_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPD27_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPD27_A::_1)
    }
}
#[doc = "Field `MSTPD28` reader - High-Speed Analog Comparator 0 Module Stop"]
pub type MSTPD28_R = crate::BitReader<MSTPD28_A>;
#[doc = "High-Speed Analog Comparator 0 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD28_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPD28_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD28_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPD28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPD28_A {
        match self.bits {
            false => MSTPD28_A::_0,
            true => MSTPD28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD28_A::_1
    }
}
#[doc = "Field `MSTPD28` writer - High-Speed Analog Comparator 0 Module Stop"]
pub type MSTPD28_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRD_SPEC, MSTPD28_A, O>;
impl<'a, const O: u8> MSTPD28_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPD28_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPD28_A::_1)
    }
}
impl R {
    #[doc = "Bit 2 - AGT1 Module Stop Note: AGT1 is in the module stop state when the count source is either of PCLKB, PCLKB/2 or PCLKB/8. In case the count source is sub-clock or LOCO, this bit should be set to 1 except when accessing the registers of AGT1."]
    #[inline(always)]
    pub fn mstpd2(&self) -> MSTPD2_R {
        MSTPD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AGT0 Module Stop Note: AGT0 is in the module stop state when the count source is either of PCLKB, PCLKB/2 or PCLKB/8. In case the count source is sub-clock or LOCO, this bit should be set to 1 except when accessing the registers of AGT0."]
    #[inline(always)]
    pub fn mstpd3(&self) -> MSTPD3_R {
        MSTPD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - General PWM Timer 32EH0 to 32EH3 and 32E4 to 32E7 and PWM Delay Generation Circuit Module Stop"]
    #[inline(always)]
    pub fn mstpd5(&self) -> MSTPD5_R {
        MSTPD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - General PWM Timer 32_8 to 32_12 Module Stop"]
    #[inline(always)]
    pub fn mstpd6(&self) -> MSTPD6_R {
        MSTPD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 14 - Port Output Enable for GPT Module Stop"]
    #[inline(always)]
    pub fn mstpd14(&self) -> MSTPD14_R {
        MSTPD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 12-Bit A/D Converter 1 Module Stop"]
    #[inline(always)]
    pub fn mstpd15(&self) -> MSTPD15_R {
        MSTPD15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 12-Bit A/D Converter 0 Module Stop"]
    #[inline(always)]
    pub fn mstpd16(&self) -> MSTPD16_R {
        MSTPD16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 12-Bit D/A Converter Module Stop"]
    #[inline(always)]
    pub fn mstpd20(&self) -> MSTPD20_R {
        MSTPD20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Temperature Sensor Module Stop"]
    #[inline(always)]
    pub fn mstpd22(&self) -> MSTPD22_R {
        MSTPD22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - High-Speed Analog Comparator 5 Module Stop"]
    #[inline(always)]
    pub fn mstpd23(&self) -> MSTPD23_R {
        MSTPD23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - High-Speed Analog Comparator 4 Module Stop"]
    #[inline(always)]
    pub fn mstpd24(&self) -> MSTPD24_R {
        MSTPD24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - High-Speed Analog Comparator 3 Module Stop"]
    #[inline(always)]
    pub fn mstpd25(&self) -> MSTPD25_R {
        MSTPD25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - High-Speed Analog Comparator 2 Module Stop"]
    #[inline(always)]
    pub fn mstpd26(&self) -> MSTPD26_R {
        MSTPD26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - High-Speed Analog Comparator 1 Module Stop"]
    #[inline(always)]
    pub fn mstpd27(&self) -> MSTPD27_R {
        MSTPD27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - High-Speed Analog Comparator 0 Module Stop"]
    #[inline(always)]
    pub fn mstpd28(&self) -> MSTPD28_R {
        MSTPD28_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - AGT1 Module Stop Note: AGT1 is in the module stop state when the count source is either of PCLKB, PCLKB/2 or PCLKB/8. In case the count source is sub-clock or LOCO, this bit should be set to 1 except when accessing the registers of AGT1."]
    #[inline(always)]
    #[must_use]
    pub fn mstpd2(&mut self) -> MSTPD2_W<2> {
        MSTPD2_W::new(self)
    }
    #[doc = "Bit 3 - AGT0 Module Stop Note: AGT0 is in the module stop state when the count source is either of PCLKB, PCLKB/2 or PCLKB/8. In case the count source is sub-clock or LOCO, this bit should be set to 1 except when accessing the registers of AGT0."]
    #[inline(always)]
    #[must_use]
    pub fn mstpd3(&mut self) -> MSTPD3_W<3> {
        MSTPD3_W::new(self)
    }
    #[doc = "Bit 5 - General PWM Timer 32EH0 to 32EH3 and 32E4 to 32E7 and PWM Delay Generation Circuit Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpd5(&mut self) -> MSTPD5_W<5> {
        MSTPD5_W::new(self)
    }
    #[doc = "Bit 6 - General PWM Timer 32_8 to 32_12 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpd6(&mut self) -> MSTPD6_W<6> {
        MSTPD6_W::new(self)
    }
    #[doc = "Bit 14 - Port Output Enable for GPT Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpd14(&mut self) -> MSTPD14_W<14> {
        MSTPD14_W::new(self)
    }
    #[doc = "Bit 15 - 12-Bit A/D Converter 1 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpd15(&mut self) -> MSTPD15_W<15> {
        MSTPD15_W::new(self)
    }
    #[doc = "Bit 16 - 12-Bit A/D Converter 0 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpd16(&mut self) -> MSTPD16_W<16> {
        MSTPD16_W::new(self)
    }
    #[doc = "Bit 20 - 12-Bit D/A Converter Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpd20(&mut self) -> MSTPD20_W<20> {
        MSTPD20_W::new(self)
    }
    #[doc = "Bit 22 - Temperature Sensor Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpd22(&mut self) -> MSTPD22_W<22> {
        MSTPD22_W::new(self)
    }
    #[doc = "Bit 23 - High-Speed Analog Comparator 5 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpd23(&mut self) -> MSTPD23_W<23> {
        MSTPD23_W::new(self)
    }
    #[doc = "Bit 24 - High-Speed Analog Comparator 4 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpd24(&mut self) -> MSTPD24_W<24> {
        MSTPD24_W::new(self)
    }
    #[doc = "Bit 25 - High-Speed Analog Comparator 3 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpd25(&mut self) -> MSTPD25_W<25> {
        MSTPD25_W::new(self)
    }
    #[doc = "Bit 26 - High-Speed Analog Comparator 2 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpd26(&mut self) -> MSTPD26_W<26> {
        MSTPD26_W::new(self)
    }
    #[doc = "Bit 27 - High-Speed Analog Comparator 1 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpd27(&mut self) -> MSTPD27_W<27> {
        MSTPD27_W::new(self)
    }
    #[doc = "Bit 28 - High-Speed Analog Comparator 0 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpd28(&mut self) -> MSTPD28_W<28> {
        MSTPD28_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Stop Control Register D\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mstpcrd](index.html) module"]
pub struct MSTPCRD_SPEC;
impl crate::RegisterSpec for MSTPCRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mstpcrd::R](R) reader structure"]
impl crate::Readable for MSTPCRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mstpcrd::W](W) writer structure"]
impl crate::Writable for MSTPCRD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSTPCRD to value 0xffff_ffff"]
impl crate::Resettable for MSTPCRD_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
