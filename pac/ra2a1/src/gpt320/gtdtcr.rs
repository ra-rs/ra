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
    #[doc = "0: GTCCRB is set without using GTDVU and GTDVD."]
    _0 = 0,
    #[doc = "1: GTDVU and GTDVD are used to set the compare match value for negative-phase waveform with dead time automatically in GTCCRB."]
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
    #[doc = "GTCCRB is set without using GTDVU and GTDVD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDE_A::_0)
    }
    #[doc = "GTDVU and GTDVD are used to set the compare match value for negative-phase waveform with dead time automatically in GTCCRB."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Negative-Phase Waveform Setting"]
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Negative-Phase Waveform Setting"]
    #[inline(always)]
    #[must_use]
    pub fn tde(&mut self) -> TDE_W<0> {
        TDE_W::new(self)
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
