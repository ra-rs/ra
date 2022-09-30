#[doc = "Register `CTSUCRBL` reader"]
pub struct R(crate::R<CTSUCRBL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUCRBL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUCRBL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUCRBL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUCRBL` writer"]
pub struct W(crate::W<CTSUCRBL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUCRBL_SPEC>;
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
impl From<crate::W<CTSUCRBL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUCRBL_SPEC>) -> Self {
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
#[doc = "CTSU Control Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsucrbl](index.html) module"]
pub struct CTSUCRBL_SPEC;
impl crate::RegisterSpec for CTSUCRBL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctsucrbl::R](R) reader structure"]
impl crate::Readable for CTSUCRBL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsucrbl::W](W) writer structure"]
impl crate::Writable for CTSUCRBL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTSUCRBL to value 0"]
impl crate::Resettable for CTSUCRBL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
