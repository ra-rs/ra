#[doc = "Register `PESAR` reader"]
pub struct R(crate::R<PESAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PESAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PESAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PESAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PESAR` writer"]
pub struct W(crate::W<PESAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PESAR_SPEC>;
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
impl From<crate::W<PESAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PESAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMNSA` reader - Pmn Security Attribution"]
pub type PMNSA_R = crate::FieldReader<u16, PMNSA_A>;
#[doc = "Pmn Security Attribution\n\nValue on reset: 65535"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PMNSA_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<PMNSA_A> for u16 {
    #[inline(always)]
    fn from(variant: PMNSA_A) -> Self {
        variant as _
    }
}
impl PMNSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PMNSA_A> {
        match self.bits {
            0 => Some(PMNSA_A::_0),
            1 => Some(PMNSA_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PMNSA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PMNSA_A::_1
    }
}
#[doc = "Field `PMNSA` writer - Pmn Security Attribution"]
pub type PMNSA_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PESAR_SPEC, u16, PMNSA_A, 16, O>;
impl<'a, const O: u8> PMNSA_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PMNSA_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PMNSA_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:15 - Pmn Security Attribution"]
    #[inline(always)]
    pub fn pmnsa(&self) -> PMNSA_R {
        PMNSA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pmn Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn pmnsa(&mut self) -> PMNSA_W<0> {
        PMNSA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port E Security Attribution register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pesar](index.html) module"]
pub struct PESAR_SPEC;
impl crate::RegisterSpec for PESAR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pesar::R](R) reader structure"]
impl crate::Readable for PESAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pesar::W](W) writer structure"]
impl crate::Writable for PESAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PESAR to value 0xffff"]
impl crate::Resettable for PESAR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
