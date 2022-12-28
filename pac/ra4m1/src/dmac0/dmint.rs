#[doc = "Register `DMINT` reader"]
pub struct R(crate::R<DMINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMINT` writer"]
pub struct W(crate::W<DMINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMINT_SPEC>;
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
impl From<crate::W<DMINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DARIE` reader - Destination Address Extended Repeat Area Overflow Interrupt Enable"]
pub type DARIE_R = crate::BitReader<DARIE_A>;
#[doc = "Destination Address Extended Repeat Area Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DARIE_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<DARIE_A> for bool {
    #[inline(always)]
    fn from(variant: DARIE_A) -> Self {
        variant as u8 != 0
    }
}
impl DARIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DARIE_A {
        match self.bits {
            false => DARIE_A::_0,
            true => DARIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DARIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DARIE_A::_1
    }
}
#[doc = "Field `DARIE` writer - Destination Address Extended Repeat Area Overflow Interrupt Enable"]
pub type DARIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, DMINT_SPEC, DARIE_A, O>;
impl<'a, const O: u8> DARIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DARIE_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DARIE_A::_1)
    }
}
#[doc = "Field `SARIE` reader - Source Address Extended Repeat Area Overflow Interrupt Enable"]
pub type SARIE_R = crate::BitReader<SARIE_A>;
#[doc = "Source Address Extended Repeat Area Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SARIE_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<SARIE_A> for bool {
    #[inline(always)]
    fn from(variant: SARIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SARIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SARIE_A {
        match self.bits {
            false => SARIE_A::_0,
            true => SARIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SARIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SARIE_A::_1
    }
}
#[doc = "Field `SARIE` writer - Source Address Extended Repeat Area Overflow Interrupt Enable"]
pub type SARIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, DMINT_SPEC, SARIE_A, O>;
impl<'a, const O: u8> SARIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SARIE_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SARIE_A::_1)
    }
}
#[doc = "Field `RPTIE` reader - Repeat Size End Interrupt Enable"]
pub type RPTIE_R = crate::BitReader<RPTIE_A>;
#[doc = "Repeat Size End Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPTIE_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<RPTIE_A> for bool {
    #[inline(always)]
    fn from(variant: RPTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RPTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPTIE_A {
        match self.bits {
            false => RPTIE_A::_0,
            true => RPTIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPTIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPTIE_A::_1
    }
}
#[doc = "Field `RPTIE` writer - Repeat Size End Interrupt Enable"]
pub type RPTIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, DMINT_SPEC, RPTIE_A, O>;
impl<'a, const O: u8> RPTIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RPTIE_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RPTIE_A::_1)
    }
}
#[doc = "Field `ESIE` reader - Transfer Escape End Interrupt Enable"]
pub type ESIE_R = crate::BitReader<ESIE_A>;
#[doc = "Transfer Escape End Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESIE_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<ESIE_A> for bool {
    #[inline(always)]
    fn from(variant: ESIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ESIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESIE_A {
        match self.bits {
            false => ESIE_A::_0,
            true => ESIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ESIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ESIE_A::_1
    }
}
#[doc = "Field `ESIE` writer - Transfer Escape End Interrupt Enable"]
pub type ESIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, DMINT_SPEC, ESIE_A, O>;
impl<'a, const O: u8> ESIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ESIE_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ESIE_A::_1)
    }
}
#[doc = "Field `DTIE` reader - Transfer End Interrupt Enable"]
pub type DTIE_R = crate::BitReader<DTIE_A>;
#[doc = "Transfer End Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTIE_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<DTIE_A> for bool {
    #[inline(always)]
    fn from(variant: DTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl DTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTIE_A {
        match self.bits {
            false => DTIE_A::_0,
            true => DTIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTIE_A::_1
    }
}
#[doc = "Field `DTIE` writer - Transfer End Interrupt Enable"]
pub type DTIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, DMINT_SPEC, DTIE_A, O>;
impl<'a, const O: u8> DTIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTIE_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTIE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Destination Address Extended Repeat Area Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn darie(&self) -> DARIE_R {
        DARIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Source Address Extended Repeat Area Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn sarie(&self) -> SARIE_R {
        SARIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Repeat Size End Interrupt Enable"]
    #[inline(always)]
    pub fn rptie(&self) -> RPTIE_R {
        RPTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transfer Escape End Interrupt Enable"]
    #[inline(always)]
    pub fn esie(&self) -> ESIE_R {
        ESIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transfer End Interrupt Enable"]
    #[inline(always)]
    pub fn dtie(&self) -> DTIE_R {
        DTIE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Destination Address Extended Repeat Area Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn darie(&mut self) -> DARIE_W<0> {
        DARIE_W::new(self)
    }
    #[doc = "Bit 1 - Source Address Extended Repeat Area Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sarie(&mut self) -> SARIE_W<1> {
        SARIE_W::new(self)
    }
    #[doc = "Bit 2 - Repeat Size End Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rptie(&mut self) -> RPTIE_W<2> {
        RPTIE_W::new(self)
    }
    #[doc = "Bit 3 - Transfer Escape End Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn esie(&mut self) -> ESIE_W<3> {
        ESIE_W::new(self)
    }
    #[doc = "Bit 4 - Transfer End Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtie(&mut self) -> DTIE_W<4> {
        DTIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Interrupt Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmint](index.html) module"]
pub struct DMINT_SPEC;
impl crate::RegisterSpec for DMINT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dmint::R](R) reader structure"]
impl crate::Readable for DMINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmint::W](W) writer structure"]
impl crate::Writable for DMINT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMINT to value 0"]
impl crate::Resettable for DMINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
