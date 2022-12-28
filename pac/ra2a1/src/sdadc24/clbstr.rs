#[doc = "Register `CLBSTR` reader"]
pub struct R(crate::R<CLBSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLBSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLBSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLBSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLBSTR` writer"]
pub struct W(crate::W<CLBSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLBSTR_SPEC>;
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
impl From<crate::W<CLBSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLBSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLBST` reader - Calibration start control"]
pub type CLBST_R = crate::BitReader<CLBST_A>;
#[doc = "Calibration start control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLBST_A {
    #[doc = "0: Disable writing"]
    _0 = 0,
    #[doc = "1: Start calibration"]
    _1 = 1,
}
impl From<CLBST_A> for bool {
    #[inline(always)]
    fn from(variant: CLBST_A) -> Self {
        variant as u8 != 0
    }
}
impl CLBST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLBST_A {
        match self.bits {
            false => CLBST_A::_0,
            true => CLBST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLBST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLBST_A::_1
    }
}
#[doc = "Field `CLBST` writer - Calibration start control"]
pub type CLBST_W<'a, const O: u8> = crate::BitWriter<'a, u8, CLBSTR_SPEC, CLBST_A, O>;
impl<'a, const O: u8> CLBST_W<'a, O> {
    #[doc = "Disable writing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLBST_A::_0)
    }
    #[doc = "Start calibration"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLBST_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Calibration start control"]
    #[inline(always)]
    pub fn clbst(&self) -> CLBST_R {
        CLBST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Calibration start control"]
    #[inline(always)]
    #[must_use]
    pub fn clbst(&mut self) -> CLBST_W<0> {
        CLBST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration Start Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clbstr](index.html) module"]
pub struct CLBSTR_SPEC;
impl crate::RegisterSpec for CLBSTR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [clbstr::R](R) reader structure"]
impl crate::Readable for CLBSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clbstr::W](W) writer structure"]
impl crate::Writable for CLBSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLBSTR to value 0"]
impl crate::Resettable for CLBSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
