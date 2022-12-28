#[doc = "Register `JINTS1` reader"]
pub struct R(crate::R<JINTS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JINTS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JINTS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JINTS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JINTS1` writer"]
pub struct W(crate::W<JINTS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JINTS1_SPEC>;
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
impl From<crate::W<JINTS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JINTS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUTLF` reader - In decompression, this bit is set to 1 when the number of lines of output image data indicated by JIFDDLC have been written. This bit is only valid when the DOUTLC bit in JIFDCNT is set to 1.\n\nThe field is **modified** in some way after a read operation."]
pub type DOUTLF_R = crate::BitReader<bool>;
#[doc = "Field `DOUTLF` writer - In decompression, this bit is set to 1 when the number of lines of output image data indicated by JIFDDLC have been written. This bit is only valid when the DOUTLC bit in JIFDCNT is set to 1."]
pub type DOUTLF_W<'a, const O: u8> = crate::BitWriter<'a, u32, JINTS1_SPEC, bool, O>;
#[doc = "Field `JINF` reader - This bit is set to 1 when the amount of input coded data indicated by JIFDSDC is read in decompression. This bit is valid only when the JINC bit in JIFDCNT is set to 1.\n\nThe field is **modified** in some way after a read operation."]
pub type JINF_R = crate::BitReader<bool>;
#[doc = "Field `JINF` writer - This bit is set to 1 when the amount of input coded data indicated by JIFDSDC is read in decompression. This bit is valid only when the JINC bit in JIFDCNT is set to 1."]
pub type JINF_W<'a, const O: u8> = crate::BitWriter<'a, u32, JINTS1_SPEC, bool, O>;
#[doc = "Field `DBTF` reader - This bit is set to 1 when the last output image data is written in decompression.\n\nThe field is **modified** in some way after a read operation."]
pub type DBTF_R = crate::BitReader<bool>;
#[doc = "Field `DBTF` writer - This bit is set to 1 when the last output image data is written in decompression."]
pub type DBTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, JINTS1_SPEC, bool, O>;
#[doc = "Field `DINLF` reader - This bit is set to 1 when the number of input image data lines indicated by JIFESLC is read in compression. This bit is valid only when the DINLC bit in JIFECNT is set to 1.\n\nThe field is **modified** in some way after a read operation."]
pub type DINLF_R = crate::BitReader<bool>;
#[doc = "Field `DINLF` writer - This bit is set to 1 when the number of input image data lines indicated by JIFESLC is read in compression. This bit is valid only when the DINLC bit in JIFECNT is set to 1."]
pub type DINLF_W<'a, const O: u8> = crate::BitWriter<'a, u32, JINTS1_SPEC, bool, O>;
#[doc = "Field `CBTF` reader - This bit is set to 1 when the last output coded data is written in compression.\n\nThe field is **modified** in some way after a read operation."]
pub type CBTF_R = crate::BitReader<bool>;
#[doc = "Field `CBTF` writer - This bit is set to 1 when the last output coded data is written in compression."]
pub type CBTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, JINTS1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - In decompression, this bit is set to 1 when the number of lines of output image data indicated by JIFDDLC have been written. This bit is only valid when the DOUTLC bit in JIFDCNT is set to 1."]
    #[inline(always)]
    pub fn doutlf(&self) -> DOUTLF_R {
        DOUTLF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is set to 1 when the amount of input coded data indicated by JIFDSDC is read in decompression. This bit is valid only when the JINC bit in JIFDCNT is set to 1."]
    #[inline(always)]
    pub fn jinf(&self) -> JINF_R {
        JINF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is set to 1 when the last output image data is written in decompression."]
    #[inline(always)]
    pub fn dbtf(&self) -> DBTF_R {
        DBTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit is set to 1 when the number of input image data lines indicated by JIFESLC is read in compression. This bit is valid only when the DINLC bit in JIFECNT is set to 1."]
    #[inline(always)]
    pub fn dinlf(&self) -> DINLF_R {
        DINLF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit is set to 1 when the last output coded data is written in compression."]
    #[inline(always)]
    pub fn cbtf(&self) -> CBTF_R {
        CBTF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - In decompression, this bit is set to 1 when the number of lines of output image data indicated by JIFDDLC have been written. This bit is only valid when the DOUTLC bit in JIFDCNT is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn doutlf(&mut self) -> DOUTLF_W<0> {
        DOUTLF_W::new(self)
    }
    #[doc = "Bit 1 - This bit is set to 1 when the amount of input coded data indicated by JIFDSDC is read in decompression. This bit is valid only when the JINC bit in JIFDCNT is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn jinf(&mut self) -> JINF_W<1> {
        JINF_W::new(self)
    }
    #[doc = "Bit 2 - This bit is set to 1 when the last output image data is written in decompression."]
    #[inline(always)]
    #[must_use]
    pub fn dbtf(&mut self) -> DBTF_W<2> {
        DBTF_W::new(self)
    }
    #[doc = "Bit 5 - This bit is set to 1 when the number of input image data lines indicated by JIFESLC is read in compression. This bit is valid only when the DINLC bit in JIFECNT is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn dinlf(&mut self) -> DINLF_W<5> {
        DINLF_W::new(self)
    }
    #[doc = "Bit 6 - This bit is set to 1 when the last output coded data is written in compression."]
    #[inline(always)]
    #[must_use]
    pub fn cbtf(&mut self) -> CBTF_W<6> {
        CBTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG Interrupt Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jints1](index.html) module"]
pub struct JINTS1_SPEC;
impl crate::RegisterSpec for JINTS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jints1::R](R) reader structure"]
impl crate::Readable for JINTS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jints1::W](W) writer structure"]
impl crate::Writable for JINTS1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JINTS1 to value 0"]
impl crate::Resettable for JINTS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
