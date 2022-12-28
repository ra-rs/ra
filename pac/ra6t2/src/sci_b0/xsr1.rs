#[doc = "Register `XSR1` reader"]
pub struct R(crate::R<XSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TCNT` reader - Timer Count Capture value"]
pub type TCNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timer Count Capture value"]
    #[inline(always)]
    pub fn tcnt(&self) -> TCNT_R {
        TCNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Simple LIN Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xsr1](index.html) module"]
pub struct XSR1_SPEC;
impl crate::RegisterSpec for XSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xsr1::R](R) reader structure"]
impl crate::Readable for XSR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets XSR1 to value 0"]
impl crate::Resettable for XSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
