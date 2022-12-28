#[doc = "Register `HOCOWTCR` reader"]
pub struct R(crate::R<HOCOWTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOCOWTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOCOWTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOCOWTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOCOWTCR` writer"]
pub struct W(crate::W<HOCOWTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOCOWTCR_SPEC>;
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
impl From<crate::W<HOCOWTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOCOWTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSTS` reader - HOCO Wait Time Setting"]
pub type HSTS_R = crate::FieldReader<u8, HSTS_A>;
#[doc = "HOCO Wait Time Setting\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HSTS_A {
    #[doc = "5: Value after reset."]
    _101 = 5,
    #[doc = "3: Before starting high-speed on-chip oscillator by setting HOCOCR.HCSTP bit, the HSTS\\[2:0\\]
bits must be set to 011b beforehand. Wait time = 46 cycles (5.75 µs) Wait time is calculated at MOCO = 8 MHz (typically 0.125 µs)."]
    _011 = 3,
}
impl From<HSTS_A> for u8 {
    #[inline(always)]
    fn from(variant: HSTS_A) -> Self {
        variant as _
    }
}
impl HSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HSTS_A> {
        match self.bits {
            5 => Some(HSTS_A::_101),
            3 => Some(HSTS_A::_011),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == HSTS_A::_101
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == HSTS_A::_011
    }
}
#[doc = "Field `HSTS` writer - HOCO Wait Time Setting"]
pub type HSTS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, HOCOWTCR_SPEC, u8, HSTS_A, 3, O>;
impl<'a, const O: u8> HSTS_W<'a, O> {
    #[doc = "Value after reset."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(HSTS_A::_101)
    }
    #[doc = "Before starting high-speed on-chip oscillator by setting HOCOCR.HCSTP bit, the HSTS\\[2:0\\]
bits must be set to 011b beforehand. Wait time = 46 cycles (5.75 µs) Wait time is calculated at MOCO = 8 MHz (typically 0.125 µs)."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(HSTS_A::_011)
    }
}
impl R {
    #[doc = "Bits 0:2 - HOCO Wait Time Setting"]
    #[inline(always)]
    pub fn hsts(&self) -> HSTS_R {
        HSTS_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - HOCO Wait Time Setting"]
    #[inline(always)]
    #[must_use]
    pub fn hsts(&mut self) -> HSTS_W<0> {
        HSTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High-Speed On-Chip Oscillator Wait Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hocowtcr](index.html) module"]
pub struct HOCOWTCR_SPEC;
impl crate::RegisterSpec for HOCOWTCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [hocowtcr::R](R) reader structure"]
impl crate::Readable for HOCOWTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hocowtcr::W](W) writer structure"]
impl crate::Writable for HOCOWTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOCOWTCR to value 0x05"]
impl crate::Resettable for HOCOWTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
