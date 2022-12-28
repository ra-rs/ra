#[doc = "Register `BEMPSTS` reader"]
pub struct R(crate::R<BEMPSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BEMPSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BEMPSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BEMPSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BEMPSTS` writer"]
pub struct W(crate::W<BEMPSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BEMPSTS_SPEC>;
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
impl From<crate::W<BEMPSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BEMPSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIPEBEMP` reader - BEMP Interrupt Status for Each Pipe"]
pub type PIPEBEMP_R = crate::FieldReader<u16, PIPEBEMP_A>;
#[doc = "BEMP Interrupt Status for Each Pipe\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PIPEBEMP_A {
    #[doc = "0: Interrupts are not generated"]
    _0 = 0,
    #[doc = "1: Interrupts are generated"]
    _1 = 1,
}
impl From<PIPEBEMP_A> for u16 {
    #[inline(always)]
    fn from(variant: PIPEBEMP_A) -> Self {
        variant as _
    }
}
impl PIPEBEMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PIPEBEMP_A> {
        match self.bits {
            0 => Some(PIPEBEMP_A::_0),
            1 => Some(PIPEBEMP_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPEBEMP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPEBEMP_A::_1
    }
}
#[doc = "Field `PIPEBEMP` writer - BEMP Interrupt Status for Each Pipe"]
pub type PIPEBEMP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, BEMPSTS_SPEC, u16, PIPEBEMP_A, 10, O>;
impl<'a, const O: u8> PIPEBEMP_W<'a, O> {
    #[doc = "Interrupts are not generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPEBEMP_A::_0)
    }
    #[doc = "Interrupts are generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPEBEMP_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:9 - BEMP Interrupt Status for Each Pipe"]
    #[inline(always)]
    pub fn pipebemp(&self) -> PIPEBEMP_R {
        PIPEBEMP_R::new(self.bits & 0x03ff)
    }
}
impl W {
    #[doc = "Bits 0:9 - BEMP Interrupt Status for Each Pipe"]
    #[inline(always)]
    #[must_use]
    pub fn pipebemp(&mut self) -> PIPEBEMP_W<0> {
        PIPEBEMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BEMP Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bempsts](index.html) module"]
pub struct BEMPSTS_SPEC;
impl crate::RegisterSpec for BEMPSTS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [bempsts::R](R) reader structure"]
impl crate::Readable for BEMPSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bempsts::W](W) writer structure"]
impl crate::Writable for BEMPSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BEMPSTS to value 0"]
impl crate::Resettable for BEMPSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
