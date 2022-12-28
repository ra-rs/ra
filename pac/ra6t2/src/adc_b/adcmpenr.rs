#[doc = "Register `ADCMPENR` reader"]
pub struct R(crate::R<ADCMPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPENR` writer"]
pub struct W(crate::W<ADCMPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPENR_SPEC>;
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
impl From<crate::W<ADCMPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPEN0` reader - Compare Match n Enable"]
pub type CMPEN0_R = crate::BitReader<CMPEN0_A>;
#[doc = "Compare Match n Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPEN0_A {
    #[doc = "0: Disable the compare match n"]
    _0 = 0,
    #[doc = "1: Enable the compare match n"]
    _1 = 1,
}
impl From<CMPEN0_A> for bool {
    #[inline(always)]
    fn from(variant: CMPEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPEN0_A {
        match self.bits {
            false => CMPEN0_A::_0,
            true => CMPEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPEN0_A::_1
    }
}
#[doc = "Field `CMPEN0` writer - Compare Match n Enable"]
pub type CMPEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPENR_SPEC, CMPEN0_A, O>;
impl<'a, const O: u8> CMPEN0_W<'a, O> {
    #[doc = "Disable the compare match n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPEN0_A::_0)
    }
    #[doc = "Enable the compare match n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPEN0_A::_1)
    }
}
#[doc = "Field `CMPEN1` reader - Compare Match n Enable"]
pub type CMPEN1_R = crate::BitReader<CMPEN1_A>;
#[doc = "Compare Match n Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPEN1_A {
    #[doc = "0: Disable the compare match n"]
    _0 = 0,
    #[doc = "1: Enable the compare match n"]
    _1 = 1,
}
impl From<CMPEN1_A> for bool {
    #[inline(always)]
    fn from(variant: CMPEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPEN1_A {
        match self.bits {
            false => CMPEN1_A::_0,
            true => CMPEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPEN1_A::_1
    }
}
#[doc = "Field `CMPEN1` writer - Compare Match n Enable"]
pub type CMPEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPENR_SPEC, CMPEN1_A, O>;
impl<'a, const O: u8> CMPEN1_W<'a, O> {
    #[doc = "Disable the compare match n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPEN1_A::_0)
    }
    #[doc = "Enable the compare match n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPEN1_A::_1)
    }
}
#[doc = "Field `CMPEN2` reader - Compare Match n Enable"]
pub type CMPEN2_R = crate::BitReader<CMPEN2_A>;
#[doc = "Compare Match n Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPEN2_A {
    #[doc = "0: Disable the compare match n"]
    _0 = 0,
    #[doc = "1: Enable the compare match n"]
    _1 = 1,
}
impl From<CMPEN2_A> for bool {
    #[inline(always)]
    fn from(variant: CMPEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPEN2_A {
        match self.bits {
            false => CMPEN2_A::_0,
            true => CMPEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPEN2_A::_1
    }
}
#[doc = "Field `CMPEN2` writer - Compare Match n Enable"]
pub type CMPEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPENR_SPEC, CMPEN2_A, O>;
impl<'a, const O: u8> CMPEN2_W<'a, O> {
    #[doc = "Disable the compare match n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPEN2_A::_0)
    }
    #[doc = "Enable the compare match n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPEN2_A::_1)
    }
}
#[doc = "Field `CMPEN3` reader - Compare Match n Enable"]
pub type CMPEN3_R = crate::BitReader<CMPEN3_A>;
#[doc = "Compare Match n Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPEN3_A {
    #[doc = "0: Disable the compare match n"]
    _0 = 0,
    #[doc = "1: Enable the compare match n"]
    _1 = 1,
}
impl From<CMPEN3_A> for bool {
    #[inline(always)]
    fn from(variant: CMPEN3_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPEN3_A {
        match self.bits {
            false => CMPEN3_A::_0,
            true => CMPEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPEN3_A::_1
    }
}
#[doc = "Field `CMPEN3` writer - Compare Match n Enable"]
pub type CMPEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPENR_SPEC, CMPEN3_A, O>;
impl<'a, const O: u8> CMPEN3_W<'a, O> {
    #[doc = "Disable the compare match n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPEN3_A::_0)
    }
    #[doc = "Enable the compare match n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPEN3_A::_1)
    }
}
#[doc = "Field `CMPEN4` reader - Compare Match n Enable"]
pub type CMPEN4_R = crate::BitReader<CMPEN4_A>;
#[doc = "Compare Match n Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPEN4_A {
    #[doc = "0: Disable the compare match n"]
    _0 = 0,
    #[doc = "1: Enable the compare match n"]
    _1 = 1,
}
impl From<CMPEN4_A> for bool {
    #[inline(always)]
    fn from(variant: CMPEN4_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPEN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPEN4_A {
        match self.bits {
            false => CMPEN4_A::_0,
            true => CMPEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPEN4_A::_1
    }
}
#[doc = "Field `CMPEN4` writer - Compare Match n Enable"]
pub type CMPEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPENR_SPEC, CMPEN4_A, O>;
impl<'a, const O: u8> CMPEN4_W<'a, O> {
    #[doc = "Disable the compare match n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPEN4_A::_0)
    }
    #[doc = "Enable the compare match n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPEN4_A::_1)
    }
}
#[doc = "Field `CMPEN5` reader - Compare Match n Enable"]
pub type CMPEN5_R = crate::BitReader<CMPEN5_A>;
#[doc = "Compare Match n Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPEN5_A {
    #[doc = "0: Disable the compare match n"]
    _0 = 0,
    #[doc = "1: Enable the compare match n"]
    _1 = 1,
}
impl From<CMPEN5_A> for bool {
    #[inline(always)]
    fn from(variant: CMPEN5_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPEN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPEN5_A {
        match self.bits {
            false => CMPEN5_A::_0,
            true => CMPEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPEN5_A::_1
    }
}
#[doc = "Field `CMPEN5` writer - Compare Match n Enable"]
pub type CMPEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPENR_SPEC, CMPEN5_A, O>;
impl<'a, const O: u8> CMPEN5_W<'a, O> {
    #[doc = "Disable the compare match n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPEN5_A::_0)
    }
    #[doc = "Enable the compare match n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPEN5_A::_1)
    }
}
#[doc = "Field `CMPEN6` reader - Compare Match n Enable"]
pub type CMPEN6_R = crate::BitReader<CMPEN6_A>;
#[doc = "Compare Match n Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPEN6_A {
    #[doc = "0: Disable the compare match n"]
    _0 = 0,
    #[doc = "1: Enable the compare match n"]
    _1 = 1,
}
impl From<CMPEN6_A> for bool {
    #[inline(always)]
    fn from(variant: CMPEN6_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPEN6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPEN6_A {
        match self.bits {
            false => CMPEN6_A::_0,
            true => CMPEN6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPEN6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPEN6_A::_1
    }
}
#[doc = "Field `CMPEN6` writer - Compare Match n Enable"]
pub type CMPEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPENR_SPEC, CMPEN6_A, O>;
impl<'a, const O: u8> CMPEN6_W<'a, O> {
    #[doc = "Disable the compare match n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPEN6_A::_0)
    }
    #[doc = "Enable the compare match n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPEN6_A::_1)
    }
}
#[doc = "Field `CMPEN7` reader - Compare Match n Enable"]
pub type CMPEN7_R = crate::BitReader<CMPEN7_A>;
#[doc = "Compare Match n Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPEN7_A {
    #[doc = "0: Disable the compare match n"]
    _0 = 0,
    #[doc = "1: Enable the compare match n"]
    _1 = 1,
}
impl From<CMPEN7_A> for bool {
    #[inline(always)]
    fn from(variant: CMPEN7_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPEN7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPEN7_A {
        match self.bits {
            false => CMPEN7_A::_0,
            true => CMPEN7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPEN7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPEN7_A::_1
    }
}
#[doc = "Field `CMPEN7` writer - Compare Match n Enable"]
pub type CMPEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPENR_SPEC, CMPEN7_A, O>;
impl<'a, const O: u8> CMPEN7_W<'a, O> {
    #[doc = "Disable the compare match n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPEN7_A::_0)
    }
    #[doc = "Enable the compare match n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPEN7_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Compare Match n Enable"]
    #[inline(always)]
    pub fn cmpen0(&self) -> CMPEN0_R {
        CMPEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare Match n Enable"]
    #[inline(always)]
    pub fn cmpen1(&self) -> CMPEN1_R {
        CMPEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare Match n Enable"]
    #[inline(always)]
    pub fn cmpen2(&self) -> CMPEN2_R {
        CMPEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare Match n Enable"]
    #[inline(always)]
    pub fn cmpen3(&self) -> CMPEN3_R {
        CMPEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare Match n Enable"]
    #[inline(always)]
    pub fn cmpen4(&self) -> CMPEN4_R {
        CMPEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Compare Match n Enable"]
    #[inline(always)]
    pub fn cmpen5(&self) -> CMPEN5_R {
        CMPEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Compare Match n Enable"]
    #[inline(always)]
    pub fn cmpen6(&self) -> CMPEN6_R {
        CMPEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Compare Match n Enable"]
    #[inline(always)]
    pub fn cmpen7(&self) -> CMPEN7_R {
        CMPEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare Match n Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpen0(&mut self) -> CMPEN0_W<0> {
        CMPEN0_W::new(self)
    }
    #[doc = "Bit 1 - Compare Match n Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpen1(&mut self) -> CMPEN1_W<1> {
        CMPEN1_W::new(self)
    }
    #[doc = "Bit 2 - Compare Match n Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpen2(&mut self) -> CMPEN2_W<2> {
        CMPEN2_W::new(self)
    }
    #[doc = "Bit 3 - Compare Match n Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpen3(&mut self) -> CMPEN3_W<3> {
        CMPEN3_W::new(self)
    }
    #[doc = "Bit 4 - Compare Match n Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpen4(&mut self) -> CMPEN4_W<4> {
        CMPEN4_W::new(self)
    }
    #[doc = "Bit 5 - Compare Match n Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpen5(&mut self) -> CMPEN5_W<5> {
        CMPEN5_W::new(self)
    }
    #[doc = "Bit 6 - Compare Match n Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpen6(&mut self) -> CMPEN6_W<6> {
        CMPEN6_W::new(self)
    }
    #[doc = "Bit 7 - Compare Match n Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpen7(&mut self) -> CMPEN7_W<7> {
        CMPEN7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare Match Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpenr](index.html) module"]
pub struct ADCMPENR_SPEC;
impl crate::RegisterSpec for ADCMPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcmpenr::R](R) reader structure"]
impl crate::Readable for ADCMPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmpenr::W](W) writer structure"]
impl crate::Writable for ADCMPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPENR to value 0"]
impl crate::Resettable for ADCMPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
