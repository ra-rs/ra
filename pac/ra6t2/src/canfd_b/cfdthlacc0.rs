#[doc = "Register `CFDTHLACC0` reader"]
pub struct R(crate::R<CFDTHLACC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDTHLACC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDTHLACC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDTHLACC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BT` reader - Buffer Type"]
pub type BT_R = crate::FieldReader<u8, BT_A>;
#[doc = "Buffer Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BT_A {
    #[doc = "1: Flat TX message buffer"]
    _001 = 1,
    #[doc = "2: TX FIFO message buffer number"]
    _010 = 2,
    #[doc = "4: TX Queue message buffer number"]
    _100 = 4,
}
impl From<BT_A> for u8 {
    #[inline(always)]
    fn from(variant: BT_A) -> Self {
        variant as _
    }
}
impl BT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BT_A> {
        match self.bits {
            1 => Some(BT_A::_001),
            2 => Some(BT_A::_010),
            4 => Some(BT_A::_100),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == BT_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == BT_A::_010
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == BT_A::_100
    }
}
#[doc = "Field `BN` reader - Buffer Number"]
pub type BN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TMTS` reader - Transmit Timestamp"]
pub type TMTS_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:2 - Buffer Type"]
    #[inline(always)]
    pub fn bt(&self) -> BT_R {
        BT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Buffer Number"]
    #[inline(always)]
    pub fn bn(&self) -> BN_R {
        BN_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 16:31 - Transmit Timestamp"]
    #[inline(always)]
    pub fn tmts(&self) -> TMTS_R {
        TMTS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "TX History List Access Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdthlacc0](index.html) module"]
pub struct CFDTHLACC0_SPEC;
impl crate::RegisterSpec for CFDTHLACC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdthlacc0::R](R) reader structure"]
impl crate::Readable for CFDTHLACC0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFDTHLACC0 to value 0"]
impl crate::Resettable for CFDTHLACC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
