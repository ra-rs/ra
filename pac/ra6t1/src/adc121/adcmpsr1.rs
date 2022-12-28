#[doc = "Register `ADCMPSR1` reader"]
pub struct R(crate::R<ADCMPSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPSR1` writer"]
pub struct W(crate::W<ADCMPSR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPSR1_SPEC>;
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
impl From<crate::W<ADCMPSR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPSR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPSTCHA16` reader - Compare window A flag of AN016\n\nThe field is **modified** in some way after a read operation."]
pub type CMPSTCHA16_R = crate::BitReader<CMPSTCHA16_A>;
#[doc = "Compare window A flag of AN016\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA16_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA16_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA16_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA16_A {
        match self.bits {
            false => CMPSTCHA16_A::_0,
            true => CMPSTCHA16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA16_A::_1
    }
}
#[doc = "Field `CMPSTCHA16` writer - Compare window A flag of AN016"]
pub type CMPSTCHA16_W<'a, const O: u8> =
    crate::BitWriter0C<'a, u16, ADCMPSR1_SPEC, CMPSTCHA16_A, O>;
impl<'a, const O: u8> CMPSTCHA16_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA16_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA16_A::_1)
    }
}
#[doc = "Field `CMPSTCHA17` reader - Compare window A flag of AN017\n\nThe field is **modified** in some way after a read operation."]
pub type CMPSTCHA17_R = crate::BitReader<CMPSTCHA17_A>;
#[doc = "Compare window A flag of AN017\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA17_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA17_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA17_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA17_A {
        match self.bits {
            false => CMPSTCHA17_A::_0,
            true => CMPSTCHA17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA17_A::_1
    }
}
#[doc = "Field `CMPSTCHA17` writer - Compare window A flag of AN017"]
pub type CMPSTCHA17_W<'a, const O: u8> =
    crate::BitWriter0C<'a, u16, ADCMPSR1_SPEC, CMPSTCHA17_A, O>;
impl<'a, const O: u8> CMPSTCHA17_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA17_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA17_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Compare window A flag of AN016"]
    #[inline(always)]
    pub fn cmpstcha16(&self) -> CMPSTCHA16_R {
        CMPSTCHA16_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare window A flag of AN017"]
    #[inline(always)]
    pub fn cmpstcha17(&self) -> CMPSTCHA17_R {
        CMPSTCHA17_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare window A flag of AN016"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha16(&mut self) -> CMPSTCHA16_W<0> {
        CMPSTCHA16_W::new(self)
    }
    #[doc = "Bit 1 - Compare window A flag of AN017"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha17(&mut self) -> CMPSTCHA17_W<1> {
        CMPSTCHA17_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Compare Function Window A Channel Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpsr1](index.html) module"]
pub struct ADCMPSR1_SPEC;
impl crate::RegisterSpec for ADCMPSR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcmpsr1::R](R) reader structure"]
impl crate::Readable for ADCMPSR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmpsr1::W](W) writer structure"]
impl crate::Writable for ADCMPSR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x03;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPSR1 to value 0"]
impl crate::Resettable for ADCMPSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
