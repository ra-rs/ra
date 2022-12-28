#[doc = "Register `BSTE` reader"]
pub struct R(crate::R<BSTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BSTE` writer"]
pub struct W(crate::W<BSTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSTE_SPEC>;
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
impl From<crate::W<BSTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STCNDDE` reader - START Condition Detection Enable"]
pub type STCNDDE_R = crate::BitReader<STCNDDE_A>;
#[doc = "START Condition Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STCNDDE_A {
    #[doc = "0: Disables START condition Detection Interrupt Status logging."]
    _0 = 0,
    #[doc = "1: Enables START condition Detection Interrupt Status logging."]
    _1 = 1,
}
impl From<STCNDDE_A> for bool {
    #[inline(always)]
    fn from(variant: STCNDDE_A) -> Self {
        variant as u8 != 0
    }
}
impl STCNDDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STCNDDE_A {
        match self.bits {
            false => STCNDDE_A::_0,
            true => STCNDDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STCNDDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STCNDDE_A::_1
    }
}
#[doc = "Field `STCNDDE` writer - START Condition Detection Enable"]
pub type STCNDDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSTE_SPEC, STCNDDE_A, O>;
impl<'a, const O: u8> STCNDDE_W<'a, O> {
    #[doc = "Disables START condition Detection Interrupt Status logging."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STCNDDE_A::_0)
    }
    #[doc = "Enables START condition Detection Interrupt Status logging."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STCNDDE_A::_1)
    }
}
#[doc = "Field `SPCNDDE` reader - STOP Condition Detection Enable"]
pub type SPCNDDE_R = crate::BitReader<SPCNDDE_A>;
#[doc = "STOP Condition Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPCNDDE_A {
    #[doc = "0: Disables STOP condition Detection Interrupt Status logging."]
    _0 = 0,
    #[doc = "1: Enables STOP condition Detection Interrupt Status logging."]
    _1 = 1,
}
impl From<SPCNDDE_A> for bool {
    #[inline(always)]
    fn from(variant: SPCNDDE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPCNDDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPCNDDE_A {
        match self.bits {
            false => SPCNDDE_A::_0,
            true => SPCNDDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPCNDDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPCNDDE_A::_1
    }
}
#[doc = "Field `SPCNDDE` writer - STOP Condition Detection Enable"]
pub type SPCNDDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSTE_SPEC, SPCNDDE_A, O>;
impl<'a, const O: u8> SPCNDDE_W<'a, O> {
    #[doc = "Disables STOP condition Detection Interrupt Status logging."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPCNDDE_A::_0)
    }
    #[doc = "Enables STOP condition Detection Interrupt Status logging."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPCNDDE_A::_1)
    }
}
#[doc = "Field `HDREXDE` reader - HDR Exit Pattern Detection Enable"]
pub type HDREXDE_R = crate::BitReader<HDREXDE_A>;
#[doc = "HDR Exit Pattern Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDREXDE_A {
    #[doc = "0: Disables HDR Exit Pattern Detection Interrupt Status logging."]
    _0 = 0,
    #[doc = "1: Enables HDR Exit Pattern Detection Interrupt Status logging."]
    _1 = 1,
}
impl From<HDREXDE_A> for bool {
    #[inline(always)]
    fn from(variant: HDREXDE_A) -> Self {
        variant as u8 != 0
    }
}
impl HDREXDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDREXDE_A {
        match self.bits {
            false => HDREXDE_A::_0,
            true => HDREXDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HDREXDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HDREXDE_A::_1
    }
}
#[doc = "Field `HDREXDE` writer - HDR Exit Pattern Detection Enable"]
pub type HDREXDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSTE_SPEC, HDREXDE_A, O>;
impl<'a, const O: u8> HDREXDE_W<'a, O> {
    #[doc = "Disables HDR Exit Pattern Detection Interrupt Status logging."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HDREXDE_A::_0)
    }
    #[doc = "Enables HDR Exit Pattern Detection Interrupt Status logging."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HDREXDE_A::_1)
    }
}
#[doc = "Field `NACKDE` reader - NACK Detection Enable"]
pub type NACKDE_R = crate::BitReader<NACKDE_A>;
#[doc = "NACK Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKDE_A {
    #[doc = "0: Disables NACK Detection Interrupt Status logging."]
    _0 = 0,
    #[doc = "1: Enables NACK Detection Interrupt Status logging."]
    _1 = 1,
}
impl From<NACKDE_A> for bool {
    #[inline(always)]
    fn from(variant: NACKDE_A) -> Self {
        variant as u8 != 0
    }
}
impl NACKDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NACKDE_A {
        match self.bits {
            false => NACKDE_A::_0,
            true => NACKDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NACKDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NACKDE_A::_1
    }
}
#[doc = "Field `NACKDE` writer - NACK Detection Enable"]
pub type NACKDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSTE_SPEC, NACKDE_A, O>;
impl<'a, const O: u8> NACKDE_W<'a, O> {
    #[doc = "Disables NACK Detection Interrupt Status logging."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NACKDE_A::_0)
    }
    #[doc = "Enables NACK Detection Interrupt Status logging."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NACKDE_A::_1)
    }
}
#[doc = "Field `TENDE` reader - Transmit End Enable"]
pub type TENDE_R = crate::BitReader<TENDE_A>;
#[doc = "Transmit End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TENDE_A {
    #[doc = "0: Disables Transmit End Interrupt Status logging."]
    _0 = 0,
    #[doc = "1: Enables Transmit End Interrupt Status logging."]
    _1 = 1,
}
impl From<TENDE_A> for bool {
    #[inline(always)]
    fn from(variant: TENDE_A) -> Self {
        variant as u8 != 0
    }
}
impl TENDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TENDE_A {
        match self.bits {
            false => TENDE_A::_0,
            true => TENDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TENDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TENDE_A::_1
    }
}
#[doc = "Field `TENDE` writer - Transmit End Enable"]
pub type TENDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSTE_SPEC, TENDE_A, O>;
impl<'a, const O: u8> TENDE_W<'a, O> {
    #[doc = "Disables Transmit End Interrupt Status logging."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TENDE_A::_0)
    }
    #[doc = "Enables Transmit End Interrupt Status logging."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TENDE_A::_1)
    }
}
#[doc = "Field `ALE` reader - Arbitration Lost Enable"]
pub type ALE_R = crate::BitReader<ALE_A>;
#[doc = "Arbitration Lost Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALE_A {
    #[doc = "0: Disables Arbitration Lost Interrupt Status logging."]
    _0 = 0,
    #[doc = "1: Enables Arbitration Lost Interrupt Status logging."]
    _1 = 1,
}
impl From<ALE_A> for bool {
    #[inline(always)]
    fn from(variant: ALE_A) -> Self {
        variant as u8 != 0
    }
}
impl ALE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALE_A {
        match self.bits {
            false => ALE_A::_0,
            true => ALE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALE_A::_1
    }
}
#[doc = "Field `ALE` writer - Arbitration Lost Enable"]
pub type ALE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSTE_SPEC, ALE_A, O>;
impl<'a, const O: u8> ALE_W<'a, O> {
    #[doc = "Disables Arbitration Lost Interrupt Status logging."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALE_A::_0)
    }
    #[doc = "Enables Arbitration Lost Interrupt Status logging."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALE_A::_1)
    }
}
#[doc = "Field `TODE` reader - Timeout Detection Enable"]
pub type TODE_R = crate::BitReader<TODE_A>;
#[doc = "Timeout Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TODE_A {
    #[doc = "0: Disables Timeout Detection Interrupt Status logging."]
    _0 = 0,
    #[doc = "1: Enables Timeout Detection Interrupt Status logging."]
    _1 = 1,
}
impl From<TODE_A> for bool {
    #[inline(always)]
    fn from(variant: TODE_A) -> Self {
        variant as u8 != 0
    }
}
impl TODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TODE_A {
        match self.bits {
            false => TODE_A::_0,
            true => TODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TODE_A::_1
    }
}
#[doc = "Field `TODE` writer - Timeout Detection Enable"]
pub type TODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSTE_SPEC, TODE_A, O>;
impl<'a, const O: u8> TODE_W<'a, O> {
    #[doc = "Disables Timeout Detection Interrupt Status logging."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TODE_A::_0)
    }
    #[doc = "Enables Timeout Detection Interrupt Status logging."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TODE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - START Condition Detection Enable"]
    #[inline(always)]
    pub fn stcndde(&self) -> STCNDDE_R {
        STCNDDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - STOP Condition Detection Enable"]
    #[inline(always)]
    pub fn spcndde(&self) -> SPCNDDE_R {
        SPCNDDE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HDR Exit Pattern Detection Enable"]
    #[inline(always)]
    pub fn hdrexde(&self) -> HDREXDE_R {
        HDREXDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - NACK Detection Enable"]
    #[inline(always)]
    pub fn nackde(&self) -> NACKDE_R {
        NACKDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit End Enable"]
    #[inline(always)]
    pub fn tende(&self) -> TENDE_R {
        TENDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Arbitration Lost Enable"]
    #[inline(always)]
    pub fn ale(&self) -> ALE_R {
        ALE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Timeout Detection Enable"]
    #[inline(always)]
    pub fn tode(&self) -> TODE_R {
        TODE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - START Condition Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stcndde(&mut self) -> STCNDDE_W<0> {
        STCNDDE_W::new(self)
    }
    #[doc = "Bit 1 - STOP Condition Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spcndde(&mut self) -> SPCNDDE_W<1> {
        SPCNDDE_W::new(self)
    }
    #[doc = "Bit 2 - HDR Exit Pattern Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hdrexde(&mut self) -> HDREXDE_W<2> {
        HDREXDE_W::new(self)
    }
    #[doc = "Bit 4 - NACK Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nackde(&mut self) -> NACKDE_W<4> {
        NACKDE_W::new(self)
    }
    #[doc = "Bit 8 - Transmit End Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tende(&mut self) -> TENDE_W<8> {
        TENDE_W::new(self)
    }
    #[doc = "Bit 16 - Arbitration Lost Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ale(&mut self) -> ALE_W<16> {
        ALE_W::new(self)
    }
    #[doc = "Bit 20 - Timeout Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tode(&mut self) -> TODE_W<20> {
        TODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus Status Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bste](index.html) module"]
pub struct BSTE_SPEC;
impl crate::RegisterSpec for BSTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bste::R](R) reader structure"]
impl crate::Readable for BSTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bste::W](W) writer structure"]
impl crate::Writable for BSTE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BSTE to value 0"]
impl crate::Resettable for BSTE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
