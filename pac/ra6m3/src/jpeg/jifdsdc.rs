#[doc = "Register `JIFDSDC` reader"]
pub struct R(crate::R<JIFDSDC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JIFDSDC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JIFDSDC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JIFDSDC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JIFDSDC` writer"]
pub struct W(crate::W<JIFDSDC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JIFDSDC_SPEC>;
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
impl From<crate::W<JIFDSDC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JIFDSDC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JDATAS` reader - Amount of Input Coded Data to be Read (in 8-byte units) The lower three bits should be set to 0."]
pub type JDATAS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `JDATAS` writer - Amount of Input Coded Data to be Read (in 8-byte units) The lower three bits should be set to 0."]
pub type JDATAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JIFDSDC_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Amount of Input Coded Data to be Read (in 8-byte units) The lower three bits should be set to 0."]
    #[inline(always)]
    pub fn jdatas(&self) -> JDATAS_R {
        JDATAS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Amount of Input Coded Data to be Read (in 8-byte units) The lower three bits should be set to 0."]
    #[inline(always)]
    #[must_use]
    pub fn jdatas(&mut self) -> JDATAS_W<0> {
        JDATAS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG Interface Decompression Source Data Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jifdsdc](index.html) module"]
pub struct JIFDSDC_SPEC;
impl crate::RegisterSpec for JIFDSDC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jifdsdc::R](R) reader structure"]
impl crate::Readable for JIFDSDC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jifdsdc::W](W) writer structure"]
impl crate::Writable for JIFDSDC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JIFDSDC to value 0xfff8_fff8"]
impl crate::Resettable for JIFDSDC_SPEC {
    const RESET_VALUE: Self::Ux = 0xfff8_fff8;
}
