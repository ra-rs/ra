#[doc = "Register `DAASWCR` reader"]
pub struct R(crate::R<DAASWCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAASWCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAASWCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAASWCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAASWCR` writer"]
pub struct W(crate::W<DAASWCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAASWCR_SPEC>;
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
impl From<crate::W<DAASWCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAASWCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAASW0` reader - D/A Amplifier Stabilization Wait 0"]
pub type DAASW0_R = crate::BitReader<DAASW0_A>;
#[doc = "D/A Amplifier Stabilization Wait 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAASW0_A {
    #[doc = "0: Amplifier stabilization wait off (output) for channel 0"]
    _0 = 0,
    #[doc = "1: Amplifier stabilization wait on (high-Z) for channel 0"]
    _1 = 1,
}
impl From<DAASW0_A> for bool {
    #[inline(always)]
    fn from(variant: DAASW0_A) -> Self {
        variant as u8 != 0
    }
}
impl DAASW0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAASW0_A {
        match self.bits {
            false => DAASW0_A::_0,
            true => DAASW0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAASW0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAASW0_A::_1
    }
}
#[doc = "Field `DAASW0` writer - D/A Amplifier Stabilization Wait 0"]
pub type DAASW0_W<'a, const O: u8> = crate::BitWriter<'a, u8, DAASWCR_SPEC, DAASW0_A, O>;
impl<'a, const O: u8> DAASW0_W<'a, O> {
    #[doc = "Amplifier stabilization wait off (output) for channel 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DAASW0_A::_0)
    }
    #[doc = "Amplifier stabilization wait on (high-Z) for channel 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DAASW0_A::_1)
    }
}
impl R {
    #[doc = "Bit 6 - D/A Amplifier Stabilization Wait 0"]
    #[inline(always)]
    pub fn daasw0(&self) -> DAASW0_R {
        DAASW0_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - D/A Amplifier Stabilization Wait 0"]
    #[inline(always)]
    #[must_use]
    pub fn daasw0(&mut self) -> DAASW0_W<6> {
        DAASW0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "D/A Amplifier Stabilization Wait Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daaswcr](index.html) module"]
pub struct DAASWCR_SPEC;
impl crate::RegisterSpec for DAASWCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [daaswcr::R](R) reader structure"]
impl crate::Readable for DAASWCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [daaswcr::W](W) writer structure"]
impl crate::Writable for DAASWCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAASWCR to value 0"]
impl crate::Resettable for DAASWCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
