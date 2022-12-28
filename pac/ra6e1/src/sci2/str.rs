#[doc = "Register `STR` reader"]
pub struct R(crate::R<STR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BFDF` reader - Break Field Low Width Detection Flag"]
pub type BFDF_R = crate::BitReader<bool>;
#[doc = "Field `CF0MF` reader - Control Field 0 Match Flag"]
pub type CF0MF_R = crate::BitReader<bool>;
#[doc = "Field `CF1MF` reader - Control Field 1 Match Flag"]
pub type CF1MF_R = crate::BitReader<bool>;
#[doc = "Field `PIBDF` reader - Priority Interrupt Bit Detection Flag"]
pub type PIBDF_R = crate::BitReader<bool>;
#[doc = "Field `BCDF` reader - Bus Collision Detected Flag"]
pub type BCDF_R = crate::BitReader<bool>;
#[doc = "Field `AEDF` reader - Valid Edge Detection Flag"]
pub type AEDF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Break Field Low Width Detection Flag"]
    #[inline(always)]
    pub fn bfdf(&self) -> BFDF_R {
        BFDF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control Field 0 Match Flag"]
    #[inline(always)]
    pub fn cf0mf(&self) -> CF0MF_R {
        CF0MF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Control Field 1 Match Flag"]
    #[inline(always)]
    pub fn cf1mf(&self) -> CF1MF_R {
        CF1MF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Priority Interrupt Bit Detection Flag"]
    #[inline(always)]
    pub fn pibdf(&self) -> PIBDF_R {
        PIBDF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bus Collision Detected Flag"]
    #[inline(always)]
    pub fn bcdf(&self) -> BCDF_R {
        BCDF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Valid Edge Detection Flag"]
    #[inline(always)]
    pub fn aedf(&self) -> AEDF_R {
        AEDF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [str](index.html) module"]
pub struct STR_SPEC;
impl crate::RegisterSpec for STR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [str::R](R) reader structure"]
impl crate::Readable for STR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STR to value 0"]
impl crate::Resettable for STR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
