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
#[doc = "Field `ADS16` reader - A/D-Converted Value Addition/Average Channel AN016 Select"]
pub type ADS16_R = crate::BitReader<ADS16_A>;
#[doc = "A/D-Converted Value Addition/Average Channel AN016 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS16_A {
    #[doc = "0: AN016 is not selected."]
    _0 = 0,
    #[doc = "1: AN016 is selected."]
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
#[doc = "Field `ADS16` writer - A/D-Converted Value Addition/Average Channel AN016 Select"]
pub type ADS16_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS1_SPEC, ADS16_A, O>;
impl<'a, const O: u8> ADS16_W<'a, O> {
    #[doc = "AN016 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS16_A::_0)
    }
    #[doc = "AN016 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS16_A::_1)
    }
}
#[doc = "Field `ADS17` reader - A/D-Converted Value Addition/Average Channel AN017 Select"]
pub type ADS17_R = crate::BitReader<ADS17_A>;
#[doc = "A/D-Converted Value Addition/Average Channel AN017 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS17_A {
    #[doc = "0: AN017 is not selected."]
    _0 = 0,
    #[doc = "1: AN017 is selected."]
    _1 = 1,
}
impl From<ADS17_A> for bool {
    #[inline(always)]
    fn from(variant: ADS17_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS17_A {
        match self.bits {
            false => ADS17_A::_0,
            true => ADS17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS17_A::_1
    }
}
#[doc = "Field `ADS17` writer - A/D-Converted Value Addition/Average Channel AN017 Select"]
pub type ADS17_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS1_SPEC, ADS17_A, O>;
impl<'a, const O: u8> ADS17_W<'a, O> {
    #[doc = "AN017 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS17_A::_0)
    }
    #[doc = "AN017 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS17_A::_1)
    }
}
#[doc = "Field `ADS18` reader - A/D-Converted Value Addition/Average Channel AN018 Select"]
pub type ADS18_R = crate::BitReader<ADS18_A>;
#[doc = "A/D-Converted Value Addition/Average Channel AN018 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS18_A {
    #[doc = "0: AN018 is not selected."]
    _0 = 0,
    #[doc = "1: AN018 is selected."]
    _1 = 1,
}
impl From<ADS18_A> for bool {
    #[inline(always)]
    fn from(variant: ADS18_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS18_A {
        match self.bits {
            false => ADS18_A::_0,
            true => ADS18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS18_A::_1
    }
}
#[doc = "Field `ADS18` writer - A/D-Converted Value Addition/Average Channel AN018 Select"]
pub type ADS18_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS1_SPEC, ADS18_A, O>;
impl<'a, const O: u8> ADS18_W<'a, O> {
    #[doc = "AN018 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS18_A::_0)
    }
    #[doc = "AN018 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS18_A::_1)
    }
}
#[doc = "Field `ADS19` reader - A/D-Converted Value Addition/Average Channel AN019 Select"]
pub type ADS19_R = crate::BitReader<ADS19_A>;
#[doc = "A/D-Converted Value Addition/Average Channel AN019 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS19_A {
    #[doc = "0: AN019 is not selected."]
    _0 = 0,
    #[doc = "1: AN019 is selected."]
    _1 = 1,
}
impl From<ADS19_A> for bool {
    #[inline(always)]
    fn from(variant: ADS19_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS19_A {
        match self.bits {
            false => ADS19_A::_0,
            true => ADS19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS19_A::_1
    }
}
#[doc = "Field `ADS19` writer - A/D-Converted Value Addition/Average Channel AN019 Select"]
pub type ADS19_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS1_SPEC, ADS19_A, O>;
impl<'a, const O: u8> ADS19_W<'a, O> {
    #[doc = "AN019 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS19_A::_0)
    }
    #[doc = "AN019 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS19_A::_1)
    }
}
#[doc = "Field `ADS20` reader - A/D-Converted Value Addition/Average Channel AN020 Select"]
pub type ADS20_R = crate::BitReader<ADS20_A>;
#[doc = "A/D-Converted Value Addition/Average Channel AN020 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS20_A {
    #[doc = "0: AN020 is not selected."]
    _0 = 0,
    #[doc = "1: AN020 is selected."]
    _1 = 1,
}
impl From<ADS20_A> for bool {
    #[inline(always)]
    fn from(variant: ADS20_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS20_A {
        match self.bits {
            false => ADS20_A::_0,
            true => ADS20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS20_A::_1
    }
}
#[doc = "Field `ADS20` writer - A/D-Converted Value Addition/Average Channel AN020 Select"]
pub type ADS20_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS1_SPEC, ADS20_A, O>;
impl<'a, const O: u8> ADS20_W<'a, O> {
    #[doc = "AN020 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS20_A::_0)
    }
    #[doc = "AN020 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS20_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - A/D-Converted Value Addition/Average Channel AN016 Select"]
    #[inline(always)]
    pub fn ads16(&self) -> ADS16_R {
        ADS16_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A/D-Converted Value Addition/Average Channel AN017 Select"]
    #[inline(always)]
    pub fn ads17(&self) -> ADS17_R {
        ADS17_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A/D-Converted Value Addition/Average Channel AN018 Select"]
    #[inline(always)]
    pub fn ads18(&self) -> ADS18_R {
        ADS18_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - A/D-Converted Value Addition/Average Channel AN019 Select"]
    #[inline(always)]
    pub fn ads19(&self) -> ADS19_R {
        ADS19_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A/D-Converted Value Addition/Average Channel AN020 Select"]
    #[inline(always)]
    pub fn ads20(&self) -> ADS20_R {
        ADS20_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A/D-Converted Value Addition/Average Channel AN016 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads16(&mut self) -> ADS16_W<0> {
        ADS16_W::new(self)
    }
    #[doc = "Bit 1 - A/D-Converted Value Addition/Average Channel AN017 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads17(&mut self) -> ADS17_W<1> {
        ADS17_W::new(self)
    }
    #[doc = "Bit 2 - A/D-Converted Value Addition/Average Channel AN018 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads18(&mut self) -> ADS18_W<2> {
        ADS18_W::new(self)
    }
    #[doc = "Bit 3 - A/D-Converted Value Addition/Average Channel AN019 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads19(&mut self) -> ADS19_W<3> {
        ADS19_W::new(self)
    }
    #[doc = "Bit 4 - A/D-Converted Value Addition/Average Channel AN020 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads20(&mut self) -> ADS20_W<4> {
        ADS20_W::new(self)
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
