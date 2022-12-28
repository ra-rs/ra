#[doc = "Register `PLL2CCR` reader"]
pub struct R(crate::R<PLL2CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL2CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL2CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL2CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL2CCR` writer"]
pub struct W(crate::W<PLL2CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL2CCR_SPEC>;
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
impl From<crate::W<PLL2CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL2CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PL2IDIV` reader - PLL2 Input Frequency Division Ratio Select"]
pub type PL2IDIV_R = crate::FieldReader<u8, PL2IDIV_A>;
#[doc = "PLL2 Input Frequency Division Ratio Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PL2IDIV_A {
    #[doc = "0: â\u{88}\u{95} 1 (value after reset)"]
    _00 = 0,
    #[doc = "1: â\u{88}\u{95} 2"]
    _01 = 1,
    #[doc = "2: â\u{88}\u{95} 3"]
    _10 = 2,
}
impl From<PL2IDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PL2IDIV_A) -> Self {
        variant as _
    }
}
impl PL2IDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PL2IDIV_A {
        match self.bits {
            0 => PL2IDIV_A::_00,
            1 => PL2IDIV_A::_01,
            2 => PL2IDIV_A::_10,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PL2IDIV_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PL2IDIV_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PL2IDIV_A::_10
    }
}
#[doc = "Field `PL2IDIV` writer - PLL2 Input Frequency Division Ratio Select"]
pub type PL2IDIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, PLL2CCR_SPEC, u8, PL2IDIV_A, 2, O>;
impl<'a, const O: u8> PL2IDIV_W<'a, O> {
    #[doc = "â\u{88}\u{95} 1 (value after reset)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PL2IDIV_A::_00)
    }
    #[doc = "â\u{88}\u{95} 2"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PL2IDIV_A::_01)
    }
    #[doc = "â\u{88}\u{95} 3"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PL2IDIV_A::_10)
    }
}
#[doc = "Field `PL2SRCSEL` reader - PLL2 Clock Source Select"]
pub type PL2SRCSEL_R = crate::BitReader<PL2SRCSEL_A>;
#[doc = "PLL2 Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PL2SRCSEL_A {
    #[doc = "0: Main clock oscillator"]
    _0 = 0,
    #[doc = "1: HOCO"]
    _1 = 1,
}
impl From<PL2SRCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PL2SRCSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl PL2SRCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PL2SRCSEL_A {
        match self.bits {
            false => PL2SRCSEL_A::_0,
            true => PL2SRCSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PL2SRCSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PL2SRCSEL_A::_1
    }
}
#[doc = "Field `PL2SRCSEL` writer - PLL2 Clock Source Select"]
pub type PL2SRCSEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, PLL2CCR_SPEC, PL2SRCSEL_A, O>;
impl<'a, const O: u8> PL2SRCSEL_W<'a, O> {
    #[doc = "Main clock oscillator"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PL2SRCSEL_A::_0)
    }
    #[doc = "HOCO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PL2SRCSEL_A::_1)
    }
}
#[doc = "Field `PLL2MUL` reader - PLL2 Frequency Multiplication Factor Select"]
pub type PLL2MUL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL2MUL` writer - PLL2 Frequency Multiplication Factor Select"]
pub type PLL2MUL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PLL2CCR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:1 - PLL2 Input Frequency Division Ratio Select"]
    #[inline(always)]
    pub fn pl2idiv(&self) -> PL2IDIV_R {
        PL2IDIV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - PLL2 Clock Source Select"]
    #[inline(always)]
    pub fn pl2srcsel(&self) -> PL2SRCSEL_R {
        PL2SRCSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:13 - PLL2 Frequency Multiplication Factor Select"]
    #[inline(always)]
    pub fn pll2mul(&self) -> PLL2MUL_R {
        PLL2MUL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PLL2 Input Frequency Division Ratio Select"]
    #[inline(always)]
    #[must_use]
    pub fn pl2idiv(&mut self) -> PL2IDIV_W<0> {
        PL2IDIV_W::new(self)
    }
    #[doc = "Bit 4 - PLL2 Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn pl2srcsel(&mut self) -> PL2SRCSEL_W<4> {
        PL2SRCSEL_W::new(self)
    }
    #[doc = "Bits 8:13 - PLL2 Frequency Multiplication Factor Select"]
    #[inline(always)]
    #[must_use]
    pub fn pll2mul(&mut self) -> PLL2MUL_W<8> {
        PLL2MUL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL2 Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll2ccr](index.html) module"]
pub struct PLL2CCR_SPEC;
impl crate::RegisterSpec for PLL2CCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pll2ccr::R](R) reader structure"]
impl crate::Readable for PLL2CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll2ccr::W](W) writer structure"]
impl crate::Writable for PLL2CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL2CCR to value 0x1300"]
impl crate::Resettable for PLL2CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1300;
}
