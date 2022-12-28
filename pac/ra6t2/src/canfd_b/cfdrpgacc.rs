#[doc = "Register `CFDRPGACC%s` reader"]
pub struct R(crate::R<CFDRPGACC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDRPGACC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDRPGACC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDRPGACC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDRPGACC%s` writer"]
pub struct W(crate::W<CFDRPGACC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDRPGACC_SPEC>;
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
impl From<crate::W<CFDRPGACC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDRPGACC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDTA` reader - RAM Data Test Access"]
pub type RDTA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RDTA` writer - RAM Data Test Access"]
pub type RDTA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDRPGACC_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - RAM Data Test Access"]
    #[inline(always)]
    pub fn rdta(&self) -> RDTA_R {
        RDTA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RAM Data Test Access"]
    #[inline(always)]
    #[must_use]
    pub fn rdta(&mut self) -> RDTA_W<0> {
        RDTA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAM Test Page Access Registers %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdrpgacc](index.html) module"]
pub struct CFDRPGACC_SPEC;
impl crate::RegisterSpec for CFDRPGACC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdrpgacc::R](R) reader structure"]
impl crate::Readable for CFDRPGACC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdrpgacc::W](W) writer structure"]
impl crate::Writable for CFDRPGACC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDRPGACC%s to value 0"]
impl crate::Resettable for CFDRPGACC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
