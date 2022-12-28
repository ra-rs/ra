#[doc = "Register `ADFIFOINTLR1` reader"]
pub struct R(crate::R<ADFIFOINTLR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADFIFOINTLR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADFIFOINTLR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADFIFOINTLR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADFIFOINTLR1` writer"]
pub struct W(crate::W<ADFIFOINTLR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADFIFOINTLR1_SPEC>;
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
impl From<crate::W<ADFIFOINTLR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADFIFOINTLR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOILV2` reader - Scan Group 2 FIFO Interrupt Output Timing Setting"]
pub type FIFOILV2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFOILV2` writer - Scan Group 2 FIFO Interrupt Output Timing Setting"]
pub type FIFOILV2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADFIFOINTLR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `FIFOILV3` reader - Scan Group 3 FIFO Interrupt Output Timing Setting"]
pub type FIFOILV3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFOILV3` writer - Scan Group 3 FIFO Interrupt Output Timing Setting"]
pub type FIFOILV3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADFIFOINTLR1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Scan Group 2 FIFO Interrupt Output Timing Setting"]
    #[inline(always)]
    pub fn fifoilv2(&self) -> FIFOILV2_R {
        FIFOILV2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Scan Group 3 FIFO Interrupt Output Timing Setting"]
    #[inline(always)]
    pub fn fifoilv3(&self) -> FIFOILV3_R {
        FIFOILV3_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Scan Group 2 FIFO Interrupt Output Timing Setting"]
    #[inline(always)]
    #[must_use]
    pub fn fifoilv2(&mut self) -> FIFOILV2_W<0> {
        FIFOILV2_W::new(self)
    }
    #[doc = "Bits 16:19 - Scan Group 3 FIFO Interrupt Output Timing Setting"]
    #[inline(always)]
    #[must_use]
    pub fn fifoilv3(&mut self) -> FIFOILV3_W<16> {
        FIFOILV3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Interrupt Generation Level Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adfifointlr1](index.html) module"]
pub struct ADFIFOINTLR1_SPEC;
impl crate::RegisterSpec for ADFIFOINTLR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adfifointlr1::R](R) reader structure"]
impl crate::Readable for ADFIFOINTLR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adfifointlr1::W](W) writer structure"]
impl crate::Writable for ADFIFOINTLR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADFIFOINTLR1 to value 0"]
impl crate::Resettable for ADFIFOINTLR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
