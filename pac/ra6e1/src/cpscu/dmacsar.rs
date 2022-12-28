#[doc = "Register `DMACSAR` reader"]
pub struct R(crate::R<DMACSAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACSAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACSAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACSAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACSAR` writer"]
pub struct W(crate::W<DMACSAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACSAR_SPEC>;
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
impl From<crate::W<DMACSAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACSAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMASTSA` reader - DMAST Security Attribution"]
pub type DMASTSA_R = crate::BitReader<DMASTSA_A>;
#[doc = "DMAST Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMASTSA_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<DMASTSA_A> for bool {
    #[inline(always)]
    fn from(variant: DMASTSA_A) -> Self {
        variant as u8 != 0
    }
}
impl DMASTSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMASTSA_A {
        match self.bits {
            false => DMASTSA_A::_0,
            true => DMASTSA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMASTSA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMASTSA_A::_1
    }
}
#[doc = "Field `DMASTSA` writer - DMAST Security Attribution"]
pub type DMASTSA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACSAR_SPEC, DMASTSA_A, O>;
impl<'a, const O: u8> DMASTSA_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMASTSA_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMASTSA_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - DMAST Security Attribution"]
    #[inline(always)]
    pub fn dmastsa(&self) -> DMASTSA_R {
        DMASTSA_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMAST Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn dmastsa(&mut self) -> DMASTSA_W<0> {
        DMASTSA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Controller Security Attribution Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacsar](index.html) module"]
pub struct DMACSAR_SPEC;
impl crate::RegisterSpec for DMACSAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmacsar::R](R) reader structure"]
impl crate::Readable for DMACSAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmacsar::W](W) writer structure"]
impl crate::Writable for DMACSAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACSAR to value 0xffff_ffff"]
impl crate::Resettable for DMACSAR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
