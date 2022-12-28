#[doc = "Register `MDLR` reader"]
pub struct R(crate::R<MDLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDLR` writer"]
pub struct W(crate::W<MDLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDLR_SPEC>;
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
impl From<crate::W<MDLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DV0RDL` reader - Device 0 Read dummy length setting"]
pub type DV0RDL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DV0RDL` writer - Device 0 Read dummy length setting"]
pub type DV0RDL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDLR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DV0WDL` reader - Device 0 Write dummy length setting"]
pub type DV0WDL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DV0WDL` writer - Device 0 Write dummy length setting"]
pub type DV0WDL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDLR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DV1RDL` reader - Device 1 Read dummy length setting"]
pub type DV1RDL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DV1RDL` writer - Device 1 Read dummy length setting"]
pub type DV1RDL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDLR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DV1WDL` reader - Device 1 Write dummy length setting"]
pub type DV1WDL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DV1WDL` writer - Device 1 Write dummy length setting"]
pub type DV1WDL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDLR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Device 0 Read dummy length setting"]
    #[inline(always)]
    pub fn dv0rdl(&self) -> DV0RDL_R {
        DV0RDL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Device 0 Write dummy length setting"]
    #[inline(always)]
    pub fn dv0wdl(&self) -> DV0WDL_R {
        DV0WDL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Device 1 Read dummy length setting"]
    #[inline(always)]
    pub fn dv1rdl(&self) -> DV1RDL_R {
        DV1RDL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Device 1 Write dummy length setting"]
    #[inline(always)]
    pub fn dv1wdl(&self) -> DV1WDL_R {
        DV1WDL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Device 0 Read dummy length setting"]
    #[inline(always)]
    #[must_use]
    pub fn dv0rdl(&mut self) -> DV0RDL_W<0> {
        DV0RDL_W::new(self)
    }
    #[doc = "Bits 8:15 - Device 0 Write dummy length setting"]
    #[inline(always)]
    #[must_use]
    pub fn dv0wdl(&mut self) -> DV0WDL_W<8> {
        DV0WDL_W::new(self)
    }
    #[doc = "Bits 16:23 - Device 1 Read dummy length setting"]
    #[inline(always)]
    #[must_use]
    pub fn dv1rdl(&mut self) -> DV1RDL_W<16> {
        DV1RDL_W::new(self)
    }
    #[doc = "Bits 24:31 - Device 1 Write dummy length setting"]
    #[inline(always)]
    #[must_use]
    pub fn dv1wdl(&mut self) -> DV1WDL_W<24> {
        DV1WDL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory Map Dummy Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdlr](index.html) module"]
pub struct MDLR_SPEC;
impl crate::RegisterSpec for MDLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdlr::R](R) reader structure"]
impl crate::Readable for MDLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdlr::W](W) writer structure"]
impl crate::Writable for MDLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDLR to value 0"]
impl crate::Resettable for MDLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
