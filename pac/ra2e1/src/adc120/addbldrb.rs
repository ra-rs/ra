#[doc = "Register `ADDBLDRB` reader"]
pub struct R(crate::R<ADDBLDRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDBLDRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDBLDRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDBLDRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADDBLDRn` reader - Converted Value 15 to 0"]
pub type ADDBLDRN_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Converted Value 15 to 0"]
    #[inline(always)]
    pub fn addbldrn(&self) -> ADDBLDRN_R {
        ADDBLDRN_R::new(self.bits)
    }
}
#[doc = "A/D Data Duplexing Register B\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addbldrb](index.html) module"]
pub struct ADDBLDRB_SPEC;
impl crate::RegisterSpec for ADDBLDRB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [addbldrb::R](R) reader structure"]
impl crate::Readable for ADDBLDRB_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADDBLDRB to value 0"]
impl crate::Resettable for ADDBLDRB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
