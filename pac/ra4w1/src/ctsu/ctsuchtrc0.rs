#[doc = "Register `CTSUCHTRC0` reader"]
pub struct R(crate::R<CTSUCHTRC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUCHTRC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUCHTRC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUCHTRC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUCHTRC0` writer"]
pub struct W(crate::W<CTSUCHTRC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUCHTRC0_SPEC>;
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
impl From<crate::W<CTSUCHTRC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUCHTRC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTSUCHTRC0` reader - CTSU Channel Transmit/Receive Control 0"]
pub type CTSUCHTRC0_R = crate::FieldReader<u8, CTSUCHTRC0_A>;
#[doc = "CTSU Channel Transmit/Receive Control 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUCHTRC0_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CTSUCHTRC0_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUCHTRC0_A) -> Self {
        variant as _
    }
}
impl CTSUCHTRC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTSUCHTRC0_A> {
        match self.bits {
            0 => Some(CTSUCHTRC0_A::_0),
            1 => Some(CTSUCHTRC0_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUCHTRC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUCHTRC0_A::_1
    }
}
#[doc = "Field `CTSUCHTRC0` writer - CTSU Channel Transmit/Receive Control 0"]
pub type CTSUCHTRC0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, CTSUCHTRC0_SPEC, u8, CTSUCHTRC0_A, 8, O>;
impl<'a, const O: u8> CTSUCHTRC0_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSUCHTRC0_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSUCHTRC0_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - CTSU Channel Transmit/Receive Control 0"]
    #[inline(always)]
    pub fn ctsuchtrc0(&self) -> CTSUCHTRC0_R {
        CTSUCHTRC0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - CTSU Channel Transmit/Receive Control 0"]
    #[inline(always)]
    #[must_use]
    pub fn ctsuchtrc0(&mut self) -> CTSUCHTRC0_W<0> {
        CTSUCHTRC0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU Channel Transmit/Receive Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsuchtrc0](index.html) module"]
pub struct CTSUCHTRC0_SPEC;
impl crate::RegisterSpec for CTSUCHTRC0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctsuchtrc0::R](R) reader structure"]
impl crate::Readable for CTSUCHTRC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsuchtrc0::W](W) writer structure"]
impl crate::Writable for CTSUCHTRC0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUCHTRC0 to value 0"]
impl crate::Resettable for CTSUCHTRC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
