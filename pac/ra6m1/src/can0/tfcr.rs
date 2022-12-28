#[doc = "Register `TFCR` reader"]
pub struct R(crate::R<TFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TFCR` writer"]
pub struct W(crate::W<TFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TFCR_SPEC>;
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
impl From<crate::W<TFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TFE` reader - Transmit FIFO Enable"]
pub type TFE_R = crate::BitReader<TFE_A>;
#[doc = "Transmit FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFE_A {
    #[doc = "0: Transmit FIFO disabled"]
    _0 = 0,
    #[doc = "1: Transmit FIFO enabled"]
    _1 = 1,
}
impl From<TFE_A> for bool {
    #[inline(always)]
    fn from(variant: TFE_A) -> Self {
        variant as u8 != 0
    }
}
impl TFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFE_A {
        match self.bits {
            false => TFE_A::_0,
            true => TFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFE_A::_1
    }
}
#[doc = "Field `TFE` writer - Transmit FIFO Enable"]
pub type TFE_W<'a, const O: u8> = crate::BitWriter<'a, u8, TFCR_SPEC, TFE_A, O>;
impl<'a, const O: u8> TFE_W<'a, O> {
    #[doc = "Transmit FIFO disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFE_A::_0)
    }
    #[doc = "Transmit FIFO enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFE_A::_1)
    }
}
#[doc = "Field `TFUST` reader - Transmit FIFO Unsent Message Number Status"]
pub type TFUST_R = crate::FieldReader<u8, TFUST_A>;
#[doc = "Transmit FIFO Unsent Message Number Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TFUST_A {
    #[doc = "0: No unsent message"]
    _000 = 0,
    #[doc = "1: 1 unsent message"]
    _001 = 1,
    #[doc = "2: 2 unsent messages"]
    _010 = 2,
    #[doc = "3: 3 unsent messages"]
    _011 = 3,
    #[doc = "4: 4 unsent messages"]
    _100 = 4,
}
impl From<TFUST_A> for u8 {
    #[inline(always)]
    fn from(variant: TFUST_A) -> Self {
        variant as _
    }
}
impl TFUST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TFUST_A> {
        match self.bits {
            0 => Some(TFUST_A::_000),
            1 => Some(TFUST_A::_001),
            2 => Some(TFUST_A::_010),
            3 => Some(TFUST_A::_011),
            4 => Some(TFUST_A::_100),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == TFUST_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == TFUST_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == TFUST_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == TFUST_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == TFUST_A::_100
    }
}
#[doc = "Field `TFFST` reader - Transmit FIFO Full Status"]
pub type TFFST_R = crate::BitReader<TFFST_A>;
#[doc = "Transmit FIFO Full Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFFST_A {
    #[doc = "0: Transmit FIFO is not full"]
    _0 = 0,
    #[doc = "1: Transmit FIFO is full (4 unsent messages)"]
    _1 = 1,
}
impl From<TFFST_A> for bool {
    #[inline(always)]
    fn from(variant: TFFST_A) -> Self {
        variant as u8 != 0
    }
}
impl TFFST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFFST_A {
        match self.bits {
            false => TFFST_A::_0,
            true => TFFST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFFST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFFST_A::_1
    }
}
#[doc = "Field `TFEST` reader - Transmit FIFO Empty Status"]
pub type TFEST_R = crate::BitReader<TFEST_A>;
#[doc = "Transmit FIFO Empty Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFEST_A {
    #[doc = "0: Unsent message in transmit FIFO"]
    _0 = 0,
    #[doc = "1: No unsent message in transmit FIFO"]
    _1 = 1,
}
impl From<TFEST_A> for bool {
    #[inline(always)]
    fn from(variant: TFEST_A) -> Self {
        variant as u8 != 0
    }
}
impl TFEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFEST_A {
        match self.bits {
            false => TFEST_A::_0,
            true => TFEST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFEST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFEST_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Transmit FIFO Enable"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Transmit FIFO Unsent Message Number Status"]
    #[inline(always)]
    pub fn tfust(&self) -> TFUST_R {
        TFUST_R::new((self.bits >> 1) & 7)
    }
    #[doc = "Bit 6 - Transmit FIFO Full Status"]
    #[inline(always)]
    pub fn tffst(&self) -> TFFST_R {
        TFFST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO Empty Status"]
    #[inline(always)]
    pub fn tfest(&self) -> TFEST_R {
        TFEST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TFE_W<0> {
        TFE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfcr](index.html) module"]
pub struct TFCR_SPEC;
impl crate::RegisterSpec for TFCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tfcr::R](R) reader structure"]
impl crate::Readable for TFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tfcr::W](W) writer structure"]
impl crate::Writable for TFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TFCR to value 0x80"]
impl crate::Resettable for TFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
