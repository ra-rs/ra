#[doc = "Register `MSPMPUCTL` reader"]
pub struct R(crate::R<MSPMPUCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSPMPUCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSPMPUCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSPMPUCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSPMPUCTL` writer"]
pub struct W(crate::W<MSPMPUCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSPMPUCTL_SPEC>;
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
impl From<crate::W<MSPMPUCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSPMPUCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Stack Pointer Monitor Enable"]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
#[doc = "Stack Pointer Monitor Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    #[doc = "0: Stack pointer monitor is disabled"]
    _0 = 0,
    #[doc = "1: Stack pointer monitor is enabled"]
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
#[doc = "Field `ENABLE` writer - Stack Pointer Monitor Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u16, MSPMPUCTL_SPEC, ENABLE_A, O>;
impl<'a, const O: u8> ENABLE_W<'a, O> {
    #[doc = "Stack pointer monitor is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENABLE_A::_0)
    }
    #[doc = "Stack pointer monitor is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENABLE_A::_1)
    }
}
#[doc = "Field `ERROR` reader - Stack Pointer Monitor Error Flag"]
pub type ERROR_R = crate::BitReader<ERROR_A>;
#[doc = "Stack Pointer Monitor Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERROR_A {
    #[doc = "0: Stack pointer has not overflowed or underflowed"]
    _0 = 0,
    #[doc = "1: Stack pointer has overflowed or underflowed"]
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
#[doc = "Field `ERROR` writer - Stack Pointer Monitor Error Flag"]
pub type ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u16, MSPMPUCTL_SPEC, ERROR_A, O>;
impl<'a, const O: u8> ERROR_W<'a, O> {
    #[doc = "Stack pointer has not overflowed or underflowed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERROR_A::_0)
    }
    #[doc = "Stack pointer has overflowed or underflowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERROR_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Stack Pointer Monitor Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Stack Pointer Monitor Error Flag"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stack Pointer Monitor Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 8 - Stack Pointer Monitor Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<8> {
        ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stack Pointer Monitor Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mspmpuctl](index.html) module"]
pub struct MSPMPUCTL_SPEC;
impl crate::RegisterSpec for MSPMPUCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mspmpuctl::R](R) reader structure"]
impl crate::Readable for MSPMPUCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mspmpuctl::W](W) writer structure"]
impl crate::Writable for MSPMPUCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSPMPUCTL to value 0"]
impl crate::Resettable for MSPMPUCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
