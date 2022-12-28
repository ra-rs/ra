#[doc = "Register `JIFESLC` reader"]
pub struct R(crate::R<JIFESLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JIFESLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JIFESLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JIFESLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JIFESLC` writer"]
pub struct W(crate::W<JIFESLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JIFESLC_SPEC>;
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
impl From<crate::W<JIFESLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JIFESLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINES` reader - Number of Input Image Data Lines to be Read (in 8-line units) The lower three bits should be set to 0."]
pub type LINES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LINES` writer - Number of Input Image Data Lines to be Read (in 8-line units) The lower three bits should be set to 0."]
pub type LINES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JIFESLC_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Number of Input Image Data Lines to be Read (in 8-line units) The lower three bits should be set to 0."]
    #[inline(always)]
    pub fn lines(&self) -> LINES_R {
        LINES_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of Input Image Data Lines to be Read (in 8-line units) The lower three bits should be set to 0."]
    #[inline(always)]
    #[must_use]
    pub fn lines(&mut self) -> LINES_W<0> {
        LINES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG Interface Compression Source Line Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jifeslc](index.html) module"]
pub struct JIFESLC_SPEC;
impl crate::RegisterSpec for JIFESLC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jifeslc::R](R) reader structure"]
impl crate::Readable for JIFESLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jifeslc::W](W) writer structure"]
impl crate::Writable for JIFESLC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JIFESLC to value 0xfff8_fff8"]
impl crate::Resettable for JIFESLC_SPEC {
    const RESET_VALUE: Self::Ux = 0xfff8_fff8;
}
