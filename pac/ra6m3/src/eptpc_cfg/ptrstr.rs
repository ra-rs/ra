#[doc = "Register `PTRSTR` reader"]
pub struct R(crate::R<PTRSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTRSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTRSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTRSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTRSTR` writer"]
pub struct W(crate::W<PTRSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTRSTR_SPEC>;
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
impl From<crate::W<PTRSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTRSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET` reader - EPTPC Software Reset"]
pub type RESET_R = crate::BitReader<RESET_A>;
#[doc = "EPTPC Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_A {
    #[doc = "0: Do not reset the EPTPC"]
    _0 = 0,
    #[doc = "1: Reset the EPTPC. Do not access the EPTPC-related registers other than this register while a software reset is being issued."]
    _1 = 1,
}
impl From<RESET_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_A) -> Self {
        variant as u8 != 0
    }
}
impl RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_A {
        match self.bits {
            false => RESET_A::_0,
            true => RESET_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RESET_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RESET_A::_1
    }
}
#[doc = "Field `RESET` writer - EPTPC Software Reset"]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTRSTR_SPEC, RESET_A, O>;
impl<'a, const O: u8> RESET_W<'a, O> {
    #[doc = "Do not reset the EPTPC"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RESET_A::_0)
    }
    #[doc = "Reset the EPTPC. Do not access the EPTPC-related registers other than this register while a software reset is being issued."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RESET_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - EPTPC Software Reset"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EPTPC Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<0> {
        RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPTPC Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptrstr](index.html) module"]
pub struct PTRSTR_SPEC;
impl crate::RegisterSpec for PTRSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptrstr::R](R) reader structure"]
impl crate::Readable for PTRSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptrstr::W](W) writer structure"]
impl crate::Writable for PTRSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTRSTR to value 0"]
impl crate::Resettable for PTRSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
