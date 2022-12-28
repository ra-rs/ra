#[doc = "Register `D0FIFO` reader"]
pub struct R(crate::R<D0FIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D0FIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D0FIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D0FIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D0FIFO` writer"]
pub struct W(crate::W<D0FIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D0FIFO_SPEC>;
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
impl From<crate::W<D0FIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D0FIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOPORT` reader - FIFO Port Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
pub type FIFOPORT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FIFOPORT` writer - FIFO Port Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
pub type FIFOPORT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, D0FIFO_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - FIFO Port Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
    #[inline(always)]
    pub fn fifoport(&self) -> FIFOPORT_R {
        FIFOPORT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - FIFO Port Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
    #[inline(always)]
    #[must_use]
    pub fn fifoport(&mut self) -> FIFOPORT_W<0> {
        FIFOPORT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "D0FIFO Port Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d0fifo](index.html) module"]
pub struct D0FIFO_SPEC;
impl crate::RegisterSpec for D0FIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d0fifo::R](R) reader structure"]
impl crate::Readable for D0FIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d0fifo::W](W) writer structure"]
impl crate::Writable for D0FIFO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D0FIFO to value 0"]
impl crate::Resettable for D0FIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
