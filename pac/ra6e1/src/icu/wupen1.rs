#[doc = "Register `WUPEN1` reader"]
pub struct R(crate::R<WUPEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUPEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUPEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUPEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUPEN1` writer"]
pub struct W(crate::W<WUPEN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUPEN1_SPEC>;
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
impl From<crate::W<WUPEN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUPEN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AGT3UDWUPEN` reader - AGT3 Underflow Interrupt Software Standby Return Enable bit"]
pub type AGT3UDWUPEN_R = crate::BitReader<AGT3UDWUPEN_A>;
#[doc = "AGT3 Underflow Interrupt Software Standby Return Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AGT3UDWUPEN_A {
    #[doc = "0: Software standby returns by AGT3 underflow interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software standby returns by AGT3 underflow interrupt is enabled"]
    _1 = 1,
}
impl From<AGT3UDWUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: AGT3UDWUPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AGT3UDWUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AGT3UDWUPEN_A {
        match self.bits {
            false => AGT3UDWUPEN_A::_0,
            true => AGT3UDWUPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AGT3UDWUPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AGT3UDWUPEN_A::_1
    }
}
#[doc = "Field `AGT3UDWUPEN` writer - AGT3 Underflow Interrupt Software Standby Return Enable bit"]
pub type AGT3UDWUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN1_SPEC, AGT3UDWUPEN_A, O>;
impl<'a, const O: u8> AGT3UDWUPEN_W<'a, O> {
    #[doc = "Software standby returns by AGT3 underflow interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AGT3UDWUPEN_A::_0)
    }
    #[doc = "Software standby returns by AGT3 underflow interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AGT3UDWUPEN_A::_1)
    }
}
#[doc = "Field `AGT3CAWUPEN` reader - AGT3 Compare Match A Interrupt Software Standby Return Enable bit"]
pub type AGT3CAWUPEN_R = crate::BitReader<AGT3CAWUPEN_A>;
#[doc = "AGT3 Compare Match A Interrupt Software Standby Return Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AGT3CAWUPEN_A {
    #[doc = "0: Software standby returns by AGT3 compare match A interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software standby returns by AGT3 compare match A interrupt is enabled"]
    _1 = 1,
}
impl From<AGT3CAWUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: AGT3CAWUPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AGT3CAWUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AGT3CAWUPEN_A {
        match self.bits {
            false => AGT3CAWUPEN_A::_0,
            true => AGT3CAWUPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AGT3CAWUPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AGT3CAWUPEN_A::_1
    }
}
#[doc = "Field `AGT3CAWUPEN` writer - AGT3 Compare Match A Interrupt Software Standby Return Enable bit"]
pub type AGT3CAWUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN1_SPEC, AGT3CAWUPEN_A, O>;
impl<'a, const O: u8> AGT3CAWUPEN_W<'a, O> {
    #[doc = "Software standby returns by AGT3 compare match A interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AGT3CAWUPEN_A::_0)
    }
    #[doc = "Software standby returns by AGT3 compare match A interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AGT3CAWUPEN_A::_1)
    }
}
#[doc = "Field `AGT3CBWUPEN` reader - AGT3 Compare Match B Interrupt Software Standby Return Enable bit"]
pub type AGT3CBWUPEN_R = crate::BitReader<AGT3CBWUPEN_A>;
#[doc = "AGT3 Compare Match B Interrupt Software Standby Return Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AGT3CBWUPEN_A {
    #[doc = "0: Software standby returns by AGT3 compare match B interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software standby returns by AGT3 compare match B interrupt is enabled"]
    _1 = 1,
}
impl From<AGT3CBWUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: AGT3CBWUPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AGT3CBWUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AGT3CBWUPEN_A {
        match self.bits {
            false => AGT3CBWUPEN_A::_0,
            true => AGT3CBWUPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AGT3CBWUPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AGT3CBWUPEN_A::_1
    }
}
#[doc = "Field `AGT3CBWUPEN` writer - AGT3 Compare Match B Interrupt Software Standby Return Enable bit"]
pub type AGT3CBWUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN1_SPEC, AGT3CBWUPEN_A, O>;
impl<'a, const O: u8> AGT3CBWUPEN_W<'a, O> {
    #[doc = "Software standby returns by AGT3 compare match B interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AGT3CBWUPEN_A::_0)
    }
    #[doc = "Software standby returns by AGT3 compare match B interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AGT3CBWUPEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AGT3 Underflow Interrupt Software Standby Return Enable bit"]
    #[inline(always)]
    pub fn agt3udwupen(&self) -> AGT3UDWUPEN_R {
        AGT3UDWUPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AGT3 Compare Match A Interrupt Software Standby Return Enable bit"]
    #[inline(always)]
    pub fn agt3cawupen(&self) -> AGT3CAWUPEN_R {
        AGT3CAWUPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AGT3 Compare Match B Interrupt Software Standby Return Enable bit"]
    #[inline(always)]
    pub fn agt3cbwupen(&self) -> AGT3CBWUPEN_R {
        AGT3CBWUPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AGT3 Underflow Interrupt Software Standby Return Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn agt3udwupen(&mut self) -> AGT3UDWUPEN_W<0> {
        AGT3UDWUPEN_W::new(self)
    }
    #[doc = "Bit 1 - AGT3 Compare Match A Interrupt Software Standby Return Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn agt3cawupen(&mut self) -> AGT3CAWUPEN_W<1> {
        AGT3CAWUPEN_W::new(self)
    }
    #[doc = "Bit 2 - AGT3 Compare Match B Interrupt Software Standby Return Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn agt3cbwupen(&mut self) -> AGT3CBWUPEN_W<2> {
        AGT3CBWUPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wake Up interrupt enable register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wupen1](index.html) module"]
pub struct WUPEN1_SPEC;
impl crate::RegisterSpec for WUPEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wupen1::R](R) reader structure"]
impl crate::Readable for WUPEN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wupen1::W](W) writer structure"]
impl crate::Writable for WUPEN1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WUPEN1 to value 0"]
impl crate::Resettable for WUPEN1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
