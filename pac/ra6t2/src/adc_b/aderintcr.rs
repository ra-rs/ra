#[doc = "Register `ADERINTCR` reader"]
pub struct R(crate::R<ADERINTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADERINTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADERINTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADERINTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADERINTCR` writer"]
pub struct W(crate::W<ADERINTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADERINTCR_SPEC>;
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
impl From<crate::W<ADERINTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADERINTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADEIE0` reader - ADC0 A/D Conversion Error Interrupt Enable"]
pub type ADEIE0_R = crate::BitReader<ADEIE0_A>;
#[doc = "ADC0 A/D Conversion Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADEIE0_A {
    #[doc = "0: Disable A/D conversion error interrupt for ADC0"]
    _0 = 0,
    #[doc = "1: Enable A/D conversion error interrupt for ADC0"]
    _1 = 1,
}
impl From<ADEIE0_A> for bool {
    #[inline(always)]
    fn from(variant: ADEIE0_A) -> Self {
        variant as u8 != 0
    }
}
impl ADEIE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADEIE0_A {
        match self.bits {
            false => ADEIE0_A::_0,
            true => ADEIE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADEIE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADEIE0_A::_1
    }
}
#[doc = "Field `ADEIE0` writer - ADC0 A/D Conversion Error Interrupt Enable"]
pub type ADEIE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADERINTCR_SPEC, ADEIE0_A, O>;
impl<'a, const O: u8> ADEIE0_W<'a, O> {
    #[doc = "Disable A/D conversion error interrupt for ADC0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADEIE0_A::_0)
    }
    #[doc = "Enable A/D conversion error interrupt for ADC0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADEIE0_A::_1)
    }
}
#[doc = "Field `ADEIE1` reader - ADC1 A/D Conversion Error Interrupt Enable"]
pub type ADEIE1_R = crate::BitReader<ADEIE1_A>;
#[doc = "ADC1 A/D Conversion Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADEIE1_A {
    #[doc = "0: Disable A/D conversion error interrupt for ADC1"]
    _0 = 0,
    #[doc = "1: Enable A/D conversion error interrupt for ADC1"]
    _1 = 1,
}
impl From<ADEIE1_A> for bool {
    #[inline(always)]
    fn from(variant: ADEIE1_A) -> Self {
        variant as u8 != 0
    }
}
impl ADEIE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADEIE1_A {
        match self.bits {
            false => ADEIE1_A::_0,
            true => ADEIE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADEIE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADEIE1_A::_1
    }
}
#[doc = "Field `ADEIE1` writer - ADC1 A/D Conversion Error Interrupt Enable"]
pub type ADEIE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADERINTCR_SPEC, ADEIE1_A, O>;
impl<'a, const O: u8> ADEIE1_W<'a, O> {
    #[doc = "Disable A/D conversion error interrupt for ADC1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADEIE1_A::_0)
    }
    #[doc = "Enable A/D conversion error interrupt for ADC1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADEIE1_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - ADC0 A/D Conversion Error Interrupt Enable"]
    #[inline(always)]
    pub fn adeie0(&self) -> ADEIE0_R {
        ADEIE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC1 A/D Conversion Error Interrupt Enable"]
    #[inline(always)]
    pub fn adeie1(&self) -> ADEIE1_R {
        ADEIE1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC0 A/D Conversion Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adeie0(&mut self) -> ADEIE0_W<0> {
        ADEIE0_W::new(self)
    }
    #[doc = "Bit 1 - ADC1 A/D Conversion Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adeie1(&mut self) -> ADEIE1_W<1> {
        ADEIE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Conversion Error Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aderintcr](index.html) module"]
pub struct ADERINTCR_SPEC;
impl crate::RegisterSpec for ADERINTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aderintcr::R](R) reader structure"]
impl crate::Readable for ADERINTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aderintcr::W](W) writer structure"]
impl crate::Writable for ADERINTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADERINTCR to value 0"]
impl crate::Resettable for ADERINTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
