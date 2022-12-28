#[doc = "Register `CFDCFDF%s` reader"]
pub struct R(crate::R<CFDCFDF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDCFDF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDCFDF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDCFDF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDCFDF%s` writer"]
pub struct W(crate::W<CFDCFDF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDCFDF_SPEC>;
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
impl From<crate::W<CFDCFDF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDCFDF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFDB_LL` reader - Common FIFO Buffer Data Bytes (p Ã\u{97} 4)"]
pub type CFDB_LL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFDB_LL` writer - Common FIFO Buffer Data Bytes (p Ã\u{97} 4)"]
pub type CFDB_LL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDCFDF_SPEC, u8, u8, 8, O>;
#[doc = "Field `CFDB_LH` reader - Common FIFO Buffer Data Bytes ((p Ã\u{97} 4) + 1)"]
pub type CFDB_LH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFDB_LH` writer - Common FIFO Buffer Data Bytes ((p Ã\u{97} 4) + 1)"]
pub type CFDB_LH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDCFDF_SPEC, u8, u8, 8, O>;
#[doc = "Field `CFDB_HL` reader - Common FIFO Buffer Data Bytes ((p Ã\u{97} 4) + 2)"]
pub type CFDB_HL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFDB_HL` writer - Common FIFO Buffer Data Bytes ((p Ã\u{97} 4) + 2)"]
pub type CFDB_HL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDCFDF_SPEC, u8, u8, 8, O>;
#[doc = "Field `CFDB_HH` reader - Common FIFO Buffer Data Bytes ((p Ã\u{97} 4) + 3)"]
pub type CFDB_HH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFDB_HH` writer - Common FIFO Buffer Data Bytes ((p Ã\u{97} 4) + 3)"]
pub type CFDB_HH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDCFDF_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Common FIFO Buffer Data Bytes (p Ã\u{97} 4)"]
    #[inline(always)]
    pub fn cfdb_ll(&self) -> CFDB_LL_R {
        CFDB_LL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Common FIFO Buffer Data Bytes ((p Ã\u{97} 4) + 1)"]
    #[inline(always)]
    pub fn cfdb_lh(&self) -> CFDB_LH_R {
        CFDB_LH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Common FIFO Buffer Data Bytes ((p Ã\u{97} 4) + 2)"]
    #[inline(always)]
    pub fn cfdb_hl(&self) -> CFDB_HL_R {
        CFDB_HL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Common FIFO Buffer Data Bytes ((p Ã\u{97} 4) + 3)"]
    #[inline(always)]
    pub fn cfdb_hh(&self) -> CFDB_HH_R {
        CFDB_HH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Common FIFO Buffer Data Bytes (p Ã\u{97} 4)"]
    #[inline(always)]
    #[must_use]
    pub fn cfdb_ll(&mut self) -> CFDB_LL_W<0> {
        CFDB_LL_W::new(self)
    }
    #[doc = "Bits 8:15 - Common FIFO Buffer Data Bytes ((p Ã\u{97} 4) + 1)"]
    #[inline(always)]
    #[must_use]
    pub fn cfdb_lh(&mut self) -> CFDB_LH_W<8> {
        CFDB_LH_W::new(self)
    }
    #[doc = "Bits 16:23 - Common FIFO Buffer Data Bytes ((p Ã\u{97} 4) + 2)"]
    #[inline(always)]
    #[must_use]
    pub fn cfdb_hl(&mut self) -> CFDB_HL_W<16> {
        CFDB_HL_W::new(self)
    }
    #[doc = "Bits 24:31 - Common FIFO Buffer Data Bytes ((p Ã\u{97} 4) + 3)"]
    #[inline(always)]
    #[must_use]
    pub fn cfdb_hh(&mut self) -> CFDB_HH_W<24> {
        CFDB_HH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common FIFO Access Data Field %s Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdcfdf](index.html) module"]
pub struct CFDCFDF_SPEC;
impl crate::RegisterSpec for CFDCFDF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdcfdf::R](R) reader structure"]
impl crate::Readable for CFDCFDF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdcfdf::W](W) writer structure"]
impl crate::Writable for CFDCFDF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDCFDF%s to value 0"]
impl crate::Resettable for CFDCFDF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
