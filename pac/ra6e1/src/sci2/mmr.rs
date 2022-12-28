#[doc = "Register `MMR` reader"]
pub struct R(crate::R<MMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMR` writer"]
pub struct W(crate::W<MMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMR_SPEC>;
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
impl From<crate::W<MMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RMPOL` reader - Polarity of Received Manchester Code"]
pub type RMPOL_R = crate::BitReader<RMPOL_A>;
#[doc = "Polarity of Received Manchester Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMPOL_A {
    #[doc = "0: Logic 0 is coded as a zero-to-one transition in Manchester code Logic 1 is coded as a one-to-zero transition in Manchester code"]
    _0 = 0,
    #[doc = "1: Logic 0 is coded as a one-to-zero transition in Manchester code Logic 1 is coded as a zero-to-one transition in Manchester code"]
    _1 = 1,
}
impl From<RMPOL_A> for bool {
    #[inline(always)]
    fn from(variant: RMPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl RMPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMPOL_A {
        match self.bits {
            false => RMPOL_A::_0,
            true => RMPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RMPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RMPOL_A::_1
    }
}
#[doc = "Field `RMPOL` writer - Polarity of Received Manchester Code"]
pub type RMPOL_W<'a, const O: u8> = crate::BitWriter<'a, u8, MMR_SPEC, RMPOL_A, O>;
impl<'a, const O: u8> RMPOL_W<'a, O> {
    #[doc = "Logic 0 is coded as a zero-to-one transition in Manchester code Logic 1 is coded as a one-to-zero transition in Manchester code"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RMPOL_A::_0)
    }
    #[doc = "Logic 0 is coded as a one-to-zero transition in Manchester code Logic 1 is coded as a zero-to-one transition in Manchester code"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RMPOL_A::_1)
    }
}
#[doc = "Field `TMPOL` reader - Polarity of Transmit Manchester Code"]
pub type TMPOL_R = crate::BitReader<TMPOL_A>;
#[doc = "Polarity of Transmit Manchester Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMPOL_A {
    #[doc = "0: Logic 0 is coded as a zero-to-one transition in Manchester code Logic 1 is coded as a one-to-zero transition in Manchester code"]
    _0 = 0,
    #[doc = "1: Logic 0 is coded as a one-to-zero transition in Manchester code Logic 1 is coded as a zero-to-one transition in Manchester code"]
    _1 = 1,
}
impl From<TMPOL_A> for bool {
    #[inline(always)]
    fn from(variant: TMPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl TMPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMPOL_A {
        match self.bits {
            false => TMPOL_A::_0,
            true => TMPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMPOL_A::_1
    }
}
#[doc = "Field `TMPOL` writer - Polarity of Transmit Manchester Code"]
pub type TMPOL_W<'a, const O: u8> = crate::BitWriter<'a, u8, MMR_SPEC, TMPOL_A, O>;
impl<'a, const O: u8> TMPOL_W<'a, O> {
    #[doc = "Logic 0 is coded as a zero-to-one transition in Manchester code Logic 1 is coded as a one-to-zero transition in Manchester code"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMPOL_A::_0)
    }
    #[doc = "Logic 0 is coded as a one-to-zero transition in Manchester code Logic 1 is coded as a zero-to-one transition in Manchester code"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMPOL_A::_1)
    }
}
#[doc = "Field `ERTEN` reader - Manchester Edge Retiming Enable"]
pub type ERTEN_R = crate::BitReader<ERTEN_A>;
#[doc = "Manchester Edge Retiming Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERTEN_A {
    #[doc = "0: Disables the receive retiming function"]
    _0 = 0,
    #[doc = "1: Enables the receive retiming function"]
    _1 = 1,
}
impl From<ERTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ERTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ERTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERTEN_A {
        match self.bits {
            false => ERTEN_A::_0,
            true => ERTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERTEN_A::_1
    }
}
#[doc = "Field `ERTEN` writer - Manchester Edge Retiming Enable"]
pub type ERTEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, MMR_SPEC, ERTEN_A, O>;
impl<'a, const O: u8> ERTEN_W<'a, O> {
    #[doc = "Disables the receive retiming function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERTEN_A::_0)
    }
    #[doc = "Enables the receive retiming function"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERTEN_A::_1)
    }
}
#[doc = "Field `SYNVAL` reader - SYNC value Setting"]
pub type SYNVAL_R = crate::BitReader<SYNVAL_A>;
#[doc = "SYNC value Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNVAL_A {
    #[doc = "0: The start bit is added as a zero-to-one transition."]
    _0 = 0,
    #[doc = "1: The start bit is added as a one-to-zero transition."]
    _1 = 1,
}
impl From<SYNVAL_A> for bool {
    #[inline(always)]
    fn from(variant: SYNVAL_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNVAL_A {
        match self.bits {
            false => SYNVAL_A::_0,
            true => SYNVAL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNVAL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNVAL_A::_1
    }
}
#[doc = "Field `SYNVAL` writer - SYNC value Setting"]
pub type SYNVAL_W<'a, const O: u8> = crate::BitWriter<'a, u8, MMR_SPEC, SYNVAL_A, O>;
impl<'a, const O: u8> SYNVAL_W<'a, O> {
    #[doc = "The start bit is added as a zero-to-one transition."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNVAL_A::_0)
    }
    #[doc = "The start bit is added as a one-to-zero transition."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNVAL_A::_1)
    }
}
#[doc = "Field `SYNSEL` reader - SYNC Select"]
pub type SYNSEL_R = crate::BitReader<SYNSEL_A>;
#[doc = "SYNC Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNSEL_A {
    #[doc = "0: The start bit pattern is set with the SYNVAL bit"]
    _0 = 0,
    #[doc = "1: The start bit pattern is set with the TSYNC bit."]
    _1 = 1,
}
impl From<SYNSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SYNSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNSEL_A {
        match self.bits {
            false => SYNSEL_A::_0,
            true => SYNSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNSEL_A::_1
    }
}
#[doc = "Field `SYNSEL` writer - SYNC Select"]
pub type SYNSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, MMR_SPEC, SYNSEL_A, O>;
impl<'a, const O: u8> SYNSEL_W<'a, O> {
    #[doc = "The start bit pattern is set with the SYNVAL bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNSEL_A::_0)
    }
    #[doc = "The start bit pattern is set with the TSYNC bit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNSEL_A::_1)
    }
}
#[doc = "Field `SBSEL` reader - Start Bit Select"]
pub type SBSEL_R = crate::BitReader<SBSEL_A>;
#[doc = "Start Bit Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBSEL_A {
    #[doc = "0: The start bit area consists of one bit."]
    _0 = 0,
    #[doc = "1: The start bit area consists of three bits (COMMAND SYNC or DATA SYNC)"]
    _1 = 1,
}
impl From<SBSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SBSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SBSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBSEL_A {
        match self.bits {
            false => SBSEL_A::_0,
            true => SBSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBSEL_A::_1
    }
}
#[doc = "Field `SBSEL` writer - Start Bit Select"]
pub type SBSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, MMR_SPEC, SBSEL_A, O>;
impl<'a, const O: u8> SBSEL_W<'a, O> {
    #[doc = "The start bit area consists of one bit."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBSEL_A::_0)
    }
    #[doc = "The start bit area consists of three bits (COMMAND SYNC or DATA SYNC)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBSEL_A::_1)
    }
}
#[doc = "Field `MANEN` reader - Manchester Mode Enable"]
pub type MANEN_R = crate::BitReader<MANEN_A>;
#[doc = "Manchester Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MANEN_A {
    #[doc = "0: Disables the Manchester mode"]
    _0 = 0,
    #[doc = "1: Enables the Manchester mode"]
    _1 = 1,
}
impl From<MANEN_A> for bool {
    #[inline(always)]
    fn from(variant: MANEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MANEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MANEN_A {
        match self.bits {
            false => MANEN_A::_0,
            true => MANEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MANEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MANEN_A::_1
    }
}
#[doc = "Field `MANEN` writer - Manchester Mode Enable"]
pub type MANEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, MMR_SPEC, MANEN_A, O>;
impl<'a, const O: u8> MANEN_W<'a, O> {
    #[doc = "Disables the Manchester mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MANEN_A::_0)
    }
    #[doc = "Enables the Manchester mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MANEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Polarity of Received Manchester Code"]
    #[inline(always)]
    pub fn rmpol(&self) -> RMPOL_R {
        RMPOL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Polarity of Transmit Manchester Code"]
    #[inline(always)]
    pub fn tmpol(&self) -> TMPOL_R {
        TMPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Manchester Edge Retiming Enable"]
    #[inline(always)]
    pub fn erten(&self) -> ERTEN_R {
        ERTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - SYNC value Setting"]
    #[inline(always)]
    pub fn synval(&self) -> SYNVAL_R {
        SYNVAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SYNC Select"]
    #[inline(always)]
    pub fn synsel(&self) -> SYNSEL_R {
        SYNSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Start Bit Select"]
    #[inline(always)]
    pub fn sbsel(&self) -> SBSEL_R {
        SBSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Manchester Mode Enable"]
    #[inline(always)]
    pub fn manen(&self) -> MANEN_R {
        MANEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Polarity of Received Manchester Code"]
    #[inline(always)]
    #[must_use]
    pub fn rmpol(&mut self) -> RMPOL_W<0> {
        RMPOL_W::new(self)
    }
    #[doc = "Bit 1 - Polarity of Transmit Manchester Code"]
    #[inline(always)]
    #[must_use]
    pub fn tmpol(&mut self) -> TMPOL_W<1> {
        TMPOL_W::new(self)
    }
    #[doc = "Bit 2 - Manchester Edge Retiming Enable"]
    #[inline(always)]
    #[must_use]
    pub fn erten(&mut self) -> ERTEN_W<2> {
        ERTEN_W::new(self)
    }
    #[doc = "Bit 4 - SYNC value Setting"]
    #[inline(always)]
    #[must_use]
    pub fn synval(&mut self) -> SYNVAL_W<4> {
        SYNVAL_W::new(self)
    }
    #[doc = "Bit 5 - SYNC Select"]
    #[inline(always)]
    #[must_use]
    pub fn synsel(&mut self) -> SYNSEL_W<5> {
        SYNSEL_W::new(self)
    }
    #[doc = "Bit 6 - Start Bit Select"]
    #[inline(always)]
    #[must_use]
    pub fn sbsel(&mut self) -> SBSEL_W<6> {
        SBSEL_W::new(self)
    }
    #[doc = "Bit 7 - Manchester Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn manen(&mut self) -> MANEN_W<7> {
        MANEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Manchester Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmr](index.html) module"]
pub struct MMR_SPEC;
impl crate::RegisterSpec for MMR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mmr::R](R) reader structure"]
impl crate::Readable for MMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmr::W](W) writer structure"]
impl crate::Writable for MMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMR to value 0"]
impl crate::Resettable for MMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
