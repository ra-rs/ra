#[doc = "Register `NTDTBP0_BY` reader"]
pub struct R(crate::R<NTDTBP0_BY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NTDTBP0_BY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NTDTBP0_BY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NTDTBP0_BY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NTDTBP0_BY` writer"]
pub struct W(crate::W<NTDTBP0_BY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NTDTBP0_BY_SPEC>;
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
impl From<crate::W<NTDTBP0_BY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NTDTBP0_BY_SPEC>) -> Self {
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
#[doc = "Normal Transfer Data Buffer Port Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ntdtbp0_by](index.html) module"]
pub struct NTDTBP0_BY_SPEC;
impl crate::RegisterSpec for NTDTBP0_BY_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ntdtbp0_by::R](R) reader structure"]
impl crate::Readable for NTDTBP0_BY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ntdtbp0_by::W](W) writer structure"]
impl crate::Writable for NTDTBP0_BY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NTDTBP0_BY to value 0"]
impl crate::Resettable for NTDTBP0_BY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
