#[doc = "Register `SDRFEN` reader"]
pub struct R(crate::R<SDRFEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDRFEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDRFEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDRFEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDRFEN` writer"]
pub struct W(crate::W<SDRFEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDRFEN_SPEC>;
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
impl From<crate::W<SDRFEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDRFEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFEN` reader - Auto-Refresh Operation Enable"]
pub type RFEN_R = crate::BitReader<RFEN_A>;
#[doc = "Auto-Refresh Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFEN_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<RFEN_A> for bool {
    #[inline(always)]
    fn from(variant: RFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFEN_A {
        match self.bits {
            false => RFEN_A::_0,
            true => RFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFEN_A::_1
    }
}
#[doc = "Field `RFEN` writer - Auto-Refresh Operation Enable"]
pub type RFEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, SDRFEN_SPEC, RFEN_A, O>;
impl<'a, const O: u8> RFEN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFEN_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Auto-Refresh Operation Enable"]
    #[inline(always)]
    pub fn rfen(&self) -> RFEN_R {
        RFEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Auto-Refresh Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfen(&mut self) -> RFEN_W<0> {
        RFEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAM Auto-Refresh Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdrfen](index.html) module"]
pub struct SDRFEN_SPEC;
impl crate::RegisterSpec for SDRFEN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sdrfen::R](R) reader structure"]
impl crate::Readable for SDRFEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdrfen::W](W) writer structure"]
impl crate::Writable for SDRFEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDRFEN to value 0"]
impl crate::Resettable for SDRFEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
