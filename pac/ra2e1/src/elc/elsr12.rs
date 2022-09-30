#[doc = "Register `ELSR12` reader"]
pub struct R(crate::R<ELSR12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ELSR12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ELSR12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ELSR12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ELSR12` writer"]
pub struct W(crate::W<ELSR12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ELSR12_SPEC>;
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
impl From<crate::W<ELSR12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ELSR12_SPEC>) -> Self {
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
#[doc = "Event Link Setting Register 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [elsr12](index.html) module"]
pub struct ELSR12_SPEC;
impl crate::RegisterSpec for ELSR12_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [elsr12::R](R) reader structure"]
impl crate::Readable for ELSR12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [elsr12::W](W) writer structure"]
impl crate::Writable for ELSR12_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ELSR12 to value 0"]
impl crate::Resettable for ELSR12_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
