#[doc = "Register `SCACTL` reader"]
pub struct R(crate::R<SCACTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCACTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCACTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCACTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCACTL` writer"]
pub struct W(crate::W<SCACTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCACTL_SPEC>;
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
impl From<crate::W<SCACTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCACTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENS` reader - S-Cache Enable"]
pub type ENS_R = crate::BitReader<ENS_A>;
#[doc = "S-Cache Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENS_A {
    #[doc = "0: Disable S-cache"]
    _0 = 0,
    #[doc = "1: Enable S-cache"]
    _1 = 1,
}
impl From<ENS_A> for bool {
    #[inline(always)]
    fn from(variant: ENS_A) -> Self {
        variant as u8 != 0
    }
}
impl ENS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENS_A {
        match self.bits {
            false => ENS_A::_0,
            true => ENS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENS_A::_1
    }
}
#[doc = "Field `ENS` writer - S-Cache Enable"]
pub type ENS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCACTL_SPEC, ENS_A, O>;
impl<'a, const O: u8> ENS_W<'a, O> {
    #[doc = "Disable S-cache"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENS_A::_0)
    }
    #[doc = "Enable S-cache"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENS_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - S-Cache Enable"]
    #[inline(always)]
    pub fn ens(&self) -> ENS_R {
        ENS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - S-Cache Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ens(&mut self) -> ENS_W<0> {
        ENS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "S-Cache Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scactl](index.html) module"]
pub struct SCACTL_SPEC;
impl crate::RegisterSpec for SCACTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scactl::R](R) reader structure"]
impl crate::Readable for SCACTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scactl::W](W) writer structure"]
impl crate::Writable for SCACTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCACTL to value 0"]
impl crate::Resettable for SCACTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
