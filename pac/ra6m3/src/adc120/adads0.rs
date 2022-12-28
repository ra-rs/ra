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
#[doc = "Field `ADS00` reader - A/D-Converted Value Addition/Average Channel AN000 Select"]
pub type ADS00_R = crate::BitReader<ADS00_A>;
#[doc = "A/D-Converted Value Addition/Average Channel AN000 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS00_A {
    #[doc = "0: AN000 is not selected."]
    _0 = 0,
    #[doc = "1: AN000 is selected."]
    _1 = 1,
}
impl From<ADS00_A> for bool {
    #[inline(always)]
    fn from(variant: ADS00_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS00_A {
        match self.bits {
            false => ADS00_A::_0,
            true => ADS00_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS00_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS00_A::_1
    }
}
#[doc = "Field `ADS00` writer - A/D-Converted Value Addition/Average Channel AN000 Select"]
pub type ADS00_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS0_SPEC, ADS00_A, O>;
impl<'a, const O: u8> ADS00_W<'a, O> {
    #[doc = "AN000 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS00_A::_0)
    }
    #[doc = "AN000 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS00_A::_1)
    }
}
#[doc = "Field `ADS01` reader - A/D-Converted Value Addition/Average Channel AN001 Select"]
pub type ADS01_R = crate::BitReader<ADS01_A>;
#[doc = "A/D-Converted Value Addition/Average Channel AN001 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS01_A {
    #[doc = "0: AN001 is not selected."]
    _0 = 0,
    #[doc = "1: AN001 is selected."]
    _1 = 1,
}
impl From<ADS01_A> for bool {
    #[inline(always)]
    fn from(variant: ADS01_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS01_A {
        match self.bits {
            false => ADS01_A::_0,
            true => ADS01_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS01_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS01_A::_1
    }
}
#[doc = "Field `ADS01` writer - A/D-Converted Value Addition/Average Channel AN001 Select"]
pub type ADS01_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS0_SPEC, ADS01_A, O>;
impl<'a, const O: u8> ADS01_W<'a, O> {
    #[doc = "AN001 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS01_A::_0)
    }
    #[doc = "AN001 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS01_A::_1)
    }
}
#[doc = "Field `ADS02` reader - A/D-Converted Value Addition/Average Channel AN002 Select"]
pub type ADS02_R = crate::BitReader<ADS02_A>;
#[doc = "A/D-Converted Value Addition/Average Channel AN002 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS02_A {
    #[doc = "0: AN002 is not selected."]
    _0 = 0,
    #[doc = "1: AN002 is selected."]
    _1 = 1,
}
impl From<ADS02_A> for bool {
    #[inline(always)]
    fn from(variant: ADS02_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS02_A {
        match self.bits {
            false => ADS02_A::_0,
            true => ADS02_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS02_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS02_A::_1
    }
}
#[doc = "Field `ADS02` writer - A/D-Converted Value Addition/Average Channel AN002 Select"]
pub type ADS02_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS0_SPEC, ADS02_A, O>;
impl<'a, const O: u8> ADS02_W<'a, O> {
    #[doc = "AN002 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS02_A::_0)
    }
    #[doc = "AN002 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS02_A::_1)
    }
}
#[doc = "Field `ADS03` reader - A/D-Converted Value Addition/Average Channel AN003 Select"]
pub type ADS03_R = crate::BitReader<ADS03_A>;
#[doc = "A/D-Converted Value Addition/Average Channel AN003 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS03_A {
    #[doc = "0: AN003 is not selected."]
    _0 = 0,
    #[doc = "1: AN003 is selected."]
    _1 = 1,
}
impl From<ADS03_A> for bool {
    #[inline(always)]
    fn from(variant: ADS03_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS03_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS03_A {
        match self.bits {
            false => ADS03_A::_0,
            true => ADS03_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS03_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS03_A::_1
    }
}
#[doc = "Field `ADS03` writer - A/D-Converted Value Addition/Average Channel AN003 Select"]
pub type ADS03_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS0_SPEC, ADS03_A, O>;
impl<'a, const O: u8> ADS03_W<'a, O> {
    #[doc = "AN003 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS03_A::_0)
    }
    #[doc = "AN003 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS03_A::_1)
    }
}
#[doc = "Field `ADS04` reader - A/D-Converted Value Addition/Average Channel AN004 Select"]
pub type ADS04_R = crate::BitReader<ADS04_A>;
#[doc = "A/D-Converted Value Addition/Average Channel AN004 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS04_A {
    #[doc = "0: AN004 is not selected."]
    _0 = 0,
    #[doc = "1: AN004 is selected."]
    _1 = 1,
}
impl From<ADS04_A> for bool {
    #[inline(always)]
    fn from(variant: ADS04_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS04_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS04_A {
        match self.bits {
            false => ADS04_A::_0,
            true => ADS04_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS04_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS04_A::_1
    }
}
#[doc = "Field `ADS04` writer - A/D-Converted Value Addition/Average Channel AN004 Select"]
pub type ADS04_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS0_SPEC, ADS04_A, O>;
impl<'a, const O: u8> ADS04_W<'a, O> {
    #[doc = "AN004 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS04_A::_0)
    }
    #[doc = "AN004 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS04_A::_1)
    }
}
#[doc = "Field `ADS05` reader - A/D-Converted Value Addition/Average Channel AN005 Select"]
pub type ADS05_R = crate::BitReader<ADS05_A>;
#[doc = "A/D-Converted Value Addition/Average Channel AN005 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS05_A {
    #[doc = "0: AN005 is not selected."]
    _0 = 0,
    #[doc = "1: AN005 is selected."]
    _1 = 1,
}
impl From<ADS05_A> for bool {
    #[inline(always)]
    fn from(variant: ADS05_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS05_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS05_A {
        match self.bits {
            false => ADS05_A::_0,
            true => ADS05_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS05_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS05_A::_1
    }
}
#[doc = "Field `ADS05` writer - A/D-Converted Value Addition/Average Channel AN005 Select"]
pub type ADS05_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS0_SPEC, ADS05_A, O>;
impl<'a, const O: u8> ADS05_W<'a, O> {
    #[doc = "AN005 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS05_A::_0)
    }
    #[doc = "AN005 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS05_A::_1)
    }
}
#[doc = "Field `ADS06` reader - A/D-Converted Value Addition/Average Channel AN006 Select"]
pub type ADS06_R = crate::BitReader<ADS06_A>;
#[doc = "A/D-Converted Value Addition/Average Channel AN006 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS06_A {
    #[doc = "0: AN006 is not selected."]
    _0 = 0,
    #[doc = "1: AN006 is selected."]
    _1 = 1,
}
impl From<ADS06_A> for bool {
    #[inline(always)]
    fn from(variant: ADS06_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS06_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS06_A {
        match self.bits {
            false => ADS06_A::_0,
            true => ADS06_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS06_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS06_A::_1
    }
}
#[doc = "Field `ADS06` writer - A/D-Converted Value Addition/Average Channel AN006 Select"]
pub type ADS06_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS0_SPEC, ADS06_A, O>;
impl<'a, const O: u8> ADS06_W<'a, O> {
    #[doc = "AN006 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS06_A::_0)
    }
    #[doc = "AN006 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS06_A::_1)
    }
}
#[doc = "Field `ADS07` reader - A/D-Converted Value Addition/Average Channel AN007 Select"]
pub type ADS07_R = crate::BitReader<ADS07_A>;
#[doc = "A/D-Converted Value Addition/Average Channel AN007 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS07_A {
    #[doc = "0: AN007 is not selected."]
    _0 = 0,
    #[doc = "1: AN007 is selected."]
    _1 = 1,
}
impl From<ADS07_A> for bool {
    #[inline(always)]
    fn from(variant: ADS07_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS07_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADS07_A {
        match self.bits {
            false => ADS07_A::_0,
            true => ADS07_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS07_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS07_A::_1
    }
}
#[doc = "Field `ADS07` writer - A/D-Converted Value Addition/Average Channel AN007 Select"]
pub type ADS07_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADADS0_SPEC, ADS07_A, O>;
impl<'a, const O: u8> ADS07_W<'a, O> {
    #[doc = "AN007 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADS07_A::_0)
    }
    #[doc = "AN007 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADS07_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - A/D-Converted Value Addition/Average Channel AN000 Select"]
    #[inline(always)]
    pub fn ads00(&self) -> ADS00_R {
        ADS00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A/D-Converted Value Addition/Average Channel AN001 Select"]
    #[inline(always)]
    pub fn ads01(&self) -> ADS01_R {
        ADS01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A/D-Converted Value Addition/Average Channel AN002 Select"]
    #[inline(always)]
    pub fn ads02(&self) -> ADS02_R {
        ADS02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - A/D-Converted Value Addition/Average Channel AN003 Select"]
    #[inline(always)]
    pub fn ads03(&self) -> ADS03_R {
        ADS03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A/D-Converted Value Addition/Average Channel AN004 Select"]
    #[inline(always)]
    pub fn ads04(&self) -> ADS04_R {
        ADS04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - A/D-Converted Value Addition/Average Channel AN005 Select"]
    #[inline(always)]
    pub fn ads05(&self) -> ADS05_R {
        ADS05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - A/D-Converted Value Addition/Average Channel AN006 Select"]
    #[inline(always)]
    pub fn ads06(&self) -> ADS06_R {
        ADS06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - A/D-Converted Value Addition/Average Channel AN007 Select"]
    #[inline(always)]
    pub fn ads07(&self) -> ADS07_R {
        ADS07_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A/D-Converted Value Addition/Average Channel AN000 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads00(&mut self) -> ADS00_W<0> {
        ADS00_W::new(self)
    }
    #[doc = "Bit 1 - A/D-Converted Value Addition/Average Channel AN001 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads01(&mut self) -> ADS01_W<1> {
        ADS01_W::new(self)
    }
    #[doc = "Bit 2 - A/D-Converted Value Addition/Average Channel AN002 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads02(&mut self) -> ADS02_W<2> {
        ADS02_W::new(self)
    }
    #[doc = "Bit 3 - A/D-Converted Value Addition/Average Channel AN003 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads03(&mut self) -> ADS03_W<3> {
        ADS03_W::new(self)
    }
    #[doc = "Bit 4 - A/D-Converted Value Addition/Average Channel AN004 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads04(&mut self) -> ADS04_W<4> {
        ADS04_W::new(self)
    }
    #[doc = "Bit 5 - A/D-Converted Value Addition/Average Channel AN005 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads05(&mut self) -> ADS05_W<5> {
        ADS05_W::new(self)
    }
    #[doc = "Bit 6 - A/D-Converted Value Addition/Average Channel AN006 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads06(&mut self) -> ADS06_W<6> {
        ADS06_W::new(self)
    }
    #[doc = "Bit 7 - A/D-Converted Value Addition/Average Channel AN007 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads07(&mut self) -> ADS07_W<7> {
        ADS07_W::new(self)
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
