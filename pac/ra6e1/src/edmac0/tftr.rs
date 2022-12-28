#[doc = "Register `TFTR` reader"]
pub struct R(crate::R<TFTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TFTR` writer"]
pub struct W(crate::W<TFTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TFTR_SPEC>;
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
impl From<crate::W<TFTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TFTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TFT` reader - Transmit FIFO Threshold"]
pub type TFT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TFT` writer - Transmit FIFO Threshold"]
pub type TFT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TFTR_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - Transmit FIFO Threshold"]
    #[inline(always)]
    pub fn tft(&self) -> TFT_R {
        TFT_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Transmit FIFO Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn tft(&mut self) -> TFT_W<0> {
        TFT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit FIFO Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tftr](index.html) module"]
pub struct TFTR_SPEC;
impl crate::RegisterSpec for TFTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tftr::R](R) reader structure"]
impl crate::Readable for TFTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tftr::W](W) writer structure"]
impl crate::Writable for TFTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TFTR to value 0"]
impl crate::Resettable for TFTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
