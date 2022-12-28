#[doc = "Register `JIFDDLC` reader"]
pub struct R(crate::R<JIFDDLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JIFDDLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JIFDDLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JIFDDLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JIFDDLC` writer"]
pub struct W(crate::W<JIFDDLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JIFDDLC_SPEC>;
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
impl From<crate::W<JIFDDLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JIFDDLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINES` reader - Number of Input Image Lines to Be ReadThe lower three bits should be set to 0. These bits are read as0.Number of input image data lines to be read, in 8-line units."]
pub type LINES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LINES` writer - Number of Input Image Lines to Be ReadThe lower three bits should be set to 0. These bits are read as0.Number of input image data lines to be read, in 8-line units."]
pub type LINES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JIFDDLC_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Number of Input Image Lines to Be ReadThe lower three bits should be set to 0. These bits are read as0.Number of input image data lines to be read, in 8-line units."]
    #[inline(always)]
    pub fn lines(&self) -> LINES_R {
        LINES_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of Input Image Lines to Be ReadThe lower three bits should be set to 0. These bits are read as0.Number of input image data lines to be read, in 8-line units."]
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
#[doc = "JPEG Interface Decompression Destination Line Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jifddlc](index.html) module"]
pub struct JIFDDLC_SPEC;
impl crate::RegisterSpec for JIFDDLC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jifddlc::R](R) reader structure"]
impl crate::Readable for JIFDDLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jifddlc::W](W) writer structure"]
impl crate::Writable for JIFDDLC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JIFDDLC to value 0xfff8_fff8"]
impl crate::Resettable for JIFDDLC_SPEC {
    const RESET_VALUE: Self::Ux = 0xfff8_fff8;
}
