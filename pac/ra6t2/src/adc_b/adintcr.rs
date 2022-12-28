#[doc = "Register `ADINTCR` reader"]
pub struct R(crate::R<ADINTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADINTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADINTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADINTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADINTCR` writer"]
pub struct W(crate::W<ADINTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADINTCR_SPEC>;
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
impl From<crate::W<ADINTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADINTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADIE0` reader - Scan Group n Scan End Interrupt Enable"]
pub type ADIE0_R = crate::BitReader<ADIE0_A>;
#[doc = "Scan Group n Scan End Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADIE0_A {
    #[doc = "0: Disable scan end interrupt"]
    _0 = 0,
    #[doc = "1: Enable scan end interrupt"]
    _1 = 1,
}
impl From<ADIE0_A> for bool {
    #[inline(always)]
    fn from(variant: ADIE0_A) -> Self {
        variant as u8 != 0
    }
}
impl ADIE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADIE0_A {
        match self.bits {
            false => ADIE0_A::_0,
            true => ADIE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADIE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADIE0_A::_1
    }
}
#[doc = "Field `ADIE0` writer - Scan Group n Scan End Interrupt Enable"]
pub type ADIE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADINTCR_SPEC, ADIE0_A, O>;
impl<'a, const O: u8> ADIE0_W<'a, O> {
    #[doc = "Disable scan end interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADIE0_A::_0)
    }
    #[doc = "Enable scan end interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADIE0_A::_1)
    }
}
#[doc = "Field `ADIE1` reader - Scan Group n Scan End Interrupt Enable"]
pub type ADIE1_R = crate::BitReader<ADIE1_A>;
#[doc = "Scan Group n Scan End Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADIE1_A {
    #[doc = "0: Disable scan end interrupt"]
    _0 = 0,
    #[doc = "1: Enable scan end interrupt"]
    _1 = 1,
}
impl From<ADIE1_A> for bool {
    #[inline(always)]
    fn from(variant: ADIE1_A) -> Self {
        variant as u8 != 0
    }
}
impl ADIE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADIE1_A {
        match self.bits {
            false => ADIE1_A::_0,
            true => ADIE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADIE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADIE1_A::_1
    }
}
#[doc = "Field `ADIE1` writer - Scan Group n Scan End Interrupt Enable"]
pub type ADIE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADINTCR_SPEC, ADIE1_A, O>;
impl<'a, const O: u8> ADIE1_W<'a, O> {
    #[doc = "Disable scan end interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADIE1_A::_0)
    }
    #[doc = "Enable scan end interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADIE1_A::_1)
    }
}
#[doc = "Field `ADIE2` reader - Scan Group n Scan End Interrupt Enable"]
pub type ADIE2_R = crate::BitReader<ADIE2_A>;
#[doc = "Scan Group n Scan End Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADIE2_A {
    #[doc = "0: Disable scan end interrupt"]
    _0 = 0,
    #[doc = "1: Enable scan end interrupt"]
    _1 = 1,
}
impl From<ADIE2_A> for bool {
    #[inline(always)]
    fn from(variant: ADIE2_A) -> Self {
        variant as u8 != 0
    }
}
impl ADIE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADIE2_A {
        match self.bits {
            false => ADIE2_A::_0,
            true => ADIE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADIE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADIE2_A::_1
    }
}
#[doc = "Field `ADIE2` writer - Scan Group n Scan End Interrupt Enable"]
pub type ADIE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADINTCR_SPEC, ADIE2_A, O>;
impl<'a, const O: u8> ADIE2_W<'a, O> {
    #[doc = "Disable scan end interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADIE2_A::_0)
    }
    #[doc = "Enable scan end interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADIE2_A::_1)
    }
}
#[doc = "Field `ADIE3` reader - Scan Group n Scan End Interrupt Enable"]
pub type ADIE3_R = crate::BitReader<ADIE3_A>;
#[doc = "Scan Group n Scan End Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADIE3_A {
    #[doc = "0: Disable scan end interrupt"]
    _0 = 0,
    #[doc = "1: Enable scan end interrupt"]
    _1 = 1,
}
impl From<ADIE3_A> for bool {
    #[inline(always)]
    fn from(variant: ADIE3_A) -> Self {
        variant as u8 != 0
    }
}
impl ADIE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADIE3_A {
        match self.bits {
            false => ADIE3_A::_0,
            true => ADIE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADIE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADIE3_A::_1
    }
}
#[doc = "Field `ADIE3` writer - Scan Group n Scan End Interrupt Enable"]
pub type ADIE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADINTCR_SPEC, ADIE3_A, O>;
impl<'a, const O: u8> ADIE3_W<'a, O> {
    #[doc = "Disable scan end interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADIE3_A::_0)
    }
    #[doc = "Enable scan end interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADIE3_A::_1)
    }
}
#[doc = "Field `ADIE4` reader - Scan Group n Scan End Interrupt Enable"]
pub type ADIE4_R = crate::BitReader<ADIE4_A>;
#[doc = "Scan Group n Scan End Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADIE4_A {
    #[doc = "0: Disable scan end interrupt"]
    _0 = 0,
    #[doc = "1: Enable scan end interrupt"]
    _1 = 1,
}
impl From<ADIE4_A> for bool {
    #[inline(always)]
    fn from(variant: ADIE4_A) -> Self {
        variant as u8 != 0
    }
}
impl ADIE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADIE4_A {
        match self.bits {
            false => ADIE4_A::_0,
            true => ADIE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADIE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADIE4_A::_1
    }
}
#[doc = "Field `ADIE4` writer - Scan Group n Scan End Interrupt Enable"]
pub type ADIE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADINTCR_SPEC, ADIE4_A, O>;
impl<'a, const O: u8> ADIE4_W<'a, O> {
    #[doc = "Disable scan end interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADIE4_A::_0)
    }
    #[doc = "Enable scan end interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADIE4_A::_1)
    }
}
#[doc = "Field `ADIE5` reader - Scan Group n Scan End Interrupt Enable"]
pub type ADIE5_R = crate::BitReader<ADIE5_A>;
#[doc = "Scan Group n Scan End Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADIE5_A {
    #[doc = "0: Disable scan end interrupt"]
    _0 = 0,
    #[doc = "1: Enable scan end interrupt"]
    _1 = 1,
}
impl From<ADIE5_A> for bool {
    #[inline(always)]
    fn from(variant: ADIE5_A) -> Self {
        variant as u8 != 0
    }
}
impl ADIE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADIE5_A {
        match self.bits {
            false => ADIE5_A::_0,
            true => ADIE5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADIE5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADIE5_A::_1
    }
}
#[doc = "Field `ADIE5` writer - Scan Group n Scan End Interrupt Enable"]
pub type ADIE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADINTCR_SPEC, ADIE5_A, O>;
impl<'a, const O: u8> ADIE5_W<'a, O> {
    #[doc = "Disable scan end interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADIE5_A::_0)
    }
    #[doc = "Enable scan end interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADIE5_A::_1)
    }
}
#[doc = "Field `ADIE6` reader - Scan Group n Scan End Interrupt Enable"]
pub type ADIE6_R = crate::BitReader<ADIE6_A>;
#[doc = "Scan Group n Scan End Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADIE6_A {
    #[doc = "0: Disable scan end interrupt"]
    _0 = 0,
    #[doc = "1: Enable scan end interrupt"]
    _1 = 1,
}
impl From<ADIE6_A> for bool {
    #[inline(always)]
    fn from(variant: ADIE6_A) -> Self {
        variant as u8 != 0
    }
}
impl ADIE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADIE6_A {
        match self.bits {
            false => ADIE6_A::_0,
            true => ADIE6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADIE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADIE6_A::_1
    }
}
#[doc = "Field `ADIE6` writer - Scan Group n Scan End Interrupt Enable"]
pub type ADIE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADINTCR_SPEC, ADIE6_A, O>;
impl<'a, const O: u8> ADIE6_W<'a, O> {
    #[doc = "Disable scan end interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADIE6_A::_0)
    }
    #[doc = "Enable scan end interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADIE6_A::_1)
    }
}
#[doc = "Field `ADIE7` reader - Scan Group n Scan End Interrupt Enable"]
pub type ADIE7_R = crate::BitReader<ADIE7_A>;
#[doc = "Scan Group n Scan End Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADIE7_A {
    #[doc = "0: Disable scan end interrupt"]
    _0 = 0,
    #[doc = "1: Enable scan end interrupt"]
    _1 = 1,
}
impl From<ADIE7_A> for bool {
    #[inline(always)]
    fn from(variant: ADIE7_A) -> Self {
        variant as u8 != 0
    }
}
impl ADIE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADIE7_A {
        match self.bits {
            false => ADIE7_A::_0,
            true => ADIE7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADIE7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADIE7_A::_1
    }
}
#[doc = "Field `ADIE7` writer - Scan Group n Scan End Interrupt Enable"]
pub type ADIE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADINTCR_SPEC, ADIE7_A, O>;
impl<'a, const O: u8> ADIE7_W<'a, O> {
    #[doc = "Disable scan end interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADIE7_A::_0)
    }
    #[doc = "Enable scan end interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADIE7_A::_1)
    }
}
#[doc = "Field `ADIE8` reader - Scan Group n Scan End Interrupt Enable"]
pub type ADIE8_R = crate::BitReader<ADIE8_A>;
#[doc = "Scan Group n Scan End Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADIE8_A {
    #[doc = "0: Disable scan end interrupt"]
    _0 = 0,
    #[doc = "1: Enable scan end interrupt"]
    _1 = 1,
}
impl From<ADIE8_A> for bool {
    #[inline(always)]
    fn from(variant: ADIE8_A) -> Self {
        variant as u8 != 0
    }
}
impl ADIE8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADIE8_A {
        match self.bits {
            false => ADIE8_A::_0,
            true => ADIE8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADIE8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADIE8_A::_1
    }
}
#[doc = "Field `ADIE8` writer - Scan Group n Scan End Interrupt Enable"]
pub type ADIE8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADINTCR_SPEC, ADIE8_A, O>;
impl<'a, const O: u8> ADIE8_W<'a, O> {
    #[doc = "Disable scan end interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADIE8_A::_0)
    }
    #[doc = "Enable scan end interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADIE8_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Scan Group n Scan End Interrupt Enable"]
    #[inline(always)]
    pub fn adie0(&self) -> ADIE0_R {
        ADIE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan Group n Scan End Interrupt Enable"]
    #[inline(always)]
    pub fn adie1(&self) -> ADIE1_R {
        ADIE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Scan Group n Scan End Interrupt Enable"]
    #[inline(always)]
    pub fn adie2(&self) -> ADIE2_R {
        ADIE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Scan Group n Scan End Interrupt Enable"]
    #[inline(always)]
    pub fn adie3(&self) -> ADIE3_R {
        ADIE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Scan Group n Scan End Interrupt Enable"]
    #[inline(always)]
    pub fn adie4(&self) -> ADIE4_R {
        ADIE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Scan Group n Scan End Interrupt Enable"]
    #[inline(always)]
    pub fn adie5(&self) -> ADIE5_R {
        ADIE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Scan Group n Scan End Interrupt Enable"]
    #[inline(always)]
    pub fn adie6(&self) -> ADIE6_R {
        ADIE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Scan Group n Scan End Interrupt Enable"]
    #[inline(always)]
    pub fn adie7(&self) -> ADIE7_R {
        ADIE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Scan Group n Scan End Interrupt Enable"]
    #[inline(always)]
    pub fn adie8(&self) -> ADIE8_R {
        ADIE8_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Scan Group n Scan End Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adie0(&mut self) -> ADIE0_W<0> {
        ADIE0_W::new(self)
    }
    #[doc = "Bit 1 - Scan Group n Scan End Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adie1(&mut self) -> ADIE1_W<1> {
        ADIE1_W::new(self)
    }
    #[doc = "Bit 2 - Scan Group n Scan End Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adie2(&mut self) -> ADIE2_W<2> {
        ADIE2_W::new(self)
    }
    #[doc = "Bit 3 - Scan Group n Scan End Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adie3(&mut self) -> ADIE3_W<3> {
        ADIE3_W::new(self)
    }
    #[doc = "Bit 4 - Scan Group n Scan End Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adie4(&mut self) -> ADIE4_W<4> {
        ADIE4_W::new(self)
    }
    #[doc = "Bit 5 - Scan Group n Scan End Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adie5(&mut self) -> ADIE5_W<5> {
        ADIE5_W::new(self)
    }
    #[doc = "Bit 6 - Scan Group n Scan End Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adie6(&mut self) -> ADIE6_W<6> {
        ADIE6_W::new(self)
    }
    #[doc = "Bit 7 - Scan Group n Scan End Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adie7(&mut self) -> ADIE7_W<7> {
        ADIE7_W::new(self)
    }
    #[doc = "Bit 8 - Scan Group n Scan End Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adie8(&mut self) -> ADIE8_W<8> {
        ADIE8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scan End Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adintcr](index.html) module"]
pub struct ADINTCR_SPEC;
impl crate::RegisterSpec for ADINTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adintcr::R](R) reader structure"]
impl crate::Readable for ADINTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adintcr::W](W) writer structure"]
impl crate::Writable for ADINTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADINTCR to value 0"]
impl crate::Resettable for ADINTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
