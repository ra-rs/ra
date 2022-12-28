#[doc = "Register `DPSIER1` reader"]
pub struct R(crate::R<DPSIER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPSIER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPSIER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPSIER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPSIER1` writer"]
pub struct W(crate::W<DPSIER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPSIER1_SPEC>;
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
impl From<crate::W<DPSIER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPSIER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIRQ8E` reader - IRQ8-DS Pin Enable"]
pub type DIRQ8E_R = crate::BitReader<DIRQ8E_A>;
#[doc = "IRQ8-DS Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ8E_A {
    #[doc = "0: Cancelling Deep Software Standby mode is disabled"]
    _0 = 0,
    #[doc = "1: Cancelling Deep Software Standby mode is enabled"]
    _1 = 1,
}
impl From<DIRQ8E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ8E_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ8E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ8E_A {
        match self.bits {
            false => DIRQ8E_A::_0,
            true => DIRQ8E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ8E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ8E_A::_1
    }
}
#[doc = "Field `DIRQ8E` writer - IRQ8-DS Pin Enable"]
pub type DIRQ8E_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIER1_SPEC, DIRQ8E_A, O>;
impl<'a, const O: u8> DIRQ8E_W<'a, O> {
    #[doc = "Cancelling Deep Software Standby mode is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ8E_A::_0)
    }
    #[doc = "Cancelling Deep Software Standby mode is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ8E_A::_1)
    }
}
#[doc = "Field `DIRQ9E` reader - IRQ9-DS Pin Enable"]
pub type DIRQ9E_R = crate::BitReader<DIRQ9E_A>;
#[doc = "IRQ9-DS Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ9E_A {
    #[doc = "0: Cancelling Deep Software Standby mode is disabled"]
    _0 = 0,
    #[doc = "1: Cancelling Deep Software Standby mode is enabled"]
    _1 = 1,
}
impl From<DIRQ9E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ9E_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ9E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ9E_A {
        match self.bits {
            false => DIRQ9E_A::_0,
            true => DIRQ9E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ9E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ9E_A::_1
    }
}
#[doc = "Field `DIRQ9E` writer - IRQ9-DS Pin Enable"]
pub type DIRQ9E_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIER1_SPEC, DIRQ9E_A, O>;
impl<'a, const O: u8> DIRQ9E_W<'a, O> {
    #[doc = "Cancelling Deep Software Standby mode is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ9E_A::_0)
    }
    #[doc = "Cancelling Deep Software Standby mode is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ9E_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - IRQ8-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq8e(&self) -> DIRQ8E_R {
        DIRQ8E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRQ9-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq9e(&self) -> DIRQ9E_R {
        DIRQ9E_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRQ8-DS Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dirq8e(&mut self) -> DIRQ8E_W<0> {
        DIRQ8E_W::new(self)
    }
    #[doc = "Bit 1 - IRQ9-DS Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dirq9e(&mut self) -> DIRQ9E_W<1> {
        DIRQ9E_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep Standby Interrupt Enable Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpsier1](index.html) module"]
pub struct DPSIER1_SPEC;
impl crate::RegisterSpec for DPSIER1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dpsier1::R](R) reader structure"]
impl crate::Readable for DPSIER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpsier1::W](W) writer structure"]
impl crate::Writable for DPSIER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPSIER1 to value 0"]
impl crate::Resettable for DPSIER1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
