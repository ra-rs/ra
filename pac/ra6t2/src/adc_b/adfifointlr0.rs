#[doc = "Register `ADFIFOINTLR0` reader"]
pub struct R(crate::R<ADFIFOINTLR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADFIFOINTLR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADFIFOINTLR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADFIFOINTLR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADFIFOINTLR0` writer"]
pub struct W(crate::W<ADFIFOINTLR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADFIFOINTLR0_SPEC>;
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
impl From<crate::W<ADFIFOINTLR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADFIFOINTLR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOILV0` reader - Scan Group 0 FIFO Interrupt Output Timing Setting"]
pub type FIFOILV0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFOILV0` writer - Scan Group 0 FIFO Interrupt Output Timing Setting"]
pub type FIFOILV0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADFIFOINTLR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `FIFOILV1` reader - Scan Group 1 FIFO Interrupt Output Timing Setting"]
pub type FIFOILV1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFOILV1` writer - Scan Group 1 FIFO Interrupt Output Timing Setting"]
pub type FIFOILV1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADFIFOINTLR0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Scan Group 0 FIFO Interrupt Output Timing Setting"]
    #[inline(always)]
    pub fn fifoilv0(&self) -> FIFOILV0_R {
        FIFOILV0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Scan Group 1 FIFO Interrupt Output Timing Setting"]
    #[inline(always)]
    pub fn fifoilv1(&self) -> FIFOILV1_R {
        FIFOILV1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Scan Group 0 FIFO Interrupt Output Timing Setting"]
    #[inline(always)]
    #[must_use]
    pub fn fifoilv0(&mut self) -> FIFOILV0_W<0> {
        FIFOILV0_W::new(self)
    }
    #[doc = "Bits 16:19 - Scan Group 1 FIFO Interrupt Output Timing Setting"]
    #[inline(always)]
    #[must_use]
    pub fn fifoilv1(&mut self) -> FIFOILV1_W<16> {
        FIFOILV1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Interrupt Generation Level Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adfifointlr0](index.html) module"]
pub struct ADFIFOINTLR0_SPEC;
impl crate::RegisterSpec for ADFIFOINTLR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adfifointlr0::R](R) reader structure"]
impl crate::Readable for ADFIFOINTLR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adfifointlr0::W](W) writer structure"]
impl crate::Writable for ADFIFOINTLR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADFIFOINTLR0 to value 0"]
impl crate::Resettable for ADFIFOINTLR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
