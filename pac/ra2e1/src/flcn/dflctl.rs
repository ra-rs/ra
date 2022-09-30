#[doc = "Register `DFLCTL` reader"]
pub struct R(crate::R<DFLCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFLCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFLCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFLCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFLCTL` writer"]
pub struct W(crate::W<DFLCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFLCTL_SPEC>;
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
impl From<crate::W<DFLCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFLCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFLEN` reader - Data Flash Access Enable"]
pub type DFLEN_R = crate::BitReader<DFLEN_A>;
#[doc = "Data Flash Access Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFLEN_A {
    #[doc = "0: Access to the data flash is disabled"]
    _0 = 0,
    #[doc = "1: Access to the data flash is enabled"]
    _1 = 1,
}
impl From<DFLEN_A> for bool {
    #[inline(always)]
    fn from(variant: DFLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DFLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFLEN_A {
        match self.bits {
            false => DFLEN_A::_0,
            true => DFLEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFLEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFLEN_A::_1
    }
}
#[doc = "Field `DFLEN` writer - Data Flash Access Enable"]
pub type DFLEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, DFLCTL_SPEC, DFLEN_A, O>;
impl<'a, const O: u8> DFLEN_W<'a, O> {
    #[doc = "Access to the data flash is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFLEN_A::_0)
    }
    #[doc = "Access to the data flash is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFLEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Data Flash Access Enable"]
    #[inline(always)]
    pub fn dflen(&self) -> DFLEN_R {
        DFLEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Flash Access Enable"]
    #[inline(always)]
    pub fn dflen(&mut self) -> DFLEN_W<0> {
        DFLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Flash Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dflctl](index.html) module"]
pub struct DFLCTL_SPEC;
impl crate::RegisterSpec for DFLCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dflctl::R](R) reader structure"]
impl crate::Readable for DFLCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dflctl::W](W) writer structure"]
impl crate::Writable for DFLCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFLCTL to value 0"]
impl crate::Resettable for DFLCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
