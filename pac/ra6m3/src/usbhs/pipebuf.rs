#[doc = "Register `PIPEBUF` reader"]
pub struct R(crate::R<PIPEBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIPEBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIPEBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIPEBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIPEBUF` writer"]
pub struct W(crate::W<PIPEBUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIPEBUF_SPEC>;
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
impl From<crate::W<PIPEBUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIPEBUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUFNMB` reader - Buffer NumberThese bits specify the FIFO buffer number of the selected pipe (04h to 87h)."]
pub type BUFNMB_R = crate::FieldReader<u8, BUFNMB_A>;
#[doc = "Buffer NumberThese bits specify the FIFO buffer number of the selected pipe (04h to 87h).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BUFNMB_A(u8);
impl From<BUFNMB_A> for u8 {
    #[inline(always)]
    fn from(val: BUFNMB_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `BUFNMB` writer - Buffer NumberThese bits specify the FIFO buffer number of the selected pipe (04h to 87h)."]
pub type BUFNMB_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PIPEBUF_SPEC, u8, BUFNMB_A, 8, O>;
#[doc = "Field `BUFSIZE` reader - Buffer Size 00h: 64 bytes 01h: 128 bytes : 1Fh: 2 Kbytes"]
pub type BUFSIZE_R = crate::FieldReader<u8, BUFSIZE_A>;
#[doc = "Buffer Size 00h: 64 bytes 01h: 128 bytes : 1Fh: 2 Kbytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BUFSIZE_A(u8);
impl From<BUFSIZE_A> for u8 {
    #[inline(always)]
    fn from(val: BUFSIZE_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `BUFSIZE` writer - Buffer Size 00h: 64 bytes 01h: 128 bytes : 1Fh: 2 Kbytes"]
pub type BUFSIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, PIPEBUF_SPEC, u8, BUFSIZE_A, 5, O>;
impl R {
    #[doc = "Bits 0:7 - Buffer NumberThese bits specify the FIFO buffer number of the selected pipe (04h to 87h)."]
    #[inline(always)]
    pub fn bufnmb(&self) -> BUFNMB_R {
        BUFNMB_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 10:14 - Buffer Size 00h: 64 bytes 01h: 128 bytes : 1Fh: 2 Kbytes"]
    #[inline(always)]
    pub fn bufsize(&self) -> BUFSIZE_R {
        BUFSIZE_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Buffer NumberThese bits specify the FIFO buffer number of the selected pipe (04h to 87h)."]
    #[inline(always)]
    #[must_use]
    pub fn bufnmb(&mut self) -> BUFNMB_W<0> {
        BUFNMB_W::new(self)
    }
    #[doc = "Bits 10:14 - Buffer Size 00h: 64 bytes 01h: 128 bytes : 1Fh: 2 Kbytes"]
    #[inline(always)]
    #[must_use]
    pub fn bufsize(&mut self) -> BUFSIZE_W<10> {
        BUFSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pipe Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pipebuf](index.html) module"]
pub struct PIPEBUF_SPEC;
impl crate::RegisterSpec for PIPEBUF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pipebuf::R](R) reader structure"]
impl crate::Readable for PIPEBUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pipebuf::W](W) writer structure"]
impl crate::Writable for PIPEBUF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIPEBUF to value 0"]
impl crate::Resettable for PIPEBUF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
