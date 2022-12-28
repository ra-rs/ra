#[doc = "Register `CFDRMDF%s_4` reader"]
pub struct R(crate::R<CFDRMDF_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDRMDF_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDRMDF_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDRMDF_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RMDB_LL` reader - RX Message Buffer Data Byte (p Ã\u{97} 4)"]
pub type RMDB_LL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RMDB_LH` reader - RX Message Buffer Data Byte ((p Ã\u{97} 4) + 1)"]
pub type RMDB_LH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RMDB_HL` reader - RX Message Buffer Data Byte ((p Ã\u{97} 4) + 2)"]
pub type RMDB_HL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RMDB_HH` reader - RX Message Buffer Data Byte ((p Ã\u{97} 4) + 3)"]
pub type RMDB_HH_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - RX Message Buffer Data Byte (p Ã\u{97} 4)"]
    #[inline(always)]
    pub fn rmdb_ll(&self) -> RMDB_LL_R {
        RMDB_LL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RX Message Buffer Data Byte ((p Ã\u{97} 4) + 1)"]
    #[inline(always)]
    pub fn rmdb_lh(&self) -> RMDB_LH_R {
        RMDB_LH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - RX Message Buffer Data Byte ((p Ã\u{97} 4) + 2)"]
    #[inline(always)]
    pub fn rmdb_hl(&self) -> RMDB_HL_R {
        RMDB_HL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - RX Message Buffer Data Byte ((p Ã\u{97} 4) + 3)"]
    #[inline(always)]
    pub fn rmdb_hh(&self) -> RMDB_HH_R {
        RMDB_HH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "RX Message Buffer Data Field 4 Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdrmdf_4](index.html) module"]
pub struct CFDRMDF_4_SPEC;
impl crate::RegisterSpec for CFDRMDF_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdrmdf_4::R](R) reader structure"]
impl crate::Readable for CFDRMDF_4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFDRMDF%s_4 to value 0"]
impl crate::Resettable for CFDRMDF_4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
