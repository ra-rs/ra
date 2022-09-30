#[doc = "Register `CAICR` reader"]
pub struct R(crate::R<CAICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAICR` writer"]
pub struct W(crate::W<CAICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAICR_SPEC>;
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
impl From<crate::W<CAICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FERRIE` reader - Frequency Error Interrupt Request Enable"]
pub type FERRIE_R = crate::BitReader<FERRIE_A>;
#[doc = "Frequency Error Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FERRIE_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<FERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: FERRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl FERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FERRIE_A {
        match self.bits {
            false => FERRIE_A::_0,
            true => FERRIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FERRIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FERRIE_A::_1
    }
}
#[doc = "Field `FERRIE` writer - Frequency Error Interrupt Request Enable"]
pub type FERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CAICR_SPEC, FERRIE_A, O>;
impl<'a, const O: u8> FERRIE_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FERRIE_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FERRIE_A::_1)
    }
}
#[doc = "Field `MENDIE` reader - Measurement End Interrupt Request Enable"]
pub type MENDIE_R = crate::BitReader<MENDIE_A>;
#[doc = "Measurement End Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MENDIE_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<MENDIE_A> for bool {
    #[inline(always)]
    fn from(variant: MENDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl MENDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MENDIE_A {
        match self.bits {
            false => MENDIE_A::_0,
            true => MENDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MENDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MENDIE_A::_1
    }
}
#[doc = "Field `MENDIE` writer - Measurement End Interrupt Request Enable"]
pub type MENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CAICR_SPEC, MENDIE_A, O>;
impl<'a, const O: u8> MENDIE_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MENDIE_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MENDIE_A::_1)
    }
}
#[doc = "Field `OVFIE` reader - Overflow Interrupt Request Enable"]
pub type OVFIE_R = crate::BitReader<OVFIE_A>;
#[doc = "Overflow Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVFIE_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<OVFIE_A> for bool {
    #[inline(always)]
    fn from(variant: OVFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl OVFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVFIE_A {
        match self.bits {
            false => OVFIE_A::_0,
            true => OVFIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVFIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVFIE_A::_1
    }
}
#[doc = "Field `OVFIE` writer - Overflow Interrupt Request Enable"]
pub type OVFIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CAICR_SPEC, OVFIE_A, O>;
impl<'a, const O: u8> OVFIE_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFIE_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFIE_A::_1)
    }
}
#[doc = "FERRF Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FERRFCL_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: The CASTR.FERRF flag is cleared"]
    _1 = 1,
}
impl From<FERRFCL_AW> for bool {
    #[inline(always)]
    fn from(variant: FERRFCL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FERRFCL` writer - FERRF Clear"]
pub type FERRFCL_W<'a, const O: u8> = crate::BitWriter<'a, u8, CAICR_SPEC, FERRFCL_AW, O>;
impl<'a, const O: u8> FERRFCL_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FERRFCL_AW::_0)
    }
    #[doc = "The CASTR.FERRF flag is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FERRFCL_AW::_1)
    }
}
#[doc = "MENDF Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MENDFCL_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: The CASTR.MENDF flag is cleared"]
    _1 = 1,
}
impl From<MENDFCL_AW> for bool {
    #[inline(always)]
    fn from(variant: MENDFCL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MENDFCL` writer - MENDF Clear"]
pub type MENDFCL_W<'a, const O: u8> = crate::BitWriter<'a, u8, CAICR_SPEC, MENDFCL_AW, O>;
impl<'a, const O: u8> MENDFCL_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MENDFCL_AW::_0)
    }
    #[doc = "The CASTR.MENDF flag is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MENDFCL_AW::_1)
    }
}
#[doc = "OVFF Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVFFCL_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: The CASTR.OVFF flag is cleared."]
    _1 = 1,
}
impl From<OVFFCL_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFFCL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFFCL` writer - OVFF Clear"]
pub type OVFFCL_W<'a, const O: u8> = crate::BitWriter<'a, u8, CAICR_SPEC, OVFFCL_AW, O>;
impl<'a, const O: u8> OVFFCL_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFFCL_AW::_0)
    }
    #[doc = "The CASTR.OVFF flag is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFFCL_AW::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Frequency Error Interrupt Request Enable"]
    #[inline(always)]
    pub fn ferrie(&self) -> FERRIE_R {
        FERRIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Measurement End Interrupt Request Enable"]
    #[inline(always)]
    pub fn mendie(&self) -> MENDIE_R {
        MENDIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow Interrupt Request Enable"]
    #[inline(always)]
    pub fn ovfie(&self) -> OVFIE_R {
        OVFIE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frequency Error Interrupt Request Enable"]
    #[inline(always)]
    pub fn ferrie(&mut self) -> FERRIE_W<0> {
        FERRIE_W::new(self)
    }
    #[doc = "Bit 1 - Measurement End Interrupt Request Enable"]
    #[inline(always)]
    pub fn mendie(&mut self) -> MENDIE_W<1> {
        MENDIE_W::new(self)
    }
    #[doc = "Bit 2 - Overflow Interrupt Request Enable"]
    #[inline(always)]
    pub fn ovfie(&mut self) -> OVFIE_W<2> {
        OVFIE_W::new(self)
    }
    #[doc = "Bit 4 - FERRF Clear"]
    #[inline(always)]
    pub fn ferrfcl(&mut self) -> FERRFCL_W<4> {
        FERRFCL_W::new(self)
    }
    #[doc = "Bit 5 - MENDF Clear"]
    #[inline(always)]
    pub fn mendfcl(&mut self) -> MENDFCL_W<5> {
        MENDFCL_W::new(self)
    }
    #[doc = "Bit 6 - OVFF Clear"]
    #[inline(always)]
    pub fn ovffcl(&mut self) -> OVFFCL_W<6> {
        OVFFCL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAC Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [caicr](index.html) module"]
pub struct CAICR_SPEC;
impl crate::RegisterSpec for CAICR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [caicr::R](R) reader structure"]
impl crate::Readable for CAICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [caicr::W](W) writer structure"]
impl crate::Writable for CAICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAICR to value 0"]
impl crate::Resettable for CAICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
