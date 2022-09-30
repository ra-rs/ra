#[doc = "Register `ICDRT` reader"]
pub struct R(crate::R<ICDRT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICDRT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICDRT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICDRT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICDRT` writer"]
pub struct W(crate::W<ICDRT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICDRT_SPEC>;
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
impl From<crate::W<ICDRT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICDRT_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Bus Transmit Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icdrt](index.html) module"]
pub struct ICDRT_SPEC;
impl crate::RegisterSpec for ICDRT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [icdrt::R](R) reader structure"]
impl crate::Readable for ICDRT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icdrt::W](W) writer structure"]
impl crate::Writable for ICDRT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICDRT to value 0xff"]
impl crate::Resettable for ICDRT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
