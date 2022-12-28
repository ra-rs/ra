#[doc = "Register `SD_OPTION` reader"]
pub struct R(crate::R<SD_OPTION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_OPTION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_OPTION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_OPTION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SD_OPTION` writer"]
pub struct W(crate::W<SD_OPTION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_OPTION_SPEC>;
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
impl From<crate::W<SD_OPTION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_OPTION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTOP` reader - Card Detection Time Counter"]
pub type CTOP_R = crate::FieldReader<u8, CTOP_A>;
#[doc = "Card Detection Time Counter\n\nValue on reset: 14"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTOP_A {
    #[doc = "0: PCLKB × 210"]
    _0X0 = 0,
    #[doc = "1: PCLKB × 211"]
    _0X1 = 1,
    #[doc = "2: PCLKB × 212"]
    _0X2 = 2,
    #[doc = "3: PCLKB × 213"]
    _0X3 = 3,
    #[doc = "4: PCLKB × 214"]
    _0X4 = 4,
    #[doc = "5: PCLKB × 215"]
    _0X5 = 5,
    #[doc = "6: PCLKB × 216"]
    _0X6 = 6,
    #[doc = "7: PCLKB × 217"]
    _0X7 = 7,
    #[doc = "8: PCLKB × 218"]
    _0X8 = 8,
    #[doc = "9: PCLKB × 219"]
    _0X9 = 9,
    #[doc = "10: PCLKB × 220"]
    _0X_A = 10,
    #[doc = "11: PCLKB × 221"]
    _0X_B = 11,
    #[doc = "12: PCLKB × 222"]
    _0X_C = 12,
    #[doc = "13: PCLKB × 223"]
    _0X_D = 13,
    #[doc = "14: PCLKB × 224"]
    _0X_E = 14,
    #[doc = "15: Setting prohibited"]
    _0X_F = 15,
}
impl From<CTOP_A> for u8 {
    #[inline(always)]
    fn from(variant: CTOP_A) -> Self {
        variant as _
    }
}
impl CTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTOP_A {
        match self.bits {
            0 => CTOP_A::_0X0,
            1 => CTOP_A::_0X1,
            2 => CTOP_A::_0X2,
            3 => CTOP_A::_0X3,
            4 => CTOP_A::_0X4,
            5 => CTOP_A::_0X5,
            6 => CTOP_A::_0X6,
            7 => CTOP_A::_0X7,
            8 => CTOP_A::_0X8,
            9 => CTOP_A::_0X9,
            10 => CTOP_A::_0X_A,
            11 => CTOP_A::_0X_B,
            12 => CTOP_A::_0X_C,
            13 => CTOP_A::_0X_D,
            14 => CTOP_A::_0X_E,
            15 => CTOP_A::_0X_F,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == CTOP_A::_0X0
    }
    #[doc = "Checks if the value of the field is `_0X1`"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == CTOP_A::_0X1
    }
    #[doc = "Checks if the value of the field is `_0X2`"]
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == CTOP_A::_0X2
    }
    #[doc = "Checks if the value of the field is `_0X3`"]
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == CTOP_A::_0X3
    }
    #[doc = "Checks if the value of the field is `_0X4`"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == CTOP_A::_0X4
    }
    #[doc = "Checks if the value of the field is `_0X5`"]
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == CTOP_A::_0X5
    }
    #[doc = "Checks if the value of the field is `_0X6`"]
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == CTOP_A::_0X6
    }
    #[doc = "Checks if the value of the field is `_0X7`"]
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == CTOP_A::_0X7
    }
    #[doc = "Checks if the value of the field is `_0X8`"]
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == CTOP_A::_0X8
    }
    #[doc = "Checks if the value of the field is `_0X9`"]
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == CTOP_A::_0X9
    }
    #[doc = "Checks if the value of the field is `_0X_A`"]
    #[inline(always)]
    pub fn is_0x_a(&self) -> bool {
        *self == CTOP_A::_0X_A
    }
    #[doc = "Checks if the value of the field is `_0X_B`"]
    #[inline(always)]
    pub fn is_0x_b(&self) -> bool {
        *self == CTOP_A::_0X_B
    }
    #[doc = "Checks if the value of the field is `_0X_C`"]
    #[inline(always)]
    pub fn is_0x_c(&self) -> bool {
        *self == CTOP_A::_0X_C
    }
    #[doc = "Checks if the value of the field is `_0X_D`"]
    #[inline(always)]
    pub fn is_0x_d(&self) -> bool {
        *self == CTOP_A::_0X_D
    }
    #[doc = "Checks if the value of the field is `_0X_E`"]
    #[inline(always)]
    pub fn is_0x_e(&self) -> bool {
        *self == CTOP_A::_0X_E
    }
    #[doc = "Checks if the value of the field is `_0X_F`"]
    #[inline(always)]
    pub fn is_0x_f(&self) -> bool {
        *self == CTOP_A::_0X_F
    }
}
#[doc = "Field `CTOP` writer - Card Detection Time Counter"]
pub type CTOP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SD_OPTION_SPEC, u8, CTOP_A, 4, O>;
impl<'a, const O: u8> CTOP_W<'a, O> {
    #[doc = "PCLKB × 210"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(CTOP_A::_0X0)
    }
    #[doc = "PCLKB × 211"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut W {
        self.variant(CTOP_A::_0X1)
    }
    #[doc = "PCLKB × 212"]
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut W {
        self.variant(CTOP_A::_0X2)
    }
    #[doc = "PCLKB × 213"]
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut W {
        self.variant(CTOP_A::_0X3)
    }
    #[doc = "PCLKB × 214"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut W {
        self.variant(CTOP_A::_0X4)
    }
    #[doc = "PCLKB × 215"]
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut W {
        self.variant(CTOP_A::_0X5)
    }
    #[doc = "PCLKB × 216"]
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut W {
        self.variant(CTOP_A::_0X6)
    }
    #[doc = "PCLKB × 217"]
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut W {
        self.variant(CTOP_A::_0X7)
    }
    #[doc = "PCLKB × 218"]
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut W {
        self.variant(CTOP_A::_0X8)
    }
    #[doc = "PCLKB × 219"]
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut W {
        self.variant(CTOP_A::_0X9)
    }
    #[doc = "PCLKB × 220"]
    #[inline(always)]
    pub fn _0x_a(self) -> &'a mut W {
        self.variant(CTOP_A::_0X_A)
    }
    #[doc = "PCLKB × 221"]
    #[inline(always)]
    pub fn _0x_b(self) -> &'a mut W {
        self.variant(CTOP_A::_0X_B)
    }
    #[doc = "PCLKB × 222"]
    #[inline(always)]
    pub fn _0x_c(self) -> &'a mut W {
        self.variant(CTOP_A::_0X_C)
    }
    #[doc = "PCLKB × 223"]
    #[inline(always)]
    pub fn _0x_d(self) -> &'a mut W {
        self.variant(CTOP_A::_0X_D)
    }
    #[doc = "PCLKB × 224"]
    #[inline(always)]
    pub fn _0x_e(self) -> &'a mut W {
        self.variant(CTOP_A::_0X_E)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _0x_f(self) -> &'a mut W {
        self.variant(CTOP_A::_0X_F)
    }
}
#[doc = "Field `TOP` reader - Timeout Counter"]
pub type TOP_R = crate::FieldReader<u8, TOP_A>;
#[doc = "Timeout Counter\n\nValue on reset: 14"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TOP_A {
    #[doc = "0: SDHI clock × 213"]
    _0X0 = 0,
    #[doc = "1: SDHI clock × 214"]
    _0X1 = 1,
    #[doc = "2: SDHI clock × 215"]
    _0X2 = 2,
    #[doc = "3: SDHI clock × 216"]
    _0X3 = 3,
    #[doc = "4: SDHI clock × 217"]
    _0X4 = 4,
    #[doc = "5: SDHI clock × 218"]
    _0X5 = 5,
    #[doc = "6: SDHI clock × 219"]
    _0X6 = 6,
    #[doc = "7: SDHI clock × 220"]
    _0X7 = 7,
    #[doc = "8: SDHI clock × 221"]
    _0X8 = 8,
    #[doc = "9: SDHI clock × 222"]
    _0X9 = 9,
    #[doc = "10: SDHI clock × 223"]
    _0X_A = 10,
    #[doc = "11: SDHI clock × 224"]
    _0X_B = 11,
    #[doc = "12: SDHI clock × 225"]
    _0X_C = 12,
    #[doc = "13: SDHI clock × 226"]
    _0X_D = 13,
    #[doc = "14: SDHI clock × 227"]
    _0X_E = 14,
    #[doc = "15: Setting prohibited"]
    _0X_F = 15,
}
impl From<TOP_A> for u8 {
    #[inline(always)]
    fn from(variant: TOP_A) -> Self {
        variant as _
    }
}
impl TOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOP_A {
        match self.bits {
            0 => TOP_A::_0X0,
            1 => TOP_A::_0X1,
            2 => TOP_A::_0X2,
            3 => TOP_A::_0X3,
            4 => TOP_A::_0X4,
            5 => TOP_A::_0X5,
            6 => TOP_A::_0X6,
            7 => TOP_A::_0X7,
            8 => TOP_A::_0X8,
            9 => TOP_A::_0X9,
            10 => TOP_A::_0X_A,
            11 => TOP_A::_0X_B,
            12 => TOP_A::_0X_C,
            13 => TOP_A::_0X_D,
            14 => TOP_A::_0X_E,
            15 => TOP_A::_0X_F,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == TOP_A::_0X0
    }
    #[doc = "Checks if the value of the field is `_0X1`"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == TOP_A::_0X1
    }
    #[doc = "Checks if the value of the field is `_0X2`"]
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == TOP_A::_0X2
    }
    #[doc = "Checks if the value of the field is `_0X3`"]
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == TOP_A::_0X3
    }
    #[doc = "Checks if the value of the field is `_0X4`"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == TOP_A::_0X4
    }
    #[doc = "Checks if the value of the field is `_0X5`"]
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == TOP_A::_0X5
    }
    #[doc = "Checks if the value of the field is `_0X6`"]
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == TOP_A::_0X6
    }
    #[doc = "Checks if the value of the field is `_0X7`"]
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == TOP_A::_0X7
    }
    #[doc = "Checks if the value of the field is `_0X8`"]
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == TOP_A::_0X8
    }
    #[doc = "Checks if the value of the field is `_0X9`"]
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == TOP_A::_0X9
    }
    #[doc = "Checks if the value of the field is `_0X_A`"]
    #[inline(always)]
    pub fn is_0x_a(&self) -> bool {
        *self == TOP_A::_0X_A
    }
    #[doc = "Checks if the value of the field is `_0X_B`"]
    #[inline(always)]
    pub fn is_0x_b(&self) -> bool {
        *self == TOP_A::_0X_B
    }
    #[doc = "Checks if the value of the field is `_0X_C`"]
    #[inline(always)]
    pub fn is_0x_c(&self) -> bool {
        *self == TOP_A::_0X_C
    }
    #[doc = "Checks if the value of the field is `_0X_D`"]
    #[inline(always)]
    pub fn is_0x_d(&self) -> bool {
        *self == TOP_A::_0X_D
    }
    #[doc = "Checks if the value of the field is `_0X_E`"]
    #[inline(always)]
    pub fn is_0x_e(&self) -> bool {
        *self == TOP_A::_0X_E
    }
    #[doc = "Checks if the value of the field is `_0X_F`"]
    #[inline(always)]
    pub fn is_0x_f(&self) -> bool {
        *self == TOP_A::_0X_F
    }
}
#[doc = "Field `TOP` writer - Timeout Counter"]
pub type TOP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SD_OPTION_SPEC, u8, TOP_A, 4, O>;
impl<'a, const O: u8> TOP_W<'a, O> {
    #[doc = "SDHI clock × 213"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(TOP_A::_0X0)
    }
    #[doc = "SDHI clock × 214"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut W {
        self.variant(TOP_A::_0X1)
    }
    #[doc = "SDHI clock × 215"]
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut W {
        self.variant(TOP_A::_0X2)
    }
    #[doc = "SDHI clock × 216"]
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut W {
        self.variant(TOP_A::_0X3)
    }
    #[doc = "SDHI clock × 217"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut W {
        self.variant(TOP_A::_0X4)
    }
    #[doc = "SDHI clock × 218"]
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut W {
        self.variant(TOP_A::_0X5)
    }
    #[doc = "SDHI clock × 219"]
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut W {
        self.variant(TOP_A::_0X6)
    }
    #[doc = "SDHI clock × 220"]
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut W {
        self.variant(TOP_A::_0X7)
    }
    #[doc = "SDHI clock × 221"]
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut W {
        self.variant(TOP_A::_0X8)
    }
    #[doc = "SDHI clock × 222"]
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut W {
        self.variant(TOP_A::_0X9)
    }
    #[doc = "SDHI clock × 223"]
    #[inline(always)]
    pub fn _0x_a(self) -> &'a mut W {
        self.variant(TOP_A::_0X_A)
    }
    #[doc = "SDHI clock × 224"]
    #[inline(always)]
    pub fn _0x_b(self) -> &'a mut W {
        self.variant(TOP_A::_0X_B)
    }
    #[doc = "SDHI clock × 225"]
    #[inline(always)]
    pub fn _0x_c(self) -> &'a mut W {
        self.variant(TOP_A::_0X_C)
    }
    #[doc = "SDHI clock × 226"]
    #[inline(always)]
    pub fn _0x_d(self) -> &'a mut W {
        self.variant(TOP_A::_0X_D)
    }
    #[doc = "SDHI clock × 227"]
    #[inline(always)]
    pub fn _0x_e(self) -> &'a mut W {
        self.variant(TOP_A::_0X_E)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _0x_f(self) -> &'a mut W {
        self.variant(TOP_A::_0X_F)
    }
}
#[doc = "Field `TOUTMASK` reader - Timeout Mask"]
pub type TOUTMASK_R = crate::BitReader<TOUTMASK_A>;
#[doc = "Timeout Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOUTMASK_A {
    #[doc = "0: Activate timeout"]
    _0 = 0,
    #[doc = "1: Inactivate timeout (do not set RSPTO and DTO bits of SD_INFO2 or CRCBSYTO, CRCTO, RDTO, BSYTO1, BSYTO0, RSPTO1 and RSPTO0 bits of SD_ERR_STS2) When timeout occurs because of an inactivated timeout, execute a software reset to terminate the command sequence."]
    _1 = 1,
}
impl From<TOUTMASK_A> for bool {
    #[inline(always)]
    fn from(variant: TOUTMASK_A) -> Self {
        variant as u8 != 0
    }
}
impl TOUTMASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOUTMASK_A {
        match self.bits {
            false => TOUTMASK_A::_0,
            true => TOUTMASK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOUTMASK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOUTMASK_A::_1
    }
}
#[doc = "Field `TOUTMASK` writer - Timeout Mask"]
pub type TOUTMASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_OPTION_SPEC, TOUTMASK_A, O>;
impl<'a, const O: u8> TOUTMASK_W<'a, O> {
    #[doc = "Activate timeout"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOUTMASK_A::_0)
    }
    #[doc = "Inactivate timeout (do not set RSPTO and DTO bits of SD_INFO2 or CRCBSYTO, CRCTO, RDTO, BSYTO1, BSYTO0, RSPTO1 and RSPTO0 bits of SD_ERR_STS2) When timeout occurs because of an inactivated timeout, execute a software reset to terminate the command sequence."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOUTMASK_A::_1)
    }
}
#[doc = "Field `WIDTH8` reader - Bus Width"]
pub type WIDTH8_R = crate::BitReader<bool>;
#[doc = "Field `WIDTH8` writer - Bus Width"]
pub type WIDTH8_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_OPTION_SPEC, bool, O>;
#[doc = "Field `WIDTH` reader - Bus Width"]
pub type WIDTH_R = crate::BitReader<bool>;
#[doc = "Field `WIDTH` writer - Bus Width"]
pub type WIDTH_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_OPTION_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Card Detection Time Counter"]
    #[inline(always)]
    pub fn ctop(&self) -> CTOP_R {
        CTOP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Timeout Counter"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Timeout Mask"]
    #[inline(always)]
    pub fn toutmask(&self) -> TOUTMASK_R {
        TOUTMASK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 13 - Bus Width"]
    #[inline(always)]
    pub fn width8(&self) -> WIDTH8_R {
        WIDTH8_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Bus Width"]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Card Detection Time Counter"]
    #[inline(always)]
    #[must_use]
    pub fn ctop(&mut self) -> CTOP_W<0> {
        CTOP_W::new(self)
    }
    #[doc = "Bits 4:7 - Timeout Counter"]
    #[inline(always)]
    #[must_use]
    pub fn top(&mut self) -> TOP_W<4> {
        TOP_W::new(self)
    }
    #[doc = "Bit 8 - Timeout Mask"]
    #[inline(always)]
    #[must_use]
    pub fn toutmask(&mut self) -> TOUTMASK_W<8> {
        TOUTMASK_W::new(self)
    }
    #[doc = "Bit 13 - Bus Width"]
    #[inline(always)]
    #[must_use]
    pub fn width8(&mut self) -> WIDTH8_W<13> {
        WIDTH8_W::new(self)
    }
    #[doc = "Bit 15 - Bus Width"]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WIDTH_W<15> {
        WIDTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD Card Access Control Option Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_option](index.html) module"]
pub struct SD_OPTION_SPEC;
impl crate::RegisterSpec for SD_OPTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_option::R](R) reader structure"]
impl crate::Readable for SD_OPTION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_option::W](W) writer structure"]
impl crate::Writable for SD_OPTION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SD_OPTION to value 0x40ee"]
impl crate::Resettable for SD_OPTION_SPEC {
    const RESET_VALUE: Self::Ux = 0x40ee;
}
