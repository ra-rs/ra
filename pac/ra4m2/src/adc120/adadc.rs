#[doc = "Register `ADADC` reader"]
pub struct R(crate::R<ADADC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADADC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADADC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADADC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADADC` writer"]
pub struct W(crate::W<ADADC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADADC_SPEC>;
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
impl From<crate::W<ADADC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADADC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC` reader - Addition/Average Count Select"]
pub type ADC_R = crate::FieldReader<u8, ADC_A>;
#[doc = "Addition/Average Count Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_A {
    #[doc = "0: 1-time conversion (no addition, same as normal conversion)"]
    _000 = 0,
    #[doc = "1: 2-time conversion (one addition)"]
    _001 = 1,
    #[doc = "2: 3-time conversion (two additions)Setting prohibited"]
    _010 = 2,
    #[doc = "3: 4-time conversion (three additions)"]
    _011 = 3,
    #[doc = "5: 16-time conversion (15 additions)"]
    _101 = 5,
}
impl From<ADC_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_A) -> Self {
        variant as _
    }
}
impl ADC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC_A> {
        match self.bits {
            0 => Some(ADC_A::_000),
            1 => Some(ADC_A::_001),
            2 => Some(ADC_A::_010),
            3 => Some(ADC_A::_011),
            5 => Some(ADC_A::_101),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == ADC_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == ADC_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == ADC_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == ADC_A::_011
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == ADC_A::_101
    }
}
#[doc = "Field `ADC` writer - Addition/Average Count Select"]
pub type ADC_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ADADC_SPEC, u8, ADC_A, 3, O>;
impl<'a, const O: u8> ADC_W<'a, O> {
    #[doc = "1-time conversion (no addition, same as normal conversion)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(ADC_A::_000)
    }
    #[doc = "2-time conversion (one addition)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(ADC_A::_001)
    }
    #[doc = "3-time conversion (two additions)Setting prohibited"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(ADC_A::_010)
    }
    #[doc = "4-time conversion (three additions)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(ADC_A::_011)
    }
    #[doc = "16-time conversion (15 additions)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(ADC_A::_101)
    }
}
#[doc = "Field `AVEE` reader - Average Mode Select"]
pub type AVEE_R = crate::BitReader<AVEE_A>;
#[doc = "Average Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVEE_A {
    #[doc = "0: Enable addition mode"]
    _0 = 0,
    #[doc = "1: Enable average mode"]
    _1 = 1,
}
impl From<AVEE_A> for bool {
    #[inline(always)]
    fn from(variant: AVEE_A) -> Self {
        variant as u8 != 0
    }
}
impl AVEE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVEE_A {
        match self.bits {
            false => AVEE_A::_0,
            true => AVEE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AVEE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AVEE_A::_1
    }
}
#[doc = "Field `AVEE` writer - Average Mode Select"]
pub type AVEE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADADC_SPEC, AVEE_A, O>;
impl<'a, const O: u8> AVEE_W<'a, O> {
    #[doc = "Enable addition mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AVEE_A::_0)
    }
    #[doc = "Enable average mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AVEE_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Addition/Average Count Select"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(self.bits & 7)
    }
    #[doc = "Bit 7 - Average Mode Select"]
    #[inline(always)]
    pub fn avee(&self) -> AVEE_R {
        AVEE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Addition/Average Count Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc(&mut self) -> ADC_W<0> {
        ADC_W::new(self)
    }
    #[doc = "Bit 7 - Average Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn avee(&mut self) -> AVEE_W<7> {
        AVEE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D-Converted Value Addition/Average Count Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adadc](index.html) module"]
pub struct ADADC_SPEC;
impl crate::RegisterSpec for ADADC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adadc::R](R) reader structure"]
impl crate::Readable for ADADC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adadc::W](W) writer structure"]
impl crate::Writable for ADADC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADADC to value 0"]
impl crate::Resettable for ADADC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
