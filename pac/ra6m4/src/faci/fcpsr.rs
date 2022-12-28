#[doc = "Register `FCPSR` reader"]
pub struct R(crate::R<FCPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCPSR` writer"]
pub struct W(crate::W<FCPSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCPSR_SPEC>;
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
impl From<crate::W<FCPSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCPSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ESUSPMD` reader - Erasure Suspend Mode"]
pub type ESUSPMD_R = crate::BitReader<ESUSPMD_A>;
#[doc = "Erasure Suspend Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESUSPMD_A {
    #[doc = "0: Suspension priority mode"]
    _0 = 0,
    #[doc = "1: Erasure priority mode."]
    _1 = 1,
}
impl From<ESUSPMD_A> for bool {
    #[inline(always)]
    fn from(variant: ESUSPMD_A) -> Self {
        variant as u8 != 0
    }
}
impl ESUSPMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESUSPMD_A {
        match self.bits {
            false => ESUSPMD_A::_0,
            true => ESUSPMD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ESUSPMD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ESUSPMD_A::_1
    }
}
#[doc = "Field `ESUSPMD` writer - Erasure Suspend Mode"]
pub type ESUSPMD_W<'a, const O: u8> = crate::BitWriter<'a, u16, FCPSR_SPEC, ESUSPMD_A, O>;
impl<'a, const O: u8> ESUSPMD_W<'a, O> {
    #[doc = "Suspension priority mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ESUSPMD_A::_0)
    }
    #[doc = "Erasure priority mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ESUSPMD_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Erasure Suspend Mode"]
    #[inline(always)]
    pub fn esuspmd(&self) -> ESUSPMD_R {
        ESUSPMD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Erasure Suspend Mode"]
    #[inline(always)]
    #[must_use]
    pub fn esuspmd(&mut self) -> ESUSPMD_W<0> {
        ESUSPMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Sequencer Processing Switching Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcpsr](index.html) module"]
pub struct FCPSR_SPEC;
impl crate::RegisterSpec for FCPSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fcpsr::R](R) reader structure"]
impl crate::Readable for FCPSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcpsr::W](W) writer structure"]
impl crate::Writable for FCPSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCPSR to value 0"]
impl crate::Resettable for FCPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
