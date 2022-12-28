#[doc = "Register `DPSIER0` reader"]
pub struct R(crate::R<DPSIER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPSIER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPSIER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPSIER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPSIER0` writer"]
pub struct W(crate::W<DPSIER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPSIER0_SPEC>;
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
impl From<crate::W<DPSIER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPSIER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIRQ0E` reader - IRQ0-DS Pin Enable"]
pub type DIRQ0E_R = crate::BitReader<DIRQ0E_A>;
#[doc = "IRQ0-DS Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ0E_A {
    #[doc = "0: Canceling deep software standby mode is disabled"]
    _0 = 0,
    #[doc = "1: Canceling deep software standby mode is enabled"]
    _1 = 1,
}
impl From<DIRQ0E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ0E_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ0E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ0E_A {
        match self.bits {
            false => DIRQ0E_A::_0,
            true => DIRQ0E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ0E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ0E_A::_1
    }
}
#[doc = "Field `DIRQ0E` writer - IRQ0-DS Pin Enable"]
pub type DIRQ0E_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIER0_SPEC, DIRQ0E_A, O>;
impl<'a, const O: u8> DIRQ0E_W<'a, O> {
    #[doc = "Canceling deep software standby mode is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ0E_A::_0)
    }
    #[doc = "Canceling deep software standby mode is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ0E_A::_1)
    }
}
#[doc = "Field `DIRQ1E` reader - IRQ1-DS Pin Enable"]
pub type DIRQ1E_R = crate::BitReader<DIRQ1E_A>;
#[doc = "IRQ1-DS Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ1E_A {
    #[doc = "0: Canceling deep software standby mode is disabled"]
    _0 = 0,
    #[doc = "1: Canceling deep software standby mode is enabled"]
    _1 = 1,
}
impl From<DIRQ1E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ1E_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ1E_A {
        match self.bits {
            false => DIRQ1E_A::_0,
            true => DIRQ1E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ1E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ1E_A::_1
    }
}
#[doc = "Field `DIRQ1E` writer - IRQ1-DS Pin Enable"]
pub type DIRQ1E_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIER0_SPEC, DIRQ1E_A, O>;
impl<'a, const O: u8> DIRQ1E_W<'a, O> {
    #[doc = "Canceling deep software standby mode is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ1E_A::_0)
    }
    #[doc = "Canceling deep software standby mode is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ1E_A::_1)
    }
}
#[doc = "Field `DIRQ2E` reader - IRQ2-DS Pin Enable"]
pub type DIRQ2E_R = crate::BitReader<DIRQ2E_A>;
#[doc = "IRQ2-DS Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ2E_A {
    #[doc = "0: Canceling deep software standby mode is disabled"]
    _0 = 0,
    #[doc = "1: Canceling deep software standby mode is enabled"]
    _1 = 1,
}
impl From<DIRQ2E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ2E_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ2E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ2E_A {
        match self.bits {
            false => DIRQ2E_A::_0,
            true => DIRQ2E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ2E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ2E_A::_1
    }
}
#[doc = "Field `DIRQ2E` writer - IRQ2-DS Pin Enable"]
pub type DIRQ2E_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIER0_SPEC, DIRQ2E_A, O>;
impl<'a, const O: u8> DIRQ2E_W<'a, O> {
    #[doc = "Canceling deep software standby mode is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ2E_A::_0)
    }
    #[doc = "Canceling deep software standby mode is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ2E_A::_1)
    }
}
#[doc = "Field `DIRQ3E` reader - IRQ3-DS Pin Enable"]
pub type DIRQ3E_R = crate::BitReader<DIRQ3E_A>;
#[doc = "IRQ3-DS Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ3E_A {
    #[doc = "0: Canceling deep software standby mode is disabled"]
    _0 = 0,
    #[doc = "1: Canceling deep software standby mode is enabled"]
    _1 = 1,
}
impl From<DIRQ3E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ3E_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ3E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ3E_A {
        match self.bits {
            false => DIRQ3E_A::_0,
            true => DIRQ3E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ3E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ3E_A::_1
    }
}
#[doc = "Field `DIRQ3E` writer - IRQ3-DS Pin Enable"]
pub type DIRQ3E_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIER0_SPEC, DIRQ3E_A, O>;
impl<'a, const O: u8> DIRQ3E_W<'a, O> {
    #[doc = "Canceling deep software standby mode is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ3E_A::_0)
    }
    #[doc = "Canceling deep software standby mode is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ3E_A::_1)
    }
}
#[doc = "Field `DIRQ4E` reader - IRQ4-DS Pin Enable"]
pub type DIRQ4E_R = crate::BitReader<DIRQ4E_A>;
#[doc = "IRQ4-DS Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ4E_A {
    #[doc = "0: Canceling deep software standby mode is disabled"]
    _0 = 0,
    #[doc = "1: Canceling deep software standby mode is enabled"]
    _1 = 1,
}
impl From<DIRQ4E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ4E_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ4E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ4E_A {
        match self.bits {
            false => DIRQ4E_A::_0,
            true => DIRQ4E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ4E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ4E_A::_1
    }
}
#[doc = "Field `DIRQ4E` writer - IRQ4-DS Pin Enable"]
pub type DIRQ4E_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIER0_SPEC, DIRQ4E_A, O>;
impl<'a, const O: u8> DIRQ4E_W<'a, O> {
    #[doc = "Canceling deep software standby mode is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ4E_A::_0)
    }
    #[doc = "Canceling deep software standby mode is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ4E_A::_1)
    }
}
#[doc = "Field `DIRQ5E` reader - IRQ5-DS Pin Enable"]
pub type DIRQ5E_R = crate::BitReader<DIRQ5E_A>;
#[doc = "IRQ5-DS Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ5E_A {
    #[doc = "0: Canceling deep software standby mode is disabled"]
    _0 = 0,
    #[doc = "1: Canceling deep software standby mode is enabled"]
    _1 = 1,
}
impl From<DIRQ5E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ5E_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ5E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ5E_A {
        match self.bits {
            false => DIRQ5E_A::_0,
            true => DIRQ5E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ5E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ5E_A::_1
    }
}
#[doc = "Field `DIRQ5E` writer - IRQ5-DS Pin Enable"]
pub type DIRQ5E_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIER0_SPEC, DIRQ5E_A, O>;
impl<'a, const O: u8> DIRQ5E_W<'a, O> {
    #[doc = "Canceling deep software standby mode is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ5E_A::_0)
    }
    #[doc = "Canceling deep software standby mode is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ5E_A::_1)
    }
}
#[doc = "Field `DIRQ6E` reader - IRQ6-DS Pin Enable"]
pub type DIRQ6E_R = crate::BitReader<DIRQ6E_A>;
#[doc = "IRQ6-DS Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ6E_A {
    #[doc = "0: Canceling deep software standby mode is disabled"]
    _0 = 0,
    #[doc = "1: Canceling deep software standby mode is enabled"]
    _1 = 1,
}
impl From<DIRQ6E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ6E_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ6E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ6E_A {
        match self.bits {
            false => DIRQ6E_A::_0,
            true => DIRQ6E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ6E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ6E_A::_1
    }
}
#[doc = "Field `DIRQ6E` writer - IRQ6-DS Pin Enable"]
pub type DIRQ6E_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIER0_SPEC, DIRQ6E_A, O>;
impl<'a, const O: u8> DIRQ6E_W<'a, O> {
    #[doc = "Canceling deep software standby mode is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ6E_A::_0)
    }
    #[doc = "Canceling deep software standby mode is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ6E_A::_1)
    }
}
#[doc = "Field `DIRQ7E` reader - IRQ7-DS Pin Enable"]
pub type DIRQ7E_R = crate::BitReader<DIRQ7E_A>;
#[doc = "IRQ7-DS Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ7E_A {
    #[doc = "0: Canceling deep software standby mode is disabled"]
    _0 = 0,
    #[doc = "1: Canceling deep software standby mode is enabled"]
    _1 = 1,
}
impl From<DIRQ7E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ7E_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ7E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ7E_A {
        match self.bits {
            false => DIRQ7E_A::_0,
            true => DIRQ7E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ7E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ7E_A::_1
    }
}
#[doc = "Field `DIRQ7E` writer - IRQ7-DS Pin Enable"]
pub type DIRQ7E_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIER0_SPEC, DIRQ7E_A, O>;
impl<'a, const O: u8> DIRQ7E_W<'a, O> {
    #[doc = "Canceling deep software standby mode is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ7E_A::_0)
    }
    #[doc = "Canceling deep software standby mode is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ7E_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - IRQ0-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq0e(&self) -> DIRQ0E_R {
        DIRQ0E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRQ1-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq1e(&self) -> DIRQ1E_R {
        DIRQ1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IRQ2-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq2e(&self) -> DIRQ2E_R {
        DIRQ2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IRQ3-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq3e(&self) -> DIRQ3E_R {
        DIRQ3E_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IRQ4-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq4e(&self) -> DIRQ4E_R {
        DIRQ4E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IRQ5-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq5e(&self) -> DIRQ5E_R {
        DIRQ5E_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IRQ6-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq6e(&self) -> DIRQ6E_R {
        DIRQ6E_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IRQ7-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq7e(&self) -> DIRQ7E_R {
        DIRQ7E_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRQ0-DS Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dirq0e(&mut self) -> DIRQ0E_W<0> {
        DIRQ0E_W::new(self)
    }
    #[doc = "Bit 1 - IRQ1-DS Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dirq1e(&mut self) -> DIRQ1E_W<1> {
        DIRQ1E_W::new(self)
    }
    #[doc = "Bit 2 - IRQ2-DS Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dirq2e(&mut self) -> DIRQ2E_W<2> {
        DIRQ2E_W::new(self)
    }
    #[doc = "Bit 3 - IRQ3-DS Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dirq3e(&mut self) -> DIRQ3E_W<3> {
        DIRQ3E_W::new(self)
    }
    #[doc = "Bit 4 - IRQ4-DS Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dirq4e(&mut self) -> DIRQ4E_W<4> {
        DIRQ4E_W::new(self)
    }
    #[doc = "Bit 5 - IRQ5-DS Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dirq5e(&mut self) -> DIRQ5E_W<5> {
        DIRQ5E_W::new(self)
    }
    #[doc = "Bit 6 - IRQ6-DS Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dirq6e(&mut self) -> DIRQ6E_W<6> {
        DIRQ6E_W::new(self)
    }
    #[doc = "Bit 7 - IRQ7-DS Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dirq7e(&mut self) -> DIRQ7E_W<7> {
        DIRQ7E_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep Standby Interrupt Enable Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpsier0](index.html) module"]
pub struct DPSIER0_SPEC;
impl crate::RegisterSpec for DPSIER0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dpsier0::R](R) reader structure"]
impl crate::Readable for DPSIER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpsier0::W](W) writer structure"]
impl crate::Writable for DPSIER0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPSIER0 to value 0"]
impl crate::Resettable for DPSIER0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
