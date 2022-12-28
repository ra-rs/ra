#[doc = "Register `LPSTS` reader"]
pub struct R(crate::R<LPSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPSTS` writer"]
pub struct W(crate::W<LPSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPSTS_SPEC>;
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
impl From<crate::W<LPSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUSPENDM` reader - UTMI SuspendM Control"]
pub type SUSPENDM_R = crate::BitReader<SUSPENDM_A>;
#[doc = "UTMI SuspendM Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSPENDM_A {
    #[doc = "0: UTMI suspension mode"]
    _0 = 0,
    #[doc = "1: UTMI normal mode"]
    _1 = 1,
}
impl From<SUSPENDM_A> for bool {
    #[inline(always)]
    fn from(variant: SUSPENDM_A) -> Self {
        variant as u8 != 0
    }
}
impl SUSPENDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUSPENDM_A {
        match self.bits {
            false => SUSPENDM_A::_0,
            true => SUSPENDM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SUSPENDM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SUSPENDM_A::_1
    }
}
#[doc = "Field `SUSPENDM` writer - UTMI SuspendM Control"]
pub type SUSPENDM_W<'a, const O: u8> = crate::BitWriter<'a, u16, LPSTS_SPEC, SUSPENDM_A, O>;
impl<'a, const O: u8> SUSPENDM_W<'a, O> {
    #[doc = "UTMI suspension mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SUSPENDM_A::_0)
    }
    #[doc = "UTMI normal mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SUSPENDM_A::_1)
    }
}
impl R {
    #[doc = "Bit 14 - UTMI SuspendM Control"]
    #[inline(always)]
    pub fn suspendm(&self) -> SUSPENDM_R {
        SUSPENDM_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - UTMI SuspendM Control"]
    #[inline(always)]
    #[must_use]
    pub fn suspendm(&mut self) -> SUSPENDM_W<14> {
        SUSPENDM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Power Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpsts](index.html) module"]
pub struct LPSTS_SPEC;
impl crate::RegisterSpec for LPSTS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [lpsts::R](R) reader structure"]
impl crate::Readable for LPSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpsts::W](W) writer structure"]
impl crate::Writable for LPSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPSTS to value 0"]
impl crate::Resettable for LPSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
