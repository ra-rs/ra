#[doc = "Register `CFDTHLACC1` reader"]
pub struct R(crate::R<CFDTHLACC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDTHLACC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDTHLACC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDTHLACC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TID` reader - Transmit ID"]
pub type TID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIFL` reader - Transmit Information Label"]
pub type TIFL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Transmit ID"]
    #[inline(always)]
    pub fn tid(&self) -> TID_R {
        TID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - Transmit Information Label"]
    #[inline(always)]
    pub fn tifl(&self) -> TIFL_R {
        TIFL_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[doc = "TX History List Access Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdthlacc1](index.html) module"]
pub struct CFDTHLACC1_SPEC;
impl crate::RegisterSpec for CFDTHLACC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdthlacc1::R](R) reader structure"]
impl crate::Readable for CFDTHLACC1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFDTHLACC1 to value 0"]
impl crate::Resettable for CFDTHLACC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
