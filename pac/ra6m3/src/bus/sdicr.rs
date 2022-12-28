#[doc = "Register `SDICR` reader"]
pub struct R(crate::R<SDICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDICR` writer"]
pub struct W(crate::W<SDICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDICR_SPEC>;
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
impl From<crate::W<SDICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INIRQ` reader - Initialization Sequence Start"]
pub type INIRQ_R = crate::BitReader<INIRQ_A>;
#[doc = "Initialization Sequence Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INIRQ_A {
    #[doc = "0: Invalid"]
    _0 = 0,
    #[doc = "1: Initialization sequence starts"]
    _1 = 1,
}
impl From<INIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: INIRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl INIRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INIRQ_A {
        match self.bits {
            false => INIRQ_A::_0,
            true => INIRQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INIRQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INIRQ_A::_1
    }
}
#[doc = "Field `INIRQ` writer - Initialization Sequence Start"]
pub type INIRQ_W<'a, const O: u8> = crate::BitWriter<'a, u8, SDICR_SPEC, INIRQ_A, O>;
impl<'a, const O: u8> INIRQ_W<'a, O> {
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INIRQ_A::_0)
    }
    #[doc = "Initialization sequence starts"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INIRQ_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Initialization Sequence Start"]
    #[inline(always)]
    pub fn inirq(&self) -> INIRQ_R {
        INIRQ_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initialization Sequence Start"]
    #[inline(always)]
    #[must_use]
    pub fn inirq(&mut self) -> INIRQ_W<0> {
        INIRQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAM Initialization Sequence Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdicr](index.html) module"]
pub struct SDICR_SPEC;
impl crate::RegisterSpec for SDICR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sdicr::R](R) reader structure"]
impl crate::Readable for SDICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdicr::W](W) writer structure"]
impl crate::Writable for SDICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDICR to value 0"]
impl crate::Resettable for SDICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
