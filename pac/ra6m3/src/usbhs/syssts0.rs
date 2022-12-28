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
    #[doc = "0: SE0 (During Low-Speed Operation:only when the host controller function is selected or During Full-Speed Operation) /Squelch (During Hi-Speed Operation or During Chirp Operation)"]
    _00 = 0,
    #[doc = "1: K-State (During Low-Speed Operation:only when the host controller function is selected) /J-State (During Full-Speed Operation) /Unsquelch (During Hi-Speed Operation) /Chirp J (During Chirp Operation)"]
    _01 = 1,
    #[doc = "2: J-State (During Low-Speed Operation:only when the host controller function is selected) /K-State (During Full-Speed Operation) /Invalid (During Hi-Speed Operation) /Chirp K (During Chirp Operation)"]
    _10 = 2,
    #[doc = "3: SE1 (During Low-Speed Operation:only when the host controller function is selected or During Full-Speed Operation) /Invalid (During Hi-Speed Operation or During Chirp Operation)"]
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
#[doc = "Field `IDMON` reader - ID0 Pin Monitor"]
pub type IDMON_R = crate::BitReader<IDMON_A>;
#[doc = "ID0 Pin Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDMON_A {
    #[doc = "0: ID0 = Low"]
    _0 = 0,
    #[doc = "1: ID0 = High"]
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
#[doc = "Field `SOFEA` reader - SOF Active Monitor While Host Controller Function is Selected."]
pub type SOFEA_R = crate::BitReader<SOFEA_A>;
#[doc = "SOF Active Monitor While Host Controller Function is Selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFEA_A {
    #[doc = "0: SOF output is stopped."]
    _0 = 0,
    #[doc = "1: SOF output is operating."]
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
#[doc = "Field `HTACT` reader - Host Sequencer Status Monitor"]
pub type HTACT_R = crate::BitReader<HTACT_A>;
#[doc = "Host Sequencer Status Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTACT_A {
    #[doc = "0: Host sequencer is stopped."]
    _0 = 0,
    #[doc = "1: Host sequencer is operating."]
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
#[doc = "Field `OVCMON` reader - External USB1_OVRCURA/USB1_OVRCURB Input Pin MonitorThe OCVMON\\[1\\]
bit indicates the status of the USBHS_OVRCURA pin. The OCVMON\\[0\\]
bit indicates the status of the USBHS_OVRCURB pin."]
pub type OVCMON_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - USB Data Line Status Monitor"]
    #[inline(always)]
    pub fn lnst(&self) -> LNST_R {
        LNST_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - ID0 Pin Monitor"]
    #[inline(always)]
    pub fn idmon(&self) -> IDMON_R {
        IDMON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - SOF Active Monitor While Host Controller Function is Selected."]
    #[inline(always)]
    pub fn sofea(&self) -> SOFEA_R {
        SOFEA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Host Sequencer Status Monitor"]
    #[inline(always)]
    pub fn htact(&self) -> HTACT_R {
        HTACT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 14:15 - External USB1_OVRCURA/USB1_OVRCURB Input Pin MonitorThe OCVMON\\[1\\]
bit indicates the status of the USBHS_OVRCURA pin. The OCVMON\\[0\\]
bit indicates the status of the USBHS_OVRCURB pin."]
    #[inline(always)]
    pub fn ovcmon(&self) -> OVCMON_R {
        OVCMON_R::new(((self.bits >> 14) & 3) as u8)
    }
}
#[doc = "System Configuration Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syssts0](index.html) module"]
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
