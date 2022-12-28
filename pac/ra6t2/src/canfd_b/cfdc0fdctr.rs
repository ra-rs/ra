#[doc = "Register `CFDC0FDCTR` reader"]
pub struct R(crate::R<CFDC0FDCTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDC0FDCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDC0FDCTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDC0FDCTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDC0FDCTR` writer"]
pub struct W(crate::W<CFDC0FDCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDC0FDCTR_SPEC>;
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
impl From<crate::W<CFDC0FDCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDC0FDCTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EOCCLR` reader - Error Occurrence Counter Clear"]
pub type EOCCLR_R = crate::BitReader<EOCCLR_A>;
#[doc = "Error Occurrence Counter Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCCLR_A {
    #[doc = "0: No error occurrence counter clear"]
    _0 = 0,
    #[doc = "1: Clear error occurrence counter"]
    _1 = 1,
}
impl From<EOCCLR_A> for bool {
    #[inline(always)]
    fn from(variant: EOCCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl EOCCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOCCLR_A {
        match self.bits {
            false => EOCCLR_A::_0,
            true => EOCCLR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOCCLR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOCCLR_A::_1
    }
}
#[doc = "Field `EOCCLR` writer - Error Occurrence Counter Clear"]
pub type EOCCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0FDCTR_SPEC, EOCCLR_A, O>;
impl<'a, const O: u8> EOCCLR_W<'a, O> {
    #[doc = "No error occurrence counter clear"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOCCLR_A::_0)
    }
    #[doc = "Clear error occurrence counter"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOCCLR_A::_1)
    }
}
#[doc = "Field `SOCCLR` reader - Successful Occurrence Counter Clear"]
pub type SOCCLR_R = crate::BitReader<SOCCLR_A>;
#[doc = "Successful Occurrence Counter Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOCCLR_A {
    #[doc = "0: No successful occurrence counter clear"]
    _0 = 0,
    #[doc = "1: Clear successful occurrence counter"]
    _1 = 1,
}
impl From<SOCCLR_A> for bool {
    #[inline(always)]
    fn from(variant: SOCCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl SOCCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOCCLR_A {
        match self.bits {
            false => SOCCLR_A::_0,
            true => SOCCLR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOCCLR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOCCLR_A::_1
    }
}
#[doc = "Field `SOCCLR` writer - Successful Occurrence Counter Clear"]
pub type SOCCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0FDCTR_SPEC, SOCCLR_A, O>;
impl<'a, const O: u8> SOCCLR_W<'a, O> {
    #[doc = "No successful occurrence counter clear"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOCCLR_A::_0)
    }
    #[doc = "Clear successful occurrence counter"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOCCLR_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Error Occurrence Counter Clear"]
    #[inline(always)]
    pub fn eocclr(&self) -> EOCCLR_R {
        EOCCLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Successful Occurrence Counter Clear"]
    #[inline(always)]
    pub fn socclr(&self) -> SOCCLR_R {
        SOCCLR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error Occurrence Counter Clear"]
    #[inline(always)]
    #[must_use]
    pub fn eocclr(&mut self) -> EOCCLR_W<0> {
        EOCCLR_W::new(self)
    }
    #[doc = "Bit 1 - Successful Occurrence Counter Clear"]
    #[inline(always)]
    #[must_use]
    pub fn socclr(&mut self) -> SOCCLR_W<1> {
        SOCCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 0 CANFD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdc0fdctr](index.html) module"]
pub struct CFDC0FDCTR_SPEC;
impl crate::RegisterSpec for CFDC0FDCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdc0fdctr::R](R) reader structure"]
impl crate::Readable for CFDC0FDCTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdc0fdctr::W](W) writer structure"]
impl crate::Writable for CFDC0FDCTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDC0FDCTR to value 0"]
impl crate::Resettable for CFDC0FDCTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
