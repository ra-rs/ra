#[doc = "Register `CCACTL` reader"]
pub struct R(crate::R<CCACTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCACTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCACTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCACTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCACTL` writer"]
pub struct W(crate::W<CCACTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCACTL_SPEC>;
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
impl From<crate::W<CCACTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCACTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENC` reader - C-Cache Enable"]
pub type ENC_R = crate::BitReader<ENC_A>;
#[doc = "C-Cache Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENC_A {
    #[doc = "0: Disable C-cache"]
    _0 = 0,
    #[doc = "1: Enable C-cache"]
    _1 = 1,
}
impl From<ENC_A> for bool {
    #[inline(always)]
    fn from(variant: ENC_A) -> Self {
        variant as u8 != 0
    }
}
impl ENC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENC_A {
        match self.bits {
            false => ENC_A::_0,
            true => ENC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENC_A::_1
    }
}
#[doc = "Field `ENC` writer - C-Cache Enable"]
pub type ENC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCACTL_SPEC, ENC_A, O>;
impl<'a, const O: u8> ENC_W<'a, O> {
    #[doc = "Disable C-cache"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENC_A::_0)
    }
    #[doc = "Enable C-cache"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENC_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - C-Cache Enable"]
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - C-Cache Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enc(&mut self) -> ENC_W<0> {
        ENC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "C-Cache Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccactl](index.html) module"]
pub struct CCACTL_SPEC;
impl crate::RegisterSpec for CCACTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccactl::R](R) reader structure"]
impl crate::Readable for CCACTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccactl::W](W) writer structure"]
impl crate::Writable for CCACTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCACTL to value 0"]
impl crate::Resettable for CCACTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
