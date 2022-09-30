#[doc = "Register `ELSR8` reader"]
pub struct R(crate::R<ELSR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ELSR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ELSR8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ELSR8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ELSR8` writer"]
pub struct W(crate::W<ELSR8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ELSR8_SPEC>;
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
impl From<crate::W<ELSR8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ELSR8_SPEC>) -> Self {
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
#[doc = "Event Link Setting Register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [elsr8](index.html) module"]
pub struct ELSR8_SPEC;
impl crate::RegisterSpec for ELSR8_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [elsr8::R](R) reader structure"]
impl crate::Readable for ELSR8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [elsr8::W](W) writer structure"]
impl crate::Writable for ELSR8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ELSR8 to value 0"]
impl crate::Resettable for ELSR8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
