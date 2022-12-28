#[doc = "Register `SNZREQCR1` reader"]
pub struct R(crate::R<SNZREQCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SNZREQCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SNZREQCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SNZREQCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SNZREQCR1` writer"]
pub struct W(crate::W<SNZREQCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SNZREQCR1_SPEC>;
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
impl From<crate::W<SNZREQCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SNZREQCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SNZREQEN0` reader - Enable AGT3 underflow snooze request"]
pub type SNZREQEN0_R = crate::BitReader<SNZREQEN0_A>;
#[doc = "Enable AGT3 underflow snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN0_A {
    #[doc = "0: Disable the snooze request"]
    _0 = 0,
    #[doc = "1: Enable the snooze request"]
    _1 = 1,
}
impl From<SNZREQEN0_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNZREQEN0_A {
        match self.bits {
            false => SNZREQEN0_A::_0,
            true => SNZREQEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN0_A::_1
    }
}
#[doc = "Field `SNZREQEN0` writer - Enable AGT3 underflow snooze request"]
pub type SNZREQEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNZREQCR1_SPEC, SNZREQEN0_A, O>;
impl<'a, const O: u8> SNZREQEN0_W<'a, O> {
    #[doc = "Disable the snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNZREQEN0_A::_0)
    }
    #[doc = "Enable the snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNZREQEN0_A::_1)
    }
}
#[doc = "Field `SNZREQEN1` reader - Enable AGT3 compare match A snooze request"]
pub type SNZREQEN1_R = crate::BitReader<SNZREQEN1_A>;
#[doc = "Enable AGT3 compare match A snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN1_A {
    #[doc = "0: Disable the snooze request"]
    _0 = 0,
    #[doc = "1: Enable the snooze request"]
    _1 = 1,
}
impl From<SNZREQEN1_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNZREQEN1_A {
        match self.bits {
            false => SNZREQEN1_A::_0,
            true => SNZREQEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN1_A::_1
    }
}
#[doc = "Field `SNZREQEN1` writer - Enable AGT3 compare match A snooze request"]
pub type SNZREQEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNZREQCR1_SPEC, SNZREQEN1_A, O>;
impl<'a, const O: u8> SNZREQEN1_W<'a, O> {
    #[doc = "Disable the snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNZREQEN1_A::_0)
    }
    #[doc = "Enable the snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNZREQEN1_A::_1)
    }
}
#[doc = "Field `SNZREQEN2` reader - Enable AGT3 compare match B snooze request"]
pub type SNZREQEN2_R = crate::BitReader<SNZREQEN2_A>;
#[doc = "Enable AGT3 compare match B snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN2_A {
    #[doc = "0: Disable the snooze request"]
    _0 = 0,
    #[doc = "1: Enable the snooze request"]
    _1 = 1,
}
impl From<SNZREQEN2_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNZREQEN2_A {
        match self.bits {
            false => SNZREQEN2_A::_0,
            true => SNZREQEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN2_A::_1
    }
}
#[doc = "Field `SNZREQEN2` writer - Enable AGT3 compare match B snooze request"]
pub type SNZREQEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNZREQCR1_SPEC, SNZREQEN2_A, O>;
impl<'a, const O: u8> SNZREQEN2_W<'a, O> {
    #[doc = "Disable the snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNZREQEN2_A::_0)
    }
    #[doc = "Enable the snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNZREQEN2_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Enable AGT3 underflow snooze request"]
    #[inline(always)]
    pub fn snzreqen0(&self) -> SNZREQEN0_R {
        SNZREQEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable AGT3 compare match A snooze request"]
    #[inline(always)]
    pub fn snzreqen1(&self) -> SNZREQEN1_R {
        SNZREQEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable AGT3 compare match B snooze request"]
    #[inline(always)]
    pub fn snzreqen2(&self) -> SNZREQEN2_R {
        SNZREQEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable AGT3 underflow snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen0(&mut self) -> SNZREQEN0_W<0> {
        SNZREQEN0_W::new(self)
    }
    #[doc = "Bit 1 - Enable AGT3 compare match A snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen1(&mut self) -> SNZREQEN1_W<1> {
        SNZREQEN1_W::new(self)
    }
    #[doc = "Bit 2 - Enable AGT3 compare match B snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen2(&mut self) -> SNZREQEN2_W<2> {
        SNZREQEN2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Snooze Request Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snzreqcr1](index.html) module"]
pub struct SNZREQCR1_SPEC;
impl crate::RegisterSpec for SNZREQCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [snzreqcr1::R](R) reader structure"]
impl crate::Readable for SNZREQCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [snzreqcr1::W](W) writer structure"]
impl crate::Writable for SNZREQCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SNZREQCR1 to value 0"]
impl crate::Resettable for SNZREQCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
