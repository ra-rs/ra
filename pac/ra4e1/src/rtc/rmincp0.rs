#[doc = "Register `RMINCP0` reader"]
pub struct R(crate::R<RMINCP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMINCP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMINCP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMINCP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MIN1` reader - 1-Minute Capture"]
pub type MIN1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIN10` reader - 10-Minute Capture"]
pub type MIN10_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - 1-Minute Capture"]
    #[inline(always)]
    pub fn min1(&self) -> MIN1_R {
        MIN1_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - 10-Minute Capture"]
    #[inline(always)]
    pub fn min10(&self) -> MIN10_R {
        MIN10_R::new((self.bits >> 4) & 7)
    }
}
#[doc = "Minute Capture Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmincp0](index.html) module"]
pub struct RMINCP0_SPEC;
impl crate::RegisterSpec for RMINCP0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rmincp0::R](R) reader structure"]
impl crate::Readable for RMINCP0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RMINCP0 to value 0"]
impl crate::Resettable for RMINCP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
