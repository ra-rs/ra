#[doc = "Register `IOSR` reader"]
pub struct R(crate::R<IOSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOSR` writer"]
pub struct W(crate::W<IOSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOSR_SPEC>;
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
impl From<crate::W<IOSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ELB` reader - External Loopback Mode"]
pub type ELB_R = crate::BitReader<ELB_A>;
#[doc = "External Loopback Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELB_A {
    #[doc = "0: Output low on the ET0_EXOUT pin"]
    _0 = 0,
    #[doc = "1: Output high on the ET0_EXOUT pin."]
    _1 = 1,
}
impl From<ELB_A> for bool {
    #[inline(always)]
    fn from(variant: ELB_A) -> Self {
        variant as u8 != 0
    }
}
impl ELB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELB_A {
        match self.bits {
            false => ELB_A::_0,
            true => ELB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELB_A::_1
    }
}
#[doc = "Field `ELB` writer - External Loopback Mode"]
pub type ELB_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSR_SPEC, ELB_A, O>;
impl<'a, const O: u8> ELB_W<'a, O> {
    #[doc = "Output low on the ET0_EXOUT pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELB_A::_0)
    }
    #[doc = "Output high on the ET0_EXOUT pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELB_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - External Loopback Mode"]
    #[inline(always)]
    pub fn elb(&self) -> ELB_R {
        ELB_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Loopback Mode"]
    #[inline(always)]
    #[must_use]
    pub fn elb(&mut self) -> ELB_W<0> {
        ELB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Independent Output Signal Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iosr](index.html) module"]
pub struct IOSR_SPEC;
impl crate::RegisterSpec for IOSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iosr::R](R) reader structure"]
impl crate::Readable for IOSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iosr::W](W) writer structure"]
impl crate::Writable for IOSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOSR to value 0"]
impl crate::Resettable for IOSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
