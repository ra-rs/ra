#[doc = "Register `CFDGCTR` reader"]
pub struct R(crate::R<CFDGCTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDGCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDGCTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDGCTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDGCTR` writer"]
pub struct W(crate::W<CFDGCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDGCTR_SPEC>;
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
impl From<crate::W<CFDGCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDGCTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GMDC` reader - Global Mode Control"]
pub type GMDC_R = crate::FieldReader<u8, GMDC_A>;
#[doc = "Global Mode Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GMDC_A {
    #[doc = "0: Global operation mode request"]
    _00 = 0,
    #[doc = "1: Global reset mode request"]
    _01 = 1,
    #[doc = "2: Global halt mode request"]
    _10 = 2,
    #[doc = "3: Keep current value"]
    _11 = 3,
}
impl From<GMDC_A> for u8 {
    #[inline(always)]
    fn from(variant: GMDC_A) -> Self {
        variant as _
    }
}
impl GMDC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GMDC_A {
        match self.bits {
            0 => GMDC_A::_00,
            1 => GMDC_A::_01,
            2 => GMDC_A::_10,
            3 => GMDC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == GMDC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == GMDC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == GMDC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == GMDC_A::_11
    }
}
#[doc = "Field `GMDC` writer - Global Mode Control"]
pub type GMDC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFDGCTR_SPEC, u8, GMDC_A, 2, O>;
impl<'a, const O: u8> GMDC_W<'a, O> {
    #[doc = "Global operation mode request"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(GMDC_A::_00)
    }
    #[doc = "Global reset mode request"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(GMDC_A::_01)
    }
    #[doc = "Global halt mode request"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(GMDC_A::_10)
    }
    #[doc = "Keep current value"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(GMDC_A::_11)
    }
}
#[doc = "Field `GSLPR` reader - Global Sleep Request"]
pub type GSLPR_R = crate::BitReader<GSLPR_A>;
#[doc = "Global Sleep Request\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GSLPR_A {
    #[doc = "0: Global sleep request disabled"]
    _0 = 0,
    #[doc = "1: Global sleep request enabled"]
    _1 = 1,
}
impl From<GSLPR_A> for bool {
    #[inline(always)]
    fn from(variant: GSLPR_A) -> Self {
        variant as u8 != 0
    }
}
impl GSLPR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GSLPR_A {
        match self.bits {
            false => GSLPR_A::_0,
            true => GSLPR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GSLPR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GSLPR_A::_1
    }
}
#[doc = "Field `GSLPR` writer - Global Sleep Request"]
pub type GSLPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGCTR_SPEC, GSLPR_A, O>;
impl<'a, const O: u8> GSLPR_W<'a, O> {
    #[doc = "Global sleep request disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GSLPR_A::_0)
    }
    #[doc = "Global sleep request enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GSLPR_A::_1)
    }
}
#[doc = "Field `DEIE` reader - DLC Check Interrupt Enable"]
pub type DEIE_R = crate::BitReader<DEIE_A>;
#[doc = "DLC Check Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEIE_A {
    #[doc = "0: DLC check interrupt disabled"]
    _0 = 0,
    #[doc = "1: DLC check interrupt enabled"]
    _1 = 1,
}
impl From<DEIE_A> for bool {
    #[inline(always)]
    fn from(variant: DEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl DEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEIE_A {
        match self.bits {
            false => DEIE_A::_0,
            true => DEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DEIE_A::_1
    }
}
#[doc = "Field `DEIE` writer - DLC Check Interrupt Enable"]
pub type DEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGCTR_SPEC, DEIE_A, O>;
impl<'a, const O: u8> DEIE_W<'a, O> {
    #[doc = "DLC check interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DEIE_A::_0)
    }
    #[doc = "DLC check interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DEIE_A::_1)
    }
}
#[doc = "Field `MEIE` reader - Message Lost Error Interrupt Enable"]
pub type MEIE_R = crate::BitReader<MEIE_A>;
#[doc = "Message Lost Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEIE_A {
    #[doc = "0: Message lost error interrupt disabled"]
    _0 = 0,
    #[doc = "1: Message lost error interrupt enabled"]
    _1 = 1,
}
impl From<MEIE_A> for bool {
    #[inline(always)]
    fn from(variant: MEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl MEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEIE_A {
        match self.bits {
            false => MEIE_A::_0,
            true => MEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MEIE_A::_1
    }
}
#[doc = "Field `MEIE` writer - Message Lost Error Interrupt Enable"]
pub type MEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGCTR_SPEC, MEIE_A, O>;
impl<'a, const O: u8> MEIE_W<'a, O> {
    #[doc = "Message lost error interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MEIE_A::_0)
    }
    #[doc = "Message lost error interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MEIE_A::_1)
    }
}
#[doc = "Field `THLEIE` reader - TX History List Entry Lost Interrupt Enable"]
pub type THLEIE_R = crate::BitReader<THLEIE_A>;
#[doc = "TX History List Entry Lost Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THLEIE_A {
    #[doc = "0: TX history list entry lost interrupt disabled"]
    _0 = 0,
    #[doc = "1: TX history list entry lost interrupt enabled"]
    _1 = 1,
}
impl From<THLEIE_A> for bool {
    #[inline(always)]
    fn from(variant: THLEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl THLEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THLEIE_A {
        match self.bits {
            false => THLEIE_A::_0,
            true => THLEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == THLEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == THLEIE_A::_1
    }
}
#[doc = "Field `THLEIE` writer - TX History List Entry Lost Interrupt Enable"]
pub type THLEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGCTR_SPEC, THLEIE_A, O>;
impl<'a, const O: u8> THLEIE_W<'a, O> {
    #[doc = "TX history list entry lost interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(THLEIE_A::_0)
    }
    #[doc = "TX history list entry lost interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(THLEIE_A::_1)
    }
}
#[doc = "Field `CMPOFIE` reader - CANFD Message Payload Overflow Flag Interrupt Enable"]
pub type CMPOFIE_R = crate::BitReader<CMPOFIE_A>;
#[doc = "CANFD Message Payload Overflow Flag Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPOFIE_A {
    #[doc = "0: CANFD message payload overflow flag interrupt disabled"]
    _0 = 0,
    #[doc = "1: CANFD message payload overflow flag interrupt enabled"]
    _1 = 1,
}
impl From<CMPOFIE_A> for bool {
    #[inline(always)]
    fn from(variant: CMPOFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPOFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPOFIE_A {
        match self.bits {
            false => CMPOFIE_A::_0,
            true => CMPOFIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPOFIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPOFIE_A::_1
    }
}
#[doc = "Field `CMPOFIE` writer - CANFD Message Payload Overflow Flag Interrupt Enable"]
pub type CMPOFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGCTR_SPEC, CMPOFIE_A, O>;
impl<'a, const O: u8> CMPOFIE_W<'a, O> {
    #[doc = "CANFD message payload overflow flag interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPOFIE_A::_0)
    }
    #[doc = "CANFD message payload overflow flag interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPOFIE_A::_1)
    }
}
#[doc = "Field `TSRST` reader - Timestamp Reset"]
pub type TSRST_R = crate::BitReader<TSRST_A>;
#[doc = "Timestamp Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSRST_A {
    #[doc = "0: Timestamp not reset"]
    _0 = 0,
    #[doc = "1: Timestamp reset"]
    _1 = 1,
}
impl From<TSRST_A> for bool {
    #[inline(always)]
    fn from(variant: TSRST_A) -> Self {
        variant as u8 != 0
    }
}
impl TSRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSRST_A {
        match self.bits {
            false => TSRST_A::_0,
            true => TSRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSRST_A::_1
    }
}
#[doc = "Field `TSRST` writer - Timestamp Reset"]
pub type TSRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGCTR_SPEC, TSRST_A, O>;
impl<'a, const O: u8> TSRST_W<'a, O> {
    #[doc = "Timestamp not reset"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSRST_A::_0)
    }
    #[doc = "Timestamp reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSRST_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Global Mode Control"]
    #[inline(always)]
    pub fn gmdc(&self) -> GMDC_R {
        GMDC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Global Sleep Request"]
    #[inline(always)]
    pub fn gslpr(&self) -> GSLPR_R {
        GSLPR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - DLC Check Interrupt Enable"]
    #[inline(always)]
    pub fn deie(&self) -> DEIE_R {
        DEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Message Lost Error Interrupt Enable"]
    #[inline(always)]
    pub fn meie(&self) -> MEIE_R {
        MEIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TX History List Entry Lost Interrupt Enable"]
    #[inline(always)]
    pub fn thleie(&self) -> THLEIE_R {
        THLEIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CANFD Message Payload Overflow Flag Interrupt Enable"]
    #[inline(always)]
    pub fn cmpofie(&self) -> CMPOFIE_R {
        CMPOFIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Timestamp Reset"]
    #[inline(always)]
    pub fn tsrst(&self) -> TSRST_R {
        TSRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Global Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn gmdc(&mut self) -> GMDC_W<0> {
        GMDC_W::new(self)
    }
    #[doc = "Bit 2 - Global Sleep Request"]
    #[inline(always)]
    #[must_use]
    pub fn gslpr(&mut self) -> GSLPR_W<2> {
        GSLPR_W::new(self)
    }
    #[doc = "Bit 8 - DLC Check Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn deie(&mut self) -> DEIE_W<8> {
        DEIE_W::new(self)
    }
    #[doc = "Bit 9 - Message Lost Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn meie(&mut self) -> MEIE_W<9> {
        MEIE_W::new(self)
    }
    #[doc = "Bit 10 - TX History List Entry Lost Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn thleie(&mut self) -> THLEIE_W<10> {
        THLEIE_W::new(self)
    }
    #[doc = "Bit 11 - CANFD Message Payload Overflow Flag Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpofie(&mut self) -> CMPOFIE_W<11> {
        CMPOFIE_W::new(self)
    }
    #[doc = "Bit 16 - Timestamp Reset"]
    #[inline(always)]
    #[must_use]
    pub fn tsrst(&mut self) -> TSRST_W<16> {
        TSRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdgctr](index.html) module"]
pub struct CFDGCTR_SPEC;
impl crate::RegisterSpec for CFDGCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdgctr::R](R) reader structure"]
impl crate::Readable for CFDGCTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdgctr::W](W) writer structure"]
impl crate::Writable for CFDGCTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDGCTR to value 0x05"]
impl crate::Resettable for CFDGCTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
