#[doc = "Register `CTSUSSC` reader"]
pub struct R(crate::R<CTSUSSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUSSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUSSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUSSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUSSC` writer"]
pub struct W(crate::W<CTSUSSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUSSC_SPEC>;
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
impl From<crate::W<CTSUSSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUSSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTSUSSDIV` reader - CTSU Spectrum Diffusion Frequency Division Setting"]
pub type CTSUSSDIV_R = crate::FieldReader<u8, CTSUSSDIV_A>;
#[doc = "CTSU Spectrum Diffusion Frequency Division Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUSSDIV_A {
    #[doc = "0: 4.00 <= fb"]
    _0000 = 0,
    #[doc = "1: 2.00 <= fb < 4.00"]
    _0001 = 1,
    #[doc = "2: 1.33 <= fb < 2.00"]
    _0010 = 2,
    #[doc = "3: 1.00 <= fb < 1.33"]
    _0011 = 3,
    #[doc = "4: 0.80 <= fb < 1.00"]
    _0100 = 4,
    #[doc = "5: 0.67 <= fb < 0.80"]
    _0101 = 5,
    #[doc = "6: 0.57 <= fb < 0.67"]
    _0110 = 6,
    #[doc = "7: 0.50 <= fb < 0.57"]
    _0111 = 7,
    #[doc = "8: 0.44 <= fb < 0.50"]
    _1000 = 8,
    #[doc = "9: 0.40 <= fb < 0.44"]
    _1001 = 9,
    #[doc = "10: 0.36 <= fb < 0.40"]
    _1010 = 10,
    #[doc = "11: 0.33 <= fb < 0.36"]
    _1011 = 11,
    #[doc = "12: 0.31 <= fb < 0.33"]
    _1100 = 12,
    #[doc = "13: 0.29 <= fb < 0.31"]
    _1101 = 13,
    #[doc = "14: 0.27 <= fb < 0.29"]
    _1110 = 14,
    #[doc = "15: fb < 0.27"]
    _1111 = 15,
}
impl From<CTSUSSDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUSSDIV_A) -> Self {
        variant as _
    }
}
impl CTSUSSDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSUSSDIV_A {
        match self.bits {
            0 => CTSUSSDIV_A::_0000,
            1 => CTSUSSDIV_A::_0001,
            2 => CTSUSSDIV_A::_0010,
            3 => CTSUSSDIV_A::_0011,
            4 => CTSUSSDIV_A::_0100,
            5 => CTSUSSDIV_A::_0101,
            6 => CTSUSSDIV_A::_0110,
            7 => CTSUSSDIV_A::_0111,
            8 => CTSUSSDIV_A::_1000,
            9 => CTSUSSDIV_A::_1001,
            10 => CTSUSSDIV_A::_1010,
            11 => CTSUSSDIV_A::_1011,
            12 => CTSUSSDIV_A::_1100,
            13 => CTSUSSDIV_A::_1101,
            14 => CTSUSSDIV_A::_1110,
            15 => CTSUSSDIV_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == CTSUSSDIV_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == CTSUSSDIV_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == CTSUSSDIV_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == CTSUSSDIV_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == CTSUSSDIV_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == CTSUSSDIV_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == CTSUSSDIV_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == CTSUSSDIV_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == CTSUSSDIV_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == CTSUSSDIV_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == CTSUSSDIV_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == CTSUSSDIV_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == CTSUSSDIV_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == CTSUSSDIV_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == CTSUSSDIV_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == CTSUSSDIV_A::_1111
    }
}
#[doc = "Field `CTSUSSDIV` writer - CTSU Spectrum Diffusion Frequency Division Setting"]
pub type CTSUSSDIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CTSUSSC_SPEC, u8, CTSUSSDIV_A, 4, O>;
impl<'a, const O: u8> CTSUSSDIV_W<'a, O> {
    #[doc = "4.00 <= fb"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(CTSUSSDIV_A::_0000)
    }
    #[doc = "2.00 <= fb < 4.00"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(CTSUSSDIV_A::_0001)
    }
    #[doc = "1.33 <= fb < 2.00"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(CTSUSSDIV_A::_0010)
    }
    #[doc = "1.00 <= fb < 1.33"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(CTSUSSDIV_A::_0011)
    }
    #[doc = "0.80 <= fb < 1.00"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(CTSUSSDIV_A::_0100)
    }
    #[doc = "0.67 <= fb < 0.80"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(CTSUSSDIV_A::_0101)
    }
    #[doc = "0.57 <= fb < 0.67"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(CTSUSSDIV_A::_0110)
    }
    #[doc = "0.50 <= fb < 0.57"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(CTSUSSDIV_A::_0111)
    }
    #[doc = "0.44 <= fb < 0.50"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(CTSUSSDIV_A::_1000)
    }
    #[doc = "0.40 <= fb < 0.44"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(CTSUSSDIV_A::_1001)
    }
    #[doc = "0.36 <= fb < 0.40"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(CTSUSSDIV_A::_1010)
    }
    #[doc = "0.33 <= fb < 0.36"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(CTSUSSDIV_A::_1011)
    }
    #[doc = "0.31 <= fb < 0.33"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(CTSUSSDIV_A::_1100)
    }
    #[doc = "0.29 <= fb < 0.31"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(CTSUSSDIV_A::_1101)
    }
    #[doc = "0.27 <= fb < 0.29"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(CTSUSSDIV_A::_1110)
    }
    #[doc = "fb < 0.27"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(CTSUSSDIV_A::_1111)
    }
}
impl R {
    #[doc = "Bits 8:11 - CTSU Spectrum Diffusion Frequency Division Setting"]
    #[inline(always)]
    pub fn ctsussdiv(&self) -> CTSUSSDIV_R {
        CTSUSSDIV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - CTSU Spectrum Diffusion Frequency Division Setting"]
    #[inline(always)]
    #[must_use]
    pub fn ctsussdiv(&mut self) -> CTSUSSDIV_W<8> {
        CTSUSSDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU High-Pass Noise Reduction Spectrum Diffusion Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsussc](index.html) module"]
pub struct CTSUSSC_SPEC;
impl crate::RegisterSpec for CTSUSSC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctsussc::R](R) reader structure"]
impl crate::Readable for CTSUSSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsussc::W](W) writer structure"]
impl crate::Writable for CTSUSSC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUSSC to value 0"]
impl crate::Resettable for CTSUSSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
