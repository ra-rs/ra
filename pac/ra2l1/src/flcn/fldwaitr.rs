#[doc = "Register `FLDWAITR` reader"]
pub struct R(crate::R<FLDWAITR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLDWAITR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLDWAITR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLDWAITR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLDWAITR` writer"]
pub struct W(crate::W<FLDWAITR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLDWAITR_SPEC>;
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
impl From<crate::W<FLDWAITR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLDWAITR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLDWAIT1` reader - Memory Wait Cycle Select for Data Flash"]
pub type FLDWAIT1_R = crate::BitReader<FLDWAIT1_A>;
#[doc = "Memory Wait Cycle Select for Data Flash\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLDWAIT1_A {
    #[doc = "0: 1 wait access (Default)"]
    _0 = 0,
    #[doc = "1: 2 wait access"]
    _1 = 1,
}
impl From<FLDWAIT1_A> for bool {
    #[inline(always)]
    fn from(variant: FLDWAIT1_A) -> Self {
        variant as u8 != 0
    }
}
impl FLDWAIT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLDWAIT1_A {
        match self.bits {
            false => FLDWAIT1_A::_0,
            true => FLDWAIT1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLDWAIT1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLDWAIT1_A::_1
    }
}
#[doc = "Field `FLDWAIT1` writer - Memory Wait Cycle Select for Data Flash"]
pub type FLDWAIT1_W<'a, const O: u8> = crate::BitWriter<'a, u8, FLDWAITR_SPEC, FLDWAIT1_A, O>;
impl<'a, const O: u8> FLDWAIT1_W<'a, O> {
    #[doc = "1 wait access (Default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLDWAIT1_A::_0)
    }
    #[doc = "2 wait access"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLDWAIT1_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Memory Wait Cycle Select for Data Flash"]
    #[inline(always)]
    pub fn fldwait1(&self) -> FLDWAIT1_R {
        FLDWAIT1_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Memory Wait Cycle Select for Data Flash"]
    #[inline(always)]
    #[must_use]
    pub fn fldwait1(&mut self) -> FLDWAIT1_W<0> {
        FLDWAIT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory Wait Cycle Control Register for Data Flash\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fldwaitr](index.html) module"]
pub struct FLDWAITR_SPEC;
impl crate::RegisterSpec for FLDWAITR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fldwaitr::R](R) reader structure"]
impl crate::Readable for FLDWAITR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fldwaitr::W](W) writer structure"]
impl crate::Writable for FLDWAITR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLDWAITR to value 0"]
impl crate::Resettable for FLDWAITR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
