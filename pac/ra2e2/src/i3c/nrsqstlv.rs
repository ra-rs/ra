#[doc = "Register `NRSQSTLV` reader"]
pub struct R(crate::R<NRSQSTLV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NRSQSTLV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NRSQSTLV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NRSQSTLV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RSQLV` reader - Normal Receive Status Queue Level"]
pub type RSQLV_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Normal Receive Status Queue Level"]
    #[inline(always)]
    pub fn rsqlv(&self) -> RSQLV_R {
        RSQLV_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Normal Receive Status Queue Status Level Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nrsqstlv](index.html) module"]
pub struct NRSQSTLV_SPEC;
impl crate::RegisterSpec for NRSQSTLV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nrsqstlv::R](R) reader structure"]
impl crate::Readable for NRSQSTLV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NRSQSTLV to value 0"]
impl crate::Resettable for NRSQSTLV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
