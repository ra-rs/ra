#[doc = "Register `ECC1STSEN` reader"]
pub struct R(crate::R<ECC1STSEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECC1STSEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECC1STSEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECC1STSEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECC1STSEN` writer"]
pub struct W(crate::W<ECC1STSEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECC1STSEN_SPEC>;
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
impl From<crate::W<ECC1STSEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECC1STSEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `E1STSEN` reader - ECC 1-Bit Error Information Update Enable"]
pub type E1STSEN_R = crate::BitReader<E1STSEN_A>;
#[doc = "ECC 1-Bit Error Information Update Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E1STSEN_A {
    #[doc = "0: Disables updating of the 1-bit ECC error information."]
    _0 = 0,
    #[doc = "1: Enables updating of the 1-bit ECC error information."]
    _1 = 1,
}
impl From<E1STSEN_A> for bool {
    #[inline(always)]
    fn from(variant: E1STSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl E1STSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E1STSEN_A {
        match self.bits {
            false => E1STSEN_A::_0,
            true => E1STSEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == E1STSEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == E1STSEN_A::_1
    }
}
#[doc = "Field `E1STSEN` writer - ECC 1-Bit Error Information Update Enable"]
pub type E1STSEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, ECC1STSEN_SPEC, E1STSEN_A, O>;
impl<'a, const O: u8> E1STSEN_W<'a, O> {
    #[doc = "Disables updating of the 1-bit ECC error information."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(E1STSEN_A::_0)
    }
    #[doc = "Enables updating of the 1-bit ECC error information."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(E1STSEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - ECC 1-Bit Error Information Update Enable"]
    #[inline(always)]
    pub fn e1stsen(&self) -> E1STSEN_R {
        E1STSEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC 1-Bit Error Information Update Enable"]
    #[inline(always)]
    #[must_use]
    pub fn e1stsen(&mut self) -> E1STSEN_W<0> {
        E1STSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECCRAM 1-Bit Error Information Update Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc1stsen](index.html) module"]
pub struct ECC1STSEN_SPEC;
impl crate::RegisterSpec for ECC1STSEN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ecc1stsen::R](R) reader structure"]
impl crate::Readable for ECC1STSEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecc1stsen::W](W) writer structure"]
impl crate::Writable for ECC1STSEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECC1STSEN to value 0"]
impl crate::Resettable for ECC1STSEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
