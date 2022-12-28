#[doc = "Register `EC710EAD0` reader"]
pub struct R(crate::R<EC710EAD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EC710EAD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EC710EAD0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EC710EAD0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ECEAD` reader - ECC Error Address"]
pub type ECEAD_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - ECC Error Address"]
    #[inline(always)]
    pub fn ecead(&self) -> ECEAD_R {
        ECEAD_R::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "ECC Error Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ec710ead0](index.html) module"]
pub struct EC710EAD0_SPEC;
impl crate::RegisterSpec for EC710EAD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ec710ead0::R](R) reader structure"]
impl crate::Readable for EC710EAD0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EC710EAD0 to value 0"]
impl crate::Resettable for EC710EAD0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
