#[doc = "Register `CTSUCHACBL` reader"]
pub struct R(crate::R<CTSUCHACBL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUCHACBL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUCHACBL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUCHACBL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUCHACBL` writer"]
pub struct W(crate::W<CTSUCHACBL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUCHACBL_SPEC>;
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
impl From<crate::W<CTSUCHACBL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUCHACBL_SPEC>) -> Self {
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
#[doc = "CTSU Channel Enable Control Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsuchacbl](index.html) module"]
pub struct CTSUCHACBL_SPEC;
impl crate::RegisterSpec for CTSUCHACBL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctsuchacbl::R](R) reader structure"]
impl crate::Readable for CTSUCHACBL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsuchacbl::W](W) writer structure"]
impl crate::Writable for CTSUCHACBL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTSUCHACBL to value 0"]
impl crate::Resettable for CTSUCHACBL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
