#[doc = "Register `RFOCR` reader"]
pub struct R(crate::R<RFOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFOCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFOCR` writer"]
pub struct W(crate::W<RFOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFOCR_SPEC>;
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
impl From<crate::W<RFOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFOCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVER` reader - Receive FIFO Overflow CountThese bits indicate how many times the receive FIFO has overflowed. The counter stops when the counter value reaches FFFFh."]
pub type OVER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OVER` writer - Receive FIFO Overflow CountThese bits indicate how many times the receive FIFO has overflowed. The counter stops when the counter value reaches FFFFh."]
pub type OVER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RFOCR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Receive FIFO Overflow CountThese bits indicate how many times the receive FIFO has overflowed. The counter stops when the counter value reaches FFFFh."]
    #[inline(always)]
    pub fn over(&self) -> OVER_R {
        OVER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive FIFO Overflow CountThese bits indicate how many times the receive FIFO has overflowed. The counter stops when the counter value reaches FFFFh."]
    #[inline(always)]
    #[must_use]
    pub fn over(&mut self) -> OVER_W<0> {
        OVER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive FIFO Overflow Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfocr](index.html) module"]
pub struct RFOCR_SPEC;
impl crate::RegisterSpec for RFOCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfocr::R](R) reader structure"]
impl crate::Readable for RFOCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfocr::W](W) writer structure"]
impl crate::Writable for RFOCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFOCR to value 0"]
impl crate::Resettable for RFOCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
