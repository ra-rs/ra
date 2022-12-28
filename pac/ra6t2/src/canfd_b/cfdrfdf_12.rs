#[doc = "Register `CFDRFDF%s_12` reader"]
pub struct R(crate::R<CFDRFDF_12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDRFDF_12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDRFDF_12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDRFDF_12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RFDB_LL` reader - RX FIFO Buffer Data Byte (p Ã\u{97} 4)"]
pub type RFDB_LL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFDB_LH` reader - RX FIFO Buffer Data Byte ((p Ã\u{97} 4) + 1)"]
pub type RFDB_LH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFDB_HL` reader - RX FIFO Buffer Data Byte ((p Ã\u{97} 4) + 2)"]
pub type RFDB_HL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFDB_HH` reader - RX FIFO Buffer Data Byte ((p Ã\u{97} 4) + 3)"]
pub type RFDB_HH_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - RX FIFO Buffer Data Byte (p Ã\u{97} 4)"]
    #[inline(always)]
    pub fn rfdb_ll(&self) -> RFDB_LL_R {
        RFDB_LL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RX FIFO Buffer Data Byte ((p Ã\u{97} 4) + 1)"]
    #[inline(always)]
    pub fn rfdb_lh(&self) -> RFDB_LH_R {
        RFDB_LH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - RX FIFO Buffer Data Byte ((p Ã\u{97} 4) + 2)"]
    #[inline(always)]
    pub fn rfdb_hl(&self) -> RFDB_HL_R {
        RFDB_HL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - RX FIFO Buffer Data Byte ((p Ã\u{97} 4) + 3)"]
    #[inline(always)]
    pub fn rfdb_hh(&self) -> RFDB_HH_R {
        RFDB_HH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "RX FIFO Access Data Field 12 Register %s\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdrfdf_12](index.html) module"]
pub struct CFDRFDF_12_SPEC;
impl crate::RegisterSpec for CFDRFDF_12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdrfdf_12::R](R) reader structure"]
impl crate::Readable for CFDRFDF_12_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFDRFDF%s_12 to value 0"]
impl crate::Resettable for CFDRFDF_12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
