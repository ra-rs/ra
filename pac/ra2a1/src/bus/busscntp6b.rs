#[doc = "Register `BUSSCNTP6B` reader"]
pub struct R(crate::R<BUSSCNTP6B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSSCNTP6B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSSCNTP6B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSSCNTP6B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUSSCNTP6B` writer"]
pub struct W(crate::W<BUSSCNTP6B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUSSCNTP6B_SPEC>;
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
impl From<crate::W<BUSSCNTP6B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUSSCNTP6B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARBMET` reader - Arbitration MethodSpecify the priority between groups"]
pub type ARBMET_R = crate::FieldReader<u8, ARBMET_A>;
#[doc = "Arbitration MethodSpecify the priority between groups\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ARBMET_A {
    #[doc = "0: fixed priority"]
    _00 = 0,
    #[doc = "1: round-robin"]
    _01 = 1,
}
impl From<ARBMET_A> for u8 {
    #[inline(always)]
    fn from(variant: ARBMET_A) -> Self {
        variant as _
    }
}
impl ARBMET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ARBMET_A> {
        match self.bits {
            0 => Some(ARBMET_A::_00),
            1 => Some(ARBMET_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ARBMET_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ARBMET_A::_01
    }
}
#[doc = "Field `ARBMET` writer - Arbitration MethodSpecify the priority between groups"]
pub type ARBMET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, BUSSCNTP6B_SPEC, u8, ARBMET_A, 2, O>;
impl<'a, const O: u8> ARBMET_W<'a, O> {
    #[doc = "fixed priority"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ARBMET_A::_00)
    }
    #[doc = "round-robin"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ARBMET_A::_01)
    }
}
impl R {
    #[doc = "Bits 4:5 - Arbitration MethodSpecify the priority between groups"]
    #[inline(always)]
    pub fn arbmet(&self) -> ARBMET_R {
        ARBMET_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - Arbitration MethodSpecify the priority between groups"]
    #[inline(always)]
    #[must_use]
    pub fn arbmet(&mut self) -> ARBMET_W<4> {
        ARBMET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Bus Control Register P6B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busscntp6b](index.html) module"]
pub struct BUSSCNTP6B_SPEC;
impl crate::RegisterSpec for BUSSCNTP6B_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [busscntp6b::R](R) reader structure"]
impl crate::Readable for BUSSCNTP6B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [busscntp6b::W](W) writer structure"]
impl crate::Writable for BUSSCNTP6B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUSSCNTP6B to value 0"]
impl crate::Resettable for BUSSCNTP6B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
