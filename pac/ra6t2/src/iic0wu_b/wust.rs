#[doc = "Register `WUST` reader"]
pub struct R(crate::R<WUST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WUASYNF` reader - Wake-up function asynchronous operation status flag"]
pub type WUASYNF_R = crate::BitReader<WUASYNF_A>;
#[doc = "Wake-up function asynchronous operation status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUASYNF_A {
    #[doc = "0: IIC synchronous circuit enable condition."]
    _0 = 0,
    #[doc = "1: IIC asynchronous circuit enable condition."]
    _1 = 1,
}
impl From<WUASYNF_A> for bool {
    #[inline(always)]
    fn from(variant: WUASYNF_A) -> Self {
        variant as u8 != 0
    }
}
impl WUASYNF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUASYNF_A {
        match self.bits {
            false => WUASYNF_A::_0,
            true => WUASYNF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUASYNF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUASYNF_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Wake-up function asynchronous operation status flag"]
    #[inline(always)]
    pub fn wuasynf(&self) -> WUASYNF_R {
        WUASYNF_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Wake Up Unit Operating Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wust](index.html) module"]
pub struct WUST_SPEC;
impl crate::RegisterSpec for WUST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wust::R](R) reader structure"]
impl crate::Readable for WUST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WUST to value 0"]
impl crate::Resettable for WUST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
