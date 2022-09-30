#[doc = "Register `CTSUCHTRCAH` reader"]
pub struct R(crate::R<CTSUCHTRCAH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUCHTRCAH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUCHTRCAH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUCHTRCAH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUCHTRCAH` writer"]
pub struct W(crate::W<CTSUCHTRCAH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUCHTRCAH_SPEC>;
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
impl From<crate::W<CTSUCHTRCAH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUCHTRCAH_SPEC>) -> Self {
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
#[doc = "CTSU Channel Transmit/Receive Control Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsuchtrcah](index.html) module"]
pub struct CTSUCHTRCAH_SPEC;
impl crate::RegisterSpec for CTSUCHTRCAH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctsuchtrcah::R](R) reader structure"]
impl crate::Readable for CTSUCHTRCAH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsuchtrcah::W](W) writer structure"]
impl crate::Writable for CTSUCHTRCAH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTSUCHTRCAH to value 0"]
impl crate::Resettable for CTSUCHTRCAH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
