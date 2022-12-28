#[doc = "Register `VBTWCH0OTSR` reader"]
pub struct R(crate::R<VBTWCH0OTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VBTWCH0OTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VBTWCH0OTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VBTWCH0OTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VBTWCH0OTSR` writer"]
pub struct W(crate::W<VBTWCH0OTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VBTWCH0OTSR_SPEC>;
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
impl From<crate::W<VBTWCH0OTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VBTWCH0OTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0VCH1TE` reader - VBATWIO0 Output VBATWIO1 Trigger Enable"]
pub type CH0VCH1TE_R = crate::BitReader<CH0VCH1TE_A>;
#[doc = "VBATWIO0 Output VBATWIO1 Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0VCH1TE_A {
    #[doc = "0: VBATT wakeup I/O 0 output trigger by the VBATWIO1 pin is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup I/O 0 output trigger by the VBATWIO1 pin is enabled."]
    _1 = 1,
}
impl From<CH0VCH1TE_A> for bool {
    #[inline(always)]
    fn from(variant: CH0VCH1TE_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0VCH1TE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0VCH1TE_A {
        match self.bits {
            false => CH0VCH1TE_A::_0,
            true => CH0VCH1TE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH0VCH1TE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH0VCH1TE_A::_1
    }
}
#[doc = "Field `CH0VCH1TE` writer - VBATWIO0 Output VBATWIO1 Trigger Enable"]
pub type CH0VCH1TE_W<'a, const O: u8> = crate::BitWriter<'a, u8, VBTWCH0OTSR_SPEC, CH0VCH1TE_A, O>;
impl<'a, const O: u8> CH0VCH1TE_W<'a, O> {
    #[doc = "VBATT wakeup I/O 0 output trigger by the VBATWIO1 pin is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH0VCH1TE_A::_0)
    }
    #[doc = "VBATT wakeup I/O 0 output trigger by the VBATWIO1 pin is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH0VCH1TE_A::_1)
    }
}
#[doc = "Field `CH0VCH2TE` reader - VBATWIO0 Output VBATWIO2 Trigger Enable"]
pub type CH0VCH2TE_R = crate::BitReader<CH0VCH2TE_A>;
#[doc = "VBATWIO0 Output VBATWIO2 Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0VCH2TE_A {
    #[doc = "0: VBATT wakeup I/O 0 output trigger by the VBATWIO2 pin is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup I/O 0 output trigger by the VBATWIO2 pin is enabled."]
    _1 = 1,
}
impl From<CH0VCH2TE_A> for bool {
    #[inline(always)]
    fn from(variant: CH0VCH2TE_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0VCH2TE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0VCH2TE_A {
        match self.bits {
            false => CH0VCH2TE_A::_0,
            true => CH0VCH2TE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH0VCH2TE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH0VCH2TE_A::_1
    }
}
#[doc = "Field `CH0VCH2TE` writer - VBATWIO0 Output VBATWIO2 Trigger Enable"]
pub type CH0VCH2TE_W<'a, const O: u8> = crate::BitWriter<'a, u8, VBTWCH0OTSR_SPEC, CH0VCH2TE_A, O>;
impl<'a, const O: u8> CH0VCH2TE_W<'a, O> {
    #[doc = "VBATT wakeup I/O 0 output trigger by the VBATWIO2 pin is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH0VCH2TE_A::_0)
    }
    #[doc = "VBATT wakeup I/O 0 output trigger by the VBATWIO2 pin is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH0VCH2TE_A::_1)
    }
}
#[doc = "Field `CH0VRTCTE` reader - VBATWIO0 Output RTC Periodic Signal Enable"]
pub type CH0VRTCTE_R = crate::BitReader<CH0VRTCTE_A>;
#[doc = "VBATWIO0 Output RTC Periodic Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0VRTCTE_A {
    #[doc = "0: VBATT wakeup I/O 0 output trigger by the RTC periodic signal is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup I/O 0 output trigger by the RTC periodic signal is enabled."]
    _1 = 1,
}
impl From<CH0VRTCTE_A> for bool {
    #[inline(always)]
    fn from(variant: CH0VRTCTE_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0VRTCTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0VRTCTE_A {
        match self.bits {
            false => CH0VRTCTE_A::_0,
            true => CH0VRTCTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH0VRTCTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH0VRTCTE_A::_1
    }
}
#[doc = "Field `CH0VRTCTE` writer - VBATWIO0 Output RTC Periodic Signal Enable"]
pub type CH0VRTCTE_W<'a, const O: u8> = crate::BitWriter<'a, u8, VBTWCH0OTSR_SPEC, CH0VRTCTE_A, O>;
impl<'a, const O: u8> CH0VRTCTE_W<'a, O> {
    #[doc = "VBATT wakeup I/O 0 output trigger by the RTC periodic signal is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH0VRTCTE_A::_0)
    }
    #[doc = "VBATT wakeup I/O 0 output trigger by the RTC periodic signal is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH0VRTCTE_A::_1)
    }
}
#[doc = "Field `CH0VRTCATE` reader - VBATWIO0 Output RTC Alarm Signal Enable"]
pub type CH0VRTCATE_R = crate::BitReader<CH0VRTCATE_A>;
#[doc = "VBATWIO0 Output RTC Alarm Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0VRTCATE_A {
    #[doc = "0: VBATT wakeup I/O 0 output trigger by the RTC alarm signal is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup I/O 0 output trigger by the RTC alarm signal is enabled."]
    _1 = 1,
}
impl From<CH0VRTCATE_A> for bool {
    #[inline(always)]
    fn from(variant: CH0VRTCATE_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0VRTCATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0VRTCATE_A {
        match self.bits {
            false => CH0VRTCATE_A::_0,
            true => CH0VRTCATE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH0VRTCATE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH0VRTCATE_A::_1
    }
}
#[doc = "Field `CH0VRTCATE` writer - VBATWIO0 Output RTC Alarm Signal Enable"]
pub type CH0VRTCATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, VBTWCH0OTSR_SPEC, CH0VRTCATE_A, O>;
impl<'a, const O: u8> CH0VRTCATE_W<'a, O> {
    #[doc = "VBATT wakeup I/O 0 output trigger by the RTC alarm signal is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH0VRTCATE_A::_0)
    }
    #[doc = "VBATT wakeup I/O 0 output trigger by the RTC alarm signal is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH0VRTCATE_A::_1)
    }
}
impl R {
    #[doc = "Bit 1 - VBATWIO0 Output VBATWIO1 Trigger Enable"]
    #[inline(always)]
    pub fn ch0vch1te(&self) -> CH0VCH1TE_R {
        CH0VCH1TE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VBATWIO0 Output VBATWIO2 Trigger Enable"]
    #[inline(always)]
    pub fn ch0vch2te(&self) -> CH0VCH2TE_R {
        CH0VCH2TE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VBATWIO0 Output RTC Periodic Signal Enable"]
    #[inline(always)]
    pub fn ch0vrtcte(&self) -> CH0VRTCTE_R {
        CH0VRTCTE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VBATWIO0 Output RTC Alarm Signal Enable"]
    #[inline(always)]
    pub fn ch0vrtcate(&self) -> CH0VRTCATE_R {
        CH0VRTCATE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - VBATWIO0 Output VBATWIO1 Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0vch1te(&mut self) -> CH0VCH1TE_W<1> {
        CH0VCH1TE_W::new(self)
    }
    #[doc = "Bit 2 - VBATWIO0 Output VBATWIO2 Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0vch2te(&mut self) -> CH0VCH2TE_W<2> {
        CH0VCH2TE_W::new(self)
    }
    #[doc = "Bit 3 - VBATWIO0 Output RTC Periodic Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0vrtcte(&mut self) -> CH0VRTCTE_W<3> {
        CH0VRTCTE_W::new(self)
    }
    #[doc = "Bit 4 - VBATWIO0 Output RTC Alarm Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0vrtcate(&mut self) -> CH0VRTCATE_W<4> {
        CH0VRTCATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VBATT Wakeup I/O 0 Output Trigger Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vbtwch0otsr](index.html) module"]
pub struct VBTWCH0OTSR_SPEC;
impl crate::RegisterSpec for VBTWCH0OTSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [vbtwch0otsr::R](R) reader structure"]
impl crate::Readable for VBTWCH0OTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vbtwch0otsr::W](W) writer structure"]
impl crate::Writable for VBTWCH0OTSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VBTWCH0OTSR to value 0"]
impl crate::Resettable for VBTWCH0OTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
