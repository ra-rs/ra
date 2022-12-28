#[doc = "Register `NRDYENB` reader"]
pub struct R(crate::R<NRDYENB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NRDYENB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NRDYENB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NRDYENB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NRDYENB` writer"]
pub struct W(crate::W<NRDYENB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NRDYENB_SPEC>;
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
impl From<crate::W<NRDYENB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NRDYENB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIPENRDYE` reader - NRDY Interrupt Enable for Each Pipe"]
pub type PIPENRDYE_R = crate::FieldReader<u16, PIPENRDYE_A>;
#[doc = "NRDY Interrupt Enable for Each Pipe\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PIPENRDYE_A {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<PIPENRDYE_A> for u16 {
    #[inline(always)]
    fn from(variant: PIPENRDYE_A) -> Self {
        variant as _
    }
}
impl PIPENRDYE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PIPENRDYE_A> {
        match self.bits {
            0 => Some(PIPENRDYE_A::_0),
            1 => Some(PIPENRDYE_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPENRDYE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPENRDYE_A::_1
    }
}
#[doc = "Field `PIPENRDYE` writer - NRDY Interrupt Enable for Each Pipe"]
pub type PIPENRDYE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, NRDYENB_SPEC, u16, PIPENRDYE_A, 10, O>;
impl<'a, const O: u8> PIPENRDYE_W<'a, O> {
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPENRDYE_A::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPENRDYE_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:9 - NRDY Interrupt Enable for Each Pipe"]
    #[inline(always)]
    pub fn pipenrdye(&self) -> PIPENRDYE_R {
        PIPENRDYE_R::new(self.bits & 0x03ff)
    }
}
impl W {
    #[doc = "Bits 0:9 - NRDY Interrupt Enable for Each Pipe"]
    #[inline(always)]
    #[must_use]
    pub fn pipenrdye(&mut self) -> PIPENRDYE_W<0> {
        PIPENRDYE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NRDY Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nrdyenb](index.html) module"]
pub struct NRDYENB_SPEC;
impl crate::RegisterSpec for NRDYENB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [nrdyenb::R](R) reader structure"]
impl crate::Readable for NRDYENB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nrdyenb::W](W) writer structure"]
impl crate::Writable for NRDYENB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NRDYENB to value 0"]
impl crate::Resettable for NRDYENB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
