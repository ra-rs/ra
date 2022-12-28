#[doc = "Register `CS%sWCR2` reader"]
pub struct R(crate::R<CSWCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSWCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSWCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSWCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CS%sWCR2` writer"]
pub struct W(crate::W<CSWCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSWCR2_SPEC>;
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
impl From<crate::W<CSWCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSWCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSROFF` reader - Read-Access CS Extension Cycle Select"]
pub type CSROFF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSROFF` writer - Read-Access CS Extension Cycle Select"]
pub type CSROFF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSWCR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `CSWOFF` reader - Write-Access CS Extension Cycle Select"]
pub type CSWOFF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSWOFF` writer - Write-Access CS Extension Cycle Select"]
pub type CSWOFF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSWCR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `WDOFF` reader - Write Data Output Extension Cycle Select"]
pub type WDOFF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDOFF` writer - Write Data Output Extension Cycle Select"]
pub type WDOFF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSWCR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `AWAIT` reader - Address Cycle Wait Select"]
pub type AWAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWAIT` writer - Address Cycle Wait Select"]
pub type AWAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSWCR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `RDON` reader - RD Assert Wait Select"]
pub type RDON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDON` writer - RD Assert Wait Select"]
pub type RDON_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSWCR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `WRON` reader - WR Assert Wait Select"]
pub type WRON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRON` writer - WR Assert Wait Select"]
pub type WRON_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSWCR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `WDON` reader - Write Data Output Wait Select"]
pub type WDON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDON` writer - Write Data Output Wait Select"]
pub type WDON_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSWCR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `CSON` reader - CS Assert Wait Select"]
pub type CSON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSON` writer - CS Assert Wait Select"]
pub type CSON_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSWCR2_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Read-Access CS Extension Cycle Select"]
    #[inline(always)]
    pub fn csroff(&self) -> CSROFF_R {
        CSROFF_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Write-Access CS Extension Cycle Select"]
    #[inline(always)]
    pub fn cswoff(&self) -> CSWOFF_R {
        CSWOFF_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Write Data Output Extension Cycle Select"]
    #[inline(always)]
    pub fn wdoff(&self) -> WDOFF_R {
        WDOFF_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Address Cycle Wait Select"]
    #[inline(always)]
    pub fn await_(&self) -> AWAIT_R {
        AWAIT_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:18 - RD Assert Wait Select"]
    #[inline(always)]
    pub fn rdon(&self) -> RDON_R {
        RDON_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - WR Assert Wait Select"]
    #[inline(always)]
    pub fn wron(&self) -> WRON_R {
        WRON_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Write Data Output Wait Select"]
    #[inline(always)]
    pub fn wdon(&self) -> WDON_R {
        WDON_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - CS Assert Wait Select"]
    #[inline(always)]
    pub fn cson(&self) -> CSON_R {
        CSON_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Read-Access CS Extension Cycle Select"]
    #[inline(always)]
    #[must_use]
    pub fn csroff(&mut self) -> CSROFF_W<0> {
        CSROFF_W::new(self)
    }
    #[doc = "Bits 4:6 - Write-Access CS Extension Cycle Select"]
    #[inline(always)]
    #[must_use]
    pub fn cswoff(&mut self) -> CSWOFF_W<4> {
        CSWOFF_W::new(self)
    }
    #[doc = "Bits 8:10 - Write Data Output Extension Cycle Select"]
    #[inline(always)]
    #[must_use]
    pub fn wdoff(&mut self) -> WDOFF_W<8> {
        WDOFF_W::new(self)
    }
    #[doc = "Bits 12:13 - Address Cycle Wait Select"]
    #[inline(always)]
    #[must_use]
    pub fn await_(&mut self) -> AWAIT_W<12> {
        AWAIT_W::new(self)
    }
    #[doc = "Bits 16:18 - RD Assert Wait Select"]
    #[inline(always)]
    #[must_use]
    pub fn rdon(&mut self) -> RDON_W<16> {
        RDON_W::new(self)
    }
    #[doc = "Bits 20:22 - WR Assert Wait Select"]
    #[inline(always)]
    #[must_use]
    pub fn wron(&mut self) -> WRON_W<20> {
        WRON_W::new(self)
    }
    #[doc = "Bits 24:26 - Write Data Output Wait Select"]
    #[inline(always)]
    #[must_use]
    pub fn wdon(&mut self) -> WDON_W<24> {
        WDON_W::new(self)
    }
    #[doc = "Bits 28:30 - CS Assert Wait Select"]
    #[inline(always)]
    #[must_use]
    pub fn cson(&mut self) -> CSON_W<28> {
        CSON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CS%s Wait Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cswcr2](index.html) module"]
pub struct CSWCR2_SPEC;
impl crate::RegisterSpec for CSWCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cswcr2::R](R) reader structure"]
impl crate::Readable for CSWCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cswcr2::W](W) writer structure"]
impl crate::Writable for CSWCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CS%sWCR2 to value 0x07"]
impl crate::Resettable for CSWCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
