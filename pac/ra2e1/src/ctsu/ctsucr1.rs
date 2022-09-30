#[doc = "Register `CTSUCR1` reader"]
pub struct R(crate::R<CTSUCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUCR1` writer"]
pub struct W(crate::W<CTSUCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUCR1_SPEC>;
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
impl From<crate::W<CTSUCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUCR1_SPEC>) -> Self {
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
#[doc = "CTSU Control Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsucr1](index.html) module"]
pub struct CTSUCR1_SPEC;
impl crate::RegisterSpec for CTSUCR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctsucr1::R](R) reader structure"]
impl crate::Readable for CTSUCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsucr1::W](W) writer structure"]
impl crate::Writable for CTSUCR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTSUCR1 to value 0"]
impl crate::Resettable for CTSUCR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
