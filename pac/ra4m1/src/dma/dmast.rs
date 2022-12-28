#[doc = "Register `DMAST` reader"]
pub struct R(crate::R<DMAST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAST` writer"]
pub struct W(crate::W<DMAST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAST_SPEC>;
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
impl From<crate::W<DMAST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMST` reader - DMAC Operation Enable"]
pub type DMST_R = crate::BitReader<DMST_A>;
#[doc = "DMAC Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMST_A {
    #[doc = "0: Disabled."]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<DMST_A> for bool {
    #[inline(always)]
    fn from(variant: DMST_A) -> Self {
        variant as u8 != 0
    }
}
impl DMST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMST_A {
        match self.bits {
            false => DMST_A::_0,
            true => DMST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMST_A::_1
    }
}
#[doc = "Field `DMST` writer - DMAC Operation Enable"]
pub type DMST_W<'a, const O: u8> = crate::BitWriter<'a, u8, DMAST_SPEC, DMST_A, O>;
impl<'a, const O: u8> DMST_W<'a, O> {
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMST_A::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMST_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - DMAC Operation Enable"]
    #[inline(always)]
    pub fn dmst(&self) -> DMST_R {
        DMST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMAC Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmst(&mut self) -> DMST_W<0> {
        DMST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Module Activation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmast](index.html) module"]
pub struct DMAST_SPEC;
impl crate::RegisterSpec for DMAST_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dmast::R](R) reader structure"]
impl crate::Readable for DMAST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmast::W](W) writer structure"]
impl crate::Writable for DMAST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAST to value 0"]
impl crate::Resettable for DMAST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
