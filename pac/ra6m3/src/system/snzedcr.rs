#[doc = "Register `SNZEDCR` reader"]
pub struct R(crate::R<SNZEDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SNZEDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SNZEDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SNZEDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SNZEDCR` writer"]
pub struct W(crate::W<SNZEDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SNZEDCR_SPEC>;
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
impl From<crate::W<SNZEDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SNZEDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AGT1UNFED` reader - AGT1 underflow Snooze End Enable"]
pub type AGT1UNFED_R = crate::BitReader<AGT1UNFED_A>;
#[doc = "AGT1 underflow Snooze End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AGT1UNFED_A {
    #[doc = "0: Disable the Snooze End request"]
    _0 = 0,
    #[doc = "1: Enable the Snooze End request"]
    _1 = 1,
}
impl From<AGT1UNFED_A> for bool {
    #[inline(always)]
    fn from(variant: AGT1UNFED_A) -> Self {
        variant as u8 != 0
    }
}
impl AGT1UNFED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AGT1UNFED_A {
        match self.bits {
            false => AGT1UNFED_A::_0,
            true => AGT1UNFED_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AGT1UNFED_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AGT1UNFED_A::_1
    }
}
#[doc = "Field `AGT1UNFED` writer - AGT1 underflow Snooze End Enable"]
pub type AGT1UNFED_W<'a, const O: u8> = crate::BitWriter<'a, u8, SNZEDCR_SPEC, AGT1UNFED_A, O>;
impl<'a, const O: u8> AGT1UNFED_W<'a, O> {
    #[doc = "Disable the Snooze End request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AGT1UNFED_A::_0)
    }
    #[doc = "Enable the Snooze End request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AGT1UNFED_A::_1)
    }
}
#[doc = "Field `DTCZRED` reader - Last DTC transmission completion Snooze End Enable"]
pub type DTCZRED_R = crate::BitReader<DTCZRED_A>;
#[doc = "Last DTC transmission completion Snooze End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTCZRED_A {
    #[doc = "0: Disable the Snooze End request"]
    _0 = 0,
    #[doc = "1: Enable the Snooze End request"]
    _1 = 1,
}
impl From<DTCZRED_A> for bool {
    #[inline(always)]
    fn from(variant: DTCZRED_A) -> Self {
        variant as u8 != 0
    }
}
impl DTCZRED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTCZRED_A {
        match self.bits {
            false => DTCZRED_A::_0,
            true => DTCZRED_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTCZRED_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTCZRED_A::_1
    }
}
#[doc = "Field `DTCZRED` writer - Last DTC transmission completion Snooze End Enable"]
pub type DTCZRED_W<'a, const O: u8> = crate::BitWriter<'a, u8, SNZEDCR_SPEC, DTCZRED_A, O>;
impl<'a, const O: u8> DTCZRED_W<'a, O> {
    #[doc = "Disable the Snooze End request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTCZRED_A::_0)
    }
    #[doc = "Enable the Snooze End request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTCZRED_A::_1)
    }
}
#[doc = "Field `DTCNZRED` reader - Not Last DTC transmission completion Snooze End Enable"]
pub type DTCNZRED_R = crate::BitReader<DTCNZRED_A>;
#[doc = "Not Last DTC transmission completion Snooze End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTCNZRED_A {
    #[doc = "0: Disable the Snooze End request"]
    _0 = 0,
    #[doc = "1: Enable the Snooze End request"]
    _1 = 1,
}
impl From<DTCNZRED_A> for bool {
    #[inline(always)]
    fn from(variant: DTCNZRED_A) -> Self {
        variant as u8 != 0
    }
}
impl DTCNZRED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTCNZRED_A {
        match self.bits {
            false => DTCNZRED_A::_0,
            true => DTCNZRED_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTCNZRED_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTCNZRED_A::_1
    }
}
#[doc = "Field `DTCNZRED` writer - Not Last DTC transmission completion Snooze End Enable"]
pub type DTCNZRED_W<'a, const O: u8> = crate::BitWriter<'a, u8, SNZEDCR_SPEC, DTCNZRED_A, O>;
impl<'a, const O: u8> DTCNZRED_W<'a, O> {
    #[doc = "Disable the Snooze End request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTCNZRED_A::_0)
    }
    #[doc = "Enable the Snooze End request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTCNZRED_A::_1)
    }
}
#[doc = "Field `AD0MATED` reader - AD compare match 0 Snooze End Enable"]
pub type AD0MATED_R = crate::BitReader<AD0MATED_A>;
#[doc = "AD compare match 0 Snooze End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD0MATED_A {
    #[doc = "0: Disable the Snooze End request"]
    _0 = 0,
    #[doc = "1: Enable the Snooze End request"]
    _1 = 1,
}
impl From<AD0MATED_A> for bool {
    #[inline(always)]
    fn from(variant: AD0MATED_A) -> Self {
        variant as u8 != 0
    }
}
impl AD0MATED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AD0MATED_A {
        match self.bits {
            false => AD0MATED_A::_0,
            true => AD0MATED_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AD0MATED_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AD0MATED_A::_1
    }
}
#[doc = "Field `AD0MATED` writer - AD compare match 0 Snooze End Enable"]
pub type AD0MATED_W<'a, const O: u8> = crate::BitWriter<'a, u8, SNZEDCR_SPEC, AD0MATED_A, O>;
impl<'a, const O: u8> AD0MATED_W<'a, O> {
    #[doc = "Disable the Snooze End request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AD0MATED_A::_0)
    }
    #[doc = "Enable the Snooze End request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AD0MATED_A::_1)
    }
}
#[doc = "Field `AD0UMTED` reader - AD compare mismatch 0 Snooze End Enable"]
pub type AD0UMTED_R = crate::BitReader<AD0UMTED_A>;
#[doc = "AD compare mismatch 0 Snooze End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD0UMTED_A {
    #[doc = "0: Disable the Snooze End request"]
    _0 = 0,
    #[doc = "1: Enable the Snooze End request"]
    _1 = 1,
}
impl From<AD0UMTED_A> for bool {
    #[inline(always)]
    fn from(variant: AD0UMTED_A) -> Self {
        variant as u8 != 0
    }
}
impl AD0UMTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AD0UMTED_A {
        match self.bits {
            false => AD0UMTED_A::_0,
            true => AD0UMTED_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AD0UMTED_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AD0UMTED_A::_1
    }
}
#[doc = "Field `AD0UMTED` writer - AD compare mismatch 0 Snooze End Enable"]
pub type AD0UMTED_W<'a, const O: u8> = crate::BitWriter<'a, u8, SNZEDCR_SPEC, AD0UMTED_A, O>;
impl<'a, const O: u8> AD0UMTED_W<'a, O> {
    #[doc = "Disable the Snooze End request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AD0UMTED_A::_0)
    }
    #[doc = "Enable the Snooze End request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AD0UMTED_A::_1)
    }
}
#[doc = "Field `AD1MATED` reader - AD compare match 1 Snooze End Enable"]
pub type AD1MATED_R = crate::BitReader<AD1MATED_A>;
#[doc = "AD compare match 1 Snooze End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD1MATED_A {
    #[doc = "0: Disable the Snooze End request"]
    _0 = 0,
    #[doc = "1: Enable the Snooze End request"]
    _1 = 1,
}
impl From<AD1MATED_A> for bool {
    #[inline(always)]
    fn from(variant: AD1MATED_A) -> Self {
        variant as u8 != 0
    }
}
impl AD1MATED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AD1MATED_A {
        match self.bits {
            false => AD1MATED_A::_0,
            true => AD1MATED_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AD1MATED_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AD1MATED_A::_1
    }
}
#[doc = "Field `AD1MATED` writer - AD compare match 1 Snooze End Enable"]
pub type AD1MATED_W<'a, const O: u8> = crate::BitWriter<'a, u8, SNZEDCR_SPEC, AD1MATED_A, O>;
impl<'a, const O: u8> AD1MATED_W<'a, O> {
    #[doc = "Disable the Snooze End request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AD1MATED_A::_0)
    }
    #[doc = "Enable the Snooze End request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AD1MATED_A::_1)
    }
}
#[doc = "Field `AD1UMTED` reader - AD compare mismatch 1 Snooze End Enable"]
pub type AD1UMTED_R = crate::BitReader<AD1UMTED_A>;
#[doc = "AD compare mismatch 1 Snooze End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD1UMTED_A {
    #[doc = "0: Disable the Snooze End request"]
    _0 = 0,
    #[doc = "1: Enable the Snooze End request"]
    _1 = 1,
}
impl From<AD1UMTED_A> for bool {
    #[inline(always)]
    fn from(variant: AD1UMTED_A) -> Self {
        variant as u8 != 0
    }
}
impl AD1UMTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AD1UMTED_A {
        match self.bits {
            false => AD1UMTED_A::_0,
            true => AD1UMTED_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AD1UMTED_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AD1UMTED_A::_1
    }
}
#[doc = "Field `AD1UMTED` writer - AD compare mismatch 1 Snooze End Enable"]
pub type AD1UMTED_W<'a, const O: u8> = crate::BitWriter<'a, u8, SNZEDCR_SPEC, AD1UMTED_A, O>;
impl<'a, const O: u8> AD1UMTED_W<'a, O> {
    #[doc = "Disable the Snooze End request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AD1UMTED_A::_0)
    }
    #[doc = "Enable the Snooze End request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AD1UMTED_A::_1)
    }
}
#[doc = "Field `SCI0UMTED` reader - SCI0 address unmatch Snooze End EnableNote: Do not set to 1 other than in asynchronous mode."]
pub type SCI0UMTED_R = crate::BitReader<SCI0UMTED_A>;
#[doc = "SCI0 address unmatch Snooze End EnableNote: Do not set to 1 other than in asynchronous mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCI0UMTED_A {
    #[doc = "0: Disable the Snooze End request"]
    _0 = 0,
    #[doc = "1: Enable the Snooze End request"]
    _1 = 1,
}
impl From<SCI0UMTED_A> for bool {
    #[inline(always)]
    fn from(variant: SCI0UMTED_A) -> Self {
        variant as u8 != 0
    }
}
impl SCI0UMTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCI0UMTED_A {
        match self.bits {
            false => SCI0UMTED_A::_0,
            true => SCI0UMTED_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCI0UMTED_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCI0UMTED_A::_1
    }
}
#[doc = "Field `SCI0UMTED` writer - SCI0 address unmatch Snooze End EnableNote: Do not set to 1 other than in asynchronous mode."]
pub type SCI0UMTED_W<'a, const O: u8> = crate::BitWriter<'a, u8, SNZEDCR_SPEC, SCI0UMTED_A, O>;
impl<'a, const O: u8> SCI0UMTED_W<'a, O> {
    #[doc = "Disable the Snooze End request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCI0UMTED_A::_0)
    }
    #[doc = "Enable the Snooze End request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCI0UMTED_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AGT1 underflow Snooze End Enable"]
    #[inline(always)]
    pub fn agt1unfed(&self) -> AGT1UNFED_R {
        AGT1UNFED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Last DTC transmission completion Snooze End Enable"]
    #[inline(always)]
    pub fn dtczred(&self) -> DTCZRED_R {
        DTCZRED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Not Last DTC transmission completion Snooze End Enable"]
    #[inline(always)]
    pub fn dtcnzred(&self) -> DTCNZRED_R {
        DTCNZRED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AD compare match 0 Snooze End Enable"]
    #[inline(always)]
    pub fn ad0mated(&self) -> AD0MATED_R {
        AD0MATED_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AD compare mismatch 0 Snooze End Enable"]
    #[inline(always)]
    pub fn ad0umted(&self) -> AD0UMTED_R {
        AD0UMTED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AD compare match 1 Snooze End Enable"]
    #[inline(always)]
    pub fn ad1mated(&self) -> AD1MATED_R {
        AD1MATED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AD compare mismatch 1 Snooze End Enable"]
    #[inline(always)]
    pub fn ad1umted(&self) -> AD1UMTED_R {
        AD1UMTED_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCI0 address unmatch Snooze End EnableNote: Do not set to 1 other than in asynchronous mode."]
    #[inline(always)]
    pub fn sci0umted(&self) -> SCI0UMTED_R {
        SCI0UMTED_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AGT1 underflow Snooze End Enable"]
    #[inline(always)]
    #[must_use]
    pub fn agt1unfed(&mut self) -> AGT1UNFED_W<0> {
        AGT1UNFED_W::new(self)
    }
    #[doc = "Bit 1 - Last DTC transmission completion Snooze End Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtczred(&mut self) -> DTCZRED_W<1> {
        DTCZRED_W::new(self)
    }
    #[doc = "Bit 2 - Not Last DTC transmission completion Snooze End Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtcnzred(&mut self) -> DTCNZRED_W<2> {
        DTCNZRED_W::new(self)
    }
    #[doc = "Bit 3 - AD compare match 0 Snooze End Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ad0mated(&mut self) -> AD0MATED_W<3> {
        AD0MATED_W::new(self)
    }
    #[doc = "Bit 4 - AD compare mismatch 0 Snooze End Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ad0umted(&mut self) -> AD0UMTED_W<4> {
        AD0UMTED_W::new(self)
    }
    #[doc = "Bit 5 - AD compare match 1 Snooze End Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ad1mated(&mut self) -> AD1MATED_W<5> {
        AD1MATED_W::new(self)
    }
    #[doc = "Bit 6 - AD compare mismatch 1 Snooze End Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ad1umted(&mut self) -> AD1UMTED_W<6> {
        AD1UMTED_W::new(self)
    }
    #[doc = "Bit 7 - SCI0 address unmatch Snooze End EnableNote: Do not set to 1 other than in asynchronous mode."]
    #[inline(always)]
    #[must_use]
    pub fn sci0umted(&mut self) -> SCI0UMTED_W<7> {
        SCI0UMTED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Snooze End Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snzedcr](index.html) module"]
pub struct SNZEDCR_SPEC;
impl crate::RegisterSpec for SNZEDCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [snzedcr::R](R) reader structure"]
impl crate::Readable for SNZEDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [snzedcr::W](W) writer structure"]
impl crate::Writable for SNZEDCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SNZEDCR to value 0"]
impl crate::Resettable for SNZEDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
