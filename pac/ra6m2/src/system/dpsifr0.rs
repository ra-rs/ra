#[doc = "Register `DPSIFR0` reader"]
pub struct R(crate::R<DPSIFR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPSIFR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPSIFR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPSIFR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPSIFR0` writer"]
pub struct W(crate::W<DPSIFR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPSIFR0_SPEC>;
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
impl From<crate::W<DPSIFR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPSIFR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIRQ0F` reader - IRQ0-DS Pin Deep Standby Cancel Flag\n\nThe field is **modified** in some way after a read operation."]
pub type DIRQ0F_R = crate::BitReader<DIRQ0F_A>;
#[doc = "IRQ0-DS Pin Deep Standby Cancel Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ0F_A {
    #[doc = "0: The cancel request is not generated"]
    _0 = 0,
    #[doc = "1: The cancel request is generated"]
    _1 = 1,
}
impl From<DIRQ0F_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ0F_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ0F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ0F_A {
        match self.bits {
            false => DIRQ0F_A::_0,
            true => DIRQ0F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ0F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ0F_A::_1
    }
}
#[doc = "Field `DIRQ0F` writer - IRQ0-DS Pin Deep Standby Cancel Flag"]
pub type DIRQ0F_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, DPSIFR0_SPEC, DIRQ0F_A, O>;
impl<'a, const O: u8> DIRQ0F_W<'a, O> {
    #[doc = "The cancel request is not generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ0F_A::_0)
    }
    #[doc = "The cancel request is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ0F_A::_1)
    }
}
#[doc = "Field `DIRQ1F` reader - IRQ1-DS Pin Deep Standby Cancel Flag\n\nThe field is **modified** in some way after a read operation."]
pub type DIRQ1F_R = crate::BitReader<DIRQ1F_A>;
#[doc = "IRQ1-DS Pin Deep Standby Cancel Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ1F_A {
    #[doc = "0: The cancel request is not generated"]
    _0 = 0,
    #[doc = "1: The cancel request is generated"]
    _1 = 1,
}
impl From<DIRQ1F_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ1F_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ1F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ1F_A {
        match self.bits {
            false => DIRQ1F_A::_0,
            true => DIRQ1F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ1F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ1F_A::_1
    }
}
#[doc = "Field `DIRQ1F` writer - IRQ1-DS Pin Deep Standby Cancel Flag"]
pub type DIRQ1F_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, DPSIFR0_SPEC, DIRQ1F_A, O>;
impl<'a, const O: u8> DIRQ1F_W<'a, O> {
    #[doc = "The cancel request is not generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ1F_A::_0)
    }
    #[doc = "The cancel request is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ1F_A::_1)
    }
}
#[doc = "Field `DIRQ2F` reader - IRQ2-DS Pin Deep Standby Cancel Flag\n\nThe field is **modified** in some way after a read operation."]
pub type DIRQ2F_R = crate::BitReader<DIRQ2F_A>;
#[doc = "IRQ2-DS Pin Deep Standby Cancel Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ2F_A {
    #[doc = "0: The cancel request is not generated"]
    _0 = 0,
    #[doc = "1: The cancel request is generated"]
    _1 = 1,
}
impl From<DIRQ2F_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ2F_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ2F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ2F_A {
        match self.bits {
            false => DIRQ2F_A::_0,
            true => DIRQ2F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ2F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ2F_A::_1
    }
}
#[doc = "Field `DIRQ2F` writer - IRQ2-DS Pin Deep Standby Cancel Flag"]
pub type DIRQ2F_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, DPSIFR0_SPEC, DIRQ2F_A, O>;
impl<'a, const O: u8> DIRQ2F_W<'a, O> {
    #[doc = "The cancel request is not generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ2F_A::_0)
    }
    #[doc = "The cancel request is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ2F_A::_1)
    }
}
#[doc = "Field `DIRQ3F` reader - IRQ3-DS Pin Deep Standby Cancel Flag\n\nThe field is **modified** in some way after a read operation."]
pub type DIRQ3F_R = crate::BitReader<DIRQ3F_A>;
#[doc = "IRQ3-DS Pin Deep Standby Cancel Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ3F_A {
    #[doc = "0: The cancel request is not generated"]
    _0 = 0,
    #[doc = "1: The cancel request is generated"]
    _1 = 1,
}
impl From<DIRQ3F_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ3F_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ3F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ3F_A {
        match self.bits {
            false => DIRQ3F_A::_0,
            true => DIRQ3F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ3F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ3F_A::_1
    }
}
#[doc = "Field `DIRQ3F` writer - IRQ3-DS Pin Deep Standby Cancel Flag"]
pub type DIRQ3F_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, DPSIFR0_SPEC, DIRQ3F_A, O>;
impl<'a, const O: u8> DIRQ3F_W<'a, O> {
    #[doc = "The cancel request is not generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ3F_A::_0)
    }
    #[doc = "The cancel request is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ3F_A::_1)
    }
}
#[doc = "Field `DIRQ4F` reader - IRQ4-DS Pin Deep Standby Cancel Flag\n\nThe field is **modified** in some way after a read operation."]
pub type DIRQ4F_R = crate::BitReader<DIRQ4F_A>;
#[doc = "IRQ4-DS Pin Deep Standby Cancel Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ4F_A {
    #[doc = "0: The cancel request is not generated"]
    _0 = 0,
    #[doc = "1: The cancel request is generated"]
    _1 = 1,
}
impl From<DIRQ4F_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ4F_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ4F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ4F_A {
        match self.bits {
            false => DIRQ4F_A::_0,
            true => DIRQ4F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ4F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ4F_A::_1
    }
}
#[doc = "Field `DIRQ4F` writer - IRQ4-DS Pin Deep Standby Cancel Flag"]
pub type DIRQ4F_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, DPSIFR0_SPEC, DIRQ4F_A, O>;
impl<'a, const O: u8> DIRQ4F_W<'a, O> {
    #[doc = "The cancel request is not generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ4F_A::_0)
    }
    #[doc = "The cancel request is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ4F_A::_1)
    }
}
#[doc = "Field `DIRQ5F` reader - IRQ5-DS Pin Deep Standby Cancel Flag\n\nThe field is **modified** in some way after a read operation."]
pub type DIRQ5F_R = crate::BitReader<DIRQ5F_A>;
#[doc = "IRQ5-DS Pin Deep Standby Cancel Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ5F_A {
    #[doc = "0: The cancel request is not generated"]
    _0 = 0,
    #[doc = "1: The cancel request is generated"]
    _1 = 1,
}
impl From<DIRQ5F_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ5F_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ5F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ5F_A {
        match self.bits {
            false => DIRQ5F_A::_0,
            true => DIRQ5F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ5F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ5F_A::_1
    }
}
#[doc = "Field `DIRQ5F` writer - IRQ5-DS Pin Deep Standby Cancel Flag"]
pub type DIRQ5F_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, DPSIFR0_SPEC, DIRQ5F_A, O>;
impl<'a, const O: u8> DIRQ5F_W<'a, O> {
    #[doc = "The cancel request is not generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ5F_A::_0)
    }
    #[doc = "The cancel request is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ5F_A::_1)
    }
}
#[doc = "Field `DIRQ6F` reader - IRQ6-DS Pin Deep Standby Cancel Flag\n\nThe field is **modified** in some way after a read operation."]
pub type DIRQ6F_R = crate::BitReader<DIRQ6F_A>;
#[doc = "IRQ6-DS Pin Deep Standby Cancel Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ6F_A {
    #[doc = "0: The cancel request is not generated"]
    _0 = 0,
    #[doc = "1: The cancel request is generated"]
    _1 = 1,
}
impl From<DIRQ6F_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ6F_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ6F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ6F_A {
        match self.bits {
            false => DIRQ6F_A::_0,
            true => DIRQ6F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ6F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ6F_A::_1
    }
}
#[doc = "Field `DIRQ6F` writer - IRQ6-DS Pin Deep Standby Cancel Flag"]
pub type DIRQ6F_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, DPSIFR0_SPEC, DIRQ6F_A, O>;
impl<'a, const O: u8> DIRQ6F_W<'a, O> {
    #[doc = "The cancel request is not generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ6F_A::_0)
    }
    #[doc = "The cancel request is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ6F_A::_1)
    }
}
#[doc = "Field `DIRQ7F` reader - IRQ7-DS Pin Deep Standby Cancel Flag\n\nThe field is **modified** in some way after a read operation."]
pub type DIRQ7F_R = crate::BitReader<DIRQ7F_A>;
#[doc = "IRQ7-DS Pin Deep Standby Cancel Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ7F_A {
    #[doc = "0: The cancel request is not generated"]
    _0 = 0,
    #[doc = "1: The cancel request is generated"]
    _1 = 1,
}
impl From<DIRQ7F_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ7F_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ7F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ7F_A {
        match self.bits {
            false => DIRQ7F_A::_0,
            true => DIRQ7F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ7F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ7F_A::_1
    }
}
#[doc = "Field `DIRQ7F` writer - IRQ7-DS Pin Deep Standby Cancel Flag"]
pub type DIRQ7F_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, DPSIFR0_SPEC, DIRQ7F_A, O>;
impl<'a, const O: u8> DIRQ7F_W<'a, O> {
    #[doc = "The cancel request is not generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ7F_A::_0)
    }
    #[doc = "The cancel request is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ7F_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - IRQ0-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq0f(&self) -> DIRQ0F_R {
        DIRQ0F_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRQ1-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq1f(&self) -> DIRQ1F_R {
        DIRQ1F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IRQ2-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq2f(&self) -> DIRQ2F_R {
        DIRQ2F_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IRQ3-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq3f(&self) -> DIRQ3F_R {
        DIRQ3F_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IRQ4-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq4f(&self) -> DIRQ4F_R {
        DIRQ4F_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IRQ5-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq5f(&self) -> DIRQ5F_R {
        DIRQ5F_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IRQ6-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq6f(&self) -> DIRQ6F_R {
        DIRQ6F_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IRQ7-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq7f(&self) -> DIRQ7F_R {
        DIRQ7F_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRQ0-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dirq0f(&mut self) -> DIRQ0F_W<0> {
        DIRQ0F_W::new(self)
    }
    #[doc = "Bit 1 - IRQ1-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dirq1f(&mut self) -> DIRQ1F_W<1> {
        DIRQ1F_W::new(self)
    }
    #[doc = "Bit 2 - IRQ2-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dirq2f(&mut self) -> DIRQ2F_W<2> {
        DIRQ2F_W::new(self)
    }
    #[doc = "Bit 3 - IRQ3-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dirq3f(&mut self) -> DIRQ3F_W<3> {
        DIRQ3F_W::new(self)
    }
    #[doc = "Bit 4 - IRQ4-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dirq4f(&mut self) -> DIRQ4F_W<4> {
        DIRQ4F_W::new(self)
    }
    #[doc = "Bit 5 - IRQ5-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dirq5f(&mut self) -> DIRQ5F_W<5> {
        DIRQ5F_W::new(self)
    }
    #[doc = "Bit 6 - IRQ6-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dirq6f(&mut self) -> DIRQ6F_W<6> {
        DIRQ6F_W::new(self)
    }
    #[doc = "Bit 7 - IRQ7-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dirq7f(&mut self) -> DIRQ7F_W<7> {
        DIRQ7F_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep Standby Interrupt Flag Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpsifr0](index.html) module"]
pub struct DPSIFR0_SPEC;
impl crate::RegisterSpec for DPSIFR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dpsifr0::R](R) reader structure"]
impl crate::Readable for DPSIFR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpsifr0::W](W) writer structure"]
impl crate::Writable for DPSIFR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xff;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPSIFR0 to value 0"]
impl crate::Resettable for DPSIFR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
