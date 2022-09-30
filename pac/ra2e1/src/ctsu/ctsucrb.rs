#[doc = "Register `CTSUCRB` reader"]
pub struct R(crate::R<CTSUCRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUCRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUCRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUCRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUCRB` writer"]
pub struct W(crate::W<CTSUCRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUCRB_SPEC>;
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
impl From<crate::W<CTSUCRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUCRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRRATIO` reader - Frequency of Drive Pulse Phase Control"]
pub type PRRATIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRRATIO` writer - Frequency of Drive Pulse Phase Control"]
pub type PRRATIO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTSUCRB_SPEC, u8, u8, 4, O>;
#[doc = "Field `PRMODE` reader - Phase Control Period"]
pub type PRMODE_R = crate::FieldReader<u8, PRMODE_A>;
#[doc = "Phase Control Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRMODE_A {
    #[doc = "0: 510 pulses (512 pulses when PROFF = 1)"]
    _00 = 0,
    #[doc = "1: 126 pulses (128 pulses when PROFF = 1)"]
    _01 = 1,
    #[doc = "2: 62 pulses (64 pulses when PROFF = 1)"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<PRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRMODE_A) -> Self {
        variant as _
    }
}
impl PRMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRMODE_A {
        match self.bits {
            0 => PRMODE_A::_00,
            1 => PRMODE_A::_01,
            2 => PRMODE_A::_10,
            3 => PRMODE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PRMODE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PRMODE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PRMODE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PRMODE_A::_11
    }
}
#[doc = "Field `PRMODE` writer - Phase Control Period"]
pub type PRMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTSUCRB_SPEC, u8, PRMODE_A, 2, O>;
impl<'a, const O: u8> PRMODE_W<'a, O> {
    #[doc = "510 pulses (512 pulses when PROFF = 1)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PRMODE_A::_00)
    }
    #[doc = "126 pulses (128 pulses when PROFF = 1)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PRMODE_A::_01)
    }
    #[doc = "62 pulses (64 pulses when PROFF = 1)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PRMODE_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PRMODE_A::_11)
    }
}
#[doc = "Field `SOFF` reader - High-Pass Noise Reduction Function Disable"]
pub type SOFF_R = crate::BitReader<SOFF_A>;
#[doc = "High-Pass Noise Reduction Function Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFF_A {
    #[doc = "0: Turn the spread spectrum on"]
    _0 = 0,
    #[doc = "1: Turn the spread spectrum off"]
    _1 = 1,
}
impl From<SOFF_A> for bool {
    #[inline(always)]
    fn from(variant: SOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl SOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFF_A {
        match self.bits {
            false => SOFF_A::_0,
            true => SOFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOFF_A::_1
    }
}
#[doc = "Field `SOFF` writer - High-Pass Noise Reduction Function Disable"]
pub type SOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCRB_SPEC, SOFF_A, O>;
impl<'a, const O: u8> SOFF_W<'a, O> {
    #[doc = "Turn the spread spectrum on"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOFF_A::_0)
    }
    #[doc = "Turn the spread spectrum off"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOFF_A::_1)
    }
}
#[doc = "Field `PROFF` reader - Drive Pulse Phase Control"]
pub type PROFF_R = crate::BitReader<PROFF_A>;
#[doc = "Drive Pulse Phase Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROFF_A {
    #[doc = "0: The drive pulse phase is controlled by random numbers."]
    _0 = 0,
    #[doc = "1: The drive pulse phase is not controlled by random numbers."]
    _1 = 1,
}
impl From<PROFF_A> for bool {
    #[inline(always)]
    fn from(variant: PROFF_A) -> Self {
        variant as u8 != 0
    }
}
impl PROFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROFF_A {
        match self.bits {
            false => PROFF_A::_0,
            true => PROFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PROFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PROFF_A::_1
    }
}
#[doc = "Field `PROFF` writer - Drive Pulse Phase Control"]
pub type PROFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCRB_SPEC, PROFF_A, O>;
impl<'a, const O: u8> PROFF_W<'a, O> {
    #[doc = "The drive pulse phase is controlled by random numbers."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PROFF_A::_0)
    }
    #[doc = "The drive pulse phase is not controlled by random numbers."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PROFF_A::_1)
    }
}
#[doc = "Field `SST` reader - Wait Time Sensor Stabilization"]
pub type SST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SST` writer - Wait Time Sensor Stabilization"]
pub type SST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTSUCRB_SPEC, u8, u8, 8, O>;
#[doc = "Field `SSMOD` reader - Spread Spectrum Modulation Frequency"]
pub type SSMOD_R = crate::FieldReader<u8, SSMOD_A>;
#[doc = "Spread Spectrum Modulation Frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSMOD_A {
    #[doc = "0: 125 kHz (recommended)"]
    _000 = 0,
    #[doc = "1: 83.3 kHz"]
    _001 = 1,
    #[doc = "2: 62.5 kHz"]
    _010 = 2,
    #[doc = "3: 31.3 kHz"]
    _011 = 3,
}
impl From<SSMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: SSMOD_A) -> Self {
        variant as _
    }
}
impl SSMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SSMOD_A> {
        match self.bits {
            0 => Some(SSMOD_A::_000),
            1 => Some(SSMOD_A::_001),
            2 => Some(SSMOD_A::_010),
            3 => Some(SSMOD_A::_011),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SSMOD_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SSMOD_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SSMOD_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SSMOD_A::_011
    }
}
#[doc = "Field `SSMOD` writer - Spread Spectrum Modulation Frequency"]
pub type SSMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTSUCRB_SPEC, u8, SSMOD_A, 3, O>;
impl<'a, const O: u8> SSMOD_W<'a, O> {
    #[doc = "125 kHz (recommended)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(SSMOD_A::_000)
    }
    #[doc = "83.3 kHz"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(SSMOD_A::_001)
    }
    #[doc = "62.5 kHz"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(SSMOD_A::_010)
    }
    #[doc = "31.3 kHz"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(SSMOD_A::_011)
    }
}
#[doc = "Field `SSCNT` reader - Adjusting the SUCLK frequency"]
pub type SSCNT_R = crate::FieldReader<u8, SSCNT_A>;
#[doc = "Adjusting the SUCLK frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSCNT_A {
    #[doc = "0: CTSUTRIMA.SUADJD + 0x00 (SDPSEL = 0) CTSUSUCLKx.SUADJDn + 0x00 (SDPSEL = 1)"]
    _00 = 0,
    #[doc = "1: CTSUTRIMA.SUADJD + 0x10 (SDPSEL = 0) CTSUSUCLKx.SUADJDn + 0x20 (SDPSEL = 1)"]
    _01 = 1,
    #[doc = "2: CTSUTRIMA.SUADJD + 0x20 (SDPSEL = 0) CTSUSUCLKx.SUADJDn + 0x40 (SDPSEL = 1)"]
    _10 = 2,
    #[doc = "3: CTSUTRIMA.SUADJD + 0x30 (SDPSEL = 0) CTSUSUCLKx.SUADJDn + 0x40 (SDPSEL = 1)"]
    _11 = 3,
}
impl From<SSCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: SSCNT_A) -> Self {
        variant as _
    }
}
impl SSCNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSCNT_A {
        match self.bits {
            0 => SSCNT_A::_00,
            1 => SSCNT_A::_01,
            2 => SSCNT_A::_10,
            3 => SSCNT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SSCNT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SSCNT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SSCNT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SSCNT_A::_11
    }
}
#[doc = "Field `SSCNT` writer - Adjusting the SUCLK frequency"]
pub type SSCNT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTSUCRB_SPEC, u8, SSCNT_A, 2, O>;
impl<'a, const O: u8> SSCNT_W<'a, O> {
    #[doc = "CTSUTRIMA.SUADJD + 0x00 (SDPSEL = 0) CTSUSUCLKx.SUADJDn + 0x00 (SDPSEL = 1)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SSCNT_A::_00)
    }
    #[doc = "CTSUTRIMA.SUADJD + 0x10 (SDPSEL = 0) CTSUSUCLKx.SUADJDn + 0x20 (SDPSEL = 1)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SSCNT_A::_01)
    }
    #[doc = "CTSUTRIMA.SUADJD + 0x20 (SDPSEL = 0) CTSUSUCLKx.SUADJDn + 0x40 (SDPSEL = 1)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SSCNT_A::_10)
    }
    #[doc = "CTSUTRIMA.SUADJD + 0x30 (SDPSEL = 0) CTSUSUCLKx.SUADJDn + 0x40 (SDPSEL = 1)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SSCNT_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:3 - Frequency of Drive Pulse Phase Control"]
    #[inline(always)]
    pub fn prratio(&self) -> PRRATIO_R {
        PRRATIO_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Phase Control Period"]
    #[inline(always)]
    pub fn prmode(&self) -> PRMODE_R {
        PRMODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - High-Pass Noise Reduction Function Disable"]
    #[inline(always)]
    pub fn soff(&self) -> SOFF_R {
        SOFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Drive Pulse Phase Control"]
    #[inline(always)]
    pub fn proff(&self) -> PROFF_R {
        PROFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Wait Time Sensor Stabilization"]
    #[inline(always)]
    pub fn sst(&self) -> SST_R {
        SST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - Spread Spectrum Modulation Frequency"]
    #[inline(always)]
    pub fn ssmod(&self) -> SSMOD_R {
        SSMOD_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - Adjusting the SUCLK frequency"]
    #[inline(always)]
    pub fn sscnt(&self) -> SSCNT_R {
        SSCNT_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Frequency of Drive Pulse Phase Control"]
    #[inline(always)]
    pub fn prratio(&mut self) -> PRRATIO_W<0> {
        PRRATIO_W::new(self)
    }
    #[doc = "Bits 4:5 - Phase Control Period"]
    #[inline(always)]
    pub fn prmode(&mut self) -> PRMODE_W<4> {
        PRMODE_W::new(self)
    }
    #[doc = "Bit 6 - High-Pass Noise Reduction Function Disable"]
    #[inline(always)]
    pub fn soff(&mut self) -> SOFF_W<6> {
        SOFF_W::new(self)
    }
    #[doc = "Bit 7 - Drive Pulse Phase Control"]
    #[inline(always)]
    pub fn proff(&mut self) -> PROFF_W<7> {
        PROFF_W::new(self)
    }
    #[doc = "Bits 8:15 - Wait Time Sensor Stabilization"]
    #[inline(always)]
    pub fn sst(&mut self) -> SST_W<8> {
        SST_W::new(self)
    }
    #[doc = "Bits 24:26 - Spread Spectrum Modulation Frequency"]
    #[inline(always)]
    pub fn ssmod(&mut self) -> SSMOD_W<24> {
        SSMOD_W::new(self)
    }
    #[doc = "Bits 28:29 - Adjusting the SUCLK frequency"]
    #[inline(always)]
    pub fn sscnt(&mut self) -> SSCNT_W<28> {
        SSCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU Control Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsucrb](index.html) module"]
pub struct CTSUCRB_SPEC;
impl crate::RegisterSpec for CTSUCRB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctsucrb::R](R) reader structure"]
impl crate::Readable for CTSUCRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsucrb::W](W) writer structure"]
impl crate::Writable for CTSUCRB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTSUCRB to value 0"]
impl crate::Resettable for CTSUCRB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
