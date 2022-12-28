#[doc = "Register `FCACHEIV` reader"]
pub struct R(crate::R<FCACHEIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCACHEIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCACHEIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCACHEIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCACHEIV` writer"]
pub struct W(crate::W<FCACHEIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCACHEIV_SPEC>;
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
impl From<crate::W<FCACHEIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCACHEIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FCACHEIV` reader - Flash Cache Invalidate"]
pub type FCACHEIV_R = crate::BitReader<FCACHEIV_A>;
#[doc = "Flash Cache Invalidate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCACHEIV_A {
    #[doc = "0: Read: Do not invalidate. Write: The setting is ignored."]
    _0 = 0,
    #[doc = "1: Invalidate FCACHE is invalidated."]
    _1 = 1,
}
impl From<FCACHEIV_A> for bool {
    #[inline(always)]
    fn from(variant: FCACHEIV_A) -> Self {
        variant as u8 != 0
    }
}
impl FCACHEIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCACHEIV_A {
        match self.bits {
            false => FCACHEIV_A::_0,
            true => FCACHEIV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FCACHEIV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FCACHEIV_A::_1
    }
}
#[doc = "Field `FCACHEIV` writer - Flash Cache Invalidate"]
pub type FCACHEIV_W<'a, const O: u8> = crate::BitWriter<'a, u16, FCACHEIV_SPEC, FCACHEIV_A, O>;
impl<'a, const O: u8> FCACHEIV_W<'a, O> {
    #[doc = "Read: Do not invalidate. Write: The setting is ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FCACHEIV_A::_0)
    }
    #[doc = "Invalidate FCACHE is invalidated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FCACHEIV_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Flash Cache Invalidate"]
    #[inline(always)]
    pub fn fcacheiv(&self) -> FCACHEIV_R {
        FCACHEIV_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Cache Invalidate"]
    #[inline(always)]
    #[must_use]
    pub fn fcacheiv(&mut self) -> FCACHEIV_W<0> {
        FCACHEIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Cache Invalidate Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcacheiv](index.html) module"]
pub struct FCACHEIV_SPEC;
impl crate::RegisterSpec for FCACHEIV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fcacheiv::R](R) reader structure"]
impl crate::Readable for FCACHEIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcacheiv::W](W) writer structure"]
impl crate::Writable for FCACHEIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCACHEIV to value 0"]
impl crate::Resettable for FCACHEIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
