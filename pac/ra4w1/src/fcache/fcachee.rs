#[doc = "Register `FCACHEE` reader"]
pub struct R(crate::R<FCACHEE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCACHEE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCACHEE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCACHEE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCACHEE` writer"]
pub struct W(crate::W<FCACHEE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCACHEE_SPEC>;
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
impl From<crate::W<FCACHEE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCACHEE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FCACHEEN` reader - FCACHE Enable"]
pub type FCACHEEN_R = crate::BitReader<FCACHEEN_A>;
#[doc = "FCACHE Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCACHEEN_A {
    #[doc = "0: FCACHE is disabled"]
    _0 = 0,
    #[doc = "1: FCACHE is enabled"]
    _1 = 1,
}
impl From<FCACHEEN_A> for bool {
    #[inline(always)]
    fn from(variant: FCACHEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FCACHEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCACHEEN_A {
        match self.bits {
            false => FCACHEEN_A::_0,
            true => FCACHEEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FCACHEEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FCACHEEN_A::_1
    }
}
#[doc = "Field `FCACHEEN` writer - FCACHE Enable"]
pub type FCACHEEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, FCACHEE_SPEC, FCACHEEN_A, O>;
impl<'a, const O: u8> FCACHEEN_W<'a, O> {
    #[doc = "FCACHE is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FCACHEEN_A::_0)
    }
    #[doc = "FCACHE is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FCACHEEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - FCACHE Enable"]
    #[inline(always)]
    pub fn fcacheen(&self) -> FCACHEEN_R {
        FCACHEEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FCACHE Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fcacheen(&mut self) -> FCACHEEN_W<0> {
        FCACHEEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Cache Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcachee](index.html) module"]
pub struct FCACHEE_SPEC;
impl crate::RegisterSpec for FCACHEE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fcachee::R](R) reader structure"]
impl crate::Readable for FCACHEE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcachee::W](W) writer structure"]
impl crate::Writable for FCACHEE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCACHEE to value 0"]
impl crate::Resettable for FCACHEE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
