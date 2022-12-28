#[doc = "Register `RMCR` reader"]
pub struct R(crate::R<RMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RMCR` writer"]
pub struct W(crate::W<RMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RMCR_SPEC>;
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
impl From<crate::W<RMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RNR` reader - Receive Request Reset"]
pub type RNR_R = crate::BitReader<RNR_A>;
#[doc = "Receive Request Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNR_A {
    #[doc = "0: EDRRR.RR bit (receive request bit) is cleared to 0 when one frame is received"]
    _0 = 0,
    #[doc = "1: EDRRR.RR bit (receive request bit) is not cleared to 0 when one frame is received."]
    _1 = 1,
}
impl From<RNR_A> for bool {
    #[inline(always)]
    fn from(variant: RNR_A) -> Self {
        variant as u8 != 0
    }
}
impl RNR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNR_A {
        match self.bits {
            false => RNR_A::_0,
            true => RNR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RNR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RNR_A::_1
    }
}
#[doc = "Field `RNR` writer - Receive Request Reset"]
pub type RNR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMCR_SPEC, RNR_A, O>;
impl<'a, const O: u8> RNR_W<'a, O> {
    #[doc = "EDRRR.RR bit (receive request bit) is cleared to 0 when one frame is received"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RNR_A::_0)
    }
    #[doc = "EDRRR.RR bit (receive request bit) is not cleared to 0 when one frame is received."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RNR_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Receive Request Reset"]
    #[inline(always)]
    pub fn rnr(&self) -> RNR_R {
        RNR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Request Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rnr(&mut self) -> RNR_W<0> {
        RNR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Method Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmcr](index.html) module"]
pub struct RMCR_SPEC;
impl crate::RegisterSpec for RMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rmcr::R](R) reader structure"]
impl crate::Readable for RMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rmcr::W](W) writer structure"]
impl crate::Writable for RMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RMCR to value 0"]
impl crate::Resettable for RMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
