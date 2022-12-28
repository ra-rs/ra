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
#[doc = "Field `LVD1LVL` reader - Voltage Detection 1 Level Select (Standard voltage during drop in voltage)"]
pub type LVD1LVL_R = crate::FieldReader<u8, LVD1LVL_A>;
#[doc = "Voltage Detection 1 Level Select (Standard voltage during drop in voltage)\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LVD1LVL_A {
    #[doc = "0: 4.29V (Vdet1_0)"]
    _00000 = 0,
    #[doc = "1: 4.14V (Vdet1_1)"]
    _00001 = 1,
    #[doc = "2: 4.02V (Vdet1_2)"]
    _00010 = 2,
    #[doc = "3: 3.84V (Vdet1_3)"]
    _00011 = 3,
    #[doc = "4: 3.10V (Vdet1_4)"]
    _00100 = 4,
    #[doc = "5: 3.00V (Vdet1_5)"]
    _00101 = 5,
    #[doc = "6: 2.90V (Vdet1_6)"]
    _00110 = 6,
    #[doc = "7: 2.79V (Vdet1_7)"]
    _00111 = 7,
    #[doc = "8: 2.68V (Vdet1_8)"]
    _01000 = 8,
    #[doc = "9: 2.58V (Vdet1_9)"]
    _01001 = 9,
    #[doc = "10: 2.48V (Vdet1_A)"]
    _01010 = 10,
    #[doc = "11: 2.20V (Vdet1_B)"]
    _01011 = 11,
    #[doc = "12: 1.96V (Vdet1_C)"]
    _01100 = 12,
    #[doc = "13: 1.86V (Vdet1_D)"]
    _01101 = 13,
    #[doc = "14: 1.75V (Vdet1_E)"]
    _01110 = 14,
    #[doc = "15: 1.65V (Vdet1_F)"]
    _01111 = 15,
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
            0 => Some(LVD1LVL_A::_00000),
            1 => Some(LVD1LVL_A::_00001),
            2 => Some(LVD1LVL_A::_00010),
            3 => Some(LVD1LVL_A::_00011),
            4 => Some(LVD1LVL_A::_00100),
            5 => Some(LVD1LVL_A::_00101),
            6 => Some(LVD1LVL_A::_00110),
            7 => Some(LVD1LVL_A::_00111),
            8 => Some(LVD1LVL_A::_01000),
            9 => Some(LVD1LVL_A::_01001),
            10 => Some(LVD1LVL_A::_01010),
            11 => Some(LVD1LVL_A::_01011),
            12 => Some(LVD1LVL_A::_01100),
            13 => Some(LVD1LVL_A::_01101),
            14 => Some(LVD1LVL_A::_01110),
            15 => Some(LVD1LVL_A::_01111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00000`"]
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        *self == LVD1LVL_A::_00000
    }
    #[doc = "Checks if the value of the field is `_00001`"]
    #[inline(always)]
    pub fn is_00001(&self) -> bool {
        *self == LVD1LVL_A::_00001
    }
    #[doc = "Checks if the value of the field is `_00010`"]
    #[inline(always)]
    pub fn is_00010(&self) -> bool {
        *self == LVD1LVL_A::_00010
    }
    #[doc = "Checks if the value of the field is `_00011`"]
    #[inline(always)]
    pub fn is_00011(&self) -> bool {
        *self == LVD1LVL_A::_00011
    }
    #[doc = "Checks if the value of the field is `_00100`"]
    #[inline(always)]
    pub fn is_00100(&self) -> bool {
        *self == LVD1LVL_A::_00100
    }
    #[doc = "Checks if the value of the field is `_00101`"]
    #[inline(always)]
    pub fn is_00101(&self) -> bool {
        *self == LVD1LVL_A::_00101
    }
    #[doc = "Checks if the value of the field is `_00110`"]
    #[inline(always)]
    pub fn is_00110(&self) -> bool {
        *self == LVD1LVL_A::_00110
    }
    #[doc = "Checks if the value of the field is `_00111`"]
    #[inline(always)]
    pub fn is_00111(&self) -> bool {
        *self == LVD1LVL_A::_00111
    }
    #[doc = "Checks if the value of the field is `_01000`"]
    #[inline(always)]
    pub fn is_01000(&self) -> bool {
        *self == LVD1LVL_A::_01000
    }
    #[doc = "Checks if the value of the field is `_01001`"]
    #[inline(always)]
    pub fn is_01001(&self) -> bool {
        *self == LVD1LVL_A::_01001
    }
    #[doc = "Checks if the value of the field is `_01010`"]
    #[inline(always)]
    pub fn is_01010(&self) -> bool {
        *self == LVD1LVL_A::_01010
    }
    #[doc = "Checks if the value of the field is `_01011`"]
    #[inline(always)]
    pub fn is_01011(&self) -> bool {
        *self == LVD1LVL_A::_01011
    }
    #[doc = "Checks if the value of the field is `_01100`"]
    #[inline(always)]
    pub fn is_01100(&self) -> bool {
        *self == LVD1LVL_A::_01100
    }
    #[doc = "Checks if the value of the field is `_01101`"]
    #[inline(always)]
    pub fn is_01101(&self) -> bool {
        *self == LVD1LVL_A::_01101
    }
    #[doc = "Checks if the value of the field is `_01110`"]
    #[inline(always)]
    pub fn is_01110(&self) -> bool {
        *self == LVD1LVL_A::_01110
    }
    #[doc = "Checks if the value of the field is `_01111`"]
    #[inline(always)]
    pub fn is_01111(&self) -> bool {
        *self == LVD1LVL_A::_01111
    }
}
#[doc = "Field `LVD1LVL` writer - Voltage Detection 1 Level Select (Standard voltage during drop in voltage)"]
pub type LVD1LVL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LVDLVLR_SPEC, u8, LVD1LVL_A, 5, O>;
impl<'a, const O: u8> LVD1LVL_W<'a, O> {
    #[doc = "4.29V (Vdet1_0)"]
    #[inline(always)]
    pub fn _00000(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_00000)
    }
    #[doc = "4.14V (Vdet1_1)"]
    #[inline(always)]
    pub fn _00001(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_00001)
    }
    #[doc = "4.02V (Vdet1_2)"]
    #[inline(always)]
    pub fn _00010(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_00010)
    }
    #[doc = "3.84V (Vdet1_3)"]
    #[inline(always)]
    pub fn _00011(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_00011)
    }
    #[doc = "3.10V (Vdet1_4)"]
    #[inline(always)]
    pub fn _00100(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_00100)
    }
    #[doc = "3.00V (Vdet1_5)"]
    #[inline(always)]
    pub fn _00101(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_00101)
    }
    #[doc = "2.90V (Vdet1_6)"]
    #[inline(always)]
    pub fn _00110(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_00110)
    }
    #[doc = "2.79V (Vdet1_7)"]
    #[inline(always)]
    pub fn _00111(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_00111)
    }
    #[doc = "2.68V (Vdet1_8)"]
    #[inline(always)]
    pub fn _01000(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_01000)
    }
    #[doc = "2.58V (Vdet1_9)"]
    #[inline(always)]
    pub fn _01001(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_01001)
    }
    #[doc = "2.48V (Vdet1_A)"]
    #[inline(always)]
    pub fn _01010(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_01010)
    }
    #[doc = "2.20V (Vdet1_B)"]
    #[inline(always)]
    pub fn _01011(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_01011)
    }
    #[doc = "1.96V (Vdet1_C)"]
    #[inline(always)]
    pub fn _01100(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_01100)
    }
    #[doc = "1.86V (Vdet1_D)"]
    #[inline(always)]
    pub fn _01101(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_01101)
    }
    #[doc = "1.75V (Vdet1_E)"]
    #[inline(always)]
    pub fn _01110(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_01110)
    }
    #[doc = "1.65V (Vdet1_F)"]
    #[inline(always)]
    pub fn _01111(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_01111)
    }
}
#[doc = "Field `LVD2LVL` reader - Voltage Detection 2 Level Select (Standard voltage during drop in voltage)"]
pub type LVD2LVL_R = crate::FieldReader<u8, LVD2LVL_A>;
#[doc = "Voltage Detection 2 Level Select (Standard voltage during drop in voltage)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LVD2LVL_A {
    #[doc = "0: 4.29V (Vdet2_0)"]
    _000 = 0,
    #[doc = "1: 4.14V (Vdet2_1)"]
    _001 = 1,
    #[doc = "2: 4.02V (Vdet2_2)"]
    _010 = 2,
    #[doc = "3: 3.84V (Vdet2_3)"]
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
#[doc = "Field `LVD2LVL` writer - Voltage Detection 2 Level Select (Standard voltage during drop in voltage)"]
pub type LVD2LVL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LVDLVLR_SPEC, u8, LVD2LVL_A, 3, O>;
impl<'a, const O: u8> LVD2LVL_W<'a, O> {
    #[doc = "4.29V (Vdet2_0)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(LVD2LVL_A::_000)
    }
    #[doc = "4.14V (Vdet2_1)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(LVD2LVL_A::_001)
    }
    #[doc = "4.02V (Vdet2_2)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(LVD2LVL_A::_010)
    }
    #[doc = "3.84V (Vdet2_3)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(LVD2LVL_A::_011)
    }
}
impl R {
    #[doc = "Bits 0:4 - Voltage Detection 1 Level Select (Standard voltage during drop in voltage)"]
    #[inline(always)]
    pub fn lvd1lvl(&self) -> LVD1LVL_R {
        LVD1LVL_R::new(self.bits & 0x1f)
    }
    #[doc = "Bits 5:7 - Voltage Detection 2 Level Select (Standard voltage during drop in voltage)"]
    #[inline(always)]
    pub fn lvd2lvl(&self) -> LVD2LVL_R {
        LVD2LVL_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bits 0:4 - Voltage Detection 1 Level Select (Standard voltage during drop in voltage)"]
    #[inline(always)]
    #[must_use]
    pub fn lvd1lvl(&mut self) -> LVD1LVL_W<0> {
        LVD1LVL_W::new(self)
    }
    #[doc = "Bits 5:7 - Voltage Detection 2 Level Select (Standard voltage during drop in voltage)"]
    #[inline(always)]
    #[must_use]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LVDLVLR to value 0x07"]
impl crate::Resettable for LVDLVLR_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
