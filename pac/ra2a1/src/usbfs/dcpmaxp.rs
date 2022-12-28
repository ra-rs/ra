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
#[doc = "Field `MXPS` reader - Maximum Packet SizeThese bits set the maximum amount of data (maximum packet size) in payloads for the DCP."]
pub type MXPS_R = crate::FieldReader<u8, MXPS_A>;
#[doc = "Maximum Packet SizeThese bits set the maximum amount of data (maximum packet size) in payloads for the DCP.\n\nValue on reset: 64"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MXPS_A {
    #[doc = "8: 8bytes"]
    _0001000 = 8,
    #[doc = "16: 16bytes"]
    _0010000 = 16,
    #[doc = "24: 24bytes"]
    _0011000 = 24,
    #[doc = "32: 32bytes"]
    _0100000 = 32,
    #[doc = "40: 40bytes"]
    _0101000 = 40,
    #[doc = "48: 48bytes"]
    _0110000 = 48,
    #[doc = "56: 56bytes"]
    _0111000 = 56,
    #[doc = "64: 64bytes"]
    _1000000 = 64,
    #[doc = "72: 72bytes"]
    _1001000 = 72,
    #[doc = "80: 80bytes"]
    _1010000 = 80,
    #[doc = "88: 88bytes"]
    _1011000 = 88,
    #[doc = "96: 96bytes"]
    _1100000 = 96,
    #[doc = "104: 104bytes"]
    _1101000 = 104,
    #[doc = "112: 112bytes"]
    _1110000 = 112,
    #[doc = "120: 120bytes"]
    _1111000 = 120,
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
            8 => Some(MXPS_A::_0001000),
            16 => Some(MXPS_A::_0010000),
            24 => Some(MXPS_A::_0011000),
            32 => Some(MXPS_A::_0100000),
            40 => Some(MXPS_A::_0101000),
            48 => Some(MXPS_A::_0110000),
            56 => Some(MXPS_A::_0111000),
            64 => Some(MXPS_A::_1000000),
            72 => Some(MXPS_A::_1001000),
            80 => Some(MXPS_A::_1010000),
            88 => Some(MXPS_A::_1011000),
            96 => Some(MXPS_A::_1100000),
            104 => Some(MXPS_A::_1101000),
            112 => Some(MXPS_A::_1110000),
            120 => Some(MXPS_A::_1111000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0001000`"]
    #[inline(always)]
    pub fn is_0001000(&self) -> bool {
        *self == MXPS_A::_0001000
    }
    #[doc = "Checks if the value of the field is `_0010000`"]
    #[inline(always)]
    pub fn is_0010000(&self) -> bool {
        *self == MXPS_A::_0010000
    }
    #[doc = "Checks if the value of the field is `_0011000`"]
    #[inline(always)]
    pub fn is_0011000(&self) -> bool {
        *self == MXPS_A::_0011000
    }
    #[doc = "Checks if the value of the field is `_0100000`"]
    #[inline(always)]
    pub fn is_0100000(&self) -> bool {
        *self == MXPS_A::_0100000
    }
    #[doc = "Checks if the value of the field is `_0101000`"]
    #[inline(always)]
    pub fn is_0101000(&self) -> bool {
        *self == MXPS_A::_0101000
    }
    #[doc = "Checks if the value of the field is `_0110000`"]
    #[inline(always)]
    pub fn is_0110000(&self) -> bool {
        *self == MXPS_A::_0110000
    }
    #[doc = "Checks if the value of the field is `_0111000`"]
    #[inline(always)]
    pub fn is_0111000(&self) -> bool {
        *self == MXPS_A::_0111000
    }
    #[doc = "Checks if the value of the field is `_1000000`"]
    #[inline(always)]
    pub fn is_1000000(&self) -> bool {
        *self == MXPS_A::_1000000
    }
    #[doc = "Checks if the value of the field is `_1001000`"]
    #[inline(always)]
    pub fn is_1001000(&self) -> bool {
        *self == MXPS_A::_1001000
    }
    #[doc = "Checks if the value of the field is `_1010000`"]
    #[inline(always)]
    pub fn is_1010000(&self) -> bool {
        *self == MXPS_A::_1010000
    }
    #[doc = "Checks if the value of the field is `_1011000`"]
    #[inline(always)]
    pub fn is_1011000(&self) -> bool {
        *self == MXPS_A::_1011000
    }
    #[doc = "Checks if the value of the field is `_1100000`"]
    #[inline(always)]
    pub fn is_1100000(&self) -> bool {
        *self == MXPS_A::_1100000
    }
    #[doc = "Checks if the value of the field is `_1101000`"]
    #[inline(always)]
    pub fn is_1101000(&self) -> bool {
        *self == MXPS_A::_1101000
    }
    #[doc = "Checks if the value of the field is `_1110000`"]
    #[inline(always)]
    pub fn is_1110000(&self) -> bool {
        *self == MXPS_A::_1110000
    }
    #[doc = "Checks if the value of the field is `_1111000`"]
    #[inline(always)]
    pub fn is_1111000(&self) -> bool {
        *self == MXPS_A::_1111000
    }
}
#[doc = "Field `MXPS` writer - Maximum Packet SizeThese bits set the maximum amount of data (maximum packet size) in payloads for the DCP."]
pub type MXPS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DCPMAXP_SPEC, u8, MXPS_A, 7, O>;
impl<'a, const O: u8> MXPS_W<'a, O> {
    #[doc = "8bytes"]
    #[inline(always)]
    pub fn _0001000(self) -> &'a mut W {
        self.variant(MXPS_A::_0001000)
    }
    #[doc = "16bytes"]
    #[inline(always)]
    pub fn _0010000(self) -> &'a mut W {
        self.variant(MXPS_A::_0010000)
    }
    #[doc = "24bytes"]
    #[inline(always)]
    pub fn _0011000(self) -> &'a mut W {
        self.variant(MXPS_A::_0011000)
    }
    #[doc = "32bytes"]
    #[inline(always)]
    pub fn _0100000(self) -> &'a mut W {
        self.variant(MXPS_A::_0100000)
    }
    #[doc = "40bytes"]
    #[inline(always)]
    pub fn _0101000(self) -> &'a mut W {
        self.variant(MXPS_A::_0101000)
    }
    #[doc = "48bytes"]
    #[inline(always)]
    pub fn _0110000(self) -> &'a mut W {
        self.variant(MXPS_A::_0110000)
    }
    #[doc = "56bytes"]
    #[inline(always)]
    pub fn _0111000(self) -> &'a mut W {
        self.variant(MXPS_A::_0111000)
    }
    #[doc = "64bytes"]
    #[inline(always)]
    pub fn _1000000(self) -> &'a mut W {
        self.variant(MXPS_A::_1000000)
    }
    #[doc = "72bytes"]
    #[inline(always)]
    pub fn _1001000(self) -> &'a mut W {
        self.variant(MXPS_A::_1001000)
    }
    #[doc = "80bytes"]
    #[inline(always)]
    pub fn _1010000(self) -> &'a mut W {
        self.variant(MXPS_A::_1010000)
    }
    #[doc = "88bytes"]
    #[inline(always)]
    pub fn _1011000(self) -> &'a mut W {
        self.variant(MXPS_A::_1011000)
    }
    #[doc = "96bytes"]
    #[inline(always)]
    pub fn _1100000(self) -> &'a mut W {
        self.variant(MXPS_A::_1100000)
    }
    #[doc = "104bytes"]
    #[inline(always)]
    pub fn _1101000(self) -> &'a mut W {
        self.variant(MXPS_A::_1101000)
    }
    #[doc = "112bytes"]
    #[inline(always)]
    pub fn _1110000(self) -> &'a mut W {
        self.variant(MXPS_A::_1110000)
    }
    #[doc = "120bytes"]
    #[inline(always)]
    pub fn _1111000(self) -> &'a mut W {
        self.variant(MXPS_A::_1111000)
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
    #[doc = "Bits 0:6 - Maximum Packet SizeThese bits set the maximum amount of data (maximum packet size) in payloads for the DCP."]
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
    #[doc = "Bits 0:6 - Maximum Packet SizeThese bits set the maximum amount of data (maximum packet size) in payloads for the DCP."]
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
