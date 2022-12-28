#[doc = "Register `CFDTXQCC` reader"]
pub struct R(crate::R<CFDTXQCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDTXQCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDTXQCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDTXQCC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDTXQCC` writer"]
pub struct W(crate::W<CFDTXQCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDTXQCC_SPEC>;
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
impl From<crate::W<CFDTXQCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDTXQCC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXQE` reader - TX Queue Enable"]
pub type TXQE_R = crate::BitReader<TXQE_A>;
#[doc = "TX Queue Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXQE_A {
    #[doc = "0: TX Queue disabled"]
    _0 = 0,
    #[doc = "1: TX Queue enabled"]
    _1 = 1,
}
impl From<TXQE_A> for bool {
    #[inline(always)]
    fn from(variant: TXQE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXQE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXQE_A {
        match self.bits {
            false => TXQE_A::_0,
            true => TXQE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXQE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXQE_A::_1
    }
}
#[doc = "Field `TXQE` writer - TX Queue Enable"]
pub type TXQE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDTXQCC_SPEC, TXQE_A, O>;
impl<'a, const O: u8> TXQE_W<'a, O> {
    #[doc = "TX Queue disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXQE_A::_0)
    }
    #[doc = "TX Queue enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXQE_A::_1)
    }
}
#[doc = "Field `TXQTXIE` reader - TX Queue TX Interrupt Enable"]
pub type TXQTXIE_R = crate::BitReader<TXQTXIE_A>;
#[doc = "TX Queue TX Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXQTXIE_A {
    #[doc = "0: TX Queue TX interrupt disabled"]
    _0 = 0,
    #[doc = "1: TX Queue TX interrupt enabled"]
    _1 = 1,
}
impl From<TXQTXIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXQTXIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXQTXIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXQTXIE_A {
        match self.bits {
            false => TXQTXIE_A::_0,
            true => TXQTXIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXQTXIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXQTXIE_A::_1
    }
}
#[doc = "Field `TXQTXIE` writer - TX Queue TX Interrupt Enable"]
pub type TXQTXIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDTXQCC_SPEC, TXQTXIE_A, O>;
impl<'a, const O: u8> TXQTXIE_W<'a, O> {
    #[doc = "TX Queue TX interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXQTXIE_A::_0)
    }
    #[doc = "TX Queue TX interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXQTXIE_A::_1)
    }
}
#[doc = "Field `TXQIM` reader - TX Queue Interrupt Mode"]
pub type TXQIM_R = crate::BitReader<TXQIM_A>;
#[doc = "TX Queue Interrupt Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXQIM_A {
    #[doc = "0: When the last message is successfully transmitted"]
    _0 = 0,
    #[doc = "1: At every successful transmission"]
    _1 = 1,
}
impl From<TXQIM_A> for bool {
    #[inline(always)]
    fn from(variant: TXQIM_A) -> Self {
        variant as u8 != 0
    }
}
impl TXQIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXQIM_A {
        match self.bits {
            false => TXQIM_A::_0,
            true => TXQIM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXQIM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXQIM_A::_1
    }
}
#[doc = "Field `TXQIM` writer - TX Queue Interrupt Mode"]
pub type TXQIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDTXQCC_SPEC, TXQIM_A, O>;
impl<'a, const O: u8> TXQIM_W<'a, O> {
    #[doc = "When the last message is successfully transmitted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXQIM_A::_0)
    }
    #[doc = "At every successful transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXQIM_A::_1)
    }
}
#[doc = "Field `TXQDC` reader - TX Queue Depth Configuration"]
pub type TXQDC_R = crate::FieldReader<u8, TXQDC_A>;
#[doc = "TX Queue Depth Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXQDC_A {
    #[doc = "0: 0 messages"]
    ZERO = 0,
    #[doc = "2: 3 messages"]
    THREE = 2,
    #[doc = "3: 4 messages"]
    FOUR = 3,
}
impl From<TXQDC_A> for u8 {
    #[inline(always)]
    fn from(variant: TXQDC_A) -> Self {
        variant as _
    }
}
impl TXQDC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXQDC_A> {
        match self.bits {
            0 => Some(TXQDC_A::ZERO),
            2 => Some(TXQDC_A::THREE),
            3 => Some(TXQDC_A::FOUR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == TXQDC_A::ZERO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == TXQDC_A::THREE
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == TXQDC_A::FOUR
    }
}
#[doc = "Field `TXQDC` writer - TX Queue Depth Configuration"]
pub type TXQDC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDTXQCC_SPEC, u8, TXQDC_A, 2, O>;
impl<'a, const O: u8> TXQDC_W<'a, O> {
    #[doc = "0 messages"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(TXQDC_A::ZERO)
    }
    #[doc = "3 messages"]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(TXQDC_A::THREE)
    }
    #[doc = "4 messages"]
    #[inline(always)]
    pub fn four(self) -> &'a mut W {
        self.variant(TXQDC_A::FOUR)
    }
}
impl R {
    #[doc = "Bit 0 - TX Queue Enable"]
    #[inline(always)]
    pub fn txqe(&self) -> TXQE_R {
        TXQE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - TX Queue TX Interrupt Enable"]
    #[inline(always)]
    pub fn txqtxie(&self) -> TXQTXIE_R {
        TXQTXIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - TX Queue Interrupt Mode"]
    #[inline(always)]
    pub fn txqim(&self) -> TXQIM_R {
        TXQIM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - TX Queue Depth Configuration"]
    #[inline(always)]
    pub fn txqdc(&self) -> TXQDC_R {
        TXQDC_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TX Queue Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txqe(&mut self) -> TXQE_W<0> {
        TXQE_W::new(self)
    }
    #[doc = "Bit 5 - TX Queue TX Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txqtxie(&mut self) -> TXQTXIE_W<5> {
        TXQTXIE_W::new(self)
    }
    #[doc = "Bit 7 - TX Queue Interrupt Mode"]
    #[inline(always)]
    #[must_use]
    pub fn txqim(&mut self) -> TXQIM_W<7> {
        TXQIM_W::new(self)
    }
    #[doc = "Bits 8:9 - TX Queue Depth Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn txqdc(&mut self) -> TXQDC_W<8> {
        TXQDC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX Queue Configuration/Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdtxqcc](index.html) module"]
pub struct CFDTXQCC_SPEC;
impl crate::RegisterSpec for CFDTXQCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdtxqcc::R](R) reader structure"]
impl crate::Readable for CFDTXQCC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdtxqcc::W](W) writer structure"]
impl crate::Writable for CFDTXQCC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDTXQCC to value 0"]
impl crate::Resettable for CFDTXQCC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
