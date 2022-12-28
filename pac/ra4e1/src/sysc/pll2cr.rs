#[doc = "Register `PLL2CR` reader"]
pub struct R(crate::R<PLL2CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL2CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL2CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL2CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL2CR` writer"]
pub struct W(crate::W<PLL2CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL2CR_SPEC>;
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
impl From<crate::W<PLL2CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL2CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL2STP` reader - PLL2 Stop Control"]
pub type PLL2STP_R = crate::BitReader<PLL2STP_A>;
#[doc = "PLL2 Stop Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL2STP_A {
    #[doc = "0: PLL2 is operating"]
    _0 = 0,
    #[doc = "1: PLL2 is stopped."]
    _1 = 1,
}
impl From<PLL2STP_A> for bool {
    #[inline(always)]
    fn from(variant: PLL2STP_A) -> Self {
        variant as u8 != 0
    }
}
impl PLL2STP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL2STP_A {
        match self.bits {
            false => PLL2STP_A::_0,
            true => PLL2STP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLL2STP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLL2STP_A::_1
    }
}
#[doc = "Field `PLL2STP` writer - PLL2 Stop Control"]
pub type PLL2STP_W<'a, const O: u8> = crate::BitWriter<'a, u8, PLL2CR_SPEC, PLL2STP_A, O>;
impl<'a, const O: u8> PLL2STP_W<'a, O> {
    #[doc = "PLL2 is operating"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLL2STP_A::_0)
    }
    #[doc = "PLL2 is stopped."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLL2STP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - PLL2 Stop Control"]
    #[inline(always)]
    pub fn pll2stp(&self) -> PLL2STP_R {
        PLL2STP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL2 Stop Control"]
    #[inline(always)]
    #[must_use]
    pub fn pll2stp(&mut self) -> PLL2STP_W<0> {
        PLL2STP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL2 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll2cr](index.html) module"]
pub struct PLL2CR_SPEC;
impl crate::RegisterSpec for PLL2CR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pll2cr::R](R) reader structure"]
impl crate::Readable for PLL2CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll2cr::W](W) writer structure"]
impl crate::Writable for PLL2CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL2CR to value 0x01"]
impl crate::Resettable for PLL2CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
