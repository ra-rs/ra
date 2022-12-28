#[doc = "Register `ECC1STS` reader"]
pub struct R(crate::R<ECC1STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECC1STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECC1STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECC1STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECC1STS` writer"]
pub struct W(crate::W<ECC1STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECC1STS_SPEC>;
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
impl From<crate::W<ECC1STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECC1STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECC1ERR` reader - ECC 1-Bit Error Status\n\nThe field is **modified** in some way after a read operation."]
pub type ECC1ERR_R = crate::BitReader<ECC1ERR_A>;
#[doc = "ECC 1-Bit Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECC1ERR_A {
    #[doc = "0: No 1-bit ECC error occurred"]
    _0 = 0,
    #[doc = "1: 1-bit ECC error occurred"]
    _1 = 1,
}
impl From<ECC1ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ECC1ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl ECC1ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECC1ERR_A {
        match self.bits {
            false => ECC1ERR_A::_0,
            true => ECC1ERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECC1ERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECC1ERR_A::_1
    }
}
#[doc = "Field `ECC1ERR` writer - ECC 1-Bit Error Status"]
pub type ECC1ERR_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, ECC1STS_SPEC, ECC1ERR_A, O>;
impl<'a, const O: u8> ECC1ERR_W<'a, O> {
    #[doc = "No 1-bit ECC error occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ECC1ERR_A::_0)
    }
    #[doc = "1-bit ECC error occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ECC1ERR_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - ECC 1-Bit Error Status"]
    #[inline(always)]
    pub fn ecc1err(&self) -> ECC1ERR_R {
        ECC1ERR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC 1-Bit Error Status"]
    #[inline(always)]
    #[must_use]
    pub fn ecc1err(&mut self) -> ECC1ERR_W<0> {
        ECC1ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECCRAM 1-Bit Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc1sts](index.html) module"]
pub struct ECC1STS_SPEC;
impl crate::RegisterSpec for ECC1STS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ecc1sts::R](R) reader structure"]
impl crate::Readable for ECC1STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecc1sts::W](W) writer structure"]
impl crate::Writable for ECC1STS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECC1STS to value 0"]
impl crate::Resettable for ECC1STS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
