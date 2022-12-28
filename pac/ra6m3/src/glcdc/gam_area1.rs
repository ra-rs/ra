#[doc = "Register `GAM%s_AREA1` reader"]
pub struct R(crate::R<GAM_AREA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAM_AREA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAM_AREA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAM_AREA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAM%s_AREA1` writer"]
pub struct W(crate::W<GAM_AREA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAM_AREA1_SPEC>;
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
impl From<crate::W<GAM_AREA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAM_AREA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TH03` reader - Start threshold of area 3Unsigned 10-bit integer"]
pub type TH03_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TH03` writer - Start threshold of area 3Unsigned 10-bit integer"]
pub type TH03_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GAM_AREA1_SPEC, u16, u16, 10, O>;
#[doc = "Field `TH02` reader - Start threshold of area 2Unsigned 10-bit integer"]
pub type TH02_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TH02` writer - Start threshold of area 2Unsigned 10-bit integer"]
pub type TH02_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GAM_AREA1_SPEC, u16, u16, 10, O>;
#[doc = "Field `TH01` reader - Start threshold of area 1Unsigned 10-bit integer"]
pub type TH01_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TH01` writer - Start threshold of area 1Unsigned 10-bit integer"]
pub type TH01_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GAM_AREA1_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Start threshold of area 3Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th03(&self) -> TH03_R {
        TH03_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Start threshold of area 2Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th02(&self) -> TH02_R {
        TH02_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - Start threshold of area 1Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th01(&self) -> TH01_R {
        TH01_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Start threshold of area 3Unsigned 10-bit integer"]
    #[inline(always)]
    #[must_use]
    pub fn th03(&mut self) -> TH03_W<0> {
        TH03_W::new(self)
    }
    #[doc = "Bits 10:19 - Start threshold of area 2Unsigned 10-bit integer"]
    #[inline(always)]
    #[must_use]
    pub fn th02(&mut self) -> TH02_W<10> {
        TH02_W::new(self)
    }
    #[doc = "Bits 20:29 - Start threshold of area 1Unsigned 10-bit integer"]
    #[inline(always)]
    #[must_use]
    pub fn th01(&mut self) -> TH01_W<20> {
        TH01_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Gamma %s Correction Block Area Setting Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gam_area1](index.html) module"]
pub struct GAM_AREA1_SPEC;
impl crate::RegisterSpec for GAM_AREA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gam_area1::R](R) reader structure"]
impl crate::Readable for GAM_AREA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gam_area1::W](W) writer structure"]
impl crate::Writable for GAM_AREA1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GAM%s_AREA1 to value 0"]
impl crate::Resettable for GAM_AREA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
