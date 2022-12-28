#[doc = "Register `ADFIFOINTLR3` reader"]
pub struct R(crate::R<ADFIFOINTLR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADFIFOINTLR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADFIFOINTLR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADFIFOINTLR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADFIFOINTLR3` writer"]
pub struct W(crate::W<ADFIFOINTLR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADFIFOINTLR3_SPEC>;
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
impl From<crate::W<ADFIFOINTLR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADFIFOINTLR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOILV6` reader - Scan Group 6 FIFO Interrupt Output Timing Setting"]
pub type FIFOILV6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFOILV6` writer - Scan Group 6 FIFO Interrupt Output Timing Setting"]
pub type FIFOILV6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADFIFOINTLR3_SPEC, u8, u8, 4, O>;
#[doc = "Field `FIFOILV7` reader - Scan Group 7 FIFO Interrupt Output Timing Setting"]
pub type FIFOILV7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFOILV7` writer - Scan Group 7 FIFO Interrupt Output Timing Setting"]
pub type FIFOILV7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADFIFOINTLR3_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Scan Group 6 FIFO Interrupt Output Timing Setting"]
    #[inline(always)]
    pub fn fifoilv6(&self) -> FIFOILV6_R {
        FIFOILV6_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Scan Group 7 FIFO Interrupt Output Timing Setting"]
    #[inline(always)]
    pub fn fifoilv7(&self) -> FIFOILV7_R {
        FIFOILV7_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Scan Group 6 FIFO Interrupt Output Timing Setting"]
    #[inline(always)]
    #[must_use]
    pub fn fifoilv6(&mut self) -> FIFOILV6_W<0> {
        FIFOILV6_W::new(self)
    }
    #[doc = "Bits 16:19 - Scan Group 7 FIFO Interrupt Output Timing Setting"]
    #[inline(always)]
    #[must_use]
    pub fn fifoilv7(&mut self) -> FIFOILV7_W<16> {
        FIFOILV7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Interrupt Generation Level Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adfifointlr3](index.html) module"]
pub struct ADFIFOINTLR3_SPEC;
impl crate::RegisterSpec for ADFIFOINTLR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adfifointlr3::R](R) reader structure"]
impl crate::Readable for ADFIFOINTLR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adfifointlr3::W](W) writer structure"]
impl crate::Writable for ADFIFOINTLR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADFIFOINTLR3 to value 0"]
impl crate::Resettable for ADFIFOINTLR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
