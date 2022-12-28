#[doc = "Register `SFMSDC` reader"]
pub struct R(crate::R<SFMSDC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFMSDC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFMSDC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFMSDC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFMSDC` writer"]
pub struct W(crate::W<SFMSDC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFMSDC_SPEC>;
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
impl From<crate::W<SFMSDC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFMSDC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFMDN` reader - Number of dummy cycles select for Fast Read instructions"]
pub type SFMDN_R = crate::FieldReader<u8, SFMDN_A>;
#[doc = "Number of dummy cycles select for Fast Read instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SFMDN_A {
    #[doc = "0: Default dummy cycles for each instruction: - Fast Read Quad I/O: 6 QSPCLK - Fast Read Quad Output: 8 QSPCLK - Fast Read Dual I/O: 4 QSPCLK - Fast Read Dual Output: 8 QSPCLK - Fast Read: 8 QSPCLK"]
    _0X0 = 0,
    #[doc = "1: 3 QSPCLK"]
    _0X1 = 1,
    #[doc = "2: 4 QSPCLK"]
    _0X2 = 2,
    #[doc = "3: 5 QSPCLK"]
    _0X3 = 3,
    #[doc = "4: 6 QSPCLK"]
    _0X4 = 4,
    #[doc = "5: 7 QSPCLK"]
    _0X5 = 5,
    #[doc = "6: 8 QSPCLK"]
    _0X6 = 6,
    #[doc = "7: 9 QSPCLK"]
    _0X7 = 7,
    #[doc = "8: 10 QSPCLK"]
    _0X8 = 8,
    #[doc = "9: 11 QSPCLK"]
    _0X9 = 9,
    #[doc = "10: 12 QSPCLK"]
    _0X_A = 10,
    #[doc = "11: 13 QSPCLK"]
    _0X_B = 11,
    #[doc = "12: 14 QSPCLK"]
    _0X_C = 12,
    #[doc = "13: 15 QSPCLK"]
    _0X_D = 13,
    #[doc = "14: 16 QSPCLK"]
    _0X_E = 14,
    #[doc = "15: 17 QSPCLK"]
    _0X_F = 15,
}
impl From<SFMDN_A> for u8 {
    #[inline(always)]
    fn from(variant: SFMDN_A) -> Self {
        variant as _
    }
}
impl SFMDN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFMDN_A {
        match self.bits {
            0 => SFMDN_A::_0X0,
            1 => SFMDN_A::_0X1,
            2 => SFMDN_A::_0X2,
            3 => SFMDN_A::_0X3,
            4 => SFMDN_A::_0X4,
            5 => SFMDN_A::_0X5,
            6 => SFMDN_A::_0X6,
            7 => SFMDN_A::_0X7,
            8 => SFMDN_A::_0X8,
            9 => SFMDN_A::_0X9,
            10 => SFMDN_A::_0X_A,
            11 => SFMDN_A::_0X_B,
            12 => SFMDN_A::_0X_C,
            13 => SFMDN_A::_0X_D,
            14 => SFMDN_A::_0X_E,
            15 => SFMDN_A::_0X_F,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == SFMDN_A::_0X0
    }
    #[doc = "Checks if the value of the field is `_0X1`"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == SFMDN_A::_0X1
    }
    #[doc = "Checks if the value of the field is `_0X2`"]
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == SFMDN_A::_0X2
    }
    #[doc = "Checks if the value of the field is `_0X3`"]
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == SFMDN_A::_0X3
    }
    #[doc = "Checks if the value of the field is `_0X4`"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == SFMDN_A::_0X4
    }
    #[doc = "Checks if the value of the field is `_0X5`"]
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == SFMDN_A::_0X5
    }
    #[doc = "Checks if the value of the field is `_0X6`"]
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == SFMDN_A::_0X6
    }
    #[doc = "Checks if the value of the field is `_0X7`"]
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == SFMDN_A::_0X7
    }
    #[doc = "Checks if the value of the field is `_0X8`"]
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == SFMDN_A::_0X8
    }
    #[doc = "Checks if the value of the field is `_0X9`"]
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == SFMDN_A::_0X9
    }
    #[doc = "Checks if the value of the field is `_0X_A`"]
    #[inline(always)]
    pub fn is_0x_a(&self) -> bool {
        *self == SFMDN_A::_0X_A
    }
    #[doc = "Checks if the value of the field is `_0X_B`"]
    #[inline(always)]
    pub fn is_0x_b(&self) -> bool {
        *self == SFMDN_A::_0X_B
    }
    #[doc = "Checks if the value of the field is `_0X_C`"]
    #[inline(always)]
    pub fn is_0x_c(&self) -> bool {
        *self == SFMDN_A::_0X_C
    }
    #[doc = "Checks if the value of the field is `_0X_D`"]
    #[inline(always)]
    pub fn is_0x_d(&self) -> bool {
        *self == SFMDN_A::_0X_D
    }
    #[doc = "Checks if the value of the field is `_0X_E`"]
    #[inline(always)]
    pub fn is_0x_e(&self) -> bool {
        *self == SFMDN_A::_0X_E
    }
    #[doc = "Checks if the value of the field is `_0X_F`"]
    #[inline(always)]
    pub fn is_0x_f(&self) -> bool {
        *self == SFMDN_A::_0X_F
    }
}
#[doc = "Field `SFMDN` writer - Number of dummy cycles select for Fast Read instructions"]
pub type SFMDN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SFMSDC_SPEC, u8, SFMDN_A, 4, O>;
impl<'a, const O: u8> SFMDN_W<'a, O> {
    #[doc = "Default dummy cycles for each instruction: - Fast Read Quad I/O: 6 QSPCLK - Fast Read Quad Output: 8 QSPCLK - Fast Read Dual I/O: 4 QSPCLK - Fast Read Dual Output: 8 QSPCLK - Fast Read: 8 QSPCLK"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(SFMDN_A::_0X0)
    }
    #[doc = "3 QSPCLK"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut W {
        self.variant(SFMDN_A::_0X1)
    }
    #[doc = "4 QSPCLK"]
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut W {
        self.variant(SFMDN_A::_0X2)
    }
    #[doc = "5 QSPCLK"]
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut W {
        self.variant(SFMDN_A::_0X3)
    }
    #[doc = "6 QSPCLK"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut W {
        self.variant(SFMDN_A::_0X4)
    }
    #[doc = "7 QSPCLK"]
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut W {
        self.variant(SFMDN_A::_0X5)
    }
    #[doc = "8 QSPCLK"]
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut W {
        self.variant(SFMDN_A::_0X6)
    }
    #[doc = "9 QSPCLK"]
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut W {
        self.variant(SFMDN_A::_0X7)
    }
    #[doc = "10 QSPCLK"]
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut W {
        self.variant(SFMDN_A::_0X8)
    }
    #[doc = "11 QSPCLK"]
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut W {
        self.variant(SFMDN_A::_0X9)
    }
    #[doc = "12 QSPCLK"]
    #[inline(always)]
    pub fn _0x_a(self) -> &'a mut W {
        self.variant(SFMDN_A::_0X_A)
    }
    #[doc = "13 QSPCLK"]
    #[inline(always)]
    pub fn _0x_b(self) -> &'a mut W {
        self.variant(SFMDN_A::_0X_B)
    }
    #[doc = "14 QSPCLK"]
    #[inline(always)]
    pub fn _0x_c(self) -> &'a mut W {
        self.variant(SFMDN_A::_0X_C)
    }
    #[doc = "15 QSPCLK"]
    #[inline(always)]
    pub fn _0x_d(self) -> &'a mut W {
        self.variant(SFMDN_A::_0X_D)
    }
    #[doc = "16 QSPCLK"]
    #[inline(always)]
    pub fn _0x_e(self) -> &'a mut W {
        self.variant(SFMDN_A::_0X_E)
    }
    #[doc = "17 QSPCLK"]
    #[inline(always)]
    pub fn _0x_f(self) -> &'a mut W {
        self.variant(SFMDN_A::_0X_F)
    }
}
#[doc = "Field `SFMXST` reader - XIP mode status"]
pub type SFMXST_R = crate::BitReader<SFMXST_A>;
#[doc = "XIP mode status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMXST_A {
    #[doc = "0: Normal (non-XIP) mode"]
    _0 = 0,
    #[doc = "1: XIP mode"]
    _1 = 1,
}
impl From<SFMXST_A> for bool {
    #[inline(always)]
    fn from(variant: SFMXST_A) -> Self {
        variant as u8 != 0
    }
}
impl SFMXST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFMXST_A {
        match self.bits {
            false => SFMXST_A::_0,
            true => SFMXST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMXST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMXST_A::_1
    }
}
#[doc = "Field `SFMXEN` reader - XIP mode permission"]
pub type SFMXEN_R = crate::BitReader<SFMXEN_A>;
#[doc = "XIP mode permission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMXEN_A {
    #[doc = "0: Prohibit XIP mode"]
    _0 = 0,
    #[doc = "1: Permit XIP mode"]
    _1 = 1,
}
impl From<SFMXEN_A> for bool {
    #[inline(always)]
    fn from(variant: SFMXEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SFMXEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFMXEN_A {
        match self.bits {
            false => SFMXEN_A::_0,
            true => SFMXEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMXEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMXEN_A::_1
    }
}
#[doc = "Field `SFMXEN` writer - XIP mode permission"]
pub type SFMXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFMSDC_SPEC, SFMXEN_A, O>;
impl<'a, const O: u8> SFMXEN_W<'a, O> {
    #[doc = "Prohibit XIP mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SFMXEN_A::_0)
    }
    #[doc = "Permit XIP mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SFMXEN_A::_1)
    }
}
#[doc = "Field `SFMXD` reader - Mode data for serial flash (Controls XIP mode.)"]
pub type SFMXD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SFMXD` writer - Mode data for serial flash (Controls XIP mode.)"]
pub type SFMXD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SFMSDC_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:3 - Number of dummy cycles select for Fast Read instructions"]
    #[inline(always)]
    pub fn sfmdn(&self) -> SFMDN_R {
        SFMDN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - XIP mode status"]
    #[inline(always)]
    pub fn sfmxst(&self) -> SFMXST_R {
        SFMXST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - XIP mode permission"]
    #[inline(always)]
    pub fn sfmxen(&self) -> SFMXEN_R {
        SFMXEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Mode data for serial flash (Controls XIP mode.)"]
    #[inline(always)]
    pub fn sfmxd(&self) -> SFMXD_R {
        SFMXD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Number of dummy cycles select for Fast Read instructions"]
    #[inline(always)]
    #[must_use]
    pub fn sfmdn(&mut self) -> SFMDN_W<0> {
        SFMDN_W::new(self)
    }
    #[doc = "Bit 7 - XIP mode permission"]
    #[inline(always)]
    #[must_use]
    pub fn sfmxen(&mut self) -> SFMXEN_W<7> {
        SFMXEN_W::new(self)
    }
    #[doc = "Bits 8:15 - Mode data for serial flash (Controls XIP mode.)"]
    #[inline(always)]
    #[must_use]
    pub fn sfmxd(&mut self) -> SFMXD_W<8> {
        SFMXD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dummy Cycle Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfmsdc](index.html) module"]
pub struct SFMSDC_SPEC;
impl crate::RegisterSpec for SFMSDC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfmsdc::R](R) reader structure"]
impl crate::Readable for SFMSDC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfmsdc::W](W) writer structure"]
impl crate::Writable for SFMSDC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFMSDC to value 0xff00"]
impl crate::Resettable for SFMSDC_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00;
}
