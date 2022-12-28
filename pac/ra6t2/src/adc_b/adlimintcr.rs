#[doc = "Register `ADLIMINTCR` reader"]
pub struct R(crate::R<ADLIMINTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADLIMINTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADLIMINTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADLIMINTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADLIMINTCR` writer"]
pub struct W(crate::W<ADLIMINTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADLIMINTCR_SPEC>;
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
impl From<crate::W<ADLIMINTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADLIMINTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LIMIE0` reader - Limiter Clip Interrupt n Enable bit"]
pub type LIMIE0_R = crate::BitReader<LIMIE0_A>;
#[doc = "Limiter Clip Interrupt n Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMIE0_A {
    #[doc = "0: Disable the limiter clip interrupt n"]
    _0 = 0,
    #[doc = "1: Enable the limiter clip interrupt n"]
    _1 = 1,
}
impl From<LIMIE0_A> for bool {
    #[inline(always)]
    fn from(variant: LIMIE0_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMIE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMIE0_A {
        match self.bits {
            false => LIMIE0_A::_0,
            true => LIMIE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMIE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMIE0_A::_1
    }
}
#[doc = "Field `LIMIE0` writer - Limiter Clip Interrupt n Enable bit"]
pub type LIMIE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMINTCR_SPEC, LIMIE0_A, O>;
impl<'a, const O: u8> LIMIE0_W<'a, O> {
    #[doc = "Disable the limiter clip interrupt n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMIE0_A::_0)
    }
    #[doc = "Enable the limiter clip interrupt n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMIE0_A::_1)
    }
}
#[doc = "Field `LIMIE1` reader - Limiter Clip Interrupt n Enable bit"]
pub type LIMIE1_R = crate::BitReader<LIMIE1_A>;
#[doc = "Limiter Clip Interrupt n Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMIE1_A {
    #[doc = "0: Disable the limiter clip interrupt n"]
    _0 = 0,
    #[doc = "1: Enable the limiter clip interrupt n"]
    _1 = 1,
}
impl From<LIMIE1_A> for bool {
    #[inline(always)]
    fn from(variant: LIMIE1_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMIE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMIE1_A {
        match self.bits {
            false => LIMIE1_A::_0,
            true => LIMIE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMIE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMIE1_A::_1
    }
}
#[doc = "Field `LIMIE1` writer - Limiter Clip Interrupt n Enable bit"]
pub type LIMIE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMINTCR_SPEC, LIMIE1_A, O>;
impl<'a, const O: u8> LIMIE1_W<'a, O> {
    #[doc = "Disable the limiter clip interrupt n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMIE1_A::_0)
    }
    #[doc = "Enable the limiter clip interrupt n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMIE1_A::_1)
    }
}
#[doc = "Field `LIMIE2` reader - Limiter Clip Interrupt n Enable bit"]
pub type LIMIE2_R = crate::BitReader<LIMIE2_A>;
#[doc = "Limiter Clip Interrupt n Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMIE2_A {
    #[doc = "0: Disable the limiter clip interrupt n"]
    _0 = 0,
    #[doc = "1: Enable the limiter clip interrupt n"]
    _1 = 1,
}
impl From<LIMIE2_A> for bool {
    #[inline(always)]
    fn from(variant: LIMIE2_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMIE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMIE2_A {
        match self.bits {
            false => LIMIE2_A::_0,
            true => LIMIE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMIE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMIE2_A::_1
    }
}
#[doc = "Field `LIMIE2` writer - Limiter Clip Interrupt n Enable bit"]
pub type LIMIE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMINTCR_SPEC, LIMIE2_A, O>;
impl<'a, const O: u8> LIMIE2_W<'a, O> {
    #[doc = "Disable the limiter clip interrupt n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMIE2_A::_0)
    }
    #[doc = "Enable the limiter clip interrupt n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMIE2_A::_1)
    }
}
#[doc = "Field `LIMIE3` reader - Limiter Clip Interrupt n Enable bit"]
pub type LIMIE3_R = crate::BitReader<LIMIE3_A>;
#[doc = "Limiter Clip Interrupt n Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMIE3_A {
    #[doc = "0: Disable the limiter clip interrupt n"]
    _0 = 0,
    #[doc = "1: Enable the limiter clip interrupt n"]
    _1 = 1,
}
impl From<LIMIE3_A> for bool {
    #[inline(always)]
    fn from(variant: LIMIE3_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMIE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMIE3_A {
        match self.bits {
            false => LIMIE3_A::_0,
            true => LIMIE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMIE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMIE3_A::_1
    }
}
#[doc = "Field `LIMIE3` writer - Limiter Clip Interrupt n Enable bit"]
pub type LIMIE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMINTCR_SPEC, LIMIE3_A, O>;
impl<'a, const O: u8> LIMIE3_W<'a, O> {
    #[doc = "Disable the limiter clip interrupt n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMIE3_A::_0)
    }
    #[doc = "Enable the limiter clip interrupt n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMIE3_A::_1)
    }
}
#[doc = "Field `LIMIE4` reader - Limiter Clip Interrupt n Enable bit"]
pub type LIMIE4_R = crate::BitReader<LIMIE4_A>;
#[doc = "Limiter Clip Interrupt n Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMIE4_A {
    #[doc = "0: Disable the limiter clip interrupt n"]
    _0 = 0,
    #[doc = "1: Enable the limiter clip interrupt n"]
    _1 = 1,
}
impl From<LIMIE4_A> for bool {
    #[inline(always)]
    fn from(variant: LIMIE4_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMIE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMIE4_A {
        match self.bits {
            false => LIMIE4_A::_0,
            true => LIMIE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMIE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMIE4_A::_1
    }
}
#[doc = "Field `LIMIE4` writer - Limiter Clip Interrupt n Enable bit"]
pub type LIMIE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMINTCR_SPEC, LIMIE4_A, O>;
impl<'a, const O: u8> LIMIE4_W<'a, O> {
    #[doc = "Disable the limiter clip interrupt n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMIE4_A::_0)
    }
    #[doc = "Enable the limiter clip interrupt n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMIE4_A::_1)
    }
}
#[doc = "Field `LIMIE5` reader - Limiter Clip Interrupt n Enable bit"]
pub type LIMIE5_R = crate::BitReader<LIMIE5_A>;
#[doc = "Limiter Clip Interrupt n Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMIE5_A {
    #[doc = "0: Disable the limiter clip interrupt n"]
    _0 = 0,
    #[doc = "1: Enable the limiter clip interrupt n"]
    _1 = 1,
}
impl From<LIMIE5_A> for bool {
    #[inline(always)]
    fn from(variant: LIMIE5_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMIE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMIE5_A {
        match self.bits {
            false => LIMIE5_A::_0,
            true => LIMIE5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMIE5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMIE5_A::_1
    }
}
#[doc = "Field `LIMIE5` writer - Limiter Clip Interrupt n Enable bit"]
pub type LIMIE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMINTCR_SPEC, LIMIE5_A, O>;
impl<'a, const O: u8> LIMIE5_W<'a, O> {
    #[doc = "Disable the limiter clip interrupt n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMIE5_A::_0)
    }
    #[doc = "Enable the limiter clip interrupt n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMIE5_A::_1)
    }
}
#[doc = "Field `LIMIE6` reader - Limiter Clip Interrupt n Enable bit"]
pub type LIMIE6_R = crate::BitReader<LIMIE6_A>;
#[doc = "Limiter Clip Interrupt n Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMIE6_A {
    #[doc = "0: Disable the limiter clip interrupt n"]
    _0 = 0,
    #[doc = "1: Enable the limiter clip interrupt n"]
    _1 = 1,
}
impl From<LIMIE6_A> for bool {
    #[inline(always)]
    fn from(variant: LIMIE6_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMIE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMIE6_A {
        match self.bits {
            false => LIMIE6_A::_0,
            true => LIMIE6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMIE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMIE6_A::_1
    }
}
#[doc = "Field `LIMIE6` writer - Limiter Clip Interrupt n Enable bit"]
pub type LIMIE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMINTCR_SPEC, LIMIE6_A, O>;
impl<'a, const O: u8> LIMIE6_W<'a, O> {
    #[doc = "Disable the limiter clip interrupt n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMIE6_A::_0)
    }
    #[doc = "Enable the limiter clip interrupt n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMIE6_A::_1)
    }
}
#[doc = "Field `LIMIE7` reader - Limiter Clip Interrupt n Enable bit"]
pub type LIMIE7_R = crate::BitReader<LIMIE7_A>;
#[doc = "Limiter Clip Interrupt n Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMIE7_A {
    #[doc = "0: Disable the limiter clip interrupt n"]
    _0 = 0,
    #[doc = "1: Enable the limiter clip interrupt n"]
    _1 = 1,
}
impl From<LIMIE7_A> for bool {
    #[inline(always)]
    fn from(variant: LIMIE7_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMIE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMIE7_A {
        match self.bits {
            false => LIMIE7_A::_0,
            true => LIMIE7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMIE7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMIE7_A::_1
    }
}
#[doc = "Field `LIMIE7` writer - Limiter Clip Interrupt n Enable bit"]
pub type LIMIE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMINTCR_SPEC, LIMIE7_A, O>;
impl<'a, const O: u8> LIMIE7_W<'a, O> {
    #[doc = "Disable the limiter clip interrupt n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMIE7_A::_0)
    }
    #[doc = "Enable the limiter clip interrupt n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMIE7_A::_1)
    }
}
#[doc = "Field `LIMIE8` reader - Limiter Clip Interrupt n Enable bit"]
pub type LIMIE8_R = crate::BitReader<LIMIE8_A>;
#[doc = "Limiter Clip Interrupt n Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMIE8_A {
    #[doc = "0: Disable the limiter clip interrupt n"]
    _0 = 0,
    #[doc = "1: Enable the limiter clip interrupt n"]
    _1 = 1,
}
impl From<LIMIE8_A> for bool {
    #[inline(always)]
    fn from(variant: LIMIE8_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMIE8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMIE8_A {
        match self.bits {
            false => LIMIE8_A::_0,
            true => LIMIE8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMIE8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMIE8_A::_1
    }
}
#[doc = "Field `LIMIE8` writer - Limiter Clip Interrupt n Enable bit"]
pub type LIMIE8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMINTCR_SPEC, LIMIE8_A, O>;
impl<'a, const O: u8> LIMIE8_W<'a, O> {
    #[doc = "Disable the limiter clip interrupt n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMIE8_A::_0)
    }
    #[doc = "Enable the limiter clip interrupt n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMIE8_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Limiter Clip Interrupt n Enable bit"]
    #[inline(always)]
    pub fn limie0(&self) -> LIMIE0_R {
        LIMIE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Limiter Clip Interrupt n Enable bit"]
    #[inline(always)]
    pub fn limie1(&self) -> LIMIE1_R {
        LIMIE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Limiter Clip Interrupt n Enable bit"]
    #[inline(always)]
    pub fn limie2(&self) -> LIMIE2_R {
        LIMIE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Limiter Clip Interrupt n Enable bit"]
    #[inline(always)]
    pub fn limie3(&self) -> LIMIE3_R {
        LIMIE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Limiter Clip Interrupt n Enable bit"]
    #[inline(always)]
    pub fn limie4(&self) -> LIMIE4_R {
        LIMIE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Limiter Clip Interrupt n Enable bit"]
    #[inline(always)]
    pub fn limie5(&self) -> LIMIE5_R {
        LIMIE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Limiter Clip Interrupt n Enable bit"]
    #[inline(always)]
    pub fn limie6(&self) -> LIMIE6_R {
        LIMIE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Limiter Clip Interrupt n Enable bit"]
    #[inline(always)]
    pub fn limie7(&self) -> LIMIE7_R {
        LIMIE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Limiter Clip Interrupt n Enable bit"]
    #[inline(always)]
    pub fn limie8(&self) -> LIMIE8_R {
        LIMIE8_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Limiter Clip Interrupt n Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn limie0(&mut self) -> LIMIE0_W<0> {
        LIMIE0_W::new(self)
    }
    #[doc = "Bit 1 - Limiter Clip Interrupt n Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn limie1(&mut self) -> LIMIE1_W<1> {
        LIMIE1_W::new(self)
    }
    #[doc = "Bit 2 - Limiter Clip Interrupt n Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn limie2(&mut self) -> LIMIE2_W<2> {
        LIMIE2_W::new(self)
    }
    #[doc = "Bit 3 - Limiter Clip Interrupt n Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn limie3(&mut self) -> LIMIE3_W<3> {
        LIMIE3_W::new(self)
    }
    #[doc = "Bit 4 - Limiter Clip Interrupt n Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn limie4(&mut self) -> LIMIE4_W<4> {
        LIMIE4_W::new(self)
    }
    #[doc = "Bit 5 - Limiter Clip Interrupt n Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn limie5(&mut self) -> LIMIE5_W<5> {
        LIMIE5_W::new(self)
    }
    #[doc = "Bit 6 - Limiter Clip Interrupt n Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn limie6(&mut self) -> LIMIE6_W<6> {
        LIMIE6_W::new(self)
    }
    #[doc = "Bit 7 - Limiter Clip Interrupt n Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn limie7(&mut self) -> LIMIE7_W<7> {
        LIMIE7_W::new(self)
    }
    #[doc = "Bit 8 - Limiter Clip Interrupt n Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn limie8(&mut self) -> LIMIE8_W<8> {
        LIMIE8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Limiter Clip Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adlimintcr](index.html) module"]
pub struct ADLIMINTCR_SPEC;
impl crate::RegisterSpec for ADLIMINTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adlimintcr::R](R) reader structure"]
impl crate::Readable for ADLIMINTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adlimintcr::W](W) writer structure"]
impl crate::Writable for ADLIMINTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADLIMINTCR to value 0"]
impl crate::Resettable for ADLIMINTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
