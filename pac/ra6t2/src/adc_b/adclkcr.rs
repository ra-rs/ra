#[doc = "Register `ADCLKCR` reader"]
pub struct R(crate::R<ADCLKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCLKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCLKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCLKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCLKCR` writer"]
pub struct W(crate::W<ADCLKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCLKCR_SPEC>;
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
impl From<crate::W<ADCLKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCLKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKSEL` reader - ADCLK Clock Source Select"]
pub type CLKSEL_R = crate::FieldReader<u8, CLKSEL_A>;
#[doc = "ADCLK Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: Peripheral Module Clock C"]
    PCLKC = 0,
    #[doc = "1: GPT clock"]
    GPTCLK = 1,
    #[doc = "2: Peripheral Module Clock A"]
    PCLKA = 2,
    #[doc = "3: Setting prohibited"]
    INVALID = 3,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSEL_A {
        match self.bits {
            0 => CLKSEL_A::PCLKC,
            1 => CLKSEL_A::GPTCLK,
            2 => CLKSEL_A::PCLKA,
            3 => CLKSEL_A::INVALID,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PCLKC`"]
    #[inline(always)]
    pub fn is_pclkc(&self) -> bool {
        *self == CLKSEL_A::PCLKC
    }
    #[doc = "Checks if the value of the field is `GPTCLK`"]
    #[inline(always)]
    pub fn is_gptclk(&self) -> bool {
        *self == CLKSEL_A::GPTCLK
    }
    #[doc = "Checks if the value of the field is `PCLKA`"]
    #[inline(always)]
    pub fn is_pclka(&self) -> bool {
        *self == CLKSEL_A::PCLKA
    }
    #[doc = "Checks if the value of the field is `INVALID`"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == CLKSEL_A::INVALID
    }
}
#[doc = "Field `CLKSEL` writer - ADCLK Clock Source Select"]
pub type CLKSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADCLKCR_SPEC, u8, CLKSEL_A, 2, O>;
impl<'a, const O: u8> CLKSEL_W<'a, O> {
    #[doc = "Peripheral Module Clock C"]
    #[inline(always)]
    pub fn pclkc(self) -> &'a mut W {
        self.variant(CLKSEL_A::PCLKC)
    }
    #[doc = "GPT clock"]
    #[inline(always)]
    pub fn gptclk(self) -> &'a mut W {
        self.variant(CLKSEL_A::GPTCLK)
    }
    #[doc = "Peripheral Module Clock A"]
    #[inline(always)]
    pub fn pclka(self) -> &'a mut W {
        self.variant(CLKSEL_A::PCLKA)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn invalid(self) -> &'a mut W {
        self.variant(CLKSEL_A::INVALID)
    }
}
#[doc = "Field `DIVR` reader - Clock Division Ratio Select"]
pub type DIVR_R = crate::FieldReader<u8, DIVR_A>;
#[doc = "Clock Division Ratio Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVR_A {
    #[doc = "0: 1/1"]
    _000 = 0,
    #[doc = "1: 1/2"]
    _001 = 1,
    #[doc = "2: 1/3"]
    _010 = 2,
    #[doc = "3: 1/4"]
    _011 = 3,
    #[doc = "4: 1/5"]
    _100 = 4,
    #[doc = "5: 1/6"]
    _101 = 5,
    #[doc = "6: 1/7"]
    _110 = 6,
    #[doc = "7: 1/8"]
    _111 = 7,
}
impl From<DIVR_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVR_A) -> Self {
        variant as _
    }
}
impl DIVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVR_A {
        match self.bits {
            0 => DIVR_A::_000,
            1 => DIVR_A::_001,
            2 => DIVR_A::_010,
            3 => DIVR_A::_011,
            4 => DIVR_A::_100,
            5 => DIVR_A::_101,
            6 => DIVR_A::_110,
            7 => DIVR_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DIVR_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DIVR_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DIVR_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DIVR_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DIVR_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DIVR_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == DIVR_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == DIVR_A::_111
    }
}
#[doc = "Field `DIVR` writer - Clock Division Ratio Select"]
pub type DIVR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ADCLKCR_SPEC, u8, DIVR_A, 3, O>;
impl<'a, const O: u8> DIVR_W<'a, O> {
    #[doc = "1/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(DIVR_A::_000)
    }
    #[doc = "1/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(DIVR_A::_001)
    }
    #[doc = "1/3"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(DIVR_A::_010)
    }
    #[doc = "1/4"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(DIVR_A::_011)
    }
    #[doc = "1/5"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(DIVR_A::_100)
    }
    #[doc = "1/6"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(DIVR_A::_101)
    }
    #[doc = "1/7"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(DIVR_A::_110)
    }
    #[doc = "1/8"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(DIVR_A::_111)
    }
}
impl R {
    #[doc = "Bits 0:1 - ADCLK Clock Source Select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:18 - Clock Division Ratio Select"]
    #[inline(always)]
    pub fn divr(&self) -> DIVR_R {
        DIVR_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADCLK Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<0> {
        CLKSEL_W::new(self)
    }
    #[doc = "Bits 16:18 - Clock Division Ratio Select"]
    #[inline(always)]
    #[must_use]
    pub fn divr(&mut self) -> DIVR_W<16> {
        DIVR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Conversion Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adclkcr](index.html) module"]
pub struct ADCLKCR_SPEC;
impl crate::RegisterSpec for ADCLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adclkcr::R](R) reader structure"]
impl crate::Readable for ADCLKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adclkcr::W](W) writer structure"]
impl crate::Writable for ADCLKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCLKCR to value 0"]
impl crate::Resettable for ADCLKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
