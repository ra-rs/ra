#[doc = "Register `ADTRGEXT%s` reader"]
pub struct R(crate::R<ADTRGEXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADTRGEXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADTRGEXT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADTRGEXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADTRGEXT%s` writer"]
pub struct W(crate::W<ADTRGEXT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADTRGEXT_SPEC>;
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
impl From<crate::W<ADTRGEXT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADTRGEXT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRGEXT0` reader - External Trigger Input 0 (ADTRG0) Enable"]
pub type TRGEXT0_R = crate::BitReader<TRGEXT0_A>;
#[doc = "External Trigger Input 0 (ADTRG0) Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGEXT0_A {
    #[doc = "0: Disable ADTRG0"]
    _0 = 0,
    #[doc = "1: Enable ADTRG0"]
    _1 = 1,
}
impl From<TRGEXT0_A> for bool {
    #[inline(always)]
    fn from(variant: TRGEXT0_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGEXT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGEXT0_A {
        match self.bits {
            false => TRGEXT0_A::_0,
            true => TRGEXT0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGEXT0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGEXT0_A::_1
    }
}
#[doc = "Field `TRGEXT0` writer - External Trigger Input 0 (ADTRG0) Enable"]
pub type TRGEXT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGEXT_SPEC, TRGEXT0_A, O>;
impl<'a, const O: u8> TRGEXT0_W<'a, O> {
    #[doc = "Disable ADTRG0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGEXT0_A::_0)
    }
    #[doc = "Enable ADTRG0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGEXT0_A::_1)
    }
}
#[doc = "Field `TRGEXT1` reader - External Trigger Input 1 (ADTRG1) Enable"]
pub type TRGEXT1_R = crate::BitReader<TRGEXT1_A>;
#[doc = "External Trigger Input 1 (ADTRG1) Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGEXT1_A {
    #[doc = "0: Disable ADTRG1"]
    _0 = 0,
    #[doc = "1: Enable ADTRG1"]
    _1 = 1,
}
impl From<TRGEXT1_A> for bool {
    #[inline(always)]
    fn from(variant: TRGEXT1_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGEXT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGEXT1_A {
        match self.bits {
            false => TRGEXT1_A::_0,
            true => TRGEXT1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGEXT1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGEXT1_A::_1
    }
}
#[doc = "Field `TRGEXT1` writer - External Trigger Input 1 (ADTRG1) Enable"]
pub type TRGEXT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGEXT_SPEC, TRGEXT1_A, O>;
impl<'a, const O: u8> TRGEXT1_W<'a, O> {
    #[doc = "Disable ADTRG1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGEXT1_A::_0)
    }
    #[doc = "Enable ADTRG1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGEXT1_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - External Trigger Input 0 (ADTRG0) Enable"]
    #[inline(always)]
    pub fn trgext0(&self) -> TRGEXT0_R {
        TRGEXT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Trigger Input 1 (ADTRG1) Enable"]
    #[inline(always)]
    pub fn trgext1(&self) -> TRGEXT1_R {
        TRGEXT1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Trigger Input 0 (ADTRG0) Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgext0(&mut self) -> TRGEXT0_W<0> {
        TRGEXT0_W::new(self)
    }
    #[doc = "Bit 1 - External Trigger Input 1 (ADTRG1) Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgext1(&mut self) -> TRGEXT1_W<1> {
        TRGEXT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Trigger Enable Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adtrgext](index.html) module"]
pub struct ADTRGEXT_SPEC;
impl crate::RegisterSpec for ADTRGEXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adtrgext::R](R) reader structure"]
impl crate::Readable for ADTRGEXT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adtrgext::W](W) writer structure"]
impl crate::Writable for ADTRGEXT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADTRGEXT%s to value 0"]
impl crate::Resettable for ADTRGEXT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
