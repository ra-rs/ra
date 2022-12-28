#[doc = "Register `LDOSCR` reader"]
pub struct R(crate::R<LDOSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDOSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDOSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDOSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LDOSCR` writer"]
pub struct W(crate::W<LDOSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDOSCR_SPEC>;
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
impl From<crate::W<LDOSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDOSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LDOSTP0` reader - LDO0 Stop"]
pub type LDOSTP0_R = crate::BitReader<LDOSTP0_A>;
#[doc = "LDO0 Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDOSTP0_A {
    #[doc = "0: LDO0 is enabled"]
    _0 = 0,
    #[doc = "1: LDO0 is stopped"]
    _1 = 1,
}
impl From<LDOSTP0_A> for bool {
    #[inline(always)]
    fn from(variant: LDOSTP0_A) -> Self {
        variant as u8 != 0
    }
}
impl LDOSTP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDOSTP0_A {
        match self.bits {
            false => LDOSTP0_A::_0,
            true => LDOSTP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LDOSTP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LDOSTP0_A::_1
    }
}
#[doc = "Field `LDOSTP0` writer - LDO0 Stop"]
pub type LDOSTP0_W<'a, const O: u8> = crate::BitWriter<'a, u8, LDOSCR_SPEC, LDOSTP0_A, O>;
impl<'a, const O: u8> LDOSTP0_W<'a, O> {
    #[doc = "LDO0 is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LDOSTP0_A::_0)
    }
    #[doc = "LDO0 is stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LDOSTP0_A::_1)
    }
}
#[doc = "Field `LDOSTP1` reader - LDO1 Stop"]
pub type LDOSTP1_R = crate::BitReader<LDOSTP1_A>;
#[doc = "LDO1 Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDOSTP1_A {
    #[doc = "0: LDO1 is enabled"]
    _0 = 0,
    #[doc = "1: LDO1 is stopped"]
    _1 = 1,
}
impl From<LDOSTP1_A> for bool {
    #[inline(always)]
    fn from(variant: LDOSTP1_A) -> Self {
        variant as u8 != 0
    }
}
impl LDOSTP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDOSTP1_A {
        match self.bits {
            false => LDOSTP1_A::_0,
            true => LDOSTP1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LDOSTP1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LDOSTP1_A::_1
    }
}
#[doc = "Field `LDOSTP1` writer - LDO1 Stop"]
pub type LDOSTP1_W<'a, const O: u8> = crate::BitWriter<'a, u8, LDOSCR_SPEC, LDOSTP1_A, O>;
impl<'a, const O: u8> LDOSTP1_W<'a, O> {
    #[doc = "LDO1 is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LDOSTP1_A::_0)
    }
    #[doc = "LDO1 is stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LDOSTP1_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - LDO0 Stop"]
    #[inline(always)]
    pub fn ldostp0(&self) -> LDOSTP0_R {
        LDOSTP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LDO1 Stop"]
    #[inline(always)]
    pub fn ldostp1(&self) -> LDOSTP1_R {
        LDOSTP1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LDO0 Stop"]
    #[inline(always)]
    #[must_use]
    pub fn ldostp0(&mut self) -> LDOSTP0_W<0> {
        LDOSTP0_W::new(self)
    }
    #[doc = "Bit 1 - LDO1 Stop"]
    #[inline(always)]
    #[must_use]
    pub fn ldostp1(&mut self) -> LDOSTP1_W<1> {
        LDOSTP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LDO Stop Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldoscr](index.html) module"]
pub struct LDOSCR_SPEC;
impl crate::RegisterSpec for LDOSCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ldoscr::R](R) reader structure"]
impl crate::Readable for LDOSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldoscr::W](W) writer structure"]
impl crate::Writable for LDOSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LDOSCR to value 0"]
impl crate::Resettable for LDOSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
