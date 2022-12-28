#[doc = "Register `VBATTMNSELR` reader"]
pub struct R(crate::R<VBATTMNSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VBATTMNSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VBATTMNSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VBATTMNSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VBATTMNSELR` writer"]
pub struct W(crate::W<VBATTMNSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VBATTMNSELR_SPEC>;
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
impl From<crate::W<VBATTMNSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VBATTMNSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBATTMNSEL` reader - VBATT Low Voltage Detect Function Select Bit"]
pub type VBATTMNSEL_R = crate::BitReader<VBATTMNSEL_A>;
#[doc = "VBATT Low Voltage Detect Function Select Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATTMNSEL_A {
    #[doc = "0: Disables VBATT low voltage detect function"]
    _0 = 0,
    #[doc = "1: Enables VBATT low voltage detect function"]
    _1 = 1,
}
impl From<VBATTMNSEL_A> for bool {
    #[inline(always)]
    fn from(variant: VBATTMNSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl VBATTMNSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBATTMNSEL_A {
        match self.bits {
            false => VBATTMNSEL_A::_0,
            true => VBATTMNSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBATTMNSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBATTMNSEL_A::_1
    }
}
#[doc = "Field `VBATTMNSEL` writer - VBATT Low Voltage Detect Function Select Bit"]
pub type VBATTMNSEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, VBATTMNSELR_SPEC, VBATTMNSEL_A, O>;
impl<'a, const O: u8> VBATTMNSEL_W<'a, O> {
    #[doc = "Disables VBATT low voltage detect function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VBATTMNSEL_A::_0)
    }
    #[doc = "Enables VBATT low voltage detect function"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VBATTMNSEL_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - VBATT Low Voltage Detect Function Select Bit"]
    #[inline(always)]
    pub fn vbattmnsel(&self) -> VBATTMNSEL_R {
        VBATTMNSEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBATT Low Voltage Detect Function Select Bit"]
    #[inline(always)]
    #[must_use]
    pub fn vbattmnsel(&mut self) -> VBATTMNSEL_W<0> {
        VBATTMNSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Battery Backup Voltage Monitor Function Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vbattmnselr](index.html) module"]
pub struct VBATTMNSELR_SPEC;
impl crate::RegisterSpec for VBATTMNSELR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [vbattmnselr::R](R) reader structure"]
impl crate::Readable for VBATTMNSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vbattmnselr::W](W) writer structure"]
impl crate::Writable for VBATTMNSELR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VBATTMNSELR to value 0"]
impl crate::Resettable for VBATTMNSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
