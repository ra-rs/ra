#[doc = "Register `SFMSKC` reader"]
pub struct R(crate::R<SFMSKC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFMSKC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFMSKC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFMSKC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFMSKC` writer"]
pub struct W(crate::W<SFMSKC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFMSKC_SPEC>;
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
impl From<crate::W<SFMSKC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFMSKC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFMDV` reader - Serial interface reference cycle selection (* Pay attention to the irregularity.)NOTE: When PCLKA multiplied by an odd number is selected, the high-level width of the SCK signal is longer than the low-level width by 1 x PCLKA before duty ratio correction."]
pub type SFMDV_R = crate::FieldReader<u8, SFMDV_A>;
#[doc = "Serial interface reference cycle selection (* Pay attention to the irregularity.)NOTE: When PCLKA multiplied by an odd number is selected, the high-level width of the SCK signal is longer than the low-level width by 1 x PCLKA before duty ratio correction.\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SFMDV_A {
    #[doc = "16: 18 x PCLKA"]
    _10000 = 16,
    #[doc = "17: 20 x PCLKA"]
    _10001 = 17,
    #[doc = "18: 22 x PCLKA"]
    _10010 = 18,
    #[doc = "19: 24 x PCLKA"]
    _10011 = 19,
    #[doc = "20: 26 x PCLKA"]
    _10100 = 20,
    #[doc = "21: 28 x PCLKA"]
    _10101 = 21,
    #[doc = "22: 30 x PCLKA"]
    _10110 = 22,
    #[doc = "23: 32 x PCLKA"]
    _10111 = 23,
    #[doc = "24: 34 x PCLKA"]
    _11000 = 24,
    #[doc = "25: 36 x PCLKA"]
    _11001 = 25,
    #[doc = "26: 38 x PCLKA"]
    _11010 = 26,
    #[doc = "27: 40 x PCLKA"]
    _11011 = 27,
    #[doc = "28: 42 x PCLKA"]
    _11100 = 28,
    #[doc = "29: 44 x PCLKA"]
    _11101 = 29,
    #[doc = "30: 46 x PCLKA"]
    _11110 = 30,
    #[doc = "31: 48 x PCLKA"]
    _11111 = 31,
}
impl From<SFMDV_A> for u8 {
    #[inline(always)]
    fn from(variant: SFMDV_A) -> Self {
        variant as _
    }
}
impl SFMDV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SFMDV_A> {
        match self.bits {
            16 => Some(SFMDV_A::_10000),
            17 => Some(SFMDV_A::_10001),
            18 => Some(SFMDV_A::_10010),
            19 => Some(SFMDV_A::_10011),
            20 => Some(SFMDV_A::_10100),
            21 => Some(SFMDV_A::_10101),
            22 => Some(SFMDV_A::_10110),
            23 => Some(SFMDV_A::_10111),
            24 => Some(SFMDV_A::_11000),
            25 => Some(SFMDV_A::_11001),
            26 => Some(SFMDV_A::_11010),
            27 => Some(SFMDV_A::_11011),
            28 => Some(SFMDV_A::_11100),
            29 => Some(SFMDV_A::_11101),
            30 => Some(SFMDV_A::_11110),
            31 => Some(SFMDV_A::_11111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_10000`"]
    #[inline(always)]
    pub fn is_10000(&self) -> bool {
        *self == SFMDV_A::_10000
    }
    #[doc = "Checks if the value of the field is `_10001`"]
    #[inline(always)]
    pub fn is_10001(&self) -> bool {
        *self == SFMDV_A::_10001
    }
    #[doc = "Checks if the value of the field is `_10010`"]
    #[inline(always)]
    pub fn is_10010(&self) -> bool {
        *self == SFMDV_A::_10010
    }
    #[doc = "Checks if the value of the field is `_10011`"]
    #[inline(always)]
    pub fn is_10011(&self) -> bool {
        *self == SFMDV_A::_10011
    }
    #[doc = "Checks if the value of the field is `_10100`"]
    #[inline(always)]
    pub fn is_10100(&self) -> bool {
        *self == SFMDV_A::_10100
    }
    #[doc = "Checks if the value of the field is `_10101`"]
    #[inline(always)]
    pub fn is_10101(&self) -> bool {
        *self == SFMDV_A::_10101
    }
    #[doc = "Checks if the value of the field is `_10110`"]
    #[inline(always)]
    pub fn is_10110(&self) -> bool {
        *self == SFMDV_A::_10110
    }
    #[doc = "Checks if the value of the field is `_10111`"]
    #[inline(always)]
    pub fn is_10111(&self) -> bool {
        *self == SFMDV_A::_10111
    }
    #[doc = "Checks if the value of the field is `_11000`"]
    #[inline(always)]
    pub fn is_11000(&self) -> bool {
        *self == SFMDV_A::_11000
    }
    #[doc = "Checks if the value of the field is `_11001`"]
    #[inline(always)]
    pub fn is_11001(&self) -> bool {
        *self == SFMDV_A::_11001
    }
    #[doc = "Checks if the value of the field is `_11010`"]
    #[inline(always)]
    pub fn is_11010(&self) -> bool {
        *self == SFMDV_A::_11010
    }
    #[doc = "Checks if the value of the field is `_11011`"]
    #[inline(always)]
    pub fn is_11011(&self) -> bool {
        *self == SFMDV_A::_11011
    }
    #[doc = "Checks if the value of the field is `_11100`"]
    #[inline(always)]
    pub fn is_11100(&self) -> bool {
        *self == SFMDV_A::_11100
    }
    #[doc = "Checks if the value of the field is `_11101`"]
    #[inline(always)]
    pub fn is_11101(&self) -> bool {
        *self == SFMDV_A::_11101
    }
    #[doc = "Checks if the value of the field is `_11110`"]
    #[inline(always)]
    pub fn is_11110(&self) -> bool {
        *self == SFMDV_A::_11110
    }
    #[doc = "Checks if the value of the field is `_11111`"]
    #[inline(always)]
    pub fn is_11111(&self) -> bool {
        *self == SFMDV_A::_11111
    }
}
#[doc = "Field `SFMDV` writer - Serial interface reference cycle selection (* Pay attention to the irregularity.)NOTE: When PCLKA multiplied by an odd number is selected, the high-level width of the SCK signal is longer than the low-level width by 1 x PCLKA before duty ratio correction."]
pub type SFMDV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SFMSKC_SPEC, u8, SFMDV_A, 5, O>;
impl<'a, const O: u8> SFMDV_W<'a, O> {
    #[doc = "18 x PCLKA"]
    #[inline(always)]
    pub fn _10000(self) -> &'a mut W {
        self.variant(SFMDV_A::_10000)
    }
    #[doc = "20 x PCLKA"]
    #[inline(always)]
    pub fn _10001(self) -> &'a mut W {
        self.variant(SFMDV_A::_10001)
    }
    #[doc = "22 x PCLKA"]
    #[inline(always)]
    pub fn _10010(self) -> &'a mut W {
        self.variant(SFMDV_A::_10010)
    }
    #[doc = "24 x PCLKA"]
    #[inline(always)]
    pub fn _10011(self) -> &'a mut W {
        self.variant(SFMDV_A::_10011)
    }
    #[doc = "26 x PCLKA"]
    #[inline(always)]
    pub fn _10100(self) -> &'a mut W {
        self.variant(SFMDV_A::_10100)
    }
    #[doc = "28 x PCLKA"]
    #[inline(always)]
    pub fn _10101(self) -> &'a mut W {
        self.variant(SFMDV_A::_10101)
    }
    #[doc = "30 x PCLKA"]
    #[inline(always)]
    pub fn _10110(self) -> &'a mut W {
        self.variant(SFMDV_A::_10110)
    }
    #[doc = "32 x PCLKA"]
    #[inline(always)]
    pub fn _10111(self) -> &'a mut W {
        self.variant(SFMDV_A::_10111)
    }
    #[doc = "34 x PCLKA"]
    #[inline(always)]
    pub fn _11000(self) -> &'a mut W {
        self.variant(SFMDV_A::_11000)
    }
    #[doc = "36 x PCLKA"]
    #[inline(always)]
    pub fn _11001(self) -> &'a mut W {
        self.variant(SFMDV_A::_11001)
    }
    #[doc = "38 x PCLKA"]
    #[inline(always)]
    pub fn _11010(self) -> &'a mut W {
        self.variant(SFMDV_A::_11010)
    }
    #[doc = "40 x PCLKA"]
    #[inline(always)]
    pub fn _11011(self) -> &'a mut W {
        self.variant(SFMDV_A::_11011)
    }
    #[doc = "42 x PCLKA"]
    #[inline(always)]
    pub fn _11100(self) -> &'a mut W {
        self.variant(SFMDV_A::_11100)
    }
    #[doc = "44 x PCLKA"]
    #[inline(always)]
    pub fn _11101(self) -> &'a mut W {
        self.variant(SFMDV_A::_11101)
    }
    #[doc = "46 x PCLKA"]
    #[inline(always)]
    pub fn _11110(self) -> &'a mut W {
        self.variant(SFMDV_A::_11110)
    }
    #[doc = "48 x PCLKA"]
    #[inline(always)]
    pub fn _11111(self) -> &'a mut W {
        self.variant(SFMDV_A::_11111)
    }
}
#[doc = "Field `SFMDTY` reader - Selection of a duty ratio correction function for the SCK signal"]
pub type SFMDTY_R = crate::BitReader<SFMDTY_A>;
#[doc = "Selection of a duty ratio correction function for the SCK signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMDTY_A {
    #[doc = "0: Serial interface reference cycle selection (* Pay attention to the irregularity.)"]
    _0 = 0,
    #[doc = "1: Delays the rising of the SCK signal by 0.5*PCLKA.(* Valid with PCLKA multiplied by an odd number)"]
    _1 = 1,
}
impl From<SFMDTY_A> for bool {
    #[inline(always)]
    fn from(variant: SFMDTY_A) -> Self {
        variant as u8 != 0
    }
}
impl SFMDTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFMDTY_A {
        match self.bits {
            false => SFMDTY_A::_0,
            true => SFMDTY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMDTY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMDTY_A::_1
    }
}
#[doc = "Field `SFMDTY` writer - Selection of a duty ratio correction function for the SCK signal"]
pub type SFMDTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFMSKC_SPEC, SFMDTY_A, O>;
impl<'a, const O: u8> SFMDTY_W<'a, O> {
    #[doc = "Serial interface reference cycle selection (* Pay attention to the irregularity.)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SFMDTY_A::_0)
    }
    #[doc = "Delays the rising of the SCK signal by 0.5*PCLKA.(* Valid with PCLKA multiplied by an odd number)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SFMDTY_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:4 - Serial interface reference cycle selection (* Pay attention to the irregularity.)NOTE: When PCLKA multiplied by an odd number is selected, the high-level width of the SCK signal is longer than the low-level width by 1 x PCLKA before duty ratio correction."]
    #[inline(always)]
    pub fn sfmdv(&self) -> SFMDV_R {
        SFMDV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Selection of a duty ratio correction function for the SCK signal"]
    #[inline(always)]
    pub fn sfmdty(&self) -> SFMDTY_R {
        SFMDTY_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Serial interface reference cycle selection (* Pay attention to the irregularity.)NOTE: When PCLKA multiplied by an odd number is selected, the high-level width of the SCK signal is longer than the low-level width by 1 x PCLKA before duty ratio correction."]
    #[inline(always)]
    #[must_use]
    pub fn sfmdv(&mut self) -> SFMDV_W<0> {
        SFMDV_W::new(self)
    }
    #[doc = "Bit 5 - Selection of a duty ratio correction function for the SCK signal"]
    #[inline(always)]
    #[must_use]
    pub fn sfmdty(&mut self) -> SFMDTY_W<5> {
        SFMDTY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfmskc](index.html) module"]
pub struct SFMSKC_SPEC;
impl crate::RegisterSpec for SFMSKC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfmskc::R](R) reader structure"]
impl crate::Readable for SFMSKC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfmskc::W](W) writer structure"]
impl crate::Writable for SFMSKC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFMSKC to value 0x08"]
impl crate::Resettable for SFMSKC_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
