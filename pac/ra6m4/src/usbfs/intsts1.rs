#[doc = "Register `INTSTS1` reader"]
pub struct R(crate::R<INTSTS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSTS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSTS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTSTS1` writer"]
pub struct W(crate::W<INTSTS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTSTS1_SPEC>;
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
impl From<crate::W<INTSTS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTSTS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDDETINT` reader - PDDET Detection Interrupt Status Flag"]
pub type PDDETINT_R = crate::BitReader<PDDETINT_A>;
#[doc = "PDDET Detection Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDDETINT_A {
    #[doc = "0: No PDDET interrupt occurred"]
    _0 = 0,
    #[doc = "1: PDDET interrupt occurred"]
    _1 = 1,
}
impl From<PDDETINT_A> for bool {
    #[inline(always)]
    fn from(variant: PDDETINT_A) -> Self {
        variant as u8 != 0
    }
}
impl PDDETINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDDETINT_A {
        match self.bits {
            false => PDDETINT_A::_0,
            true => PDDETINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDDETINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDDETINT_A::_1
    }
}
#[doc = "Field `PDDETINT` writer - PDDET Detection Interrupt Status Flag"]
pub type PDDETINT_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTSTS1_SPEC, PDDETINT_A, O>;
impl<'a, const O: u8> PDDETINT_W<'a, O> {
    #[doc = "No PDDET interrupt occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDDETINT_A::_0)
    }
    #[doc = "PDDET interrupt occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDDETINT_A::_1)
    }
}
#[doc = "Field `SACK` reader - Setup Transaction Normal Response Interrupt Status"]
pub type SACK_R = crate::BitReader<SACK_A>;
#[doc = "Setup Transaction Normal Response Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SACK_A {
    #[doc = "0: No SACK interrupt occurred"]
    _0 = 0,
    #[doc = "1: SACK interrupt occurred"]
    _1 = 1,
}
impl From<SACK_A> for bool {
    #[inline(always)]
    fn from(variant: SACK_A) -> Self {
        variant as u8 != 0
    }
}
impl SACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SACK_A {
        match self.bits {
            false => SACK_A::_0,
            true => SACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SACK_A::_1
    }
}
#[doc = "Field `SACK` writer - Setup Transaction Normal Response Interrupt Status"]
pub type SACK_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTSTS1_SPEC, SACK_A, O>;
impl<'a, const O: u8> SACK_W<'a, O> {
    #[doc = "No SACK interrupt occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SACK_A::_0)
    }
    #[doc = "SACK interrupt occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SACK_A::_1)
    }
}
#[doc = "Field `SIGN` reader - Setup Transaction Error Interrupt Status"]
pub type SIGN_R = crate::BitReader<SIGN_A>;
#[doc = "Setup Transaction Error Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIGN_A {
    #[doc = "0: No SIGN interrupt occurred"]
    _0 = 0,
    #[doc = "1: SIGN interrupt occurred"]
    _1 = 1,
}
impl From<SIGN_A> for bool {
    #[inline(always)]
    fn from(variant: SIGN_A) -> Self {
        variant as u8 != 0
    }
}
impl SIGN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIGN_A {
        match self.bits {
            false => SIGN_A::_0,
            true => SIGN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SIGN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SIGN_A::_1
    }
}
#[doc = "Field `SIGN` writer - Setup Transaction Error Interrupt Status"]
pub type SIGN_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTSTS1_SPEC, SIGN_A, O>;
impl<'a, const O: u8> SIGN_W<'a, O> {
    #[doc = "No SIGN interrupt occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SIGN_A::_0)
    }
    #[doc = "SIGN interrupt occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SIGN_A::_1)
    }
}
#[doc = "Field `EOFERR` reader - EOF Error Detection Interrupt Status"]
pub type EOFERR_R = crate::BitReader<EOFERR_A>;
#[doc = "EOF Error Detection Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOFERR_A {
    #[doc = "0: No EOFERR interrupt occurred"]
    _0 = 0,
    #[doc = "1: EOFERR interrupt occurred"]
    _1 = 1,
}
impl From<EOFERR_A> for bool {
    #[inline(always)]
    fn from(variant: EOFERR_A) -> Self {
        variant as u8 != 0
    }
}
impl EOFERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOFERR_A {
        match self.bits {
            false => EOFERR_A::_0,
            true => EOFERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOFERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOFERR_A::_1
    }
}
#[doc = "Field `EOFERR` writer - EOF Error Detection Interrupt Status"]
pub type EOFERR_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTSTS1_SPEC, EOFERR_A, O>;
impl<'a, const O: u8> EOFERR_W<'a, O> {
    #[doc = "No EOFERR interrupt occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOFERR_A::_0)
    }
    #[doc = "EOFERR interrupt occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOFERR_A::_1)
    }
}
#[doc = "Field `ATTCH` reader - ATTCH Interrupt Status"]
pub type ATTCH_R = crate::BitReader<ATTCH_A>;
#[doc = "ATTCH Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ATTCH_A {
    #[doc = "0: No ATTCH interrupt occurred"]
    _0 = 0,
    #[doc = "1: ATTCH interrupt occurred"]
    _1 = 1,
}
impl From<ATTCH_A> for bool {
    #[inline(always)]
    fn from(variant: ATTCH_A) -> Self {
        variant as u8 != 0
    }
}
impl ATTCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATTCH_A {
        match self.bits {
            false => ATTCH_A::_0,
            true => ATTCH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ATTCH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ATTCH_A::_1
    }
}
#[doc = "Field `ATTCH` writer - ATTCH Interrupt Status"]
pub type ATTCH_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTSTS1_SPEC, ATTCH_A, O>;
impl<'a, const O: u8> ATTCH_W<'a, O> {
    #[doc = "No ATTCH interrupt occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ATTCH_A::_0)
    }
    #[doc = "ATTCH interrupt occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ATTCH_A::_1)
    }
}
#[doc = "Field `DTCH` reader - USB Disconnection Detection Interrupt Status"]
pub type DTCH_R = crate::BitReader<DTCH_A>;
#[doc = "USB Disconnection Detection Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTCH_A {
    #[doc = "0: No DTCH interrupt occurred"]
    _0 = 0,
    #[doc = "1: DTCH interrupt occurred"]
    _1 = 1,
}
impl From<DTCH_A> for bool {
    #[inline(always)]
    fn from(variant: DTCH_A) -> Self {
        variant as u8 != 0
    }
}
impl DTCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTCH_A {
        match self.bits {
            false => DTCH_A::_0,
            true => DTCH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTCH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTCH_A::_1
    }
}
#[doc = "Field `DTCH` writer - USB Disconnection Detection Interrupt Status"]
pub type DTCH_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTSTS1_SPEC, DTCH_A, O>;
impl<'a, const O: u8> DTCH_W<'a, O> {
    #[doc = "No DTCH interrupt occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTCH_A::_0)
    }
    #[doc = "DTCH interrupt occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTCH_A::_1)
    }
}
#[doc = "Field `BCHG` reader - USB Bus Change Interrupt Status"]
pub type BCHG_R = crate::BitReader<BCHG_A>;
#[doc = "USB Bus Change Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCHG_A {
    #[doc = "0: No BCHG interrupt occurred"]
    _0 = 0,
    #[doc = "1: BCHG interrupt occurred"]
    _1 = 1,
}
impl From<BCHG_A> for bool {
    #[inline(always)]
    fn from(variant: BCHG_A) -> Self {
        variant as u8 != 0
    }
}
impl BCHG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCHG_A {
        match self.bits {
            false => BCHG_A::_0,
            true => BCHG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BCHG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BCHG_A::_1
    }
}
#[doc = "Field `BCHG` writer - USB Bus Change Interrupt Status"]
pub type BCHG_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTSTS1_SPEC, BCHG_A, O>;
impl<'a, const O: u8> BCHG_W<'a, O> {
    #[doc = "No BCHG interrupt occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCHG_A::_0)
    }
    #[doc = "BCHG interrupt occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCHG_A::_1)
    }
}
#[doc = "Field `OVRCR` reader - Overcurrent Input Change Interrupt Status"]
pub type OVRCR_R = crate::BitReader<OVRCR_A>;
#[doc = "Overcurrent Input Change Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRCR_A {
    #[doc = "0: No OVRCR interrupt occurred"]
    _0 = 0,
    #[doc = "1: OVRCR interrupt occurred"]
    _1 = 1,
}
impl From<OVRCR_A> for bool {
    #[inline(always)]
    fn from(variant: OVRCR_A) -> Self {
        variant as u8 != 0
    }
}
impl OVRCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRCR_A {
        match self.bits {
            false => OVRCR_A::_0,
            true => OVRCR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVRCR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVRCR_A::_1
    }
}
#[doc = "Field `OVRCR` writer - Overcurrent Input Change Interrupt Status"]
pub type OVRCR_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTSTS1_SPEC, OVRCR_A, O>;
impl<'a, const O: u8> OVRCR_W<'a, O> {
    #[doc = "No OVRCR interrupt occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVRCR_A::_0)
    }
    #[doc = "OVRCR interrupt occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVRCR_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - PDDET Detection Interrupt Status Flag"]
    #[inline(always)]
    pub fn pddetint(&self) -> PDDETINT_R {
        PDDETINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Setup Transaction Normal Response Interrupt Status"]
    #[inline(always)]
    pub fn sack(&self) -> SACK_R {
        SACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Setup Transaction Error Interrupt Status"]
    #[inline(always)]
    pub fn sign(&self) -> SIGN_R {
        SIGN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EOF Error Detection Interrupt Status"]
    #[inline(always)]
    pub fn eoferr(&self) -> EOFERR_R {
        EOFERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 11 - ATTCH Interrupt Status"]
    #[inline(always)]
    pub fn attch(&self) -> ATTCH_R {
        ATTCH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB Disconnection Detection Interrupt Status"]
    #[inline(always)]
    pub fn dtch(&self) -> DTCH_R {
        DTCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USB Bus Change Interrupt Status"]
    #[inline(always)]
    pub fn bchg(&self) -> BCHG_R {
        BCHG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Overcurrent Input Change Interrupt Status"]
    #[inline(always)]
    pub fn ovrcr(&self) -> OVRCR_R {
        OVRCR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDDET Detection Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pddetint(&mut self) -> PDDETINT_W<0> {
        PDDETINT_W::new(self)
    }
    #[doc = "Bit 4 - Setup Transaction Normal Response Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn sack(&mut self) -> SACK_W<4> {
        SACK_W::new(self)
    }
    #[doc = "Bit 5 - Setup Transaction Error Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn sign(&mut self) -> SIGN_W<5> {
        SIGN_W::new(self)
    }
    #[doc = "Bit 6 - EOF Error Detection Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn eoferr(&mut self) -> EOFERR_W<6> {
        EOFERR_W::new(self)
    }
    #[doc = "Bit 11 - ATTCH Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn attch(&mut self) -> ATTCH_W<11> {
        ATTCH_W::new(self)
    }
    #[doc = "Bit 12 - USB Disconnection Detection Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn dtch(&mut self) -> DTCH_W<12> {
        DTCH_W::new(self)
    }
    #[doc = "Bit 14 - USB Bus Change Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn bchg(&mut self) -> BCHG_W<14> {
        BCHG_W::new(self)
    }
    #[doc = "Bit 15 - Overcurrent Input Change Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn ovrcr(&mut self) -> OVRCR_W<15> {
        OVRCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intsts1](index.html) module"]
pub struct INTSTS1_SPEC;
impl crate::RegisterSpec for INTSTS1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [intsts1::R](R) reader structure"]
impl crate::Readable for INTSTS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intsts1::W](W) writer structure"]
impl crate::Writable for INTSTS1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTSTS1 to value 0"]
impl crate::Resettable for INTSTS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
