#[doc = "Register `CANFDCKDIVCR` reader"]
pub struct R(crate::R<CANFDCKDIVCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CANFDCKDIVCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CANFDCKDIVCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CANFDCKDIVCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CANFDCKDIVCR` writer"]
pub struct W(crate::W<CANFDCKDIVCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CANFDCKDIVCR_SPEC>;
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
impl From<crate::W<CANFDCKDIVCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CANFDCKDIVCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CANFDCKDIV` reader - CANFD clock (CANFDCLK) Division Select"]
pub type CANFDCKDIV_R = crate::FieldReader<u8, CANFDCKDIV_A>;
#[doc = "CANFD clock (CANFDCLK) Division Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CANFDCKDIV_A {
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
impl From<CANFDCKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CANFDCKDIV_A) -> Self {
        variant as _
    }
}
impl CANFDCKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CANFDCKDIV_A> {
        match self.bits {
            0 => Some(CANFDCKDIV_A::_000),
            1 => Some(CANFDCKDIV_A::_001),
            2 => Some(CANFDCKDIV_A::_010),
            3 => Some(CANFDCKDIV_A::_011),
            4 => Some(CANFDCKDIV_A::_100),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CANFDCKDIV_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CANFDCKDIV_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CANFDCKDIV_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CANFDCKDIV_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CANFDCKDIV_A::_100
    }
}
#[doc = "Field `CANFDCKDIV` writer - CANFD clock (CANFDCLK) Division Select"]
pub type CANFDCKDIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, CANFDCKDIVCR_SPEC, u8, CANFDCKDIV_A, 3, O>;
impl<'a, const O: u8> CANFDCKDIV_W<'a, O> {
    #[doc = "/1 (value after reset)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CANFDCKDIV_A::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CANFDCKDIV_A::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CANFDCKDIV_A::_010)
    }
    #[doc = "/6"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CANFDCKDIV_A::_011)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CANFDCKDIV_A::_100)
    }
}
impl R {
    #[doc = "Bits 0:2 - CANFD clock (CANFDCLK) Division Select"]
    #[inline(always)]
    pub fn canfdckdiv(&self) -> CANFDCKDIV_R {
        CANFDCKDIV_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - CANFD clock (CANFDCLK) Division Select"]
    #[inline(always)]
    #[must_use]
    pub fn canfdckdiv(&mut self) -> CANFDCKDIV_W<0> {
        CANFDCKDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CANFD Clock Division Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canfdckdivcr](index.html) module"]
pub struct CANFDCKDIVCR_SPEC;
impl crate::RegisterSpec for CANFDCKDIVCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [canfdckdivcr::R](R) reader structure"]
impl crate::Readable for CANFDCKDIVCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [canfdckdivcr::W](W) writer structure"]
impl crate::Writable for CANFDCKDIVCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CANFDCKDIVCR to value 0"]
impl crate::Resettable for CANFDCKDIVCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
