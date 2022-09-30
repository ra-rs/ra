#[doc = "Register `CRCDIR_BY` reader"]
pub struct R(crate::R<CRCDIR_BY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCDIR_BY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCDIR_BY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCDIR_BY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCDIR_BY` writer"]
pub struct W(crate::W<CRCDIR_BY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCDIR_BY_SPEC>;
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
impl From<crate::W<CRCDIR_BY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCDIR_BY_SPEC>) -> Self {
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
#[doc = "CRC Data Input Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcdir_by](index.html) module"]
pub struct CRCDIR_BY_SPEC;
impl crate::RegisterSpec for CRCDIR_BY_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [crcdir_by::R](R) reader structure"]
impl crate::Readable for CRCDIR_BY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crcdir_by::W](W) writer structure"]
impl crate::Writable for CRCDIR_BY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRCDIR_BY to value 0"]
impl crate::Resettable for CRCDIR_BY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
