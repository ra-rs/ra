#[doc = "Register `CTSUCHTRCBL` reader"]
pub struct R(crate::R<CTSUCHTRCBL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUCHTRCBL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUCHTRCBL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUCHTRCBL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUCHTRCBL` writer"]
pub struct W(crate::W<CTSUCHTRCBL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUCHTRCBL_SPEC>;
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
impl From<crate::W<CTSUCHTRCBL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUCHTRCBL_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU Channel Transmit/Receive Control Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsuchtrcbl](index.html) module"]
pub struct CTSUCHTRCBL_SPEC;
impl crate::RegisterSpec for CTSUCHTRCBL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctsuchtrcbl::R](R) reader structure"]
impl crate::Readable for CTSUCHTRCBL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsuchtrcbl::W](W) writer structure"]
impl crate::Writable for CTSUCHTRCBL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTSUCHTRCBL to value 0"]
impl crate::Resettable for CTSUCHTRCBL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
