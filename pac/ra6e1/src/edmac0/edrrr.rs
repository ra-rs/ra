#[doc = "Register `EDRRR` reader"]
pub struct R(crate::R<EDRRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EDRRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EDRRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EDRRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EDRRR` writer"]
pub struct W(crate::W<EDRRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EDRRR_SPEC>;
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
impl From<crate::W<EDRRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EDRRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RR` reader - Receive Request"]
pub type RR_R = crate::BitReader<RR_A>;
#[doc = "Receive Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RR_A {
    #[doc = "0: Disable the receive function"]
    _0 = 0,
    #[doc = "1: Read receive descriptor and enable the receive function."]
    _1 = 1,
}
impl From<RR_A> for bool {
    #[inline(always)]
    fn from(variant: RR_A) -> Self {
        variant as u8 != 0
    }
}
impl RR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR_A {
        match self.bits {
            false => RR_A::_0,
            true => RR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RR_A::_1
    }
}
#[doc = "Field `RR` writer - Receive Request"]
pub type RR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDRRR_SPEC, RR_A, O>;
impl<'a, const O: u8> RR_W<'a, O> {
    #[doc = "Disable the receive function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RR_A::_0)
    }
    #[doc = "Read receive descriptor and enable the receive function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RR_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Receive Request"]
    #[inline(always)]
    pub fn rr(&self) -> RR_R {
        RR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Request"]
    #[inline(always)]
    #[must_use]
    pub fn rr(&mut self) -> RR_W<0> {
        RR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EDMAC Receive Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edrrr](index.html) module"]
pub struct EDRRR_SPEC;
impl crate::RegisterSpec for EDRRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [edrrr::R](R) reader structure"]
impl crate::Readable for EDRRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [edrrr::W](W) writer structure"]
impl crate::Writable for EDRRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EDRRR to value 0"]
impl crate::Resettable for EDRRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
