#[doc = "Register `PCDR` reader"]
pub struct R(crate::R<PCDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PCDR` reader - The PDC includes a 32-bit-wide, 22-stage FIFO for the storage of captured data. The PCDR register is a 4-byte space to which the FIFO is mapped, and four bytes of data are read from the PCDR register at a time."]
pub type PCDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The PDC includes a 32-bit-wide, 22-stage FIFO for the storage of captured data. The PCDR register is a 4-byte space to which the FIFO is mapped, and four bytes of data are read from the PCDR register at a time."]
    #[inline(always)]
    pub fn pcdr(&self) -> PCDR_R {
        PCDR_R::new(self.bits)
    }
}
#[doc = "PDC Receive Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcdr](index.html) module"]
pub struct PCDR_SPEC;
impl crate::RegisterSpec for PCDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcdr::R](R) reader structure"]
impl crate::Readable for PCDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PCDR to value 0"]
impl crate::Resettable for PCDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
