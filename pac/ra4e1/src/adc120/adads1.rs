#[doc = "Register `ADADS1` reader"]
pub struct R(crate::R<ADADS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADADS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADADS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADADS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADADS1` writer"]
pub struct W(crate::W<ADADS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADADS1_SPEC>;
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
impl From<crate::W<ADADS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADADS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADS16` reader - A/D-Converted Value Addition/Average Channel Select 16"]
pub type ADS16_R = crate::BitReader<ADS16_A>;
#[doc = "A/D-Converted Value Addition/Average Channel Select 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS16_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ADS16_A> for bool {
    #[inline(always)]
    fn from(variant: ADS16_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS16_A {
        match self.bits {
            false => ADS16_A::_0,
            true => ADS16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS16_A::_1
    }
}
#[doc = "Field `ADS16` writer - A/D-Converted Value Addition/Average Channel Select 16"]
pub type ADS16_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS1_SPEC, ADS16_A, O>;
impl<'a, const O: u8> ADS16_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS16_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS16_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - A/D-Converted Value Addition/Average Channel Select 16"]
    #[inline(always)]
    pub fn ads16(&self) -> ADS16_R {
        ADS16_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A/D-Converted Value Addition/Average Channel Select 16"]
    #[inline(always)]
    #[must_use]
    pub fn ads16(&mut self) -> ADS16_W<0> {
        ADS16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D-Converted Value Addition/Average Channel Select Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adads1](index.html) module"]
pub struct ADADS1_SPEC;
impl crate::RegisterSpec for ADADS1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adads1::R](R) reader structure"]
impl crate::Readable for ADADS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adads1::W](W) writer structure"]
impl crate::Writable for ADADS1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADADS1 to value 0"]
impl crate::Resettable for ADADS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
