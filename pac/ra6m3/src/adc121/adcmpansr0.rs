#[doc = "Register `ADCMPANSR0` reader"]
pub struct R(crate::R<ADCMPANSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPANSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPANSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPANSR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPANSR0` writer"]
pub struct W(crate::W<ADCMPANSR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPANSR0_SPEC>;
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
impl From<crate::W<ADCMPANSR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPANSR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPCHA00` reader - Compare Window A Channel AN100 Select"]
pub type CMPCHA00_R = crate::BitReader<CMPCHA00_A>;
#[doc = "Compare Window A Channel AN100 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA00_A {
    #[doc = "0: Disable compare function for AN100"]
    _0 = 0,
    #[doc = "1: Enable compare function for AN100"]
    _1 = 1,
}
impl From<CMPCHA00_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA00_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA00_A {
        match self.bits {
            false => CMPCHA00_A::_0,
            true => CMPCHA00_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA00_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA00_A::_1
    }
}
#[doc = "Field `CMPCHA00` writer - Compare Window A Channel AN100 Select"]
pub type CMPCHA00_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA00_A, O>;
impl<'a, const O: u8> CMPCHA00_W<'a, O> {
    #[doc = "Disable compare function for AN100"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA00_A::_0)
    }
    #[doc = "Enable compare function for AN100"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA00_A::_1)
    }
}
#[doc = "Field `CMPCHA01` reader - Compare Window A Channel AN101 Select"]
pub type CMPCHA01_R = crate::BitReader<CMPCHA01_A>;
#[doc = "Compare Window A Channel AN101 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA01_A {
    #[doc = "0: Disable compare function for AN101"]
    _0 = 0,
    #[doc = "1: Enable compare function for AN101"]
    _1 = 1,
}
impl From<CMPCHA01_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA01_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA01_A {
        match self.bits {
            false => CMPCHA01_A::_0,
            true => CMPCHA01_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA01_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA01_A::_1
    }
}
#[doc = "Field `CMPCHA01` writer - Compare Window A Channel AN101 Select"]
pub type CMPCHA01_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA01_A, O>;
impl<'a, const O: u8> CMPCHA01_W<'a, O> {
    #[doc = "Disable compare function for AN101"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA01_A::_0)
    }
    #[doc = "Enable compare function for AN101"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA01_A::_1)
    }
}
#[doc = "Field `CMPCHA02` reader - Compare Window A Channel AN102 Select"]
pub type CMPCHA02_R = crate::BitReader<CMPCHA02_A>;
#[doc = "Compare Window A Channel AN102 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA02_A {
    #[doc = "0: Disable compare function for AN102"]
    _0 = 0,
    #[doc = "1: Enable compare function for AN102"]
    _1 = 1,
}
impl From<CMPCHA02_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA02_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA02_A {
        match self.bits {
            false => CMPCHA02_A::_0,
            true => CMPCHA02_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA02_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA02_A::_1
    }
}
#[doc = "Field `CMPCHA02` writer - Compare Window A Channel AN102 Select"]
pub type CMPCHA02_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA02_A, O>;
impl<'a, const O: u8> CMPCHA02_W<'a, O> {
    #[doc = "Disable compare function for AN102"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA02_A::_0)
    }
    #[doc = "Enable compare function for AN102"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA02_A::_1)
    }
}
#[doc = "Field `CMPCHA03` reader - Compare Window A Channel AN103 Select"]
pub type CMPCHA03_R = crate::BitReader<CMPCHA03_A>;
#[doc = "Compare Window A Channel AN103 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA03_A {
    #[doc = "0: Disable compare function for AN103"]
    _0 = 0,
    #[doc = "1: Enable compare function for AN103"]
    _1 = 1,
}
impl From<CMPCHA03_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA03_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA03_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA03_A {
        match self.bits {
            false => CMPCHA03_A::_0,
            true => CMPCHA03_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA03_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA03_A::_1
    }
}
#[doc = "Field `CMPCHA03` writer - Compare Window A Channel AN103 Select"]
pub type CMPCHA03_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA03_A, O>;
impl<'a, const O: u8> CMPCHA03_W<'a, O> {
    #[doc = "Disable compare function for AN103"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA03_A::_0)
    }
    #[doc = "Enable compare function for AN103"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA03_A::_1)
    }
}
#[doc = "Field `CMPCHA05` reader - Compare Window A Channel AN105 Select"]
pub type CMPCHA05_R = crate::BitReader<CMPCHA05_A>;
#[doc = "Compare Window A Channel AN105 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA05_A {
    #[doc = "0: Disable compare function for AN105"]
    _0 = 0,
    #[doc = "1: Enable compare function for AN105"]
    _1 = 1,
}
impl From<CMPCHA05_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA05_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA05_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA05_A {
        match self.bits {
            false => CMPCHA05_A::_0,
            true => CMPCHA05_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA05_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA05_A::_1
    }
}
#[doc = "Field `CMPCHA05` writer - Compare Window A Channel AN105 Select"]
pub type CMPCHA05_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR0_SPEC, CMPCHA05_A, O>;
impl<'a, const O: u8> CMPCHA05_W<'a, O> {
    #[doc = "Disable compare function for AN105"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA05_A::_0)
    }
    #[doc = "Enable compare function for AN105"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA05_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Compare Window A Channel AN100 Select"]
    #[inline(always)]
    pub fn cmpcha00(&self) -> CMPCHA00_R {
        CMPCHA00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare Window A Channel AN101 Select"]
    #[inline(always)]
    pub fn cmpcha01(&self) -> CMPCHA01_R {
        CMPCHA01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare Window A Channel AN102 Select"]
    #[inline(always)]
    pub fn cmpcha02(&self) -> CMPCHA02_R {
        CMPCHA02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare Window A Channel AN103 Select"]
    #[inline(always)]
    pub fn cmpcha03(&self) -> CMPCHA03_R {
        CMPCHA03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Compare Window A Channel AN105 Select"]
    #[inline(always)]
    pub fn cmpcha05(&self) -> CMPCHA05_R {
        CMPCHA05_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare Window A Channel AN100 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha00(&mut self) -> CMPCHA00_W<0> {
        CMPCHA00_W::new(self)
    }
    #[doc = "Bit 1 - Compare Window A Channel AN101 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha01(&mut self) -> CMPCHA01_W<1> {
        CMPCHA01_W::new(self)
    }
    #[doc = "Bit 2 - Compare Window A Channel AN102 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha02(&mut self) -> CMPCHA02_W<2> {
        CMPCHA02_W::new(self)
    }
    #[doc = "Bit 3 - Compare Window A Channel AN103 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha03(&mut self) -> CMPCHA03_W<3> {
        CMPCHA03_W::new(self)
    }
    #[doc = "Bit 5 - Compare Window A Channel AN105 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha05(&mut self) -> CMPCHA05_W<5> {
        CMPCHA05_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Compare Function Window A Channel Select Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpansr0](index.html) module"]
pub struct ADCMPANSR0_SPEC;
impl crate::RegisterSpec for ADCMPANSR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcmpansr0::R](R) reader structure"]
impl crate::Readable for ADCMPANSR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmpansr0::W](W) writer structure"]
impl crate::Writable for ADCMPANSR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPANSR0 to value 0"]
impl crate::Resettable for ADCMPANSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
