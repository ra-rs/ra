#[doc = "Register `NRDYSTS` reader"]
pub struct R(crate::R<NRDYSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NRDYSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NRDYSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NRDYSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NRDYSTS` writer"]
pub struct W(crate::W<NRDYSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NRDYSTS_SPEC>;
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
impl From<crate::W<NRDYSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NRDYSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIPENRDY` reader - NRDY Interrupt Status for Each Pipe"]
pub type PIPENRDY_R = crate::FieldReader<u16, PIPENRDY_A>;
#[doc = "NRDY Interrupt Status for Each Pipe\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PIPENRDY_A {
    #[doc = "0: Interrupts are not generated"]
    _0 = 0,
    #[doc = "1: Interrupts are generated"]
    _1 = 1,
}
impl From<PIPENRDY_A> for u16 {
    #[inline(always)]
    fn from(variant: PIPENRDY_A) -> Self {
        variant as _
    }
}
impl PIPENRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PIPENRDY_A> {
        match self.bits {
            0 => Some(PIPENRDY_A::_0),
            1 => Some(PIPENRDY_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPENRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPENRDY_A::_1
    }
}
#[doc = "Field `PIPENRDY` writer - NRDY Interrupt Status for Each Pipe"]
pub type PIPENRDY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, NRDYSTS_SPEC, u16, PIPENRDY_A, 10, O>;
impl<'a, const O: u8> PIPENRDY_W<'a, O> {
    #[doc = "Interrupts are not generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIPENRDY_A::_0)
    }
    #[doc = "Interrupts are generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIPENRDY_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:9 - NRDY Interrupt Status for Each Pipe"]
    #[inline(always)]
    pub fn pipenrdy(&self) -> PIPENRDY_R {
        PIPENRDY_R::new(self.bits & 0x03ff)
    }
}
impl W {
    #[doc = "Bits 0:9 - NRDY Interrupt Status for Each Pipe"]
    #[inline(always)]
    #[must_use]
    pub fn pipenrdy(&mut self) -> PIPENRDY_W<0> {
        PIPENRDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NRDY Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nrdysts](index.html) module"]
pub struct NRDYSTS_SPEC;
impl crate::RegisterSpec for NRDYSTS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [nrdysts::R](R) reader structure"]
impl crate::Readable for NRDYSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nrdysts::W](W) writer structure"]
impl crate::Writable for NRDYSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NRDYSTS to value 0"]
impl crate::Resettable for NRDYSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
