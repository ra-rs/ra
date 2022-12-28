#[doc = "Register `CNDCTL` reader"]
pub struct R(crate::R<CNDCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNDCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNDCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNDCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNDCTL` writer"]
pub struct W(crate::W<CNDCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNDCTL_SPEC>;
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
impl From<crate::W<CNDCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNDCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STCND` reader - START (S) Condition Issuance"]
pub type STCND_R = crate::BitReader<STCND_A>;
#[doc = "START (S) Condition Issuance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STCND_A {
    #[doc = "0: Does not request to issue a START condition."]
    _0 = 0,
    #[doc = "1: Requests to issue a START condition."]
    _1 = 1,
}
impl From<STCND_A> for bool {
    #[inline(always)]
    fn from(variant: STCND_A) -> Self {
        variant as u8 != 0
    }
}
impl STCND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STCND_A {
        match self.bits {
            false => STCND_A::_0,
            true => STCND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STCND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STCND_A::_1
    }
}
#[doc = "Field `STCND` writer - START (S) Condition Issuance"]
pub type STCND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNDCTL_SPEC, STCND_A, O>;
impl<'a, const O: u8> STCND_W<'a, O> {
    #[doc = "Does not request to issue a START condition."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STCND_A::_0)
    }
    #[doc = "Requests to issue a START condition."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STCND_A::_1)
    }
}
#[doc = "Field `SRCND` reader - Repeated START (Sr) Condition Issuance"]
pub type SRCND_R = crate::BitReader<SRCND_A>;
#[doc = "Repeated START (Sr) Condition Issuance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRCND_A {
    #[doc = "0: Does not request to issue a Repeated START condition."]
    _0 = 0,
    #[doc = "1: Requests to issue a Repeated START condition."]
    _1 = 1,
}
impl From<SRCND_A> for bool {
    #[inline(always)]
    fn from(variant: SRCND_A) -> Self {
        variant as u8 != 0
    }
}
impl SRCND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRCND_A {
        match self.bits {
            false => SRCND_A::_0,
            true => SRCND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRCND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRCND_A::_1
    }
}
#[doc = "Field `SRCND` writer - Repeated START (Sr) Condition Issuance"]
pub type SRCND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNDCTL_SPEC, SRCND_A, O>;
impl<'a, const O: u8> SRCND_W<'a, O> {
    #[doc = "Does not request to issue a Repeated START condition."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRCND_A::_0)
    }
    #[doc = "Requests to issue a Repeated START condition."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRCND_A::_1)
    }
}
#[doc = "Field `SPCND` reader - STOP (P) Condition Issuance"]
pub type SPCND_R = crate::BitReader<SPCND_A>;
#[doc = "STOP (P) Condition Issuance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPCND_A {
    #[doc = "0: Does not request to issue a STOP condition."]
    _0 = 0,
    #[doc = "1: Requests to issue a STOP condition."]
    _1 = 1,
}
impl From<SPCND_A> for bool {
    #[inline(always)]
    fn from(variant: SPCND_A) -> Self {
        variant as u8 != 0
    }
}
impl SPCND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPCND_A {
        match self.bits {
            false => SPCND_A::_0,
            true => SPCND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPCND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPCND_A::_1
    }
}
#[doc = "Field `SPCND` writer - STOP (P) Condition Issuance"]
pub type SPCND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNDCTL_SPEC, SPCND_A, O>;
impl<'a, const O: u8> SPCND_W<'a, O> {
    #[doc = "Does not request to issue a STOP condition."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPCND_A::_0)
    }
    #[doc = "Requests to issue a STOP condition."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPCND_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - START (S) Condition Issuance"]
    #[inline(always)]
    pub fn stcnd(&self) -> STCND_R {
        STCND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Repeated START (Sr) Condition Issuance"]
    #[inline(always)]
    pub fn srcnd(&self) -> SRCND_R {
        SRCND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - STOP (P) Condition Issuance"]
    #[inline(always)]
    pub fn spcnd(&self) -> SPCND_R {
        SPCND_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - START (S) Condition Issuance"]
    #[inline(always)]
    #[must_use]
    pub fn stcnd(&mut self) -> STCND_W<0> {
        STCND_W::new(self)
    }
    #[doc = "Bit 1 - Repeated START (Sr) Condition Issuance"]
    #[inline(always)]
    #[must_use]
    pub fn srcnd(&mut self) -> SRCND_W<1> {
        SRCND_W::new(self)
    }
    #[doc = "Bit 2 - STOP (P) Condition Issuance"]
    #[inline(always)]
    #[must_use]
    pub fn spcnd(&mut self) -> SPCND_W<2> {
        SPCND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Condition Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndctl](index.html) module"]
pub struct CNDCTL_SPEC;
impl crate::RegisterSpec for CNDCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cndctl::R](R) reader structure"]
impl crate::Readable for CNDCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cndctl::W](W) writer structure"]
impl crate::Writable for CNDCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNDCTL to value 0"]
impl crate::Resettable for CNDCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
