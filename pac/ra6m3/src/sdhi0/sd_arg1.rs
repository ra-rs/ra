#[doc = "Register `SD_ARG1` reader"]
pub struct R(crate::R<SD_ARG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_ARG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_ARG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_ARG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SD_ARG1` writer"]
pub struct W(crate::W<SD_ARG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_ARG1_SPEC>;
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
impl From<crate::W<SD_ARG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_ARG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SD_ARG1` reader - Argument Register 1Set command format\\[39:24\\]
(argument)"]
pub type SD_ARG1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SD_ARG1` writer - Argument Register 1Set command format\\[39:24\\]
(argument)"]
pub type SD_ARG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SD_ARG1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Argument Register 1Set command format\\[39:24\\]
(argument)"]
    #[inline(always)]
    pub fn sd_arg1(&self) -> SD_ARG1_R {
        SD_ARG1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Argument Register 1Set command format\\[39:24\\]
(argument)"]
    #[inline(always)]
    #[must_use]
    pub fn sd_arg1(&mut self) -> SD_ARG1_W<0> {
        SD_ARG1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD Command Argument Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_arg1](index.html) module"]
pub struct SD_ARG1_SPEC;
impl crate::RegisterSpec for SD_ARG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_arg1::R](R) reader structure"]
impl crate::Readable for SD_ARG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_arg1::W](W) writer structure"]
impl crate::Writable for SD_ARG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SD_ARG1 to value 0"]
impl crate::Resettable for SD_ARG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
