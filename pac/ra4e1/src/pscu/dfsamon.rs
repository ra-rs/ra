#[doc = "Register `DFSAMON` reader"]
pub struct R(crate::R<DFSAMON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSAMON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSAMON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSAMON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DFS` reader - Data flash Secure area"]
pub type DFS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 10:15 - Data flash Secure area"]
    #[inline(always)]
    pub fn dfs(&self) -> DFS_R {
        DFS_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
#[doc = "Data Flash Security Attribution Monitor Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsamon](index.html) module"]
pub struct DFSAMON_SPEC;
impl crate::RegisterSpec for DFSAMON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsamon::R](R) reader structure"]
impl crate::Readable for DFSAMON_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DFSAMON to value 0"]
impl crate::Resettable for DFSAMON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
