#[doc = "Register `GAM%s_AREA5` reader"]
pub struct R(crate::R<GAM_AREA5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAM_AREA5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAM_AREA5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAM_AREA5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAM%s_AREA5` writer"]
pub struct W(crate::W<GAM_AREA5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAM_AREA5_SPEC>;
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
impl From<crate::W<GAM_AREA5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAM_AREA5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TH15` reader - Start threshold of area 15Unsigned 10-bit integer"]
pub type TH15_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TH15` writer - Start threshold of area 15Unsigned 10-bit integer"]
pub type TH15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GAM_AREA5_SPEC, u16, u16, 10, O>;
#[doc = "Field `TH14` reader - Start threshold of area 14Unsigned 10-bit integer"]
pub type TH14_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TH14` writer - Start threshold of area 14Unsigned 10-bit integer"]
pub type TH14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GAM_AREA5_SPEC, u16, u16, 10, O>;
#[doc = "Field `TH13` reader - Start threshold of area 13Unsigned 10-bit integer"]
pub type TH13_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TH13` writer - Start threshold of area 13Unsigned 10-bit integer"]
pub type TH13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GAM_AREA5_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Start threshold of area 15Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th15(&self) -> TH15_R {
        TH15_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Start threshold of area 14Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th14(&self) -> TH14_R {
        TH14_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - Start threshold of area 13Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th13(&self) -> TH13_R {
        TH13_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Start threshold of area 15Unsigned 10-bit integer"]
    #[inline(always)]
    #[must_use]
    pub fn th15(&mut self) -> TH15_W<0> {
        TH15_W::new(self)
    }
    #[doc = "Bits 10:19 - Start threshold of area 14Unsigned 10-bit integer"]
    #[inline(always)]
    #[must_use]
    pub fn th14(&mut self) -> TH14_W<10> {
        TH14_W::new(self)
    }
    #[doc = "Bits 20:29 - Start threshold of area 13Unsigned 10-bit integer"]
    #[inline(always)]
    #[must_use]
    pub fn th13(&mut self) -> TH13_W<20> {
        TH13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Gamma %s Correction Block Area Setting Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gam_area5](index.html) module"]
pub struct GAM_AREA5_SPEC;
impl crate::RegisterSpec for GAM_AREA5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gam_area5::R](R) reader structure"]
impl crate::Readable for GAM_AREA5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gam_area5::W](W) writer structure"]
impl crate::Writable for GAM_AREA5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GAM%s_AREA5 to value 0"]
impl crate::Resettable for GAM_AREA5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
