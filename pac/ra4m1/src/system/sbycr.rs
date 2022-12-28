#[doc = "Register `SBYCR` reader"]
pub struct R(crate::R<SBYCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SBYCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SBYCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SBYCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SBYCR` writer"]
pub struct W(crate::W<SBYCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SBYCR_SPEC>;
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
impl From<crate::W<SBYCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SBYCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSBY` reader - Software Standby"]
pub type SSBY_R = crate::BitReader<SSBY_A>;
#[doc = "Software Standby\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSBY_A {
    #[doc = "0: Sleep mode"]
    _0 = 0,
    #[doc = "1: Software Standby mode"]
    _1 = 1,
}
impl From<SSBY_A> for bool {
    #[inline(always)]
    fn from(variant: SSBY_A) -> Self {
        variant as u8 != 0
    }
}
impl SSBY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSBY_A {
        match self.bits {
            false => SSBY_A::_0,
            true => SSBY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSBY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSBY_A::_1
    }
}
#[doc = "Field `SSBY` writer - Software Standby"]
pub type SSBY_W<'a, const O: u8> = crate::BitWriter<'a, u16, SBYCR_SPEC, SSBY_A, O>;
impl<'a, const O: u8> SSBY_W<'a, O> {
    #[doc = "Sleep mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSBY_A::_0)
    }
    #[doc = "Software Standby mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSBY_A::_1)
    }
}
impl R {
    #[doc = "Bit 15 - Software Standby"]
    #[inline(always)]
    pub fn ssby(&self) -> SSBY_R {
        SSBY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Software Standby"]
    #[inline(always)]
    #[must_use]
    pub fn ssby(&mut self) -> SSBY_W<15> {
        SSBY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Standby Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sbycr](index.html) module"]
pub struct SBYCR_SPEC;
impl crate::RegisterSpec for SBYCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sbycr::R](R) reader structure"]
impl crate::Readable for SBYCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sbycr::W](W) writer structure"]
impl crate::Writable for SBYCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SBYCR to value 0x4000"]
impl crate::Resettable for SBYCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000;
}
