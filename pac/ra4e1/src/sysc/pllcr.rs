#[doc = "Register `PLLCR` reader"]
pub struct R(crate::R<PLLCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLCR` writer"]
pub struct W(crate::W<PLLCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCR_SPEC>;
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
impl From<crate::W<PLLCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLSTP` reader - PLL Stop Control"]
pub type PLLSTP_R = crate::BitReader<PLLSTP_A>;
#[doc = "PLL Stop Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSTP_A {
    #[doc = "0: PLL is operating"]
    _0 = 0,
    #[doc = "1: PLL is stopped."]
    _1 = 1,
}
impl From<PLLSTP_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSTP_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSTP_A {
        match self.bits {
            false => PLLSTP_A::_0,
            true => PLLSTP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLLSTP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLLSTP_A::_1
    }
}
#[doc = "Field `PLLSTP` writer - PLL Stop Control"]
pub type PLLSTP_W<'a, const O: u8> = crate::BitWriter<'a, u8, PLLCR_SPEC, PLLSTP_A, O>;
impl<'a, const O: u8> PLLSTP_W<'a, O> {
    #[doc = "PLL is operating"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLSTP_A::_0)
    }
    #[doc = "PLL is stopped."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLSTP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - PLL Stop Control"]
    #[inline(always)]
    pub fn pllstp(&self) -> PLLSTP_R {
        PLLSTP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL Stop Control"]
    #[inline(always)]
    #[must_use]
    pub fn pllstp(&mut self) -> PLLSTP_W<0> {
        PLLSTP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcr](index.html) module"]
pub struct PLLCR_SPEC;
impl crate::RegisterSpec for PLLCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pllcr::R](R) reader structure"]
impl crate::Readable for PLLCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllcr::W](W) writer structure"]
impl crate::Writable for PLLCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLCR to value 0x01"]
impl crate::Resettable for PLLCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
