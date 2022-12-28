#[doc = "Register `RSTOUTR` reader"]
pub struct R(crate::R<RSTOUTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTOUTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTOUTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTOUTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTOUTR` writer"]
pub struct W(crate::W<RSTOUTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTOUTR_SPEC>;
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
impl From<crate::W<RSTOUTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTOUTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSTOUTR` reader - Response Message Reception Timeout Time SettingA response message not being received within n x 1024 (ns), where n is the setting, is judged to represent a timeout."]
pub type RSTOUTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RSTOUTR` writer - Response Message Reception Timeout Time SettingA response message not being received within n x 1024 (ns), where n is the setting, is judged to represent a timeout."]
pub type RSTOUTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RSTOUTR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Response Message Reception Timeout Time SettingA response message not being received within n x 1024 (ns), where n is the setting, is judged to represent a timeout."]
    #[inline(always)]
    pub fn rstoutr(&self) -> RSTOUTR_R {
        RSTOUTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Response Message Reception Timeout Time SettingA response message not being received within n x 1024 (ns), where n is the setting, is judged to represent a timeout."]
    #[inline(always)]
    #[must_use]
    pub fn rstoutr(&mut self) -> RSTOUTR_W<0> {
        RSTOUTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Response Message Reception Timeout Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstoutr](index.html) module"]
pub struct RSTOUTR_SPEC;
impl crate::RegisterSpec for RSTOUTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstoutr::R](R) reader structure"]
impl crate::Readable for RSTOUTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstoutr::W](W) writer structure"]
impl crate::Writable for RSTOUTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSTOUTR to value 0"]
impl crate::Resettable for RSTOUTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
