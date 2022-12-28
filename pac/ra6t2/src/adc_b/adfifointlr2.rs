#[doc = "Register `ADFIFOINTLR2` reader"]
pub struct R(crate::R<ADFIFOINTLR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADFIFOINTLR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADFIFOINTLR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADFIFOINTLR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADFIFOINTLR2` writer"]
pub struct W(crate::W<ADFIFOINTLR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADFIFOINTLR2_SPEC>;
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
impl From<crate::W<ADFIFOINTLR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADFIFOINTLR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOILV4` reader - Scan Group 4 FIFO Interrupt Output Timing Setting"]
pub type FIFOILV4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFOILV4` writer - Scan Group 4 FIFO Interrupt Output Timing Setting"]
pub type FIFOILV4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADFIFOINTLR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `FIFOILV5` reader - Scan Group 5 FIFO Interrupt Output Timing Setting"]
pub type FIFOILV5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFOILV5` writer - Scan Group 5 FIFO Interrupt Output Timing Setting"]
pub type FIFOILV5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADFIFOINTLR2_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Scan Group 4 FIFO Interrupt Output Timing Setting"]
    #[inline(always)]
    pub fn fifoilv4(&self) -> FIFOILV4_R {
        FIFOILV4_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Scan Group 5 FIFO Interrupt Output Timing Setting"]
    #[inline(always)]
    pub fn fifoilv5(&self) -> FIFOILV5_R {
        FIFOILV5_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Scan Group 4 FIFO Interrupt Output Timing Setting"]
    #[inline(always)]
    #[must_use]
    pub fn fifoilv4(&mut self) -> FIFOILV4_W<0> {
        FIFOILV4_W::new(self)
    }
    #[doc = "Bits 16:19 - Scan Group 5 FIFO Interrupt Output Timing Setting"]
    #[inline(always)]
    #[must_use]
    pub fn fifoilv5(&mut self) -> FIFOILV5_W<16> {
        FIFOILV5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Interrupt Generation Level Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adfifointlr2](index.html) module"]
pub struct ADFIFOINTLR2_SPEC;
impl crate::RegisterSpec for ADFIFOINTLR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adfifointlr2::R](R) reader structure"]
impl crate::Readable for ADFIFOINTLR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adfifointlr2::W](W) writer structure"]
impl crate::Writable for ADFIFOINTLR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADFIFOINTLR2 to value 0"]
impl crate::Resettable for ADFIFOINTLR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
