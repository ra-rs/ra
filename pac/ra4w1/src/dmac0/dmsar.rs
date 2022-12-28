#[doc = "Register `DMSAR` reader"]
pub struct R(crate::R<DMSAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMSAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMSAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMSAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMSAR` writer"]
pub struct W(crate::W<DMSAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMSAR_SPEC>;
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
impl From<crate::W<DMSAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMSAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMSAR` reader - Specifies the transfer source start address."]
pub type DMSAR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DMSAR` writer - Specifies the transfer source start address."]
pub type DMSAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMSAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Specifies the transfer source start address."]
    #[inline(always)]
    pub fn dmsar(&self) -> DMSAR_R {
        DMSAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies the transfer source start address."]
    #[inline(always)]
    #[must_use]
    pub fn dmsar(&mut self) -> DMSAR_W<0> {
        DMSAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Source Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmsar](index.html) module"]
pub struct DMSAR_SPEC;
impl crate::RegisterSpec for DMSAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmsar::R](R) reader structure"]
impl crate::Readable for DMSAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmsar::W](W) writer structure"]
impl crate::Writable for DMSAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMSAR to value 0"]
impl crate::Resettable for DMSAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
