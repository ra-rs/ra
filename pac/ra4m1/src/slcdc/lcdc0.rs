#[doc = "Register `LCDC0` reader"]
pub struct R(crate::R<LCDC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDC0` writer"]
pub struct W(crate::W<LCDC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDC0_SPEC>;
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
impl From<crate::W<LCDC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDC` reader - LCD clock (LCDCL)"]
pub type LCDC_R = crate::FieldReader<u8, LCDC_A>;
#[doc = "LCD clock (LCDCL)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LCDC_A {
    #[doc = "1: (Sub clock)/22 or (LOCO clock)/22"]
    _000001 = 1,
    #[doc = "2: (Sub clock)/23 or (LOCO clock)/23"]
    _000010 = 2,
    #[doc = "3: (Sub clock)/24 or (LOCO clock)/24"]
    _000011 = 3,
    #[doc = "4: (Sub clock)/25 or (LOCO clock)/25"]
    _000100 = 4,
    #[doc = "5: (Sub clock)/26 or (LOCO clock)/26"]
    _000101 = 5,
    #[doc = "6: (Sub clock)/27 or (LOCO clock)/27"]
    _000110 = 6,
    #[doc = "7: (Sub clock)/28 or (LOCO clock)/28"]
    _000111 = 7,
    #[doc = "8: (Sub clock)/29 or (LOCO clock)/29"]
    _001000 = 8,
    #[doc = "9: (Sub clock)/210 or (LOCO clock)/210"]
    _001001 = 9,
    #[doc = "17: (Main clock)/28 or (HOCO clock)/28"]
    _010001 = 17,
    #[doc = "18: (Main clock)/29 or (HOCO clock)/29"]
    _010010 = 18,
    #[doc = "19: (Main clock)/210 or (HOCO clock)/210"]
    _010011 = 19,
    #[doc = "20: (Main clock)/211 or (HOCO clock)/211"]
    _010100 = 20,
    #[doc = "21: (Main clock)/212 or (HOCO clock)/212"]
    _010101 = 21,
    #[doc = "22: (Main clock)/213 or (HOCO clock)/213"]
    _010110 = 22,
    #[doc = "23: (Main clock)/214 or (HOCO clock)/214"]
    _010111 = 23,
    #[doc = "24: (Main clock)/215 or (HOCO clock)/215"]
    _011000 = 24,
    #[doc = "25: (Main clock)/216 or (HOCO clock)/216"]
    _011001 = 25,
    #[doc = "26: (Main clock)/217 or (HOCO clock)/217"]
    _011010 = 26,
    #[doc = "27: (Main clock)/218 or (HOCO clock)/218"]
    _011011 = 27,
    #[doc = "43: (Main clock)/219 or (HOCO clock)/219"]
    _101011 = 43,
}
impl From<LCDC_A> for u8 {
    #[inline(always)]
    fn from(variant: LCDC_A) -> Self {
        variant as _
    }
}
impl LCDC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LCDC_A> {
        match self.bits {
            1 => Some(LCDC_A::_000001),
            2 => Some(LCDC_A::_000010),
            3 => Some(LCDC_A::_000011),
            4 => Some(LCDC_A::_000100),
            5 => Some(LCDC_A::_000101),
            6 => Some(LCDC_A::_000110),
            7 => Some(LCDC_A::_000111),
            8 => Some(LCDC_A::_001000),
            9 => Some(LCDC_A::_001001),
            17 => Some(LCDC_A::_010001),
            18 => Some(LCDC_A::_010010),
            19 => Some(LCDC_A::_010011),
            20 => Some(LCDC_A::_010100),
            21 => Some(LCDC_A::_010101),
            22 => Some(LCDC_A::_010110),
            23 => Some(LCDC_A::_010111),
            24 => Some(LCDC_A::_011000),
            25 => Some(LCDC_A::_011001),
            26 => Some(LCDC_A::_011010),
            27 => Some(LCDC_A::_011011),
            43 => Some(LCDC_A::_101011),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000001`"]
    #[inline(always)]
    pub fn is_000001(&self) -> bool {
        *self == LCDC_A::_000001
    }
    #[doc = "Checks if the value of the field is `_000010`"]
    #[inline(always)]
    pub fn is_000010(&self) -> bool {
        *self == LCDC_A::_000010
    }
    #[doc = "Checks if the value of the field is `_000011`"]
    #[inline(always)]
    pub fn is_000011(&self) -> bool {
        *self == LCDC_A::_000011
    }
    #[doc = "Checks if the value of the field is `_000100`"]
    #[inline(always)]
    pub fn is_000100(&self) -> bool {
        *self == LCDC_A::_000100
    }
    #[doc = "Checks if the value of the field is `_000101`"]
    #[inline(always)]
    pub fn is_000101(&self) -> bool {
        *self == LCDC_A::_000101
    }
    #[doc = "Checks if the value of the field is `_000110`"]
    #[inline(always)]
    pub fn is_000110(&self) -> bool {
        *self == LCDC_A::_000110
    }
    #[doc = "Checks if the value of the field is `_000111`"]
    #[inline(always)]
    pub fn is_000111(&self) -> bool {
        *self == LCDC_A::_000111
    }
    #[doc = "Checks if the value of the field is `_001000`"]
    #[inline(always)]
    pub fn is_001000(&self) -> bool {
        *self == LCDC_A::_001000
    }
    #[doc = "Checks if the value of the field is `_001001`"]
    #[inline(always)]
    pub fn is_001001(&self) -> bool {
        *self == LCDC_A::_001001
    }
    #[doc = "Checks if the value of the field is `_010001`"]
    #[inline(always)]
    pub fn is_010001(&self) -> bool {
        *self == LCDC_A::_010001
    }
    #[doc = "Checks if the value of the field is `_010010`"]
    #[inline(always)]
    pub fn is_010010(&self) -> bool {
        *self == LCDC_A::_010010
    }
    #[doc = "Checks if the value of the field is `_010011`"]
    #[inline(always)]
    pub fn is_010011(&self) -> bool {
        *self == LCDC_A::_010011
    }
    #[doc = "Checks if the value of the field is `_010100`"]
    #[inline(always)]
    pub fn is_010100(&self) -> bool {
        *self == LCDC_A::_010100
    }
    #[doc = "Checks if the value of the field is `_010101`"]
    #[inline(always)]
    pub fn is_010101(&self) -> bool {
        *self == LCDC_A::_010101
    }
    #[doc = "Checks if the value of the field is `_010110`"]
    #[inline(always)]
    pub fn is_010110(&self) -> bool {
        *self == LCDC_A::_010110
    }
    #[doc = "Checks if the value of the field is `_010111`"]
    #[inline(always)]
    pub fn is_010111(&self) -> bool {
        *self == LCDC_A::_010111
    }
    #[doc = "Checks if the value of the field is `_011000`"]
    #[inline(always)]
    pub fn is_011000(&self) -> bool {
        *self == LCDC_A::_011000
    }
    #[doc = "Checks if the value of the field is `_011001`"]
    #[inline(always)]
    pub fn is_011001(&self) -> bool {
        *self == LCDC_A::_011001
    }
    #[doc = "Checks if the value of the field is `_011010`"]
    #[inline(always)]
    pub fn is_011010(&self) -> bool {
        *self == LCDC_A::_011010
    }
    #[doc = "Checks if the value of the field is `_011011`"]
    #[inline(always)]
    pub fn is_011011(&self) -> bool {
        *self == LCDC_A::_011011
    }
    #[doc = "Checks if the value of the field is `_101011`"]
    #[inline(always)]
    pub fn is_101011(&self) -> bool {
        *self == LCDC_A::_101011
    }
}
#[doc = "Field `LCDC` writer - LCD clock (LCDCL)"]
pub type LCDC_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LCDC0_SPEC, u8, LCDC_A, 6, O>;
impl<'a, const O: u8> LCDC_W<'a, O> {
    #[doc = "(Sub clock)/22 or (LOCO clock)/22"]
    #[inline(always)]
    pub fn _000001(self) -> &'a mut W {
        self.variant(LCDC_A::_000001)
    }
    #[doc = "(Sub clock)/23 or (LOCO clock)/23"]
    #[inline(always)]
    pub fn _000010(self) -> &'a mut W {
        self.variant(LCDC_A::_000010)
    }
    #[doc = "(Sub clock)/24 or (LOCO clock)/24"]
    #[inline(always)]
    pub fn _000011(self) -> &'a mut W {
        self.variant(LCDC_A::_000011)
    }
    #[doc = "(Sub clock)/25 or (LOCO clock)/25"]
    #[inline(always)]
    pub fn _000100(self) -> &'a mut W {
        self.variant(LCDC_A::_000100)
    }
    #[doc = "(Sub clock)/26 or (LOCO clock)/26"]
    #[inline(always)]
    pub fn _000101(self) -> &'a mut W {
        self.variant(LCDC_A::_000101)
    }
    #[doc = "(Sub clock)/27 or (LOCO clock)/27"]
    #[inline(always)]
    pub fn _000110(self) -> &'a mut W {
        self.variant(LCDC_A::_000110)
    }
    #[doc = "(Sub clock)/28 or (LOCO clock)/28"]
    #[inline(always)]
    pub fn _000111(self) -> &'a mut W {
        self.variant(LCDC_A::_000111)
    }
    #[doc = "(Sub clock)/29 or (LOCO clock)/29"]
    #[inline(always)]
    pub fn _001000(self) -> &'a mut W {
        self.variant(LCDC_A::_001000)
    }
    #[doc = "(Sub clock)/210 or (LOCO clock)/210"]
    #[inline(always)]
    pub fn _001001(self) -> &'a mut W {
        self.variant(LCDC_A::_001001)
    }
    #[doc = "(Main clock)/28 or (HOCO clock)/28"]
    #[inline(always)]
    pub fn _010001(self) -> &'a mut W {
        self.variant(LCDC_A::_010001)
    }
    #[doc = "(Main clock)/29 or (HOCO clock)/29"]
    #[inline(always)]
    pub fn _010010(self) -> &'a mut W {
        self.variant(LCDC_A::_010010)
    }
    #[doc = "(Main clock)/210 or (HOCO clock)/210"]
    #[inline(always)]
    pub fn _010011(self) -> &'a mut W {
        self.variant(LCDC_A::_010011)
    }
    #[doc = "(Main clock)/211 or (HOCO clock)/211"]
    #[inline(always)]
    pub fn _010100(self) -> &'a mut W {
        self.variant(LCDC_A::_010100)
    }
    #[doc = "(Main clock)/212 or (HOCO clock)/212"]
    #[inline(always)]
    pub fn _010101(self) -> &'a mut W {
        self.variant(LCDC_A::_010101)
    }
    #[doc = "(Main clock)/213 or (HOCO clock)/213"]
    #[inline(always)]
    pub fn _010110(self) -> &'a mut W {
        self.variant(LCDC_A::_010110)
    }
    #[doc = "(Main clock)/214 or (HOCO clock)/214"]
    #[inline(always)]
    pub fn _010111(self) -> &'a mut W {
        self.variant(LCDC_A::_010111)
    }
    #[doc = "(Main clock)/215 or (HOCO clock)/215"]
    #[inline(always)]
    pub fn _011000(self) -> &'a mut W {
        self.variant(LCDC_A::_011000)
    }
    #[doc = "(Main clock)/216 or (HOCO clock)/216"]
    #[inline(always)]
    pub fn _011001(self) -> &'a mut W {
        self.variant(LCDC_A::_011001)
    }
    #[doc = "(Main clock)/217 or (HOCO clock)/217"]
    #[inline(always)]
    pub fn _011010(self) -> &'a mut W {
        self.variant(LCDC_A::_011010)
    }
    #[doc = "(Main clock)/218 or (HOCO clock)/218"]
    #[inline(always)]
    pub fn _011011(self) -> &'a mut W {
        self.variant(LCDC_A::_011011)
    }
    #[doc = "(Main clock)/219 or (HOCO clock)/219"]
    #[inline(always)]
    pub fn _101011(self) -> &'a mut W {
        self.variant(LCDC_A::_101011)
    }
}
impl R {
    #[doc = "Bits 0:5 - LCD clock (LCDCL)"]
    #[inline(always)]
    pub fn lcdc(&self) -> LCDC_R {
        LCDC_R::new(self.bits & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:5 - LCD clock (LCDCL)"]
    #[inline(always)]
    #[must_use]
    pub fn lcdc(&mut self) -> LCDC_W<0> {
        LCDC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Clock Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdc0](index.html) module"]
pub struct LCDC0_SPEC;
impl crate::RegisterSpec for LCDC0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lcdc0::R](R) reader structure"]
impl crate::Readable for LCDC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdc0::W](W) writer structure"]
impl crate::Writable for LCDC0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCDC0 to value 0"]
impl crate::Resettable for LCDC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
