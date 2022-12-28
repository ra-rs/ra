#[doc = "Register `SFMSSC` reader"]
pub struct R(crate::R<SFMSSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFMSSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFMSSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFMSSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFMSSC` writer"]
pub struct W(crate::W<SFMSSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFMSSC_SPEC>;
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
impl From<crate::W<SFMSSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFMSSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFMSW` reader - Minimum high-level width select for QSSL signal"]
pub type SFMSW_R = crate::FieldReader<u8, SFMSW_A>;
#[doc = "Minimum high-level width select for QSSL signal\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SFMSW_A {
    #[doc = "0: 1 QSPCLK"]
    _0X0 = 0,
    #[doc = "1: 2 QSPCLK"]
    _0X1 = 1,
    #[doc = "2: 3 QSPCLK"]
    _0X2 = 2,
    #[doc = "3: 4 QSPCLK"]
    _0X3 = 3,
    #[doc = "4: 5 QSPCLK"]
    _0X4 = 4,
    #[doc = "5: 6 QSPCLK"]
    _0X5 = 5,
    #[doc = "6: 7 QSPCLK"]
    _0X6 = 6,
    #[doc = "7: 8 QSPCLK"]
    _0X7 = 7,
    #[doc = "8: 9 QSPCLK"]
    _0X8 = 8,
    #[doc = "9: 10 QSPCLK"]
    _0X9 = 9,
    #[doc = "10: 11 QSPCLK"]
    _0X_A = 10,
    #[doc = "11: 12 QSPCLK"]
    _0X_B = 11,
    #[doc = "12: 13 QSPCLK"]
    _0X_C = 12,
    #[doc = "13: 14 QSPCLK"]
    _0X_D = 13,
    #[doc = "14: 15 QSPCLK"]
    _0X_E = 14,
    #[doc = "15: 16 QSPCLK"]
    _0X_F = 15,
}
impl From<SFMSW_A> for u8 {
    #[inline(always)]
    fn from(variant: SFMSW_A) -> Self {
        variant as _
    }
}
impl SFMSW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFMSW_A {
        match self.bits {
            0 => SFMSW_A::_0X0,
            1 => SFMSW_A::_0X1,
            2 => SFMSW_A::_0X2,
            3 => SFMSW_A::_0X3,
            4 => SFMSW_A::_0X4,
            5 => SFMSW_A::_0X5,
            6 => SFMSW_A::_0X6,
            7 => SFMSW_A::_0X7,
            8 => SFMSW_A::_0X8,
            9 => SFMSW_A::_0X9,
            10 => SFMSW_A::_0X_A,
            11 => SFMSW_A::_0X_B,
            12 => SFMSW_A::_0X_C,
            13 => SFMSW_A::_0X_D,
            14 => SFMSW_A::_0X_E,
            15 => SFMSW_A::_0X_F,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == SFMSW_A::_0X0
    }
    #[doc = "Checks if the value of the field is `_0X1`"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == SFMSW_A::_0X1
    }
    #[doc = "Checks if the value of the field is `_0X2`"]
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == SFMSW_A::_0X2
    }
    #[doc = "Checks if the value of the field is `_0X3`"]
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == SFMSW_A::_0X3
    }
    #[doc = "Checks if the value of the field is `_0X4`"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == SFMSW_A::_0X4
    }
    #[doc = "Checks if the value of the field is `_0X5`"]
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == SFMSW_A::_0X5
    }
    #[doc = "Checks if the value of the field is `_0X6`"]
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == SFMSW_A::_0X6
    }
    #[doc = "Checks if the value of the field is `_0X7`"]
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == SFMSW_A::_0X7
    }
    #[doc = "Checks if the value of the field is `_0X8`"]
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == SFMSW_A::_0X8
    }
    #[doc = "Checks if the value of the field is `_0X9`"]
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == SFMSW_A::_0X9
    }
    #[doc = "Checks if the value of the field is `_0X_A`"]
    #[inline(always)]
    pub fn is_0x_a(&self) -> bool {
        *self == SFMSW_A::_0X_A
    }
    #[doc = "Checks if the value of the field is `_0X_B`"]
    #[inline(always)]
    pub fn is_0x_b(&self) -> bool {
        *self == SFMSW_A::_0X_B
    }
    #[doc = "Checks if the value of the field is `_0X_C`"]
    #[inline(always)]
    pub fn is_0x_c(&self) -> bool {
        *self == SFMSW_A::_0X_C
    }
    #[doc = "Checks if the value of the field is `_0X_D`"]
    #[inline(always)]
    pub fn is_0x_d(&self) -> bool {
        *self == SFMSW_A::_0X_D
    }
    #[doc = "Checks if the value of the field is `_0X_E`"]
    #[inline(always)]
    pub fn is_0x_e(&self) -> bool {
        *self == SFMSW_A::_0X_E
    }
    #[doc = "Checks if the value of the field is `_0X_F`"]
    #[inline(always)]
    pub fn is_0x_f(&self) -> bool {
        *self == SFMSW_A::_0X_F
    }
}
#[doc = "Field `SFMSW` writer - Minimum high-level width select for QSSL signal"]
pub type SFMSW_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SFMSSC_SPEC, u8, SFMSW_A, 4, O>;
impl<'a, const O: u8> SFMSW_W<'a, O> {
    #[doc = "1 QSPCLK"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(SFMSW_A::_0X0)
    }
    #[doc = "2 QSPCLK"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut W {
        self.variant(SFMSW_A::_0X1)
    }
    #[doc = "3 QSPCLK"]
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut W {
        self.variant(SFMSW_A::_0X2)
    }
    #[doc = "4 QSPCLK"]
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut W {
        self.variant(SFMSW_A::_0X3)
    }
    #[doc = "5 QSPCLK"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut W {
        self.variant(SFMSW_A::_0X4)
    }
    #[doc = "6 QSPCLK"]
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut W {
        self.variant(SFMSW_A::_0X5)
    }
    #[doc = "7 QSPCLK"]
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut W {
        self.variant(SFMSW_A::_0X6)
    }
    #[doc = "8 QSPCLK"]
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut W {
        self.variant(SFMSW_A::_0X7)
    }
    #[doc = "9 QSPCLK"]
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut W {
        self.variant(SFMSW_A::_0X8)
    }
    #[doc = "10 QSPCLK"]
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut W {
        self.variant(SFMSW_A::_0X9)
    }
    #[doc = "11 QSPCLK"]
    #[inline(always)]
    pub fn _0x_a(self) -> &'a mut W {
        self.variant(SFMSW_A::_0X_A)
    }
    #[doc = "12 QSPCLK"]
    #[inline(always)]
    pub fn _0x_b(self) -> &'a mut W {
        self.variant(SFMSW_A::_0X_B)
    }
    #[doc = "13 QSPCLK"]
    #[inline(always)]
    pub fn _0x_c(self) -> &'a mut W {
        self.variant(SFMSW_A::_0X_C)
    }
    #[doc = "14 QSPCLK"]
    #[inline(always)]
    pub fn _0x_d(self) -> &'a mut W {
        self.variant(SFMSW_A::_0X_D)
    }
    #[doc = "15 QSPCLK"]
    #[inline(always)]
    pub fn _0x_e(self) -> &'a mut W {
        self.variant(SFMSW_A::_0X_E)
    }
    #[doc = "16 QSPCLK"]
    #[inline(always)]
    pub fn _0x_f(self) -> &'a mut W {
        self.variant(SFMSW_A::_0X_F)
    }
}
#[doc = "Field `SFMSHD` reader - QSSL Signal Hold Time"]
pub type SFMSHD_R = crate::BitReader<SFMSHD_A>;
#[doc = "QSSL Signal Hold Time\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMSHD_A {
    #[doc = "0: QSSL outputs high after 0.5 QSPCLK cycles from the last rising edge of QSPCLK."]
    _0 = 0,
    #[doc = "1: QSSL outputs high after 1.5 QSPCLK cycles from the last rising edge of QSPCLK."]
    _1 = 1,
}
impl From<SFMSHD_A> for bool {
    #[inline(always)]
    fn from(variant: SFMSHD_A) -> Self {
        variant as u8 != 0
    }
}
impl SFMSHD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFMSHD_A {
        match self.bits {
            false => SFMSHD_A::_0,
            true => SFMSHD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMSHD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMSHD_A::_1
    }
}
#[doc = "Field `SFMSHD` writer - QSSL Signal Hold Time"]
pub type SFMSHD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFMSSC_SPEC, SFMSHD_A, O>;
impl<'a, const O: u8> SFMSHD_W<'a, O> {
    #[doc = "QSSL outputs high after 0.5 QSPCLK cycles from the last rising edge of QSPCLK."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SFMSHD_A::_0)
    }
    #[doc = "QSSL outputs high after 1.5 QSPCLK cycles from the last rising edge of QSPCLK."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SFMSHD_A::_1)
    }
}
#[doc = "Field `SFMSLD` reader - QSSL Signal Setup Time"]
pub type SFMSLD_R = crate::BitReader<SFMSLD_A>;
#[doc = "QSSL Signal Setup Time\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMSLD_A {
    #[doc = "0: QSSL outputs low before 0.5 QSPCLK cycles from the first rising edge of QSPCLK."]
    _0 = 0,
    #[doc = "1: QSSL outputs low before 1.5 QSPCLK cycles from the first rising edge of QSPCLK."]
    _1 = 1,
}
impl From<SFMSLD_A> for bool {
    #[inline(always)]
    fn from(variant: SFMSLD_A) -> Self {
        variant as u8 != 0
    }
}
impl SFMSLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFMSLD_A {
        match self.bits {
            false => SFMSLD_A::_0,
            true => SFMSLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMSLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMSLD_A::_1
    }
}
#[doc = "Field `SFMSLD` writer - QSSL Signal Setup Time"]
pub type SFMSLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFMSSC_SPEC, SFMSLD_A, O>;
impl<'a, const O: u8> SFMSLD_W<'a, O> {
    #[doc = "QSSL outputs low before 0.5 QSPCLK cycles from the first rising edge of QSPCLK."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SFMSLD_A::_0)
    }
    #[doc = "QSSL outputs low before 1.5 QSPCLK cycles from the first rising edge of QSPCLK."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SFMSLD_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Minimum high-level width select for QSSL signal"]
    #[inline(always)]
    pub fn sfmsw(&self) -> SFMSW_R {
        SFMSW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - QSSL Signal Hold Time"]
    #[inline(always)]
    pub fn sfmshd(&self) -> SFMSHD_R {
        SFMSHD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - QSSL Signal Setup Time"]
    #[inline(always)]
    pub fn sfmsld(&self) -> SFMSLD_R {
        SFMSLD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Minimum high-level width select for QSSL signal"]
    #[inline(always)]
    #[must_use]
    pub fn sfmsw(&mut self) -> SFMSW_W<0> {
        SFMSW_W::new(self)
    }
    #[doc = "Bit 4 - QSSL Signal Hold Time"]
    #[inline(always)]
    #[must_use]
    pub fn sfmshd(&mut self) -> SFMSHD_W<4> {
        SFMSHD_W::new(self)
    }
    #[doc = "Bit 5 - QSSL Signal Setup Time"]
    #[inline(always)]
    #[must_use]
    pub fn sfmsld(&mut self) -> SFMSLD_W<5> {
        SFMSLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Chip Selection Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfmssc](index.html) module"]
pub struct SFMSSC_SPEC;
impl crate::RegisterSpec for SFMSSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfmssc::R](R) reader structure"]
impl crate::Readable for SFMSSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfmssc::W](W) writer structure"]
impl crate::Writable for SFMSSC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFMSSC to value 0x37"]
impl crate::Resettable for SFMSSC_SPEC {
    const RESET_VALUE: Self::Ux = 0x37;
}
