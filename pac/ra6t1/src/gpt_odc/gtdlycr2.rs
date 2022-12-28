#[doc = "Register `GTDLYCR2` reader"]
pub struct R(crate::R<GTDLYCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTDLYCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTDLYCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTDLYCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTDLYCR2` writer"]
pub struct W(crate::W<GTDLYCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTDLYCR2_SPEC>;
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
impl From<crate::W<GTDLYCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTDLYCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLYBS0` reader - PWM Delay Generation Circuit bypass for channel 0"]
pub type DLYBS0_R = crate::BitReader<DLYBS0_A>;
#[doc = "PWM Delay Generation Circuit bypass for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYBS0_A {
    #[doc = "0: Bypass delay generation circuit of channel 0"]
    _0 = 0,
    #[doc = "1: Do not bypass delay generation circuit of channel 0."]
    _1 = 1,
}
impl From<DLYBS0_A> for bool {
    #[inline(always)]
    fn from(variant: DLYBS0_A) -> Self {
        variant as u8 != 0
    }
}
impl DLYBS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLYBS0_A {
        match self.bits {
            false => DLYBS0_A::_0,
            true => DLYBS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLYBS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLYBS0_A::_1
    }
}
#[doc = "Field `DLYBS0` writer - PWM Delay Generation Circuit bypass for channel 0"]
pub type DLYBS0_W<'a, const O: u8> = crate::BitWriter<'a, u16, GTDLYCR2_SPEC, DLYBS0_A, O>;
impl<'a, const O: u8> DLYBS0_W<'a, O> {
    #[doc = "Bypass delay generation circuit of channel 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DLYBS0_A::_0)
    }
    #[doc = "Do not bypass delay generation circuit of channel 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DLYBS0_A::_1)
    }
}
#[doc = "Field `DLYBS1` reader - PWM Delay Generation Circuit bypass for channel 1"]
pub type DLYBS1_R = crate::BitReader<DLYBS1_A>;
#[doc = "PWM Delay Generation Circuit bypass for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYBS1_A {
    #[doc = "0: Bypass delay generation circuit of channel 1"]
    _0 = 0,
    #[doc = "1: Do not bypass delay generation circuit of channel 1."]
    _1 = 1,
}
impl From<DLYBS1_A> for bool {
    #[inline(always)]
    fn from(variant: DLYBS1_A) -> Self {
        variant as u8 != 0
    }
}
impl DLYBS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLYBS1_A {
        match self.bits {
            false => DLYBS1_A::_0,
            true => DLYBS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLYBS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLYBS1_A::_1
    }
}
#[doc = "Field `DLYBS1` writer - PWM Delay Generation Circuit bypass for channel 1"]
pub type DLYBS1_W<'a, const O: u8> = crate::BitWriter<'a, u16, GTDLYCR2_SPEC, DLYBS1_A, O>;
impl<'a, const O: u8> DLYBS1_W<'a, O> {
    #[doc = "Bypass delay generation circuit of channel 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DLYBS1_A::_0)
    }
    #[doc = "Do not bypass delay generation circuit of channel 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DLYBS1_A::_1)
    }
}
#[doc = "Field `DLYBS2` reader - PWM Delay Generation Circuit bypass for channel 2"]
pub type DLYBS2_R = crate::BitReader<DLYBS2_A>;
#[doc = "PWM Delay Generation Circuit bypass for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYBS2_A {
    #[doc = "0: Bypass delay generation circuit of channel 2"]
    _0 = 0,
    #[doc = "1: Do not bypass delay generation circuit of channel 2."]
    _1 = 1,
}
impl From<DLYBS2_A> for bool {
    #[inline(always)]
    fn from(variant: DLYBS2_A) -> Self {
        variant as u8 != 0
    }
}
impl DLYBS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLYBS2_A {
        match self.bits {
            false => DLYBS2_A::_0,
            true => DLYBS2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLYBS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLYBS2_A::_1
    }
}
#[doc = "Field `DLYBS2` writer - PWM Delay Generation Circuit bypass for channel 2"]
pub type DLYBS2_W<'a, const O: u8> = crate::BitWriter<'a, u16, GTDLYCR2_SPEC, DLYBS2_A, O>;
impl<'a, const O: u8> DLYBS2_W<'a, O> {
    #[doc = "Bypass delay generation circuit of channel 2"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DLYBS2_A::_0)
    }
    #[doc = "Do not bypass delay generation circuit of channel 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DLYBS2_A::_1)
    }
}
#[doc = "Field `DLYBS3` reader - PWM Delay Generation Circuit bypass for channel 3"]
pub type DLYBS3_R = crate::BitReader<DLYBS3_A>;
#[doc = "PWM Delay Generation Circuit bypass for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYBS3_A {
    #[doc = "0: Bypass delay generation circuit of channel 3"]
    _0 = 0,
    #[doc = "1: Do not bypass delay generation circuit of channel 3."]
    _1 = 1,
}
impl From<DLYBS3_A> for bool {
    #[inline(always)]
    fn from(variant: DLYBS3_A) -> Self {
        variant as u8 != 0
    }
}
impl DLYBS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLYBS3_A {
        match self.bits {
            false => DLYBS3_A::_0,
            true => DLYBS3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLYBS3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLYBS3_A::_1
    }
}
#[doc = "Field `DLYBS3` writer - PWM Delay Generation Circuit bypass for channel 3"]
pub type DLYBS3_W<'a, const O: u8> = crate::BitWriter<'a, u16, GTDLYCR2_SPEC, DLYBS3_A, O>;
impl<'a, const O: u8> DLYBS3_W<'a, O> {
    #[doc = "Bypass delay generation circuit of channel 3"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DLYBS3_A::_0)
    }
    #[doc = "Do not bypass delay generation circuit of channel 3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DLYBS3_A::_1)
    }
}
#[doc = "Field `DLYEN0` reader - PWM Delay Generation Circuit enable for channel 0"]
pub type DLYEN0_R = crate::BitReader<DLYEN0_A>;
#[doc = "PWM Delay Generation Circuit enable for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYEN0_A {
    #[doc = "0: Enable delay generation circuit of channel 0"]
    _0 = 0,
    #[doc = "1: Disable delay generation circuit of channel 0."]
    _1 = 1,
}
impl From<DLYEN0_A> for bool {
    #[inline(always)]
    fn from(variant: DLYEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl DLYEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLYEN0_A {
        match self.bits {
            false => DLYEN0_A::_0,
            true => DLYEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLYEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLYEN0_A::_1
    }
}
#[doc = "Field `DLYEN0` writer - PWM Delay Generation Circuit enable for channel 0"]
pub type DLYEN0_W<'a, const O: u8> = crate::BitWriter<'a, u16, GTDLYCR2_SPEC, DLYEN0_A, O>;
impl<'a, const O: u8> DLYEN0_W<'a, O> {
    #[doc = "Enable delay generation circuit of channel 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DLYEN0_A::_0)
    }
    #[doc = "Disable delay generation circuit of channel 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DLYEN0_A::_1)
    }
}
#[doc = "Field `DLYEN1` reader - PWM Delay Generation Circuit enable for channel 1"]
pub type DLYEN1_R = crate::BitReader<DLYEN1_A>;
#[doc = "PWM Delay Generation Circuit enable for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYEN1_A {
    #[doc = "0: Enable delay generation circuit of channel 1"]
    _0 = 0,
    #[doc = "1: Disable delay generation circuit of channel 1."]
    _1 = 1,
}
impl From<DLYEN1_A> for bool {
    #[inline(always)]
    fn from(variant: DLYEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl DLYEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLYEN1_A {
        match self.bits {
            false => DLYEN1_A::_0,
            true => DLYEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLYEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLYEN1_A::_1
    }
}
#[doc = "Field `DLYEN1` writer - PWM Delay Generation Circuit enable for channel 1"]
pub type DLYEN1_W<'a, const O: u8> = crate::BitWriter<'a, u16, GTDLYCR2_SPEC, DLYEN1_A, O>;
impl<'a, const O: u8> DLYEN1_W<'a, O> {
    #[doc = "Enable delay generation circuit of channel 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DLYEN1_A::_0)
    }
    #[doc = "Disable delay generation circuit of channel 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DLYEN1_A::_1)
    }
}
#[doc = "Field `DLYEN2` reader - PWM Delay Generation Circuit enable for channel 2"]
pub type DLYEN2_R = crate::BitReader<DLYEN2_A>;
#[doc = "PWM Delay Generation Circuit enable for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYEN2_A {
    #[doc = "0: Enable delay generation circuit of channel 2"]
    _0 = 0,
    #[doc = "1: Disable delay generation circuit of channel 2."]
    _1 = 1,
}
impl From<DLYEN2_A> for bool {
    #[inline(always)]
    fn from(variant: DLYEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl DLYEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLYEN2_A {
        match self.bits {
            false => DLYEN2_A::_0,
            true => DLYEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLYEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLYEN2_A::_1
    }
}
#[doc = "Field `DLYEN2` writer - PWM Delay Generation Circuit enable for channel 2"]
pub type DLYEN2_W<'a, const O: u8> = crate::BitWriter<'a, u16, GTDLYCR2_SPEC, DLYEN2_A, O>;
impl<'a, const O: u8> DLYEN2_W<'a, O> {
    #[doc = "Enable delay generation circuit of channel 2"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DLYEN2_A::_0)
    }
    #[doc = "Disable delay generation circuit of channel 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DLYEN2_A::_1)
    }
}
#[doc = "Field `DLYEN3` reader - PWM Delay Generation Circuit enable for channel 3"]
pub type DLYEN3_R = crate::BitReader<DLYEN3_A>;
#[doc = "PWM Delay Generation Circuit enable for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYEN3_A {
    #[doc = "0: Enable delay generation circuit of channel 3"]
    _0 = 0,
    #[doc = "1: Disable delay generation circuit of channel 3"]
    _1 = 1,
}
impl From<DLYEN3_A> for bool {
    #[inline(always)]
    fn from(variant: DLYEN3_A) -> Self {
        variant as u8 != 0
    }
}
impl DLYEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLYEN3_A {
        match self.bits {
            false => DLYEN3_A::_0,
            true => DLYEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLYEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLYEN3_A::_1
    }
}
#[doc = "Field `DLYEN3` writer - PWM Delay Generation Circuit enable for channel 3"]
pub type DLYEN3_W<'a, const O: u8> = crate::BitWriter<'a, u16, GTDLYCR2_SPEC, DLYEN3_A, O>;
impl<'a, const O: u8> DLYEN3_W<'a, O> {
    #[doc = "Enable delay generation circuit of channel 3"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DLYEN3_A::_0)
    }
    #[doc = "Disable delay generation circuit of channel 3"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DLYEN3_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - PWM Delay Generation Circuit bypass for channel 0"]
    #[inline(always)]
    pub fn dlybs0(&self) -> DLYBS0_R {
        DLYBS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM Delay Generation Circuit bypass for channel 1"]
    #[inline(always)]
    pub fn dlybs1(&self) -> DLYBS1_R {
        DLYBS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM Delay Generation Circuit bypass for channel 2"]
    #[inline(always)]
    pub fn dlybs2(&self) -> DLYBS2_R {
        DLYBS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM Delay Generation Circuit bypass for channel 3"]
    #[inline(always)]
    pub fn dlybs3(&self) -> DLYBS3_R {
        DLYBS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - PWM Delay Generation Circuit enable for channel 0"]
    #[inline(always)]
    pub fn dlyen0(&self) -> DLYEN0_R {
        DLYEN0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PWM Delay Generation Circuit enable for channel 1"]
    #[inline(always)]
    pub fn dlyen1(&self) -> DLYEN1_R {
        DLYEN1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PWM Delay Generation Circuit enable for channel 2"]
    #[inline(always)]
    pub fn dlyen2(&self) -> DLYEN2_R {
        DLYEN2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PWM Delay Generation Circuit enable for channel 3"]
    #[inline(always)]
    pub fn dlyen3(&self) -> DLYEN3_R {
        DLYEN3_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM Delay Generation Circuit bypass for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn dlybs0(&mut self) -> DLYBS0_W<0> {
        DLYBS0_W::new(self)
    }
    #[doc = "Bit 1 - PWM Delay Generation Circuit bypass for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn dlybs1(&mut self) -> DLYBS1_W<1> {
        DLYBS1_W::new(self)
    }
    #[doc = "Bit 2 - PWM Delay Generation Circuit bypass for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn dlybs2(&mut self) -> DLYBS2_W<2> {
        DLYBS2_W::new(self)
    }
    #[doc = "Bit 3 - PWM Delay Generation Circuit bypass for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn dlybs3(&mut self) -> DLYBS3_W<3> {
        DLYBS3_W::new(self)
    }
    #[doc = "Bit 8 - PWM Delay Generation Circuit enable for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn dlyen0(&mut self) -> DLYEN0_W<8> {
        DLYEN0_W::new(self)
    }
    #[doc = "Bit 9 - PWM Delay Generation Circuit enable for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn dlyen1(&mut self) -> DLYEN1_W<9> {
        DLYEN1_W::new(self)
    }
    #[doc = "Bit 10 - PWM Delay Generation Circuit enable for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn dlyen2(&mut self) -> DLYEN2_W<10> {
        DLYEN2_W::new(self)
    }
    #[doc = "Bit 11 - PWM Delay Generation Circuit enable for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn dlyen3(&mut self) -> DLYEN3_W<11> {
        DLYEN3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Output Delay Control Register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtdlycr2](index.html) module"]
pub struct GTDLYCR2_SPEC;
impl crate::RegisterSpec for GTDLYCR2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gtdlycr2::R](R) reader structure"]
impl crate::Readable for GTDLYCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtdlycr2::W](W) writer structure"]
impl crate::Writable for GTDLYCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTDLYCR2 to value 0"]
impl crate::Resettable for GTDLYCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
