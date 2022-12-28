#[doc = "Register `AGTCR` reader"]
pub struct R(crate::R<AGTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AGTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AGTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AGTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AGTCR` writer"]
pub struct W(crate::W<AGTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AGTCR_SPEC>;
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
impl From<crate::W<AGTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AGTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSTART` reader - AGT Count Start"]
pub type TSTART_R = crate::BitReader<TSTART_A>;
#[doc = "AGT Count Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSTART_A {
    #[doc = "0: Count stops"]
    _0 = 0,
    #[doc = "1: Count starts"]
    _1 = 1,
}
impl From<TSTART_A> for bool {
    #[inline(always)]
    fn from(variant: TSTART_A) -> Self {
        variant as u8 != 0
    }
}
impl TSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSTART_A {
        match self.bits {
            false => TSTART_A::_0,
            true => TSTART_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSTART_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSTART_A::_1
    }
}
#[doc = "Field `TSTART` writer - AGT Count Start"]
pub type TSTART_W<'a, const O: u8> = crate::BitWriter<'a, u8, AGTCR_SPEC, TSTART_A, O>;
impl<'a, const O: u8> TSTART_W<'a, O> {
    #[doc = "Count stops"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSTART_A::_0)
    }
    #[doc = "Count starts"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSTART_A::_1)
    }
}
#[doc = "Field `TCSTF` reader - AGT Count Status Flag"]
pub type TCSTF_R = crate::BitReader<TCSTF_A>;
#[doc = "AGT Count Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCSTF_A {
    #[doc = "0: Count stopped"]
    _0 = 0,
    #[doc = "1: Count in progress"]
    _1 = 1,
}
impl From<TCSTF_A> for bool {
    #[inline(always)]
    fn from(variant: TCSTF_A) -> Self {
        variant as u8 != 0
    }
}
impl TCSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCSTF_A {
        match self.bits {
            false => TCSTF_A::_0,
            true => TCSTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCSTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCSTF_A::_1
    }
}
#[doc = "AGT Count Forced Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSTOP_AW {
    #[doc = "0: Writing is invalid"]
    _0 = 0,
    #[doc = "1: The count is forcibly stopped"]
    _1 = 1,
}
impl From<TSTOP_AW> for bool {
    #[inline(always)]
    fn from(variant: TSTOP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTOP` writer - AGT Count Forced Stop"]
pub type TSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u8, AGTCR_SPEC, TSTOP_AW, O>;
impl<'a, const O: u8> TSTOP_W<'a, O> {
    #[doc = "Writing is invalid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSTOP_AW::_0)
    }
    #[doc = "The count is forcibly stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSTOP_AW::_1)
    }
}
#[doc = "Field `TEDGF` reader - Active Edge Judgment Flag"]
pub type TEDGF_R = crate::BitReader<TEDGF_A>;
#[doc = "Active Edge Judgment Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEDGF_A {
    #[doc = "0: No active edge received"]
    _0 = 0,
    #[doc = "1: Active edge received"]
    _1 = 1,
}
impl From<TEDGF_A> for bool {
    #[inline(always)]
    fn from(variant: TEDGF_A) -> Self {
        variant as u8 != 0
    }
}
impl TEDGF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEDGF_A {
        match self.bits {
            false => TEDGF_A::_0,
            true => TEDGF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEDGF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEDGF_A::_1
    }
}
#[doc = "Field `TEDGF` writer - Active Edge Judgment Flag"]
pub type TEDGF_W<'a, const O: u8> = crate::BitWriter<'a, u8, AGTCR_SPEC, TEDGF_A, O>;
impl<'a, const O: u8> TEDGF_W<'a, O> {
    #[doc = "No active edge received"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEDGF_A::_0)
    }
    #[doc = "Active edge received"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEDGF_A::_1)
    }
}
#[doc = "Field `TUNDF` reader - Underflow Flag"]
pub type TUNDF_R = crate::BitReader<TUNDF_A>;
#[doc = "Underflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TUNDF_A {
    #[doc = "0: No underflow"]
    _0 = 0,
    #[doc = "1: Underflow"]
    _1 = 1,
}
impl From<TUNDF_A> for bool {
    #[inline(always)]
    fn from(variant: TUNDF_A) -> Self {
        variant as u8 != 0
    }
}
impl TUNDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TUNDF_A {
        match self.bits {
            false => TUNDF_A::_0,
            true => TUNDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TUNDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TUNDF_A::_1
    }
}
#[doc = "Field `TUNDF` writer - Underflow Flag"]
pub type TUNDF_W<'a, const O: u8> = crate::BitWriter<'a, u8, AGTCR_SPEC, TUNDF_A, O>;
impl<'a, const O: u8> TUNDF_W<'a, O> {
    #[doc = "No underflow"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TUNDF_A::_0)
    }
    #[doc = "Underflow"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TUNDF_A::_1)
    }
}
#[doc = "Field `TCMAF` reader - Compare Match A Flag"]
pub type TCMAF_R = crate::BitReader<TCMAF_A>;
#[doc = "Compare Match A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCMAF_A {
    #[doc = "0: No match"]
    _0 = 0,
    #[doc = "1: Match"]
    _1 = 1,
}
impl From<TCMAF_A> for bool {
    #[inline(always)]
    fn from(variant: TCMAF_A) -> Self {
        variant as u8 != 0
    }
}
impl TCMAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCMAF_A {
        match self.bits {
            false => TCMAF_A::_0,
            true => TCMAF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCMAF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCMAF_A::_1
    }
}
#[doc = "Field `TCMAF` writer - Compare Match A Flag"]
pub type TCMAF_W<'a, const O: u8> = crate::BitWriter<'a, u8, AGTCR_SPEC, TCMAF_A, O>;
impl<'a, const O: u8> TCMAF_W<'a, O> {
    #[doc = "No match"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCMAF_A::_0)
    }
    #[doc = "Match"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCMAF_A::_1)
    }
}
#[doc = "Field `TCMBF` reader - Compare Match B Flag"]
pub type TCMBF_R = crate::BitReader<TCMBF_A>;
#[doc = "Compare Match B Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCMBF_A {
    #[doc = "0: No match"]
    _0 = 0,
    #[doc = "1: Match"]
    _1 = 1,
}
impl From<TCMBF_A> for bool {
    #[inline(always)]
    fn from(variant: TCMBF_A) -> Self {
        variant as u8 != 0
    }
}
impl TCMBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCMBF_A {
        match self.bits {
            false => TCMBF_A::_0,
            true => TCMBF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCMBF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCMBF_A::_1
    }
}
#[doc = "Field `TCMBF` writer - Compare Match B Flag"]
pub type TCMBF_W<'a, const O: u8> = crate::BitWriter<'a, u8, AGTCR_SPEC, TCMBF_A, O>;
impl<'a, const O: u8> TCMBF_W<'a, O> {
    #[doc = "No match"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCMBF_A::_0)
    }
    #[doc = "Match"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCMBF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AGT Count Start"]
    #[inline(always)]
    pub fn tstart(&self) -> TSTART_R {
        TSTART_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AGT Count Status Flag"]
    #[inline(always)]
    pub fn tcstf(&self) -> TCSTF_R {
        TCSTF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Active Edge Judgment Flag"]
    #[inline(always)]
    pub fn tedgf(&self) -> TEDGF_R {
        TEDGF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Underflow Flag"]
    #[inline(always)]
    pub fn tundf(&self) -> TUNDF_R {
        TUNDF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Compare Match A Flag"]
    #[inline(always)]
    pub fn tcmaf(&self) -> TCMAF_R {
        TCMAF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Compare Match B Flag"]
    #[inline(always)]
    pub fn tcmbf(&self) -> TCMBF_R {
        TCMBF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AGT Count Start"]
    #[inline(always)]
    #[must_use]
    pub fn tstart(&mut self) -> TSTART_W<0> {
        TSTART_W::new(self)
    }
    #[doc = "Bit 2 - AGT Count Forced Stop"]
    #[inline(always)]
    #[must_use]
    pub fn tstop(&mut self) -> TSTOP_W<2> {
        TSTOP_W::new(self)
    }
    #[doc = "Bit 4 - Active Edge Judgment Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tedgf(&mut self) -> TEDGF_W<4> {
        TEDGF_W::new(self)
    }
    #[doc = "Bit 5 - Underflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tundf(&mut self) -> TUNDF_W<5> {
        TUNDF_W::new(self)
    }
    #[doc = "Bit 6 - Compare Match A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcmaf(&mut self) -> TCMAF_W<6> {
        TCMAF_W::new(self)
    }
    #[doc = "Bit 7 - Compare Match B Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcmbf(&mut self) -> TCMBF_W<7> {
        TCMBF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AGT Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [agtcr](index.html) module"]
pub struct AGTCR_SPEC;
impl crate::RegisterSpec for AGTCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [agtcr::R](R) reader structure"]
impl crate::Readable for AGTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [agtcr::W](W) writer structure"]
impl crate::Writable for AGTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AGTCR to value 0"]
impl crate::Resettable for AGTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
