#[doc = "Register `SVDVAD0` reader"]
pub struct R(crate::R<SVDVAD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SVDVAD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SVDVAD0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SVDVAD0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SVAD` reader - Slave Address"]
pub type SVAD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SADLG` reader - Slave Address Length"]
pub type SADLG_R = crate::BitReader<SADLG_A>;
#[doc = "Slave Address Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SADLG_A {
    #[doc = "0: The 7-bit address format is selected."]
    _0 = 0,
    #[doc = "1: The 10-bit address format is selected."]
    _1 = 1,
}
impl From<SADLG_A> for bool {
    #[inline(always)]
    fn from(variant: SADLG_A) -> Self {
        variant as u8 != 0
    }
}
impl SADLG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SADLG_A {
        match self.bits {
            false => SADLG_A::_0,
            true => SADLG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SADLG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SADLG_A::_1
    }
}
#[doc = "Field `SSTADV` reader - Slave Static Address Valid"]
pub type SSTADV_R = crate::BitReader<SSTADV_A>;
#[doc = "Slave Static Address Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSTADV_A {
    #[doc = "0: Slave address is disabled."]
    _0 = 0,
    #[doc = "1: Slave address is enabled."]
    _1 = 1,
}
impl From<SSTADV_A> for bool {
    #[inline(always)]
    fn from(variant: SSTADV_A) -> Self {
        variant as u8 != 0
    }
}
impl SSTADV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSTADV_A {
        match self.bits {
            false => SSTADV_A::_0,
            true => SSTADV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSTADV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSTADV_A::_1
    }
}
#[doc = "Field `SDYADV` reader - Slave Dynamic Address Valid"]
pub type SDYADV_R = crate::BitReader<SDYADV_A>;
#[doc = "Slave Dynamic Address Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDYADV_A {
    #[doc = "0: Dynamic Address is disabled."]
    _0 = 0,
    #[doc = "1: Dynamic Address is enabled."]
    _1 = 1,
}
impl From<SDYADV_A> for bool {
    #[inline(always)]
    fn from(variant: SDYADV_A) -> Self {
        variant as u8 != 0
    }
}
impl SDYADV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDYADV_A {
        match self.bits {
            false => SDYADV_A::_0,
            true => SDYADV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDYADV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDYADV_A::_1
    }
}
impl R {
    #[doc = "Bits 16:25 - Slave Address"]
    #[inline(always)]
    pub fn svad(&self) -> SVAD_R {
        SVAD_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 27 - Slave Address Length"]
    #[inline(always)]
    pub fn sadlg(&self) -> SADLG_R {
        SADLG_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - Slave Static Address Valid"]
    #[inline(always)]
    pub fn sstadv(&self) -> SSTADV_R {
        SSTADV_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Slave Dynamic Address Valid"]
    #[inline(always)]
    pub fn sdyadv(&self) -> SDYADV_R {
        SDYADV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Slave Device Address Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [svdvad0](index.html) module"]
pub struct SVDVAD0_SPEC;
impl crate::RegisterSpec for SVDVAD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [svdvad0::R](R) reader structure"]
impl crate::Readable for SVDVAD0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SVDVAD0 to value 0"]
impl crate::Resettable for SVDVAD0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
