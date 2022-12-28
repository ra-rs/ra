#[doc = "Register `BRDYSTS` reader"]
pub struct R(crate::R<BRDYSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRDYSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRDYSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRDYSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRDYSTS` writer"]
pub struct W(crate::W<BRDYSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRDYSTS_SPEC>;
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
impl From<crate::W<BRDYSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRDYSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIPEBRDY` reader - BRDY Interrupt Status for Each Pipe"]
pub type PIPEBRDY_R = crate::FieldReader<u16, PIPEBRDY_A>;
#[doc = "BRDY Interrupt Status for Each Pipe\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PIPEBRDY_A {
    #[doc = "0: Interrupts are not generated"]
    _0 = 0,
    #[doc = "1: Interrupts are generated"]
    _1 = 1,
}
impl From<PIPEBRDY_A> for u16 {
    #[inline(always)]
    fn from(variant: PIPEBRDY_A) -> Self {
        variant as _
    }
}
impl PIPEBRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PIPEBRDY_A> {
        match self.bits {
            0 => Some(PIPEBRDY_A::_0),
            1 => Some(PIPEBRDY_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPEBRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPEBRDY_A::_1
    }
}
#[doc = "Field `PIPEBRDY` writer - BRDY Interrupt Status for Each Pipe"]
pub type PIPEBRDY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, BRDYSTS_SPEC, u16, PIPEBRDY_A, 10, O>;
impl<'a, const O: u8> PIPEBRDY_W<'a, O> {
    #[doc = "Interrupts are not generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPEBRDY_A::_0)
    }
    #[doc = "Interrupts are generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPEBRDY_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:9 - BRDY Interrupt Status for Each Pipe"]
    #[inline(always)]
    pub fn pipebrdy(&self) -> PIPEBRDY_R {
        PIPEBRDY_R::new(self.bits & 0x03ff)
    }
}
impl W {
    #[doc = "Bits 0:9 - BRDY Interrupt Status for Each Pipe"]
    #[inline(always)]
    #[must_use]
    pub fn pipebrdy(&mut self) -> PIPEBRDY_W<0> {
        PIPEBRDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BRDY Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brdysts](index.html) module"]
pub struct BRDYSTS_SPEC;
impl crate::RegisterSpec for BRDYSTS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [brdysts::R](R) reader structure"]
impl crate::Readable for BRDYSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brdysts::W](W) writer structure"]
impl crate::Writable for BRDYSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BRDYSTS to value 0"]
impl crate::Resettable for BRDYSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
