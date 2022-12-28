#[doc = "Register `DCPMAXP` reader"]
pub struct R(crate::R<DCPMAXP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCPMAXP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCPMAXP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCPMAXP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCPMAXP` writer"]
pub struct W(crate::W<DCPMAXP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCPMAXP_SPEC>;
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
impl From<crate::W<DCPMAXP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCPMAXP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MXPS` reader - Maximum Packet Size These bits set the maximum amount of data (maximum packet size) in payloads for the DCP."]
pub type MXPS_R = crate::FieldReader<u8, MXPS_A>;
#[doc = "Maximum Packet Size These bits set the maximum amount of data (maximum packet size) in payloads for the DCP.\n\nValue on reset: 64"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MXPS_A {
    #[doc = "8: 8 bytes"]
    _0X08 = 8,
    #[doc = "16: 16 bytes"]
    _0X10 = 16,
    #[doc = "24: 24 bytes"]
    _0X18 = 24,
    #[doc = "32: 32 bytes"]
    _0X20 = 32,
    #[doc = "40: 40 bytes"]
    _0X28 = 40,
    #[doc = "48: 48 bytes"]
    _0X30 = 48,
    #[doc = "56: 56 bytes"]
    _0X38 = 56,
    #[doc = "64: 64 bytes"]
    _0X40 = 64,
    #[doc = "72: 72 bytes"]
    _0X48 = 72,
    #[doc = "80: 80 bytes"]
    _0X50 = 80,
    #[doc = "88: 88 bytes"]
    _0X58 = 88,
    #[doc = "96: 96 bytes"]
    _0X60 = 96,
    #[doc = "104: 104 bytes"]
    _0X68 = 104,
    #[doc = "112: 112 bytes"]
    _0X70 = 112,
    #[doc = "120: 120 bytes"]
    _0X78 = 120,
}
impl From<MXPS_A> for u8 {
    #[inline(always)]
    fn from(variant: MXPS_A) -> Self {
        variant as _
    }
}
impl MXPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MXPS_A> {
        match self.bits {
            8 => Some(MXPS_A::_0X08),
            16 => Some(MXPS_A::_0X10),
            24 => Some(MXPS_A::_0X18),
            32 => Some(MXPS_A::_0X20),
            40 => Some(MXPS_A::_0X28),
            48 => Some(MXPS_A::_0X30),
            56 => Some(MXPS_A::_0X38),
            64 => Some(MXPS_A::_0X40),
            72 => Some(MXPS_A::_0X48),
            80 => Some(MXPS_A::_0X50),
            88 => Some(MXPS_A::_0X58),
            96 => Some(MXPS_A::_0X60),
            104 => Some(MXPS_A::_0X68),
            112 => Some(MXPS_A::_0X70),
            120 => Some(MXPS_A::_0X78),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X08`"]
    #[inline(always)]
    pub fn is_0x08(&self) -> bool {
        *self == MXPS_A::_0X08
    }
    #[doc = "Checks if the value of the field is `_0X10`"]
    #[inline(always)]
    pub fn is_0x10(&self) -> bool {
        *self == MXPS_A::_0X10
    }
    #[doc = "Checks if the value of the field is `_0X18`"]
    #[inline(always)]
    pub fn is_0x18(&self) -> bool {
        *self == MXPS_A::_0X18
    }
    #[doc = "Checks if the value of the field is `_0X20`"]
    #[inline(always)]
    pub fn is_0x20(&self) -> bool {
        *self == MXPS_A::_0X20
    }
    #[doc = "Checks if the value of the field is `_0X28`"]
    #[inline(always)]
    pub fn is_0x28(&self) -> bool {
        *self == MXPS_A::_0X28
    }
    #[doc = "Checks if the value of the field is `_0X30`"]
    #[inline(always)]
    pub fn is_0x30(&self) -> bool {
        *self == MXPS_A::_0X30
    }
    #[doc = "Checks if the value of the field is `_0X38`"]
    #[inline(always)]
    pub fn is_0x38(&self) -> bool {
        *self == MXPS_A::_0X38
    }
    #[doc = "Checks if the value of the field is `_0X40`"]
    #[inline(always)]
    pub fn is_0x40(&self) -> bool {
        *self == MXPS_A::_0X40
    }
    #[doc = "Checks if the value of the field is `_0X48`"]
    #[inline(always)]
    pub fn is_0x48(&self) -> bool {
        *self == MXPS_A::_0X48
    }
    #[doc = "Checks if the value of the field is `_0X50`"]
    #[inline(always)]
    pub fn is_0x50(&self) -> bool {
        *self == MXPS_A::_0X50
    }
    #[doc = "Checks if the value of the field is `_0X58`"]
    #[inline(always)]
    pub fn is_0x58(&self) -> bool {
        *self == MXPS_A::_0X58
    }
    #[doc = "Checks if the value of the field is `_0X60`"]
    #[inline(always)]
    pub fn is_0x60(&self) -> bool {
        *self == MXPS_A::_0X60
    }
    #[doc = "Checks if the value of the field is `_0X68`"]
    #[inline(always)]
    pub fn is_0x68(&self) -> bool {
        *self == MXPS_A::_0X68
    }
    #[doc = "Checks if the value of the field is `_0X70`"]
    #[inline(always)]
    pub fn is_0x70(&self) -> bool {
        *self == MXPS_A::_0X70
    }
    #[doc = "Checks if the value of the field is `_0X78`"]
    #[inline(always)]
    pub fn is_0x78(&self) -> bool {
        *self == MXPS_A::_0X78
    }
}
#[doc = "Field `MXPS` writer - Maximum Packet Size These bits set the maximum amount of data (maximum packet size) in payloads for the DCP."]
pub type MXPS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DCPMAXP_SPEC, u8, MXPS_A, 7, O>;
impl<'a, const O: u8> MXPS_W<'a, O> {
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn _0x08(self) -> &'a mut W {
        self.variant(MXPS_A::_0X08)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn _0x10(self) -> &'a mut W {
        self.variant(MXPS_A::_0X10)
    }
    #[doc = "24 bytes"]
    #[inline(always)]
    pub fn _0x18(self) -> &'a mut W {
        self.variant(MXPS_A::_0X18)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _0x20(self) -> &'a mut W {
        self.variant(MXPS_A::_0X20)
    }
    #[doc = "40 bytes"]
    #[inline(always)]
    pub fn _0x28(self) -> &'a mut W {
        self.variant(MXPS_A::_0X28)
    }
    #[doc = "48 bytes"]
    #[inline(always)]
    pub fn _0x30(self) -> &'a mut W {
        self.variant(MXPS_A::_0X30)
    }
    #[doc = "56 bytes"]
    #[inline(always)]
    pub fn _0x38(self) -> &'a mut W {
        self.variant(MXPS_A::_0X38)
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _0x40(self) -> &'a mut W {
        self.variant(MXPS_A::_0X40)
    }
    #[doc = "72 bytes"]
    #[inline(always)]
    pub fn _0x48(self) -> &'a mut W {
        self.variant(MXPS_A::_0X48)
    }
    #[doc = "80 bytes"]
    #[inline(always)]
    pub fn _0x50(self) -> &'a mut W {
        self.variant(MXPS_A::_0X50)
    }
    #[doc = "88 bytes"]
    #[inline(always)]
    pub fn _0x58(self) -> &'a mut W {
        self.variant(MXPS_A::_0X58)
    }
    #[doc = "96 bytes"]
    #[inline(always)]
    pub fn _0x60(self) -> &'a mut W {
        self.variant(MXPS_A::_0X60)
    }
    #[doc = "104 bytes"]
    #[inline(always)]
    pub fn _0x68(self) -> &'a mut W {
        self.variant(MXPS_A::_0X68)
    }
    #[doc = "112 bytes"]
    #[inline(always)]
    pub fn _0x70(self) -> &'a mut W {
        self.variant(MXPS_A::_0X70)
    }
    #[doc = "120 bytes"]
    #[inline(always)]
    pub fn _0x78(self) -> &'a mut W {
        self.variant(MXPS_A::_0X78)
    }
}
#[doc = "Field `DEVSEL` reader - Device Select"]
pub type DEVSEL_R = crate::FieldReader<u8, DEVSEL_A>;
#[doc = "Device Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DEVSEL_A {
    #[doc = "0: Address 0000"]
    _0000 = 0,
    #[doc = "1: Address 0001"]
    _0001 = 1,
    #[doc = "2: Address 0010"]
    _0010 = 2,
    #[doc = "3: Address 0011"]
    _0011 = 3,
    #[doc = "4: Address 0100"]
    _0100 = 4,
    #[doc = "5: Address 0101"]
    _0101 = 5,
}
impl From<DEVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DEVSEL_A) -> Self {
        variant as _
    }
}
impl DEVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DEVSEL_A> {
        match self.bits {
            0 => Some(DEVSEL_A::_0000),
            1 => Some(DEVSEL_A::_0001),
            2 => Some(DEVSEL_A::_0010),
            3 => Some(DEVSEL_A::_0011),
            4 => Some(DEVSEL_A::_0100),
            5 => Some(DEVSEL_A::_0101),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == DEVSEL_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == DEVSEL_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == DEVSEL_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == DEVSEL_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == DEVSEL_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == DEVSEL_A::_0101
    }
}
#[doc = "Field `DEVSEL` writer - Device Select"]
pub type DEVSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DCPMAXP_SPEC, u8, DEVSEL_A, 4, O>;
impl<'a, const O: u8> DEVSEL_W<'a, O> {
    #[doc = "Address 0000"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(DEVSEL_A::_0000)
    }
    #[doc = "Address 0001"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(DEVSEL_A::_0001)
    }
    #[doc = "Address 0010"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(DEVSEL_A::_0010)
    }
    #[doc = "Address 0011"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(DEVSEL_A::_0011)
    }
    #[doc = "Address 0100"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(DEVSEL_A::_0100)
    }
    #[doc = "Address 0101"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(DEVSEL_A::_0101)
    }
}
impl R {
    #[doc = "Bits 0:6 - Maximum Packet Size These bits set the maximum amount of data (maximum packet size) in payloads for the DCP."]
    #[inline(always)]
    pub fn mxps(&self) -> MXPS_R {
        MXPS_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 12:15 - Device Select"]
    #[inline(always)]
    pub fn devsel(&self) -> DEVSEL_R {
        DEVSEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Maximum Packet Size These bits set the maximum amount of data (maximum packet size) in payloads for the DCP."]
    #[inline(always)]
    #[must_use]
    pub fn mxps(&mut self) -> MXPS_W<0> {
        MXPS_W::new(self)
    }
    #[doc = "Bits 12:15 - Device Select"]
    #[inline(always)]
    #[must_use]
    pub fn devsel(&mut self) -> DEVSEL_W<12> {
        DEVSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCP Maximum Packet Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcpmaxp](index.html) module"]
pub struct DCPMAXP_SPEC;
impl crate::RegisterSpec for DCPMAXP_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dcpmaxp::R](R) reader structure"]
impl crate::Readable for DCPMAXP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcpmaxp::W](W) writer structure"]
impl crate::Writable for DCPMAXP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCPMAXP to value 0x40"]
impl crate::Resettable for DCPMAXP_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
