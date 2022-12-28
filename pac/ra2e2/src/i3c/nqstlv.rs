#[doc = "Register `NQSTLV` reader"]
pub struct R(crate::R<NQSTLV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NQSTLV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NQSTLV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NQSTLV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMDQFLV` reader - Normal Command Queue Free Level"]
pub type CMDQFLV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSPQLV` reader - Normal Response Queue Level"]
pub type RSPQLV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IBIQLV` reader - Normal IBI Queue Level"]
pub type IBIQLV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IBISCNT` reader - Normal IBI Status Count"]
pub type IBISCNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Normal Command Queue Free Level"]
    #[inline(always)]
    pub fn cmdqflv(&self) -> CMDQFLV_R {
        CMDQFLV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Normal Response Queue Level"]
    #[inline(always)]
    pub fn rspqlv(&self) -> RSPQLV_R {
        RSPQLV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Normal IBI Queue Level"]
    #[inline(always)]
    pub fn ibiqlv(&self) -> IBIQLV_R {
        IBIQLV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:28 - Normal IBI Status Count"]
    #[inline(always)]
    pub fn ibiscnt(&self) -> IBISCNT_R {
        IBISCNT_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
#[doc = "Normal Queue Status Level Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nqstlv](index.html) module"]
pub struct NQSTLV_SPEC;
impl crate::RegisterSpec for NQSTLV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nqstlv::R](R) reader structure"]
impl crate::Readable for NQSTLV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NQSTLV to value 0x02"]
impl crate::Resettable for NQSTLV_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
