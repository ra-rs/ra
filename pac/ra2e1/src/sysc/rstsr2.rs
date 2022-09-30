#[doc = "Register `RSTSR2` reader"]
pub struct R(crate::R<RSTSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTSR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTSR2` writer"]
pub struct W(crate::W<RSTSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTSR2_SPEC>;
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
impl From<crate::W<RSTSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTSR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CWSF` reader - Cold/Warm Start Determination Flag"]
pub type CWSF_R = crate::BitReader<CWSF_A>;
#[doc = "Cold/Warm Start Determination Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CWSF_A {
    #[doc = "0: Cold start"]
    _0 = 0,
    #[doc = "1: Warm start"]
    _1 = 1,
}
impl From<CWSF_A> for bool {
    #[inline(always)]
    fn from(variant: CWSF_A) -> Self {
        variant as u8 != 0
    }
}
impl CWSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CWSF_A {
        match self.bits {
            false => CWSF_A::_0,
            true => CWSF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CWSF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CWSF_A::_1
    }
}
#[doc = "Field `CWSF` writer - Cold/Warm Start Determination Flag"]
pub type CWSF_W<'a, const O: u8> = crate::BitWriter<'a, u8, RSTSR2_SPEC, CWSF_A, O>;
impl<'a, const O: u8> CWSF_W<'a, O> {
    #[doc = "Cold start"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CWSF_A::_0)
    }
    #[doc = "Warm start"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CWSF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Cold/Warm Start Determination Flag"]
    #[inline(always)]
    pub fn cwsf(&self) -> CWSF_R {
        CWSF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cold/Warm Start Determination Flag"]
    #[inline(always)]
    pub fn cwsf(&mut self) -> CWSF_W<0> {
        CWSF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Status Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstsr2](index.html) module"]
pub struct RSTSR2_SPEC;
impl crate::RegisterSpec for RSTSR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rstsr2::R](R) reader structure"]
impl crate::Readable for RSTSR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstsr2::W](W) writer structure"]
impl crate::Writable for RSTSR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSTSR2 to value 0"]
impl crate::Resettable for RSTSR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
