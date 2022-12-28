#[doc = "Register `BCCTRL` reader"]
pub struct R(crate::R<BCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCCTRL` writer"]
pub struct W(crate::W<BCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCCTRL_SPEC>;
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
impl From<crate::W<BCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDPSRCE` reader - IDPSRC Control"]
pub type IDPSRCE_R = crate::BitReader<IDPSRCE_A>;
#[doc = "IDPSRC Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDPSRCE_A {
    #[doc = "0: The IDP_SRC circuit is disabled. (Initial value)"]
    _0 = 0,
    #[doc = "1: The IDP_SRC circuit is enabled."]
    _1 = 1,
}
impl From<IDPSRCE_A> for bool {
    #[inline(always)]
    fn from(variant: IDPSRCE_A) -> Self {
        variant as u8 != 0
    }
}
impl IDPSRCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDPSRCE_A {
        match self.bits {
            false => IDPSRCE_A::_0,
            true => IDPSRCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDPSRCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDPSRCE_A::_1
    }
}
#[doc = "Field `IDPSRCE` writer - IDPSRC Control"]
pub type IDPSRCE_W<'a, const O: u8> = crate::BitWriter<'a, u16, BCCTRL_SPEC, IDPSRCE_A, O>;
impl<'a, const O: u8> IDPSRCE_W<'a, O> {
    #[doc = "The IDP_SRC circuit is disabled. (Initial value)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IDPSRCE_A::_0)
    }
    #[doc = "The IDP_SRC circuit is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IDPSRCE_A::_1)
    }
}
#[doc = "Field `IDMSINKE` reader - IDMSINK Control"]
pub type IDMSINKE_R = crate::BitReader<IDMSINKE_A>;
#[doc = "IDMSINK Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDMSINKE_A {
    #[doc = "0: The IDM_SINK circuit is disabled. (Initial value)"]
    _0 = 0,
    #[doc = "1: The IDM_SINK circuit is enabled."]
    _1 = 1,
}
impl From<IDMSINKE_A> for bool {
    #[inline(always)]
    fn from(variant: IDMSINKE_A) -> Self {
        variant as u8 != 0
    }
}
impl IDMSINKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDMSINKE_A {
        match self.bits {
            false => IDMSINKE_A::_0,
            true => IDMSINKE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDMSINKE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDMSINKE_A::_1
    }
}
#[doc = "Field `IDMSINKE` writer - IDMSINK Control"]
pub type IDMSINKE_W<'a, const O: u8> = crate::BitWriter<'a, u16, BCCTRL_SPEC, IDMSINKE_A, O>;
impl<'a, const O: u8> IDMSINKE_W<'a, O> {
    #[doc = "The IDM_SINK circuit is disabled. (Initial value)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IDMSINKE_A::_0)
    }
    #[doc = "The IDM_SINK circuit is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IDMSINKE_A::_1)
    }
}
#[doc = "Field `VDPSRCE` reader - VDPSRC Control"]
pub type VDPSRCE_R = crate::BitReader<VDPSRCE_A>;
#[doc = "VDPSRC Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDPSRCE_A {
    #[doc = "0: The VDP_SRC circuit is disabled. (Initial value)"]
    _0 = 0,
    #[doc = "1: The VDP_SRC circuit is enabled."]
    _1 = 1,
}
impl From<VDPSRCE_A> for bool {
    #[inline(always)]
    fn from(variant: VDPSRCE_A) -> Self {
        variant as u8 != 0
    }
}
impl VDPSRCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDPSRCE_A {
        match self.bits {
            false => VDPSRCE_A::_0,
            true => VDPSRCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VDPSRCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VDPSRCE_A::_1
    }
}
#[doc = "Field `VDPSRCE` writer - VDPSRC Control"]
pub type VDPSRCE_W<'a, const O: u8> = crate::BitWriter<'a, u16, BCCTRL_SPEC, VDPSRCE_A, O>;
impl<'a, const O: u8> VDPSRCE_W<'a, O> {
    #[doc = "The VDP_SRC circuit is disabled. (Initial value)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VDPSRCE_A::_0)
    }
    #[doc = "The VDP_SRC circuit is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VDPSRCE_A::_1)
    }
}
#[doc = "Field `IDPSINKE` reader - IDPSINK Control"]
pub type IDPSINKE_R = crate::BitReader<IDPSINKE_A>;
#[doc = "IDPSINK Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDPSINKE_A {
    #[doc = "0: The IDP_SINK circuit is disabled. (Initial value)"]
    _0 = 0,
    #[doc = "1: The IDP_SINK circuit is enabled."]
    _1 = 1,
}
impl From<IDPSINKE_A> for bool {
    #[inline(always)]
    fn from(variant: IDPSINKE_A) -> Self {
        variant as u8 != 0
    }
}
impl IDPSINKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDPSINKE_A {
        match self.bits {
            false => IDPSINKE_A::_0,
            true => IDPSINKE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDPSINKE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDPSINKE_A::_1
    }
}
#[doc = "Field `IDPSINKE` writer - IDPSINK Control"]
pub type IDPSINKE_W<'a, const O: u8> = crate::BitWriter<'a, u16, BCCTRL_SPEC, IDPSINKE_A, O>;
impl<'a, const O: u8> IDPSINKE_W<'a, O> {
    #[doc = "The IDP_SINK circuit is disabled. (Initial value)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IDPSINKE_A::_0)
    }
    #[doc = "The IDP_SINK circuit is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IDPSINKE_A::_1)
    }
}
#[doc = "Field `VDMSRCE` reader - VDMSRC Control"]
pub type VDMSRCE_R = crate::BitReader<VDMSRCE_A>;
#[doc = "VDMSRC Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDMSRCE_A {
    #[doc = "0: The VDM_SRC circuit is disabled. (Initial value)"]
    _0 = 0,
    #[doc = "1: The VDM_SRC circuit is enabled."]
    _1 = 1,
}
impl From<VDMSRCE_A> for bool {
    #[inline(always)]
    fn from(variant: VDMSRCE_A) -> Self {
        variant as u8 != 0
    }
}
impl VDMSRCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDMSRCE_A {
        match self.bits {
            false => VDMSRCE_A::_0,
            true => VDMSRCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VDMSRCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VDMSRCE_A::_1
    }
}
#[doc = "Field `VDMSRCE` writer - VDMSRC Control"]
pub type VDMSRCE_W<'a, const O: u8> = crate::BitWriter<'a, u16, BCCTRL_SPEC, VDMSRCE_A, O>;
impl<'a, const O: u8> VDMSRCE_W<'a, O> {
    #[doc = "The VDM_SRC circuit is disabled. (Initial value)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VDMSRCE_A::_0)
    }
    #[doc = "The VDM_SRC circuit is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VDMSRCE_A::_1)
    }
}
#[doc = "Field `DCPMODE` reader - DCP Mode Control"]
pub type DCPMODE_R = crate::BitReader<DCPMODE_A>;
#[doc = "DCP Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCPMODE_A {
    #[doc = "0: The RDCP_DAT resistor is disabled"]
    _0 = 0,
    #[doc = "1: The RDCP_DAT resistor is enabled."]
    _1 = 1,
}
impl From<DCPMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DCPMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl DCPMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCPMODE_A {
        match self.bits {
            false => DCPMODE_A::_0,
            true => DCPMODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCPMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCPMODE_A::_1
    }
}
#[doc = "Field `DCPMODE` writer - DCP Mode Control"]
pub type DCPMODE_W<'a, const O: u8> = crate::BitWriter<'a, u16, BCCTRL_SPEC, DCPMODE_A, O>;
impl<'a, const O: u8> DCPMODE_W<'a, O> {
    #[doc = "The RDCP_DAT resistor is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCPMODE_A::_0)
    }
    #[doc = "The RDCP_DAT resistor is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCPMODE_A::_1)
    }
}
#[doc = "Field `CHGDETSTS` reader - CHGDET Status"]
pub type CHGDETSTS_R = crate::BitReader<CHGDETSTS_A>;
#[doc = "CHGDET Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHGDETSTS_A {
    #[doc = "0: The CHGDET pin is at low level."]
    _0 = 0,
    #[doc = "1: The CHGDET pin is at high level."]
    _1 = 1,
}
impl From<CHGDETSTS_A> for bool {
    #[inline(always)]
    fn from(variant: CHGDETSTS_A) -> Self {
        variant as u8 != 0
    }
}
impl CHGDETSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHGDETSTS_A {
        match self.bits {
            false => CHGDETSTS_A::_0,
            true => CHGDETSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHGDETSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHGDETSTS_A::_1
    }
}
#[doc = "Field `PDDETSTS` reader - PDDET Status"]
pub type PDDETSTS_R = crate::BitReader<PDDETSTS_A>;
#[doc = "PDDET Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDDETSTS_A {
    #[doc = "0: The PDDET pin is at low level."]
    _0 = 0,
    #[doc = "1: The PDDET pin is at high level."]
    _1 = 1,
}
impl From<PDDETSTS_A> for bool {
    #[inline(always)]
    fn from(variant: PDDETSTS_A) -> Self {
        variant as u8 != 0
    }
}
impl PDDETSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDDETSTS_A {
        match self.bits {
            false => PDDETSTS_A::_0,
            true => PDDETSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDDETSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDDETSTS_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - IDPSRC Control"]
    #[inline(always)]
    pub fn idpsrce(&self) -> IDPSRCE_R {
        IDPSRCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IDMSINK Control"]
    #[inline(always)]
    pub fn idmsinke(&self) -> IDMSINKE_R {
        IDMSINKE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VDPSRC Control"]
    #[inline(always)]
    pub fn vdpsrce(&self) -> VDPSRCE_R {
        VDPSRCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IDPSINK Control"]
    #[inline(always)]
    pub fn idpsinke(&self) -> IDPSINKE_R {
        IDPSINKE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VDMSRC Control"]
    #[inline(always)]
    pub fn vdmsrce(&self) -> VDMSRCE_R {
        VDMSRCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DCP Mode Control"]
    #[inline(always)]
    pub fn dcpmode(&self) -> DCPMODE_R {
        DCPMODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CHGDET Status"]
    #[inline(always)]
    pub fn chgdetsts(&self) -> CHGDETSTS_R {
        CHGDETSTS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PDDET Status"]
    #[inline(always)]
    pub fn pddetsts(&self) -> PDDETSTS_R {
        PDDETSTS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IDPSRC Control"]
    #[inline(always)]
    #[must_use]
    pub fn idpsrce(&mut self) -> IDPSRCE_W<0> {
        IDPSRCE_W::new(self)
    }
    #[doc = "Bit 1 - IDMSINK Control"]
    #[inline(always)]
    #[must_use]
    pub fn idmsinke(&mut self) -> IDMSINKE_W<1> {
        IDMSINKE_W::new(self)
    }
    #[doc = "Bit 2 - VDPSRC Control"]
    #[inline(always)]
    #[must_use]
    pub fn vdpsrce(&mut self) -> VDPSRCE_W<2> {
        VDPSRCE_W::new(self)
    }
    #[doc = "Bit 3 - IDPSINK Control"]
    #[inline(always)]
    #[must_use]
    pub fn idpsinke(&mut self) -> IDPSINKE_W<3> {
        IDPSINKE_W::new(self)
    }
    #[doc = "Bit 4 - VDMSRC Control"]
    #[inline(always)]
    #[must_use]
    pub fn vdmsrce(&mut self) -> VDMSRCE_W<4> {
        VDMSRCE_W::new(self)
    }
    #[doc = "Bit 5 - DCP Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn dcpmode(&mut self) -> DCPMODE_W<5> {
        DCPMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Battery Charging Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcctrl](index.html) module"]
pub struct BCCTRL_SPEC;
impl crate::RegisterSpec for BCCTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [bcctrl::R](R) reader structure"]
impl crate::Readable for BCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcctrl::W](W) writer structure"]
impl crate::Writable for BCCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCCTRL to value 0"]
impl crate::Resettable for BCCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
