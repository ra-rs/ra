#[doc = "Register `AGTISR` reader"]
pub struct R(crate::R<AGTISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AGTISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AGTISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AGTISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AGTISR` writer"]
pub struct W(crate::W<AGTISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AGTISR_SPEC>;
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
impl From<crate::W<AGTISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AGTISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EEPS` reader - AGTEE polarty selection"]
pub type EEPS_R = crate::BitReader<EEPS_A>;
#[doc = "AGTEE polarty selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEPS_A {
    #[doc = "0: An event is counted during the low-level period"]
    _0 = 0,
    #[doc = "1: An event is counted during the high-level period"]
    _1 = 1,
}
impl From<EEPS_A> for bool {
    #[inline(always)]
    fn from(variant: EEPS_A) -> Self {
        variant as u8 != 0
    }
}
impl EEPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEPS_A {
        match self.bits {
            false => EEPS_A::_0,
            true => EEPS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EEPS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EEPS_A::_1
    }
}
#[doc = "Field `EEPS` writer - AGTEE polarty selection"]
pub type EEPS_W<'a, const O: u8> = crate::BitWriter<'a, u8, AGTISR_SPEC, EEPS_A, O>;
impl<'a, const O: u8> EEPS_W<'a, O> {
    #[doc = "An event is counted during the low-level period"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEPS_A::_0)
    }
    #[doc = "An event is counted during the high-level period"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEPS_A::_1)
    }
}
impl R {
    #[doc = "Bit 2 - AGTEE polarty selection"]
    #[inline(always)]
    pub fn eeps(&self) -> EEPS_R {
        EEPS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - AGTEE polarty selection"]
    #[inline(always)]
    #[must_use]
    pub fn eeps(&mut self) -> EEPS_W<2> {
        EEPS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AGT Event Pin Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [agtisr](index.html) module"]
pub struct AGTISR_SPEC;
impl crate::RegisterSpec for AGTISR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [agtisr::R](R) reader structure"]
impl crate::Readable for AGTISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [agtisr::W](W) writer structure"]
impl crate::Writable for AGTISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AGTISR to value 0"]
impl crate::Resettable for AGTISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
