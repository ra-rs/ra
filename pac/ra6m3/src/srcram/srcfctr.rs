#[doc = "Register `SRCFCTR[%s]` reader"]
pub struct R(crate::R<SRCFCTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRCFCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRCFCTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRCFCTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRCFCTR[%s]` writer"]
pub struct W(crate::W<SRCFCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRCFCTR_SPEC>;
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
impl From<crate::W<SRCFCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRCFCTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRCFCOE` reader - Stores a filter coefficient value."]
pub type SRCFCOE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SRCFCOE` writer - Stores a filter coefficient value."]
pub type SRCFCOE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRCFCTR_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bits 0:21 - Stores a filter coefficient value."]
    #[inline(always)]
    pub fn srcfcoe(&self) -> SRCFCOE_R {
        SRCFCOE_R::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:21 - Stores a filter coefficient value."]
    #[inline(always)]
    #[must_use]
    pub fn srcfcoe(&mut self) -> SRCFCOE_W<0> {
        SRCFCOE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter Coefficient Table \\[%s\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcfctr](index.html) module"]
pub struct SRCFCTR_SPEC;
impl crate::RegisterSpec for SRCFCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srcfctr::R](R) reader structure"]
impl crate::Readable for SRCFCTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srcfctr::W](W) writer structure"]
impl crate::Writable for SRCFCTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRCFCTR[%s]
to value 0"]
impl crate::Resettable for SRCFCTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
