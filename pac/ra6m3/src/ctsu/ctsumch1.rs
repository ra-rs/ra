#[doc = "Register `CTSUMCH1` reader"]
pub struct R(crate::R<CTSUMCH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUMCH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUMCH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUMCH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CTSUMCH1` reader - CTSU Measurement Channel 1 Note1: If the value of CTSUMCH1 was set to b'11111, the measurement is stopped."]
pub type CTSUMCH1_R = crate::FieldReader<u8, CTSUMCH1_A>;
#[doc = "CTSU Measurement Channel 1 Note1: If the value of CTSUMCH1 was set to b'11111, the measurement is stopped.\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CTSUMCH1_A(u8);
impl From<CTSUMCH1_A> for u8 {
    #[inline(always)]
    fn from(val: CTSUMCH1_A) -> Self {
        val.0 as _
    }
}
impl R {
    #[doc = "Bits 0:4 - CTSU Measurement Channel 1 Note1: If the value of CTSUMCH1 was set to b'11111, the measurement is stopped."]
    #[inline(always)]
    pub fn ctsumch1(&self) -> CTSUMCH1_R {
        CTSUMCH1_R::new(self.bits & 0x1f)
    }
}
#[doc = "CTSU Measurement Channel Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsumch1](index.html) module"]
pub struct CTSUMCH1_SPEC;
impl crate::RegisterSpec for CTSUMCH1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctsumch1::R](R) reader structure"]
impl crate::Readable for CTSUMCH1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CTSUMCH1 to value 0x1f"]
impl crate::Resettable for CTSUMCH1_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
