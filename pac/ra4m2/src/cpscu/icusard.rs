#[doc = "Register `ICUSARD` reader"]
pub struct R(crate::R<ICUSARD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICUSARD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICUSARD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICUSARD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICUSARD` writer"]
pub struct W(crate::W<ICUSARD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICUSARD_SPEC>;
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
impl From<crate::W<ICUSARD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICUSARD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SASELSR0` reader - Security attributes of registers for SELSR0"]
pub type SASELSR0_R = crate::BitReader<SASELSR0_A>;
#[doc = "Security attributes of registers for SELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SASELSR0_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SASELSR0_A> for bool {
    #[inline(always)]
    fn from(variant: SASELSR0_A) -> Self {
        variant as u8 != 0
    }
}
impl SASELSR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SASELSR0_A {
        match self.bits {
            false => SASELSR0_A::_0,
            true => SASELSR0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SASELSR0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SASELSR0_A::_1
    }
}
#[doc = "Field `SASELSR0` writer - Security attributes of registers for SELSR0"]
pub type SASELSR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARD_SPEC, SASELSR0_A, O>;
impl<'a, const O: u8> SASELSR0_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SASELSR0_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SASELSR0_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Security attributes of registers for SELSR0"]
    #[inline(always)]
    pub fn saselsr0(&self) -> SASELSR0_R {
        SASELSR0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security attributes of registers for SELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saselsr0(&mut self) -> SASELSR0_W<0> {
        SASELSR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Controller Unit Security Attribution Register D\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icusard](index.html) module"]
pub struct ICUSARD_SPEC;
impl crate::RegisterSpec for ICUSARD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icusard::R](R) reader structure"]
impl crate::Readable for ICUSARD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icusard::W](W) writer structure"]
impl crate::Writable for ICUSARD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICUSARD to value 0xffff_ffff"]
impl crate::Resettable for ICUSARD_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
