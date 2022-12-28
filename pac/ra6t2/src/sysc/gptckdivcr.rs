#[doc = "Register `GPTCKDIVCR` reader"]
pub struct R(crate::R<GPTCKDIVCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPTCKDIVCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPTCKDIVCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPTCKDIVCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPTCKDIVCR` writer"]
pub struct W(crate::W<GPTCKDIVCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPTCKDIVCR_SPEC>;
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
impl From<crate::W<GPTCKDIVCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPTCKDIVCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPTCKDIV` reader - GPT clock (GPTCLK) Division Select"]
pub type GPTCKDIV_R = crate::FieldReader<u8, GPTCKDIV_A>;
#[doc = "GPT clock (GPTCLK) Division Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPTCKDIV_A {
    #[doc = "0: /1 (value after reset)"]
    _000 = 0,
    #[doc = "1: /2"]
    _001 = 1,
    #[doc = "2: /4"]
    _010 = 2,
    #[doc = "3: /6"]
    _011 = 3,
    #[doc = "4: /8"]
    _100 = 4,
}
impl From<GPTCKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: GPTCKDIV_A) -> Self {
        variant as _
    }
}
impl GPTCKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPTCKDIV_A> {
        match self.bits {
            0 => Some(GPTCKDIV_A::_000),
            1 => Some(GPTCKDIV_A::_001),
            2 => Some(GPTCKDIV_A::_010),
            3 => Some(GPTCKDIV_A::_011),
            4 => Some(GPTCKDIV_A::_100),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == GPTCKDIV_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == GPTCKDIV_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == GPTCKDIV_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == GPTCKDIV_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == GPTCKDIV_A::_100
    }
}
#[doc = "Field `GPTCKDIV` writer - GPT clock (GPTCLK) Division Select"]
pub type GPTCKDIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, GPTCKDIVCR_SPEC, u8, GPTCKDIV_A, 3, O>;
impl<'a, const O: u8> GPTCKDIV_W<'a, O> {
    #[doc = "/1 (value after reset)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(GPTCKDIV_A::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(GPTCKDIV_A::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(GPTCKDIV_A::_010)
    }
    #[doc = "/6"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(GPTCKDIV_A::_011)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(GPTCKDIV_A::_100)
    }
}
impl R {
    #[doc = "Bits 0:2 - GPT clock (GPTCLK) Division Select"]
    #[inline(always)]
    pub fn gptckdiv(&self) -> GPTCKDIV_R {
        GPTCKDIV_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - GPT clock (GPTCLK) Division Select"]
    #[inline(always)]
    #[must_use]
    pub fn gptckdiv(&mut self) -> GPTCKDIV_W<0> {
        GPTCKDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPT Clock Division Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptckdivcr](index.html) module"]
pub struct GPTCKDIVCR_SPEC;
impl crate::RegisterSpec for GPTCKDIVCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [gptckdivcr::R](R) reader structure"]
impl crate::Readable for GPTCKDIVCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gptckdivcr::W](W) writer structure"]
impl crate::Writable for GPTCKDIVCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPTCKDIVCR to value 0"]
impl crate::Resettable for GPTCKDIVCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
