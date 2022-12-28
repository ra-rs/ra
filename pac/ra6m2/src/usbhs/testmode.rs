#[doc = "Register `TESTMODE` reader"]
pub struct R(crate::R<TESTMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TESTMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TESTMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TESTMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TESTMODE` writer"]
pub struct W(crate::W<TESTMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TESTMODE_SPEC>;
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
impl From<crate::W<TESTMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TESTMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UTST` reader - Test Mode"]
pub type UTST_R = crate::FieldReader<u8, UTST_A>;
#[doc = "Test Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UTST_A {
    #[doc = "0: Normal operation"]
    _0000 = 0,
    #[doc = "1: Test_J TestMode(When the Function Controller Function is Selected)"]
    _0001 = 1,
    #[doc = "2: Test_K TestMode(When the Function Controller Function is Selected)"]
    _0010 = 2,
    #[doc = "3: Test_SE0_NAK TestMode(When the Function Controller Function is Selected)"]
    _0011 = 3,
    #[doc = "4: Test_Packet TestMode(When the Function Controller Function is Selected)"]
    _0100 = 4,
    #[doc = "5: Reserved TestMode(When the Function Controller Function is Selected)"]
    _0101 = 5,
    #[doc = "6: Reserved TestMode(When the Function Controller Function is Selected)"]
    _0110 = 6,
    #[doc = "7: Reserved TestMode(When the Function Controller Function is Selected)"]
    _0111 = 7,
    #[doc = "9: Test_J TestMode(When the Host Controller Function is Selected)"]
    _1001 = 9,
    #[doc = "10: Test_K TestMode(When the Host Controller Function is Selected)"]
    _1010 = 10,
    #[doc = "11: Test_SE0_NAK TestMode(When the Host Controller Function is Selected)"]
    _1011 = 11,
    #[doc = "12: Test_Packet TestMode(When the Host Controller Function is Selected)"]
    _1100 = 12,
    #[doc = "13: Test_Force_EnableTestMode(When the Host Controller Function is Selected)"]
    _1101 = 13,
    #[doc = "14: Reserved TestMode(When the Host Controller Function is Selected)"]
    _1110 = 14,
    #[doc = "15: Reserved TestMode(When the Host Controller Function is Selected)"]
    _1111 = 15,
}
impl From<UTST_A> for u8 {
    #[inline(always)]
    fn from(variant: UTST_A) -> Self {
        variant as _
    }
}
impl UTST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UTST_A> {
        match self.bits {
            0 => Some(UTST_A::_0000),
            1 => Some(UTST_A::_0001),
            2 => Some(UTST_A::_0010),
            3 => Some(UTST_A::_0011),
            4 => Some(UTST_A::_0100),
            5 => Some(UTST_A::_0101),
            6 => Some(UTST_A::_0110),
            7 => Some(UTST_A::_0111),
            9 => Some(UTST_A::_1001),
            10 => Some(UTST_A::_1010),
            11 => Some(UTST_A::_1011),
            12 => Some(UTST_A::_1100),
            13 => Some(UTST_A::_1101),
            14 => Some(UTST_A::_1110),
            15 => Some(UTST_A::_1111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == UTST_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == UTST_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == UTST_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == UTST_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == UTST_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == UTST_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == UTST_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == UTST_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == UTST_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == UTST_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == UTST_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == UTST_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == UTST_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == UTST_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == UTST_A::_1111
    }
}
#[doc = "Field `UTST` writer - Test Mode"]
pub type UTST_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TESTMODE_SPEC, u8, UTST_A, 4, O>;
impl<'a, const O: u8> UTST_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(UTST_A::_0000)
    }
    #[doc = "Test_J TestMode(When the Function Controller Function is Selected)"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(UTST_A::_0001)
    }
    #[doc = "Test_K TestMode(When the Function Controller Function is Selected)"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(UTST_A::_0010)
    }
    #[doc = "Test_SE0_NAK TestMode(When the Function Controller Function is Selected)"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(UTST_A::_0011)
    }
    #[doc = "Test_Packet TestMode(When the Function Controller Function is Selected)"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(UTST_A::_0100)
    }
    #[doc = "Reserved TestMode(When the Function Controller Function is Selected)"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(UTST_A::_0101)
    }
    #[doc = "Reserved TestMode(When the Function Controller Function is Selected)"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(UTST_A::_0110)
    }
    #[doc = "Reserved TestMode(When the Function Controller Function is Selected)"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(UTST_A::_0111)
    }
    #[doc = "Test_J TestMode(When the Host Controller Function is Selected)"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(UTST_A::_1001)
    }
    #[doc = "Test_K TestMode(When the Host Controller Function is Selected)"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(UTST_A::_1010)
    }
    #[doc = "Test_SE0_NAK TestMode(When the Host Controller Function is Selected)"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(UTST_A::_1011)
    }
    #[doc = "Test_Packet TestMode(When the Host Controller Function is Selected)"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(UTST_A::_1100)
    }
    #[doc = "Test_Force_EnableTestMode(When the Host Controller Function is Selected)"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(UTST_A::_1101)
    }
    #[doc = "Reserved TestMode(When the Host Controller Function is Selected)"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(UTST_A::_1110)
    }
    #[doc = "Reserved TestMode(When the Host Controller Function is Selected)"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(UTST_A::_1111)
    }
}
impl R {
    #[doc = "Bits 0:3 - Test Mode"]
    #[inline(always)]
    pub fn utst(&self) -> UTST_R {
        UTST_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Test Mode"]
    #[inline(always)]
    #[must_use]
    pub fn utst(&mut self) -> UTST_W<0> {
        UTST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Test Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [testmode](index.html) module"]
pub struct TESTMODE_SPEC;
impl crate::RegisterSpec for TESTMODE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [testmode::R](R) reader structure"]
impl crate::Readable for TESTMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [testmode::W](W) writer structure"]
impl crate::Writable for TESTMODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TESTMODE to value 0"]
impl crate::Resettable for TESTMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
