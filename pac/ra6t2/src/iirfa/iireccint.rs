#[doc = "Register `IIRECCINT` reader"]
pub struct R(crate::R<IIRECCINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIRECCINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIRECCINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIRECCINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IIRECCINT` writer"]
pub struct W(crate::W<IIRECCINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IIRECCINT_SPEC>;
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
impl From<crate::W<IIRECCINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IIRECCINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ESEIE` reader - ECC 1-bit error interrupt enable bit"]
pub type ESEIE_R = crate::BitReader<ESEIE_A>;
#[doc = "ECC 1-bit error interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESEIE_A {
    #[doc = "0: The generation of ECC 1-bit error interrupt requests is disabled."]
    _0 = 0,
    #[doc = "1: The generation of ECC 1-bit error interrupt requests is enabled."]
    _1 = 1,
}
impl From<ESEIE_A> for bool {
    #[inline(always)]
    fn from(variant: ESEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ESEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESEIE_A {
        match self.bits {
            false => ESEIE_A::_0,
            true => ESEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ESEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ESEIE_A::_1
    }
}
#[doc = "Field `ESEIE` writer - ECC 1-bit error interrupt enable bit"]
pub type ESEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IIRECCINT_SPEC, ESEIE_A, O>;
impl<'a, const O: u8> ESEIE_W<'a, O> {
    #[doc = "The generation of ECC 1-bit error interrupt requests is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ESEIE_A::_0)
    }
    #[doc = "The generation of ECC 1-bit error interrupt requests is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ESEIE_A::_1)
    }
}
#[doc = "Field `EDEIE` reader - ECC 2-bit error interrupt enable bit"]
pub type EDEIE_R = crate::BitReader<EDEIE_A>;
#[doc = "ECC 2-bit error interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDEIE_A {
    #[doc = "0: The generation of ECC 2-bit error interrupt requests is disabled."]
    _0 = 0,
    #[doc = "1: The generation of ECC 2-bit error interrupt requests is enabled."]
    _1 = 1,
}
impl From<EDEIE_A> for bool {
    #[inline(always)]
    fn from(variant: EDEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EDEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDEIE_A {
        match self.bits {
            false => EDEIE_A::_0,
            true => EDEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EDEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EDEIE_A::_1
    }
}
#[doc = "Field `EDEIE` writer - ECC 2-bit error interrupt enable bit"]
pub type EDEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IIRECCINT_SPEC, EDEIE_A, O>;
impl<'a, const O: u8> EDEIE_W<'a, O> {
    #[doc = "The generation of ECC 2-bit error interrupt requests is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDEIE_A::_0)
    }
    #[doc = "The generation of ECC 2-bit error interrupt requests is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDEIE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - ECC 1-bit error interrupt enable bit"]
    #[inline(always)]
    pub fn eseie(&self) -> ESEIE_R {
        ESEIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECC 2-bit error interrupt enable bit"]
    #[inline(always)]
    pub fn edeie(&self) -> EDEIE_R {
        EDEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC 1-bit error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn eseie(&mut self) -> ESEIE_W<0> {
        ESEIE_W::new(self)
    }
    #[doc = "Bit 1 - ECC 2-bit error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn edeie(&mut self) -> EDEIE_W<1> {
        EDEIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECC Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iireccint](index.html) module"]
pub struct IIRECCINT_SPEC;
impl crate::RegisterSpec for IIRECCINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iireccint::R](R) reader structure"]
impl crate::Readable for IIRECCINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iireccint::W](W) writer structure"]
impl crate::Writable for IIRECCINT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IIRECCINT to value 0"]
impl crate::Resettable for IIRECCINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
