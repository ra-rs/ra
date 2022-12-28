#[doc = "Register `SYSSTS0` reader"]
pub struct R(crate::R<SYSSTS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSSTS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSSTS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSSTS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LNST` reader - USB Data Line Status Monitor"]
pub type LNST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDMON` reader - External ID0 Input Pin Monitor"]
pub type IDMON_R = crate::BitReader<IDMON_A>;
#[doc = "External ID0 Input Pin Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDMON_A {
    #[doc = "0: USB_ID pin is low"]
    _0 = 0,
    #[doc = "1: USB_ID pin is high"]
    _1 = 1,
}
impl From<IDMON_A> for bool {
    #[inline(always)]
    fn from(variant: IDMON_A) -> Self {
        variant as u8 != 0
    }
}
impl IDMON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDMON_A {
        match self.bits {
            false => IDMON_A::_0,
            true => IDMON_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDMON_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDMON_A::_1
    }
}
#[doc = "Field `SOFEA` reader - Active Monitor When the Host Controller Is Selected"]
pub type SOFEA_R = crate::BitReader<SOFEA_A>;
#[doc = "Active Monitor When the Host Controller Is Selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFEA_A {
    #[doc = "0: SOF output stopped"]
    _0 = 0,
    #[doc = "1: SOF output operating"]
    _1 = 1,
}
impl From<SOFEA_A> for bool {
    #[inline(always)]
    fn from(variant: SOFEA_A) -> Self {
        variant as u8 != 0
    }
}
impl SOFEA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFEA_A {
        match self.bits {
            false => SOFEA_A::_0,
            true => SOFEA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOFEA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOFEA_A::_1
    }
}
#[doc = "Field `HTACT` reader - USB Host Sequencer Status Monitor"]
pub type HTACT_R = crate::BitReader<HTACT_A>;
#[doc = "USB Host Sequencer Status Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTACT_A {
    #[doc = "0: Host sequencer completely stopped"]
    _0 = 0,
    #[doc = "1: Host sequencer not completely stopped"]
    _1 = 1,
}
impl From<HTACT_A> for bool {
    #[inline(always)]
    fn from(variant: HTACT_A) -> Self {
        variant as u8 != 0
    }
}
impl HTACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTACT_A {
        match self.bits {
            false => HTACT_A::_0,
            true => HTACT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HTACT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HTACT_A::_1
    }
}
#[doc = "Field `OVCMON` reader - External USB_OVRCURA/ USB_OVRCURB Input Pin Monitor"]
pub type OVCMON_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - USB Data Line Status Monitor"]
    #[inline(always)]
    pub fn lnst(&self) -> LNST_R {
        LNST_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - External ID0 Input Pin Monitor"]
    #[inline(always)]
    pub fn idmon(&self) -> IDMON_R {
        IDMON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Active Monitor When the Host Controller Is Selected"]
    #[inline(always)]
    pub fn sofea(&self) -> SOFEA_R {
        SOFEA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB Host Sequencer Status Monitor"]
    #[inline(always)]
    pub fn htact(&self) -> HTACT_R {
        HTACT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 14:15 - External USB_OVRCURA/ USB_OVRCURB Input Pin Monitor"]
    #[inline(always)]
    pub fn ovcmon(&self) -> OVCMON_R {
        OVCMON_R::new(((self.bits >> 14) & 3) as u8)
    }
}
#[doc = "System Configuration Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syssts0](index.html) module"]
pub struct SYSSTS0_SPEC;
impl crate::RegisterSpec for SYSSTS0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [syssts0::R](R) reader structure"]
impl crate::Readable for SYSSTS0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSSTS0 to value 0"]
impl crate::Resettable for SYSSTS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
