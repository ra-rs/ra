#[doc = "Register `WUCTL` reader"]
pub struct R(crate::R<WUCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUCTL` writer"]
pub struct W(crate::W<WUCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<WUCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WUACKS` reader - Wake-Up Acknowledge Selection"]
pub type WUACKS_R = crate::BitReader<bool>;
#[doc = "Field `WUACKS` writer - Wake-Up Acknowledge Selection"]
pub type WUACKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUCTL_SPEC, bool, O>;
#[doc = "Field `WUANFS` reader - Wake-Up Analog Noise Filter Selection"]
pub type WUANFS_R = crate::BitReader<WUANFS_A>;
#[doc = "Wake-Up Analog Noise Filter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUANFS_A {
    #[doc = "0: Do not add the Wake Up analog filter."]
    _0 = 0,
    #[doc = "1: Add the Wake Up analog filter."]
    _1 = 1,
}
impl From<WUANFS_A> for bool {
    #[inline(always)]
    fn from(variant: WUANFS_A) -> Self {
        variant as u8 != 0
    }
}
impl WUANFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUANFS_A {
        match self.bits {
            false => WUANFS_A::_0,
            true => WUANFS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUANFS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUANFS_A::_1
    }
}
#[doc = "Field `WUANFS` writer - Wake-Up Analog Noise Filter Selection"]
pub type WUANFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUCTL_SPEC, WUANFS_A, O>;
impl<'a, const O: u8> WUANFS_W<'a, O> {
    #[doc = "Do not add the Wake Up analog filter."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUANFS_A::_0)
    }
    #[doc = "Add the Wake Up analog filter."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUANFS_A::_1)
    }
}
#[doc = "Field `WUFSYNE` reader - Wake-Up function PCLKA Synchronous Enable"]
pub type WUFSYNE_R = crate::BitReader<WUFSYNE_A>;
#[doc = "Wake-Up function PCLKA Synchronous Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUFSYNE_A {
    #[doc = "0: IIC asynchronous circuit enable"]
    _0 = 0,
    #[doc = "1: IIC synchronous circuit enable"]
    _1 = 1,
}
impl From<WUFSYNE_A> for bool {
    #[inline(always)]
    fn from(variant: WUFSYNE_A) -> Self {
        variant as u8 != 0
    }
}
impl WUFSYNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUFSYNE_A {
        match self.bits {
            false => WUFSYNE_A::_0,
            true => WUFSYNE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUFSYNE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUFSYNE_A::_1
    }
}
#[doc = "Field `WUFSYNE` writer - Wake-Up function PCLKA Synchronous Enable"]
pub type WUFSYNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUCTL_SPEC, WUFSYNE_A, O>;
impl<'a, const O: u8> WUFSYNE_W<'a, O> {
    #[doc = "IIC asynchronous circuit enable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUFSYNE_A::_0)
    }
    #[doc = "IIC synchronous circuit enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUFSYNE_A::_1)
    }
}
#[doc = "Field `WUFE` reader - Wake-Up function Enable"]
pub type WUFE_R = crate::BitReader<WUFE_A>;
#[doc = "Wake-Up function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUFE_A {
    #[doc = "0: Wake-up function disables"]
    _0 = 0,
    #[doc = "1: Wake-up function enables"]
    _1 = 1,
}
impl From<WUFE_A> for bool {
    #[inline(always)]
    fn from(variant: WUFE_A) -> Self {
        variant as u8 != 0
    }
}
impl WUFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUFE_A {
        match self.bits {
            false => WUFE_A::_0,
            true => WUFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUFE_A::_1
    }
}
#[doc = "Field `WUFE` writer - Wake-Up function Enable"]
pub type WUFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUCTL_SPEC, WUFE_A, O>;
impl<'a, const O: u8> WUFE_W<'a, O> {
    #[doc = "Wake-up function disables"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUFE_A::_0)
    }
    #[doc = "Wake-up function enables"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUFE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Wake-Up Acknowledge Selection"]
    #[inline(always)]
    pub fn wuacks(&self) -> WUACKS_R {
        WUACKS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Wake-Up Analog Noise Filter Selection"]
    #[inline(always)]
    pub fn wuanfs(&self) -> WUANFS_R {
        WUANFS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Wake-Up function PCLKA Synchronous Enable"]
    #[inline(always)]
    pub fn wufsyne(&self) -> WUFSYNE_R {
        WUFSYNE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wake-Up function Enable"]
    #[inline(always)]
    pub fn wufe(&self) -> WUFE_R {
        WUFE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-Up Acknowledge Selection"]
    #[inline(always)]
    #[must_use]
    pub fn wuacks(&mut self) -> WUACKS_W<0> {
        WUACKS_W::new(self)
    }
    #[doc = "Bit 4 - Wake-Up Analog Noise Filter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn wuanfs(&mut self) -> WUANFS_W<4> {
        WUANFS_W::new(self)
    }
    #[doc = "Bit 6 - Wake-Up function PCLKA Synchronous Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wufsyne(&mut self) -> WUFSYNE_W<6> {
        WUFSYNE_W::new(self)
    }
    #[doc = "Bit 7 - Wake-Up function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wufe(&mut self) -> WUFE_W<7> {
        WUFE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wake Up Unit Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wuctl](index.html) module"]
pub struct WUCTL_SPEC;
impl crate::RegisterSpec for WUCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wuctl::R](R) reader structure"]
impl crate::Readable for WUCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wuctl::W](W) writer structure"]
impl crate::Writable for WUCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WUCTL to value 0x41"]
impl crate::Resettable for WUCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x41;
}
