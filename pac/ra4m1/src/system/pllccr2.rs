#[doc = "Register `PLLCCR2` reader"]
pub struct R(crate::R<PLLCCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLCCR2` writer"]
pub struct W(crate::W<PLLCCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCCR2_SPEC>;
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
impl From<crate::W<PLLCCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLMUL` reader - PLL Frequency Multiplication Factor Select"]
pub type PLLMUL_R = crate::FieldReader<u8, PLLMUL_A>;
#[doc = "PLL Frequency Multiplication Factor Select\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLMUL_A {
    #[doc = "15: Settings prohibited."]
    _1111 = 15,
}
impl From<PLLMUL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLMUL_A) -> Self {
        variant as _
    }
}
impl PLLMUL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLMUL_A> {
        match self.bits {
            15 => Some(PLLMUL_A::_1111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == PLLMUL_A::_1111
    }
}
#[doc = "Field `PLLMUL` writer - PLL Frequency Multiplication Factor Select"]
pub type PLLMUL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, PLLCCR2_SPEC, u8, PLLMUL_A, 5, O>;
impl<'a, const O: u8> PLLMUL_W<'a, O> {
    #[doc = "Settings prohibited."]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(PLLMUL_A::_1111)
    }
}
#[doc = "Field `PLODIV` reader - PLL Output Frequency Division Ratio Select"]
pub type PLODIV_R = crate::FieldReader<u8, PLODIV_A>;
#[doc = "PLL Output Frequency Division Ratio Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLODIV_A {
    #[doc = "0: /1."]
    _00 = 0,
    #[doc = "1: /2."]
    _01 = 1,
    #[doc = "2: /4."]
    _10 = 2,
    #[doc = "3: Setting prohibited."]
    _11 = 3,
}
impl From<PLODIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PLODIV_A) -> Self {
        variant as _
    }
}
impl PLODIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLODIV_A {
        match self.bits {
            0 => PLODIV_A::_00,
            1 => PLODIV_A::_01,
            2 => PLODIV_A::_10,
            3 => PLODIV_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PLODIV_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PLODIV_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PLODIV_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PLODIV_A::_11
    }
}
#[doc = "Field `PLODIV` writer - PLL Output Frequency Division Ratio Select"]
pub type PLODIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, PLLCCR2_SPEC, u8, PLODIV_A, 2, O>;
impl<'a, const O: u8> PLODIV_W<'a, O> {
    #[doc = "/1."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PLODIV_A::_00)
    }
    #[doc = "/2."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PLODIV_A::_01)
    }
    #[doc = "/4."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PLODIV_A::_10)
    }
    #[doc = "Setting prohibited."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PLODIV_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:4 - PLL Frequency Multiplication Factor Select"]
    #[inline(always)]
    pub fn pllmul(&self) -> PLLMUL_R {
        PLLMUL_R::new(self.bits & 0x1f)
    }
    #[doc = "Bits 6:7 - PLL Output Frequency Division Ratio Select"]
    #[inline(always)]
    pub fn plodiv(&self) -> PLODIV_R {
        PLODIV_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:4 - PLL Frequency Multiplication Factor Select"]
    #[inline(always)]
    #[must_use]
    pub fn pllmul(&mut self) -> PLLMUL_W<0> {
        PLLMUL_W::new(self)
    }
    #[doc = "Bits 6:7 - PLL Output Frequency Division Ratio Select"]
    #[inline(always)]
    #[must_use]
    pub fn plodiv(&mut self) -> PLODIV_W<6> {
        PLODIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Clock Control Register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllccr2](index.html) module"]
pub struct PLLCCR2_SPEC;
impl crate::RegisterSpec for PLLCCR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pllccr2::R](R) reader structure"]
impl crate::Readable for PLLCCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllccr2::W](W) writer structure"]
impl crate::Writable for PLLCCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLCCR2 to value 0x07"]
impl crate::Resettable for PLLCCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
