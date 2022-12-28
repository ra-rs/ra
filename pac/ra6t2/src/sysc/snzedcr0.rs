#[doc = "Register `SNZEDCR0` reader"]
pub struct R(crate::R<SNZEDCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SNZEDCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SNZEDCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SNZEDCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SNZEDCR0` writer"]
pub struct W(crate::W<SNZEDCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SNZEDCR0_SPEC>;
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
impl From<crate::W<SNZEDCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SNZEDCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AGTUNFED` reader - AGT1 Underflow Snooze End Enable"]
pub type AGTUNFED_R = crate::BitReader<AGTUNFED_A>;
#[doc = "AGT1 Underflow Snooze End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AGTUNFED_A {
    #[doc = "0: Disable the snooze end request"]
    _0 = 0,
    #[doc = "1: Enable the snooze end request"]
    _1 = 1,
}
impl From<AGTUNFED_A> for bool {
    #[inline(always)]
    fn from(variant: AGTUNFED_A) -> Self {
        variant as u8 != 0
    }
}
impl AGTUNFED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AGTUNFED_A {
        match self.bits {
            false => AGTUNFED_A::_0,
            true => AGTUNFED_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AGTUNFED_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AGTUNFED_A::_1
    }
}
#[doc = "Field `AGTUNFED` writer - AGT1 Underflow Snooze End Enable"]
pub type AGTUNFED_W<'a, const O: u8> = crate::BitWriter<'a, u8, SNZEDCR0_SPEC, AGTUNFED_A, O>;
impl<'a, const O: u8> AGTUNFED_W<'a, O> {
    #[doc = "Disable the snooze end request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AGTUNFED_A::_0)
    }
    #[doc = "Enable the snooze end request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AGTUNFED_A::_1)
    }
}
#[doc = "Field `DTCZRED` reader - Last DTC Transmission Completion Snooze End Enable"]
pub type DTCZRED_R = crate::BitReader<DTCZRED_A>;
#[doc = "Last DTC Transmission Completion Snooze End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTCZRED_A {
    #[doc = "0: Disable the snooze end request"]
    _0 = 0,
    #[doc = "1: Enable the snooze end request"]
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
#[doc = "Field `DTCZRED` writer - Last DTC Transmission Completion Snooze End Enable"]
pub type DTCZRED_W<'a, const O: u8> = crate::BitWriter<'a, u8, SNZEDCR0_SPEC, DTCZRED_A, O>;
impl<'a, const O: u8> DTCZRED_W<'a, O> {
    #[doc = "Disable the snooze end request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTCZRED_A::_0)
    }
    #[doc = "Enable the snooze end request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTCZRED_A::_1)
    }
}
#[doc = "Field `DTCNZRED` reader - Not Last DTC Transmission Completion Snooze End Enable"]
pub type DTCNZRED_R = crate::BitReader<DTCNZRED_A>;
#[doc = "Not Last DTC Transmission Completion Snooze End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTCNZRED_A {
    #[doc = "0: Disable the snooze end request"]
    _0 = 0,
    #[doc = "1: Enable the snooze end request"]
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
#[doc = "Field `DTCNZRED` writer - Not Last DTC Transmission Completion Snooze End Enable"]
pub type DTCNZRED_W<'a, const O: u8> = crate::BitWriter<'a, u8, SNZEDCR0_SPEC, DTCNZRED_A, O>;
impl<'a, const O: u8> DTCNZRED_W<'a, O> {
    #[doc = "Disable the snooze end request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTCNZRED_A::_0)
    }
    #[doc = "Enable the snooze end request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTCNZRED_A::_1)
    }
}
#[doc = "Field `SCI0UMTED` reader - SCI0 Address Mismatch Snooze End Enable"]
pub type SCI0UMTED_R = crate::BitReader<SCI0UMTED_A>;
#[doc = "SCI0 Address Mismatch Snooze End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCI0UMTED_A {
    #[doc = "0: Disable the snooze end request"]
    _0 = 0,
    #[doc = "1: Enable the snooze end request"]
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
#[doc = "Field `SCI0UMTED` writer - SCI0 Address Mismatch Snooze End Enable"]
pub type SCI0UMTED_W<'a, const O: u8> = crate::BitWriter<'a, u8, SNZEDCR0_SPEC, SCI0UMTED_A, O>;
impl<'a, const O: u8> SCI0UMTED_W<'a, O> {
    #[doc = "Disable the snooze end request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCI0UMTED_A::_0)
    }
    #[doc = "Enable the snooze end request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCI0UMTED_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AGT1 Underflow Snooze End Enable"]
    #[inline(always)]
    pub fn agtunfed(&self) -> AGTUNFED_R {
        AGTUNFED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Last DTC Transmission Completion Snooze End Enable"]
    #[inline(always)]
    pub fn dtczred(&self) -> DTCZRED_R {
        DTCZRED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Not Last DTC Transmission Completion Snooze End Enable"]
    #[inline(always)]
    pub fn dtcnzred(&self) -> DTCNZRED_R {
        DTCNZRED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - SCI0 Address Mismatch Snooze End Enable"]
    #[inline(always)]
    pub fn sci0umted(&self) -> SCI0UMTED_R {
        SCI0UMTED_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AGT1 Underflow Snooze End Enable"]
    #[inline(always)]
    #[must_use]
    pub fn agtunfed(&mut self) -> AGTUNFED_W<0> {
        AGTUNFED_W::new(self)
    }
    #[doc = "Bit 1 - Last DTC Transmission Completion Snooze End Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtczred(&mut self) -> DTCZRED_W<1> {
        DTCZRED_W::new(self)
    }
    #[doc = "Bit 2 - Not Last DTC Transmission Completion Snooze End Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtcnzred(&mut self) -> DTCNZRED_W<2> {
        DTCNZRED_W::new(self)
    }
    #[doc = "Bit 7 - SCI0 Address Mismatch Snooze End Enable"]
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
#[doc = "Snooze End Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snzedcr0](index.html) module"]
pub struct SNZEDCR0_SPEC;
impl crate::RegisterSpec for SNZEDCR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [snzedcr0::R](R) reader structure"]
impl crate::Readable for SNZEDCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [snzedcr0::W](W) writer structure"]
impl crate::Writable for SNZEDCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SNZEDCR0 to value 0"]
impl crate::Resettable for SNZEDCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
