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
#[doc = "Field `CMPCHA16` reader - AN016 Select"]
pub type CMPCHA16_R = crate::BitReader<CMPCHA16_A>;
#[doc = "AN016 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA16_A {
    #[doc = "0: Compare function disabled for AN016"]
    _0 = 0,
    #[doc = "1: Compare function enabled for AN016"]
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
#[doc = "Field `CMPCHA16` writer - AN016 Select"]
pub type CMPCHA16_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR1_SPEC, CMPCHA16_A, O>;
impl<'a, const O: u8> CMPCHA16_W<'a, O> {
    #[doc = "Compare function disabled for AN016"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA16_A::_0)
    }
    #[doc = "Compare function enabled for AN016"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA16_A::_1)
    }
}
#[doc = "Field `CMPCHA17` reader - AN017 Select"]
pub type CMPCHA17_R = crate::BitReader<CMPCHA17_A>;
#[doc = "AN017 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA17_A {
    #[doc = "0: Compare function disabled for AN017"]
    _0 = 0,
    #[doc = "1: Compare function enabled for AN017"]
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
#[doc = "Field `CMPCHA17` writer - AN017 Select"]
pub type CMPCHA17_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR1_SPEC, CMPCHA17_A, O>;
impl<'a, const O: u8> CMPCHA17_W<'a, O> {
    #[doc = "Compare function disabled for AN017"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA17_A::_0)
    }
    #[doc = "Compare function enabled for AN017"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA17_A::_1)
    }
}
#[doc = "Field `CMPCHA18` reader - AN018 Select"]
pub type CMPCHA18_R = crate::BitReader<CMPCHA18_A>;
#[doc = "AN018 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA18_A {
    #[doc = "0: Compare function disabled for AN018"]
    _0 = 0,
    #[doc = "1: Compare function enabled for AN018"]
    _1 = 1,
}
impl From<CMPCHA18_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA18_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA18_A {
        match self.bits {
            false => CMPCHA18_A::_0,
            true => CMPCHA18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA18_A::_1
    }
}
#[doc = "Field `CMPCHA18` writer - AN018 Select"]
pub type CMPCHA18_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR1_SPEC, CMPCHA18_A, O>;
impl<'a, const O: u8> CMPCHA18_W<'a, O> {
    #[doc = "Compare function disabled for AN018"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA18_A::_0)
    }
    #[doc = "Compare function enabled for AN018"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA18_A::_1)
    }
}
#[doc = "Field `CMPCHA19` reader - AN019 Select"]
pub type CMPCHA19_R = crate::BitReader<CMPCHA19_A>;
#[doc = "AN019 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA19_A {
    #[doc = "0: Compare function disabled for AN019"]
    _0 = 0,
    #[doc = "1: Compare function enabled for AN019"]
    _1 = 1,
}
impl From<CMPCHA19_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA19_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA19_A {
        match self.bits {
            false => CMPCHA19_A::_0,
            true => CMPCHA19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA19_A::_1
    }
}
#[doc = "Field `CMPCHA19` writer - AN019 Select"]
pub type CMPCHA19_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR1_SPEC, CMPCHA19_A, O>;
impl<'a, const O: u8> CMPCHA19_W<'a, O> {
    #[doc = "Compare function disabled for AN019"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA19_A::_0)
    }
    #[doc = "Compare function enabled for AN019"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA19_A::_1)
    }
}
#[doc = "Field `CMPCHA20` reader - AN020 Select"]
pub type CMPCHA20_R = crate::BitReader<CMPCHA20_A>;
#[doc = "AN020 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA20_A {
    #[doc = "0: Compare function disabled for AN020"]
    _0 = 0,
    #[doc = "1: Compare function enabled for AN020"]
    _1 = 1,
}
impl From<CMPCHA20_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA20_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA20_A {
        match self.bits {
            false => CMPCHA20_A::_0,
            true => CMPCHA20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA20_A::_1
    }
}
#[doc = "Field `CMPCHA20` writer - AN020 Select"]
pub type CMPCHA20_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR1_SPEC, CMPCHA20_A, O>;
impl<'a, const O: u8> CMPCHA20_W<'a, O> {
    #[doc = "Compare function disabled for AN020"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA20_A::_0)
    }
    #[doc = "Compare function enabled for AN020"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA20_A::_1)
    }
}
#[doc = "Field `CMPCHA21` reader - AN021 Select"]
pub type CMPCHA21_R = crate::BitReader<CMPCHA21_A>;
#[doc = "AN021 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA21_A {
    #[doc = "0: Compare function disabled for AN021"]
    _0 = 0,
    #[doc = "1: Compare function enabled for AN021"]
    _1 = 1,
}
impl From<CMPCHA21_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA21_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA21_A {
        match self.bits {
            false => CMPCHA21_A::_0,
            true => CMPCHA21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA21_A::_1
    }
}
#[doc = "Field `CMPCHA21` writer - AN021 Select"]
pub type CMPCHA21_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR1_SPEC, CMPCHA21_A, O>;
impl<'a, const O: u8> CMPCHA21_W<'a, O> {
    #[doc = "Compare function disabled for AN021"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA21_A::_0)
    }
    #[doc = "Compare function enabled for AN021"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA21_A::_1)
    }
}
#[doc = "Field `CMPCHA22` reader - AN022 Select"]
pub type CMPCHA22_R = crate::BitReader<CMPCHA22_A>;
#[doc = "AN022 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA22_A {
    #[doc = "0: Compare function disabled for AN022"]
    _0 = 0,
    #[doc = "1: Compare function enabled for AN022"]
    _1 = 1,
}
impl From<CMPCHA22_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA22_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA22_A {
        match self.bits {
            false => CMPCHA22_A::_0,
            true => CMPCHA22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA22_A::_1
    }
}
#[doc = "Field `CMPCHA22` writer - AN022 Select"]
pub type CMPCHA22_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR1_SPEC, CMPCHA22_A, O>;
impl<'a, const O: u8> CMPCHA22_W<'a, O> {
    #[doc = "Compare function disabled for AN022"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA22_A::_0)
    }
    #[doc = "Compare function enabled for AN022"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA22_A::_1)
    }
}
#[doc = "Field `CMPCHA23` reader - AN023 Select"]
pub type CMPCHA23_R = crate::BitReader<CMPCHA23_A>;
#[doc = "AN023 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA23_A {
    #[doc = "0: Compare function disabled for AN023"]
    _0 = 0,
    #[doc = "1: Compare function enabled for AN023"]
    _1 = 1,
}
impl From<CMPCHA23_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA23_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA23_A {
        match self.bits {
            false => CMPCHA23_A::_0,
            true => CMPCHA23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA23_A::_1
    }
}
#[doc = "Field `CMPCHA23` writer - AN023 Select"]
pub type CMPCHA23_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR1_SPEC, CMPCHA23_A, O>;
impl<'a, const O: u8> CMPCHA23_W<'a, O> {
    #[doc = "Compare function disabled for AN023"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA23_A::_0)
    }
    #[doc = "Compare function enabled for AN023"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA23_A::_1)
    }
}
#[doc = "Field `CMPCHA24` reader - AN024 Select"]
pub type CMPCHA24_R = crate::BitReader<CMPCHA24_A>;
#[doc = "AN024 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA24_A {
    #[doc = "0: Compare function disabled for AN024"]
    _0 = 0,
    #[doc = "1: Compare function enabled for AN024"]
    _1 = 1,
}
impl From<CMPCHA24_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA24_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCHA24_A {
        match self.bits {
            false => CMPCHA24_A::_0,
            true => CMPCHA24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA24_A::_1
    }
}
#[doc = "Field `CMPCHA24` writer - AN024 Select"]
pub type CMPCHA24_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPANSR1_SPEC, CMPCHA24_A, O>;
impl<'a, const O: u8> CMPCHA24_W<'a, O> {
    #[doc = "Compare function disabled for AN024"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHA24_A::_0)
    }
    #[doc = "Compare function enabled for AN024"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHA24_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AN016 Select"]
    #[inline(always)]
    pub fn cmpcha16(&self) -> CMPCHA16_R {
        CMPCHA16_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AN017 Select"]
    #[inline(always)]
    pub fn cmpcha17(&self) -> CMPCHA17_R {
        CMPCHA17_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AN018 Select"]
    #[inline(always)]
    pub fn cmpcha18(&self) -> CMPCHA18_R {
        CMPCHA18_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AN019 Select"]
    #[inline(always)]
    pub fn cmpcha19(&self) -> CMPCHA19_R {
        CMPCHA19_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AN020 Select"]
    #[inline(always)]
    pub fn cmpcha20(&self) -> CMPCHA20_R {
        CMPCHA20_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AN021 Select"]
    #[inline(always)]
    pub fn cmpcha21(&self) -> CMPCHA21_R {
        CMPCHA21_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AN022 Select"]
    #[inline(always)]
    pub fn cmpcha22(&self) -> CMPCHA22_R {
        CMPCHA22_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AN023 Select"]
    #[inline(always)]
    pub fn cmpcha23(&self) -> CMPCHA23_R {
        CMPCHA23_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AN024 Select"]
    #[inline(always)]
    pub fn cmpcha24(&self) -> CMPCHA24_R {
        CMPCHA24_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AN016 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha16(&mut self) -> CMPCHA16_W<0> {
        CMPCHA16_W::new(self)
    }
    #[doc = "Bit 1 - AN017 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha17(&mut self) -> CMPCHA17_W<1> {
        CMPCHA17_W::new(self)
    }
    #[doc = "Bit 2 - AN018 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha18(&mut self) -> CMPCHA18_W<2> {
        CMPCHA18_W::new(self)
    }
    #[doc = "Bit 3 - AN019 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha19(&mut self) -> CMPCHA19_W<3> {
        CMPCHA19_W::new(self)
    }
    #[doc = "Bit 4 - AN020 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha20(&mut self) -> CMPCHA20_W<4> {
        CMPCHA20_W::new(self)
    }
    #[doc = "Bit 5 - AN021 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha21(&mut self) -> CMPCHA21_W<5> {
        CMPCHA21_W::new(self)
    }
    #[doc = "Bit 6 - AN022 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha22(&mut self) -> CMPCHA22_W<6> {
        CMPCHA22_W::new(self)
    }
    #[doc = "Bit 7 - AN023 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha23(&mut self) -> CMPCHA23_W<7> {
        CMPCHA23_W::new(self)
    }
    #[doc = "Bit 8 - AN024 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha24(&mut self) -> CMPCHA24_W<8> {
        CMPCHA24_W::new(self)
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
