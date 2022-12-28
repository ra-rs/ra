#[doc = "Register `ADSGDCR%s` reader"]
pub struct R(crate::R<ADSGDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADSGDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADSGDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADSGDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADSGDCR%s` writer"]
pub struct W(crate::W<ADSGDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSGDCR_SPEC>;
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
impl From<crate::W<ADSGDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSGDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIAGVAL` reader - Self-diagnosis Mode Selection"]
pub type DIAGVAL_R = crate::FieldReader<u8, DIAGVAL_A>;
#[doc = "Self-diagnosis Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIAGVAL_A {
    #[doc = "0: Set when any self-diagnosis channel are not included. Setting prohibited when any self-diagnosis channels are included."]
    _000 = 0,
    #[doc = "4: Self-diagnosis mode 1"]
    _100 = 4,
    #[doc = "5: Self-diagnosis mode 2"]
    _101 = 5,
    #[doc = "6: Self-diagnosis mode 3"]
    _110 = 6,
}
impl From<DIAGVAL_A> for u8 {
    #[inline(always)]
    fn from(variant: DIAGVAL_A) -> Self {
        variant as _
    }
}
impl DIAGVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIAGVAL_A> {
        match self.bits {
            0 => Some(DIAGVAL_A::_000),
            4 => Some(DIAGVAL_A::_100),
            5 => Some(DIAGVAL_A::_101),
            6 => Some(DIAGVAL_A::_110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DIAGVAL_A::_000
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DIAGVAL_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DIAGVAL_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == DIAGVAL_A::_110
    }
}
#[doc = "Field `DIAGVAL` writer - Self-diagnosis Mode Selection"]
pub type DIAGVAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADSGDCR_SPEC, u8, DIAGVAL_A, 3, O>;
impl<'a, const O: u8> DIAGVAL_W<'a, O> {
    #[doc = "Set when any self-diagnosis channel are not included. Setting prohibited when any self-diagnosis channels are included."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(DIAGVAL_A::_000)
    }
    #[doc = "Self-diagnosis mode 1"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(DIAGVAL_A::_100)
    }
    #[doc = "Self-diagnosis mode 2"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(DIAGVAL_A::_101)
    }
    #[doc = "Self-diagnosis mode 3"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(DIAGVAL_A::_110)
    }
}
#[doc = "Field `ADDISEN` reader - Disconnection Detection Assist Enable"]
pub type ADDISEN_R = crate::BitReader<ADDISEN_A>;
#[doc = "Disconnection Detection Assist Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDISEN_A {
    #[doc = "0: Disable the disconnection detection assist function"]
    _0 = 0,
    #[doc = "1: Enable the disconnection detection assist function"]
    _1 = 1,
}
impl From<ADDISEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADDISEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDISEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDISEN_A {
        match self.bits {
            false => ADDISEN_A::_0,
            true => ADDISEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADDISEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADDISEN_A::_1
    }
}
#[doc = "Field `ADDISEN` writer - Disconnection Detection Assist Enable"]
pub type ADDISEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSGDCR_SPEC, ADDISEN_A, O>;
impl<'a, const O: u8> ADDISEN_W<'a, O> {
    #[doc = "Disable the disconnection detection assist function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADDISEN_A::_0)
    }
    #[doc = "Enable the disconnection detection assist function"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADDISEN_A::_1)
    }
}
#[doc = "Field `ADDISP` reader - Disconnection Detection Assist Mode Selection"]
pub type ADDISP_R = crate::BitReader<ADDISP_A>;
#[doc = "Disconnection Detection Assist Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDISP_A {
    #[doc = "0: Discharge"]
    _0 = 0,
    #[doc = "1: Precharge"]
    _1 = 1,
}
impl From<ADDISP_A> for bool {
    #[inline(always)]
    fn from(variant: ADDISP_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDISP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDISP_A {
        match self.bits {
            false => ADDISP_A::_0,
            true => ADDISP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADDISP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADDISP_A::_1
    }
}
#[doc = "Field `ADDISP` writer - Disconnection Detection Assist Mode Selection"]
pub type ADDISP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSGDCR_SPEC, ADDISP_A, O>;
impl<'a, const O: u8> ADDISP_W<'a, O> {
    #[doc = "Discharge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADDISP_A::_0)
    }
    #[doc = "Precharge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADDISP_A::_1)
    }
}
#[doc = "Field `ADDISN` reader - Disconnection Detection Assist Mode Selection"]
pub type ADDISN_R = crate::BitReader<ADDISN_A>;
#[doc = "Disconnection Detection Assist Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDISN_A {
    #[doc = "0: Discharge"]
    _0 = 0,
    #[doc = "1: Precharge"]
    _1 = 1,
}
impl From<ADDISN_A> for bool {
    #[inline(always)]
    fn from(variant: ADDISN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDISN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDISN_A {
        match self.bits {
            false => ADDISN_A::_0,
            true => ADDISN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADDISN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADDISN_A::_1
    }
}
#[doc = "Field `ADDISN` writer - Disconnection Detection Assist Mode Selection"]
pub type ADDISN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSGDCR_SPEC, ADDISN_A, O>;
impl<'a, const O: u8> ADDISN_W<'a, O> {
    #[doc = "Discharge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADDISN_A::_0)
    }
    #[doc = "Precharge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADDISN_A::_1)
    }
}
#[doc = "Field `ADNDIS` reader - Disconnection Detection Assist Period"]
pub type ADNDIS_R = crate::FieldReader<u8, ADNDIS_A>;
#[doc = "Disconnection Detection Assist Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADNDIS_A {
    #[doc = "0: Setting prohibited when the disconnection detection assist function is enabled"]
    _0X0 = 0,
    #[doc = "1: Setting prohibited"]
    _0X1 = 1,
    #[doc = "2: Setting prohibited"]
    _0X2 = 2,
}
impl From<ADNDIS_A> for u8 {
    #[inline(always)]
    fn from(variant: ADNDIS_A) -> Self {
        variant as _
    }
}
impl ADNDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADNDIS_A> {
        match self.bits {
            0 => Some(ADNDIS_A::_0X0),
            1 => Some(ADNDIS_A::_0X1),
            2 => Some(ADNDIS_A::_0X2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == ADNDIS_A::_0X0
    }
    #[doc = "Checks if the value of the field is `_0X1`"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == ADNDIS_A::_0X1
    }
    #[doc = "Checks if the value of the field is `_0X2`"]
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == ADNDIS_A::_0X2
    }
}
#[doc = "Field `ADNDIS` writer - Disconnection Detection Assist Period"]
pub type ADNDIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSGDCR_SPEC, u8, ADNDIS_A, 4, O>;
impl<'a, const O: u8> ADNDIS_W<'a, O> {
    #[doc = "Setting prohibited when the disconnection detection assist function is enabled"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(ADNDIS_A::_0X0)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut W {
        self.variant(ADNDIS_A::_0X1)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut W {
        self.variant(ADNDIS_A::_0X2)
    }
}
impl R {
    #[doc = "Bits 0:2 - Self-diagnosis Mode Selection"]
    #[inline(always)]
    pub fn diagval(&self) -> DIAGVAL_R {
        DIAGVAL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 16 - Disconnection Detection Assist Enable"]
    #[inline(always)]
    pub fn addisen(&self) -> ADDISEN_R {
        ADDISEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Disconnection Detection Assist Mode Selection"]
    #[inline(always)]
    pub fn addisp(&self) -> ADDISP_R {
        ADDISP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Disconnection Detection Assist Mode Selection"]
    #[inline(always)]
    pub fn addisn(&self) -> ADDISN_R {
        ADDISN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Disconnection Detection Assist Period"]
    #[inline(always)]
    pub fn adndis(&self) -> ADNDIS_R {
        ADNDIS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Self-diagnosis Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn diagval(&mut self) -> DIAGVAL_W<0> {
        DIAGVAL_W::new(self)
    }
    #[doc = "Bit 16 - Disconnection Detection Assist Enable"]
    #[inline(always)]
    #[must_use]
    pub fn addisen(&mut self) -> ADDISEN_W<16> {
        ADDISEN_W::new(self)
    }
    #[doc = "Bit 20 - Disconnection Detection Assist Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn addisp(&mut self) -> ADDISP_W<20> {
        ADDISP_W::new(self)
    }
    #[doc = "Bit 21 - Disconnection Detection Assist Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn addisn(&mut self) -> ADDISN_W<21> {
        ADDISN_W::new(self)
    }
    #[doc = "Bits 24:27 - Disconnection Detection Assist Period"]
    #[inline(always)]
    #[must_use]
    pub fn adndis(&mut self) -> ADNDIS_W<24> {
        ADNDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scan Group Diagnosis Function Control Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adsgdcr](index.html) module"]
pub struct ADSGDCR_SPEC;
impl crate::RegisterSpec for ADSGDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adsgdcr::R](R) reader structure"]
impl crate::Readable for ADSGDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adsgdcr::W](W) writer structure"]
impl crate::Writable for ADSGDCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSGDCR%s to value 0"]
impl crate::Resettable for ADSGDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
