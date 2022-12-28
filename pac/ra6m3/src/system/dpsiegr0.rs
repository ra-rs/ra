#[doc = "Register `DPSIEGR0` reader"]
pub struct R(crate::R<DPSIEGR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPSIEGR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPSIEGR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPSIEGR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPSIEGR0` writer"]
pub struct W(crate::W<DPSIEGR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPSIEGR0_SPEC>;
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
impl From<crate::W<DPSIEGR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPSIEGR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIRQ0EG` reader - IRQ0-DS Pin Edge Select"]
pub type DIRQ0EG_R = crate::BitReader<DIRQ0EG_A>;
#[doc = "IRQ0-DS Pin Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ0EG_A {
    #[doc = "0: A cancel request is generated at a falling edge"]
    _0 = 0,
    #[doc = "1: A cancel request is generated at a rising edge"]
    _1 = 1,
}
impl From<DIRQ0EG_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ0EG_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ0EG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ0EG_A {
        match self.bits {
            false => DIRQ0EG_A::_0,
            true => DIRQ0EG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ0EG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ0EG_A::_1
    }
}
#[doc = "Field `DIRQ0EG` writer - IRQ0-DS Pin Edge Select"]
pub type DIRQ0EG_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIEGR0_SPEC, DIRQ0EG_A, O>;
impl<'a, const O: u8> DIRQ0EG_W<'a, O> {
    #[doc = "A cancel request is generated at a falling edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ0EG_A::_0)
    }
    #[doc = "A cancel request is generated at a rising edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ0EG_A::_1)
    }
}
#[doc = "Field `DIRQ1EG` reader - IRQ1-DS Pin Edge Select"]
pub type DIRQ1EG_R = crate::BitReader<DIRQ1EG_A>;
#[doc = "IRQ1-DS Pin Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ1EG_A {
    #[doc = "0: A cancel request is generated at a falling edge"]
    _0 = 0,
    #[doc = "1: A cancel request is generated at a rising edge"]
    _1 = 1,
}
impl From<DIRQ1EG_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ1EG_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ1EG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ1EG_A {
        match self.bits {
            false => DIRQ1EG_A::_0,
            true => DIRQ1EG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ1EG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ1EG_A::_1
    }
}
#[doc = "Field `DIRQ1EG` writer - IRQ1-DS Pin Edge Select"]
pub type DIRQ1EG_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIEGR0_SPEC, DIRQ1EG_A, O>;
impl<'a, const O: u8> DIRQ1EG_W<'a, O> {
    #[doc = "A cancel request is generated at a falling edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ1EG_A::_0)
    }
    #[doc = "A cancel request is generated at a rising edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ1EG_A::_1)
    }
}
#[doc = "Field `DIRQ2EG` reader - IRQ2-DS Pin Edge Select"]
pub type DIRQ2EG_R = crate::BitReader<DIRQ2EG_A>;
#[doc = "IRQ2-DS Pin Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ2EG_A {
    #[doc = "0: A cancel request is generated at a falling edge"]
    _0 = 0,
    #[doc = "1: A cancel request is generated at a rising edge"]
    _1 = 1,
}
impl From<DIRQ2EG_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ2EG_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ2EG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ2EG_A {
        match self.bits {
            false => DIRQ2EG_A::_0,
            true => DIRQ2EG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ2EG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ2EG_A::_1
    }
}
#[doc = "Field `DIRQ2EG` writer - IRQ2-DS Pin Edge Select"]
pub type DIRQ2EG_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIEGR0_SPEC, DIRQ2EG_A, O>;
impl<'a, const O: u8> DIRQ2EG_W<'a, O> {
    #[doc = "A cancel request is generated at a falling edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ2EG_A::_0)
    }
    #[doc = "A cancel request is generated at a rising edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ2EG_A::_1)
    }
}
#[doc = "Field `DIRQ3EG` reader - IRQ3-DS Pin Edge Select"]
pub type DIRQ3EG_R = crate::BitReader<DIRQ3EG_A>;
#[doc = "IRQ3-DS Pin Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ3EG_A {
    #[doc = "0: A cancel request is generated at a falling edge"]
    _0 = 0,
    #[doc = "1: A cancel request is generated at a rising edge"]
    _1 = 1,
}
impl From<DIRQ3EG_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ3EG_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ3EG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ3EG_A {
        match self.bits {
            false => DIRQ3EG_A::_0,
            true => DIRQ3EG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ3EG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ3EG_A::_1
    }
}
#[doc = "Field `DIRQ3EG` writer - IRQ3-DS Pin Edge Select"]
pub type DIRQ3EG_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIEGR0_SPEC, DIRQ3EG_A, O>;
impl<'a, const O: u8> DIRQ3EG_W<'a, O> {
    #[doc = "A cancel request is generated at a falling edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ3EG_A::_0)
    }
    #[doc = "A cancel request is generated at a rising edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ3EG_A::_1)
    }
}
#[doc = "Field `DIRQ4EG` reader - IRQ4-DS Pin Edge Select"]
pub type DIRQ4EG_R = crate::BitReader<DIRQ4EG_A>;
#[doc = "IRQ4-DS Pin Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ4EG_A {
    #[doc = "0: A cancel request is generated at a falling edge"]
    _0 = 0,
    #[doc = "1: A cancel request is generated at a rising edge"]
    _1 = 1,
}
impl From<DIRQ4EG_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ4EG_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ4EG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ4EG_A {
        match self.bits {
            false => DIRQ4EG_A::_0,
            true => DIRQ4EG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ4EG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ4EG_A::_1
    }
}
#[doc = "Field `DIRQ4EG` writer - IRQ4-DS Pin Edge Select"]
pub type DIRQ4EG_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIEGR0_SPEC, DIRQ4EG_A, O>;
impl<'a, const O: u8> DIRQ4EG_W<'a, O> {
    #[doc = "A cancel request is generated at a falling edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ4EG_A::_0)
    }
    #[doc = "A cancel request is generated at a rising edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ4EG_A::_1)
    }
}
#[doc = "Field `DIRQ5EG` reader - IRQ5-DS Pin Edge Select"]
pub type DIRQ5EG_R = crate::BitReader<DIRQ5EG_A>;
#[doc = "IRQ5-DS Pin Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ5EG_A {
    #[doc = "0: A cancel request is generated at a falling edge"]
    _0 = 0,
    #[doc = "1: A cancel request is generated at a rising edge"]
    _1 = 1,
}
impl From<DIRQ5EG_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ5EG_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ5EG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ5EG_A {
        match self.bits {
            false => DIRQ5EG_A::_0,
            true => DIRQ5EG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ5EG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ5EG_A::_1
    }
}
#[doc = "Field `DIRQ5EG` writer - IRQ5-DS Pin Edge Select"]
pub type DIRQ5EG_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIEGR0_SPEC, DIRQ5EG_A, O>;
impl<'a, const O: u8> DIRQ5EG_W<'a, O> {
    #[doc = "A cancel request is generated at a falling edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ5EG_A::_0)
    }
    #[doc = "A cancel request is generated at a rising edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ5EG_A::_1)
    }
}
#[doc = "Field `DIRQ6EG` reader - IRQ6-DS Pin Edge Select"]
pub type DIRQ6EG_R = crate::BitReader<DIRQ6EG_A>;
#[doc = "IRQ6-DS Pin Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ6EG_A {
    #[doc = "0: A cancel request is generated at a falling edge"]
    _0 = 0,
    #[doc = "1: A cancel request is generated at a rising edge"]
    _1 = 1,
}
impl From<DIRQ6EG_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ6EG_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ6EG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ6EG_A {
        match self.bits {
            false => DIRQ6EG_A::_0,
            true => DIRQ6EG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ6EG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ6EG_A::_1
    }
}
#[doc = "Field `DIRQ6EG` writer - IRQ6-DS Pin Edge Select"]
pub type DIRQ6EG_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIEGR0_SPEC, DIRQ6EG_A, O>;
impl<'a, const O: u8> DIRQ6EG_W<'a, O> {
    #[doc = "A cancel request is generated at a falling edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ6EG_A::_0)
    }
    #[doc = "A cancel request is generated at a rising edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ6EG_A::_1)
    }
}
#[doc = "Field `DIRQ7EG` reader - IRQ7-DS Pin Edge Select"]
pub type DIRQ7EG_R = crate::BitReader<DIRQ7EG_A>;
#[doc = "IRQ7-DS Pin Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ7EG_A {
    #[doc = "0: A cancel request is generated at a falling edge"]
    _0 = 0,
    #[doc = "1: A cancel request is generated at a rising edge"]
    _1 = 1,
}
impl From<DIRQ7EG_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ7EG_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ7EG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ7EG_A {
        match self.bits {
            false => DIRQ7EG_A::_0,
            true => DIRQ7EG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ7EG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ7EG_A::_1
    }
}
#[doc = "Field `DIRQ7EG` writer - IRQ7-DS Pin Edge Select"]
pub type DIRQ7EG_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIEGR0_SPEC, DIRQ7EG_A, O>;
impl<'a, const O: u8> DIRQ7EG_W<'a, O> {
    #[doc = "A cancel request is generated at a falling edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ7EG_A::_0)
    }
    #[doc = "A cancel request is generated at a rising edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ7EG_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - IRQ0-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq0eg(&self) -> DIRQ0EG_R {
        DIRQ0EG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRQ1-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq1eg(&self) -> DIRQ1EG_R {
        DIRQ1EG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IRQ2-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq2eg(&self) -> DIRQ2EG_R {
        DIRQ2EG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IRQ3-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq3eg(&self) -> DIRQ3EG_R {
        DIRQ3EG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IRQ4-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq4eg(&self) -> DIRQ4EG_R {
        DIRQ4EG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IRQ5-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq5eg(&self) -> DIRQ5EG_R {
        DIRQ5EG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IRQ6-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq6eg(&self) -> DIRQ6EG_R {
        DIRQ6EG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IRQ7-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq7eg(&self) -> DIRQ7EG_R {
        DIRQ7EG_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRQ0-DS Pin Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn dirq0eg(&mut self) -> DIRQ0EG_W<0> {
        DIRQ0EG_W::new(self)
    }
    #[doc = "Bit 1 - IRQ1-DS Pin Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn dirq1eg(&mut self) -> DIRQ1EG_W<1> {
        DIRQ1EG_W::new(self)
    }
    #[doc = "Bit 2 - IRQ2-DS Pin Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn dirq2eg(&mut self) -> DIRQ2EG_W<2> {
        DIRQ2EG_W::new(self)
    }
    #[doc = "Bit 3 - IRQ3-DS Pin Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn dirq3eg(&mut self) -> DIRQ3EG_W<3> {
        DIRQ3EG_W::new(self)
    }
    #[doc = "Bit 4 - IRQ4-DS Pin Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn dirq4eg(&mut self) -> DIRQ4EG_W<4> {
        DIRQ4EG_W::new(self)
    }
    #[doc = "Bit 5 - IRQ5-DS Pin Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn dirq5eg(&mut self) -> DIRQ5EG_W<5> {
        DIRQ5EG_W::new(self)
    }
    #[doc = "Bit 6 - IRQ6-DS Pin Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn dirq6eg(&mut self) -> DIRQ6EG_W<6> {
        DIRQ6EG_W::new(self)
    }
    #[doc = "Bit 7 - IRQ7-DS Pin Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn dirq7eg(&mut self) -> DIRQ7EG_W<7> {
        DIRQ7EG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep Standby Interrupt Edge Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpsiegr0](index.html) module"]
pub struct DPSIEGR0_SPEC;
impl crate::RegisterSpec for DPSIEGR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dpsiegr0::R](R) reader structure"]
impl crate::Readable for DPSIEGR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpsiegr0::W](W) writer structure"]
impl crate::Writable for DPSIEGR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPSIEGR0 to value 0"]
impl crate::Resettable for DPSIEGR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
