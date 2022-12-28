#[doc = "Register `WDTCR` reader"]
pub struct R(crate::R<WDTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCR` writer"]
pub struct W(crate::W<WDTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCR_SPEC>;
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
impl From<crate::W<WDTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOPS` reader - Timeout Period Selection"]
pub type TOPS_R = crate::FieldReader<u8, TOPS_A>;
#[doc = "Timeout Period Selection\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TOPS_A {
    #[doc = "0: 1,024 cycles (03FFh)"]
    _00 = 0,
    #[doc = "1: 4,096 cycles (0FFFh)"]
    _01 = 1,
    #[doc = "2: 8,192 cycles (1FFFh)"]
    _10 = 2,
    #[doc = "3: 16,384 cycles (3FFFh)"]
    _11 = 3,
}
impl From<TOPS_A> for u8 {
    #[inline(always)]
    fn from(variant: TOPS_A) -> Self {
        variant as _
    }
}
impl TOPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOPS_A {
        match self.bits {
            0 => TOPS_A::_00,
            1 => TOPS_A::_01,
            2 => TOPS_A::_10,
            3 => TOPS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TOPS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TOPS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TOPS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TOPS_A::_11
    }
}
#[doc = "Field `TOPS` writer - Timeout Period Selection"]
pub type TOPS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, WDTCR_SPEC, u8, TOPS_A, 2, O>;
impl<'a, const O: u8> TOPS_W<'a, O> {
    #[doc = "1,024 cycles (03FFh)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TOPS_A::_00)
    }
    #[doc = "4,096 cycles (0FFFh)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TOPS_A::_01)
    }
    #[doc = "8,192 cycles (1FFFh)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TOPS_A::_10)
    }
    #[doc = "16,384 cycles (3FFFh)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TOPS_A::_11)
    }
}
#[doc = "Field `CKS` reader - Clock Division Ratio Selection"]
pub type CKS_R = crate::FieldReader<u8, CKS_A>;
#[doc = "Clock Division Ratio Selection\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKS_A {
    #[doc = "1: PCLK/4"]
    _0001 = 1,
    #[doc = "4: PCLK/64"]
    _0100 = 4,
    #[doc = "15: PCLK/128"]
    _1111 = 15,
    #[doc = "6: PCLK/512"]
    _0110 = 6,
    #[doc = "7: PCLK/2048"]
    _0111 = 7,
    #[doc = "8: PCLK/8192"]
    _1000 = 8,
}
impl From<CKS_A> for u8 {
    #[inline(always)]
    fn from(variant: CKS_A) -> Self {
        variant as _
    }
}
impl CKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKS_A> {
        match self.bits {
            1 => Some(CKS_A::_0001),
            4 => Some(CKS_A::_0100),
            15 => Some(CKS_A::_1111),
            6 => Some(CKS_A::_0110),
            7 => Some(CKS_A::_0111),
            8 => Some(CKS_A::_1000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == CKS_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == CKS_A::_0100
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == CKS_A::_1111
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == CKS_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == CKS_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == CKS_A::_1000
    }
}
#[doc = "Field `CKS` writer - Clock Division Ratio Selection"]
pub type CKS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, WDTCR_SPEC, u8, CKS_A, 4, O>;
impl<'a, const O: u8> CKS_W<'a, O> {
    #[doc = "PCLK/4"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(CKS_A::_0001)
    }
    #[doc = "PCLK/64"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(CKS_A::_0100)
    }
    #[doc = "PCLK/128"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(CKS_A::_1111)
    }
    #[doc = "PCLK/512"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(CKS_A::_0110)
    }
    #[doc = "PCLK/2048"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(CKS_A::_0111)
    }
    #[doc = "PCLK/8192"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(CKS_A::_1000)
    }
}
#[doc = "Field `RPES` reader - Window End Position Selection"]
pub type RPES_R = crate::FieldReader<u8, RPES_A>;
#[doc = "Window End Position Selection\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RPES_A {
    #[doc = "0: 75 percent"]
    _00 = 0,
    #[doc = "1: 50 percent"]
    _01 = 1,
    #[doc = "2: 25 percent"]
    _10 = 2,
    #[doc = "3: 0 percent (window end position is not specified)"]
    _11 = 3,
}
impl From<RPES_A> for u8 {
    #[inline(always)]
    fn from(variant: RPES_A) -> Self {
        variant as _
    }
}
impl RPES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPES_A {
        match self.bits {
            0 => RPES_A::_00,
            1 => RPES_A::_01,
            2 => RPES_A::_10,
            3 => RPES_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RPES_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RPES_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RPES_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == RPES_A::_11
    }
}
#[doc = "Field `RPES` writer - Window End Position Selection"]
pub type RPES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, WDTCR_SPEC, u8, RPES_A, 2, O>;
impl<'a, const O: u8> RPES_W<'a, O> {
    #[doc = "75 percent"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RPES_A::_00)
    }
    #[doc = "50 percent"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RPES_A::_01)
    }
    #[doc = "25 percent"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RPES_A::_10)
    }
    #[doc = "0 percent (window end position is not specified)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(RPES_A::_11)
    }
}
#[doc = "Field `RPSS` reader - Window Start Position Selection"]
pub type RPSS_R = crate::FieldReader<u8, RPSS_A>;
#[doc = "Window Start Position Selection\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RPSS_A {
    #[doc = "0: 25 percent"]
    _00 = 0,
    #[doc = "1: 50 percent"]
    _01 = 1,
    #[doc = "2: 75 percent"]
    _10 = 2,
    #[doc = "3: 100 percent (window start position is not specified)"]
    _11 = 3,
}
impl From<RPSS_A> for u8 {
    #[inline(always)]
    fn from(variant: RPSS_A) -> Self {
        variant as _
    }
}
impl RPSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPSS_A {
        match self.bits {
            0 => RPSS_A::_00,
            1 => RPSS_A::_01,
            2 => RPSS_A::_10,
            3 => RPSS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RPSS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RPSS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RPSS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == RPSS_A::_11
    }
}
#[doc = "Field `RPSS` writer - Window Start Position Selection"]
pub type RPSS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, WDTCR_SPEC, u8, RPSS_A, 2, O>;
impl<'a, const O: u8> RPSS_W<'a, O> {
    #[doc = "25 percent"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RPSS_A::_00)
    }
    #[doc = "50 percent"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RPSS_A::_01)
    }
    #[doc = "75 percent"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RPSS_A::_10)
    }
    #[doc = "100 percent (window start position is not specified)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(RPSS_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Timeout Period Selection"]
    #[inline(always)]
    pub fn tops(&self) -> TOPS_R {
        TOPS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:7 - Clock Division Ratio Selection"]
    #[inline(always)]
    pub fn cks(&self) -> CKS_R {
        CKS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Window End Position Selection"]
    #[inline(always)]
    pub fn rpes(&self) -> RPES_R {
        RPES_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Window Start Position Selection"]
    #[inline(always)]
    pub fn rpss(&self) -> RPSS_R {
        RPSS_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timeout Period Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tops(&mut self) -> TOPS_W<0> {
        TOPS_W::new(self)
    }
    #[doc = "Bits 4:7 - Clock Division Ratio Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cks(&mut self) -> CKS_W<4> {
        CKS_W::new(self)
    }
    #[doc = "Bits 8:9 - Window End Position Selection"]
    #[inline(always)]
    #[must_use]
    pub fn rpes(&mut self) -> RPES_W<8> {
        RPES_W::new(self)
    }
    #[doc = "Bits 12:13 - Window Start Position Selection"]
    #[inline(always)]
    #[must_use]
    pub fn rpss(&mut self) -> RPSS_W<12> {
        RPSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDT Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtcr](index.html) module"]
pub struct WDTCR_SPEC;
impl crate::RegisterSpec for WDTCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [wdtcr::R](R) reader structure"]
impl crate::Readable for WDTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtcr::W](W) writer structure"]
impl crate::Writable for WDTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTCR to value 0x33f3"]
impl crate::Resettable for WDTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x33f3;
}
