#[doc = "Register `GTCR` reader"]
pub struct R(crate::R<GTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTCR` writer"]
pub struct W(crate::W<GTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTCR_SPEC>;
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
impl From<crate::W<GTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CST` reader - Count Start"]
pub type CST_R = crate::BitReader<CST_A>;
#[doc = "Count Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CST_A {
    #[doc = "0: Count operation is stopped"]
    _0 = 0,
    #[doc = "1: Count operation is performed"]
    _1 = 1,
}
impl From<CST_A> for bool {
    #[inline(always)]
    fn from(variant: CST_A) -> Self {
        variant as u8 != 0
    }
}
impl CST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CST_A {
        match self.bits {
            false => CST_A::_0,
            true => CST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CST_A::_1
    }
}
#[doc = "Field `CST` writer - Count Start"]
pub type CST_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCR_SPEC, CST_A, O>;
impl<'a, const O: u8> CST_W<'a, O> {
    #[doc = "Count operation is stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CST_A::_0)
    }
    #[doc = "Count operation is performed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CST_A::_1)
    }
}
#[doc = "Field `MD` reader - Mode Select"]
pub type MD_R = crate::FieldReader<u8, MD_A>;
#[doc = "Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MD_A {
    #[doc = "0: Saw-wave PWM mode (single buffer or double buffer possible)"]
    _000 = 0,
    #[doc = "1: Saw-wave one-shot pulse mode (fixed buffer operation)"]
    _001 = 1,
    #[doc = "2: Setting prohibited"]
    _010 = 2,
    #[doc = "3: Setting prohibited"]
    _011 = 3,
    #[doc = "4: Triangle-wave PWM mode 1 (32-bit transfer at trough) (single buffer or double buffer is possible)"]
    _100 = 4,
    #[doc = "5: Triangle-wave PWM mode 2 (32-bit transfer at crest and trough) (single buffer or double buffer is possible)"]
    _101 = 5,
    #[doc = "6: Triangle-wave PWM mode 3 (64-bit transfer at trough) (fixed buffer operation)"]
    _110 = 6,
    #[doc = "7: Setting prohibited"]
    _111 = 7,
}
impl From<MD_A> for u8 {
    #[inline(always)]
    fn from(variant: MD_A) -> Self {
        variant as _
    }
}
impl MD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MD_A {
        match self.bits {
            0 => MD_A::_000,
            1 => MD_A::_001,
            2 => MD_A::_010,
            3 => MD_A::_011,
            4 => MD_A::_100,
            5 => MD_A::_101,
            6 => MD_A::_110,
            7 => MD_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == MD_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == MD_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == MD_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == MD_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == MD_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == MD_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == MD_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == MD_A::_111
    }
}
#[doc = "Field `MD` writer - Mode Select"]
pub type MD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTCR_SPEC, u8, MD_A, 3, O>;
impl<'a, const O: u8> MD_W<'a, O> {
    #[doc = "Saw-wave PWM mode (single buffer or double buffer possible)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(MD_A::_000)
    }
    #[doc = "Saw-wave one-shot pulse mode (fixed buffer operation)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(MD_A::_001)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(MD_A::_010)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(MD_A::_011)
    }
    #[doc = "Triangle-wave PWM mode 1 (32-bit transfer at trough) (single buffer or double buffer is possible)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(MD_A::_100)
    }
    #[doc = "Triangle-wave PWM mode 2 (32-bit transfer at crest and trough) (single buffer or double buffer is possible)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(MD_A::_101)
    }
    #[doc = "Triangle-wave PWM mode 3 (64-bit transfer at trough) (fixed buffer operation)"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(MD_A::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(MD_A::_111)
    }
}
#[doc = "Field `TPCS` reader - Timer Prescaler Select"]
pub type TPCS_R = crate::FieldReader<u8, TPCS_A>;
#[doc = "Timer Prescaler Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TPCS_A {
    #[doc = "0: PCLKD/1"]
    _0X0 = 0,
    #[doc = "1: PCLKD/2"]
    _0X1 = 1,
    #[doc = "2: PCLKD/4"]
    _0X2 = 2,
    #[doc = "3: PCLKD/8"]
    _0X3 = 3,
    #[doc = "4: PCLKD/16"]
    _0X4 = 4,
    #[doc = "5: PCLKD/32"]
    _0X5 = 5,
    #[doc = "6: PCLKD/64"]
    _0X6 = 6,
    #[doc = "7: Setting prohibited"]
    _0X7 = 7,
    #[doc = "8: PCLKD/256"]
    _0X8 = 8,
    #[doc = "9: Setting prohibited"]
    _0X9 = 9,
    #[doc = "10: PCLKD/1024"]
    _0X_A = 10,
    #[doc = "11: Setting prohibited"]
    _0X_B = 11,
    #[doc = "12: GTETRGA (Via the POEG)"]
    _0X_C = 12,
    #[doc = "13: GTETRGB (Via the POEG)"]
    _0X_D = 13,
    #[doc = "14: GTETRGC (Via the POEG)"]
    _0X_E = 14,
    #[doc = "15: GTETRGD (Via the POEG)"]
    _0X_F = 15,
}
impl From<TPCS_A> for u8 {
    #[inline(always)]
    fn from(variant: TPCS_A) -> Self {
        variant as _
    }
}
impl TPCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPCS_A {
        match self.bits {
            0 => TPCS_A::_0X0,
            1 => TPCS_A::_0X1,
            2 => TPCS_A::_0X2,
            3 => TPCS_A::_0X3,
            4 => TPCS_A::_0X4,
            5 => TPCS_A::_0X5,
            6 => TPCS_A::_0X6,
            7 => TPCS_A::_0X7,
            8 => TPCS_A::_0X8,
            9 => TPCS_A::_0X9,
            10 => TPCS_A::_0X_A,
            11 => TPCS_A::_0X_B,
            12 => TPCS_A::_0X_C,
            13 => TPCS_A::_0X_D,
            14 => TPCS_A::_0X_E,
            15 => TPCS_A::_0X_F,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == TPCS_A::_0X0
    }
    #[doc = "Checks if the value of the field is `_0X1`"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == TPCS_A::_0X1
    }
    #[doc = "Checks if the value of the field is `_0X2`"]
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == TPCS_A::_0X2
    }
    #[doc = "Checks if the value of the field is `_0X3`"]
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == TPCS_A::_0X3
    }
    #[doc = "Checks if the value of the field is `_0X4`"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == TPCS_A::_0X4
    }
    #[doc = "Checks if the value of the field is `_0X5`"]
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == TPCS_A::_0X5
    }
    #[doc = "Checks if the value of the field is `_0X6`"]
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == TPCS_A::_0X6
    }
    #[doc = "Checks if the value of the field is `_0X7`"]
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == TPCS_A::_0X7
    }
    #[doc = "Checks if the value of the field is `_0X8`"]
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == TPCS_A::_0X8
    }
    #[doc = "Checks if the value of the field is `_0X9`"]
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == TPCS_A::_0X9
    }
    #[doc = "Checks if the value of the field is `_0X_A`"]
    #[inline(always)]
    pub fn is_0x_a(&self) -> bool {
        *self == TPCS_A::_0X_A
    }
    #[doc = "Checks if the value of the field is `_0X_B`"]
    #[inline(always)]
    pub fn is_0x_b(&self) -> bool {
        *self == TPCS_A::_0X_B
    }
    #[doc = "Checks if the value of the field is `_0X_C`"]
    #[inline(always)]
    pub fn is_0x_c(&self) -> bool {
        *self == TPCS_A::_0X_C
    }
    #[doc = "Checks if the value of the field is `_0X_D`"]
    #[inline(always)]
    pub fn is_0x_d(&self) -> bool {
        *self == TPCS_A::_0X_D
    }
    #[doc = "Checks if the value of the field is `_0X_E`"]
    #[inline(always)]
    pub fn is_0x_e(&self) -> bool {
        *self == TPCS_A::_0X_E
    }
    #[doc = "Checks if the value of the field is `_0X_F`"]
    #[inline(always)]
    pub fn is_0x_f(&self) -> bool {
        *self == TPCS_A::_0X_F
    }
}
#[doc = "Field `TPCS` writer - Timer Prescaler Select"]
pub type TPCS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTCR_SPEC, u8, TPCS_A, 4, O>;
impl<'a, const O: u8> TPCS_W<'a, O> {
    #[doc = "PCLKD/1"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(TPCS_A::_0X0)
    }
    #[doc = "PCLKD/2"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut W {
        self.variant(TPCS_A::_0X1)
    }
    #[doc = "PCLKD/4"]
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut W {
        self.variant(TPCS_A::_0X2)
    }
    #[doc = "PCLKD/8"]
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut W {
        self.variant(TPCS_A::_0X3)
    }
    #[doc = "PCLKD/16"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut W {
        self.variant(TPCS_A::_0X4)
    }
    #[doc = "PCLKD/32"]
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut W {
        self.variant(TPCS_A::_0X5)
    }
    #[doc = "PCLKD/64"]
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut W {
        self.variant(TPCS_A::_0X6)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut W {
        self.variant(TPCS_A::_0X7)
    }
    #[doc = "PCLKD/256"]
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut W {
        self.variant(TPCS_A::_0X8)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut W {
        self.variant(TPCS_A::_0X9)
    }
    #[doc = "PCLKD/1024"]
    #[inline(always)]
    pub fn _0x_a(self) -> &'a mut W {
        self.variant(TPCS_A::_0X_A)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _0x_b(self) -> &'a mut W {
        self.variant(TPCS_A::_0X_B)
    }
    #[doc = "GTETRGA (Via the POEG)"]
    #[inline(always)]
    pub fn _0x_c(self) -> &'a mut W {
        self.variant(TPCS_A::_0X_C)
    }
    #[doc = "GTETRGB (Via the POEG)"]
    #[inline(always)]
    pub fn _0x_d(self) -> &'a mut W {
        self.variant(TPCS_A::_0X_D)
    }
    #[doc = "GTETRGC (Via the POEG)"]
    #[inline(always)]
    pub fn _0x_e(self) -> &'a mut W {
        self.variant(TPCS_A::_0X_E)
    }
    #[doc = "GTETRGD (Via the POEG)"]
    #[inline(always)]
    pub fn _0x_f(self) -> &'a mut W {
        self.variant(TPCS_A::_0X_F)
    }
}
impl R {
    #[doc = "Bit 0 - Count Start"]
    #[inline(always)]
    pub fn cst(&self) -> CST_R {
        CST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:18 - Mode Select"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 23:26 - Timer Prescaler Select"]
    #[inline(always)]
    pub fn tpcs(&self) -> TPCS_R {
        TPCS_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Count Start"]
    #[inline(always)]
    #[must_use]
    pub fn cst(&mut self) -> CST_W<0> {
        CST_W::new(self)
    }
    #[doc = "Bits 16:18 - Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn md(&mut self) -> MD_W<16> {
        MD_W::new(self)
    }
    #[doc = "Bits 23:26 - Timer Prescaler Select"]
    #[inline(always)]
    #[must_use]
    pub fn tpcs(&mut self) -> TPCS_W<23> {
        TPCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtcr](index.html) module"]
pub struct GTCR_SPEC;
impl crate::RegisterSpec for GTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtcr::R](R) reader structure"]
impl crate::Readable for GTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtcr::W](W) writer structure"]
impl crate::Writable for GTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTCR to value 0"]
impl crate::Resettable for GTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
