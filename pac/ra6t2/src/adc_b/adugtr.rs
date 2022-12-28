#[doc = "Register `ADUGTR%s` reader"]
pub struct R(crate::R<ADUGTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADUGTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADUGTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADUGTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADUGTR%s` writer"]
pub struct W(crate::W<ADUGTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADUGTR_SPEC>;
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
impl From<crate::W<ADUGTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADUGTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UGAIN` reader - User Gain Table n"]
pub type UGAIN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `UGAIN` writer - User Gain Table n"]
pub type UGAIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADUGTR_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - User Gain Table n"]
    #[inline(always)]
    pub fn ugain(&self) -> UGAIN_R {
        UGAIN_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - User Gain Table n"]
    #[inline(always)]
    #[must_use]
    pub fn ugain(&mut self) -> UGAIN_W<0> {
        UGAIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "User Gain Table Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adugtr](index.html) module"]
pub struct ADUGTR_SPEC;
impl crate::RegisterSpec for ADUGTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adugtr::R](R) reader structure"]
impl crate::Readable for ADUGTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adugtr::W](W) writer structure"]
impl crate::Writable for ADUGTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADUGTR%s to value 0x0040_0000"]
impl crate::Resettable for ADUGTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_0000;
}
