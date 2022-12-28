#[doc = "Register `DPUSR0R` reader"]
pub struct R(crate::R<DPUSR0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPUSR0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPUSR0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPUSR0R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPUSR0R` writer"]
pub struct W(crate::W<DPUSR0R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPUSR0R_SPEC>;
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
impl From<crate::W<DPUSR0R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPUSR0R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRPC0` reader - USB Single End Receiver Control"]
pub type SRPC0_R = crate::BitReader<SRPC0_A>;
#[doc = "USB Single End Receiver Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRPC0_A {
    #[doc = "0: Input through the DP and DM inputs is disabled."]
    _0 = 0,
    #[doc = "1: Input through the DP and DM inputs is enabled."]
    _1 = 1,
}
impl From<SRPC0_A> for bool {
    #[inline(always)]
    fn from(variant: SRPC0_A) -> Self {
        variant as u8 != 0
    }
}
impl SRPC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRPC0_A {
        match self.bits {
            false => SRPC0_A::_0,
            true => SRPC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRPC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRPC0_A::_1
    }
}
#[doc = "Field `SRPC0` writer - USB Single End Receiver Control"]
pub type SRPC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPUSR0R_SPEC, SRPC0_A, O>;
impl<'a, const O: u8> SRPC0_W<'a, O> {
    #[doc = "Input through the DP and DM inputs is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRPC0_A::_0)
    }
    #[doc = "Input through the DP and DM inputs is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRPC0_A::_1)
    }
}
#[doc = "Field `RPUE0` reader - DP Pull-Up Resistor Control"]
pub type RPUE0_R = crate::BitReader<RPUE0_A>;
#[doc = "DP Pull-Up Resistor Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPUE0_A {
    #[doc = "0: Disables DP pull-up resistor."]
    _0 = 0,
    #[doc = "1: Enables DP pull-up resistor."]
    _1 = 1,
}
impl From<RPUE0_A> for bool {
    #[inline(always)]
    fn from(variant: RPUE0_A) -> Self {
        variant as u8 != 0
    }
}
impl RPUE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPUE0_A {
        match self.bits {
            false => RPUE0_A::_0,
            true => RPUE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPUE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPUE0_A::_1
    }
}
#[doc = "Field `RPUE0` writer - DP Pull-Up Resistor Control"]
pub type RPUE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPUSR0R_SPEC, RPUE0_A, O>;
impl<'a, const O: u8> RPUE0_W<'a, O> {
    #[doc = "Disables DP pull-up resistor."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RPUE0_A::_0)
    }
    #[doc = "Enables DP pull-up resistor."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RPUE0_A::_1)
    }
}
#[doc = "Field `DRPD0` reader - D+/D- Pull-Down Resistor Control"]
pub type DRPD0_R = crate::BitReader<DRPD0_A>;
#[doc = "D+/D- Pull-Down Resistor Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRPD0_A {
    #[doc = "0: Disables DP/DM pull-down resistor."]
    _0 = 0,
    #[doc = "1: Enables DP/DM pull-down resistor."]
    _1 = 1,
}
impl From<DRPD0_A> for bool {
    #[inline(always)]
    fn from(variant: DRPD0_A) -> Self {
        variant as u8 != 0
    }
}
impl DRPD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRPD0_A {
        match self.bits {
            false => DRPD0_A::_0,
            true => DRPD0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRPD0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRPD0_A::_1
    }
}
#[doc = "Field `DRPD0` writer - D+/D- Pull-Down Resistor Control"]
pub type DRPD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPUSR0R_SPEC, DRPD0_A, O>;
impl<'a, const O: u8> DRPD0_W<'a, O> {
    #[doc = "Disables DP/DM pull-down resistor."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRPD0_A::_0)
    }
    #[doc = "Enables DP/DM pull-down resistor."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRPD0_A::_1)
    }
}
#[doc = "Field `FIXPHY0` reader - USB Transceiver Output Fix"]
pub type FIXPHY0_R = crate::BitReader<FIXPHY0_A>;
#[doc = "USB Transceiver Output Fix\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIXPHY0_A {
    #[doc = "0: The outputs are fixed in normal mode and on return from deep software standby mode."]
    _0 = 0,
    #[doc = "1: The outputs are fixed on transitions to deep software standby mode."]
    _1 = 1,
}
impl From<FIXPHY0_A> for bool {
    #[inline(always)]
    fn from(variant: FIXPHY0_A) -> Self {
        variant as u8 != 0
    }
}
impl FIXPHY0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIXPHY0_A {
        match self.bits {
            false => FIXPHY0_A::_0,
            true => FIXPHY0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIXPHY0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIXPHY0_A::_1
    }
}
#[doc = "Field `FIXPHY0` writer - USB Transceiver Output Fix"]
pub type FIXPHY0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPUSR0R_SPEC, FIXPHY0_A, O>;
impl<'a, const O: u8> FIXPHY0_W<'a, O> {
    #[doc = "The outputs are fixed in normal mode and on return from deep software standby mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIXPHY0_A::_0)
    }
    #[doc = "The outputs are fixed on transitions to deep software standby mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIXPHY0_A::_1)
    }
}
#[doc = "Field `DP0` reader - USB0 D+ InputIndicates the D+ input signal of the USB."]
pub type DP0_R = crate::BitReader<bool>;
#[doc = "Field `DM0` reader - USB D-InputIndicates the D- input signal of the USB."]
pub type DM0_R = crate::BitReader<bool>;
#[doc = "Field `DOVCA0` reader - USB OVRCURA InputIndicates the OVRCURA input signal of the USB."]
pub type DOVCA0_R = crate::BitReader<bool>;
#[doc = "Field `DOVCB0` reader - USB OVRCURB InputIndicates the OVRCURB input signal of the USB."]
pub type DOVCB0_R = crate::BitReader<bool>;
#[doc = "Field `DVBSTS0` reader - USB VBUS InputIndicates the VBUS input signal of the USB."]
pub type DVBSTS0_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - USB Single End Receiver Control"]
    #[inline(always)]
    pub fn srpc0(&self) -> SRPC0_R {
        SRPC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DP Pull-Up Resistor Control"]
    #[inline(always)]
    pub fn rpue0(&self) -> RPUE0_R {
        RPUE0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - D+/D- Pull-Down Resistor Control"]
    #[inline(always)]
    pub fn drpd0(&self) -> DRPD0_R {
        DRPD0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB Transceiver Output Fix"]
    #[inline(always)]
    pub fn fixphy0(&self) -> FIXPHY0_R {
        FIXPHY0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - USB0 D+ InputIndicates the D+ input signal of the USB."]
    #[inline(always)]
    pub fn dp0(&self) -> DP0_R {
        DP0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USB D-InputIndicates the D- input signal of the USB."]
    #[inline(always)]
    pub fn dm0(&self) -> DM0_R {
        DM0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - USB OVRCURA InputIndicates the OVRCURA input signal of the USB."]
    #[inline(always)]
    pub fn dovca0(&self) -> DOVCA0_R {
        DOVCA0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - USB OVRCURB InputIndicates the OVRCURB input signal of the USB."]
    #[inline(always)]
    pub fn dovcb0(&self) -> DOVCB0_R {
        DOVCB0_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - USB VBUS InputIndicates the VBUS input signal of the USB."]
    #[inline(always)]
    pub fn dvbsts0(&self) -> DVBSTS0_R {
        DVBSTS0_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Single End Receiver Control"]
    #[inline(always)]
    #[must_use]
    pub fn srpc0(&mut self) -> SRPC0_W<0> {
        SRPC0_W::new(self)
    }
    #[doc = "Bit 1 - DP Pull-Up Resistor Control"]
    #[inline(always)]
    #[must_use]
    pub fn rpue0(&mut self) -> RPUE0_W<1> {
        RPUE0_W::new(self)
    }
    #[doc = "Bit 3 - D+/D- Pull-Down Resistor Control"]
    #[inline(always)]
    #[must_use]
    pub fn drpd0(&mut self) -> DRPD0_W<3> {
        DRPD0_W::new(self)
    }
    #[doc = "Bit 4 - USB Transceiver Output Fix"]
    #[inline(always)]
    #[must_use]
    pub fn fixphy0(&mut self) -> FIXPHY0_W<4> {
        FIXPHY0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep Software Standby USB Transceiver Control/Pin Monitor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpusr0r](index.html) module"]
pub struct DPUSR0R_SPEC;
impl crate::RegisterSpec for DPUSR0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dpusr0r::R](R) reader structure"]
impl crate::Readable for DPUSR0R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpusr0r::W](W) writer structure"]
impl crate::Writable for DPUSR0R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPUSR0R to value 0"]
impl crate::Resettable for DPUSR0R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
