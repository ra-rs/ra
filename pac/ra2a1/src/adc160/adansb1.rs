#[doc = "Register `ADANSB1` reader"]
pub struct R(crate::R<ADANSB1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADANSB1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADANSB1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADANSB1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADANSB1` writer"]
pub struct W(crate::W<ADANSB1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADANSB1_SPEC>;
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
impl From<crate::W<ADANSB1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADANSB1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANSB16` reader - AN016 Select"]
pub type ANSB16_R = crate::BitReader<ANSB16_A>;
#[doc = "AN016 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB16_A {
    #[doc = "0: AN016 is not selected"]
    _0 = 0,
    #[doc = "1: AN016 is selected"]
    _1 = 1,
}
impl From<ANSB16_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB16_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB16_A {
        match self.bits {
            false => ANSB16_A::_0,
            true => ANSB16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB16_A::_1
    }
}
#[doc = "Field `ANSB16` writer - AN016 Select"]
pub type ANSB16_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB16_A, O>;
impl<'a, const O: u8> ANSB16_W<'a, O> {
    #[doc = "AN016 is not selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB16_A::_0)
    }
    #[doc = "AN016 is selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB16_A::_1)
    }
}
#[doc = "Field `ANSB17` reader - AN017 Select"]
pub type ANSB17_R = crate::BitReader<ANSB17_A>;
#[doc = "AN017 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB17_A {
    #[doc = "0: AN017 is not selected"]
    _0 = 0,
    #[doc = "1: AN017 is selected"]
    _1 = 1,
}
impl From<ANSB17_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB17_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB17_A {
        match self.bits {
            false => ANSB17_A::_0,
            true => ANSB17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB17_A::_1
    }
}
#[doc = "Field `ANSB17` writer - AN017 Select"]
pub type ANSB17_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB17_A, O>;
impl<'a, const O: u8> ANSB17_W<'a, O> {
    #[doc = "AN017 is not selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB17_A::_0)
    }
    #[doc = "AN017 is selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB17_A::_1)
    }
}
#[doc = "Field `ANSB18` reader - AN018 Select"]
pub type ANSB18_R = crate::BitReader<ANSB18_A>;
#[doc = "AN018 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB18_A {
    #[doc = "0: AN018 is not selected"]
    _0 = 0,
    #[doc = "1: AN018 is selected"]
    _1 = 1,
}
impl From<ANSB18_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB18_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB18_A {
        match self.bits {
            false => ANSB18_A::_0,
            true => ANSB18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB18_A::_1
    }
}
#[doc = "Field `ANSB18` writer - AN018 Select"]
pub type ANSB18_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB18_A, O>;
impl<'a, const O: u8> ANSB18_W<'a, O> {
    #[doc = "AN018 is not selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB18_A::_0)
    }
    #[doc = "AN018 is selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB18_A::_1)
    }
}
#[doc = "Field `ANSB19` reader - AN019 Select"]
pub type ANSB19_R = crate::BitReader<ANSB19_A>;
#[doc = "AN019 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB19_A {
    #[doc = "0: AN019 is not selected"]
    _0 = 0,
    #[doc = "1: AN019 is selected"]
    _1 = 1,
}
impl From<ANSB19_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB19_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB19_A {
        match self.bits {
            false => ANSB19_A::_0,
            true => ANSB19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB19_A::_1
    }
}
#[doc = "Field `ANSB19` writer - AN019 Select"]
pub type ANSB19_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB19_A, O>;
impl<'a, const O: u8> ANSB19_W<'a, O> {
    #[doc = "AN019 is not selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB19_A::_0)
    }
    #[doc = "AN019 is selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB19_A::_1)
    }
}
#[doc = "Field `ANSB20` reader - AN020 Select"]
pub type ANSB20_R = crate::BitReader<ANSB20_A>;
#[doc = "AN020 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB20_A {
    #[doc = "0: AN020 is not selected"]
    _0 = 0,
    #[doc = "1: AN020 is selected"]
    _1 = 1,
}
impl From<ANSB20_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB20_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB20_A {
        match self.bits {
            false => ANSB20_A::_0,
            true => ANSB20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB20_A::_1
    }
}
#[doc = "Field `ANSB20` writer - AN020 Select"]
pub type ANSB20_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB20_A, O>;
impl<'a, const O: u8> ANSB20_W<'a, O> {
    #[doc = "AN020 is not selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB20_A::_0)
    }
    #[doc = "AN020 is selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB20_A::_1)
    }
}
#[doc = "Field `ANSB21` reader - AN021 Select"]
pub type ANSB21_R = crate::BitReader<ANSB21_A>;
#[doc = "AN021 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB21_A {
    #[doc = "0: AN021 is not selected"]
    _0 = 0,
    #[doc = "1: AN021 is selected"]
    _1 = 1,
}
impl From<ANSB21_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB21_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB21_A {
        match self.bits {
            false => ANSB21_A::_0,
            true => ANSB21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB21_A::_1
    }
}
#[doc = "Field `ANSB21` writer - AN021 Select"]
pub type ANSB21_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB21_A, O>;
impl<'a, const O: u8> ANSB21_W<'a, O> {
    #[doc = "AN021 is not selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB21_A::_0)
    }
    #[doc = "AN021 is selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB21_A::_1)
    }
}
#[doc = "Field `ANSB22` reader - AN022 Select"]
pub type ANSB22_R = crate::BitReader<ANSB22_A>;
#[doc = "AN022 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB22_A {
    #[doc = "0: AN022 is not selected"]
    _0 = 0,
    #[doc = "1: AN022 is selected"]
    _1 = 1,
}
impl From<ANSB22_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB22_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB22_A {
        match self.bits {
            false => ANSB22_A::_0,
            true => ANSB22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB22_A::_1
    }
}
#[doc = "Field `ANSB22` writer - AN022 Select"]
pub type ANSB22_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB22_A, O>;
impl<'a, const O: u8> ANSB22_W<'a, O> {
    #[doc = "AN022 is not selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB22_A::_0)
    }
    #[doc = "AN022 is selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB22_A::_1)
    }
}
#[doc = "Field `ANSB23` reader - AN023 Select"]
pub type ANSB23_R = crate::BitReader<ANSB23_A>;
#[doc = "AN023 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB23_A {
    #[doc = "0: AN023 is not selected"]
    _0 = 0,
    #[doc = "1: AN023 is selected"]
    _1 = 1,
}
impl From<ANSB23_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB23_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB23_A {
        match self.bits {
            false => ANSB23_A::_0,
            true => ANSB23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB23_A::_1
    }
}
#[doc = "Field `ANSB23` writer - AN023 Select"]
pub type ANSB23_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB23_A, O>;
impl<'a, const O: u8> ANSB23_W<'a, O> {
    #[doc = "AN023 is not selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB23_A::_0)
    }
    #[doc = "AN023 is selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB23_A::_1)
    }
}
#[doc = "Field `ANSB24` reader - AN024 Select"]
pub type ANSB24_R = crate::BitReader<ANSB24_A>;
#[doc = "AN024 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB24_A {
    #[doc = "0: AN024 is not selected"]
    _0 = 0,
    #[doc = "1: AN024 is selected"]
    _1 = 1,
}
impl From<ANSB24_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB24_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB24_A {
        match self.bits {
            false => ANSB24_A::_0,
            true => ANSB24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB24_A::_1
    }
}
#[doc = "Field `ANSB24` writer - AN024 Select"]
pub type ANSB24_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB1_SPEC, ANSB24_A, O>;
impl<'a, const O: u8> ANSB24_W<'a, O> {
    #[doc = "AN024 is not selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB24_A::_0)
    }
    #[doc = "AN024 is selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB24_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AN016 Select"]
    #[inline(always)]
    pub fn ansb16(&self) -> ANSB16_R {
        ANSB16_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AN017 Select"]
    #[inline(always)]
    pub fn ansb17(&self) -> ANSB17_R {
        ANSB17_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AN018 Select"]
    #[inline(always)]
    pub fn ansb18(&self) -> ANSB18_R {
        ANSB18_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AN019 Select"]
    #[inline(always)]
    pub fn ansb19(&self) -> ANSB19_R {
        ANSB19_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AN020 Select"]
    #[inline(always)]
    pub fn ansb20(&self) -> ANSB20_R {
        ANSB20_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AN021 Select"]
    #[inline(always)]
    pub fn ansb21(&self) -> ANSB21_R {
        ANSB21_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AN022 Select"]
    #[inline(always)]
    pub fn ansb22(&self) -> ANSB22_R {
        ANSB22_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AN023 Select"]
    #[inline(always)]
    pub fn ansb23(&self) -> ANSB23_R {
        ANSB23_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AN024 Select"]
    #[inline(always)]
    pub fn ansb24(&self) -> ANSB24_R {
        ANSB24_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AN016 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb16(&mut self) -> ANSB16_W<0> {
        ANSB16_W::new(self)
    }
    #[doc = "Bit 1 - AN017 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb17(&mut self) -> ANSB17_W<1> {
        ANSB17_W::new(self)
    }
    #[doc = "Bit 2 - AN018 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb18(&mut self) -> ANSB18_W<2> {
        ANSB18_W::new(self)
    }
    #[doc = "Bit 3 - AN019 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb19(&mut self) -> ANSB19_W<3> {
        ANSB19_W::new(self)
    }
    #[doc = "Bit 4 - AN020 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb20(&mut self) -> ANSB20_W<4> {
        ANSB20_W::new(self)
    }
    #[doc = "Bit 5 - AN021 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb21(&mut self) -> ANSB21_W<5> {
        ANSB21_W::new(self)
    }
    #[doc = "Bit 6 - AN022 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb22(&mut self) -> ANSB22_W<6> {
        ANSB22_W::new(self)
    }
    #[doc = "Bit 7 - AN023 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb23(&mut self) -> ANSB23_W<7> {
        ANSB23_W::new(self)
    }
    #[doc = "Bit 8 - AN024 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb24(&mut self) -> ANSB24_W<8> {
        ANSB24_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Channel Select Register B1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adansb1](index.html) module"]
pub struct ADANSB1_SPEC;
impl crate::RegisterSpec for ADANSB1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adansb1::R](R) reader structure"]
impl crate::Readable for ADANSB1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adansb1::W](W) writer structure"]
impl crate::Writable for ADANSB1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADANSB1 to value 0"]
impl crate::Resettable for ADANSB1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
