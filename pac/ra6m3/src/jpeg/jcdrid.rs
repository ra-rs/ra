#[doc = "Register `JCDRID` reader"]
pub struct R(crate::R<JCDRID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JCDRID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JCDRID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JCDRID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JCDRID` writer"]
pub struct W(crate::W<JCDRID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JCDRID_SPEC>;
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
impl From<crate::W<JCDRID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JCDRID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DRID` reader - Lower Bytes of MCUs Preceding RST MarkerWhen both upper and lower bytes are set to 00h, neither a DRI nor an RST marker is placed.NOTE: Read-only in Decompression."]
pub type DRID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRID` writer - Lower Bytes of MCUs Preceding RST MarkerWhen both upper and lower bytes are set to 00h, neither a DRI nor an RST marker is placed.NOTE: Read-only in Decompression."]
pub type DRID_W<'a, const O: u8> = crate::FieldWriter<'a, u8, JCDRID_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Lower Bytes of MCUs Preceding RST MarkerWhen both upper and lower bytes are set to 00h, neither a DRI nor an RST marker is placed.NOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn drid(&self) -> DRID_R {
        DRID_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Lower Bytes of MCUs Preceding RST MarkerWhen both upper and lower bytes are set to 00h, neither a DRI nor an RST marker is placed.NOTE: Read-only in Decompression."]
    #[inline(always)]
    #[must_use]
    pub fn drid(&mut self) -> DRID_W<0> {
        DRID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG Code DRI Lower Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jcdrid](index.html) module"]
pub struct JCDRID_SPEC;
impl crate::RegisterSpec for JCDRID_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [jcdrid::R](R) reader structure"]
impl crate::Readable for JCDRID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jcdrid::W](W) writer structure"]
impl crate::Writable for JCDRID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JCDRID to value 0"]
impl crate::Resettable for JCDRID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
