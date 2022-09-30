#[doc = "Register `WDTCSTPR` reader"]
pub struct R(crate::R<WDTCSTPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCSTPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCSTPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCSTPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCSTPR` writer"]
pub struct W(crate::W<WDTCSTPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCSTPR_SPEC>;
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
impl From<crate::W<WDTCSTPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCSTPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLCSTP` reader - WDT Count Stop Control Register"]
pub type SLCSTP_R = crate::BitReader<SLCSTP_A>;
#[doc = "WDT Count Stop Control Register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLCSTP_A {
    #[doc = "0: Disable count stop"]
    _0 = 0,
    #[doc = "1: Stop count on transition to Sleep mode"]
    _1 = 1,
}
impl From<SLCSTP_A> for bool {
    #[inline(always)]
    fn from(variant: SLCSTP_A) -> Self {
        variant as u8 != 0
    }
}
impl SLCSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLCSTP_A {
        match self.bits {
            false => SLCSTP_A::_0,
            true => SLCSTP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLCSTP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLCSTP_A::_1
    }
}
#[doc = "Field `SLCSTP` writer - WDT Count Stop Control Register"]
pub type SLCSTP_W<'a, const O: u8> = crate::BitWriter<'a, u8, WDTCSTPR_SPEC, SLCSTP_A, O>;
impl<'a, const O: u8> SLCSTP_W<'a, O> {
    #[doc = "Disable count stop"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLCSTP_A::_0)
    }
    #[doc = "Stop count on transition to Sleep mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLCSTP_A::_1)
    }
}
impl R {
    #[doc = "Bit 7 - WDT Count Stop Control Register"]
    #[inline(always)]
    pub fn slcstp(&self) -> SLCSTP_R {
        SLCSTP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - WDT Count Stop Control Register"]
    #[inline(always)]
    pub fn slcstp(&mut self) -> SLCSTP_W<7> {
        SLCSTP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDT Count Stop Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtcstpr](index.html) module"]
pub struct WDTCSTPR_SPEC;
impl crate::RegisterSpec for WDTCSTPR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [wdtcstpr::R](R) reader structure"]
impl crate::Readable for WDTCSTPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtcstpr::W](W) writer structure"]
impl crate::Writable for WDTCSTPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDTCSTPR to value 0x80"]
impl crate::Resettable for WDTCSTPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}