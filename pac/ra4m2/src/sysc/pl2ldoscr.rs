#[doc = "Register `PL2LDOSCR` reader"]
pub struct R(crate::R<PL2LDOSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PL2LDOSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PL2LDOSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PL2LDOSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PL2LDOSCR` writer"]
pub struct W(crate::W<PL2LDOSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PL2LDOSCR_SPEC>;
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
impl From<crate::W<PL2LDOSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PL2LDOSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PL2LDOSTP` reader - PLL2-LDO Stop"]
pub type PL2LDOSTP_R = crate::BitReader<PL2LDOSTP_A>;
#[doc = "PLL2-LDO Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PL2LDOSTP_A {
    #[doc = "0: PLL2-LDO is enabled"]
    _0 = 0,
    #[doc = "1: PLL2-LDO is stopped"]
    _1 = 1,
}
impl From<PL2LDOSTP_A> for bool {
    #[inline(always)]
    fn from(variant: PL2LDOSTP_A) -> Self {
        variant as u8 != 0
    }
}
impl PL2LDOSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PL2LDOSTP_A {
        match self.bits {
            false => PL2LDOSTP_A::_0,
            true => PL2LDOSTP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PL2LDOSTP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PL2LDOSTP_A::_1
    }
}
#[doc = "Field `PL2LDOSTP` writer - PLL2-LDO Stop"]
pub type PL2LDOSTP_W<'a, const O: u8> = crate::BitWriter<'a, u8, PL2LDOSCR_SPEC, PL2LDOSTP_A, O>;
impl<'a, const O: u8> PL2LDOSTP_W<'a, O> {
    #[doc = "PLL2-LDO is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PL2LDOSTP_A::_0)
    }
    #[doc = "PLL2-LDO is stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PL2LDOSTP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - PLL2-LDO Stop"]
    #[inline(always)]
    pub fn pl2ldostp(&self) -> PL2LDOSTP_R {
        PL2LDOSTP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL2-LDO Stop"]
    #[inline(always)]
    #[must_use]
    pub fn pl2ldostp(&mut self) -> PL2LDOSTP_W<0> {
        PL2LDOSTP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL2-LDO Stop Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pl2ldoscr](index.html) module"]
pub struct PL2LDOSCR_SPEC;
impl crate::RegisterSpec for PL2LDOSCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pl2ldoscr::R](R) reader structure"]
impl crate::Readable for PL2LDOSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pl2ldoscr::W](W) writer structure"]
impl crate::Writable for PL2LDOSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PL2LDOSCR to value 0"]
impl crate::Resettable for PL2LDOSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
