#[doc = "Register `ADBUF%s` reader"]
pub struct R(crate::R<ADBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADBUF` reader - Converted Value 15 to 0"]
pub type ADBUF_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Converted Value 15 to 0"]
    #[inline(always)]
    pub fn adbuf(&self) -> ADBUF_R {
        ADBUF_R::new(self.bits)
    }
}
#[doc = "A/D Data Buffer Registers %s\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adbuf](index.html) module"]
pub struct ADBUF_SPEC;
impl crate::RegisterSpec for ADBUF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adbuf::R](R) reader structure"]
impl crate::Readable for ADBUF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADBUF%s to value 0"]
impl crate::Resettable for ADBUF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
