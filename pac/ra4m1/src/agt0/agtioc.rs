#[doc = "Register `AGTIOC` reader"]
pub struct R(crate::R<AGTIOC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AGTIOC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AGTIOC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AGTIOC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AGTIOC` writer"]
pub struct W(crate::W<AGTIOC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AGTIOC_SPEC>;
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
impl From<crate::W<AGTIOC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AGTIOC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEDGSEL` reader - I/O polarity switch Function varies depending on the operating mode."]
pub type TEDGSEL_R = crate::BitReader<bool>;
#[doc = "Field `TEDGSEL` writer - I/O polarity switch Function varies depending on the operating mode."]
pub type TEDGSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, AGTIOC_SPEC, bool, O>;
#[doc = "Field `TOE` reader - AGTOn output enable"]
pub type TOE_R = crate::BitReader<TOE_A>;
#[doc = "AGTOn output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOE_A {
    #[doc = "0: AGTOn output disabled"]
    _0 = 0,
    #[doc = "1: AGTOn output enabled."]
    _1 = 1,
}
impl From<TOE_A> for bool {
    #[inline(always)]
    fn from(variant: TOE_A) -> Self {
        variant as u8 != 0
    }
}
impl TOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOE_A {
        match self.bits {
            false => TOE_A::_0,
            true => TOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOE_A::_1
    }
}
#[doc = "Field `TOE` writer - AGTOn output enable"]
pub type TOE_W<'a, const O: u8> = crate::BitWriter<'a, u8, AGTIOC_SPEC, TOE_A, O>;
impl<'a, const O: u8> TOE_W<'a, O> {
    #[doc = "AGTOn output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOE_A::_0)
    }
    #[doc = "AGTOn output enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOE_A::_1)
    }
}
#[doc = "Field `TIPF` reader - Input filter"]
pub type TIPF_R = crate::FieldReader<u8, TIPF_A>;
#[doc = "Input filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIPF_A {
    #[doc = "0: No filter"]
    _00 = 0,
    #[doc = "1: Filter sampled at PCLKB"]
    _01 = 1,
    #[doc = "2: Filter sampled at PCLKB/8"]
    _10 = 2,
    #[doc = "3: Filter sampled at PCLKB/32"]
    _11 = 3,
}
impl From<TIPF_A> for u8 {
    #[inline(always)]
    fn from(variant: TIPF_A) -> Self {
        variant as _
    }
}
impl TIPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIPF_A {
        match self.bits {
            0 => TIPF_A::_00,
            1 => TIPF_A::_01,
            2 => TIPF_A::_10,
            3 => TIPF_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TIPF_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TIPF_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TIPF_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TIPF_A::_11
    }
}
#[doc = "Field `TIPF` writer - Input filter"]
pub type TIPF_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, AGTIOC_SPEC, u8, TIPF_A, 2, O>;
impl<'a, const O: u8> TIPF_W<'a, O> {
    #[doc = "No filter"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TIPF_A::_00)
    }
    #[doc = "Filter sampled at PCLKB"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TIPF_A::_01)
    }
    #[doc = "Filter sampled at PCLKB/8"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TIPF_A::_10)
    }
    #[doc = "Filter sampled at PCLKB/32"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TIPF_A::_11)
    }
}
#[doc = "Field `TIOGT` reader - Count control"]
pub type TIOGT_R = crate::FieldReader<u8, TIOGT_A>;
#[doc = "Count control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIOGT_A {
    #[doc = "0: Event is always counted"]
    _00 = 0,
    #[doc = "1: Event is counted during polarity period specified for AGTEEn."]
    _01 = 1,
}
impl From<TIOGT_A> for u8 {
    #[inline(always)]
    fn from(variant: TIOGT_A) -> Self {
        variant as _
    }
}
impl TIOGT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIOGT_A> {
        match self.bits {
            0 => Some(TIOGT_A::_00),
            1 => Some(TIOGT_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TIOGT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TIOGT_A::_01
    }
}
#[doc = "Field `TIOGT` writer - Count control"]
pub type TIOGT_W<'a, const O: u8> = crate::FieldWriter<'a, u8, AGTIOC_SPEC, u8, TIOGT_A, 2, O>;
impl<'a, const O: u8> TIOGT_W<'a, O> {
    #[doc = "Event is always counted"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TIOGT_A::_00)
    }
    #[doc = "Event is counted during polarity period specified for AGTEEn."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TIOGT_A::_01)
    }
}
impl R {
    #[doc = "Bit 0 - I/O polarity switch Function varies depending on the operating mode."]
    #[inline(always)]
    pub fn tedgsel(&self) -> TEDGSEL_R {
        TEDGSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - AGTOn output enable"]
    #[inline(always)]
    pub fn toe(&self) -> TOE_R {
        TOE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Input filter"]
    #[inline(always)]
    pub fn tipf(&self) -> TIPF_R {
        TIPF_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Count control"]
    #[inline(always)]
    pub fn tiogt(&self) -> TIOGT_R {
        TIOGT_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - I/O polarity switch Function varies depending on the operating mode."]
    #[inline(always)]
    #[must_use]
    pub fn tedgsel(&mut self) -> TEDGSEL_W<0> {
        TEDGSEL_W::new(self)
    }
    #[doc = "Bit 2 - AGTOn output enable"]
    #[inline(always)]
    #[must_use]
    pub fn toe(&mut self) -> TOE_W<2> {
        TOE_W::new(self)
    }
    #[doc = "Bits 4:5 - Input filter"]
    #[inline(always)]
    #[must_use]
    pub fn tipf(&mut self) -> TIPF_W<4> {
        TIPF_W::new(self)
    }
    #[doc = "Bits 6:7 - Count control"]
    #[inline(always)]
    #[must_use]
    pub fn tiogt(&mut self) -> TIOGT_W<6> {
        TIOGT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AGT I/O Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [agtioc](index.html) module"]
pub struct AGTIOC_SPEC;
impl crate::RegisterSpec for AGTIOC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [agtioc::R](R) reader structure"]
impl crate::Readable for AGTIOC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [agtioc::W](W) writer structure"]
impl crate::Writable for AGTIOC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AGTIOC to value 0"]
impl crate::Resettable for AGTIOC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
