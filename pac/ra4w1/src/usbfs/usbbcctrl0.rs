#[doc = "Register `USBBCCTRL0` reader"]
pub struct R(crate::R<USBBCCTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBBCCTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBBCCTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBBCCTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBBCCTRL0` writer"]
pub struct W(crate::W<USBBCCTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBBCCTRL0_SPEC>;
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
impl From<crate::W<USBBCCTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBBCCTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPDME0` reader - D- Pin Pull-Down Control"]
pub type RPDME0_R = crate::BitReader<RPDME0_A>;
#[doc = "D- Pin Pull-Down Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPDME0_A {
    #[doc = "0: Pull-down off"]
    _0 = 0,
    #[doc = "1: Pull-down on"]
    _1 = 1,
}
impl From<RPDME0_A> for bool {
    #[inline(always)]
    fn from(variant: RPDME0_A) -> Self {
        variant as u8 != 0
    }
}
impl RPDME0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPDME0_A {
        match self.bits {
            false => RPDME0_A::_0,
            true => RPDME0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPDME0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPDME0_A::_1
    }
}
#[doc = "Field `RPDME0` writer - D- Pin Pull-Down Control"]
pub type RPDME0_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBBCCTRL0_SPEC, RPDME0_A, O>;
impl<'a, const O: u8> RPDME0_W<'a, O> {
    #[doc = "Pull-down off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RPDME0_A::_0)
    }
    #[doc = "Pull-down on"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RPDME0_A::_1)
    }
}
#[doc = "Field `IDPSRCE0` reader - D+ Pin IDPSRC Output Control"]
pub type IDPSRCE0_R = crate::BitReader<IDPSRCE0_A>;
#[doc = "D+ Pin IDPSRC Output Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDPSRCE0_A {
    #[doc = "0: Stop"]
    _0 = 0,
    #[doc = "1: 10uA output"]
    _1 = 1,
}
impl From<IDPSRCE0_A> for bool {
    #[inline(always)]
    fn from(variant: IDPSRCE0_A) -> Self {
        variant as u8 != 0
    }
}
impl IDPSRCE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDPSRCE0_A {
        match self.bits {
            false => IDPSRCE0_A::_0,
            true => IDPSRCE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDPSRCE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDPSRCE0_A::_1
    }
}
#[doc = "Field `IDPSRCE0` writer - D+ Pin IDPSRC Output Control"]
pub type IDPSRCE0_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBBCCTRL0_SPEC, IDPSRCE0_A, O>;
impl<'a, const O: u8> IDPSRCE0_W<'a, O> {
    #[doc = "Stop"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IDPSRCE0_A::_0)
    }
    #[doc = "10uA output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IDPSRCE0_A::_1)
    }
}
#[doc = "Field `IDMSINKE0` reader - D- Pin 0.6 V Input Detection (Comparator and Sink) Control"]
pub type IDMSINKE0_R = crate::BitReader<IDMSINKE0_A>;
#[doc = "D- Pin 0.6 V Input Detection (Comparator and Sink) Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDMSINKE0_A {
    #[doc = "0: Detection off"]
    _0 = 0,
    #[doc = "1: Detection on ( Comparator and sink current on )"]
    _1 = 1,
}
impl From<IDMSINKE0_A> for bool {
    #[inline(always)]
    fn from(variant: IDMSINKE0_A) -> Self {
        variant as u8 != 0
    }
}
impl IDMSINKE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDMSINKE0_A {
        match self.bits {
            false => IDMSINKE0_A::_0,
            true => IDMSINKE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDMSINKE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDMSINKE0_A::_1
    }
}
#[doc = "Field `IDMSINKE0` writer - D- Pin 0.6 V Input Detection (Comparator and Sink) Control"]
pub type IDMSINKE0_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBBCCTRL0_SPEC, IDMSINKE0_A, O>;
impl<'a, const O: u8> IDMSINKE0_W<'a, O> {
    #[doc = "Detection off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IDMSINKE0_A::_0)
    }
    #[doc = "Detection on ( Comparator and sink current on )"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IDMSINKE0_A::_1)
    }
}
#[doc = "Field `VDPSRCE0` reader - D+ Pin VDPSRC (0.6 V) Output Control"]
pub type VDPSRCE0_R = crate::BitReader<VDPSRCE0_A>;
#[doc = "D+ Pin VDPSRC (0.6 V) Output Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDPSRCE0_A {
    #[doc = "0: Stop"]
    _0 = 0,
    #[doc = "1: 0.6V output"]
    _1 = 1,
}
impl From<VDPSRCE0_A> for bool {
    #[inline(always)]
    fn from(variant: VDPSRCE0_A) -> Self {
        variant as u8 != 0
    }
}
impl VDPSRCE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDPSRCE0_A {
        match self.bits {
            false => VDPSRCE0_A::_0,
            true => VDPSRCE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VDPSRCE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VDPSRCE0_A::_1
    }
}
#[doc = "Field `VDPSRCE0` writer - D+ Pin VDPSRC (0.6 V) Output Control"]
pub type VDPSRCE0_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBBCCTRL0_SPEC, VDPSRCE0_A, O>;
impl<'a, const O: u8> VDPSRCE0_W<'a, O> {
    #[doc = "Stop"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VDPSRCE0_A::_0)
    }
    #[doc = "0.6V output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VDPSRCE0_A::_1)
    }
}
#[doc = "Field `IDPSINKE0` reader - D+ Pin 0.6 V Input Detection (Comparator and Sink) Control"]
pub type IDPSINKE0_R = crate::BitReader<IDPSINKE0_A>;
#[doc = "D+ Pin 0.6 V Input Detection (Comparator and Sink) Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDPSINKE0_A {
    #[doc = "0: Detection off"]
    _0 = 0,
    #[doc = "1: Detection on ( Comparator and sink current on )"]
    _1 = 1,
}
impl From<IDPSINKE0_A> for bool {
    #[inline(always)]
    fn from(variant: IDPSINKE0_A) -> Self {
        variant as u8 != 0
    }
}
impl IDPSINKE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDPSINKE0_A {
        match self.bits {
            false => IDPSINKE0_A::_0,
            true => IDPSINKE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDPSINKE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDPSINKE0_A::_1
    }
}
#[doc = "Field `IDPSINKE0` writer - D+ Pin 0.6 V Input Detection (Comparator and Sink) Control"]
pub type IDPSINKE0_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBBCCTRL0_SPEC, IDPSINKE0_A, O>;
impl<'a, const O: u8> IDPSINKE0_W<'a, O> {
    #[doc = "Detection off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IDPSINKE0_A::_0)
    }
    #[doc = "Detection on ( Comparator and sink current on )"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IDPSINKE0_A::_1)
    }
}
#[doc = "Field `VDMSRCE0` reader - D- Pin VDMSRC (0.6 V) Output Control"]
pub type VDMSRCE0_R = crate::BitReader<VDMSRCE0_A>;
#[doc = "D- Pin VDMSRC (0.6 V) Output Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDMSRCE0_A {
    #[doc = "0: Stop"]
    _0 = 0,
    #[doc = "1: 0.6V output"]
    _1 = 1,
}
impl From<VDMSRCE0_A> for bool {
    #[inline(always)]
    fn from(variant: VDMSRCE0_A) -> Self {
        variant as u8 != 0
    }
}
impl VDMSRCE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDMSRCE0_A {
        match self.bits {
            false => VDMSRCE0_A::_0,
            true => VDMSRCE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VDMSRCE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VDMSRCE0_A::_1
    }
}
#[doc = "Field `VDMSRCE0` writer - D- Pin VDMSRC (0.6 V) Output Control"]
pub type VDMSRCE0_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBBCCTRL0_SPEC, VDMSRCE0_A, O>;
impl<'a, const O: u8> VDMSRCE0_W<'a, O> {
    #[doc = "Stop"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VDMSRCE0_A::_0)
    }
    #[doc = "0.6V output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VDMSRCE0_A::_1)
    }
}
#[doc = "Field `BATCHGE0` reader - BC (Battery Charger) Function Ch0 General Enable Control"]
pub type BATCHGE0_R = crate::BitReader<BATCHGE0_A>;
#[doc = "BC (Battery Charger) Function Ch0 General Enable Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BATCHGE0_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<BATCHGE0_A> for bool {
    #[inline(always)]
    fn from(variant: BATCHGE0_A) -> Self {
        variant as u8 != 0
    }
}
impl BATCHGE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BATCHGE0_A {
        match self.bits {
            false => BATCHGE0_A::_0,
            true => BATCHGE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BATCHGE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BATCHGE0_A::_1
    }
}
#[doc = "Field `BATCHGE0` writer - BC (Battery Charger) Function Ch0 General Enable Control"]
pub type BATCHGE0_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBBCCTRL0_SPEC, BATCHGE0_A, O>;
impl<'a, const O: u8> BATCHGE0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BATCHGE0_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BATCHGE0_A::_1)
    }
}
#[doc = "Field `CHGDETSTS0` reader - D- Pin 0.6 V Input Detection Status"]
pub type CHGDETSTS0_R = crate::BitReader<CHGDETSTS0_A>;
#[doc = "D- Pin 0.6 V Input Detection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHGDETSTS0_A {
    #[doc = "0: Not detected"]
    _0 = 0,
    #[doc = "1: Detected"]
    _1 = 1,
}
impl From<CHGDETSTS0_A> for bool {
    #[inline(always)]
    fn from(variant: CHGDETSTS0_A) -> Self {
        variant as u8 != 0
    }
}
impl CHGDETSTS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHGDETSTS0_A {
        match self.bits {
            false => CHGDETSTS0_A::_0,
            true => CHGDETSTS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHGDETSTS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHGDETSTS0_A::_1
    }
}
#[doc = "Field `PDDETSTS0` reader - D+ Pin 0.6 V Input Detection Status"]
pub type PDDETSTS0_R = crate::BitReader<PDDETSTS0_A>;
#[doc = "D+ Pin 0.6 V Input Detection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDDETSTS0_A {
    #[doc = "0: Not detected"]
    _0 = 0,
    #[doc = "1: Detected"]
    _1 = 1,
}
impl From<PDDETSTS0_A> for bool {
    #[inline(always)]
    fn from(variant: PDDETSTS0_A) -> Self {
        variant as u8 != 0
    }
}
impl PDDETSTS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDDETSTS0_A {
        match self.bits {
            false => PDDETSTS0_A::_0,
            true => PDDETSTS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDDETSTS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDDETSTS0_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - D- Pin Pull-Down Control"]
    #[inline(always)]
    pub fn rpdme0(&self) -> RPDME0_R {
        RPDME0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - D+ Pin IDPSRC Output Control"]
    #[inline(always)]
    pub fn idpsrce0(&self) -> IDPSRCE0_R {
        IDPSRCE0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - D- Pin 0.6 V Input Detection (Comparator and Sink) Control"]
    #[inline(always)]
    pub fn idmsinke0(&self) -> IDMSINKE0_R {
        IDMSINKE0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - D+ Pin VDPSRC (0.6 V) Output Control"]
    #[inline(always)]
    pub fn vdpsrce0(&self) -> VDPSRCE0_R {
        VDPSRCE0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - D+ Pin 0.6 V Input Detection (Comparator and Sink) Control"]
    #[inline(always)]
    pub fn idpsinke0(&self) -> IDPSINKE0_R {
        IDPSINKE0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - D- Pin VDMSRC (0.6 V) Output Control"]
    #[inline(always)]
    pub fn vdmsrce0(&self) -> VDMSRCE0_R {
        VDMSRCE0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - BC (Battery Charger) Function Ch0 General Enable Control"]
    #[inline(always)]
    pub fn batchge0(&self) -> BATCHGE0_R {
        BATCHGE0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - D- Pin 0.6 V Input Detection Status"]
    #[inline(always)]
    pub fn chgdetsts0(&self) -> CHGDETSTS0_R {
        CHGDETSTS0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - D+ Pin 0.6 V Input Detection Status"]
    #[inline(always)]
    pub fn pddetsts0(&self) -> PDDETSTS0_R {
        PDDETSTS0_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - D- Pin Pull-Down Control"]
    #[inline(always)]
    #[must_use]
    pub fn rpdme0(&mut self) -> RPDME0_W<0> {
        RPDME0_W::new(self)
    }
    #[doc = "Bit 1 - D+ Pin IDPSRC Output Control"]
    #[inline(always)]
    #[must_use]
    pub fn idpsrce0(&mut self) -> IDPSRCE0_W<1> {
        IDPSRCE0_W::new(self)
    }
    #[doc = "Bit 2 - D- Pin 0.6 V Input Detection (Comparator and Sink) Control"]
    #[inline(always)]
    #[must_use]
    pub fn idmsinke0(&mut self) -> IDMSINKE0_W<2> {
        IDMSINKE0_W::new(self)
    }
    #[doc = "Bit 3 - D+ Pin VDPSRC (0.6 V) Output Control"]
    #[inline(always)]
    #[must_use]
    pub fn vdpsrce0(&mut self) -> VDPSRCE0_W<3> {
        VDPSRCE0_W::new(self)
    }
    #[doc = "Bit 4 - D+ Pin 0.6 V Input Detection (Comparator and Sink) Control"]
    #[inline(always)]
    #[must_use]
    pub fn idpsinke0(&mut self) -> IDPSINKE0_W<4> {
        IDPSINKE0_W::new(self)
    }
    #[doc = "Bit 5 - D- Pin VDMSRC (0.6 V) Output Control"]
    #[inline(always)]
    #[must_use]
    pub fn vdmsrce0(&mut self) -> VDMSRCE0_W<5> {
        VDMSRCE0_W::new(self)
    }
    #[doc = "Bit 7 - BC (Battery Charger) Function Ch0 General Enable Control"]
    #[inline(always)]
    #[must_use]
    pub fn batchge0(&mut self) -> BATCHGE0_W<7> {
        BATCHGE0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BC Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbbcctrl0](index.html) module"]
pub struct USBBCCTRL0_SPEC;
impl crate::RegisterSpec for USBBCCTRL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usbbcctrl0::R](R) reader structure"]
impl crate::Readable for USBBCCTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbbcctrl0::W](W) writer structure"]
impl crate::Writable for USBBCCTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBBCCTRL0 to value 0"]
impl crate::Resettable for USBBCCTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
