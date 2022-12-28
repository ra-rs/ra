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
#[doc = "Voltage Detection 1 Level Select (Standard voltage during fall in voltage)\n\nValue on reset: 19"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LVD1LVL_A {
    #[doc = "17: 2.99V (Vdet1_1)"]
    _10001 = 17,
    #[doc = "18: 2.92V (Vdet1_2)"]
    _10010 = 18,
    #[doc = "19: 2.85V (Vdet1_3)"]
    _10011 = 19,
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
            17 => Some(LVD1LVL_A::_10001),
            18 => Some(LVD1LVL_A::_10010),
            19 => Some(LVD1LVL_A::_10011),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_10001`"]
    #[inline(always)]
    pub fn is_10001(&self) -> bool {
        *self == LVD1LVL_A::_10001
    }
    #[doc = "Checks if the value of the field is `_10010`"]
    #[inline(always)]
    pub fn is_10010(&self) -> bool {
        *self == LVD1LVL_A::_10010
    }
    #[doc = "Checks if the value of the field is `_10011`"]
    #[inline(always)]
    pub fn is_10011(&self) -> bool {
        *self == LVD1LVL_A::_10011
    }
}
#[doc = "Field `LVD1LVL` writer - Voltage Detection 1 Level Select (Standard voltage during fall in voltage)"]
pub type LVD1LVL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LVDLVLR_SPEC, u8, LVD1LVL_A, 5, O>;
impl<'a, const O: u8> LVD1LVL_W<'a, O> {
    #[doc = "2.99V (Vdet1_1)"]
    #[inline(always)]
    pub fn _10001(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_10001)
    }
    #[doc = "2.92V (Vdet1_2)"]
    #[inline(always)]
    pub fn _10010(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_10010)
    }
    #[doc = "2.85V (Vdet1_3)"]
    #[inline(always)]
    pub fn _10011(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_10011)
    }
}
#[doc = "Field `LVD2LVL` reader - Voltage Detection 2 Level Select (Standard voltage during fall in voltage)"]
pub type LVD2LVL_R = crate::FieldReader<u8, LVD2LVL_A>;
#[doc = "Voltage Detection 2 Level Select (Standard voltage during fall in voltage)\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LVD2LVL_A {
    #[doc = "5: 2.99V (Vdet2_1)"]
    _101 = 5,
    #[doc = "6: 2.92V (Vdet2_2)"]
    _110 = 6,
    #[doc = "7: 2.85V (Vdet2_3)"]
    _111 = 7,
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
            5 => Some(LVD2LVL_A::_101),
            6 => Some(LVD2LVL_A::_110),
            7 => Some(LVD2LVL_A::_111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == LVD2LVL_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == LVD2LVL_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == LVD2LVL_A::_111
    }
}
#[doc = "Field `LVD2LVL` writer - Voltage Detection 2 Level Select (Standard voltage during fall in voltage)"]
pub type LVD2LVL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LVDLVLR_SPEC, u8, LVD2LVL_A, 3, O>;
impl<'a, const O: u8> LVD2LVL_W<'a, O> {
    #[doc = "2.99V (Vdet2_1)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(LVD2LVL_A::_101)
    }
    #[doc = "2.92V (Vdet2_2)"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(LVD2LVL_A::_110)
    }
    #[doc = "2.85V (Vdet2_3)"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(LVD2LVL_A::_111)
    }
}
impl R {
    #[doc = "Bits 0:4 - Voltage Detection 1 Level Select (Standard voltage during fall in voltage)"]
    #[inline(always)]
    pub fn lvd1lvl(&self) -> LVD1LVL_R {
        LVD1LVL_R::new(self.bits & 0x1f)
    }
    #[doc = "Bits 5:7 - Voltage Detection 2 Level Select (Standard voltage during fall in voltage)"]
    #[inline(always)]
    pub fn lvd2lvl(&self) -> LVD2LVL_R {
        LVD2LVL_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bits 0:4 - Voltage Detection 1 Level Select (Standard voltage during fall in voltage)"]
    #[inline(always)]
    #[must_use]
    pub fn lvd1lvl(&mut self) -> LVD1LVL_W<0> {
        LVD1LVL_W::new(self)
    }
    #[doc = "Bits 5:7 - Voltage Detection 2 Level Select (Standard voltage during fall in voltage)"]
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
#[doc = "`reset()` method sets LVDLVLR to value 0xf3"]
impl crate::Resettable for LVDLVLR_SPEC {
    const RESET_VALUE: Self::Ux = 0xf3;
}
