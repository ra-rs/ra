#[doc = "Register `PLLCCR` reader"]
pub struct R(crate::R<PLLCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLCCR` writer"]
pub struct W(crate::W<PLLCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCCR_SPEC>;
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
impl From<crate::W<PLLCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLIDIV` reader - PLL Input Frequency Division Ratio Select"]
pub type PLIDIV_R = crate::FieldReader<u8, PLIDIV_A>;
#[doc = "PLL Input Frequency Division Ratio Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLIDIV_A {
    #[doc = "0: /1"]
    _00 = 0,
    #[doc = "1: /2"]
    _01 = 1,
    #[doc = "2: /3"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<PLIDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PLIDIV_A) -> Self {
        variant as _
    }
}
impl PLIDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLIDIV_A {
        match self.bits {
            0 => PLIDIV_A::_00,
            1 => PLIDIV_A::_01,
            2 => PLIDIV_A::_10,
            3 => PLIDIV_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PLIDIV_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PLIDIV_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PLIDIV_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PLIDIV_A::_11
    }
}
#[doc = "Field `PLIDIV` writer - PLL Input Frequency Division Ratio Select"]
pub type PLIDIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, PLLCCR_SPEC, u8, PLIDIV_A, 2, O>;
impl<'a, const O: u8> PLIDIV_W<'a, O> {
    #[doc = "/1"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PLIDIV_A::_00)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PLIDIV_A::_01)
    }
    #[doc = "/3"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PLIDIV_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PLIDIV_A::_11)
    }
}
#[doc = "Field `PLSRCSEL` reader - PLL Clock Source Select"]
pub type PLSRCSEL_R = crate::BitReader<PLSRCSEL_A>;
#[doc = "PLL Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLSRCSEL_A {
    #[doc = "0: Main clock oscillator"]
    _0 = 0,
    #[doc = "1: HOCO"]
    _1 = 1,
}
impl From<PLSRCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PLSRCSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl PLSRCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLSRCSEL_A {
        match self.bits {
            false => PLSRCSEL_A::_0,
            true => PLSRCSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLSRCSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLSRCSEL_A::_1
    }
}
#[doc = "Field `PLSRCSEL` writer - PLL Clock Source Select"]
pub type PLSRCSEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, PLLCCR_SPEC, PLSRCSEL_A, O>;
impl<'a, const O: u8> PLSRCSEL_W<'a, O> {
    #[doc = "Main clock oscillator"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLSRCSEL_A::_0)
    }
    #[doc = "HOCO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLSRCSEL_A::_1)
    }
}
#[doc = "Field `PLLMUL` reader - PLL Frequency Multiplication Factor Select \\[PLL Frequency Multiplication Factor\\]
= (PLLUMUL+1) / 2 Range: 0x23 - 0x3B for example 010011: x10.0 010100: x10.5 010101: x11.0 : 011100: x14.5 011101: x15.0 011110: x15.5 : 111010: x29.5 111011: x30.0"]
pub type PLLMUL_R = crate::FieldReader<u8, PLLMUL_A>;
#[doc = "PLL Frequency Multiplication Factor Select \\[PLL Frequency Multiplication Factor\\]
= (PLLUMUL+1) / 2 Range: 0x23 - 0x3B for example 010011: x10.0 010100: x10.5 010101: x11.0 : 011100: x14.5 011101: x15.0 011110: x15.5 : 111010: x29.5 111011: x30.0\n\nValue on reset: 19"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLLMUL_A(u8);
impl From<PLLMUL_A> for u8 {
    #[inline(always)]
    fn from(val: PLLMUL_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `PLLMUL` writer - PLL Frequency Multiplication Factor Select \\[PLL Frequency Multiplication Factor\\]
= (PLLUMUL+1) / 2 Range: 0x23 - 0x3B for example 010011: x10.0 010100: x10.5 010101: x11.0 : 011100: x14.5 011101: x15.0 011110: x15.5 : 111010: x29.5 111011: x30.0"]
pub type PLLMUL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PLLCCR_SPEC, u8, PLLMUL_A, 6, O>;
impl R {
    #[doc = "Bits 0:1 - PLL Input Frequency Division Ratio Select"]
    #[inline(always)]
    pub fn plidiv(&self) -> PLIDIV_R {
        PLIDIV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - PLL Clock Source Select"]
    #[inline(always)]
    pub fn plsrcsel(&self) -> PLSRCSEL_R {
        PLSRCSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:13 - PLL Frequency Multiplication Factor Select \\[PLL Frequency Multiplication Factor\\]
= (PLLUMUL+1) / 2 Range: 0x23 - 0x3B for example 010011: x10.0 010100: x10.5 010101: x11.0 : 011100: x14.5 011101: x15.0 011110: x15.5 : 111010: x29.5 111011: x30.0"]
    #[inline(always)]
    pub fn pllmul(&self) -> PLLMUL_R {
        PLLMUL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PLL Input Frequency Division Ratio Select"]
    #[inline(always)]
    #[must_use]
    pub fn plidiv(&mut self) -> PLIDIV_W<0> {
        PLIDIV_W::new(self)
    }
    #[doc = "Bit 4 - PLL Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn plsrcsel(&mut self) -> PLSRCSEL_W<4> {
        PLSRCSEL_W::new(self)
    }
    #[doc = "Bits 8:13 - PLL Frequency Multiplication Factor Select \\[PLL Frequency Multiplication Factor\\]
= (PLLUMUL+1) / 2 Range: 0x23 - 0x3B for example 010011: x10.0 010100: x10.5 010101: x11.0 : 011100: x14.5 011101: x15.0 011110: x15.5 : 111010: x29.5 111011: x30.0"]
    #[inline(always)]
    #[must_use]
    pub fn pllmul(&mut self) -> PLLMUL_W<8> {
        PLLMUL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllccr](index.html) module"]
pub struct PLLCCR_SPEC;
impl crate::RegisterSpec for PLLCCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pllccr::R](R) reader structure"]
impl crate::Readable for PLLCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllccr::W](W) writer structure"]
impl crate::Writable for PLLCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLCCR to value 0x1300"]
impl crate::Resettable for PLLCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1300;
}
