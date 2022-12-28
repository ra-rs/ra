#[doc = "Register `GETW10R` reader"]
pub struct R(crate::R<GETW10R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GETW10R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GETW10R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GETW10R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GETW10R` writer"]
pub struct W(crate::W<GETW10R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GETW10R_SPEC>;
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
impl From<crate::W<GETW10R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GETW10R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GW10` reader - Worst 10 Acquisition Directive"]
pub type GW10_R = crate::BitReader<GW10_A>;
#[doc = "Worst 10 Acquisition Directive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GW10_A {
    #[doc = "0: The worst-10 values are not acquired."]
    _0 = 0,
    #[doc = "1: Starts acquisition of the worst-10 values."]
    _1 = 1,
}
impl From<GW10_A> for bool {
    #[inline(always)]
    fn from(variant: GW10_A) -> Self {
        variant as u8 != 0
    }
}
impl GW10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GW10_A {
        match self.bits {
            false => GW10_A::_0,
            true => GW10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GW10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GW10_A::_1
    }
}
#[doc = "Field `GW10` writer - Worst 10 Acquisition Directive"]
pub type GW10_W<'a, const O: u8> = crate::BitWriter<'a, u32, GETW10R_SPEC, GW10_A, O>;
impl<'a, const O: u8> GW10_W<'a, O> {
    #[doc = "The worst-10 values are not acquired."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GW10_A::_0)
    }
    #[doc = "Starts acquisition of the worst-10 values."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GW10_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Worst 10 Acquisition Directive"]
    #[inline(always)]
    pub fn gw10(&self) -> GW10_R {
        GW10_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Worst 10 Acquisition Directive"]
    #[inline(always)]
    #[must_use]
    pub fn gw10(&mut self) -> GW10_W<0> {
        GW10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Worst 10 Acquisition Directive Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [getw10r](index.html) module"]
pub struct GETW10R_SPEC;
impl crate::RegisterSpec for GETW10R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [getw10r::R](R) reader structure"]
impl crate::Readable for GETW10R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [getw10r::W](W) writer structure"]
impl crate::Writable for GETW10R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GETW10R to value 0"]
impl crate::Resettable for GETW10R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
