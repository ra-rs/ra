#[doc = "Register `TRACECTR` reader"]
pub struct R(crate::R<TRACECTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRACECTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRACECTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRACECTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRACECTR` writer"]
pub struct W(crate::W<TRACECTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRACECTR_SPEC>;
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
impl From<crate::W<TRACECTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRACECTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENETBFULL` reader - Enable bit for halt request by ETB full"]
pub type ENETBFULL_R = crate::BitReader<ENETBFULL_A>;
#[doc = "Enable bit for halt request by ETB full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENETBFULL_A {
    #[doc = "0: ETB full does not cause CPU halt"]
    _0 = 0,
    #[doc = "1: ETB full cause CPU halt"]
    _1 = 1,
}
impl From<ENETBFULL_A> for bool {
    #[inline(always)]
    fn from(variant: ENETBFULL_A) -> Self {
        variant as u8 != 0
    }
}
impl ENETBFULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENETBFULL_A {
        match self.bits {
            false => ENETBFULL_A::_0,
            true => ENETBFULL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENETBFULL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENETBFULL_A::_1
    }
}
#[doc = "Field `ENETBFULL` writer - Enable bit for halt request by ETB full"]
pub type ENETBFULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRACECTR_SPEC, ENETBFULL_A, O>;
impl<'a, const O: u8> ENETBFULL_W<'a, O> {
    #[doc = "ETB full does not cause CPU halt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENETBFULL_A::_0)
    }
    #[doc = "ETB full cause CPU halt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENETBFULL_A::_1)
    }
}
impl R {
    #[doc = "Bit 31 - Enable bit for halt request by ETB full"]
    #[inline(always)]
    pub fn enetbfull(&self) -> ENETBFULL_R {
        ENETBFULL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Enable bit for halt request by ETB full"]
    #[inline(always)]
    #[must_use]
    pub fn enetbfull(&mut self) -> ENETBFULL_W<31> {
        ENETBFULL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trace Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tracectr](index.html) module"]
pub struct TRACECTR_SPEC;
impl crate::RegisterSpec for TRACECTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tracectr::R](R) reader structure"]
impl crate::Readable for TRACECTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tracectr::W](W) writer structure"]
impl crate::Writable for TRACECTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRACECTR to value 0"]
impl crate::Resettable for TRACECTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
