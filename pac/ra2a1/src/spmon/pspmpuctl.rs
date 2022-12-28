#[doc = "Register `PSPMPUCTL` reader"]
pub struct R(crate::R<PSPMPUCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSPMPUCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSPMPUCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSPMPUCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSPMPUCTL` writer"]
pub struct W(crate::W<PSPMPUCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSPMPUCTL_SPEC>;
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
impl From<crate::W<PSPMPUCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSPMPUCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - SP_process monitor enable"]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
#[doc = "SP_process monitor enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    #[doc = "0: SP_process monitor is disabled."]
    _0 = 0,
    #[doc = "1: SP_process monitor is enabled."]
    _1 = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::_0,
            true => ENABLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENABLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENABLE_A::_1
    }
}
#[doc = "Field `ENABLE` writer - SP_process monitor enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u16, PSPMPUCTL_SPEC, ENABLE_A, O>;
impl<'a, const O: u8> ENABLE_W<'a, O> {
    #[doc = "SP_process monitor is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENABLE_A::_0)
    }
    #[doc = "SP_process monitor is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENABLE_A::_1)
    }
}
#[doc = "Field `ERROR` reader - SP_process monitor error flag"]
pub type ERROR_R = crate::BitReader<ERROR_A>;
#[doc = "SP_process monitor error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERROR_A {
    #[doc = "0: SP_process has not overflowed or underflowed."]
    _0 = 0,
    #[doc = "1: SP_process has overflowed or underflowed."]
    _1 = 1,
}
impl From<ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        variant as u8 != 0
    }
}
impl ERROR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROR_A {
        match self.bits {
            false => ERROR_A::_0,
            true => ERROR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERROR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERROR_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - SP_process monitor enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - SP_process monitor error flag"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SP_process monitor enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stack Pointer Monitor Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pspmpuctl](index.html) module"]
pub struct PSPMPUCTL_SPEC;
impl crate::RegisterSpec for PSPMPUCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pspmpuctl::R](R) reader structure"]
impl crate::Readable for PSPMPUCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pspmpuctl::W](W) writer structure"]
impl crate::Writable for PSPMPUCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSPMPUCTL to value 0"]
impl crate::Resettable for PSPMPUCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
