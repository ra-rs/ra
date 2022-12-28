#[doc = "Register `DPSWCR` reader"]
pub struct R(crate::R<DPSWCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPSWCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPSWCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPSWCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPSWCR` writer"]
pub struct W(crate::W<DPSWCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPSWCR_SPEC>;
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
impl From<crate::W<DPSWCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPSWCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WTSTS` reader - Deep Software Wait Standby Time Setting Bit"]
pub type WTSTS_R = crate::FieldReader<u8, WTSTS_A>;
#[doc = "Deep Software Wait Standby Time Setting Bit\n\nValue on reset: 25"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WTSTS_A {
    #[doc = "14: Wait cycle for fast recovery"]
    _0X0E = 14,
    #[doc = "25: Wait cycle for slow recovery"]
    _0X19 = 25,
}
impl From<WTSTS_A> for u8 {
    #[inline(always)]
    fn from(variant: WTSTS_A) -> Self {
        variant as _
    }
}
impl WTSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WTSTS_A> {
        match self.bits {
            14 => Some(WTSTS_A::_0X0E),
            25 => Some(WTSTS_A::_0X19),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0E`"]
    #[inline(always)]
    pub fn is_0x0e(&self) -> bool {
        *self == WTSTS_A::_0X0E
    }
    #[doc = "Checks if the value of the field is `_0X19`"]
    #[inline(always)]
    pub fn is_0x19(&self) -> bool {
        *self == WTSTS_A::_0X19
    }
}
#[doc = "Field `WTSTS` writer - Deep Software Wait Standby Time Setting Bit"]
pub type WTSTS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, DPSWCR_SPEC, u8, WTSTS_A, 6, O>;
impl<'a, const O: u8> WTSTS_W<'a, O> {
    #[doc = "Wait cycle for fast recovery"]
    #[inline(always)]
    pub fn _0x0e(self) -> &'a mut W {
        self.variant(WTSTS_A::_0X0E)
    }
    #[doc = "Wait cycle for slow recovery"]
    #[inline(always)]
    pub fn _0x19(self) -> &'a mut W {
        self.variant(WTSTS_A::_0X19)
    }
}
impl R {
    #[doc = "Bits 0:5 - Deep Software Wait Standby Time Setting Bit"]
    #[inline(always)]
    pub fn wtsts(&self) -> WTSTS_R {
        WTSTS_R::new(self.bits & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:5 - Deep Software Wait Standby Time Setting Bit"]
    #[inline(always)]
    #[must_use]
    pub fn wtsts(&mut self) -> WTSTS_W<0> {
        WTSTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep Software Standby Wait Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpswcr](index.html) module"]
pub struct DPSWCR_SPEC;
impl crate::RegisterSpec for DPSWCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dpswcr::R](R) reader structure"]
impl crate::Readable for DPSWCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpswcr::W](W) writer structure"]
impl crate::Writable for DPSWCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPSWCR to value 0x19"]
impl crate::Resettable for DPSWCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x19;
}
