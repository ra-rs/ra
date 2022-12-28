#[doc = "Register `GAM%s_AREA2` reader"]
pub struct R(crate::R<GAM_AREA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAM_AREA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAM_AREA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAM_AREA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAM%s_AREA2` writer"]
pub struct W(crate::W<GAM_AREA2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAM_AREA2_SPEC>;
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
impl From<crate::W<GAM_AREA2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAM_AREA2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TH06` reader - Start threshold of area 6Unsigned 10-bit integer"]
pub type TH06_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TH06` writer - Start threshold of area 6Unsigned 10-bit integer"]
pub type TH06_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GAM_AREA2_SPEC, u16, u16, 10, O>;
#[doc = "Field `TH05` reader - Start threshold of area 5Unsigned 10-bit integer"]
pub type TH05_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TH05` writer - Start threshold of area 5Unsigned 10-bit integer"]
pub type TH05_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GAM_AREA2_SPEC, u16, u16, 10, O>;
#[doc = "Field `TH04` reader - Start threshold of area 4Unsigned 10-bit integer"]
pub type TH04_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TH04` writer - Start threshold of area 4Unsigned 10-bit integer"]
pub type TH04_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GAM_AREA2_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Start threshold of area 6Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th06(&self) -> TH06_R {
        TH06_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Start threshold of area 5Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th05(&self) -> TH05_R {
        TH05_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - Start threshold of area 4Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th04(&self) -> TH04_R {
        TH04_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Start threshold of area 6Unsigned 10-bit integer"]
    #[inline(always)]
    #[must_use]
    pub fn th06(&mut self) -> TH06_W<0> {
        TH06_W::new(self)
    }
    #[doc = "Bits 10:19 - Start threshold of area 5Unsigned 10-bit integer"]
    #[inline(always)]
    #[must_use]
    pub fn th05(&mut self) -> TH05_W<10> {
        TH05_W::new(self)
    }
    #[doc = "Bits 20:29 - Start threshold of area 4Unsigned 10-bit integer"]
    #[inline(always)]
    #[must_use]
    pub fn th04(&mut self) -> TH04_W<20> {
        TH04_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Gamma %s Correction Block Area Setting Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gam_area2](index.html) module"]
pub struct GAM_AREA2_SPEC;
impl crate::RegisterSpec for GAM_AREA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gam_area2::R](R) reader structure"]
impl crate::Readable for GAM_AREA2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gam_area2::W](W) writer structure"]
impl crate::Writable for GAM_AREA2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GAM%s_AREA2 to value 0"]
impl crate::Resettable for GAM_AREA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
