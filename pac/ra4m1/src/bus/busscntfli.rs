#[doc = "Register `BUSSCNTFLI` reader"]
pub struct R(crate::R<BUSSCNTFLI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSSCNTFLI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSSCNTFLI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSSCNTFLI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUSSCNTFLI` writer"]
pub struct W(crate::W<BUSSCNTFLI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUSSCNTFLI_SPEC>;
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
impl From<crate::W<BUSSCNTFLI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUSSCNTFLI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARBMET` reader - Arbitration Method Specify the priority between groups"]
pub type ARBMET_R = crate::FieldReader<u8, ARBMET_A>;
#[doc = "Arbitration Method Specify the priority between groups\n\nValue on reset: 0"]
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
#[doc = "Field `ARBMET` writer - Arbitration Method Specify the priority between groups"]
pub type ARBMET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, BUSSCNTFLI_SPEC, u8, ARBMET_A, 2, O>;
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
    #[doc = "Bits 4:5 - Arbitration Method Specify the priority between groups"]
    #[inline(always)]
    pub fn arbmet(&self) -> ARBMET_R {
        ARBMET_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - Arbitration Method Specify the priority between groups"]
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
#[doc = "Slave Bus Control Register FLI\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busscntfli](index.html) module"]
pub struct BUSSCNTFLI_SPEC;
impl crate::RegisterSpec for BUSSCNTFLI_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [busscntfli::R](R) reader structure"]
impl crate::Readable for BUSSCNTFLI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [busscntfli::W](W) writer structure"]
impl crate::Writable for BUSSCNTFLI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUSSCNTFLI to value 0"]
impl crate::Resettable for BUSSCNTFLI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
