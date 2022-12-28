#[doc = "Register `ADCMPINTCR` reader"]
pub struct R(crate::R<ADCMPINTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPINTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPINTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPINTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPINTCR` writer"]
pub struct W(crate::W<ADCMPINTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPINTCR_SPEC>;
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
impl From<crate::W<ADCMPINTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPINTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPIE0` reader - Compare Match Interrupt n Enable"]
pub type CMPIE0_R = crate::BitReader<CMPIE0_A>;
#[doc = "Compare Match Interrupt n Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPIE0_A {
    #[doc = "0: Disable the compare match interrupt n"]
    _0 = 0,
    #[doc = "1: Enable the compare match interrupt n"]
    _1 = 1,
}
impl From<CMPIE0_A> for bool {
    #[inline(always)]
    fn from(variant: CMPIE0_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPIE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPIE0_A {
        match self.bits {
            false => CMPIE0_A::_0,
            true => CMPIE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPIE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPIE0_A::_1
    }
}
#[doc = "Field `CMPIE0` writer - Compare Match Interrupt n Enable"]
pub type CMPIE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPINTCR_SPEC, CMPIE0_A, O>;
impl<'a, const O: u8> CMPIE0_W<'a, O> {
    #[doc = "Disable the compare match interrupt n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPIE0_A::_0)
    }
    #[doc = "Enable the compare match interrupt n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPIE0_A::_1)
    }
}
#[doc = "Field `CMPIE1` reader - Compare Match Interrupt n Enable"]
pub type CMPIE1_R = crate::BitReader<CMPIE1_A>;
#[doc = "Compare Match Interrupt n Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPIE1_A {
    #[doc = "0: Disable the compare match interrupt n"]
    _0 = 0,
    #[doc = "1: Enable the compare match interrupt n"]
    _1 = 1,
}
impl From<CMPIE1_A> for bool {
    #[inline(always)]
    fn from(variant: CMPIE1_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPIE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPIE1_A {
        match self.bits {
            false => CMPIE1_A::_0,
            true => CMPIE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPIE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPIE1_A::_1
    }
}
#[doc = "Field `CMPIE1` writer - Compare Match Interrupt n Enable"]
pub type CMPIE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPINTCR_SPEC, CMPIE1_A, O>;
impl<'a, const O: u8> CMPIE1_W<'a, O> {
    #[doc = "Disable the compare match interrupt n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPIE1_A::_0)
    }
    #[doc = "Enable the compare match interrupt n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPIE1_A::_1)
    }
}
#[doc = "Field `CMPIE2` reader - Compare Match Interrupt n Enable"]
pub type CMPIE2_R = crate::BitReader<CMPIE2_A>;
#[doc = "Compare Match Interrupt n Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPIE2_A {
    #[doc = "0: Disable the compare match interrupt n"]
    _0 = 0,
    #[doc = "1: Enable the compare match interrupt n"]
    _1 = 1,
}
impl From<CMPIE2_A> for bool {
    #[inline(always)]
    fn from(variant: CMPIE2_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPIE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPIE2_A {
        match self.bits {
            false => CMPIE2_A::_0,
            true => CMPIE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPIE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPIE2_A::_1
    }
}
#[doc = "Field `CMPIE2` writer - Compare Match Interrupt n Enable"]
pub type CMPIE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPINTCR_SPEC, CMPIE2_A, O>;
impl<'a, const O: u8> CMPIE2_W<'a, O> {
    #[doc = "Disable the compare match interrupt n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPIE2_A::_0)
    }
    #[doc = "Enable the compare match interrupt n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPIE2_A::_1)
    }
}
#[doc = "Field `CMPIE3` reader - Compare Match Interrupt n Enable"]
pub type CMPIE3_R = crate::BitReader<CMPIE3_A>;
#[doc = "Compare Match Interrupt n Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPIE3_A {
    #[doc = "0: Disable the compare match interrupt n"]
    _0 = 0,
    #[doc = "1: Enable the compare match interrupt n"]
    _1 = 1,
}
impl From<CMPIE3_A> for bool {
    #[inline(always)]
    fn from(variant: CMPIE3_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPIE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPIE3_A {
        match self.bits {
            false => CMPIE3_A::_0,
            true => CMPIE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPIE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPIE3_A::_1
    }
}
#[doc = "Field `CMPIE3` writer - Compare Match Interrupt n Enable"]
pub type CMPIE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPINTCR_SPEC, CMPIE3_A, O>;
impl<'a, const O: u8> CMPIE3_W<'a, O> {
    #[doc = "Disable the compare match interrupt n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPIE3_A::_0)
    }
    #[doc = "Enable the compare match interrupt n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPIE3_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Compare Match Interrupt n Enable"]
    #[inline(always)]
    pub fn cmpie0(&self) -> CMPIE0_R {
        CMPIE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare Match Interrupt n Enable"]
    #[inline(always)]
    pub fn cmpie1(&self) -> CMPIE1_R {
        CMPIE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare Match Interrupt n Enable"]
    #[inline(always)]
    pub fn cmpie2(&self) -> CMPIE2_R {
        CMPIE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare Match Interrupt n Enable"]
    #[inline(always)]
    pub fn cmpie3(&self) -> CMPIE3_R {
        CMPIE3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare Match Interrupt n Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpie0(&mut self) -> CMPIE0_W<0> {
        CMPIE0_W::new(self)
    }
    #[doc = "Bit 1 - Compare Match Interrupt n Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpie1(&mut self) -> CMPIE1_W<1> {
        CMPIE1_W::new(self)
    }
    #[doc = "Bit 2 - Compare Match Interrupt n Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpie2(&mut self) -> CMPIE2_W<2> {
        CMPIE2_W::new(self)
    }
    #[doc = "Bit 3 - Compare Match Interrupt n Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpie3(&mut self) -> CMPIE3_W<3> {
        CMPIE3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare Match Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpintcr](index.html) module"]
pub struct ADCMPINTCR_SPEC;
impl crate::RegisterSpec for ADCMPINTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcmpintcr::R](R) reader structure"]
impl crate::Readable for ADCMPINTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmpintcr::W](W) writer structure"]
impl crate::Writable for ADCMPINTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPINTCR to value 0"]
impl crate::Resettable for ADCMPINTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
