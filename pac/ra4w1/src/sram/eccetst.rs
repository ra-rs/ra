#[doc = "Register `ECCETST` reader"]
pub struct R(crate::R<ECCETST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECCETST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECCETST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECCETST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECCETST` writer"]
pub struct W(crate::W<ECCETST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECCETST_SPEC>;
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
impl From<crate::W<ECCETST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECCETST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSTBYP` reader - ECC Bypass Select"]
pub type TSTBYP_R = crate::BitReader<TSTBYP_A>;
#[doc = "ECC Bypass Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSTBYP_A {
    #[doc = "0: ECC bypass disabled."]
    _0 = 0,
    #[doc = "1: ECC bypass enabled."]
    _1 = 1,
}
impl From<TSTBYP_A> for bool {
    #[inline(always)]
    fn from(variant: TSTBYP_A) -> Self {
        variant as u8 != 0
    }
}
impl TSTBYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSTBYP_A {
        match self.bits {
            false => TSTBYP_A::_0,
            true => TSTBYP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSTBYP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSTBYP_A::_1
    }
}
#[doc = "Field `TSTBYP` writer - ECC Bypass Select"]
pub type TSTBYP_W<'a, const O: u8> = crate::BitWriter<'a, u8, ECCETST_SPEC, TSTBYP_A, O>;
impl<'a, const O: u8> TSTBYP_W<'a, O> {
    #[doc = "ECC bypass disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSTBYP_A::_0)
    }
    #[doc = "ECC bypass enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSTBYP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - ECC Bypass Select"]
    #[inline(always)]
    pub fn tstbyp(&self) -> TSTBYP_R {
        TSTBYP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC Bypass Select"]
    #[inline(always)]
    #[must_use]
    pub fn tstbyp(&mut self) -> TSTBYP_W<0> {
        TSTBYP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECC Test Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccetst](index.html) module"]
pub struct ECCETST_SPEC;
impl crate::RegisterSpec for ECCETST_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [eccetst::R](R) reader structure"]
impl crate::Readable for ECCETST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eccetst::W](W) writer structure"]
impl crate::Writable for ECCETST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECCETST to value 0"]
impl crate::Resettable for ECCETST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
