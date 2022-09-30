#[doc = "Register `WDTRCR` reader"]
pub struct R(crate::R<WDTRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTRCR` writer"]
pub struct W(crate::W<WDTRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTRCR_SPEC>;
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
impl From<crate::W<WDTRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSTIRQS` reader - Reset Interrupt Request Select"]
pub type RSTIRQS_R = crate::BitReader<RSTIRQS_A>;
#[doc = "Reset Interrupt Request Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTIRQS_A {
    #[doc = "0: Enable non-maskable interrupt request or interrupt request output"]
    _0 = 0,
    #[doc = "1: Enable reset output"]
    _1 = 1,
}
impl From<RSTIRQS_A> for bool {
    #[inline(always)]
    fn from(variant: RSTIRQS_A) -> Self {
        variant as u8 != 0
    }
}
impl RSTIRQS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTIRQS_A {
        match self.bits {
            false => RSTIRQS_A::_0,
            true => RSTIRQS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSTIRQS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSTIRQS_A::_1
    }
}
#[doc = "Field `RSTIRQS` writer - Reset Interrupt Request Select"]
pub type RSTIRQS_W<'a, const O: u8> = crate::BitWriter<'a, u8, WDTRCR_SPEC, RSTIRQS_A, O>;
impl<'a, const O: u8> RSTIRQS_W<'a, O> {
    #[doc = "Enable non-maskable interrupt request or interrupt request output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTIRQS_A::_0)
    }
    #[doc = "Enable reset output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTIRQS_A::_1)
    }
}
impl R {
    #[doc = "Bit 7 - Reset Interrupt Request Select"]
    #[inline(always)]
    pub fn rstirqs(&self) -> RSTIRQS_R {
        RSTIRQS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Reset Interrupt Request Select"]
    #[inline(always)]
    pub fn rstirqs(&mut self) -> RSTIRQS_W<7> {
        RSTIRQS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDT Reset Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtrcr](index.html) module"]
pub struct WDTRCR_SPEC;
impl crate::RegisterSpec for WDTRCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [wdtrcr::R](R) reader structure"]
impl crate::Readable for WDTRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtrcr::W](W) writer structure"]
impl crate::Writable for WDTRCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDTRCR to value 0x80"]
impl crate::Resettable for WDTRCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
