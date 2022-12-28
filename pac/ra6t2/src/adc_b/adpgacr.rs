#[doc = "Register `ADPGACR%s` reader"]
pub struct R(crate::R<ADPGACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPGACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPGACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPGACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPGACR%s` writer"]
pub struct W(crate::W<ADPGACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPGACR_SPEC>;
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
impl From<crate::W<ADPGACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPGACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PGADEN` reader - PGA Unit n Input Mode Select"]
pub type PGADEN_R = crate::BitReader<PGADEN_A>;
#[doc = "PGA Unit n Input Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGADEN_A {
    #[doc = "0: Single-ended Input mode"]
    _0 = 0,
    #[doc = "1: Pseudo-differential Input mode"]
    _1 = 1,
}
impl From<PGADEN_A> for bool {
    #[inline(always)]
    fn from(variant: PGADEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PGADEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGADEN_A {
        match self.bits {
            false => PGADEN_A::_0,
            true => PGADEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PGADEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PGADEN_A::_1
    }
}
#[doc = "Field `PGADEN` writer - PGA Unit n Input Mode Select"]
pub type PGADEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPGACR_SPEC, PGADEN_A, O>;
impl<'a, const O: u8> PGADEN_W<'a, O> {
    #[doc = "Single-ended Input mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PGADEN_A::_0)
    }
    #[doc = "Pseudo-differential Input mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PGADEN_A::_1)
    }
}
#[doc = "Field `PGASEL1` reader - PGA Unit n Amplifier Output Enable"]
pub type PGASEL1_R = crate::BitReader<PGASEL1_A>;
#[doc = "PGA Unit n Amplifier Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGASEL1_A {
    #[doc = "0: Not output the signal in a path through the PGA"]
    _0 = 0,
    #[doc = "1: Output the signal in a path through the PGA"]
    _1 = 1,
}
impl From<PGASEL1_A> for bool {
    #[inline(always)]
    fn from(variant: PGASEL1_A) -> Self {
        variant as u8 != 0
    }
}
impl PGASEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGASEL1_A {
        match self.bits {
            false => PGASEL1_A::_0,
            true => PGASEL1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PGASEL1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PGASEL1_A::_1
    }
}
#[doc = "Field `PGASEL1` writer - PGA Unit n Amplifier Output Enable"]
pub type PGASEL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPGACR_SPEC, PGASEL1_A, O>;
impl<'a, const O: u8> PGASEL1_W<'a, O> {
    #[doc = "Not output the signal in a path through the PGA"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PGASEL1_A::_0)
    }
    #[doc = "Output the signal in a path through the PGA"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PGASEL1_A::_1)
    }
}
#[doc = "Field `PGAENAMP` reader - PGA Unit n Enable"]
pub type PGAENAMP_R = crate::BitReader<PGAENAMP_A>;
#[doc = "PGA Unit n Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGAENAMP_A {
    #[doc = "0: Disable the PGA"]
    _0 = 0,
    #[doc = "1: Enable the PGA"]
    _1 = 1,
}
impl From<PGAENAMP_A> for bool {
    #[inline(always)]
    fn from(variant: PGAENAMP_A) -> Self {
        variant as u8 != 0
    }
}
impl PGAENAMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGAENAMP_A {
        match self.bits {
            false => PGAENAMP_A::_0,
            true => PGAENAMP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PGAENAMP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PGAENAMP_A::_1
    }
}
#[doc = "Field `PGAENAMP` writer - PGA Unit n Enable"]
pub type PGAENAMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPGACR_SPEC, PGAENAMP_A, O>;
impl<'a, const O: u8> PGAENAMP_W<'a, O> {
    #[doc = "Disable the PGA"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PGAENAMP_A::_0)
    }
    #[doc = "Enable the PGA"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PGAENAMP_A::_1)
    }
}
#[doc = "Field `PGAGEN` reader - PGA Unit n Gain Setting Enable"]
pub type PGAGEN_R = crate::BitReader<PGAGEN_A>;
#[doc = "PGA Unit n Gain Setting Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGAGEN_A {
    #[doc = "0: Disable gain setting"]
    _0 = 0,
    #[doc = "1: Enable gain setting"]
    _1 = 1,
}
impl From<PGAGEN_A> for bool {
    #[inline(always)]
    fn from(variant: PGAGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PGAGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGAGEN_A {
        match self.bits {
            false => PGAGEN_A::_0,
            true => PGAGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PGAGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PGAGEN_A::_1
    }
}
#[doc = "Field `PGAGEN` writer - PGA Unit n Gain Setting Enable"]
pub type PGAGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPGACR_SPEC, PGAGEN_A, O>;
impl<'a, const O: u8> PGAGEN_W<'a, O> {
    #[doc = "Disable gain setting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PGAGEN_A::_0)
    }
    #[doc = "Enable gain setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PGAGEN_A::_1)
    }
}
#[doc = "Field `PGADG` reader - PGA Unit n Differential Input Gain Setting"]
pub type PGADG_R = crate::FieldReader<u8, PGADG_A>;
#[doc = "PGA Unit n Differential Input Gain Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PGADG_A {
    #[doc = "0: Ã\u{97} 1.500"]
    _00 = 0,
    #[doc = "1: Ã\u{97} 2.333"]
    _01 = 1,
    #[doc = "2: Ã\u{97} 4.000"]
    _10 = 2,
    #[doc = "3: Ã\u{97} 5.667"]
    _11 = 3,
}
impl From<PGADG_A> for u8 {
    #[inline(always)]
    fn from(variant: PGADG_A) -> Self {
        variant as _
    }
}
impl PGADG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGADG_A {
        match self.bits {
            0 => PGADG_A::_00,
            1 => PGADG_A::_01,
            2 => PGADG_A::_10,
            3 => PGADG_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PGADG_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PGADG_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PGADG_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PGADG_A::_11
    }
}
#[doc = "Field `PGADG` writer - PGA Unit n Differential Input Gain Setting"]
pub type PGADG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADPGACR_SPEC, u8, PGADG_A, 2, O>;
impl<'a, const O: u8> PGADG_W<'a, O> {
    #[doc = "Ã\u{97} 1.500"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PGADG_A::_00)
    }
    #[doc = "Ã\u{97} 2.333"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PGADG_A::_01)
    }
    #[doc = "Ã\u{97} 4.000"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PGADG_A::_10)
    }
    #[doc = "Ã\u{97} 5.667"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PGADG_A::_11)
    }
}
#[doc = "Field `PGAGAIN` reader - PGA Unit n Gain Setting"]
pub type PGAGAIN_R = crate::FieldReader<u8, PGAGAIN_A>;
#[doc = "PGA Unit n Gain Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PGAGAIN_A {
    #[doc = "0: Ã\u{97} 2.000 (PGA is Single-ended Input mode) Setting prohibited (PGA is Pseudo-differential Input mode)"]
    _0X0 = 0,
    #[doc = "1: Ã\u{97} 2.500 (PGA is Single-ended Input mode) Ã\u{97} 1.500 (PGA is Pseudo-differential Input mode)"]
    _0X1 = 1,
    #[doc = "2: Ã\u{97} 2.667 (PGA is Single-ended Input mode) Setting prohibited (PGA is Pseudo-differential Input mode)"]
    _0X2 = 2,
    #[doc = "3: Ã\u{97} 2.857 (PGA is Single-ended Input mode) Setting prohibited (PGA is Pseudo-differential Input mode)"]
    _0X3 = 3,
    #[doc = "4: Ã\u{97} 3.077 (PGA is Single-ended Input mode) Setting prohibited (PGA is Pseudo-differential Input mode)"]
    _0X4 = 4,
    #[doc = "5: Ã\u{97} 3.333 (PGA is Single-ended Input mode) Ã\u{97} 2.333 (PGA is Pseudo-differential Input mode)"]
    _0X5 = 5,
    #[doc = "6: Ã\u{97} 3.636 (PGA is Single-ended Input mode) Setting prohibited (PGA is Pseudo-differential Input mode)"]
    _0X6 = 6,
    #[doc = "7: Ã\u{97} 4.000 (PGA is Single-ended Input mode) Setting prohibited (PGA is Pseudo-differential Input mode)"]
    _0X7 = 7,
    #[doc = "8: Ã\u{97} 4.444 (PGA is Single-ended Input mode) Setting prohibited (PGA is Pseudo-differential Input mode)"]
    _0X8 = 8,
    #[doc = "9: Ã\u{97} 5.000 (PGA is Single-ended Input mode) Ã\u{97} 4.000 (PGA is Pseudo-differential Input mode)"]
    _0X9 = 9,
    #[doc = "10: Ã\u{97} 5.714 (PGA is Single-ended Input mode) Setting prohibited (PGA is Pseudo-differential Input mode)"]
    _0X_A = 10,
    #[doc = "11: Ã\u{97} 6.667 (PGA is Single-ended Input mode) Ã\u{97} 5.667 (PGA is Pseudo-differential Input mode)"]
    _0X_B = 11,
    #[doc = "12: Ã\u{97} 8.000 (PGA is Single-ended Input mode) Setting prohibited (PGA is Pseudo-differential Input mode)"]
    _0X_C = 12,
    #[doc = "13: Ã\u{97} 10.000 (PGA is Single-ended Input mode) Setting prohibited (PGA is Pseudo-differential Input mode)"]
    _0X_D = 13,
    #[doc = "14: Ã\u{97} 13.333 (PGA is Single-ended Input mode) Setting prohibited (PGA is Pseudo-differential Input mode)"]
    _0X_E = 14,
    #[doc = "15: Setting prohibited"]
    _0X_F = 15,
}
impl From<PGAGAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: PGAGAIN_A) -> Self {
        variant as _
    }
}
impl PGAGAIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGAGAIN_A {
        match self.bits {
            0 => PGAGAIN_A::_0X0,
            1 => PGAGAIN_A::_0X1,
            2 => PGAGAIN_A::_0X2,
            3 => PGAGAIN_A::_0X3,
            4 => PGAGAIN_A::_0X4,
            5 => PGAGAIN_A::_0X5,
            6 => PGAGAIN_A::_0X6,
            7 => PGAGAIN_A::_0X7,
            8 => PGAGAIN_A::_0X8,
            9 => PGAGAIN_A::_0X9,
            10 => PGAGAIN_A::_0X_A,
            11 => PGAGAIN_A::_0X_B,
            12 => PGAGAIN_A::_0X_C,
            13 => PGAGAIN_A::_0X_D,
            14 => PGAGAIN_A::_0X_E,
            15 => PGAGAIN_A::_0X_F,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == PGAGAIN_A::_0X0
    }
    #[doc = "Checks if the value of the field is `_0X1`"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == PGAGAIN_A::_0X1
    }
    #[doc = "Checks if the value of the field is `_0X2`"]
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == PGAGAIN_A::_0X2
    }
    #[doc = "Checks if the value of the field is `_0X3`"]
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == PGAGAIN_A::_0X3
    }
    #[doc = "Checks if the value of the field is `_0X4`"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == PGAGAIN_A::_0X4
    }
    #[doc = "Checks if the value of the field is `_0X5`"]
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == PGAGAIN_A::_0X5
    }
    #[doc = "Checks if the value of the field is `_0X6`"]
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == PGAGAIN_A::_0X6
    }
    #[doc = "Checks if the value of the field is `_0X7`"]
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == PGAGAIN_A::_0X7
    }
    #[doc = "Checks if the value of the field is `_0X8`"]
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == PGAGAIN_A::_0X8
    }
    #[doc = "Checks if the value of the field is `_0X9`"]
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == PGAGAIN_A::_0X9
    }
    #[doc = "Checks if the value of the field is `_0X_A`"]
    #[inline(always)]
    pub fn is_0x_a(&self) -> bool {
        *self == PGAGAIN_A::_0X_A
    }
    #[doc = "Checks if the value of the field is `_0X_B`"]
    #[inline(always)]
    pub fn is_0x_b(&self) -> bool {
        *self == PGAGAIN_A::_0X_B
    }
    #[doc = "Checks if the value of the field is `_0X_C`"]
    #[inline(always)]
    pub fn is_0x_c(&self) -> bool {
        *self == PGAGAIN_A::_0X_C
    }
    #[doc = "Checks if the value of the field is `_0X_D`"]
    #[inline(always)]
    pub fn is_0x_d(&self) -> bool {
        *self == PGAGAIN_A::_0X_D
    }
    #[doc = "Checks if the value of the field is `_0X_E`"]
    #[inline(always)]
    pub fn is_0x_e(&self) -> bool {
        *self == PGAGAIN_A::_0X_E
    }
    #[doc = "Checks if the value of the field is `_0X_F`"]
    #[inline(always)]
    pub fn is_0x_f(&self) -> bool {
        *self == PGAGAIN_A::_0X_F
    }
}
#[doc = "Field `PGAGAIN` writer - PGA Unit n Gain Setting"]
pub type PGAGAIN_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADPGACR_SPEC, u8, PGAGAIN_A, 4, O>;
impl<'a, const O: u8> PGAGAIN_W<'a, O> {
    #[doc = "Ã\u{97} 2.000 (PGA is Single-ended Input mode) Setting prohibited (PGA is Pseudo-differential Input mode)"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(PGAGAIN_A::_0X0)
    }
    #[doc = "Ã\u{97} 2.500 (PGA is Single-ended Input mode) Ã\u{97} 1.500 (PGA is Pseudo-differential Input mode)"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut W {
        self.variant(PGAGAIN_A::_0X1)
    }
    #[doc = "Ã\u{97} 2.667 (PGA is Single-ended Input mode) Setting prohibited (PGA is Pseudo-differential Input mode)"]
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut W {
        self.variant(PGAGAIN_A::_0X2)
    }
    #[doc = "Ã\u{97} 2.857 (PGA is Single-ended Input mode) Setting prohibited (PGA is Pseudo-differential Input mode)"]
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut W {
        self.variant(PGAGAIN_A::_0X3)
    }
    #[doc = "Ã\u{97} 3.077 (PGA is Single-ended Input mode) Setting prohibited (PGA is Pseudo-differential Input mode)"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut W {
        self.variant(PGAGAIN_A::_0X4)
    }
    #[doc = "Ã\u{97} 3.333 (PGA is Single-ended Input mode) Ã\u{97} 2.333 (PGA is Pseudo-differential Input mode)"]
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut W {
        self.variant(PGAGAIN_A::_0X5)
    }
    #[doc = "Ã\u{97} 3.636 (PGA is Single-ended Input mode) Setting prohibited (PGA is Pseudo-differential Input mode)"]
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut W {
        self.variant(PGAGAIN_A::_0X6)
    }
    #[doc = "Ã\u{97} 4.000 (PGA is Single-ended Input mode) Setting prohibited (PGA is Pseudo-differential Input mode)"]
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut W {
        self.variant(PGAGAIN_A::_0X7)
    }
    #[doc = "Ã\u{97} 4.444 (PGA is Single-ended Input mode) Setting prohibited (PGA is Pseudo-differential Input mode)"]
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut W {
        self.variant(PGAGAIN_A::_0X8)
    }
    #[doc = "Ã\u{97} 5.000 (PGA is Single-ended Input mode) Ã\u{97} 4.000 (PGA is Pseudo-differential Input mode)"]
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut W {
        self.variant(PGAGAIN_A::_0X9)
    }
    #[doc = "Ã\u{97} 5.714 (PGA is Single-ended Input mode) Setting prohibited (PGA is Pseudo-differential Input mode)"]
    #[inline(always)]
    pub fn _0x_a(self) -> &'a mut W {
        self.variant(PGAGAIN_A::_0X_A)
    }
    #[doc = "Ã\u{97} 6.667 (PGA is Single-ended Input mode) Ã\u{97} 5.667 (PGA is Pseudo-differential Input mode)"]
    #[inline(always)]
    pub fn _0x_b(self) -> &'a mut W {
        self.variant(PGAGAIN_A::_0X_B)
    }
    #[doc = "Ã\u{97} 8.000 (PGA is Single-ended Input mode) Setting prohibited (PGA is Pseudo-differential Input mode)"]
    #[inline(always)]
    pub fn _0x_c(self) -> &'a mut W {
        self.variant(PGAGAIN_A::_0X_C)
    }
    #[doc = "Ã\u{97} 10.000 (PGA is Single-ended Input mode) Setting prohibited (PGA is Pseudo-differential Input mode)"]
    #[inline(always)]
    pub fn _0x_d(self) -> &'a mut W {
        self.variant(PGAGAIN_A::_0X_D)
    }
    #[doc = "Ã\u{97} 13.333 (PGA is Single-ended Input mode) Setting prohibited (PGA is Pseudo-differential Input mode)"]
    #[inline(always)]
    pub fn _0x_e(self) -> &'a mut W {
        self.variant(PGAGAIN_A::_0X_E)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _0x_f(self) -> &'a mut W {
        self.variant(PGAGAIN_A::_0X_F)
    }
}
impl R {
    #[doc = "Bit 1 - PGA Unit n Input Mode Select"]
    #[inline(always)]
    pub fn pgaden(&self) -> PGADEN_R {
        PGADEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PGA Unit n Amplifier Output Enable"]
    #[inline(always)]
    pub fn pgasel1(&self) -> PGASEL1_R {
        PGASEL1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PGA Unit n Enable"]
    #[inline(always)]
    pub fn pgaenamp(&self) -> PGAENAMP_R {
        PGAENAMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - PGA Unit n Gain Setting Enable"]
    #[inline(always)]
    pub fn pgagen(&self) -> PGAGEN_R {
        PGAGEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21 - PGA Unit n Differential Input Gain Setting"]
    #[inline(always)]
    pub fn pgadg(&self) -> PGADG_R {
        PGADG_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:27 - PGA Unit n Gain Setting"]
    #[inline(always)]
    pub fn pgagain(&self) -> PGAGAIN_R {
        PGAGAIN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - PGA Unit n Input Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn pgaden(&mut self) -> PGADEN_W<1> {
        PGADEN_W::new(self)
    }
    #[doc = "Bit 2 - PGA Unit n Amplifier Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pgasel1(&mut self) -> PGASEL1_W<2> {
        PGASEL1_W::new(self)
    }
    #[doc = "Bit 3 - PGA Unit n Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pgaenamp(&mut self) -> PGAENAMP_W<3> {
        PGAENAMP_W::new(self)
    }
    #[doc = "Bit 16 - PGA Unit n Gain Setting Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pgagen(&mut self) -> PGAGEN_W<16> {
        PGAGEN_W::new(self)
    }
    #[doc = "Bits 20:21 - PGA Unit n Differential Input Gain Setting"]
    #[inline(always)]
    #[must_use]
    pub fn pgadg(&mut self) -> PGADG_W<20> {
        PGADG_W::new(self)
    }
    #[doc = "Bits 24:27 - PGA Unit n Gain Setting"]
    #[inline(always)]
    #[must_use]
    pub fn pgagain(&mut self) -> PGAGAIN_W<24> {
        PGAGAIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Programmable Gain Amplifier Control Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpgacr](index.html) module"]
pub struct ADPGACR_SPEC;
impl crate::RegisterSpec for ADPGACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpgacr::R](R) reader structure"]
impl crate::Readable for ADPGACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpgacr::W](W) writer structure"]
impl crate::Writable for ADPGACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADPGACR%s to value 0"]
impl crate::Resettable for ADPGACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
