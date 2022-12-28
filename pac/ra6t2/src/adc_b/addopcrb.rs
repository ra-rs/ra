#[doc = "Register `ADDOPCRB%s` reader"]
pub struct R(crate::R<ADDOPCRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDOPCRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDOPCRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDOPCRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDOPCRB%s` writer"]
pub struct W(crate::W<ADDOPCRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDOPCRB_SPEC>;
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
impl From<crate::W<ADDOPCRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDOPCRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AVEMD` reader - Addition/Averaging Mode Selection"]
pub type AVEMD_R = crate::FieldReader<u8, AVEMD_A>;
#[doc = "Addition/Averaging Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AVEMD_A {
    #[doc = "0: Not use Addition/Averaging mode"]
    _00 = 0,
    #[doc = "1: Addition mode"]
    _01 = 1,
    #[doc = "2: Averaging mode"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<AVEMD_A> for u8 {
    #[inline(always)]
    fn from(variant: AVEMD_A) -> Self {
        variant as _
    }
}
impl AVEMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVEMD_A {
        match self.bits {
            0 => AVEMD_A::_00,
            1 => AVEMD_A::_01,
            2 => AVEMD_A::_10,
            3 => AVEMD_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == AVEMD_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == AVEMD_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == AVEMD_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == AVEMD_A::_11
    }
}
#[doc = "Field `AVEMD` writer - Addition/Averaging Mode Selection"]
pub type AVEMD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADDOPCRB_SPEC, u8, AVEMD_A, 2, O>;
impl<'a, const O: u8> AVEMD_W<'a, O> {
    #[doc = "Not use Addition/Averaging mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(AVEMD_A::_00)
    }
    #[doc = "Addition mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(AVEMD_A::_01)
    }
    #[doc = "Averaging mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(AVEMD_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(AVEMD_A::_11)
    }
}
#[doc = "Field `ADC` reader - Addition/Averaging Times Selection"]
pub type ADC_R = crate::FieldReader<u8, ADC_A>;
#[doc = "Addition/Averaging Times Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_A {
    #[doc = "0: 1-time conversion (no addition, same as normal conversion)"]
    _0X0 = 0,
    #[doc = "1: 2-time conversion (1 addition)"]
    _0X1 = 1,
    #[doc = "3: 4-time conversion (3 additions)"]
    _0X3 = 3,
    #[doc = "4: 8-time conversion (7 additions)"]
    _0X4 = 4,
    #[doc = "5: 16-time conversion (15 additions)"]
    _0X5 = 5,
    #[doc = "6: 32-time conversion (31 additions)"]
    _0X6 = 6,
    #[doc = "7: 64-time conversion (63 additions)"]
    _0X7 = 7,
    #[doc = "8: 128-time conversion (127 additions)"]
    _0X8 = 8,
    #[doc = "9: 256-time conversion (255 additions)"]
    _0X9 = 9,
    #[doc = "10: 512-time conversion (511 additions)"]
    _0X_A = 10,
    #[doc = "11: 1024-time conversion (1023 additions)"]
    _0X_B = 11,
}
impl From<ADC_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_A) -> Self {
        variant as _
    }
}
impl ADC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC_A> {
        match self.bits {
            0 => Some(ADC_A::_0X0),
            1 => Some(ADC_A::_0X1),
            3 => Some(ADC_A::_0X3),
            4 => Some(ADC_A::_0X4),
            5 => Some(ADC_A::_0X5),
            6 => Some(ADC_A::_0X6),
            7 => Some(ADC_A::_0X7),
            8 => Some(ADC_A::_0X8),
            9 => Some(ADC_A::_0X9),
            10 => Some(ADC_A::_0X_A),
            11 => Some(ADC_A::_0X_B),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == ADC_A::_0X0
    }
    #[doc = "Checks if the value of the field is `_0X1`"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == ADC_A::_0X1
    }
    #[doc = "Checks if the value of the field is `_0X3`"]
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == ADC_A::_0X3
    }
    #[doc = "Checks if the value of the field is `_0X4`"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == ADC_A::_0X4
    }
    #[doc = "Checks if the value of the field is `_0X5`"]
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == ADC_A::_0X5
    }
    #[doc = "Checks if the value of the field is `_0X6`"]
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == ADC_A::_0X6
    }
    #[doc = "Checks if the value of the field is `_0X7`"]
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == ADC_A::_0X7
    }
    #[doc = "Checks if the value of the field is `_0X8`"]
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == ADC_A::_0X8
    }
    #[doc = "Checks if the value of the field is `_0X9`"]
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == ADC_A::_0X9
    }
    #[doc = "Checks if the value of the field is `_0X_A`"]
    #[inline(always)]
    pub fn is_0x_a(&self) -> bool {
        *self == ADC_A::_0X_A
    }
    #[doc = "Checks if the value of the field is `_0X_B`"]
    #[inline(always)]
    pub fn is_0x_b(&self) -> bool {
        *self == ADC_A::_0X_B
    }
}
#[doc = "Field `ADC` writer - Addition/Averaging Times Selection"]
pub type ADC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDOPCRB_SPEC, u8, ADC_A, 4, O>;
impl<'a, const O: u8> ADC_W<'a, O> {
    #[doc = "1-time conversion (no addition, same as normal conversion)"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(ADC_A::_0X0)
    }
    #[doc = "2-time conversion (1 addition)"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut W {
        self.variant(ADC_A::_0X1)
    }
    #[doc = "4-time conversion (3 additions)"]
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut W {
        self.variant(ADC_A::_0X3)
    }
    #[doc = "8-time conversion (7 additions)"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut W {
        self.variant(ADC_A::_0X4)
    }
    #[doc = "16-time conversion (15 additions)"]
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut W {
        self.variant(ADC_A::_0X5)
    }
    #[doc = "32-time conversion (31 additions)"]
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut W {
        self.variant(ADC_A::_0X6)
    }
    #[doc = "64-time conversion (63 additions)"]
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut W {
        self.variant(ADC_A::_0X7)
    }
    #[doc = "128-time conversion (127 additions)"]
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut W {
        self.variant(ADC_A::_0X8)
    }
    #[doc = "256-time conversion (255 additions)"]
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut W {
        self.variant(ADC_A::_0X9)
    }
    #[doc = "512-time conversion (511 additions)"]
    #[inline(always)]
    pub fn _0x_a(self) -> &'a mut W {
        self.variant(ADC_A::_0X_A)
    }
    #[doc = "1024-time conversion (1023 additions)"]
    #[inline(always)]
    pub fn _0x_b(self) -> &'a mut W {
        self.variant(ADC_A::_0X_B)
    }
}
#[doc = "Field `CMPTBLE0` reader - Compare Match Enable"]
pub type CMPTBLE0_R = crate::BitReader<CMPTBLE0_A>;
#[doc = "Compare Match Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPTBLE0_A {
    #[doc = "0: Disable the compare match with the compare match table m"]
    _0 = 0,
    #[doc = "1: Enable the compare match with the compare match table m"]
    _1 = 1,
}
impl From<CMPTBLE0_A> for bool {
    #[inline(always)]
    fn from(variant: CMPTBLE0_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPTBLE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPTBLE0_A {
        match self.bits {
            false => CMPTBLE0_A::_0,
            true => CMPTBLE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPTBLE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPTBLE0_A::_1
    }
}
#[doc = "Field `CMPTBLE0` writer - Compare Match Enable"]
pub type CMPTBLE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADDOPCRB_SPEC, CMPTBLE0_A, O>;
impl<'a, const O: u8> CMPTBLE0_W<'a, O> {
    #[doc = "Disable the compare match with the compare match table m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPTBLE0_A::_0)
    }
    #[doc = "Enable the compare match with the compare match table m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPTBLE0_A::_1)
    }
}
#[doc = "Field `CMPTBLE1` reader - Compare Match Enable"]
pub type CMPTBLE1_R = crate::BitReader<CMPTBLE1_A>;
#[doc = "Compare Match Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPTBLE1_A {
    #[doc = "0: Disable the compare match with the compare match table m"]
    _0 = 0,
    #[doc = "1: Enable the compare match with the compare match table m"]
    _1 = 1,
}
impl From<CMPTBLE1_A> for bool {
    #[inline(always)]
    fn from(variant: CMPTBLE1_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPTBLE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPTBLE1_A {
        match self.bits {
            false => CMPTBLE1_A::_0,
            true => CMPTBLE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPTBLE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPTBLE1_A::_1
    }
}
#[doc = "Field `CMPTBLE1` writer - Compare Match Enable"]
pub type CMPTBLE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADDOPCRB_SPEC, CMPTBLE1_A, O>;
impl<'a, const O: u8> CMPTBLE1_W<'a, O> {
    #[doc = "Disable the compare match with the compare match table m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPTBLE1_A::_0)
    }
    #[doc = "Enable the compare match with the compare match table m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPTBLE1_A::_1)
    }
}
#[doc = "Field `CMPTBLE2` reader - Compare Match Enable"]
pub type CMPTBLE2_R = crate::BitReader<CMPTBLE2_A>;
#[doc = "Compare Match Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPTBLE2_A {
    #[doc = "0: Disable the compare match with the compare match table m"]
    _0 = 0,
    #[doc = "1: Enable the compare match with the compare match table m"]
    _1 = 1,
}
impl From<CMPTBLE2_A> for bool {
    #[inline(always)]
    fn from(variant: CMPTBLE2_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPTBLE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPTBLE2_A {
        match self.bits {
            false => CMPTBLE2_A::_0,
            true => CMPTBLE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPTBLE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPTBLE2_A::_1
    }
}
#[doc = "Field `CMPTBLE2` writer - Compare Match Enable"]
pub type CMPTBLE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADDOPCRB_SPEC, CMPTBLE2_A, O>;
impl<'a, const O: u8> CMPTBLE2_W<'a, O> {
    #[doc = "Disable the compare match with the compare match table m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPTBLE2_A::_0)
    }
    #[doc = "Enable the compare match with the compare match table m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPTBLE2_A::_1)
    }
}
#[doc = "Field `CMPTBLE3` reader - Compare Match Enable"]
pub type CMPTBLE3_R = crate::BitReader<CMPTBLE3_A>;
#[doc = "Compare Match Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPTBLE3_A {
    #[doc = "0: Disable the compare match with the compare match table m"]
    _0 = 0,
    #[doc = "1: Enable the compare match with the compare match table m"]
    _1 = 1,
}
impl From<CMPTBLE3_A> for bool {
    #[inline(always)]
    fn from(variant: CMPTBLE3_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPTBLE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPTBLE3_A {
        match self.bits {
            false => CMPTBLE3_A::_0,
            true => CMPTBLE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPTBLE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPTBLE3_A::_1
    }
}
#[doc = "Field `CMPTBLE3` writer - Compare Match Enable"]
pub type CMPTBLE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADDOPCRB_SPEC, CMPTBLE3_A, O>;
impl<'a, const O: u8> CMPTBLE3_W<'a, O> {
    #[doc = "Disable the compare match with the compare match table m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPTBLE3_A::_0)
    }
    #[doc = "Enable the compare match with the compare match table m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPTBLE3_A::_1)
    }
}
#[doc = "Field `CMPTBLE4` reader - Compare Match Enable"]
pub type CMPTBLE4_R = crate::BitReader<CMPTBLE4_A>;
#[doc = "Compare Match Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPTBLE4_A {
    #[doc = "0: Disable the compare match with the compare match table m"]
    _0 = 0,
    #[doc = "1: Enable the compare match with the compare match table m"]
    _1 = 1,
}
impl From<CMPTBLE4_A> for bool {
    #[inline(always)]
    fn from(variant: CMPTBLE4_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPTBLE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPTBLE4_A {
        match self.bits {
            false => CMPTBLE4_A::_0,
            true => CMPTBLE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPTBLE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPTBLE4_A::_1
    }
}
#[doc = "Field `CMPTBLE4` writer - Compare Match Enable"]
pub type CMPTBLE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADDOPCRB_SPEC, CMPTBLE4_A, O>;
impl<'a, const O: u8> CMPTBLE4_W<'a, O> {
    #[doc = "Disable the compare match with the compare match table m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPTBLE4_A::_0)
    }
    #[doc = "Enable the compare match with the compare match table m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPTBLE4_A::_1)
    }
}
#[doc = "Field `CMPTBLE5` reader - Compare Match Enable"]
pub type CMPTBLE5_R = crate::BitReader<CMPTBLE5_A>;
#[doc = "Compare Match Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPTBLE5_A {
    #[doc = "0: Disable the compare match with the compare match table m"]
    _0 = 0,
    #[doc = "1: Enable the compare match with the compare match table m"]
    _1 = 1,
}
impl From<CMPTBLE5_A> for bool {
    #[inline(always)]
    fn from(variant: CMPTBLE5_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPTBLE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPTBLE5_A {
        match self.bits {
            false => CMPTBLE5_A::_0,
            true => CMPTBLE5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPTBLE5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPTBLE5_A::_1
    }
}
#[doc = "Field `CMPTBLE5` writer - Compare Match Enable"]
pub type CMPTBLE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADDOPCRB_SPEC, CMPTBLE5_A, O>;
impl<'a, const O: u8> CMPTBLE5_W<'a, O> {
    #[doc = "Disable the compare match with the compare match table m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPTBLE5_A::_0)
    }
    #[doc = "Enable the compare match with the compare match table m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPTBLE5_A::_1)
    }
}
#[doc = "Field `CMPTBLE6` reader - Compare Match Enable"]
pub type CMPTBLE6_R = crate::BitReader<CMPTBLE6_A>;
#[doc = "Compare Match Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPTBLE6_A {
    #[doc = "0: Disable the compare match with the compare match table m"]
    _0 = 0,
    #[doc = "1: Enable the compare match with the compare match table m"]
    _1 = 1,
}
impl From<CMPTBLE6_A> for bool {
    #[inline(always)]
    fn from(variant: CMPTBLE6_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPTBLE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPTBLE6_A {
        match self.bits {
            false => CMPTBLE6_A::_0,
            true => CMPTBLE6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPTBLE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPTBLE6_A::_1
    }
}
#[doc = "Field `CMPTBLE6` writer - Compare Match Enable"]
pub type CMPTBLE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADDOPCRB_SPEC, CMPTBLE6_A, O>;
impl<'a, const O: u8> CMPTBLE6_W<'a, O> {
    #[doc = "Disable the compare match with the compare match table m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPTBLE6_A::_0)
    }
    #[doc = "Enable the compare match with the compare match table m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPTBLE6_A::_1)
    }
}
#[doc = "Field `CMPTBLE7` reader - Compare Match Enable"]
pub type CMPTBLE7_R = crate::BitReader<CMPTBLE7_A>;
#[doc = "Compare Match Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPTBLE7_A {
    #[doc = "0: Disable the compare match with the compare match table m"]
    _0 = 0,
    #[doc = "1: Enable the compare match with the compare match table m"]
    _1 = 1,
}
impl From<CMPTBLE7_A> for bool {
    #[inline(always)]
    fn from(variant: CMPTBLE7_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPTBLE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPTBLE7_A {
        match self.bits {
            false => CMPTBLE7_A::_0,
            true => CMPTBLE7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPTBLE7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPTBLE7_A::_1
    }
}
#[doc = "Field `CMPTBLE7` writer - Compare Match Enable"]
pub type CMPTBLE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADDOPCRB_SPEC, CMPTBLE7_A, O>;
impl<'a, const O: u8> CMPTBLE7_W<'a, O> {
    #[doc = "Disable the compare match with the compare match table m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPTBLE7_A::_0)
    }
    #[doc = "Enable the compare match with the compare match table m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPTBLE7_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Addition/Averaging Mode Selection"]
    #[inline(always)]
    pub fn avemd(&self) -> AVEMD_R {
        AVEMD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - Addition/Averaging Times Selection"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Compare Match Enable"]
    #[inline(always)]
    pub fn cmptble0(&self) -> CMPTBLE0_R {
        CMPTBLE0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Compare Match Enable"]
    #[inline(always)]
    pub fn cmptble1(&self) -> CMPTBLE1_R {
        CMPTBLE1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Compare Match Enable"]
    #[inline(always)]
    pub fn cmptble2(&self) -> CMPTBLE2_R {
        CMPTBLE2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Compare Match Enable"]
    #[inline(always)]
    pub fn cmptble3(&self) -> CMPTBLE3_R {
        CMPTBLE3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Compare Match Enable"]
    #[inline(always)]
    pub fn cmptble4(&self) -> CMPTBLE4_R {
        CMPTBLE4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Compare Match Enable"]
    #[inline(always)]
    pub fn cmptble5(&self) -> CMPTBLE5_R {
        CMPTBLE5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Compare Match Enable"]
    #[inline(always)]
    pub fn cmptble6(&self) -> CMPTBLE6_R {
        CMPTBLE6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Compare Match Enable"]
    #[inline(always)]
    pub fn cmptble7(&self) -> CMPTBLE7_R {
        CMPTBLE7_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Addition/Averaging Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn avemd(&mut self) -> AVEMD_W<0> {
        AVEMD_W::new(self)
    }
    #[doc = "Bits 8:11 - Addition/Averaging Times Selection"]
    #[inline(always)]
    #[must_use]
    pub fn adc(&mut self) -> ADC_W<8> {
        ADC_W::new(self)
    }
    #[doc = "Bit 16 - Compare Match Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmptble0(&mut self) -> CMPTBLE0_W<16> {
        CMPTBLE0_W::new(self)
    }
    #[doc = "Bit 17 - Compare Match Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmptble1(&mut self) -> CMPTBLE1_W<17> {
        CMPTBLE1_W::new(self)
    }
    #[doc = "Bit 18 - Compare Match Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmptble2(&mut self) -> CMPTBLE2_W<18> {
        CMPTBLE2_W::new(self)
    }
    #[doc = "Bit 19 - Compare Match Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmptble3(&mut self) -> CMPTBLE3_W<19> {
        CMPTBLE3_W::new(self)
    }
    #[doc = "Bit 20 - Compare Match Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmptble4(&mut self) -> CMPTBLE4_W<20> {
        CMPTBLE4_W::new(self)
    }
    #[doc = "Bit 21 - Compare Match Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmptble5(&mut self) -> CMPTBLE5_W<21> {
        CMPTBLE5_W::new(self)
    }
    #[doc = "Bit 22 - Compare Match Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmptble6(&mut self) -> CMPTBLE6_W<22> {
        CMPTBLE6_W::new(self)
    }
    #[doc = "Bit 23 - Compare Match Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmptble7(&mut self) -> CMPTBLE7_W<23> {
        CMPTBLE7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Conversion Data Operation Control B Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addopcrb](index.html) module"]
pub struct ADDOPCRB_SPEC;
impl crate::RegisterSpec for ADDOPCRB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addopcrb::R](R) reader structure"]
impl crate::Readable for ADDOPCRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addopcrb::W](W) writer structure"]
impl crate::Writable for ADDOPCRB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDOPCRB%s to value 0"]
impl crate::Resettable for ADDOPCRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
