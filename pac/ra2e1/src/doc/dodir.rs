#[doc = "Register `DODIR` reader"]
pub struct R(crate::R<DODIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DODIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DODIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DODIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DODIR` writer"]
pub struct W(crate::W<DODIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DODIR_SPEC>;
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
impl From<crate::W<DODIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DODIR_SPEC>) -> Self {
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
#[doc = "DOC Data Input Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dodir](index.html) module"]
pub struct DODIR_SPEC;
impl crate::RegisterSpec for DODIR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dodir::R](R) reader structure"]
impl crate::Readable for DODIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dodir::W](W) writer structure"]
impl crate::Writable for DODIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DODIR to value 0"]
impl crate::Resettable for DODIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
