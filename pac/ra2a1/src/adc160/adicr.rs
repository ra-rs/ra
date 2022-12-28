#[doc = "Register `ADICR` reader"]
pub struct R(crate::R<ADICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADICR` writer"]
pub struct W(crate::W<ADICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADICR_SPEC>;
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
impl From<crate::W<ADICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADIC` reader - A/D Interrupt Control"]
pub type ADIC_R = crate::FieldReader<u8, ADIC_A>;
#[doc = "A/D Interrupt Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADIC_A {
    #[doc = "0: ADC160_ADI is generated at the end of A/D scan"]
    _00 = 0,
    #[doc = "3: ADC160_ADI is generated at the end of calibration"]
    _11 = 3,
}
impl From<ADIC_A> for u8 {
    #[inline(always)]
    fn from(variant: ADIC_A) -> Self {
        variant as _
    }
}
impl ADIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADIC_A> {
        match self.bits {
            0 => Some(ADIC_A::_00),
            3 => Some(ADIC_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ADIC_A::_00
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ADIC_A::_11
    }
}
#[doc = "Field `ADIC` writer - A/D Interrupt Control"]
pub type ADIC_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ADICR_SPEC, u8, ADIC_A, 2, O>;
impl<'a, const O: u8> ADIC_W<'a, O> {
    #[doc = "ADC160_ADI is generated at the end of A/D scan"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ADIC_A::_00)
    }
    #[doc = "ADC160_ADI is generated at the end of calibration"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ADIC_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - A/D Interrupt Control"]
    #[inline(always)]
    pub fn adic(&self) -> ADIC_R {
        ADIC_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - A/D Interrupt Control"]
    #[inline(always)]
    #[must_use]
    pub fn adic(&mut self) -> ADIC_W<0> {
        ADIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adicr](index.html) module"]
pub struct ADICR_SPEC;
impl crate::RegisterSpec for ADICR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adicr::R](R) reader structure"]
impl crate::Readable for ADICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adicr::W](W) writer structure"]
impl crate::Writable for ADICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADICR to value 0"]
impl crate::Resettable for ADICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
