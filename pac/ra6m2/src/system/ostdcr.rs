#[doc = "Register `OSTDCR` reader"]
pub struct R(crate::R<OSTDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSTDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSTDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSTDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSTDCR` writer"]
pub struct W(crate::W<OSTDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSTDCR_SPEC>;
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
impl From<crate::W<OSTDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSTDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSTDIE` reader - Oscillation Stop Detection Interrupt Enable"]
pub type OSTDIE_R = crate::BitReader<OSTDIE_A>;
#[doc = "Oscillation Stop Detection Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSTDIE_A {
    #[doc = "0: Disable oscillation stop detection interrupt (do not notify the POEG)"]
    _0 = 0,
    #[doc = "1: Enable oscillation stop detection interrupt (notify the POEG)"]
    _1 = 1,
}
impl From<OSTDIE_A> for bool {
    #[inline(always)]
    fn from(variant: OSTDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl OSTDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSTDIE_A {
        match self.bits {
            false => OSTDIE_A::_0,
            true => OSTDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OSTDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OSTDIE_A::_1
    }
}
#[doc = "Field `OSTDIE` writer - Oscillation Stop Detection Interrupt Enable"]
pub type OSTDIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, OSTDCR_SPEC, OSTDIE_A, O>;
impl<'a, const O: u8> OSTDIE_W<'a, O> {
    #[doc = "Disable oscillation stop detection interrupt (do not notify the POEG)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OSTDIE_A::_0)
    }
    #[doc = "Enable oscillation stop detection interrupt (notify the POEG)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OSTDIE_A::_1)
    }
}
#[doc = "Field `OSTDE` reader - Oscillation Stop Detection Function Enable"]
pub type OSTDE_R = crate::BitReader<OSTDE_A>;
#[doc = "Oscillation Stop Detection Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSTDE_A {
    #[doc = "0: Disable oscillation stop detection function"]
    _0 = 0,
    #[doc = "1: Enable oscillation stop detection function"]
    _1 = 1,
}
impl From<OSTDE_A> for bool {
    #[inline(always)]
    fn from(variant: OSTDE_A) -> Self {
        variant as u8 != 0
    }
}
impl OSTDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSTDE_A {
        match self.bits {
            false => OSTDE_A::_0,
            true => OSTDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OSTDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OSTDE_A::_1
    }
}
#[doc = "Field `OSTDE` writer - Oscillation Stop Detection Function Enable"]
pub type OSTDE_W<'a, const O: u8> = crate::BitWriter<'a, u8, OSTDCR_SPEC, OSTDE_A, O>;
impl<'a, const O: u8> OSTDE_W<'a, O> {
    #[doc = "Disable oscillation stop detection function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OSTDE_A::_0)
    }
    #[doc = "Enable oscillation stop detection function"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OSTDE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Oscillation Stop Detection Interrupt Enable"]
    #[inline(always)]
    pub fn ostdie(&self) -> OSTDIE_R {
        OSTDIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Oscillation Stop Detection Function Enable"]
    #[inline(always)]
    pub fn ostde(&self) -> OSTDE_R {
        OSTDE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Oscillation Stop Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ostdie(&mut self) -> OSTDIE_W<0> {
        OSTDIE_W::new(self)
    }
    #[doc = "Bit 7 - Oscillation Stop Detection Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ostde(&mut self) -> OSTDE_W<7> {
        OSTDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscillation Stop Detection Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ostdcr](index.html) module"]
pub struct OSTDCR_SPEC;
impl crate::RegisterSpec for OSTDCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ostdcr::R](R) reader structure"]
impl crate::Readable for OSTDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ostdcr::W](W) writer structure"]
impl crate::Writable for OSTDCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSTDCR to value 0"]
impl crate::Resettable for OSTDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
