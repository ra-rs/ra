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
pub type LNST_R = crate::FieldReader<u8, LNST_A>;
#[doc = "USB Data Line Status Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LNST_A {
    #[doc = "0: SE0"]
    _00 = 0,
    #[doc = "1: K-State (FS) / J-State(LS)"]
    _01 = 1,
    #[doc = "2: J-State(FS) / K-State(LS)"]
    _10 = 2,
    #[doc = "3: SE1"]
    _11 = 3,
}
impl From<LNST_A> for u8 {
    #[inline(always)]
    fn from(variant: LNST_A) -> Self {
        variant as _
    }
}
impl LNST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LNST_A {
        match self.bits {
            0 => LNST_A::_00,
            1 => LNST_A::_01,
            2 => LNST_A::_10,
            3 => LNST_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LNST_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LNST_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == LNST_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == LNST_A::_11
    }
}
#[doc = "Field `IDMON` reader - External ID0 Input Pin Monitor"]
pub type IDMON_R = crate::BitReader<IDMON_A>;
#[doc = "External ID0 Input Pin Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDMON_A {
    #[doc = "0: USB0_ID pin is low"]
    _0 = 0,
    #[doc = "1: USB0_ID pin is high"]
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
#[doc = "Field `HTACT` reader - USB Host Sequencer Status Monitor"]
pub type HTACT_R = crate::BitReader<HTACT_A>;
#[doc = "USB Host Sequencer Status Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTACT_A {
    #[doc = "0: Host sequencer completely stopped"]
    _0 = 0,
    #[doc = "1: Host sequencer not completely stopped."]
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
#[doc = "Field `OVCMON` reader - External USB0_OVRCURA/ USB0_OVRCURB Input Pin Monitor The OCVMON\\[1\\]
bit indicates the status of the USBHS_OVRCURA pin. The OCVMON\\[0\\]
bit indicates the status of the USBHS_OVRCURB pin."]
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
    #[doc = "Bit 6 - USB Host Sequencer Status Monitor"]
    #[inline(always)]
    pub fn htact(&self) -> HTACT_R {
        HTACT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 14:15 - External USB0_OVRCURA/ USB0_OVRCURB Input Pin Monitor The OCVMON\\[1\\]
bit indicates the status of the USBHS_OVRCURA pin. The OCVMON\\[0\\]
bit indicates the status of the USBHS_OVRCURB pin."]
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
