#[doc = "Register `BUSSARA` reader"]
pub struct R(crate::R<BUSSARA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSSARA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSSARA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSSARA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUSSARA` writer"]
pub struct W(crate::W<BUSSARA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUSSARA_SPEC>;
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
impl From<crate::W<BUSSARA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUSSARA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUSSA0` reader - BUS Security Attribution A0"]
pub type BUSSA0_R = crate::BitReader<BUSSA0_A>;
#[doc = "BUS Security Attribution A0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSSA0_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-Secure"]
    _1 = 1,
}
impl From<BUSSA0_A> for bool {
    #[inline(always)]
    fn from(variant: BUSSA0_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSSA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSSA0_A {
        match self.bits {
            false => BUSSA0_A::_0,
            true => BUSSA0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSSA0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSSA0_A::_1
    }
}
#[doc = "Field `BUSSA0` writer - BUS Security Attribution A0"]
pub type BUSSA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUSSARA_SPEC, BUSSA0_A, O>;
impl<'a, const O: u8> BUSSA0_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSSA0_A::_0)
    }
    #[doc = "Non-Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSSA0_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - BUS Security Attribution A0"]
    #[inline(always)]
    pub fn bussa0(&self) -> BUSSA0_R {
        BUSSA0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BUS Security Attribution A0"]
    #[inline(always)]
    #[must_use]
    pub fn bussa0(&mut self) -> BUSSA0_W<0> {
        BUSSA0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BUS Security Attribution Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bussara](index.html) module"]
pub struct BUSSARA_SPEC;
impl crate::RegisterSpec for BUSSARA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bussara::R](R) reader structure"]
impl crate::Readable for BUSSARA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bussara::W](W) writer structure"]
impl crate::Writable for BUSSARA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUSSARA to value 0xffff_ffff"]
impl crate::Resettable for BUSSARA_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
