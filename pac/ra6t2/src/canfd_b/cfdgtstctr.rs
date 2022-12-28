#[doc = "Register `CFDGTSTCTR` reader"]
pub struct R(crate::R<CFDGTSTCTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDGTSTCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDGTSTCTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDGTSTCTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDGTSTCTR` writer"]
pub struct W(crate::W<CFDGTSTCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDGTSTCTR_SPEC>;
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
impl From<crate::W<CFDGTSTCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDGTSTCTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTME` reader - RAM Test Mode Enable"]
pub type RTME_R = crate::BitReader<RTME_A>;
#[doc = "RAM Test Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTME_A {
    #[doc = "0: RAM test mode disabled"]
    _0 = 0,
    #[doc = "1: RAM test mode enabled"]
    _1 = 1,
}
impl From<RTME_A> for bool {
    #[inline(always)]
    fn from(variant: RTME_A) -> Self {
        variant as u8 != 0
    }
}
impl RTME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTME_A {
        match self.bits {
            false => RTME_A::_0,
            true => RTME_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTME_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTME_A::_1
    }
}
#[doc = "Field `RTME` writer - RAM Test Mode Enable"]
pub type RTME_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGTSTCTR_SPEC, RTME_A, O>;
impl<'a, const O: u8> RTME_W<'a, O> {
    #[doc = "RAM test mode disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTME_A::_0)
    }
    #[doc = "RAM test mode enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTME_A::_1)
    }
}
impl R {
    #[doc = "Bit 2 - RAM Test Mode Enable"]
    #[inline(always)]
    pub fn rtme(&self) -> RTME_R {
        RTME_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - RAM Test Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtme(&mut self) -> RTME_W<2> {
        RTME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Test Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdgtstctr](index.html) module"]
pub struct CFDGTSTCTR_SPEC;
impl crate::RegisterSpec for CFDGTSTCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdgtstctr::R](R) reader structure"]
impl crate::Readable for CFDGTSTCTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdgtstctr::W](W) writer structure"]
impl crate::Writable for CFDGTSTCTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDGTSTCTR to value 0"]
impl crate::Resettable for CFDGTSTCTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
