#[doc = "Register `AGT` reader"]
pub struct R(crate::R<AGT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AGT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AGT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AGT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AGT` writer"]
pub struct W(crate::W<AGT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AGT_SPEC>;
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
impl From<crate::W<AGT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AGT_SPEC>) -> Self {
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
#[doc = "AGT Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [agt](index.html) module"]
pub struct AGT_SPEC;
impl crate::RegisterSpec for AGT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [agt::R](R) reader structure"]
impl crate::Readable for AGT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [agt::W](W) writer structure"]
impl crate::Writable for AGT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AGT to value 0xffff"]
impl crate::Resettable for AGT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
