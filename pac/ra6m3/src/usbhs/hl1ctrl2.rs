#[doc = "Register `HL1CTRL2` reader"]
pub struct R(crate::R<HL1CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HL1CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HL1CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HL1CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HL1CTRL2` writer"]
pub struct W(crate::W<HL1CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HL1CTRL2_SPEC>;
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
impl From<crate::W<HL1CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HL1CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L1ADDR` reader - LPM Token DeviceAddressThese bits specify the value to be set in the ADDR field of LPM token."]
pub type L1ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `L1ADDR` writer - LPM Token DeviceAddressThese bits specify the value to be set in the ADDR field of LPM token."]
pub type L1ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, HL1CTRL2_SPEC, u8, u8, 4, O>;
#[doc = "Field `HIRD` reader - LPM Token HIRD"]
pub type HIRD_R = crate::FieldReader<u8, HIRD_A>;
#[doc = "LPM Token HIRD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HIRD_A {
    #[doc = "0: 50 us(Setting prohibited(BESL = 0)) / 75 us(BESL = 1)"]
    _0000 = 0,
    #[doc = "1: 125 us(BESL = 0) / 100 us(BESL = 1)"]
    _0001 = 1,
    #[doc = "2: 200 us(BESL = 0) / 150 us(BESL = 1)"]
    _0010 = 2,
    #[doc = "3: 275 us(BESL = 0) / 250 us(BESL = 1)"]
    _0011 = 3,
    #[doc = "4: 350 us(BESL = 0) / 350 us(BESL = 1)"]
    _0100 = 4,
    #[doc = "5: 425 us(BESL = 0) / 450 us(BESL = 1)"]
    _0101 = 5,
    #[doc = "6: 500 us(BESL = 0) / 950 us(BESL = 1)"]
    _0110 = 6,
    #[doc = "7: 575 us(BESL = 0) / 1950 us(BESL = 1)"]
    _0111 = 7,
    #[doc = "8: 650 us(BESL = 0) / 2950 us(BESL = 1)"]
    _1000 = 8,
    #[doc = "9: 725 us(BESL = 0) / 3950 us(BESL = 1)"]
    _1001 = 9,
    #[doc = "10: 800 us(BESL = 0) / 4950 us(BESL = 1)"]
    _1010 = 10,
    #[doc = "11: 875 us(BESL = 0) / 5950 us(BESL = 1)"]
    _1011 = 11,
    #[doc = "12: 950 us(BESL = 0) / 6950 us(BESL = 1)"]
    _1100 = 12,
    #[doc = "13: 1025 us(Setting prohibited(BESL = 0)) / 7950 us(BESL = 1)"]
    _1101 = 13,
    #[doc = "14: 1100 us(Setting prohibited(BESL = 0)) / 8950 us(BESL = 1)"]
    _1110 = 14,
    #[doc = "15: 1175 us(Setting prohibited(BESL = 0)) / 9950 us(BESL = 1)"]
    _1111 = 15,
}
impl From<HIRD_A> for u8 {
    #[inline(always)]
    fn from(variant: HIRD_A) -> Self {
        variant as _
    }
}
impl HIRD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIRD_A {
        match self.bits {
            0 => HIRD_A::_0000,
            1 => HIRD_A::_0001,
            2 => HIRD_A::_0010,
            3 => HIRD_A::_0011,
            4 => HIRD_A::_0100,
            5 => HIRD_A::_0101,
            6 => HIRD_A::_0110,
            7 => HIRD_A::_0111,
            8 => HIRD_A::_1000,
            9 => HIRD_A::_1001,
            10 => HIRD_A::_1010,
            11 => HIRD_A::_1011,
            12 => HIRD_A::_1100,
            13 => HIRD_A::_1101,
            14 => HIRD_A::_1110,
            15 => HIRD_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == HIRD_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == HIRD_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == HIRD_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == HIRD_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == HIRD_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == HIRD_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == HIRD_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == HIRD_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == HIRD_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == HIRD_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == HIRD_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == HIRD_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == HIRD_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == HIRD_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == HIRD_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == HIRD_A::_1111
    }
}
#[doc = "Field `HIRD` writer - LPM Token HIRD"]
pub type HIRD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, HL1CTRL2_SPEC, u8, HIRD_A, 4, O>;
impl<'a, const O: u8> HIRD_W<'a, O> {
    #[doc = "50 us(Setting prohibited(BESL = 0)) / 75 us(BESL = 1)"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(HIRD_A::_0000)
    }
    #[doc = "125 us(BESL = 0) / 100 us(BESL = 1)"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(HIRD_A::_0001)
    }
    #[doc = "200 us(BESL = 0) / 150 us(BESL = 1)"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(HIRD_A::_0010)
    }
    #[doc = "275 us(BESL = 0) / 250 us(BESL = 1)"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(HIRD_A::_0011)
    }
    #[doc = "350 us(BESL = 0) / 350 us(BESL = 1)"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(HIRD_A::_0100)
    }
    #[doc = "425 us(BESL = 0) / 450 us(BESL = 1)"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(HIRD_A::_0101)
    }
    #[doc = "500 us(BESL = 0) / 950 us(BESL = 1)"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(HIRD_A::_0110)
    }
    #[doc = "575 us(BESL = 0) / 1950 us(BESL = 1)"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(HIRD_A::_0111)
    }
    #[doc = "650 us(BESL = 0) / 2950 us(BESL = 1)"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(HIRD_A::_1000)
    }
    #[doc = "725 us(BESL = 0) / 3950 us(BESL = 1)"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(HIRD_A::_1001)
    }
    #[doc = "800 us(BESL = 0) / 4950 us(BESL = 1)"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(HIRD_A::_1010)
    }
    #[doc = "875 us(BESL = 0) / 5950 us(BESL = 1)"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(HIRD_A::_1011)
    }
    #[doc = "950 us(BESL = 0) / 6950 us(BESL = 1)"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(HIRD_A::_1100)
    }
    #[doc = "1025 us(Setting prohibited(BESL = 0)) / 7950 us(BESL = 1)"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(HIRD_A::_1101)
    }
    #[doc = "1100 us(Setting prohibited(BESL = 0)) / 8950 us(BESL = 1)"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(HIRD_A::_1110)
    }
    #[doc = "1175 us(Setting prohibited(BESL = 0)) / 9950 us(BESL = 1)"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(HIRD_A::_1111)
    }
}
#[doc = "Field `L1RWE` reader - LPM Token L1 RemoteWake EnableThese bits specify the value to be set in the RWE field of LPM token."]
pub type L1RWE_R = crate::BitReader<bool>;
#[doc = "Field `L1RWE` writer - LPM Token L1 RemoteWake EnableThese bits specify the value to be set in the RWE field of LPM token."]
pub type L1RWE_W<'a, const O: u8> = crate::BitWriter<'a, u16, HL1CTRL2_SPEC, bool, O>;
#[doc = "Field `BESL` reader - BESL & Alternate HIRDThis bit selects the K-State drive period at the time of L1 Resume."]
pub type BESL_R = crate::BitReader<bool>;
#[doc = "Field `BESL` writer - BESL & Alternate HIRDThis bit selects the K-State drive period at the time of L1 Resume."]
pub type BESL_W<'a, const O: u8> = crate::BitWriter<'a, u16, HL1CTRL2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - LPM Token DeviceAddressThese bits specify the value to be set in the ADDR field of LPM token."]
    #[inline(always)]
    pub fn l1addr(&self) -> L1ADDR_R {
        L1ADDR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - LPM Token HIRD"]
    #[inline(always)]
    pub fn hird(&self) -> HIRD_R {
        HIRD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - LPM Token L1 RemoteWake EnableThese bits specify the value to be set in the RWE field of LPM token."]
    #[inline(always)]
    pub fn l1rwe(&self) -> L1RWE_R {
        L1RWE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - BESL & Alternate HIRDThis bit selects the K-State drive period at the time of L1 Resume."]
    #[inline(always)]
    pub fn besl(&self) -> BESL_R {
        BESL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - LPM Token DeviceAddressThese bits specify the value to be set in the ADDR field of LPM token."]
    #[inline(always)]
    #[must_use]
    pub fn l1addr(&mut self) -> L1ADDR_W<0> {
        L1ADDR_W::new(self)
    }
    #[doc = "Bits 8:11 - LPM Token HIRD"]
    #[inline(always)]
    #[must_use]
    pub fn hird(&mut self) -> HIRD_W<8> {
        HIRD_W::new(self)
    }
    #[doc = "Bit 12 - LPM Token L1 RemoteWake EnableThese bits specify the value to be set in the RWE field of LPM token."]
    #[inline(always)]
    #[must_use]
    pub fn l1rwe(&mut self) -> L1RWE_W<12> {
        L1RWE_W::new(self)
    }
    #[doc = "Bit 15 - BESL & Alternate HIRDThis bit selects the K-State drive period at the time of L1 Resume."]
    #[inline(always)]
    #[must_use]
    pub fn besl(&mut self) -> BESL_W<15> {
        BESL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host L1 Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hl1ctrl2](index.html) module"]
pub struct HL1CTRL2_SPEC;
impl crate::RegisterSpec for HL1CTRL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [hl1ctrl2::R](R) reader structure"]
impl crate::Readable for HL1CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hl1ctrl2::W](W) writer structure"]
impl crate::Writable for HL1CTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HL1CTRL2 to value 0"]
impl crate::Resettable for HL1CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
