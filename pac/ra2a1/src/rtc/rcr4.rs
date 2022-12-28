#[doc = "Register `RCR4` reader"]
pub struct R(crate::R<RCR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCR4` writer"]
pub struct W(crate::W<RCR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCR4_SPEC>;
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
impl From<crate::W<RCR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCKSEL` reader - Count Source Select"]
pub type RCKSEL_R = crate::BitReader<RCKSEL_A>;
#[doc = "Count Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCKSEL_A {
    #[doc = "0: Sub-clock oscillator is selected."]
    _0 = 0,
    #[doc = "1: LOCO clock oscillator is selected."]
    _1 = 1,
}
impl From<RCKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: RCKSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl RCKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCKSEL_A {
        match self.bits {
            false => RCKSEL_A::_0,
            true => RCKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCKSEL_A::_1
    }
}
#[doc = "Field `RCKSEL` writer - Count Source Select"]
pub type RCKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, RCR4_SPEC, RCKSEL_A, O>;
impl<'a, const O: u8> RCKSEL_W<'a, O> {
    #[doc = "Sub-clock oscillator is selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCKSEL_A::_0)
    }
    #[doc = "LOCO clock oscillator is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCKSEL_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Count Source Select"]
    #[inline(always)]
    pub fn rcksel(&self) -> RCKSEL_R {
        RCKSEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Count Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn rcksel(&mut self) -> RCKSEL_W<0> {
        RCKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcr4](index.html) module"]
pub struct RCR4_SPEC;
impl crate::RegisterSpec for RCR4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rcr4::R](R) reader structure"]
impl crate::Readable for RCR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcr4::W](W) writer structure"]
impl crate::Writable for RCR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCR4 to value 0"]
impl crate::Resettable for RCR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
