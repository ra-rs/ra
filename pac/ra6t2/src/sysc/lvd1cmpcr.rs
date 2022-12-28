#[doc = "Register `LVD1CMPCR` reader"]
pub struct R(crate::R<LVD1CMPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LVD1CMPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LVD1CMPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LVD1CMPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LVD1CMPCR` writer"]
pub struct W(crate::W<LVD1CMPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LVD1CMPCR_SPEC>;
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
impl From<crate::W<LVD1CMPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LVD1CMPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVD1LVL` reader - Voltage Detection 1 Level Select (Standard voltage during drop in voltage)"]
pub type LVD1LVL_R = crate::FieldReader<u8, LVD1LVL_A>;
#[doc = "Voltage Detection 1 Level Select (Standard voltage during drop in voltage)\n\nValue on reset: 19"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LVD1LVL_A {
    #[doc = "17: 2.99 V (Vdet1_1)"]
    _0X11 = 17,
    #[doc = "18: 2.92 V (Vdet1_2)"]
    _0X12 = 18,
    #[doc = "19: 2.85 V (Vdet1_3)"]
    _0X13 = 19,
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
            17 => Some(LVD1LVL_A::_0X11),
            18 => Some(LVD1LVL_A::_0X12),
            19 => Some(LVD1LVL_A::_0X13),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X11`"]
    #[inline(always)]
    pub fn is_0x11(&self) -> bool {
        *self == LVD1LVL_A::_0X11
    }
    #[doc = "Checks if the value of the field is `_0X12`"]
    #[inline(always)]
    pub fn is_0x12(&self) -> bool {
        *self == LVD1LVL_A::_0X12
    }
    #[doc = "Checks if the value of the field is `_0X13`"]
    #[inline(always)]
    pub fn is_0x13(&self) -> bool {
        *self == LVD1LVL_A::_0X13
    }
}
#[doc = "Field `LVD1LVL` writer - Voltage Detection 1 Level Select (Standard voltage during drop in voltage)"]
pub type LVD1LVL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, LVD1CMPCR_SPEC, u8, LVD1LVL_A, 5, O>;
impl<'a, const O: u8> LVD1LVL_W<'a, O> {
    #[doc = "2.99 V (Vdet1_1)"]
    #[inline(always)]
    pub fn _0x11(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_0X11)
    }
    #[doc = "2.92 V (Vdet1_2)"]
    #[inline(always)]
    pub fn _0x12(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_0X12)
    }
    #[doc = "2.85 V (Vdet1_3)"]
    #[inline(always)]
    pub fn _0x13(self) -> &'a mut W {
        self.variant(LVD1LVL_A::_0X13)
    }
}
#[doc = "Field `LVD1E` reader - Voltage Detection 1 Enable"]
pub type LVD1E_R = crate::BitReader<LVD1E_A>;
#[doc = "Voltage Detection 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD1E_A {
    #[doc = "0: Voltage detection 1 circuit disabled"]
    _0 = 0,
    #[doc = "1: Voltage detection 1 circuit enabled"]
    _1 = 1,
}
impl From<LVD1E_A> for bool {
    #[inline(always)]
    fn from(variant: LVD1E_A) -> Self {
        variant as u8 != 0
    }
}
impl LVD1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVD1E_A {
        match self.bits {
            false => LVD1E_A::_0,
            true => LVD1E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD1E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD1E_A::_1
    }
}
#[doc = "Field `LVD1E` writer - Voltage Detection 1 Enable"]
pub type LVD1E_W<'a, const O: u8> = crate::BitWriter<'a, u8, LVD1CMPCR_SPEC, LVD1E_A, O>;
impl<'a, const O: u8> LVD1E_W<'a, O> {
    #[doc = "Voltage detection 1 circuit disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVD1E_A::_0)
    }
    #[doc = "Voltage detection 1 circuit enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVD1E_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:4 - Voltage Detection 1 Level Select (Standard voltage during drop in voltage)"]
    #[inline(always)]
    pub fn lvd1lvl(&self) -> LVD1LVL_R {
        LVD1LVL_R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 7 - Voltage Detection 1 Enable"]
    #[inline(always)]
    pub fn lvd1e(&self) -> LVD1E_R {
        LVD1E_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Voltage Detection 1 Level Select (Standard voltage during drop in voltage)"]
    #[inline(always)]
    #[must_use]
    pub fn lvd1lvl(&mut self) -> LVD1LVL_W<0> {
        LVD1LVL_W::new(self)
    }
    #[doc = "Bit 7 - Voltage Detection 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvd1e(&mut self) -> LVD1E_W<7> {
        LVD1E_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Voltage Monitoring 1 Comparator Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lvd1cmpcr](index.html) module"]
pub struct LVD1CMPCR_SPEC;
impl crate::RegisterSpec for LVD1CMPCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lvd1cmpcr::R](R) reader structure"]
impl crate::Readable for LVD1CMPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lvd1cmpcr::W](W) writer structure"]
impl crate::Writable for LVD1CMPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LVD1CMPCR to value 0x13"]
impl crate::Resettable for LVD1CMPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x13;
}
