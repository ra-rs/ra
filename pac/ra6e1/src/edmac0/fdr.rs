#[doc = "Register `FDR` reader"]
pub struct R(crate::R<FDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDR` writer"]
pub struct W(crate::W<FDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDR_SPEC>;
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
impl From<crate::W<FDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFD` reader - Receive FIFO Depth"]
pub type RFD_R = crate::FieldReader<u8, RFD_A>;
#[doc = "Receive FIFO Depth\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RFD_A {
    #[doc = "15: 4096 bytes"]
    _0X0F = 15,
}
impl From<RFD_A> for u8 {
    #[inline(always)]
    fn from(variant: RFD_A) -> Self {
        variant as _
    }
}
impl RFD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RFD_A> {
        match self.bits {
            15 => Some(RFD_A::_0X0F),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0F`"]
    #[inline(always)]
    pub fn is_0x0f(&self) -> bool {
        *self == RFD_A::_0X0F
    }
}
#[doc = "Field `RFD` writer - Receive FIFO Depth"]
pub type RFD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDR_SPEC, u8, RFD_A, 5, O>;
impl<'a, const O: u8> RFD_W<'a, O> {
    #[doc = "4096 bytes"]
    #[inline(always)]
    pub fn _0x0f(self) -> &'a mut W {
        self.variant(RFD_A::_0X0F)
    }
}
#[doc = "Field `TFD` reader - Transmit FIFO Depth"]
pub type TFD_R = crate::FieldReader<u8, TFD_A>;
#[doc = "Transmit FIFO Depth\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TFD_A {
    #[doc = "7: 2048 bytes"]
    _0X07 = 7,
}
impl From<TFD_A> for u8 {
    #[inline(always)]
    fn from(variant: TFD_A) -> Self {
        variant as _
    }
}
impl TFD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TFD_A> {
        match self.bits {
            7 => Some(TFD_A::_0X07),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X07`"]
    #[inline(always)]
    pub fn is_0x07(&self) -> bool {
        *self == TFD_A::_0X07
    }
}
#[doc = "Field `TFD` writer - Transmit FIFO Depth"]
pub type TFD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDR_SPEC, u8, TFD_A, 5, O>;
impl<'a, const O: u8> TFD_W<'a, O> {
    #[doc = "2048 bytes"]
    #[inline(always)]
    pub fn _0x07(self) -> &'a mut W {
        self.variant(TFD_A::_0X07)
    }
}
impl R {
    #[doc = "Bits 0:4 - Receive FIFO Depth"]
    #[inline(always)]
    pub fn rfd(&self) -> RFD_R {
        RFD_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Transmit FIFO Depth"]
    #[inline(always)]
    pub fn tfd(&self) -> TFD_R {
        TFD_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Receive FIFO Depth"]
    #[inline(always)]
    #[must_use]
    pub fn rfd(&mut self) -> RFD_W<0> {
        RFD_W::new(self)
    }
    #[doc = "Bits 8:12 - Transmit FIFO Depth"]
    #[inline(always)]
    #[must_use]
    pub fn tfd(&mut self) -> TFD_W<8> {
        TFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Depth Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdr](index.html) module"]
pub struct FDR_SPEC;
impl crate::RegisterSpec for FDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdr::R](R) reader structure"]
impl crate::Readable for FDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdr::W](W) writer structure"]
impl crate::Writable for FDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDR to value 0"]
impl crate::Resettable for FDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
