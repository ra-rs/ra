#[doc = "Register `ADACSR` reader"]
pub struct R(crate::R<ADACSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADACSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADACSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADACSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADACSR` writer"]
pub struct W(crate::W<ADACSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADACSR_SPEC>;
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
impl From<crate::W<ADACSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADACSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADSAC` reader - Successive Approximation Control Setting"]
pub type ADSAC_R = crate::BitReader<ADSAC_A>;
#[doc = "Successive Approximation Control Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADSAC_A {
    #[doc = "0: Normal conversion mode (default)"]
    _0 = 0,
    #[doc = "1: Fast conversion mode"]
    _1 = 1,
}
impl From<ADSAC_A> for bool {
    #[inline(always)]
    fn from(variant: ADSAC_A) -> Self {
        variant as u8 != 0
    }
}
impl ADSAC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADSAC_A {
        match self.bits {
            false => ADSAC_A::_0,
            true => ADSAC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADSAC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADSAC_A::_1
    }
}
#[doc = "Field `ADSAC` writer - Successive Approximation Control Setting"]
pub type ADSAC_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADACSR_SPEC, ADSAC_A, O>;
impl<'a, const O: u8> ADSAC_W<'a, O> {
    #[doc = "Normal conversion mode (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADSAC_A::_0)
    }
    #[doc = "Fast conversion mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADSAC_A::_1)
    }
}
impl R {
    #[doc = "Bit 1 - Successive Approximation Control Setting"]
    #[inline(always)]
    pub fn adsac(&self) -> ADSAC_R {
        ADSAC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Successive Approximation Control Setting"]
    #[inline(always)]
    pub fn adsac(&mut self) -> ADSAC_W<1> {
        ADSAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Conversion Operation Mode Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adacsr](index.html) module"]
pub struct ADACSR_SPEC;
impl crate::RegisterSpec for ADACSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adacsr::R](R) reader structure"]
impl crate::Readable for ADACSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adacsr::W](W) writer structure"]
impl crate::Writable for ADACSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADACSR to value 0"]
impl crate::Resettable for ADACSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
