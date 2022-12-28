#[doc = "Register `TFUCR` reader"]
pub struct R(crate::R<TFUCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFUCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFUCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFUCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TFUCR` writer"]
pub struct W(crate::W<TFUCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TFUCR_SPEC>;
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
impl From<crate::W<TFUCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TFUCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UNDER` reader - Transmit FIFO Underflow CountThese bits indicate how many times the transmit FIFO has underflowed. The counter stops when the counter value reaches FFFFh."]
pub type UNDER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `UNDER` writer - Transmit FIFO Underflow CountThese bits indicate how many times the transmit FIFO has underflowed. The counter stops when the counter value reaches FFFFh."]
pub type UNDER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TFUCR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Transmit FIFO Underflow CountThese bits indicate how many times the transmit FIFO has underflowed. The counter stops when the counter value reaches FFFFh."]
    #[inline(always)]
    pub fn under(&self) -> UNDER_R {
        UNDER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit FIFO Underflow CountThese bits indicate how many times the transmit FIFO has underflowed. The counter stops when the counter value reaches FFFFh."]
    #[inline(always)]
    #[must_use]
    pub fn under(&mut self) -> UNDER_W<0> {
        UNDER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit FIFO Underflow Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfucr](index.html) module"]
pub struct TFUCR_SPEC;
impl crate::RegisterSpec for TFUCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tfucr::R](R) reader structure"]
impl crate::Readable for TFUCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tfucr::W](W) writer structure"]
impl crate::Writable for TFUCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TFUCR to value 0"]
impl crate::Resettable for TFUCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
