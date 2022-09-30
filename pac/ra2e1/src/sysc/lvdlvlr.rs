#[doc = "Register `LVDLVLR` reader"]
pub struct R(crate::R<LVDLVLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LVDLVLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LVDLVLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LVDLVLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LVDLVLR` writer"]
pub struct W(crate::W<LVDLVLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LVDLVLR_SPEC>;
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
impl From<crate::W<LVDLVLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LVDLVLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVD1LVL` reader - Voltage Detection 1 Level Select (Standard voltage during fall in voltage)"]
pub type LVD1LVL_R = crate::FieldReader<u8, LVD1LVL_A>;
#[doc = "Voltage Detection 1 Level Select (Standard voltage during fall in voltage)\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LVD1LVL_A {
    #[doc = "0: Vdet1_0"]
    _0X00 = 0,
    #[doc = "1: Vdet1_1"]
    _0X01 = 1,
    #[doc = "2: Vdet1_2"]
    _0X02 = 2,
    #[doc = "3: Vdet1_3"]
    _0X03 = 3,
    #[doc = "4: Vdet1_4"]
    _0X04 = 4,
    #[doc = "5: Vdet1_5"]
    _0X05 = 5,
    #[doc = "6: Vdet1_6"]
    _0X06 = 6,
    #[doc = "7: Vdet1_7"]
    _0X07 = 7,
    #[doc = "8: Vdet1_8"]
    _0X08 = 8,
    #[doc = "9: Vdet1_9"]
    _0X09 = 9,
    #[doc = "10: Vdet1_A"]
    _0X0A = 10,
    #[doc = "11: Vdet1_B"]
    _0X0B = 11,
    #[doc = "12: Vdet1_C"]
    _0X0C = 12,
    #[doc = "13: Vdet1_D"]
    _0X0D = 13,
    #[doc = "14: Vdet1_E"]
    _0X0E = 14,
    #[doc = "15: Vdet1_F"]
    _0X0F = 15,
}
impl From<LVD1LVL_A> for u8 {
    #[inline(always)]
    fn from(variant: LVD1LVL_A) -> Self {
        variant as _
    }
}
impl LVD1LVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LVD1LVL_A> {
        match self.bits {
            0 => Some(LVD1LVL_A::_0X00),
            1 => Some(LVD1LVL_A::_0X01),
            2 => Some(LVD1LVL_A::_0X02),
            3 => Some(LVD1LVL_A::_0X03),
            4 => Some(LVD1LVL_A::_0X04),
            5 => Some(LVD1LVL_A::_0X05),
            6 => Some(LVD1LVL_A::_0X06),
            7 => Some(LVD1LVL_A::_0X07),
            8 => Some(LVD1LVL_A::_0X08),
            9 => Some(LVD1LVL_A::_0X09),
            10 => Some(LVD1LVL_A::_0X0A),
            11 => Some(LVD1LVL_A::_0X0B),
            12 => Some(LVD1LVL_A::_0X0C),
            13 => Some(LVD1LVL_A::_0X0D),
            14 => Some(LVD1LVL_A::_0X0E),
            15 => Some(LVD1LVL_A::_0X0F),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == LVD1LVL_A::_0X00
    }
    #[doc = "Checks if the value of the field is `_0X01`"]
    #[inline(always)]
    pub fn is_0x01(&self) -> bool {
        *self == LVD1LVL_A::_0X01
    }
    #[doc = "Checks if the value of the field is `_0X02`"]
    #[inline(always)]
    pub fn is_0x02(&self) -> bool {
        *self == LVD1LVL_A::_0X02
    }
    #[doc = "Checks if the value of the field is `_0X03`"]
    #[inline(always)]
    pub fn is_0x03(&self) -> bool {
        *self == LVD1LVL_A::_0X03
    }
    #[doc = "Checks if the value of the field is `_0X04`"]
    #[inline(always)]
    pub fn is_0x04(&self) -> bool {
        *self == LVD1LVL_A::_0X04
    }
    #[doc = "Checks if the value of the field is `_0X05`"]
    #[inline(always)]
    pub fn is_0x05(&self) -> bool {
        *self == LVD1LVL_A::_0X05
    }
    #[doc = "Checks if the value of the field is `_0X06`"]
    #[inline(always)]
    pub fn is_0x06(&self) -> bool {
        *self == LVD1LVL_A::_0X06
    }
    #[doc = "Checks if the value of the field is `_0X07`"]
    #[inline(always)]
    pub fn is_0x07(&self) -> bool {
        *self == LVD1LVL_A::_0X07
    }
    #[doc = "Checks if the value of the field is `_0X08`"]
    #[inline(always)]
    pub fn is_0x08(&self) -> bool {
        *self == LVD1LVL_A::_0X08
    }
    #[doc = "Checks if the value of the field is `_0X09`"]
    #[inline(always)]
    pub fn is_0x09(&self) -> bool {
        *self == LVD1LVL_A::_0X09
    }
    #[doc = "Checks if the value of the field is `_0X0A`"]
    #[inline(always)]
    pub fn is_0x0a(&self) -> bool {
        *self == LVD1LVL_A::_0X0A
    }
    #[doc = "Checks if the value of the field is `_0X0B`"]
    #[inline(always)]
    pub fn is_0x0b(&self) -> bool {
        *self == LVD1LVL_A::_0X0B
    }
    #[doc = "Checks if the value of the field is `_0X0C`"]
    #[inline(always)]
    pub fn is_0x0c(&self) -> bool {
        *self == LVD1LVL_A::_0X0C
    }
    #[doc = "Checks if the value of the field is `_0X0D`"]
    #[inline(always)]
    pub fn is_0x0d(&self) -> bool {
        *self == LVD1LVL_A::_0X0D
    }
    #[doc = "Checks if the value of the field is `_0X0E`"]
    #[inline(always)]
    pub fn is_0x0e(&self) -> bool {
        *self == LVD1LVL_A::_0X0E
    }
    #[doc = "Checks if the value of the field is `_0X0F`"]
    #[inline(always)]
    pub fn is_0x0f(&self) -> bool {
        *self == LVD1LVL_A::_0X0F
    }
}
#[doc = "Field `LVD1LVL` writer - Voltage Detection 1 Level Select (Standard voltage during fall in voltage)"]
pub type LVD1LVL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LVDLVLR_SPEC, u8, LVD1LVL_A, 5, O>;
impl<'a, const O: u8> LVD1LVL_W<'a, O> {
    #[doc = "Vdet1_0"]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_0X00)
    }
    #[doc = "Vdet1_1"]
    #[inline(always)]
    pub fn _0x01(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_0X01)
    }
    #[doc = "Vdet1_2"]
    #[inline(always)]
    pub fn _0x02(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_0X02)
    }
    #[doc = "Vdet1_3"]
    #[inline(always)]
    pub fn _0x03(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_0X03)
    }
    #[doc = "Vdet1_4"]
    #[inline(always)]
    pub fn _0x04(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_0X04)
    }
    #[doc = "Vdet1_5"]
    #[inline(always)]
    pub fn _0x05(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_0X05)
    }
    #[doc = "Vdet1_6"]
    #[inline(always)]
    pub fn _0x06(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_0X06)
    }
    #[doc = "Vdet1_7"]
    #[inline(always)]
    pub fn _0x07(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_0X07)
    }
    #[doc = "Vdet1_8"]
    #[inline(always)]
    pub fn _0x08(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_0X08)
    }
    #[doc = "Vdet1_9"]
    #[inline(always)]
    pub fn _0x09(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_0X09)
    }
    #[doc = "Vdet1_A"]
    #[inline(always)]
    pub fn _0x0a(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_0X0A)
    }
    #[doc = "Vdet1_B"]
    #[inline(always)]
    pub fn _0x0b(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_0X0B)
    }
    #[doc = "Vdet1_C"]
    #[inline(always)]
    pub fn _0x0c(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_0X0C)
    }
    #[doc = "Vdet1_D"]
    #[inline(always)]
    pub fn _0x0d(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_0X0D)
    }
    #[doc = "Vdet1_E"]
    #[inline(always)]
    pub fn _0x0e(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_0X0E)
    }
    #[doc = "Vdet1_F"]
    #[inline(always)]
    pub fn _0x0f(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_0X0F)
    }
}
#[doc = "Field `LVD2LVL` reader - Voltage Detection 2 Level Select (Standard voltage during fall in voltage)"]
pub type LVD2LVL_R = crate::FieldReader<u8, LVD2LVL_A>;
#[doc = "Voltage Detection 2 Level Select (Standard voltage during fall in voltage)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LVD2LVL_A {
    #[doc = "0: Vdet2_0"]
    _000 = 0,
    #[doc = "1: Vdet2_1"]
    _001 = 1,
    #[doc = "2: Vdet2_2"]
    _010 = 2,
    #[doc = "3: Vdet2_3"]
    _011 = 3,
}
impl From<LVD2LVL_A> for u8 {
    #[inline(always)]
    fn from(variant: LVD2LVL_A) -> Self {
        variant as _
    }
}
impl LVD2LVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LVD2LVL_A> {
        match self.bits {
            0 => Some(LVD2LVL_A::_000),
            1 => Some(LVD2LVL_A::_001),
            2 => Some(LVD2LVL_A::_010),
            3 => Some(LVD2LVL_A::_011),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == LVD2LVL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == LVD2LVL_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == LVD2LVL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == LVD2LVL_A::_011
    }
}
#[doc = "Field `LVD2LVL` writer - Voltage Detection 2 Level Select (Standard voltage during fall in voltage)"]
pub type LVD2LVL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LVDLVLR_SPEC, u8, LVD2LVL_A, 3, O>;
impl<'a, const O: u8> LVD2LVL_W<'a, O> {
    #[doc = "Vdet2_0"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(LVD2LVL_A::_000)
    }
    #[doc = "Vdet2_1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(LVD2LVL_A::_001)
    }
    #[doc = "Vdet2_2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(LVD2LVL_A::_010)
    }
    #[doc = "Vdet2_3"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(LVD2LVL_A::_011)
    }
}
impl R {
    #[doc = "Bits 0:4 - Voltage Detection 1 Level Select (Standard voltage during fall in voltage)"]
    #[inline(always)]
    pub fn lvd1lvl(&self) -> LVD1LVL_R {
        LVD1LVL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - Voltage Detection 2 Level Select (Standard voltage during fall in voltage)"]
    #[inline(always)]
    pub fn lvd2lvl(&self) -> LVD2LVL_R {
        LVD2LVL_R::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Voltage Detection 1 Level Select (Standard voltage during fall in voltage)"]
    #[inline(always)]
    pub fn lvd1lvl(&mut self) -> LVD1LVL_W<0> {
        LVD1LVL_W::new(self)
    }
    #[doc = "Bits 5:7 - Voltage Detection 2 Level Select (Standard voltage during fall in voltage)"]
    #[inline(always)]
    pub fn lvd2lvl(&mut self) -> LVD2LVL_W<5> {
        LVD2LVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Voltage Detection Level Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lvdlvlr](index.html) module"]
pub struct LVDLVLR_SPEC;
impl crate::RegisterSpec for LVDLVLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lvdlvlr::R](R) reader structure"]
impl crate::Readable for LVDLVLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lvdlvlr::W](W) writer structure"]
impl crate::Writable for LVDLVLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LVDLVLR to value 0x07"]
impl crate::Resettable for LVDLVLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
