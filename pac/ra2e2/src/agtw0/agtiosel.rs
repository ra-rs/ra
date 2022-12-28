#[doc = "Register `AGTIOSEL` reader"]
pub struct R(crate::R<AGTIOSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AGTIOSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AGTIOSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AGTIOSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AGTIOSEL` writer"]
pub struct W(crate::W<AGTIOSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AGTIOSEL_SPEC>;
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
impl From<crate::W<AGTIOSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AGTIOSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIES` reader - AGTWIOn Pin Input Enable"]
pub type TIES_R = crate::BitReader<TIES_A>;
#[doc = "AGTWIOn Pin Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIES_A {
    #[doc = "0: External event input is disabled during Software Standby mode"]
    _0 = 0,
    #[doc = "1: External event input is enabled during Software Standby mode"]
    _1 = 1,
}
impl From<TIES_A> for bool {
    #[inline(always)]
    fn from(variant: TIES_A) -> Self {
        variant as u8 != 0
    }
}
impl TIES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIES_A {
        match self.bits {
            false => TIES_A::_0,
            true => TIES_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIES_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIES_A::_1
    }
}
#[doc = "Field `TIES` writer - AGTWIOn Pin Input Enable"]
pub type TIES_W<'a, const O: u8> = crate::BitWriter<'a, u8, AGTIOSEL_SPEC, TIES_A, O>;
impl<'a, const O: u8> TIES_W<'a, O> {
    #[doc = "External event input is disabled during Software Standby mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIES_A::_0)
    }
    #[doc = "External event input is enabled during Software Standby mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIES_A::_1)
    }
}
impl R {
    #[doc = "Bit 4 - AGTWIOn Pin Input Enable"]
    #[inline(always)]
    pub fn ties(&self) -> TIES_R {
        TIES_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - AGTWIOn Pin Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ties(&mut self) -> TIES_W<4> {
        TIES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AGT Pin Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [agtiosel](index.html) module"]
pub struct AGTIOSEL_SPEC;
impl crate::RegisterSpec for AGTIOSEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [agtiosel::R](R) reader structure"]
impl crate::Readable for AGTIOSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [agtiosel::W](W) writer structure"]
impl crate::Writable for AGTIOSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AGTIOSEL to value 0"]
impl crate::Resettable for AGTIOSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
