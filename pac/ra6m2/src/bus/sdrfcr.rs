#[doc = "Register `SDRFCR` reader"]
pub struct R(crate::R<SDRFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDRFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDRFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDRFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDRFCR` writer"]
pub struct W(crate::W<SDRFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDRFCR_SPEC>;
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
impl From<crate::W<SDRFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDRFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFC` reader - Auto-Refresh Request Interval Setting"]
pub type RFC_R = crate::FieldReader<u16, RFC_A>;
#[doc = "Auto-Refresh Request Interval Setting\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum RFC_A {
    #[doc = "0: Setting prohibited"]
    _0X0 = 0,
}
impl From<RFC_A> for u16 {
    #[inline(always)]
    fn from(variant: RFC_A) -> Self {
        variant as _
    }
}
impl RFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RFC_A> {
        match self.bits {
            0 => Some(RFC_A::_0X0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == RFC_A::_0X0
    }
}
#[doc = "Field `RFC` writer - Auto-Refresh Request Interval Setting"]
pub type RFC_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SDRFCR_SPEC, u16, RFC_A, 12, O>;
impl<'a, const O: u8> RFC_W<'a, O> {
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(RFC_A::_0X0)
    }
}
#[doc = "Field `REFW` reader - Auto-Refresh Cycle/ Self-Refresh Clearing Cycle Count Setting. ( REFW+1 Cycles )"]
pub type REFW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REFW` writer - Auto-Refresh Cycle/ Self-Refresh Clearing Cycle Count Setting. ( REFW+1 Cycles )"]
pub type REFW_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SDRFCR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:11 - Auto-Refresh Request Interval Setting"]
    #[inline(always)]
    pub fn rfc(&self) -> RFC_R {
        RFC_R::new(self.bits & 0x0fff)
    }
    #[doc = "Bits 12:15 - Auto-Refresh Cycle/ Self-Refresh Clearing Cycle Count Setting. ( REFW+1 Cycles )"]
    #[inline(always)]
    pub fn refw(&self) -> REFW_R {
        REFW_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Auto-Refresh Request Interval Setting"]
    #[inline(always)]
    #[must_use]
    pub fn rfc(&mut self) -> RFC_W<0> {
        RFC_W::new(self)
    }
    #[doc = "Bits 12:15 - Auto-Refresh Cycle/ Self-Refresh Clearing Cycle Count Setting. ( REFW+1 Cycles )"]
    #[inline(always)]
    #[must_use]
    pub fn refw(&mut self) -> REFW_W<12> {
        REFW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAM Refresh Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdrfcr](index.html) module"]
pub struct SDRFCR_SPEC;
impl crate::RegisterSpec for SDRFCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sdrfcr::R](R) reader structure"]
impl crate::Readable for SDRFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdrfcr::W](W) writer structure"]
impl crate::Writable for SDRFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDRFCR to value 0x01"]
impl crate::Resettable for SDRFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
