#[doc = "Register `CFDCFPTR` reader"]
pub struct R(crate::R<CFDCFPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDCFPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDCFPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDCFPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDCFPTR` writer"]
pub struct W(crate::W<CFDCFPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDCFPTR_SPEC>;
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
impl From<crate::W<CFDCFPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDCFPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFTS` reader - Common FIFO Timestamp Value"]
pub type CFTS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CFTS` writer - Common FIFO Timestamp Value"]
pub type CFTS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDCFPTR_SPEC, u16, u16, 16, O>;
#[doc = "Field `CFDLC` reader - Common FIFO Buffer DLC Field"]
pub type CFDLC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFDLC` writer - Common FIFO Buffer DLC Field"]
pub type CFDLC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDCFPTR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:15 - Common FIFO Timestamp Value"]
    #[inline(always)]
    pub fn cfts(&self) -> CFTS_R {
        CFTS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - Common FIFO Buffer DLC Field"]
    #[inline(always)]
    pub fn cfdlc(&self) -> CFDLC_R {
        CFDLC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Common FIFO Timestamp Value"]
    #[inline(always)]
    #[must_use]
    pub fn cfts(&mut self) -> CFTS_W<0> {
        CFTS_W::new(self)
    }
    #[doc = "Bits 28:31 - Common FIFO Buffer DLC Field"]
    #[inline(always)]
    #[must_use]
    pub fn cfdlc(&mut self) -> CFDLC_W<28> {
        CFDLC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common FIFO Access Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdcfptr](index.html) module"]
pub struct CFDCFPTR_SPEC;
impl crate::RegisterSpec for CFDCFPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdcfptr::R](R) reader structure"]
impl crate::Readable for CFDCFPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdcfptr::W](W) writer structure"]
impl crate::Writable for CFDCFPTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDCFPTR to value 0"]
impl crate::Resettable for CFDCFPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
