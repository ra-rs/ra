#[doc = "Register `GAM%s_AREA3` reader"]
pub struct R(crate::R<GAM_AREA3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAM_AREA3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAM_AREA3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAM_AREA3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAM%s_AREA3` writer"]
pub struct W(crate::W<GAM_AREA3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAM_AREA3_SPEC>;
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
impl From<crate::W<GAM_AREA3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAM_AREA3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TH09` reader - Start threshold of area 9Unsigned 10-bit integer"]
pub type TH09_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TH09` writer - Start threshold of area 9Unsigned 10-bit integer"]
pub type TH09_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GAM_AREA3_SPEC, u16, u16, 10, O>;
#[doc = "Field `TH08` reader - Start threshold of area 8Unsigned 10-bit integer"]
pub type TH08_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TH08` writer - Start threshold of area 8Unsigned 10-bit integer"]
pub type TH08_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GAM_AREA3_SPEC, u16, u16, 10, O>;
#[doc = "Field `TH07` reader - Start threshold of area 7Unsigned 10-bit integer"]
pub type TH07_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TH07` writer - Start threshold of area 7Unsigned 10-bit integer"]
pub type TH07_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GAM_AREA3_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Start threshold of area 9Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th09(&self) -> TH09_R {
        TH09_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Start threshold of area 8Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th08(&self) -> TH08_R {
        TH08_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - Start threshold of area 7Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th07(&self) -> TH07_R {
        TH07_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Start threshold of area 9Unsigned 10-bit integer"]
    #[inline(always)]
    #[must_use]
    pub fn th09(&mut self) -> TH09_W<0> {
        TH09_W::new(self)
    }
    #[doc = "Bits 10:19 - Start threshold of area 8Unsigned 10-bit integer"]
    #[inline(always)]
    #[must_use]
    pub fn th08(&mut self) -> TH08_W<10> {
        TH08_W::new(self)
    }
    #[doc = "Bits 20:29 - Start threshold of area 7Unsigned 10-bit integer"]
    #[inline(always)]
    #[must_use]
    pub fn th07(&mut self) -> TH07_W<20> {
        TH07_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Gamma %s Correction Block Area Setting Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gam_area3](index.html) module"]
pub struct GAM_AREA3_SPEC;
impl crate::RegisterSpec for GAM_AREA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gam_area3::R](R) reader structure"]
impl crate::Readable for GAM_AREA3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gam_area3::W](W) writer structure"]
impl crate::Writable for GAM_AREA3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GAM%s_AREA3 to value 0"]
impl crate::Resettable for GAM_AREA3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
