#[doc = "Register `LVD2CMPCR` reader"]
pub struct R(crate::R<LVD2CMPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LVD2CMPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LVD2CMPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LVD2CMPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LVD2CMPCR` writer"]
pub struct W(crate::W<LVD2CMPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LVD2CMPCR_SPEC>;
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
impl From<crate::W<LVD2CMPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LVD2CMPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVD2LVL` reader - Voltage Detection 2 Level Select (Standard voltage during drop in voltage)"]
pub type LVD2LVL_R = crate::FieldReader<u8, LVD2LVL_A>;
#[doc = "Voltage Detection 2 Level Select (Standard voltage during drop in voltage)\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LVD2LVL_A {
    #[doc = "5: 2.99 V (Vdet2_5)"]
    _101 = 5,
    #[doc = "6: 2.92 V (Vdet2_6)"]
    _110 = 6,
    #[doc = "7: 2.85 V (Vdet2_7)"]
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
#[doc = "Field `LVD2LVL` writer - Voltage Detection 2 Level Select (Standard voltage during drop in voltage)"]
pub type LVD2LVL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, LVD2CMPCR_SPEC, u8, LVD2LVL_A, 3, O>;
impl<'a, const O: u8> LVD2LVL_W<'a, O> {
    #[doc = "2.99 V (Vdet2_5)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(LVD2LVL_A::_101)
    }
    #[doc = "2.92 V (Vdet2_6)"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(LVD2LVL_A::_110)
    }
    #[doc = "2.85 V (Vdet2_7)"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(LVD2LVL_A::_111)
    }
}
#[doc = "Field `LVD2E` reader - Voltage Detection 2 Enable"]
pub type LVD2E_R = crate::BitReader<LVD2E_A>;
#[doc = "Voltage Detection 2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD2E_A {
    #[doc = "0: Voltage detection 2 circuit disabled"]
    _0 = 0,
    #[doc = "1: Voltage detection 2 circuit enabled"]
    _1 = 1,
}
impl From<LVD2E_A> for bool {
    #[inline(always)]
    fn from(variant: LVD2E_A) -> Self {
        variant as u8 != 0
    }
}
impl LVD2E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVD2E_A {
        match self.bits {
            false => LVD2E_A::_0,
            true => LVD2E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD2E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD2E_A::_1
    }
}
#[doc = "Field `LVD2E` writer - Voltage Detection 2 Enable"]
pub type LVD2E_W<'a, const O: u8> = crate::BitWriter<'a, u8, LVD2CMPCR_SPEC, LVD2E_A, O>;
impl<'a, const O: u8> LVD2E_W<'a, O> {
    #[doc = "Voltage detection 2 circuit disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVD2E_A::_0)
    }
    #[doc = "Voltage detection 2 circuit enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVD2E_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Voltage Detection 2 Level Select (Standard voltage during drop in voltage)"]
    #[inline(always)]
    pub fn lvd2lvl(&self) -> LVD2LVL_R {
        LVD2LVL_R::new(self.bits & 7)
    }
    #[doc = "Bit 7 - Voltage Detection 2 Enable"]
    #[inline(always)]
    pub fn lvd2e(&self) -> LVD2E_R {
        LVD2E_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Voltage Detection 2 Level Select (Standard voltage during drop in voltage)"]
    #[inline(always)]
    #[must_use]
    pub fn lvd2lvl(&mut self) -> LVD2LVL_W<0> {
        LVD2LVL_W::new(self)
    }
    #[doc = "Bit 7 - Voltage Detection 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvd2e(&mut self) -> LVD2E_W<7> {
        LVD2E_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Voltage Monitoring 2 Comparator Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lvd2cmpcr](index.html) module"]
pub struct LVD2CMPCR_SPEC;
impl crate::RegisterSpec for LVD2CMPCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lvd2cmpcr::R](R) reader structure"]
impl crate::Readable for LVD2CMPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lvd2cmpcr::W](W) writer structure"]
impl crate::Writable for LVD2CMPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LVD2CMPCR to value 0x07"]
impl crate::Resettable for LVD2CMPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
