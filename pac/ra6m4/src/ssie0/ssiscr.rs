#[doc = "Register `SSISCR` reader"]
pub struct R(crate::R<SSISCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSISCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSISCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSISCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSISCR` writer"]
pub struct W(crate::W<SSISCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSISCR_SPEC>;
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
impl From<crate::W<SSISCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSISCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDFS` reader - RDF Setting Condition Select"]
pub type RDFS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDFS` writer - RDF Setting Condition Select"]
pub type RDFS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSISCR_SPEC, u8, u8, 5, O>;
#[doc = "Field `TDES` reader - TDE Setting Condition Select"]
pub type TDES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TDES` writer - TDE Setting Condition Select"]
pub type TDES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSISCR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - RDF Setting Condition Select"]
    #[inline(always)]
    pub fn rdfs(&self) -> RDFS_R {
        RDFS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - TDE Setting Condition Select"]
    #[inline(always)]
    pub fn tdes(&self) -> TDES_R {
        TDES_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - RDF Setting Condition Select"]
    #[inline(always)]
    #[must_use]
    pub fn rdfs(&mut self) -> RDFS_W<0> {
        RDFS_W::new(self)
    }
    #[doc = "Bits 8:12 - TDE Setting Condition Select"]
    #[inline(always)]
    #[must_use]
    pub fn tdes(&mut self) -> TDES_W<8> {
        TDES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssiscr](index.html) module"]
pub struct SSISCR_SPEC;
impl crate::RegisterSpec for SSISCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssiscr::R](R) reader structure"]
impl crate::Readable for SSISCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssiscr::W](W) writer structure"]
impl crate::Writable for SSISCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSISCR to value 0"]
impl crate::Resettable for SSISCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
