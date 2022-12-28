#[doc = "Register `GAM_SW` reader"]
pub struct R(crate::R<GAM_SW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAM_SW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAM_SW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAM_SW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAM_SW` writer"]
pub struct W(crate::W<GAM_SW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAM_SW_SPEC>;
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
impl From<crate::W<GAM_SW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAM_SW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GAMON` reader - Gamma correction on/off control"]
pub type GAMON_R = crate::BitReader<GAMON_A>;
#[doc = "Gamma correction on/off control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GAMON_A {
    #[doc = "1: Turns on gamma correction."]
    _1 = 1,
    #[doc = "0: Turns off gamma correction."]
    _0 = 0,
}
impl From<GAMON_A> for bool {
    #[inline(always)]
    fn from(variant: GAMON_A) -> Self {
        variant as u8 != 0
    }
}
impl GAMON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAMON_A {
        match self.bits {
            true => GAMON_A::_1,
            false => GAMON_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GAMON_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GAMON_A::_0
    }
}
#[doc = "Field `GAMON` writer - Gamma correction on/off control"]
pub type GAMON_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAM_SW_SPEC, GAMON_A, O>;
impl<'a, const O: u8> GAMON_W<'a, O> {
    #[doc = "Turns on gamma correction."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GAMON_A::_1)
    }
    #[doc = "Turns off gamma correction."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GAMON_A::_0)
    }
}
impl R {
    #[doc = "Bit 0 - Gamma correction on/off control"]
    #[inline(always)]
    pub fn gamon(&self) -> GAMON_R {
        GAMON_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Gamma correction on/off control"]
    #[inline(always)]
    #[must_use]
    pub fn gamon(&mut self) -> GAMON_W<0> {
        GAMON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Gamma Correction Block Function Switch Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gam_sw](index.html) module"]
pub struct GAM_SW_SPEC;
impl crate::RegisterSpec for GAM_SW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gam_sw::R](R) reader structure"]
impl crate::Readable for GAM_SW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gam_sw::W](W) writer structure"]
impl crate::Writable for GAM_SW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GAM_SW to value 0"]
impl crate::Resettable for GAM_SW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
