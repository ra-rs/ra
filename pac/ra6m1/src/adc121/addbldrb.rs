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
#[doc = "Field `ADDBLDRB` reader - This register is a 16-bit read-only registers for storing the result of A/D conversion in response to the respective triggers during extended operation in double trigger mode."]
pub type ADDBLDRB_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is a 16-bit read-only registers for storing the result of A/D conversion in response to the respective triggers during extended operation in double trigger mode."]
    #[inline(always)]
    pub fn addbldrb(&self) -> ADDBLDRB_R {
        ADDBLDRB_R::new(self.bits)
    }
}
#[doc = "A/D Data Duplication Register B\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addbldrb](index.html) module"]
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
    const RESET_VALUE: Self::Ux = 0;
}
