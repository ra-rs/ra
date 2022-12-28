#[doc = "Register `ADADS0` reader"]
pub struct R(crate::R<ADADS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADADS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADADS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADADS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADADS0` writer"]
pub struct W(crate::W<ADADS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADADS0_SPEC>;
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
impl From<crate::W<ADADS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADADS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADS0` reader - A/D-Converted Value Addition/Average Channel Select n"]
pub type ADS0_R = crate::BitReader<ADS0_A>;
#[doc = "A/D-Converted Value Addition/Average Channel Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS0_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ADS0_A> for bool {
    #[inline(always)]
    fn from(variant: ADS0_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS0_A {
        match self.bits {
            false => ADS0_A::_0,
            true => ADS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS0_A::_1
    }
}
#[doc = "Field `ADS0` writer - A/D-Converted Value Addition/Average Channel Select n"]
pub type ADS0_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS0_SPEC, ADS0_A, O>;
impl<'a, const O: u8> ADS0_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS0_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS0_A::_1)
    }
}
#[doc = "Field `ADS1` reader - A/D-Converted Value Addition/Average Channel Select n"]
pub type ADS1_R = crate::BitReader<ADS1_A>;
#[doc = "A/D-Converted Value Addition/Average Channel Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS1_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ADS1_A> for bool {
    #[inline(always)]
    fn from(variant: ADS1_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS1_A {
        match self.bits {
            false => ADS1_A::_0,
            true => ADS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS1_A::_1
    }
}
#[doc = "Field `ADS1` writer - A/D-Converted Value Addition/Average Channel Select n"]
pub type ADS1_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS0_SPEC, ADS1_A, O>;
impl<'a, const O: u8> ADS1_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS1_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS1_A::_1)
    }
}
#[doc = "Field `ADS2` reader - A/D-Converted Value Addition/Average Channel Select n"]
pub type ADS2_R = crate::BitReader<ADS2_A>;
#[doc = "A/D-Converted Value Addition/Average Channel Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS2_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ADS2_A> for bool {
    #[inline(always)]
    fn from(variant: ADS2_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS2_A {
        match self.bits {
            false => ADS2_A::_0,
            true => ADS2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS2_A::_1
    }
}
#[doc = "Field `ADS2` writer - A/D-Converted Value Addition/Average Channel Select n"]
pub type ADS2_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS0_SPEC, ADS2_A, O>;
impl<'a, const O: u8> ADS2_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS2_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS2_A::_1)
    }
}
#[doc = "Field `ADS3` reader - A/D-Converted Value Addition/Average Channel Select n"]
pub type ADS3_R = crate::BitReader<ADS3_A>;
#[doc = "A/D-Converted Value Addition/Average Channel Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS3_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ADS3_A> for bool {
    #[inline(always)]
    fn from(variant: ADS3_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS3_A {
        match self.bits {
            false => ADS3_A::_0,
            true => ADS3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS3_A::_1
    }
}
#[doc = "Field `ADS3` writer - A/D-Converted Value Addition/Average Channel Select n"]
pub type ADS3_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS0_SPEC, ADS3_A, O>;
impl<'a, const O: u8> ADS3_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS3_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS3_A::_1)
    }
}
#[doc = "Field `ADS4` reader - A/D-Converted Value Addition/Average Channel Select n"]
pub type ADS4_R = crate::BitReader<ADS4_A>;
#[doc = "A/D-Converted Value Addition/Average Channel Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS4_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ADS4_A> for bool {
    #[inline(always)]
    fn from(variant: ADS4_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS4_A {
        match self.bits {
            false => ADS4_A::_0,
            true => ADS4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS4_A::_1
    }
}
#[doc = "Field `ADS4` writer - A/D-Converted Value Addition/Average Channel Select n"]
pub type ADS4_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS0_SPEC, ADS4_A, O>;
impl<'a, const O: u8> ADS4_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS4_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS4_A::_1)
    }
}
#[doc = "Field `ADS11` reader - A/D-Converted Value Addition/Average Channel Select n"]
pub type ADS11_R = crate::BitReader<ADS11_A>;
#[doc = "A/D-Converted Value Addition/Average Channel Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS11_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ADS11_A> for bool {
    #[inline(always)]
    fn from(variant: ADS11_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS11_A {
        match self.bits {
            false => ADS11_A::_0,
            true => ADS11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS11_A::_1
    }
}
#[doc = "Field `ADS11` writer - A/D-Converted Value Addition/Average Channel Select n"]
pub type ADS11_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS0_SPEC, ADS11_A, O>;
impl<'a, const O: u8> ADS11_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS11_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS11_A::_1)
    }
}
#[doc = "Field `ADS12` reader - A/D-Converted Value Addition/Average Channel Select n"]
pub type ADS12_R = crate::BitReader<ADS12_A>;
#[doc = "A/D-Converted Value Addition/Average Channel Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS12_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ADS12_A> for bool {
    #[inline(always)]
    fn from(variant: ADS12_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS12_A {
        match self.bits {
            false => ADS12_A::_0,
            true => ADS12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS12_A::_1
    }
}
#[doc = "Field `ADS12` writer - A/D-Converted Value Addition/Average Channel Select n"]
pub type ADS12_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS0_SPEC, ADS12_A, O>;
impl<'a, const O: u8> ADS12_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS12_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS12_A::_1)
    }
}
#[doc = "Field `ADS13` reader - A/D-Converted Value Addition/Average Channel Select n"]
pub type ADS13_R = crate::BitReader<ADS13_A>;
#[doc = "A/D-Converted Value Addition/Average Channel Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS13_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ADS13_A> for bool {
    #[inline(always)]
    fn from(variant: ADS13_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS13_A {
        match self.bits {
            false => ADS13_A::_0,
            true => ADS13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS13_A::_1
    }
}
#[doc = "Field `ADS13` writer - A/D-Converted Value Addition/Average Channel Select n"]
pub type ADS13_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS0_SPEC, ADS13_A, O>;
impl<'a, const O: u8> ADS13_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS13_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS13_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - A/D-Converted Value Addition/Average Channel Select n"]
    #[inline(always)]
    pub fn ads0(&self) -> ADS0_R {
        ADS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A/D-Converted Value Addition/Average Channel Select n"]
    #[inline(always)]
    pub fn ads1(&self) -> ADS1_R {
        ADS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A/D-Converted Value Addition/Average Channel Select n"]
    #[inline(always)]
    pub fn ads2(&self) -> ADS2_R {
        ADS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - A/D-Converted Value Addition/Average Channel Select n"]
    #[inline(always)]
    pub fn ads3(&self) -> ADS3_R {
        ADS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A/D-Converted Value Addition/Average Channel Select n"]
    #[inline(always)]
    pub fn ads4(&self) -> ADS4_R {
        ADS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 11 - A/D-Converted Value Addition/Average Channel Select n"]
    #[inline(always)]
    pub fn ads11(&self) -> ADS11_R {
        ADS11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - A/D-Converted Value Addition/Average Channel Select n"]
    #[inline(always)]
    pub fn ads12(&self) -> ADS12_R {
        ADS12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - A/D-Converted Value Addition/Average Channel Select n"]
    #[inline(always)]
    pub fn ads13(&self) -> ADS13_R {
        ADS13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A/D-Converted Value Addition/Average Channel Select n"]
    #[inline(always)]
    #[must_use]
    pub fn ads0(&mut self) -> ADS0_W<0> {
        ADS0_W::new(self)
    }
    #[doc = "Bit 1 - A/D-Converted Value Addition/Average Channel Select n"]
    #[inline(always)]
    #[must_use]
    pub fn ads1(&mut self) -> ADS1_W<1> {
        ADS1_W::new(self)
    }
    #[doc = "Bit 2 - A/D-Converted Value Addition/Average Channel Select n"]
    #[inline(always)]
    #[must_use]
    pub fn ads2(&mut self) -> ADS2_W<2> {
        ADS2_W::new(self)
    }
    #[doc = "Bit 3 - A/D-Converted Value Addition/Average Channel Select n"]
    #[inline(always)]
    #[must_use]
    pub fn ads3(&mut self) -> ADS3_W<3> {
        ADS3_W::new(self)
    }
    #[doc = "Bit 4 - A/D-Converted Value Addition/Average Channel Select n"]
    #[inline(always)]
    #[must_use]
    pub fn ads4(&mut self) -> ADS4_W<4> {
        ADS4_W::new(self)
    }
    #[doc = "Bit 11 - A/D-Converted Value Addition/Average Channel Select n"]
    #[inline(always)]
    #[must_use]
    pub fn ads11(&mut self) -> ADS11_W<11> {
        ADS11_W::new(self)
    }
    #[doc = "Bit 12 - A/D-Converted Value Addition/Average Channel Select n"]
    #[inline(always)]
    #[must_use]
    pub fn ads12(&mut self) -> ADS12_W<12> {
        ADS12_W::new(self)
    }
    #[doc = "Bit 13 - A/D-Converted Value Addition/Average Channel Select n"]
    #[inline(always)]
    #[must_use]
    pub fn ads13(&mut self) -> ADS13_W<13> {
        ADS13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D-Converted Value Addition/Average Channel Select Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adads0](index.html) module"]
pub struct ADADS0_SPEC;
impl crate::RegisterSpec for ADADS0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adads0::R](R) reader structure"]
impl crate::Readable for ADADS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adads0::W](W) writer structure"]
impl crate::Writable for ADADS0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADADS0 to value 0"]
impl crate::Resettable for ADADS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
