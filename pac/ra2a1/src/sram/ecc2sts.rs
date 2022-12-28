#[doc = "Register `ECC2STS` reader"]
pub struct R(crate::R<ECC2STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECC2STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECC2STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECC2STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECC2STS` writer"]
pub struct W(crate::W<ECC2STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECC2STS_SPEC>;
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
impl From<crate::W<ECC2STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECC2STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECC2ERR` reader - ECC 2-Bit Error Status\n\nThe field is **modified** in some way after a read operation."]
pub type ECC2ERR_R = crate::BitReader<ECC2ERR_A>;
#[doc = "ECC 2-Bit Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECC2ERR_A {
    #[doc = "0: A 2-bit ECC error has not occurred."]
    _0 = 0,
    #[doc = "1: A 2-bit ECC error has occurred."]
    _1 = 1,
}
impl From<ECC2ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ECC2ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl ECC2ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECC2ERR_A {
        match self.bits {
            false => ECC2ERR_A::_0,
            true => ECC2ERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECC2ERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECC2ERR_A::_1
    }
}
#[doc = "Field `ECC2ERR` writer - ECC 2-Bit Error Status"]
pub type ECC2ERR_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, ECC2STS_SPEC, ECC2ERR_A, O>;
impl<'a, const O: u8> ECC2ERR_W<'a, O> {
    #[doc = "A 2-bit ECC error has not occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ECC2ERR_A::_0)
    }
    #[doc = "A 2-bit ECC error has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ECC2ERR_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - ECC 2-Bit Error Status"]
    #[inline(always)]
    pub fn ecc2err(&self) -> ECC2ERR_R {
        ECC2ERR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC 2-Bit Error Status"]
    #[inline(always)]
    #[must_use]
    pub fn ecc2err(&mut self) -> ECC2ERR_W<0> {
        ECC2ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECC 2-Bit Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc2sts](index.html) module"]
pub struct ECC2STS_SPEC;
impl crate::RegisterSpec for ECC2STS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ecc2sts::R](R) reader structure"]
impl crate::Readable for ECC2STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecc2sts::W](W) writer structure"]
impl crate::Writable for ECC2STS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECC2STS to value 0"]
impl crate::Resettable for ECC2STS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
