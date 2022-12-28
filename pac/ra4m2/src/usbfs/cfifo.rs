#[doc = "Register `CFIFO` reader"]
pub struct R(crate::R<CFIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFIFO` writer"]
pub struct W(crate::W<CFIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFIFO_SPEC>;
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
impl From<crate::W<CFIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOPORT` reader - FIFO Port"]
pub type FIFOPORT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FIFOPORT` writer - FIFO Port"]
pub type FIFOPORT_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CFIFO_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - FIFO Port"]
    #[inline(always)]
    pub fn fifoport(&self) -> FIFOPORT_R {
        FIFOPORT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - FIFO Port"]
    #[inline(always)]
    #[must_use]
    pub fn fifoport(&mut self) -> FIFOPORT_W<0> {
        FIFOPORT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CFIFO Port Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfifo](index.html) module"]
pub struct CFIFO_SPEC;
impl crate::RegisterSpec for CFIFO_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cfifo::R](R) reader structure"]
impl crate::Readable for CFIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfifo::W](W) writer structure"]
impl crate::Writable for CFIFO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFIFO to value 0"]
impl crate::Resettable for CFIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
