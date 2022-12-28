#[doc = "Register `ADFIFOINTLR4` reader"]
pub struct R(crate::R<ADFIFOINTLR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADFIFOINTLR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADFIFOINTLR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADFIFOINTLR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADFIFOINTLR4` writer"]
pub struct W(crate::W<ADFIFOINTLR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADFIFOINTLR4_SPEC>;
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
impl From<crate::W<ADFIFOINTLR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADFIFOINTLR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOILV8` reader - Scan Group 8 FIFO Interrupt Output Timing Setting"]
pub type FIFOILV8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFOILV8` writer - Scan Group 8 FIFO Interrupt Output Timing Setting"]
pub type FIFOILV8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADFIFOINTLR4_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Scan Group 8 FIFO Interrupt Output Timing Setting"]
    #[inline(always)]
    pub fn fifoilv8(&self) -> FIFOILV8_R {
        FIFOILV8_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Scan Group 8 FIFO Interrupt Output Timing Setting"]
    #[inline(always)]
    #[must_use]
    pub fn fifoilv8(&mut self) -> FIFOILV8_W<0> {
        FIFOILV8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Interrupt Generation Level Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adfifointlr4](index.html) module"]
pub struct ADFIFOINTLR4_SPEC;
impl crate::RegisterSpec for ADFIFOINTLR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adfifointlr4::R](R) reader structure"]
impl crate::Readable for ADFIFOINTLR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adfifointlr4::W](W) writer structure"]
impl crate::Writable for ADFIFOINTLR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADFIFOINTLR4 to value 0"]
impl crate::Resettable for ADFIFOINTLR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
