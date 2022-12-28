#[doc = "Register `ADSYCR` reader"]
pub struct R(crate::R<ADSYCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADSYCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADSYCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADSYCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADSYCR` writer"]
pub struct W(crate::W<ADSYCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSYCR_SPEC>;
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
impl From<crate::W<ADSYCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSYCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADSYCYC` reader - Synchronous Operation Period Cycle"]
pub type ADSYCYC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADSYCYC` writer - Synchronous Operation Period Cycle"]
pub type ADSYCYC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSYCR_SPEC, u16, u16, 11, O>;
#[doc = "Field `ADSYDIS0` reader - ADC0 Synchronous Operation Select"]
pub type ADSYDIS0_R = crate::BitReader<ADSYDIS0_A>;
#[doc = "ADC0 Synchronous Operation Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSYDIS0_A {
    #[doc = "0: Enable ADC0 synchronous operation"]
    _0 = 0,
    #[doc = "1: Disable ADC0 synchronous operation"]
    _1 = 1,
}
impl From<ADSYDIS0_A> for bool {
    #[inline(always)]
    fn from(variant: ADSYDIS0_A) -> Self {
        variant as u8 != 0
    }
}
impl ADSYDIS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADSYDIS0_A {
        match self.bits {
            false => ADSYDIS0_A::_0,
            true => ADSYDIS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADSYDIS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADSYDIS0_A::_1
    }
}
#[doc = "Field `ADSYDIS0` writer - ADC0 Synchronous Operation Select"]
pub type ADSYDIS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSYCR_SPEC, ADSYDIS0_A, O>;
impl<'a, const O: u8> ADSYDIS0_W<'a, O> {
    #[doc = "Enable ADC0 synchronous operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADSYDIS0_A::_0)
    }
    #[doc = "Disable ADC0 synchronous operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADSYDIS0_A::_1)
    }
}
#[doc = "Field `ADSYDIS1` reader - ADC1 Synchronous Operation Select"]
pub type ADSYDIS1_R = crate::BitReader<ADSYDIS1_A>;
#[doc = "ADC1 Synchronous Operation Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSYDIS1_A {
    #[doc = "0: Enable ADC1 synchronous operation"]
    _0 = 0,
    #[doc = "1: Disable ADC1 synchronous operation"]
    _1 = 1,
}
impl From<ADSYDIS1_A> for bool {
    #[inline(always)]
    fn from(variant: ADSYDIS1_A) -> Self {
        variant as u8 != 0
    }
}
impl ADSYDIS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADSYDIS1_A {
        match self.bits {
            false => ADSYDIS1_A::_0,
            true => ADSYDIS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADSYDIS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADSYDIS1_A::_1
    }
}
#[doc = "Field `ADSYDIS1` writer - ADC1 Synchronous Operation Select"]
pub type ADSYDIS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSYCR_SPEC, ADSYDIS1_A, O>;
impl<'a, const O: u8> ADSYDIS1_W<'a, O> {
    #[doc = "Enable ADC1 synchronous operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADSYDIS1_A::_0)
    }
    #[doc = "Disable ADC1 synchronous operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADSYDIS1_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:10 - Synchronous Operation Period Cycle"]
    #[inline(always)]
    pub fn adsycyc(&self) -> ADSYCYC_R {
        ADSYCYC_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 16 - ADC0 Synchronous Operation Select"]
    #[inline(always)]
    pub fn adsydis0(&self) -> ADSYDIS0_R {
        ADSYDIS0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC1 Synchronous Operation Select"]
    #[inline(always)]
    pub fn adsydis1(&self) -> ADSYDIS1_R {
        ADSYDIS1_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Synchronous Operation Period Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn adsycyc(&mut self) -> ADSYCYC_W<0> {
        ADSYCYC_W::new(self)
    }
    #[doc = "Bit 16 - ADC0 Synchronous Operation Select"]
    #[inline(always)]
    #[must_use]
    pub fn adsydis0(&mut self) -> ADSYDIS0_W<16> {
        ADSYDIS0_W::new(self)
    }
    #[doc = "Bit 17 - ADC1 Synchronous Operation Select"]
    #[inline(always)]
    #[must_use]
    pub fn adsydis1(&mut self) -> ADSYDIS1_W<17> {
        ADSYDIS1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Converter Synchronous Operation Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adsycr](index.html) module"]
pub struct ADSYCR_SPEC;
impl crate::RegisterSpec for ADSYCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adsycr::R](R) reader structure"]
impl crate::Readable for ADSYCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adsycr::W](W) writer structure"]
impl crate::Writable for ADSYCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSYCR to value 0x05"]
impl crate::Resettable for ADSYCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
