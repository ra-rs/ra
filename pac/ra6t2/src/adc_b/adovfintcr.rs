#[doc = "Register `ADOVFINTCR` reader"]
pub struct R(crate::R<ADOVFINTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADOVFINTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADOVFINTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADOVFINTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADOVFINTCR` writer"]
pub struct W(crate::W<ADOVFINTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADOVFINTCR_SPEC>;
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
impl From<crate::W<ADOVFINTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADOVFINTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADOVFIE0` reader - ADC0 A/D Conversion Overflow Interrupt Enable"]
pub type ADOVFIE0_R = crate::BitReader<ADOVFIE0_A>;
#[doc = "ADC0 A/D Conversion Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADOVFIE0_A {
    #[doc = "0: Disable A/D conversion overflow interrupt for ADC0"]
    _0 = 0,
    #[doc = "1: Enable A/D conversion overflow interrupt for ADC0"]
    _1 = 1,
}
impl From<ADOVFIE0_A> for bool {
    #[inline(always)]
    fn from(variant: ADOVFIE0_A) -> Self {
        variant as u8 != 0
    }
}
impl ADOVFIE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADOVFIE0_A {
        match self.bits {
            false => ADOVFIE0_A::_0,
            true => ADOVFIE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADOVFIE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADOVFIE0_A::_1
    }
}
#[doc = "Field `ADOVFIE0` writer - ADC0 A/D Conversion Overflow Interrupt Enable"]
pub type ADOVFIE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFINTCR_SPEC, ADOVFIE0_A, O>;
impl<'a, const O: u8> ADOVFIE0_W<'a, O> {
    #[doc = "Disable A/D conversion overflow interrupt for ADC0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADOVFIE0_A::_0)
    }
    #[doc = "Enable A/D conversion overflow interrupt for ADC0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADOVFIE0_A::_1)
    }
}
#[doc = "Field `ADOVFIE1` reader - ADC1 A/D Conversion Overflow Interrupt Enable"]
pub type ADOVFIE1_R = crate::BitReader<ADOVFIE1_A>;
#[doc = "ADC1 A/D Conversion Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADOVFIE1_A {
    #[doc = "0: Disable A/D conversion overflow interrupt for ADC1"]
    _0 = 0,
    #[doc = "1: Enable A/D conversion overflow interrupt for ADC1"]
    _1 = 1,
}
impl From<ADOVFIE1_A> for bool {
    #[inline(always)]
    fn from(variant: ADOVFIE1_A) -> Self {
        variant as u8 != 0
    }
}
impl ADOVFIE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADOVFIE1_A {
        match self.bits {
            false => ADOVFIE1_A::_0,
            true => ADOVFIE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADOVFIE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADOVFIE1_A::_1
    }
}
#[doc = "Field `ADOVFIE1` writer - ADC1 A/D Conversion Overflow Interrupt Enable"]
pub type ADOVFIE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFINTCR_SPEC, ADOVFIE1_A, O>;
impl<'a, const O: u8> ADOVFIE1_W<'a, O> {
    #[doc = "Disable A/D conversion overflow interrupt for ADC1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADOVFIE1_A::_0)
    }
    #[doc = "Enable A/D conversion overflow interrupt for ADC1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADOVFIE1_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - ADC0 A/D Conversion Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn adovfie0(&self) -> ADOVFIE0_R {
        ADOVFIE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC1 A/D Conversion Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn adovfie1(&self) -> ADOVFIE1_R {
        ADOVFIE1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC0 A/D Conversion Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adovfie0(&mut self) -> ADOVFIE0_W<0> {
        ADOVFIE0_W::new(self)
    }
    #[doc = "Bit 1 - ADC1 A/D Conversion Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adovfie1(&mut self) -> ADOVFIE1_W<1> {
        ADOVFIE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Conversion Overflow Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adovfintcr](index.html) module"]
pub struct ADOVFINTCR_SPEC;
impl crate::RegisterSpec for ADOVFINTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adovfintcr::R](R) reader structure"]
impl crate::Readable for ADOVFINTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adovfintcr::W](W) writer structure"]
impl crate::Writable for ADOVFINTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADOVFINTCR to value 0"]
impl crate::Resettable for ADOVFINTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
