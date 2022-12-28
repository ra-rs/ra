#[doc = "Register `ADCMPBNSR` reader"]
pub struct R(crate::R<ADCMPBNSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPBNSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPBNSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPBNSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPBNSR` writer"]
pub struct W(crate::W<ADCMPBNSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPBNSR_SPEC>;
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
impl From<crate::W<ADCMPBNSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPBNSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPCHB` reader - Compare window B channel selection bit.The channel that compares it on the condition of compare window B is selected."]
pub type CMPCHB_R = crate::FieldReader<u8, CMPCHB_A>;
#[doc = "Compare window B channel selection bit.The channel that compares it on the condition of compare window B is selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPCHB_A {
    #[doc = "0: AN000"]
    _0X00 = 0,
    #[doc = "1: AN001"]
    _0X01 = 1,
    #[doc = "2: AN002"]
    _0X02 = 2,
    #[doc = "3: AN003"]
    _0X03 = 3,
    #[doc = "4: AN004"]
    _0X04 = 4,
    #[doc = "5: AN005"]
    _0X05 = 5,
    #[doc = "6: AN006"]
    _0X06 = 6,
    #[doc = "7: AN007"]
    _0X07 = 7,
    #[doc = "8: AN008"]
    _0X08 = 8,
    #[doc = "9: AN009"]
    _0X09 = 9,
    #[doc = "10: AN010"]
    _0X0A = 10,
    #[doc = "11: AN011"]
    _0X0B = 11,
    #[doc = "12: AN012"]
    _0X0C = 12,
    #[doc = "13: AN013"]
    _0X0D = 13,
    #[doc = "14: AN014"]
    _0X0E = 14,
    #[doc = "15: AN015"]
    _0X0F = 15,
    #[doc = "16: AN016"]
    _0X10 = 16,
    #[doc = "17: AN017"]
    _0X11 = 17,
    #[doc = "18: AN018"]
    _0X12 = 18,
    #[doc = "19: AN019"]
    _0X13 = 19,
    #[doc = "20: AN020"]
    _0X14 = 20,
    #[doc = "21: AN021"]
    _0X15 = 21,
    #[doc = "22: AN022"]
    _0X16 = 22,
    #[doc = "23: AN023"]
    _0X17 = 23,
    #[doc = "24: AN024"]
    _0X18 = 24,
    #[doc = "25: AN025"]
    _0X19 = 25,
    #[doc = "26: AN026"]
    _0X1A = 26,
    #[doc = "27: AN027"]
    _0X1B = 27,
    #[doc = "32: Temperature sensor"]
    _0X20 = 32,
    #[doc = "33: Internal reference voltage"]
    _0X21 = 33,
    #[doc = "63: No channel is selected"]
    _0X3F = 63,
}
impl From<CMPCHB_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPCHB_A) -> Self {
        variant as _
    }
}
impl CMPCHB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMPCHB_A> {
        match self.bits {
            0 => Some(CMPCHB_A::_0X00),
            1 => Some(CMPCHB_A::_0X01),
            2 => Some(CMPCHB_A::_0X02),
            3 => Some(CMPCHB_A::_0X03),
            4 => Some(CMPCHB_A::_0X04),
            5 => Some(CMPCHB_A::_0X05),
            6 => Some(CMPCHB_A::_0X06),
            7 => Some(CMPCHB_A::_0X07),
            8 => Some(CMPCHB_A::_0X08),
            9 => Some(CMPCHB_A::_0X09),
            10 => Some(CMPCHB_A::_0X0A),
            11 => Some(CMPCHB_A::_0X0B),
            12 => Some(CMPCHB_A::_0X0C),
            13 => Some(CMPCHB_A::_0X0D),
            14 => Some(CMPCHB_A::_0X0E),
            15 => Some(CMPCHB_A::_0X0F),
            16 => Some(CMPCHB_A::_0X10),
            17 => Some(CMPCHB_A::_0X11),
            18 => Some(CMPCHB_A::_0X12),
            19 => Some(CMPCHB_A::_0X13),
            20 => Some(CMPCHB_A::_0X14),
            21 => Some(CMPCHB_A::_0X15),
            22 => Some(CMPCHB_A::_0X16),
            23 => Some(CMPCHB_A::_0X17),
            24 => Some(CMPCHB_A::_0X18),
            25 => Some(CMPCHB_A::_0X19),
            26 => Some(CMPCHB_A::_0X1A),
            27 => Some(CMPCHB_A::_0X1B),
            32 => Some(CMPCHB_A::_0X20),
            33 => Some(CMPCHB_A::_0X21),
            63 => Some(CMPCHB_A::_0X3F),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == CMPCHB_A::_0X00
    }
    #[doc = "Checks if the value of the field is `_0X01`"]
    #[inline(always)]
    pub fn is_0x01(&self) -> bool {
        *self == CMPCHB_A::_0X01
    }
    #[doc = "Checks if the value of the field is `_0X02`"]
    #[inline(always)]
    pub fn is_0x02(&self) -> bool {
        *self == CMPCHB_A::_0X02
    }
    #[doc = "Checks if the value of the field is `_0X03`"]
    #[inline(always)]
    pub fn is_0x03(&self) -> bool {
        *self == CMPCHB_A::_0X03
    }
    #[doc = "Checks if the value of the field is `_0X04`"]
    #[inline(always)]
    pub fn is_0x04(&self) -> bool {
        *self == CMPCHB_A::_0X04
    }
    #[doc = "Checks if the value of the field is `_0X05`"]
    #[inline(always)]
    pub fn is_0x05(&self) -> bool {
        *self == CMPCHB_A::_0X05
    }
    #[doc = "Checks if the value of the field is `_0X06`"]
    #[inline(always)]
    pub fn is_0x06(&self) -> bool {
        *self == CMPCHB_A::_0X06
    }
    #[doc = "Checks if the value of the field is `_0X07`"]
    #[inline(always)]
    pub fn is_0x07(&self) -> bool {
        *self == CMPCHB_A::_0X07
    }
    #[doc = "Checks if the value of the field is `_0X08`"]
    #[inline(always)]
    pub fn is_0x08(&self) -> bool {
        *self == CMPCHB_A::_0X08
    }
    #[doc = "Checks if the value of the field is `_0X09`"]
    #[inline(always)]
    pub fn is_0x09(&self) -> bool {
        *self == CMPCHB_A::_0X09
    }
    #[doc = "Checks if the value of the field is `_0X0A`"]
    #[inline(always)]
    pub fn is_0x0a(&self) -> bool {
        *self == CMPCHB_A::_0X0A
    }
    #[doc = "Checks if the value of the field is `_0X0B`"]
    #[inline(always)]
    pub fn is_0x0b(&self) -> bool {
        *self == CMPCHB_A::_0X0B
    }
    #[doc = "Checks if the value of the field is `_0X0C`"]
    #[inline(always)]
    pub fn is_0x0c(&self) -> bool {
        *self == CMPCHB_A::_0X0C
    }
    #[doc = "Checks if the value of the field is `_0X0D`"]
    #[inline(always)]
    pub fn is_0x0d(&self) -> bool {
        *self == CMPCHB_A::_0X0D
    }
    #[doc = "Checks if the value of the field is `_0X0E`"]
    #[inline(always)]
    pub fn is_0x0e(&self) -> bool {
        *self == CMPCHB_A::_0X0E
    }
    #[doc = "Checks if the value of the field is `_0X0F`"]
    #[inline(always)]
    pub fn is_0x0f(&self) -> bool {
        *self == CMPCHB_A::_0X0F
    }
    #[doc = "Checks if the value of the field is `_0X10`"]
    #[inline(always)]
    pub fn is_0x10(&self) -> bool {
        *self == CMPCHB_A::_0X10
    }
    #[doc = "Checks if the value of the field is `_0X11`"]
    #[inline(always)]
    pub fn is_0x11(&self) -> bool {
        *self == CMPCHB_A::_0X11
    }
    #[doc = "Checks if the value of the field is `_0X12`"]
    #[inline(always)]
    pub fn is_0x12(&self) -> bool {
        *self == CMPCHB_A::_0X12
    }
    #[doc = "Checks if the value of the field is `_0X13`"]
    #[inline(always)]
    pub fn is_0x13(&self) -> bool {
        *self == CMPCHB_A::_0X13
    }
    #[doc = "Checks if the value of the field is `_0X14`"]
    #[inline(always)]
    pub fn is_0x14(&self) -> bool {
        *self == CMPCHB_A::_0X14
    }
    #[doc = "Checks if the value of the field is `_0X15`"]
    #[inline(always)]
    pub fn is_0x15(&self) -> bool {
        *self == CMPCHB_A::_0X15
    }
    #[doc = "Checks if the value of the field is `_0X16`"]
    #[inline(always)]
    pub fn is_0x16(&self) -> bool {
        *self == CMPCHB_A::_0X16
    }
    #[doc = "Checks if the value of the field is `_0X17`"]
    #[inline(always)]
    pub fn is_0x17(&self) -> bool {
        *self == CMPCHB_A::_0X17
    }
    #[doc = "Checks if the value of the field is `_0X18`"]
    #[inline(always)]
    pub fn is_0x18(&self) -> bool {
        *self == CMPCHB_A::_0X18
    }
    #[doc = "Checks if the value of the field is `_0X19`"]
    #[inline(always)]
    pub fn is_0x19(&self) -> bool {
        *self == CMPCHB_A::_0X19
    }
    #[doc = "Checks if the value of the field is `_0X1A`"]
    #[inline(always)]
    pub fn is_0x1a(&self) -> bool {
        *self == CMPCHB_A::_0X1A
    }
    #[doc = "Checks if the value of the field is `_0X1B`"]
    #[inline(always)]
    pub fn is_0x1b(&self) -> bool {
        *self == CMPCHB_A::_0X1B
    }
    #[doc = "Checks if the value of the field is `_0X20`"]
    #[inline(always)]
    pub fn is_0x20(&self) -> bool {
        *self == CMPCHB_A::_0X20
    }
    #[doc = "Checks if the value of the field is `_0X21`"]
    #[inline(always)]
    pub fn is_0x21(&self) -> bool {
        *self == CMPCHB_A::_0X21
    }
    #[doc = "Checks if the value of the field is `_0X3F`"]
    #[inline(always)]
    pub fn is_0x3f(&self) -> bool {
        *self == CMPCHB_A::_0X3F
    }
}
#[doc = "Field `CMPCHB` writer - Compare window B channel selection bit.The channel that compares it on the condition of compare window B is selected."]
pub type CMPCHB_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ADCMPBNSR_SPEC, u8, CMPCHB_A, 6, O>;
impl<'a, const O: u8> CMPCHB_W<'a, O> {
    #[doc = "AN000"]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X00)
    }
    #[doc = "AN001"]
    #[inline(always)]
    pub fn _0x01(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X01)
    }
    #[doc = "AN002"]
    #[inline(always)]
    pub fn _0x02(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X02)
    }
    #[doc = "AN003"]
    #[inline(always)]
    pub fn _0x03(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X03)
    }
    #[doc = "AN004"]
    #[inline(always)]
    pub fn _0x04(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X04)
    }
    #[doc = "AN005"]
    #[inline(always)]
    pub fn _0x05(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X05)
    }
    #[doc = "AN006"]
    #[inline(always)]
    pub fn _0x06(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X06)
    }
    #[doc = "AN007"]
    #[inline(always)]
    pub fn _0x07(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X07)
    }
    #[doc = "AN008"]
    #[inline(always)]
    pub fn _0x08(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X08)
    }
    #[doc = "AN009"]
    #[inline(always)]
    pub fn _0x09(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X09)
    }
    #[doc = "AN010"]
    #[inline(always)]
    pub fn _0x0a(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X0A)
    }
    #[doc = "AN011"]
    #[inline(always)]
    pub fn _0x0b(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X0B)
    }
    #[doc = "AN012"]
    #[inline(always)]
    pub fn _0x0c(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X0C)
    }
    #[doc = "AN013"]
    #[inline(always)]
    pub fn _0x0d(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X0D)
    }
    #[doc = "AN014"]
    #[inline(always)]
    pub fn _0x0e(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X0E)
    }
    #[doc = "AN015"]
    #[inline(always)]
    pub fn _0x0f(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X0F)
    }
    #[doc = "AN016"]
    #[inline(always)]
    pub fn _0x10(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X10)
    }
    #[doc = "AN017"]
    #[inline(always)]
    pub fn _0x11(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X11)
    }
    #[doc = "AN018"]
    #[inline(always)]
    pub fn _0x12(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X12)
    }
    #[doc = "AN019"]
    #[inline(always)]
    pub fn _0x13(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X13)
    }
    #[doc = "AN020"]
    #[inline(always)]
    pub fn _0x14(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X14)
    }
    #[doc = "AN021"]
    #[inline(always)]
    pub fn _0x15(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X15)
    }
    #[doc = "AN022"]
    #[inline(always)]
    pub fn _0x16(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X16)
    }
    #[doc = "AN023"]
    #[inline(always)]
    pub fn _0x17(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X17)
    }
    #[doc = "AN024"]
    #[inline(always)]
    pub fn _0x18(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X18)
    }
    #[doc = "AN025"]
    #[inline(always)]
    pub fn _0x19(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X19)
    }
    #[doc = "AN026"]
    #[inline(always)]
    pub fn _0x1a(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X1A)
    }
    #[doc = "AN027"]
    #[inline(always)]
    pub fn _0x1b(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X1B)
    }
    #[doc = "Temperature sensor"]
    #[inline(always)]
    pub fn _0x20(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X20)
    }
    #[doc = "Internal reference voltage"]
    #[inline(always)]
    pub fn _0x21(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X21)
    }
    #[doc = "No channel is selected"]
    #[inline(always)]
    pub fn _0x3f(self) -> &'a mut W {
        self.variant(CMPCHB_A::_0X3F)
    }
}
#[doc = "Field `CMPLB` reader - Compare window B Compare condition setting bit."]
pub type CMPLB_R = crate::BitReader<CMPLB_A>;
#[doc = "Compare window B Compare condition setting bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLB_A {
    #[doc = "0: CMPLLB value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < CMPLLB value or CMPULB value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: CMPLLB value < A/D converted value(ADCMPCR.WCMPE=0) / CMPLLB value < A/D converted value < CMPULB value (ADCMPCR.WCMPE=1)"]
    _1 = 1,
}
impl From<CMPLB_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLB_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLB_A {
        match self.bits {
            false => CMPLB_A::_0,
            true => CMPLB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLB_A::_1
    }
}
#[doc = "Field `CMPLB` writer - Compare window B Compare condition setting bit."]
pub type CMPLB_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADCMPBNSR_SPEC, CMPLB_A, O>;
impl<'a, const O: u8> CMPLB_W<'a, O> {
    #[doc = "CMPLLB value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < CMPLLB value or CMPULB value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLB_A::_0)
    }
    #[doc = "CMPLLB value < A/D converted value(ADCMPCR.WCMPE=0) / CMPLLB value < A/D converted value < CMPULB value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLB_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:5 - Compare window B channel selection bit.The channel that compares it on the condition of compare window B is selected."]
    #[inline(always)]
    pub fn cmpchb(&self) -> CMPCHB_R {
        CMPCHB_R::new(self.bits & 0x3f)
    }
    #[doc = "Bit 7 - Compare window B Compare condition setting bit."]
    #[inline(always)]
    pub fn cmplb(&self) -> CMPLB_R {
        CMPLB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Compare window B channel selection bit.The channel that compares it on the condition of compare window B is selected."]
    #[inline(always)]
    #[must_use]
    pub fn cmpchb(&mut self) -> CMPCHB_W<0> {
        CMPCHB_W::new(self)
    }
    #[doc = "Bit 7 - Compare window B Compare condition setting bit."]
    #[inline(always)]
    #[must_use]
    pub fn cmplb(&mut self) -> CMPLB_W<7> {
        CMPLB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Compare Function Window B Channel Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpbnsr](index.html) module"]
pub struct ADCMPBNSR_SPEC;
impl crate::RegisterSpec for ADCMPBNSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adcmpbnsr::R](R) reader structure"]
impl crate::Readable for ADCMPBNSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmpbnsr::W](W) writer structure"]
impl crate::Writable for ADCMPBNSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPBNSR to value 0"]
impl crate::Resettable for ADCMPBNSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
