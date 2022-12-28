#[doc = "Register `GTDTCR` reader"]
pub struct R(crate::R<GTDTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTDTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTDTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTDTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTDTCR` writer"]
pub struct W(crate::W<GTDTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTDTCR_SPEC>;
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
impl From<crate::W<GTDTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTDTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDE` reader - Negative-Phase Waveform Setting"]
pub type TDE_R = crate::BitReader<TDE_A>;
#[doc = "Negative-Phase Waveform Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDE_A {
    #[doc = "0: Set GTCCRB without using GTDVU and GTDVD."]
    _0 = 0,
    #[doc = "1: Use GTDVU and GTDVD to set the compare match value for negative-phase waveform with automatic dead time in GTCCRB."]
    _1 = 1,
}
impl From<TDE_A> for bool {
    #[inline(always)]
    fn from(variant: TDE_A) -> Self {
        variant as u8 != 0
    }
}
impl TDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDE_A {
        match self.bits {
            false => TDE_A::_0,
            true => TDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDE_A::_1
    }
}
#[doc = "Field `TDE` writer - Negative-Phase Waveform Setting"]
pub type TDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTDTCR_SPEC, TDE_A, O>;
impl<'a, const O: u8> TDE_W<'a, O> {
    #[doc = "Set GTCCRB without using GTDVU and GTDVD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDE_A::_0)
    }
    #[doc = "Use GTDVU and GTDVD to set the compare match value for negative-phase waveform with automatic dead time in GTCCRB."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDE_A::_1)
    }
}
#[doc = "Field `TDBUE` reader - GTDVU Buffer Operation Enable"]
pub type TDBUE_R = crate::BitReader<TDBUE_A>;
#[doc = "GTDVU Buffer Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDBUE_A {
    #[doc = "0: Disable GTDVU buffer operation"]
    _0 = 0,
    #[doc = "1: Enable GTDVU buffer operation"]
    _1 = 1,
}
impl From<TDBUE_A> for bool {
    #[inline(always)]
    fn from(variant: TDBUE_A) -> Self {
        variant as u8 != 0
    }
}
impl TDBUE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDBUE_A {
        match self.bits {
            false => TDBUE_A::_0,
            true => TDBUE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDBUE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDBUE_A::_1
    }
}
#[doc = "Field `TDBUE` writer - GTDVU Buffer Operation Enable"]
pub type TDBUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTDTCR_SPEC, TDBUE_A, O>;
impl<'a, const O: u8> TDBUE_W<'a, O> {
    #[doc = "Disable GTDVU buffer operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDBUE_A::_0)
    }
    #[doc = "Enable GTDVU buffer operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDBUE_A::_1)
    }
}
#[doc = "Field `TDBDE` reader - GTDVD Buffer Operation Enable"]
pub type TDBDE_R = crate::BitReader<TDBDE_A>;
#[doc = "GTDVD Buffer Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDBDE_A {
    #[doc = "0: Disable GTDVD buffer operation"]
    _0 = 0,
    #[doc = "1: Enable GTDVD buffer operation"]
    _1 = 1,
}
impl From<TDBDE_A> for bool {
    #[inline(always)]
    fn from(variant: TDBDE_A) -> Self {
        variant as u8 != 0
    }
}
impl TDBDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDBDE_A {
        match self.bits {
            false => TDBDE_A::_0,
            true => TDBDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDBDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDBDE_A::_1
    }
}
#[doc = "Field `TDBDE` writer - GTDVD Buffer Operation Enable"]
pub type TDBDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTDTCR_SPEC, TDBDE_A, O>;
impl<'a, const O: u8> TDBDE_W<'a, O> {
    #[doc = "Disable GTDVD buffer operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDBDE_A::_0)
    }
    #[doc = "Enable GTDVD buffer operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDBDE_A::_1)
    }
}
#[doc = "Field `TDFER` reader - GTDVD Setting"]
pub type TDFER_R = crate::BitReader<TDFER_A>;
#[doc = "GTDVD Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDFER_A {
    #[doc = "0: Set GTDVU and GTDVD separately"]
    _0 = 0,
    #[doc = "1: Automatically set the value written to GTDVU to GTDVD"]
    _1 = 1,
}
impl From<TDFER_A> for bool {
    #[inline(always)]
    fn from(variant: TDFER_A) -> Self {
        variant as u8 != 0
    }
}
impl TDFER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDFER_A {
        match self.bits {
            false => TDFER_A::_0,
            true => TDFER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDFER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDFER_A::_1
    }
}
#[doc = "Field `TDFER` writer - GTDVD Setting"]
pub type TDFER_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTDTCR_SPEC, TDFER_A, O>;
impl<'a, const O: u8> TDFER_W<'a, O> {
    #[doc = "Set GTDVU and GTDVD separately"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDFER_A::_0)
    }
    #[doc = "Automatically set the value written to GTDVU to GTDVD"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDFER_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Negative-Phase Waveform Setting"]
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - GTDVU Buffer Operation Enable"]
    #[inline(always)]
    pub fn tdbue(&self) -> TDBUE_R {
        TDBUE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GTDVD Buffer Operation Enable"]
    #[inline(always)]
    pub fn tdbde(&self) -> TDBDE_R {
        TDBDE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - GTDVD Setting"]
    #[inline(always)]
    pub fn tdfer(&self) -> TDFER_R {
        TDFER_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Negative-Phase Waveform Setting"]
    #[inline(always)]
    #[must_use]
    pub fn tde(&mut self) -> TDE_W<0> {
        TDE_W::new(self)
    }
    #[doc = "Bit 4 - GTDVU Buffer Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdbue(&mut self) -> TDBUE_W<4> {
        TDBUE_W::new(self)
    }
    #[doc = "Bit 5 - GTDVD Buffer Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdbde(&mut self) -> TDBDE_W<5> {
        TDBDE_W::new(self)
    }
    #[doc = "Bit 8 - GTDVD Setting"]
    #[inline(always)]
    #[must_use]
    pub fn tdfer(&mut self) -> TDFER_W<8> {
        TDFER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Dead Time Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtdtcr](index.html) module"]
pub struct GTDTCR_SPEC;
impl crate::RegisterSpec for GTDTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtdtcr::R](R) reader structure"]
impl crate::Readable for GTDTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtdtcr::W](W) writer structure"]
impl crate::Writable for GTDTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTDTCR to value 0"]
impl crate::Resettable for GTDTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
