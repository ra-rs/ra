#[doc = "Register `MECR` reader"]
pub struct R(crate::R<MECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MECR` writer"]
pub struct W(crate::W<MECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MECR_SPEC>;
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
impl From<crate::W<MECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PFEREN` reader - Preface Error Enable"]
pub type PFEREN_R = crate::BitReader<PFEREN_A>;
#[doc = "Preface Error Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFEREN_A {
    #[doc = "0: Does not handle a preface error as an interrupt source"]
    _0 = 0,
    #[doc = "1: Handles a preface error as an interrupt source"]
    _1 = 1,
}
impl From<PFEREN_A> for bool {
    #[inline(always)]
    fn from(variant: PFEREN_A) -> Self {
        variant as u8 != 0
    }
}
impl PFEREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFEREN_A {
        match self.bits {
            false => PFEREN_A::_0,
            true => PFEREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PFEREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PFEREN_A::_1
    }
}
#[doc = "Field `PFEREN` writer - Preface Error Enable"]
pub type PFEREN_W<'a, const O: u8> = crate::BitWriter<'a, u8, MECR_SPEC, PFEREN_A, O>;
impl<'a, const O: u8> PFEREN_W<'a, O> {
    #[doc = "Does not handle a preface error as an interrupt source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PFEREN_A::_0)
    }
    #[doc = "Handles a preface error as an interrupt source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PFEREN_A::_1)
    }
}
#[doc = "Field `SYEREN` reader - Receive SYNC Error Enable"]
pub type SYEREN_R = crate::BitReader<SYEREN_A>;
#[doc = "Receive SYNC Error Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYEREN_A {
    #[doc = "0: Does not handle a receive SYNC error as an interrupt source"]
    _0 = 0,
    #[doc = "1: Handles a receive SYNC error as an interrupt source"]
    _1 = 1,
}
impl From<SYEREN_A> for bool {
    #[inline(always)]
    fn from(variant: SYEREN_A) -> Self {
        variant as u8 != 0
    }
}
impl SYEREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYEREN_A {
        match self.bits {
            false => SYEREN_A::_0,
            true => SYEREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYEREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYEREN_A::_1
    }
}
#[doc = "Field `SYEREN` writer - Receive SYNC Error Enable"]
pub type SYEREN_W<'a, const O: u8> = crate::BitWriter<'a, u8, MECR_SPEC, SYEREN_A, O>;
impl<'a, const O: u8> SYEREN_W<'a, O> {
    #[doc = "Does not handle a receive SYNC error as an interrupt source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYEREN_A::_0)
    }
    #[doc = "Handles a receive SYNC error as an interrupt source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYEREN_A::_1)
    }
}
#[doc = "Field `SBEREN` reader - Start Bit Error Enable"]
pub type SBEREN_R = crate::BitReader<SBEREN_A>;
#[doc = "Start Bit Error Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBEREN_A {
    #[doc = "0: Does not handle a start bit error as an interrupt source"]
    _0 = 0,
    #[doc = "1: Handles a start bit error as an interrupt source"]
    _1 = 1,
}
impl From<SBEREN_A> for bool {
    #[inline(always)]
    fn from(variant: SBEREN_A) -> Self {
        variant as u8 != 0
    }
}
impl SBEREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBEREN_A {
        match self.bits {
            false => SBEREN_A::_0,
            true => SBEREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBEREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBEREN_A::_1
    }
}
#[doc = "Field `SBEREN` writer - Start Bit Error Enable"]
pub type SBEREN_W<'a, const O: u8> = crate::BitWriter<'a, u8, MECR_SPEC, SBEREN_A, O>;
impl<'a, const O: u8> SBEREN_W<'a, O> {
    #[doc = "Does not handle a start bit error as an interrupt source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBEREN_A::_0)
    }
    #[doc = "Handles a start bit error as an interrupt source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBEREN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Preface Error Enable"]
    #[inline(always)]
    pub fn pferen(&self) -> PFEREN_R {
        PFEREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive SYNC Error Enable"]
    #[inline(always)]
    pub fn syeren(&self) -> SYEREN_R {
        SYEREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start Bit Error Enable"]
    #[inline(always)]
    pub fn sberen(&self) -> SBEREN_R {
        SBEREN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Preface Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pferen(&mut self) -> PFEREN_W<0> {
        PFEREN_W::new(self)
    }
    #[doc = "Bit 1 - Receive SYNC Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn syeren(&mut self) -> SYEREN_W<1> {
        SYEREN_W::new(self)
    }
    #[doc = "Bit 2 - Start Bit Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sberen(&mut self) -> SBEREN_W<2> {
        SBEREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Manchester Extended Error Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mecr](index.html) module"]
pub struct MECR_SPEC;
impl crate::RegisterSpec for MECR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mecr::R](R) reader structure"]
impl crate::Readable for MECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mecr::W](W) writer structure"]
impl crate::Writable for MECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MECR to value 0"]
impl crate::Resettable for MECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
