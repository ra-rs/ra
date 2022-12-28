#[doc = "Register `ADSGER` reader"]
pub struct R(crate::R<ADSGER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADSGER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADSGER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADSGER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADSGER` writer"]
pub struct W(crate::W<ADSGER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSGER_SPEC>;
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
impl From<crate::W<ADSGER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSGER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SGRE0` reader - Scan Group n Enable"]
pub type SGRE0_R = crate::BitReader<SGRE0_A>;
#[doc = "Scan Group n Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SGRE0_A {
    #[doc = "0: Disable the scan group n"]
    _0 = 0,
    #[doc = "1: Enable the scan group n"]
    _1 = 1,
}
impl From<SGRE0_A> for bool {
    #[inline(always)]
    fn from(variant: SGRE0_A) -> Self {
        variant as u8 != 0
    }
}
impl SGRE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SGRE0_A {
        match self.bits {
            false => SGRE0_A::_0,
            true => SGRE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SGRE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SGRE0_A::_1
    }
}
#[doc = "Field `SGRE0` writer - Scan Group n Enable"]
pub type SGRE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSGER_SPEC, SGRE0_A, O>;
impl<'a, const O: u8> SGRE0_W<'a, O> {
    #[doc = "Disable the scan group n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SGRE0_A::_0)
    }
    #[doc = "Enable the scan group n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SGRE0_A::_1)
    }
}
#[doc = "Field `SGRE1` reader - Scan Group n Enable"]
pub type SGRE1_R = crate::BitReader<SGRE1_A>;
#[doc = "Scan Group n Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SGRE1_A {
    #[doc = "0: Disable the scan group n"]
    _0 = 0,
    #[doc = "1: Enable the scan group n"]
    _1 = 1,
}
impl From<SGRE1_A> for bool {
    #[inline(always)]
    fn from(variant: SGRE1_A) -> Self {
        variant as u8 != 0
    }
}
impl SGRE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SGRE1_A {
        match self.bits {
            false => SGRE1_A::_0,
            true => SGRE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SGRE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SGRE1_A::_1
    }
}
#[doc = "Field `SGRE1` writer - Scan Group n Enable"]
pub type SGRE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSGER_SPEC, SGRE1_A, O>;
impl<'a, const O: u8> SGRE1_W<'a, O> {
    #[doc = "Disable the scan group n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SGRE1_A::_0)
    }
    #[doc = "Enable the scan group n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SGRE1_A::_1)
    }
}
#[doc = "Field `SGRE2` reader - Scan Group n Enable"]
pub type SGRE2_R = crate::BitReader<SGRE2_A>;
#[doc = "Scan Group n Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SGRE2_A {
    #[doc = "0: Disable the scan group n"]
    _0 = 0,
    #[doc = "1: Enable the scan group n"]
    _1 = 1,
}
impl From<SGRE2_A> for bool {
    #[inline(always)]
    fn from(variant: SGRE2_A) -> Self {
        variant as u8 != 0
    }
}
impl SGRE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SGRE2_A {
        match self.bits {
            false => SGRE2_A::_0,
            true => SGRE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SGRE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SGRE2_A::_1
    }
}
#[doc = "Field `SGRE2` writer - Scan Group n Enable"]
pub type SGRE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSGER_SPEC, SGRE2_A, O>;
impl<'a, const O: u8> SGRE2_W<'a, O> {
    #[doc = "Disable the scan group n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SGRE2_A::_0)
    }
    #[doc = "Enable the scan group n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SGRE2_A::_1)
    }
}
#[doc = "Field `SGRE3` reader - Scan Group n Enable"]
pub type SGRE3_R = crate::BitReader<SGRE3_A>;
#[doc = "Scan Group n Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SGRE3_A {
    #[doc = "0: Disable the scan group n"]
    _0 = 0,
    #[doc = "1: Enable the scan group n"]
    _1 = 1,
}
impl From<SGRE3_A> for bool {
    #[inline(always)]
    fn from(variant: SGRE3_A) -> Self {
        variant as u8 != 0
    }
}
impl SGRE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SGRE3_A {
        match self.bits {
            false => SGRE3_A::_0,
            true => SGRE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SGRE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SGRE3_A::_1
    }
}
#[doc = "Field `SGRE3` writer - Scan Group n Enable"]
pub type SGRE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSGER_SPEC, SGRE3_A, O>;
impl<'a, const O: u8> SGRE3_W<'a, O> {
    #[doc = "Disable the scan group n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SGRE3_A::_0)
    }
    #[doc = "Enable the scan group n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SGRE3_A::_1)
    }
}
#[doc = "Field `SGRE4` reader - Scan Group n Enable"]
pub type SGRE4_R = crate::BitReader<SGRE4_A>;
#[doc = "Scan Group n Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SGRE4_A {
    #[doc = "0: Disable the scan group n"]
    _0 = 0,
    #[doc = "1: Enable the scan group n"]
    _1 = 1,
}
impl From<SGRE4_A> for bool {
    #[inline(always)]
    fn from(variant: SGRE4_A) -> Self {
        variant as u8 != 0
    }
}
impl SGRE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SGRE4_A {
        match self.bits {
            false => SGRE4_A::_0,
            true => SGRE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SGRE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SGRE4_A::_1
    }
}
#[doc = "Field `SGRE4` writer - Scan Group n Enable"]
pub type SGRE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSGER_SPEC, SGRE4_A, O>;
impl<'a, const O: u8> SGRE4_W<'a, O> {
    #[doc = "Disable the scan group n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SGRE4_A::_0)
    }
    #[doc = "Enable the scan group n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SGRE4_A::_1)
    }
}
#[doc = "Field `SGRE5` reader - Scan Group n Enable"]
pub type SGRE5_R = crate::BitReader<SGRE5_A>;
#[doc = "Scan Group n Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SGRE5_A {
    #[doc = "0: Disable the scan group n"]
    _0 = 0,
    #[doc = "1: Enable the scan group n"]
    _1 = 1,
}
impl From<SGRE5_A> for bool {
    #[inline(always)]
    fn from(variant: SGRE5_A) -> Self {
        variant as u8 != 0
    }
}
impl SGRE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SGRE5_A {
        match self.bits {
            false => SGRE5_A::_0,
            true => SGRE5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SGRE5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SGRE5_A::_1
    }
}
#[doc = "Field `SGRE5` writer - Scan Group n Enable"]
pub type SGRE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSGER_SPEC, SGRE5_A, O>;
impl<'a, const O: u8> SGRE5_W<'a, O> {
    #[doc = "Disable the scan group n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SGRE5_A::_0)
    }
    #[doc = "Enable the scan group n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SGRE5_A::_1)
    }
}
#[doc = "Field `SGRE6` reader - Scan Group n Enable"]
pub type SGRE6_R = crate::BitReader<SGRE6_A>;
#[doc = "Scan Group n Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SGRE6_A {
    #[doc = "0: Disable the scan group n"]
    _0 = 0,
    #[doc = "1: Enable the scan group n"]
    _1 = 1,
}
impl From<SGRE6_A> for bool {
    #[inline(always)]
    fn from(variant: SGRE6_A) -> Self {
        variant as u8 != 0
    }
}
impl SGRE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SGRE6_A {
        match self.bits {
            false => SGRE6_A::_0,
            true => SGRE6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SGRE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SGRE6_A::_1
    }
}
#[doc = "Field `SGRE6` writer - Scan Group n Enable"]
pub type SGRE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSGER_SPEC, SGRE6_A, O>;
impl<'a, const O: u8> SGRE6_W<'a, O> {
    #[doc = "Disable the scan group n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SGRE6_A::_0)
    }
    #[doc = "Enable the scan group n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SGRE6_A::_1)
    }
}
#[doc = "Field `SGRE7` reader - Scan Group n Enable"]
pub type SGRE7_R = crate::BitReader<SGRE7_A>;
#[doc = "Scan Group n Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SGRE7_A {
    #[doc = "0: Disable the scan group n"]
    _0 = 0,
    #[doc = "1: Enable the scan group n"]
    _1 = 1,
}
impl From<SGRE7_A> for bool {
    #[inline(always)]
    fn from(variant: SGRE7_A) -> Self {
        variant as u8 != 0
    }
}
impl SGRE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SGRE7_A {
        match self.bits {
            false => SGRE7_A::_0,
            true => SGRE7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SGRE7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SGRE7_A::_1
    }
}
#[doc = "Field `SGRE7` writer - Scan Group n Enable"]
pub type SGRE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSGER_SPEC, SGRE7_A, O>;
impl<'a, const O: u8> SGRE7_W<'a, O> {
    #[doc = "Disable the scan group n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SGRE7_A::_0)
    }
    #[doc = "Enable the scan group n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SGRE7_A::_1)
    }
}
#[doc = "Field `SGRE8` reader - Scan Group n Enable"]
pub type SGRE8_R = crate::BitReader<SGRE8_A>;
#[doc = "Scan Group n Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SGRE8_A {
    #[doc = "0: Disable the scan group n"]
    _0 = 0,
    #[doc = "1: Enable the scan group n"]
    _1 = 1,
}
impl From<SGRE8_A> for bool {
    #[inline(always)]
    fn from(variant: SGRE8_A) -> Self {
        variant as u8 != 0
    }
}
impl SGRE8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SGRE8_A {
        match self.bits {
            false => SGRE8_A::_0,
            true => SGRE8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SGRE8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SGRE8_A::_1
    }
}
#[doc = "Field `SGRE8` writer - Scan Group n Enable"]
pub type SGRE8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSGER_SPEC, SGRE8_A, O>;
impl<'a, const O: u8> SGRE8_W<'a, O> {
    #[doc = "Disable the scan group n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SGRE8_A::_0)
    }
    #[doc = "Enable the scan group n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SGRE8_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Scan Group n Enable"]
    #[inline(always)]
    pub fn sgre0(&self) -> SGRE0_R {
        SGRE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan Group n Enable"]
    #[inline(always)]
    pub fn sgre1(&self) -> SGRE1_R {
        SGRE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Scan Group n Enable"]
    #[inline(always)]
    pub fn sgre2(&self) -> SGRE2_R {
        SGRE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Scan Group n Enable"]
    #[inline(always)]
    pub fn sgre3(&self) -> SGRE3_R {
        SGRE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Scan Group n Enable"]
    #[inline(always)]
    pub fn sgre4(&self) -> SGRE4_R {
        SGRE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Scan Group n Enable"]
    #[inline(always)]
    pub fn sgre5(&self) -> SGRE5_R {
        SGRE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Scan Group n Enable"]
    #[inline(always)]
    pub fn sgre6(&self) -> SGRE6_R {
        SGRE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Scan Group n Enable"]
    #[inline(always)]
    pub fn sgre7(&self) -> SGRE7_R {
        SGRE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Scan Group n Enable"]
    #[inline(always)]
    pub fn sgre8(&self) -> SGRE8_R {
        SGRE8_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Scan Group n Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sgre0(&mut self) -> SGRE0_W<0> {
        SGRE0_W::new(self)
    }
    #[doc = "Bit 1 - Scan Group n Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sgre1(&mut self) -> SGRE1_W<1> {
        SGRE1_W::new(self)
    }
    #[doc = "Bit 2 - Scan Group n Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sgre2(&mut self) -> SGRE2_W<2> {
        SGRE2_W::new(self)
    }
    #[doc = "Bit 3 - Scan Group n Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sgre3(&mut self) -> SGRE3_W<3> {
        SGRE3_W::new(self)
    }
    #[doc = "Bit 4 - Scan Group n Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sgre4(&mut self) -> SGRE4_W<4> {
        SGRE4_W::new(self)
    }
    #[doc = "Bit 5 - Scan Group n Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sgre5(&mut self) -> SGRE5_W<5> {
        SGRE5_W::new(self)
    }
    #[doc = "Bit 6 - Scan Group n Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sgre6(&mut self) -> SGRE6_W<6> {
        SGRE6_W::new(self)
    }
    #[doc = "Bit 7 - Scan Group n Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sgre7(&mut self) -> SGRE7_W<7> {
        SGRE7_W::new(self)
    }
    #[doc = "Bit 8 - Scan Group n Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sgre8(&mut self) -> SGRE8_W<8> {
        SGRE8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scan Group Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adsger](index.html) module"]
pub struct ADSGER_SPEC;
impl crate::RegisterSpec for ADSGER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adsger::R](R) reader structure"]
impl crate::Readable for ADSGER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adsger::W](W) writer structure"]
impl crate::Writable for ADSGER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSGER to value 0"]
impl crate::Resettable for ADSGER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
