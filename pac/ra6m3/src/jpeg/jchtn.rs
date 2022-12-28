#[doc = "Register `JCHTN` reader"]
pub struct R(crate::R<JCHTN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JCHTN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JCHTN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JCHTN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JCHTN` writer"]
pub struct W(crate::W<JCHTN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JCHTN_SPEC>;
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
impl From<crate::W<JCHTN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JCHTN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HTD1` reader - Huffman table number (DC) for the first color component NOTE: Read-only in Decompression."]
pub type HTD1_R = crate::BitReader<HTD1_A>;
#[doc = "Huffman table number (DC) for the first color component NOTE: Read-only in Decompression.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTD1_A {
    #[doc = "0: DC Huffman table 0(HTA1=0)/Setting prohibited(HTA1=1)"]
    _0 = 0,
    #[doc = "1: DC Huffman table 1(HTA1=1)/Setting prohibited(HTA1=0)"]
    _1 = 1,
}
impl From<HTD1_A> for bool {
    #[inline(always)]
    fn from(variant: HTD1_A) -> Self {
        variant as u8 != 0
    }
}
impl HTD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTD1_A {
        match self.bits {
            false => HTD1_A::_0,
            true => HTD1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HTD1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HTD1_A::_1
    }
}
#[doc = "Field `HTD1` writer - Huffman table number (DC) for the first color component NOTE: Read-only in Decompression."]
pub type HTD1_W<'a, const O: u8> = crate::BitWriter<'a, u8, JCHTN_SPEC, HTD1_A, O>;
impl<'a, const O: u8> HTD1_W<'a, O> {
    #[doc = "DC Huffman table 0(HTA1=0)/Setting prohibited(HTA1=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HTD1_A::_0)
    }
    #[doc = "DC Huffman table 1(HTA1=1)/Setting prohibited(HTA1=0)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HTD1_A::_1)
    }
}
#[doc = "Field `HTA1` reader - Huffman table number (AC) for the first color componentNOTE: Read-only in Decompression."]
pub type HTA1_R = crate::BitReader<HTA1_A>;
#[doc = "Huffman table number (AC) for the first color componentNOTE: Read-only in Decompression.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTA1_A {
    #[doc = "0: AC Huffman table 0(HTD1=0)/Setting prohibited(HTD1=1)"]
    _0 = 0,
    #[doc = "1: AC Huffman table 1(HTD1=1)/Setting prohibited(HTD1=0)"]
    _1 = 1,
}
impl From<HTA1_A> for bool {
    #[inline(always)]
    fn from(variant: HTA1_A) -> Self {
        variant as u8 != 0
    }
}
impl HTA1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTA1_A {
        match self.bits {
            false => HTA1_A::_0,
            true => HTA1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HTA1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HTA1_A::_1
    }
}
#[doc = "Field `HTA1` writer - Huffman table number (AC) for the first color componentNOTE: Read-only in Decompression."]
pub type HTA1_W<'a, const O: u8> = crate::BitWriter<'a, u8, JCHTN_SPEC, HTA1_A, O>;
impl<'a, const O: u8> HTA1_W<'a, O> {
    #[doc = "AC Huffman table 0(HTD1=0)/Setting prohibited(HTD1=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HTA1_A::_0)
    }
    #[doc = "AC Huffman table 1(HTD1=1)/Setting prohibited(HTD1=0)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HTA1_A::_1)
    }
}
#[doc = "Field `HTD2` reader - Huffman table number (DC) for the second color component NOTE: Read-only in Decompression."]
pub type HTD2_R = crate::BitReader<HTD2_A>;
#[doc = "Huffman table number (DC) for the second color component NOTE: Read-only in Decompression.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTD2_A {
    #[doc = "0: DC Huffman table 0(HTA2=0)/Setting prohibited(HTA2=1)"]
    _0 = 0,
    #[doc = "1: DC Huffman table 1(HTA2=1)/Setting prohibited(HTA2=0)"]
    _1 = 1,
}
impl From<HTD2_A> for bool {
    #[inline(always)]
    fn from(variant: HTD2_A) -> Self {
        variant as u8 != 0
    }
}
impl HTD2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTD2_A {
        match self.bits {
            false => HTD2_A::_0,
            true => HTD2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HTD2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HTD2_A::_1
    }
}
#[doc = "Field `HTD2` writer - Huffman table number (DC) for the second color component NOTE: Read-only in Decompression."]
pub type HTD2_W<'a, const O: u8> = crate::BitWriter<'a, u8, JCHTN_SPEC, HTD2_A, O>;
impl<'a, const O: u8> HTD2_W<'a, O> {
    #[doc = "DC Huffman table 0(HTA2=0)/Setting prohibited(HTA2=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HTD2_A::_0)
    }
    #[doc = "DC Huffman table 1(HTA2=1)/Setting prohibited(HTA2=0)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HTD2_A::_1)
    }
}
#[doc = "Field `HTA2` reader - Huffman table number (AC) for the second color componentNOTE: Read-only in Decompression."]
pub type HTA2_R = crate::BitReader<HTA2_A>;
#[doc = "Huffman table number (AC) for the second color componentNOTE: Read-only in Decompression.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTA2_A {
    #[doc = "0: AC Huffman table 0(HTD2=0)/Setting prohibited(HTD2=1)"]
    _0 = 0,
    #[doc = "1: AC Huffman table 1(HTD2=1)/Setting prohibited(HTD2=0)"]
    _1 = 1,
}
impl From<HTA2_A> for bool {
    #[inline(always)]
    fn from(variant: HTA2_A) -> Self {
        variant as u8 != 0
    }
}
impl HTA2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTA2_A {
        match self.bits {
            false => HTA2_A::_0,
            true => HTA2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HTA2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HTA2_A::_1
    }
}
#[doc = "Field `HTA2` writer - Huffman table number (AC) for the second color componentNOTE: Read-only in Decompression."]
pub type HTA2_W<'a, const O: u8> = crate::BitWriter<'a, u8, JCHTN_SPEC, HTA2_A, O>;
impl<'a, const O: u8> HTA2_W<'a, O> {
    #[doc = "AC Huffman table 0(HTD2=0)/Setting prohibited(HTD2=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HTA2_A::_0)
    }
    #[doc = "AC Huffman table 1(HTD2=1)/Setting prohibited(HTD2=0)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HTA2_A::_1)
    }
}
#[doc = "Field `HTD3` reader - Huffman table number (DC) for the third color component NOTE: Read-only in Decompression."]
pub type HTD3_R = crate::BitReader<HTD3_A>;
#[doc = "Huffman table number (DC) for the third color component NOTE: Read-only in Decompression.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTD3_A {
    #[doc = "0: DC Huffman table 0(HTA3=0)/Setting prohibited(HTA3=1)"]
    _0 = 0,
    #[doc = "1: DC Huffman table 1(HTA3=1)/Setting prohibited(HTA3=0)"]
    _1 = 1,
}
impl From<HTD3_A> for bool {
    #[inline(always)]
    fn from(variant: HTD3_A) -> Self {
        variant as u8 != 0
    }
}
impl HTD3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTD3_A {
        match self.bits {
            false => HTD3_A::_0,
            true => HTD3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HTD3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HTD3_A::_1
    }
}
#[doc = "Field `HTD3` writer - Huffman table number (DC) for the third color component NOTE: Read-only in Decompression."]
pub type HTD3_W<'a, const O: u8> = crate::BitWriter<'a, u8, JCHTN_SPEC, HTD3_A, O>;
impl<'a, const O: u8> HTD3_W<'a, O> {
    #[doc = "DC Huffman table 0(HTA3=0)/Setting prohibited(HTA3=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HTD3_A::_0)
    }
    #[doc = "DC Huffman table 1(HTA3=1)/Setting prohibited(HTA3=0)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HTD3_A::_1)
    }
}
#[doc = "Field `HTA3` reader - Huffman table number (AC) for the third color componentNOTE: Read-only in Decompression."]
pub type HTA3_R = crate::BitReader<HTA3_A>;
#[doc = "Huffman table number (AC) for the third color componentNOTE: Read-only in Decompression.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTA3_A {
    #[doc = "0: AC Huffman table 0(HTD3=0)/Setting prohibited(HTD3=1)"]
    _0 = 0,
    #[doc = "1: AC Huffman table 1(HTD3=1)/Setting prohibited(HTD3=0)"]
    _1 = 1,
}
impl From<HTA3_A> for bool {
    #[inline(always)]
    fn from(variant: HTA3_A) -> Self {
        variant as u8 != 0
    }
}
impl HTA3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTA3_A {
        match self.bits {
            false => HTA3_A::_0,
            true => HTA3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HTA3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HTA3_A::_1
    }
}
#[doc = "Field `HTA3` writer - Huffman table number (AC) for the third color componentNOTE: Read-only in Decompression."]
pub type HTA3_W<'a, const O: u8> = crate::BitWriter<'a, u8, JCHTN_SPEC, HTA3_A, O>;
impl<'a, const O: u8> HTA3_W<'a, O> {
    #[doc = "AC Huffman table 0(HTD3=0)/Setting prohibited(HTD3=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HTA3_A::_0)
    }
    #[doc = "AC Huffman table 1(HTD3=1)/Setting prohibited(HTD3=0)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HTA3_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Huffman table number (DC) for the first color component NOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn htd1(&self) -> HTD1_R {
        HTD1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Huffman table number (AC) for the first color componentNOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn hta1(&self) -> HTA1_R {
        HTA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Huffman table number (DC) for the second color component NOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn htd2(&self) -> HTD2_R {
        HTD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Huffman table number (AC) for the second color componentNOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn hta2(&self) -> HTA2_R {
        HTA2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Huffman table number (DC) for the third color component NOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn htd3(&self) -> HTD3_R {
        HTD3_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Huffman table number (AC) for the third color componentNOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn hta3(&self) -> HTA3_R {
        HTA3_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Huffman table number (DC) for the first color component NOTE: Read-only in Decompression."]
    #[inline(always)]
    #[must_use]
    pub fn htd1(&mut self) -> HTD1_W<0> {
        HTD1_W::new(self)
    }
    #[doc = "Bit 1 - Huffman table number (AC) for the first color componentNOTE: Read-only in Decompression."]
    #[inline(always)]
    #[must_use]
    pub fn hta1(&mut self) -> HTA1_W<1> {
        HTA1_W::new(self)
    }
    #[doc = "Bit 2 - Huffman table number (DC) for the second color component NOTE: Read-only in Decompression."]
    #[inline(always)]
    #[must_use]
    pub fn htd2(&mut self) -> HTD2_W<2> {
        HTD2_W::new(self)
    }
    #[doc = "Bit 3 - Huffman table number (AC) for the second color componentNOTE: Read-only in Decompression."]
    #[inline(always)]
    #[must_use]
    pub fn hta2(&mut self) -> HTA2_W<3> {
        HTA2_W::new(self)
    }
    #[doc = "Bit 4 - Huffman table number (DC) for the third color component NOTE: Read-only in Decompression."]
    #[inline(always)]
    #[must_use]
    pub fn htd3(&mut self) -> HTD3_W<4> {
        HTD3_W::new(self)
    }
    #[doc = "Bit 5 - Huffman table number (AC) for the third color componentNOTE: Read-only in Decompression."]
    #[inline(always)]
    #[must_use]
    pub fn hta3(&mut self) -> HTA3_W<5> {
        HTA3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG Code Huffman Table Number Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jchtn](index.html) module"]
pub struct JCHTN_SPEC;
impl crate::RegisterSpec for JCHTN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [jchtn::R](R) reader structure"]
impl crate::Readable for JCHTN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jchtn::W](W) writer structure"]
impl crate::Writable for JCHTN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JCHTN to value 0"]
impl crate::Resettable for JCHTN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
