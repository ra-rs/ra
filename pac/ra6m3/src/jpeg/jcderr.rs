#[doc = "Register `JCDERR` reader"]
pub struct R(crate::R<JCDERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JCDERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JCDERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JCDERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JCDERR` writer"]
pub struct W(crate::W<JCDERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JCDERR_SPEC>;
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
impl From<crate::W<JCDERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JCDERR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERR` reader - Error Code (See tables )Identify the type of the error which has occurred in the compressed data analysis for decompression."]
pub type ERR_R = crate::FieldReader<u8, ERR_A>;
#[doc = "Error Code (See tables )Identify the type of the error which has occurred in the compressed data analysis for decompression.\n\nValue on reset: 10"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ERR_A {
    #[doc = "0: Normal(Decompression error codes)/Normal(Segment error codes)"]
    _0000 = 0,
    #[doc = "1: SOI not detected(Decompression error codes)"]
    _0001 = 1,
    #[doc = "2: SOF1 to SOFF detected(Decompression error codes)"]
    _0010 = 2,
    #[doc = "3: Unprovided pixel format detected(Decompression error codes)"]
    _0011 = 3,
    #[doc = "4: SOF accuracy error(Decompression error codes)"]
    _0100 = 4,
    #[doc = "5: DQT accuracy error(Decompression error codes)"]
    _0101 = 5,
    #[doc = "6: Component error 1(Decompression error codes)"]
    _0110 = 6,
    #[doc = "7: Component error 2(Decompression error codes)"]
    _0111 = 7,
    #[doc = "8: SOF0, DQT, and DHT not detected when SOS detected(Decompression error codes)"]
    _1000 = 8,
    #[doc = "9: SOS not detected(Decompression error codes)"]
    _1001 = 9,
    #[doc = "10: EOI not detected (default)(Decompression error codes)"]
    _1010 = 10,
    #[doc = "11: Restart interval data number error detected(Decompression error codes)/Restart interval data number error(Segment error codes)"]
    _1011 = 11,
    #[doc = "12: Image size error detected(Decompression error codes)/Image size error(Segment error codes)"]
    _1100 = 12,
    #[doc = "13: Last MCU data number error detected(Decompression error codes)/Last MCU data number error(Segment error codes)"]
    _1101 = 13,
    #[doc = "14: Block data number error detected(Decompression error codes)/Block data number error(Segment error codes)"]
    _1110 = 14,
}
impl From<ERR_A> for u8 {
    #[inline(always)]
    fn from(variant: ERR_A) -> Self {
        variant as _
    }
}
impl ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR_A {
        match self.bits {
            0 => ERR_A::_0000,
            1 => ERR_A::_0001,
            2 => ERR_A::_0010,
            3 => ERR_A::_0011,
            4 => ERR_A::_0100,
            5 => ERR_A::_0101,
            6 => ERR_A::_0110,
            7 => ERR_A::_0111,
            8 => ERR_A::_1000,
            9 => ERR_A::_1001,
            10 => ERR_A::_1010,
            11 => ERR_A::_1011,
            12 => ERR_A::_1100,
            13 => ERR_A::_1101,
            14 => ERR_A::_1110,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == ERR_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == ERR_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == ERR_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == ERR_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == ERR_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == ERR_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == ERR_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == ERR_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == ERR_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == ERR_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == ERR_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == ERR_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == ERR_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == ERR_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == ERR_A::_1110
    }
}
#[doc = "Field `ERR` writer - Error Code (See tables )Identify the type of the error which has occurred in the compressed data analysis for decompression."]
pub type ERR_W<'a, const O: u8> = crate::FieldWriter<'a, u8, JCDERR_SPEC, u8, ERR_A, 4, O>;
impl<'a, const O: u8> ERR_W<'a, O> {
    #[doc = "Normal(Decompression error codes)/Normal(Segment error codes)"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(ERR_A::_0000)
    }
    #[doc = "SOI not detected(Decompression error codes)"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(ERR_A::_0001)
    }
    #[doc = "SOF1 to SOFF detected(Decompression error codes)"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(ERR_A::_0010)
    }
    #[doc = "Unprovided pixel format detected(Decompression error codes)"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(ERR_A::_0011)
    }
    #[doc = "SOF accuracy error(Decompression error codes)"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(ERR_A::_0100)
    }
    #[doc = "DQT accuracy error(Decompression error codes)"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(ERR_A::_0101)
    }
    #[doc = "Component error 1(Decompression error codes)"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(ERR_A::_0110)
    }
    #[doc = "Component error 2(Decompression error codes)"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(ERR_A::_0111)
    }
    #[doc = "SOF0, DQT, and DHT not detected when SOS detected(Decompression error codes)"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(ERR_A::_1000)
    }
    #[doc = "SOS not detected(Decompression error codes)"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(ERR_A::_1001)
    }
    #[doc = "EOI not detected (default)(Decompression error codes)"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(ERR_A::_1010)
    }
    #[doc = "Restart interval data number error detected(Decompression error codes)/Restart interval data number error(Segment error codes)"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(ERR_A::_1011)
    }
    #[doc = "Image size error detected(Decompression error codes)/Image size error(Segment error codes)"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(ERR_A::_1100)
    }
    #[doc = "Last MCU data number error detected(Decompression error codes)/Last MCU data number error(Segment error codes)"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(ERR_A::_1101)
    }
    #[doc = "Block data number error detected(Decompression error codes)/Block data number error(Segment error codes)"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(ERR_A::_1110)
    }
}
impl R {
    #[doc = "Bits 0:3 - Error Code (See tables )Identify the type of the error which has occurred in the compressed data analysis for decompression."]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Error Code (See tables )Identify the type of the error which has occurred in the compressed data analysis for decompression."]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<0> {
        ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG Code Decode Error Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jcderr](index.html) module"]
pub struct JCDERR_SPEC;
impl crate::RegisterSpec for JCDERR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [jcderr::R](R) reader structure"]
impl crate::Readable for JCDERR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jcderr::W](W) writer structure"]
impl crate::Writable for JCDERR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JCDERR to value 0x0a"]
impl crate::Resettable for JCDERR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a;
}
