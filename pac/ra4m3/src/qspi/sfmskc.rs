#[doc = "Register `SFMSKC` reader"]
pub struct R(crate::R<SFMSKC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFMSKC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFMSKC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFMSKC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFMSKC` writer"]
pub struct W(crate::W<SFMSKC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFMSKC_SPEC>;
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
impl From<crate::W<SFMSKC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFMSKC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFMDV` reader - Serial interface reference cycle select. (Pay attention to irregularities.)"]
pub type SFMDV_R = crate::FieldReader<u8, SFMDV_A>;
#[doc = "Serial interface reference cycle select. (Pay attention to irregularities.)\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SFMDV_A {
    #[doc = "0: 2 PCLKA"]
    _0X00 = 0,
    #[doc = "1: 3 PCLKA (divided by an odd number)"]
    _0X01 = 1,
    #[doc = "2: 4 PCLKA"]
    _0X02 = 2,
    #[doc = "3: 5 PCLKA (divided by an odd number)"]
    _0X03 = 3,
    #[doc = "4: 6 PCLKA"]
    _0X04 = 4,
    #[doc = "5: 7 PCLKA (divided by an odd number)"]
    _0X05 = 5,
    #[doc = "6: 8 PCLKA"]
    _0X06 = 6,
    #[doc = "7: 9 PCLKA (divided by an odd number)"]
    _0X07 = 7,
    #[doc = "8: 10 PCLKA"]
    _0X08 = 8,
    #[doc = "9: 11 PCLKA (divided by an odd number)"]
    _0X09 = 9,
    #[doc = "10: 12 PCLKA"]
    _0X0A = 10,
    #[doc = "11: 13 PCLKA (divided by an odd number)"]
    _0X0B = 11,
    #[doc = "12: 14 PCLKA"]
    _0X0C = 12,
    #[doc = "13: 15 PCLKA (divided by an odd number)"]
    _0X0D = 13,
    #[doc = "14: 16 PCLKA"]
    _0X0E = 14,
    #[doc = "15: 17 PCLKA (divided by an odd number)"]
    _0X0F = 15,
    #[doc = "16: 18 PCLKA"]
    _0X10 = 16,
    #[doc = "17: 20 PCLKA"]
    _0X11 = 17,
    #[doc = "18: 22 PCLKA"]
    _0X12 = 18,
    #[doc = "19: 24 PCLKA"]
    _0X13 = 19,
    #[doc = "20: 26 PCLKA"]
    _0X14 = 20,
    #[doc = "21: 28 PCLKA"]
    _0X15 = 21,
    #[doc = "22: 30 PCLKA"]
    _0X16 = 22,
    #[doc = "23: 32 PCLKA"]
    _0X17 = 23,
    #[doc = "24: 34 PCLKA"]
    _0X18 = 24,
    #[doc = "25: 36 PCLKA"]
    _0X19 = 25,
    #[doc = "26: 38 PCLKA"]
    _0X1A = 26,
    #[doc = "27: 40 PCLKA"]
    _0X1B = 27,
    #[doc = "28: 42 PCLKA"]
    _0X1C = 28,
    #[doc = "29: 44 PCLKA"]
    _0X1D = 29,
    #[doc = "30: 46 PCLKA"]
    _0X1E = 30,
    #[doc = "31: 48 PCLKA"]
    _0X1F = 31,
}
impl From<SFMDV_A> for u8 {
    #[inline(always)]
    fn from(variant: SFMDV_A) -> Self {
        variant as _
    }
}
impl SFMDV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFMDV_A {
        match self.bits {
            0 => SFMDV_A::_0X00,
            1 => SFMDV_A::_0X01,
            2 => SFMDV_A::_0X02,
            3 => SFMDV_A::_0X03,
            4 => SFMDV_A::_0X04,
            5 => SFMDV_A::_0X05,
            6 => SFMDV_A::_0X06,
            7 => SFMDV_A::_0X07,
            8 => SFMDV_A::_0X08,
            9 => SFMDV_A::_0X09,
            10 => SFMDV_A::_0X0A,
            11 => SFMDV_A::_0X0B,
            12 => SFMDV_A::_0X0C,
            13 => SFMDV_A::_0X0D,
            14 => SFMDV_A::_0X0E,
            15 => SFMDV_A::_0X0F,
            16 => SFMDV_A::_0X10,
            17 => SFMDV_A::_0X11,
            18 => SFMDV_A::_0X12,
            19 => SFMDV_A::_0X13,
            20 => SFMDV_A::_0X14,
            21 => SFMDV_A::_0X15,
            22 => SFMDV_A::_0X16,
            23 => SFMDV_A::_0X17,
            24 => SFMDV_A::_0X18,
            25 => SFMDV_A::_0X19,
            26 => SFMDV_A::_0X1A,
            27 => SFMDV_A::_0X1B,
            28 => SFMDV_A::_0X1C,
            29 => SFMDV_A::_0X1D,
            30 => SFMDV_A::_0X1E,
            31 => SFMDV_A::_0X1F,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == SFMDV_A::_0X00
    }
    #[doc = "Checks if the value of the field is `_0X01`"]
    #[inline(always)]
    pub fn is_0x01(&self) -> bool {
        *self == SFMDV_A::_0X01
    }
    #[doc = "Checks if the value of the field is `_0X02`"]
    #[inline(always)]
    pub fn is_0x02(&self) -> bool {
        *self == SFMDV_A::_0X02
    }
    #[doc = "Checks if the value of the field is `_0X03`"]
    #[inline(always)]
    pub fn is_0x03(&self) -> bool {
        *self == SFMDV_A::_0X03
    }
    #[doc = "Checks if the value of the field is `_0X04`"]
    #[inline(always)]
    pub fn is_0x04(&self) -> bool {
        *self == SFMDV_A::_0X04
    }
    #[doc = "Checks if the value of the field is `_0X05`"]
    #[inline(always)]
    pub fn is_0x05(&self) -> bool {
        *self == SFMDV_A::_0X05
    }
    #[doc = "Checks if the value of the field is `_0X06`"]
    #[inline(always)]
    pub fn is_0x06(&self) -> bool {
        *self == SFMDV_A::_0X06
    }
    #[doc = "Checks if the value of the field is `_0X07`"]
    #[inline(always)]
    pub fn is_0x07(&self) -> bool {
        *self == SFMDV_A::_0X07
    }
    #[doc = "Checks if the value of the field is `_0X08`"]
    #[inline(always)]
    pub fn is_0x08(&self) -> bool {
        *self == SFMDV_A::_0X08
    }
    #[doc = "Checks if the value of the field is `_0X09`"]
    #[inline(always)]
    pub fn is_0x09(&self) -> bool {
        *self == SFMDV_A::_0X09
    }
    #[doc = "Checks if the value of the field is `_0X0A`"]
    #[inline(always)]
    pub fn is_0x0a(&self) -> bool {
        *self == SFMDV_A::_0X0A
    }
    #[doc = "Checks if the value of the field is `_0X0B`"]
    #[inline(always)]
    pub fn is_0x0b(&self) -> bool {
        *self == SFMDV_A::_0X0B
    }
    #[doc = "Checks if the value of the field is `_0X0C`"]
    #[inline(always)]
    pub fn is_0x0c(&self) -> bool {
        *self == SFMDV_A::_0X0C
    }
    #[doc = "Checks if the value of the field is `_0X0D`"]
    #[inline(always)]
    pub fn is_0x0d(&self) -> bool {
        *self == SFMDV_A::_0X0D
    }
    #[doc = "Checks if the value of the field is `_0X0E`"]
    #[inline(always)]
    pub fn is_0x0e(&self) -> bool {
        *self == SFMDV_A::_0X0E
    }
    #[doc = "Checks if the value of the field is `_0X0F`"]
    #[inline(always)]
    pub fn is_0x0f(&self) -> bool {
        *self == SFMDV_A::_0X0F
    }
    #[doc = "Checks if the value of the field is `_0X10`"]
    #[inline(always)]
    pub fn is_0x10(&self) -> bool {
        *self == SFMDV_A::_0X10
    }
    #[doc = "Checks if the value of the field is `_0X11`"]
    #[inline(always)]
    pub fn is_0x11(&self) -> bool {
        *self == SFMDV_A::_0X11
    }
    #[doc = "Checks if the value of the field is `_0X12`"]
    #[inline(always)]
    pub fn is_0x12(&self) -> bool {
        *self == SFMDV_A::_0X12
    }
    #[doc = "Checks if the value of the field is `_0X13`"]
    #[inline(always)]
    pub fn is_0x13(&self) -> bool {
        *self == SFMDV_A::_0X13
    }
    #[doc = "Checks if the value of the field is `_0X14`"]
    #[inline(always)]
    pub fn is_0x14(&self) -> bool {
        *self == SFMDV_A::_0X14
    }
    #[doc = "Checks if the value of the field is `_0X15`"]
    #[inline(always)]
    pub fn is_0x15(&self) -> bool {
        *self == SFMDV_A::_0X15
    }
    #[doc = "Checks if the value of the field is `_0X16`"]
    #[inline(always)]
    pub fn is_0x16(&self) -> bool {
        *self == SFMDV_A::_0X16
    }
    #[doc = "Checks if the value of the field is `_0X17`"]
    #[inline(always)]
    pub fn is_0x17(&self) -> bool {
        *self == SFMDV_A::_0X17
    }
    #[doc = "Checks if the value of the field is `_0X18`"]
    #[inline(always)]
    pub fn is_0x18(&self) -> bool {
        *self == SFMDV_A::_0X18
    }
    #[doc = "Checks if the value of the field is `_0X19`"]
    #[inline(always)]
    pub fn is_0x19(&self) -> bool {
        *self == SFMDV_A::_0X19
    }
    #[doc = "Checks if the value of the field is `_0X1A`"]
    #[inline(always)]
    pub fn is_0x1a(&self) -> bool {
        *self == SFMDV_A::_0X1A
    }
    #[doc = "Checks if the value of the field is `_0X1B`"]
    #[inline(always)]
    pub fn is_0x1b(&self) -> bool {
        *self == SFMDV_A::_0X1B
    }
    #[doc = "Checks if the value of the field is `_0X1C`"]
    #[inline(always)]
    pub fn is_0x1c(&self) -> bool {
        *self == SFMDV_A::_0X1C
    }
    #[doc = "Checks if the value of the field is `_0X1D`"]
    #[inline(always)]
    pub fn is_0x1d(&self) -> bool {
        *self == SFMDV_A::_0X1D
    }
    #[doc = "Checks if the value of the field is `_0X1E`"]
    #[inline(always)]
    pub fn is_0x1e(&self) -> bool {
        *self == SFMDV_A::_0X1E
    }
    #[doc = "Checks if the value of the field is `_0X1F`"]
    #[inline(always)]
    pub fn is_0x1f(&self) -> bool {
        *self == SFMDV_A::_0X1F
    }
}
#[doc = "Field `SFMDV` writer - Serial interface reference cycle select. (Pay attention to irregularities.)"]
pub type SFMDV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SFMSKC_SPEC, u8, SFMDV_A, 5, O>;
impl<'a, const O: u8> SFMDV_W<'a, O> {
    #[doc = "2 PCLKA"]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X00)
    }
    #[doc = "3 PCLKA (divided by an odd number)"]
    #[inline(always)]
    pub fn _0x01(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X01)
    }
    #[doc = "4 PCLKA"]
    #[inline(always)]
    pub fn _0x02(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X02)
    }
    #[doc = "5 PCLKA (divided by an odd number)"]
    #[inline(always)]
    pub fn _0x03(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X03)
    }
    #[doc = "6 PCLKA"]
    #[inline(always)]
    pub fn _0x04(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X04)
    }
    #[doc = "7 PCLKA (divided by an odd number)"]
    #[inline(always)]
    pub fn _0x05(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X05)
    }
    #[doc = "8 PCLKA"]
    #[inline(always)]
    pub fn _0x06(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X06)
    }
    #[doc = "9 PCLKA (divided by an odd number)"]
    #[inline(always)]
    pub fn _0x07(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X07)
    }
    #[doc = "10 PCLKA"]
    #[inline(always)]
    pub fn _0x08(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X08)
    }
    #[doc = "11 PCLKA (divided by an odd number)"]
    #[inline(always)]
    pub fn _0x09(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X09)
    }
    #[doc = "12 PCLKA"]
    #[inline(always)]
    pub fn _0x0a(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X0A)
    }
    #[doc = "13 PCLKA (divided by an odd number)"]
    #[inline(always)]
    pub fn _0x0b(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X0B)
    }
    #[doc = "14 PCLKA"]
    #[inline(always)]
    pub fn _0x0c(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X0C)
    }
    #[doc = "15 PCLKA (divided by an odd number)"]
    #[inline(always)]
    pub fn _0x0d(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X0D)
    }
    #[doc = "16 PCLKA"]
    #[inline(always)]
    pub fn _0x0e(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X0E)
    }
    #[doc = "17 PCLKA (divided by an odd number)"]
    #[inline(always)]
    pub fn _0x0f(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X0F)
    }
    #[doc = "18 PCLKA"]
    #[inline(always)]
    pub fn _0x10(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X10)
    }
    #[doc = "20 PCLKA"]
    #[inline(always)]
    pub fn _0x11(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X11)
    }
    #[doc = "22 PCLKA"]
    #[inline(always)]
    pub fn _0x12(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X12)
    }
    #[doc = "24 PCLKA"]
    #[inline(always)]
    pub fn _0x13(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X13)
    }
    #[doc = "26 PCLKA"]
    #[inline(always)]
    pub fn _0x14(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X14)
    }
    #[doc = "28 PCLKA"]
    #[inline(always)]
    pub fn _0x15(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X15)
    }
    #[doc = "30 PCLKA"]
    #[inline(always)]
    pub fn _0x16(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X16)
    }
    #[doc = "32 PCLKA"]
    #[inline(always)]
    pub fn _0x17(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X17)
    }
    #[doc = "34 PCLKA"]
    #[inline(always)]
    pub fn _0x18(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X18)
    }
    #[doc = "36 PCLKA"]
    #[inline(always)]
    pub fn _0x19(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X19)
    }
    #[doc = "38 PCLKA"]
    #[inline(always)]
    pub fn _0x1a(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X1A)
    }
    #[doc = "40 PCLKA"]
    #[inline(always)]
    pub fn _0x1b(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X1B)
    }
    #[doc = "42 PCLKA"]
    #[inline(always)]
    pub fn _0x1c(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X1C)
    }
    #[doc = "44 PCLKA"]
    #[inline(always)]
    pub fn _0x1d(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X1D)
    }
    #[doc = "46 PCLKA"]
    #[inline(always)]
    pub fn _0x1e(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X1E)
    }
    #[doc = "48 PCLKA"]
    #[inline(always)]
    pub fn _0x1f(self) -> &'a mut W {
        self.variant(SFMDV_A::_0X1F)
    }
}
#[doc = "Field `SFMDTY` reader - Duty ratio correction function select for the QSPCLK signal when devided by an odd number"]
pub type SFMDTY_R = crate::BitReader<SFMDTY_A>;
#[doc = "Duty ratio correction function select for the QSPCLK signal when devided by an odd number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMDTY_A {
    #[doc = "0: Make no correction"]
    _0 = 0,
    #[doc = "1: Make correction"]
    _1 = 1,
}
impl From<SFMDTY_A> for bool {
    #[inline(always)]
    fn from(variant: SFMDTY_A) -> Self {
        variant as u8 != 0
    }
}
impl SFMDTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFMDTY_A {
        match self.bits {
            false => SFMDTY_A::_0,
            true => SFMDTY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMDTY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMDTY_A::_1
    }
}
#[doc = "Field `SFMDTY` writer - Duty ratio correction function select for the QSPCLK signal when devided by an odd number"]
pub type SFMDTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFMSKC_SPEC, SFMDTY_A, O>;
impl<'a, const O: u8> SFMDTY_W<'a, O> {
    #[doc = "Make no correction"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SFMDTY_A::_0)
    }
    #[doc = "Make correction"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SFMDTY_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:4 - Serial interface reference cycle select. (Pay attention to irregularities.)"]
    #[inline(always)]
    pub fn sfmdv(&self) -> SFMDV_R {
        SFMDV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Duty ratio correction function select for the QSPCLK signal when devided by an odd number"]
    #[inline(always)]
    pub fn sfmdty(&self) -> SFMDTY_R {
        SFMDTY_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Serial interface reference cycle select. (Pay attention to irregularities.)"]
    #[inline(always)]
    #[must_use]
    pub fn sfmdv(&mut self) -> SFMDV_W<0> {
        SFMDV_W::new(self)
    }
    #[doc = "Bit 5 - Duty ratio correction function select for the QSPCLK signal when devided by an odd number"]
    #[inline(always)]
    #[must_use]
    pub fn sfmdty(&mut self) -> SFMDTY_W<5> {
        SFMDTY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfmskc](index.html) module"]
pub struct SFMSKC_SPEC;
impl crate::RegisterSpec for SFMSKC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfmskc::R](R) reader structure"]
impl crate::Readable for SFMSKC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfmskc::W](W) writer structure"]
impl crate::Writable for SFMSKC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFMSKC to value 0x08"]
impl crate::Resettable for SFMSKC_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
