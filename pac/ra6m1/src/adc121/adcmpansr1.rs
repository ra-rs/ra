#[doc = "Register `ADCMPANSR1` reader"]
pub struct R(crate::R<ADCMPANSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPANSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPANSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPANSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPANSR1` writer"]
pub struct W(crate::W<ADCMPANSR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPANSR1_SPEC>;
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
impl From<crate::W<ADCMPANSR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPANSR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPCHA16` reader - AN116 Select"]
pub type CMPCHA16_R = crate::BitReader<CMPCHA16_A>;
#[doc = "AN116 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA16_A {
    #[doc = "0: Excludes AN116 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN116 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA16_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA16_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA16_A {
        match self.bits {
            false => CMPCHA16_A::_0,
            true => CMPCHA16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA16_A::_1
    }
}
#[doc = "Field `CMPCHA16` writer - AN116 Select"]
pub type CMPCHA16_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR1_SPEC, CMPCHA16_A, O>;
impl<'a, const O: u8> CMPCHA16_W<'a, O> {
    #[doc = "Excludes AN116 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA16_A::_0)
    }
    #[doc = "Includes AN116 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA16_A::_1)
    }
}
#[doc = "Field `CMPCHA17` reader - AN117 Select"]
pub type CMPCHA17_R = crate::BitReader<CMPCHA17_A>;
#[doc = "AN117 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA17_A {
    #[doc = "0: Excludes AN117 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN117 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA17_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA17_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA17_A {
        match self.bits {
            false => CMPCHA17_A::_0,
            true => CMPCHA17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA17_A::_1
    }
}
#[doc = "Field `CMPCHA17` writer - AN117 Select"]
pub type CMPCHA17_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR1_SPEC, CMPCHA17_A, O>;
impl<'a, const O: u8> CMPCHA17_W<'a, O> {
    #[doc = "Excludes AN117 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA17_A::_0)
    }
    #[doc = "Includes AN117 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA17_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AN116 Select"]
    #[inline(always)]
    pub fn cmpcha16(&self) -> CMPCHA16_R {
        CMPCHA16_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AN117 Select"]
    #[inline(always)]
    pub fn cmpcha17(&self) -> CMPCHA17_R {
        CMPCHA17_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AN116 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha16(&mut self) -> CMPCHA16_W<0> {
        CMPCHA16_W::new(self)
    }
    #[doc = "Bit 1 - AN117 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha17(&mut self) -> CMPCHA17_W<1> {
        CMPCHA17_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Compare Function Window A Channel Select Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpansr1](index.html) module"]
pub struct ADCMPANSR1_SPEC;
impl crate::RegisterSpec for ADCMPANSR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcmpansr1::R](R) reader structure"]
impl crate::Readable for ADCMPANSR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmpansr1::W](W) writer structure"]
impl crate::Writable for ADCMPANSR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPANSR1 to value 0"]
impl crate::Resettable for ADCMPANSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
