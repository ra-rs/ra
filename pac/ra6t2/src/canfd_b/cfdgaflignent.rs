#[doc = "Register `CFDGAFLIGNENT` reader"]
pub struct R(crate::R<CFDGAFLIGNENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDGAFLIGNENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDGAFLIGNENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDGAFLIGNENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDGAFLIGNENT` writer"]
pub struct W(crate::W<CFDGAFLIGNENT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDGAFLIGNENT_SPEC>;
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
impl From<crate::W<CFDGAFLIGNENT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDGAFLIGNENT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRN` reader - Ignore Rule Number"]
pub type IRN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRN` writer - Ignore Rule Number"]
pub type IRN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDGAFLIGNENT_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Ignore Rule Number"]
    #[inline(always)]
    pub fn irn(&self) -> IRN_R {
        IRN_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Ignore Rule Number"]
    #[inline(always)]
    #[must_use]
    pub fn irn(&mut self) -> IRN_W<0> {
        IRN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global AFL Ignore Entry Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdgaflignent](index.html) module"]
pub struct CFDGAFLIGNENT_SPEC;
impl crate::RegisterSpec for CFDGAFLIGNENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdgaflignent::R](R) reader structure"]
impl crate::Readable for CFDGAFLIGNENT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdgaflignent::W](W) writer structure"]
impl crate::Writable for CFDGAFLIGNENT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDGAFLIGNENT to value 0"]
impl crate::Resettable for CFDGAFLIGNENT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
