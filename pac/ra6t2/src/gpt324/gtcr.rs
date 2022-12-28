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
#[doc = "Field `ICDS` reader - Input Capture Operation Select During Count Stop"]
pub type ICDS_R = crate::BitReader<ICDS_A>;
#[doc = "Input Capture Operation Select During Count Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICDS_A {
    #[doc = "0: Input capture is operated during count stop."]
    _0 = 0,
    #[doc = "1: Input capture is not operated during count stop."]
    _1 = 1,
}
impl From<ICDS_A> for bool {
    #[inline(always)]
    fn from(variant: ICDS_A) -> Self {
        variant as u8 != 0
    }
}
impl ICDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICDS_A {
        match self.bits {
            false => ICDS_A::_0,
            true => ICDS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICDS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICDS_A::_1
    }
}
#[doc = "Field `ICDS` writer - Input Capture Operation Select During Count Stop"]
pub type ICDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCR_SPEC, ICDS_A, O>;
impl<'a, const O: u8> ICDS_W<'a, O> {
    #[doc = "Input capture is operated during count stop."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICDS_A::_0)
    }
    #[doc = "Input capture is not operated during count stop."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICDS_A::_1)
    }
}
#[doc = "Field `SCGTIOC` reader - GTIOC input Source Synchronous Clear Enable"]
pub type SCGTIOC_R = crate::BitReader<SCGTIOC_A>;
#[doc = "GTIOC input Source Synchronous Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCGTIOC_A {
    #[doc = "0: Disables to use the counter clear by GTIOC input as the clear factor for other channels"]
    _0 = 0,
    #[doc = "1: Enables to use the counter clear by GTIOC input as the clear factor for other channels"]
    _1 = 1,
}
impl From<SCGTIOC_A> for bool {
    #[inline(always)]
    fn from(variant: SCGTIOC_A) -> Self {
        variant as u8 != 0
    }
}
impl SCGTIOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCGTIOC_A {
        match self.bits {
            false => SCGTIOC_A::_0,
            true => SCGTIOC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCGTIOC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCGTIOC_A::_1
    }
}
#[doc = "Field `SCGTIOC` writer - GTIOC input Source Synchronous Clear Enable"]
pub type SCGTIOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCR_SPEC, SCGTIOC_A, O>;
impl<'a, const O: u8> SCGTIOC_W<'a, O> {
    #[doc = "Disables to use the counter clear by GTIOC input as the clear factor for other channels"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCGTIOC_A::_0)
    }
    #[doc = "Enables to use the counter clear by GTIOC input as the clear factor for other channels"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCGTIOC_A::_1)
    }
}
#[doc = "Field `SSCGRP` reader - Synchronous Set/Clear Group Select"]
pub type SSCGRP_R = crate::FieldReader<u8, SSCGRP_A>;
#[doc = "Synchronous Set/Clear Group Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SSCGRP_A {
    #[doc = "0: Select synchronous set/clear group A"]
    _00 = 0,
    #[doc = "1: Select synchronous set/clear group B"]
    _01 = 1,
    #[doc = "2: Select synchronous set/clear group C"]
    _10 = 2,
    #[doc = "3: Select synchronous set/clear group D"]
    _11 = 3,
}
impl From<SSCGRP_A> for u8 {
    #[inline(always)]
    fn from(variant: SSCGRP_A) -> Self {
        variant as _
    }
}
impl SSCGRP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSCGRP_A {
        match self.bits {
            0 => SSCGRP_A::_00,
            1 => SSCGRP_A::_01,
            2 => SSCGRP_A::_10,
            3 => SSCGRP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SSCGRP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SSCGRP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SSCGRP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SSCGRP_A::_11
    }
}
#[doc = "Field `SSCGRP` writer - Synchronous Set/Clear Group Select"]
pub type SSCGRP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTCR_SPEC, u8, SSCGRP_A, 2, O>;
impl<'a, const O: u8> SSCGRP_W<'a, O> {
    #[doc = "Select synchronous set/clear group A"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SSCGRP_A::_00)
    }
    #[doc = "Select synchronous set/clear group B"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SSCGRP_A::_01)
    }
    #[doc = "Select synchronous set/clear group C"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SSCGRP_A::_10)
    }
    #[doc = "Select synchronous set/clear group D"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SSCGRP_A::_11)
    }
}
#[doc = "Field `CPSCD` reader - Complementary PWM Mode Synchronous Clear Disable"]
pub type CPSCD_R = crate::BitReader<CPSCD_A>;
#[doc = "Complementary PWM Mode Synchronous Clear Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPSCD_A {
    #[doc = "0: Enable synchronous counter clear by other channel other than the section of trough in complementary PWM mode"]
    _0 = 0,
    #[doc = "1: Disable synchronous counter clear by other channel other than the section of trough in complementary PWM mode"]
    _1 = 1,
}
impl From<CPSCD_A> for bool {
    #[inline(always)]
    fn from(variant: CPSCD_A) -> Self {
        variant as u8 != 0
    }
}
impl CPSCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPSCD_A {
        match self.bits {
            false => CPSCD_A::_0,
            true => CPSCD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPSCD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPSCD_A::_1
    }
}
#[doc = "Field `CPSCD` writer - Complementary PWM Mode Synchronous Clear Disable"]
pub type CPSCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCR_SPEC, CPSCD_A, O>;
impl<'a, const O: u8> CPSCD_W<'a, O> {
    #[doc = "Enable synchronous counter clear by other channel other than the section of trough in complementary PWM mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPSCD_A::_0)
    }
    #[doc = "Disable synchronous counter clear by other channel other than the section of trough in complementary PWM mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPSCD_A::_1)
    }
}
#[doc = "Field `SSCEN` reader - Synchronous Set/Clear Enable"]
pub type SSCEN_R = crate::BitReader<SSCEN_A>;
#[doc = "Synchronous Set/Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSCEN_A {
    #[doc = "0: Disable Synchronous set/clear of the GTCNT counter"]
    _0 = 0,
    #[doc = "1: Enable Synchronous set/clear of the GTCNT counter"]
    _1 = 1,
}
impl From<SSCEN_A> for bool {
    #[inline(always)]
    fn from(variant: SSCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SSCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSCEN_A {
        match self.bits {
            false => SSCEN_A::_0,
            true => SSCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSCEN_A::_1
    }
}
#[doc = "Field `SSCEN` writer - Synchronous Set/Clear Enable"]
pub type SSCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCR_SPEC, SSCEN_A, O>;
impl<'a, const O: u8> SSCEN_W<'a, O> {
    #[doc = "Disable Synchronous set/clear of the GTCNT counter"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSCEN_A::_0)
    }
    #[doc = "Enable Synchronous set/clear of the GTCNT counter"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSCEN_A::_1)
    }
}
#[doc = "Field `MD` reader - Mode Select"]
pub type MD_R = crate::FieldReader<u8, MD_A>;
#[doc = "Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MD_A {
    #[doc = "0: Saw-wave PWM mode 1(single buffer or double buffer possible)"]
    _0000 = 0,
    #[doc = "1: Saw-wave one-shot pulse mode (fixed buffer operation)"]
    _0001 = 1,
    #[doc = "2: Saw-wave PWM mode 2(single buffer or double buffer possible)"]
    _0010 = 2,
    #[doc = "3: Setting prohibited"]
    _0011 = 3,
    #[doc = "4: Triangle-wave PWM mode 1 (32-bit transfer at trough) (single buffer or double buffer possible)"]
    _0100 = 4,
    #[doc = "5: Triangle-wave PWM mode 2 (32-bit transfer at crest and trough) (single buffer or double buffer possible)"]
    _0101 = 5,
    #[doc = "6: Triangle-wave PWM mode 3 (64-bit transfer at trough) (fixed buffer operation)"]
    _0110 = 6,
    #[doc = "7: Setting prohibited"]
    _0111 = 7,
    #[doc = "8: Setting prohibited"]
    _1000 = 8,
    #[doc = "9: Setting prohibited"]
    _1001 = 9,
    #[doc = "10: Setting prohibited"]
    _1010 = 10,
    #[doc = "11: Setting prohibited"]
    _1011 = 11,
    #[doc = "12: Complementary PWM mode 1(transfer at crest)"]
    _1100 = 12,
    #[doc = "13: Complementary PWM mode 2(transfer at trough)"]
    _1101 = 13,
    #[doc = "14: Complementary PWM mode 3(transfer at crest and trough)"]
    _1110 = 14,
    #[doc = "15: Complementary PWM mode 4(immediate transfer)"]
    _1111 = 15,
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
            0 => MD_A::_0000,
            1 => MD_A::_0001,
            2 => MD_A::_0010,
            3 => MD_A::_0011,
            4 => MD_A::_0100,
            5 => MD_A::_0101,
            6 => MD_A::_0110,
            7 => MD_A::_0111,
            8 => MD_A::_1000,
            9 => MD_A::_1001,
            10 => MD_A::_1010,
            11 => MD_A::_1011,
            12 => MD_A::_1100,
            13 => MD_A::_1101,
            14 => MD_A::_1110,
            15 => MD_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == MD_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == MD_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == MD_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == MD_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == MD_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == MD_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == MD_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == MD_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == MD_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == MD_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == MD_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == MD_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == MD_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == MD_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == MD_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == MD_A::_1111
    }
}
#[doc = "Field `MD` writer - Mode Select"]
pub type MD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTCR_SPEC, u8, MD_A, 4, O>;
impl<'a, const O: u8> MD_W<'a, O> {
    #[doc = "Saw-wave PWM mode 1(single buffer or double buffer possible)"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(MD_A::_0000)
    }
    #[doc = "Saw-wave one-shot pulse mode (fixed buffer operation)"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(MD_A::_0001)
    }
    #[doc = "Saw-wave PWM mode 2(single buffer or double buffer possible)"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(MD_A::_0010)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(MD_A::_0011)
    }
    #[doc = "Triangle-wave PWM mode 1 (32-bit transfer at trough) (single buffer or double buffer possible)"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(MD_A::_0100)
    }
    #[doc = "Triangle-wave PWM mode 2 (32-bit transfer at crest and trough) (single buffer or double buffer possible)"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(MD_A::_0101)
    }
    #[doc = "Triangle-wave PWM mode 3 (64-bit transfer at trough) (fixed buffer operation)"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(MD_A::_0110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(MD_A::_0111)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(MD_A::_1000)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(MD_A::_1001)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(MD_A::_1010)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(MD_A::_1011)
    }
    #[doc = "Complementary PWM mode 1(transfer at crest)"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(MD_A::_1100)
    }
    #[doc = "Complementary PWM mode 2(transfer at trough)"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(MD_A::_1101)
    }
    #[doc = "Complementary PWM mode 3(transfer at crest and trough)"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(MD_A::_1110)
    }
    #[doc = "Complementary PWM mode 4(immediate transfer)"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(MD_A::_1111)
    }
}
#[doc = "Field `TPCS` reader - Timer Prescaler Select"]
pub type TPCS_R = crate::FieldReader<u8, TPCS_A>;
#[doc = "Timer Prescaler Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TPCS_A {
    #[doc = "0: GTCLK/1"]
    _0000 = 0,
    #[doc = "1: GTCLK/2"]
    _0001 = 1,
    #[doc = "2: GTCLK/4"]
    _0010 = 2,
    #[doc = "3: GTCLK/8"]
    _0011 = 3,
    #[doc = "4: GTCLK/16"]
    _0100 = 4,
    #[doc = "5: GTCLK/32"]
    _0101 = 5,
    #[doc = "6: GTCLK/64"]
    _0110 = 6,
    #[doc = "7: GTCLK/128"]
    _0111 = 7,
    #[doc = "8: GTCLK/256"]
    _1000 = 8,
    #[doc = "9: GTCLK/512"]
    _1001 = 9,
    #[doc = "10: GTCLK/1024"]
    _1010 = 10,
    #[doc = "11: Setting prohibited"]
    _1011 = 11,
    #[doc = "12: GTETRGA (Via the POEG)"]
    _1100 = 12,
    #[doc = "13: GTETRGB (Via the POEG)"]
    _1101 = 13,
    #[doc = "14: GTETRGC (Via the POEG)"]
    _1110 = 14,
    #[doc = "15: GTETRGD (Via the POEG)POEGæ\u{8e}¥ç¶\u{9a}æ\u{95}°ã\u{81}«å¿\u{9c}ã\u{81}\u{98}ã\u{81}\u{9f}ã\u{80}\u{81}GTETRGA-Dè¡¨ç¤ºå\u{88}¶å¾¡å¿ è¦\u{81}ã\u{80}\u{82}"]
    _1111 = 15,
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
            0 => TPCS_A::_0000,
            1 => TPCS_A::_0001,
            2 => TPCS_A::_0010,
            3 => TPCS_A::_0011,
            4 => TPCS_A::_0100,
            5 => TPCS_A::_0101,
            6 => TPCS_A::_0110,
            7 => TPCS_A::_0111,
            8 => TPCS_A::_1000,
            9 => TPCS_A::_1001,
            10 => TPCS_A::_1010,
            11 => TPCS_A::_1011,
            12 => TPCS_A::_1100,
            13 => TPCS_A::_1101,
            14 => TPCS_A::_1110,
            15 => TPCS_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == TPCS_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == TPCS_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == TPCS_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == TPCS_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == TPCS_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == TPCS_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == TPCS_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == TPCS_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == TPCS_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == TPCS_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == TPCS_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == TPCS_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == TPCS_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == TPCS_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == TPCS_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == TPCS_A::_1111
    }
}
#[doc = "Field `TPCS` writer - Timer Prescaler Select"]
pub type TPCS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTCR_SPEC, u8, TPCS_A, 4, O>;
impl<'a, const O: u8> TPCS_W<'a, O> {
    #[doc = "GTCLK/1"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(TPCS_A::_0000)
    }
    #[doc = "GTCLK/2"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(TPCS_A::_0001)
    }
    #[doc = "GTCLK/4"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(TPCS_A::_0010)
    }
    #[doc = "GTCLK/8"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(TPCS_A::_0011)
    }
    #[doc = "GTCLK/16"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(TPCS_A::_0100)
    }
    #[doc = "GTCLK/32"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(TPCS_A::_0101)
    }
    #[doc = "GTCLK/64"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(TPCS_A::_0110)
    }
    #[doc = "GTCLK/128"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(TPCS_A::_0111)
    }
    #[doc = "GTCLK/256"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(TPCS_A::_1000)
    }
    #[doc = "GTCLK/512"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(TPCS_A::_1001)
    }
    #[doc = "GTCLK/1024"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(TPCS_A::_1010)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(TPCS_A::_1011)
    }
    #[doc = "GTETRGA (Via the POEG)"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(TPCS_A::_1100)
    }
    #[doc = "GTETRGB (Via the POEG)"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(TPCS_A::_1101)
    }
    #[doc = "GTETRGC (Via the POEG)"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(TPCS_A::_1110)
    }
    #[doc = "GTETRGD (Via the POEG)POEGæ\u{8e}¥ç¶\u{9a}æ\u{95}°ã\u{81}«å¿\u{9c}ã\u{81}\u{98}ã\u{81}\u{9f}ã\u{80}\u{81}GTETRGA-Dè¡¨ç¤ºå\u{88}¶å¾¡å¿ è¦\u{81}ã\u{80}\u{82}"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(TPCS_A::_1111)
    }
}
#[doc = "Field `CKEG` reader - Clock Edge Select"]
pub type CKEG_R = crate::FieldReader<u8, CKEG_A>;
#[doc = "Clock Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKEG_A {
    #[doc = "0: Select rising edge of GTETRG for clock count"]
    _00 = 0,
    #[doc = "1: Select falling edge of GTETRG for clock count"]
    _01 = 1,
}
impl From<CKEG_A> for u8 {
    #[inline(always)]
    fn from(variant: CKEG_A) -> Self {
        variant as _
    }
}
impl CKEG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKEG_A> {
        match self.bits {
            0 => Some(CKEG_A::_00),
            1 => Some(CKEG_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CKEG_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CKEG_A::_01
    }
}
#[doc = "Field `CKEG` writer - Clock Edge Select"]
pub type CKEG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTCR_SPEC, u8, CKEG_A, 2, O>;
impl<'a, const O: u8> CKEG_W<'a, O> {
    #[doc = "Select rising edge of GTETRG for clock count"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CKEG_A::_00)
    }
    #[doc = "Select falling edge of GTETRG for clock count"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CKEG_A::_01)
    }
}
impl R {
    #[doc = "Bit 0 - Count Start"]
    #[inline(always)]
    pub fn cst(&self) -> CST_R {
        CST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Input Capture Operation Select During Count Stop"]
    #[inline(always)]
    pub fn icds(&self) -> ICDS_R {
        ICDS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GTIOC input Source Synchronous Clear Enable"]
    #[inline(always)]
    pub fn scgtioc(&self) -> SCGTIOC_R {
        SCGTIOC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Synchronous Set/Clear Group Select"]
    #[inline(always)]
    pub fn sscgrp(&self) -> SSCGRP_R {
        SSCGRP_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Complementary PWM Mode Synchronous Clear Disable"]
    #[inline(always)]
    pub fn cpscd(&self) -> CPSCD_R {
        CPSCD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Synchronous Set/Clear Enable"]
    #[inline(always)]
    pub fn sscen(&self) -> SSCEN_R {
        SSCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Mode Select"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 23:26 - Timer Prescaler Select"]
    #[inline(always)]
    pub fn tpcs(&self) -> TPCS_R {
        TPCS_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 27:28 - Clock Edge Select"]
    #[inline(always)]
    pub fn ckeg(&self) -> CKEG_R {
        CKEG_R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Count Start"]
    #[inline(always)]
    #[must_use]
    pub fn cst(&mut self) -> CST_W<0> {
        CST_W::new(self)
    }
    #[doc = "Bit 8 - Input Capture Operation Select During Count Stop"]
    #[inline(always)]
    #[must_use]
    pub fn icds(&mut self) -> ICDS_W<8> {
        ICDS_W::new(self)
    }
    #[doc = "Bit 9 - GTIOC input Source Synchronous Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scgtioc(&mut self) -> SCGTIOC_W<9> {
        SCGTIOC_W::new(self)
    }
    #[doc = "Bits 10:11 - Synchronous Set/Clear Group Select"]
    #[inline(always)]
    #[must_use]
    pub fn sscgrp(&mut self) -> SSCGRP_W<10> {
        SSCGRP_W::new(self)
    }
    #[doc = "Bit 12 - Complementary PWM Mode Synchronous Clear Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cpscd(&mut self) -> CPSCD_W<12> {
        CPSCD_W::new(self)
    }
    #[doc = "Bit 15 - Synchronous Set/Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sscen(&mut self) -> SSCEN_W<15> {
        SSCEN_W::new(self)
    }
    #[doc = "Bits 16:19 - Mode Select"]
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
    #[doc = "Bits 27:28 - Clock Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn ckeg(&mut self) -> CKEG_W<27> {
        CKEG_W::new(self)
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
