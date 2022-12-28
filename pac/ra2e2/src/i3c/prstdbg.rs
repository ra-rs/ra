#[doc = "Register `PRSTDBG` reader"]
pub struct R(crate::R<PRSTDBG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRSTDBG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRSTDBG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRSTDBG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SCILV` reader - SCL Line Signal Level"]
pub type SCILV_R = crate::BitReader<bool>;
#[doc = "Field `SDILV` reader - SDA Line Signal Level"]
pub type SDILV_R = crate::BitReader<bool>;
#[doc = "Field `SCOLV` reader - SCL Output Level"]
pub type SCOLV_R = crate::BitReader<SCOLV_A>;
#[doc = "SCL Output Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCOLV_A {
    #[doc = "0: I3C has driven the SCL pin low."]
    _0 = 0,
    #[doc = "1: I3C has released the SCL pin."]
    _1 = 1,
}
impl From<SCOLV_A> for bool {
    #[inline(always)]
    fn from(variant: SCOLV_A) -> Self {
        variant as u8 != 0
    }
}
impl SCOLV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCOLV_A {
        match self.bits {
            false => SCOLV_A::_0,
            true => SCOLV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCOLV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCOLV_A::_1
    }
}
#[doc = "Field `SDOLV` reader - SDA Output Level"]
pub type SDOLV_R = crate::BitReader<SDOLV_A>;
#[doc = "SDA Output Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDOLV_A {
    #[doc = "0: I3C has driven the SDA pin low."]
    _0 = 0,
    #[doc = "1: I3C has released the SDA pin."]
    _1 = 1,
}
impl From<SDOLV_A> for bool {
    #[inline(always)]
    fn from(variant: SDOLV_A) -> Self {
        variant as u8 != 0
    }
}
impl SDOLV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDOLV_A {
        match self.bits {
            false => SDOLV_A::_0,
            true => SDOLV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDOLV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDOLV_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - SCL Line Signal Level"]
    #[inline(always)]
    pub fn scilv(&self) -> SCILV_R {
        SCILV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SDA Line Signal Level"]
    #[inline(always)]
    pub fn sdilv(&self) -> SDILV_R {
        SDILV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCL Output Level"]
    #[inline(always)]
    pub fn scolv(&self) -> SCOLV_R {
        SCOLV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SDA Output Level"]
    #[inline(always)]
    pub fn sdolv(&self) -> SDOLV_R {
        SDOLV_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Present State Debug Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prstdbg](index.html) module"]
pub struct PRSTDBG_SPEC;
impl crate::RegisterSpec for PRSTDBG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prstdbg::R](R) reader structure"]
impl crate::Readable for PRSTDBG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRSTDBG to value 0"]
impl crate::Resettable for PRSTDBG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
