#[doc = "Register `SOMCR` reader"]
pub struct R(crate::R<SOMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOMCR` writer"]
pub struct W(crate::W<SOMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOMCR_SPEC>;
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
impl From<crate::W<SOMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SODRV1` reader - Sub Clock Oscillator Drive Capability Switching"]
pub type SODRV1_R = crate::BitReader<SODRV1_A>;
#[doc = "Sub Clock Oscillator Drive Capability Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SODRV1_A {
    #[doc = "0: Standard"]
    _0 = 0,
    #[doc = "1: Middle"]
    _1 = 1,
}
impl From<SODRV1_A> for bool {
    #[inline(always)]
    fn from(variant: SODRV1_A) -> Self {
        variant as u8 != 0
    }
}
impl SODRV1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SODRV1_A {
        match self.bits {
            false => SODRV1_A::_0,
            true => SODRV1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SODRV1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SODRV1_A::_1
    }
}
#[doc = "Field `SODRV1` writer - Sub Clock Oscillator Drive Capability Switching"]
pub type SODRV1_W<'a, const O: u8> = crate::BitWriter<'a, u8, SOMCR_SPEC, SODRV1_A, O>;
impl<'a, const O: u8> SODRV1_W<'a, O> {
    #[doc = "Standard"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SODRV1_A::_0)
    }
    #[doc = "Middle"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SODRV1_A::_1)
    }
}
impl R {
    #[doc = "Bit 1 - Sub Clock Oscillator Drive Capability Switching"]
    #[inline(always)]
    pub fn sodrv1(&self) -> SODRV1_R {
        SODRV1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Sub Clock Oscillator Drive Capability Switching"]
    #[inline(always)]
    #[must_use]
    pub fn sodrv1(&mut self) -> SODRV1_W<1> {
        SODRV1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sub Clock Oscillator Mode Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [somcr](index.html) module"]
pub struct SOMCR_SPEC;
impl crate::RegisterSpec for SOMCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [somcr::R](R) reader structure"]
impl crate::Readable for SOMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [somcr::W](W) writer structure"]
impl crate::Writable for SOMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOMCR to value 0"]
impl crate::Resettable for SOMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
