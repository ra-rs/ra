#[doc = "Register `BSTFC` reader"]
pub struct R(crate::R<BSTFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSTFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSTFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSTFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BSTFC` writer"]
pub struct W(crate::W<BSTFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSTFC_SPEC>;
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
impl From<crate::W<BSTFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSTFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "START condition Detection Force\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STCNDDFC_AW {
    #[doc = "0: Not Force START condition Detection Interrupt for software testing."]
    _0 = 0,
    #[doc = "1: Force START condition Detection Interrupt for software testing."]
    _1 = 1,
}
impl From<STCNDDFC_AW> for bool {
    #[inline(always)]
    fn from(variant: STCNDDFC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STCNDDFC` writer - START condition Detection Force"]
pub type STCNDDFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSTFC_SPEC, STCNDDFC_AW, O>;
impl<'a, const O: u8> STCNDDFC_W<'a, O> {
    #[doc = "Not Force START condition Detection Interrupt for software testing."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STCNDDFC_AW::_0)
    }
    #[doc = "Force START condition Detection Interrupt for software testing."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STCNDDFC_AW::_1)
    }
}
#[doc = "STOP condition Detection Force\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPCNDDFC_AW {
    #[doc = "0: Not Force STOP condition Detection Interrupt for software testing."]
    _0 = 0,
    #[doc = "1: Force STOP condition Detection Interrupt for software testing."]
    _1 = 1,
}
impl From<SPCNDDFC_AW> for bool {
    #[inline(always)]
    fn from(variant: SPCNDDFC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPCNDDFC` writer - STOP condition Detection Force"]
pub type SPCNDDFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSTFC_SPEC, SPCNDDFC_AW, O>;
impl<'a, const O: u8> SPCNDDFC_W<'a, O> {
    #[doc = "Not Force STOP condition Detection Interrupt for software testing."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPCNDDFC_AW::_0)
    }
    #[doc = "Force STOP condition Detection Interrupt for software testing."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPCNDDFC_AW::_1)
    }
}
#[doc = "HDR Exit Pattern Detection Force\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDREXDFC_AW {
    #[doc = "0: Not Force HDR Exit Pattern Detection Interrupt for software testing."]
    _0 = 0,
    #[doc = "1: Force HDR Exit Pattern Detection Interrupt for software testing."]
    _1 = 1,
}
impl From<HDREXDFC_AW> for bool {
    #[inline(always)]
    fn from(variant: HDREXDFC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDREXDFC` writer - HDR Exit Pattern Detection Force"]
pub type HDREXDFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSTFC_SPEC, HDREXDFC_AW, O>;
impl<'a, const O: u8> HDREXDFC_W<'a, O> {
    #[doc = "Not Force HDR Exit Pattern Detection Interrupt for software testing."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HDREXDFC_AW::_0)
    }
    #[doc = "Force HDR Exit Pattern Detection Interrupt for software testing."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HDREXDFC_AW::_1)
    }
}
#[doc = "NACK Detection Force\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKDFC_AW {
    #[doc = "0: Not Force NACK Detection Interrupt for software testing."]
    _0 = 0,
    #[doc = "1: Force NACK Detection Interrupt for software testing."]
    _1 = 1,
}
impl From<NACKDFC_AW> for bool {
    #[inline(always)]
    fn from(variant: NACKDFC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACKDFC` writer - NACK Detection Force"]
pub type NACKDFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSTFC_SPEC, NACKDFC_AW, O>;
impl<'a, const O: u8> NACKDFC_W<'a, O> {
    #[doc = "Not Force NACK Detection Interrupt for software testing."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NACKDFC_AW::_0)
    }
    #[doc = "Force NACK Detection Interrupt for software testing."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NACKDFC_AW::_1)
    }
}
#[doc = "Transmit End Force\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TENDFC_AW {
    #[doc = "0: Not Force Transmit End Interrupt for software testing."]
    _0 = 0,
    #[doc = "1: Force Transmit End Interrupt for software testing."]
    _1 = 1,
}
impl From<TENDFC_AW> for bool {
    #[inline(always)]
    fn from(variant: TENDFC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TENDFC` writer - Transmit End Force"]
pub type TENDFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSTFC_SPEC, TENDFC_AW, O>;
impl<'a, const O: u8> TENDFC_W<'a, O> {
    #[doc = "Not Force Transmit End Interrupt for software testing."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TENDFC_AW::_0)
    }
    #[doc = "Force Transmit End Interrupt for software testing."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TENDFC_AW::_1)
    }
}
#[doc = "Arbitration Lost Force\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALFC_AW {
    #[doc = "0: Not Force Arbitration Lost Interrupt for software testing."]
    _0 = 0,
    #[doc = "1: Force Arbitration Lost Interrupt for software testing."]
    _1 = 1,
}
impl From<ALFC_AW> for bool {
    #[inline(always)]
    fn from(variant: ALFC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALFC` writer - Arbitration Lost Force"]
pub type ALFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSTFC_SPEC, ALFC_AW, O>;
impl<'a, const O: u8> ALFC_W<'a, O> {
    #[doc = "Not Force Arbitration Lost Interrupt for software testing."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALFC_AW::_0)
    }
    #[doc = "Force Arbitration Lost Interrupt for software testing."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALFC_AW::_1)
    }
}
#[doc = "Timeout Detection Force\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TODFC_AW {
    #[doc = "0: Not Force Timeout Detection Interrupt for software testing."]
    _0 = 0,
    #[doc = "1: Force Timeout Detection Interrupt for software testing."]
    _1 = 1,
}
impl From<TODFC_AW> for bool {
    #[inline(always)]
    fn from(variant: TODFC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TODFC` writer - Timeout Detection Force"]
pub type TODFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSTFC_SPEC, TODFC_AW, O>;
impl<'a, const O: u8> TODFC_W<'a, O> {
    #[doc = "Not Force Timeout Detection Interrupt for software testing."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TODFC_AW::_0)
    }
    #[doc = "Force Timeout Detection Interrupt for software testing."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TODFC_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - START condition Detection Force"]
    #[inline(always)]
    #[must_use]
    pub fn stcnddfc(&mut self) -> STCNDDFC_W<0> {
        STCNDDFC_W::new(self)
    }
    #[doc = "Bit 1 - STOP condition Detection Force"]
    #[inline(always)]
    #[must_use]
    pub fn spcnddfc(&mut self) -> SPCNDDFC_W<1> {
        SPCNDDFC_W::new(self)
    }
    #[doc = "Bit 2 - HDR Exit Pattern Detection Force"]
    #[inline(always)]
    #[must_use]
    pub fn hdrexdfc(&mut self) -> HDREXDFC_W<2> {
        HDREXDFC_W::new(self)
    }
    #[doc = "Bit 4 - NACK Detection Force"]
    #[inline(always)]
    #[must_use]
    pub fn nackdfc(&mut self) -> NACKDFC_W<4> {
        NACKDFC_W::new(self)
    }
    #[doc = "Bit 8 - Transmit End Force"]
    #[inline(always)]
    #[must_use]
    pub fn tendfc(&mut self) -> TENDFC_W<8> {
        TENDFC_W::new(self)
    }
    #[doc = "Bit 16 - Arbitration Lost Force"]
    #[inline(always)]
    #[must_use]
    pub fn alfc(&mut self) -> ALFC_W<16> {
        ALFC_W::new(self)
    }
    #[doc = "Bit 20 - Timeout Detection Force"]
    #[inline(always)]
    #[must_use]
    pub fn todfc(&mut self) -> TODFC_W<20> {
        TODFC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus Status Force Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bstfc](index.html) module"]
pub struct BSTFC_SPEC;
impl crate::RegisterSpec for BSTFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bstfc::R](R) reader structure"]
impl crate::Readable for BSTFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bstfc::W](W) writer structure"]
impl crate::Writable for BSTFC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BSTFC to value 0"]
impl crate::Resettable for BSTFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
