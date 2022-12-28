#[doc = "Register `SD_ERR_STS2` reader"]
pub struct R(crate::R<SD_ERR_STS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_ERR_STS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_ERR_STS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_ERR_STS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RSPTO0` reader - Response Timeout 0"]
pub type RSPTO0_R = crate::BitReader<RSPTO0_A>;
#[doc = "Response Timeout 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPTO0_A {
    #[doc = "0: Not timeout."]
    _0 = 0,
    #[doc = "1: The response (other than a response to a command issued within a command sequence) is not received though a longer time than 640 cycles of SD/MMC clock has elapsed."]
    _1 = 1,
}
impl From<RSPTO0_A> for bool {
    #[inline(always)]
    fn from(variant: RSPTO0_A) -> Self {
        variant as u8 != 0
    }
}
impl RSPTO0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSPTO0_A {
        match self.bits {
            false => RSPTO0_A::_0,
            true => RSPTO0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSPTO0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSPTO0_A::_1
    }
}
#[doc = "Field `RSPTO1` reader - Response Timeout 1"]
pub type RSPTO1_R = crate::BitReader<RSPTO1_A>;
#[doc = "Response Timeout 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPTO1_A {
    #[doc = "0: Not timeout."]
    _0 = 0,
    #[doc = "1: The response to a command issued within a command sequence*2 is not received though a longer time than 640 cycles of SD/MMC clock has elapsed. In cases where CMD12 is issued by setting a command index in SD_CMD, this is indicated in RSPTO0."]
    _1 = 1,
}
impl From<RSPTO1_A> for bool {
    #[inline(always)]
    fn from(variant: RSPTO1_A) -> Self {
        variant as u8 != 0
    }
}
impl RSPTO1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSPTO1_A {
        match self.bits {
            false => RSPTO1_A::_0,
            true => RSPTO1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSPTO1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSPTO1_A::_1
    }
}
#[doc = "Field `BSYTO0` reader - Busy Timeout 0"]
pub type BSYTO0_R = crate::BitReader<BSYTO0_A>;
#[doc = "Busy Timeout 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSYTO0_A {
    #[doc = "0: Not timeout."]
    _0 = 0,
    #[doc = "1: The busy state for longer than N-cycle continues after R1b response."]
    _1 = 1,
}
impl From<BSYTO0_A> for bool {
    #[inline(always)]
    fn from(variant: BSYTO0_A) -> Self {
        variant as u8 != 0
    }
}
impl BSYTO0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSYTO0_A {
        match self.bits {
            false => BSYTO0_A::_0,
            true => BSYTO0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSYTO0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSYTO0_A::_1
    }
}
#[doc = "Field `BSYTO1` reader - Busy Timeout 1"]
pub type BSYTO1_R = crate::BitReader<BSYTO1_A>;
#[doc = "Busy Timeout 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSYTO1_A {
    #[doc = "0: Not timeout."]
    _0 = 0,
    #[doc = "1: The busy state for longer than N-cycle continues after CMD12 has been issued within a command sequence. In cases where CMD12 is issued by setting a command index in SD_CMD, this is indicated in BSYTO0."]
    _1 = 1,
}
impl From<BSYTO1_A> for bool {
    #[inline(always)]
    fn from(variant: BSYTO1_A) -> Self {
        variant as u8 != 0
    }
}
impl BSYTO1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSYTO1_A {
        match self.bits {
            false => BSYTO1_A::_0,
            true => BSYTO1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSYTO1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSYTO1_A::_1
    }
}
#[doc = "Field `RDTO` reader - Read Data Timeout"]
pub type RDTO_R = crate::BitReader<RDTO_A>;
#[doc = "Read Data Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDTO_A {
    #[doc = "0: Not timeout"]
    _0 = 0,
    #[doc = "1: The read data is not received though a longer time than N-cycle has elapsed after read command. / The read data for the next block are not received though a longer time than N-cycle has elapsed after the reception of read data. / The read data for the next block are not received though a longer time than N-cycle has elapsed after release of the read wait state."]
    _1 = 1,
}
impl From<RDTO_A> for bool {
    #[inline(always)]
    fn from(variant: RDTO_A) -> Self {
        variant as u8 != 0
    }
}
impl RDTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDTO_A {
        match self.bits {
            false => RDTO_A::_0,
            true => RDTO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDTO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDTO_A::_1
    }
}
#[doc = "Field `CRCTO` reader - CRC Status Token Timeout"]
pub type CRCTO_R = crate::BitReader<CRCTO_A>;
#[doc = "CRC Status Token Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCTO_A {
    #[doc = "0: Not timeout"]
    _0 = 0,
    #[doc = "1: The CRC status is not received though a longer time than N-cycle has elapsed after data writing."]
    _1 = 1,
}
impl From<CRCTO_A> for bool {
    #[inline(always)]
    fn from(variant: CRCTO_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCTO_A {
        match self.bits {
            false => CRCTO_A::_0,
            true => CRCTO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRCTO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRCTO_A::_1
    }
}
#[doc = "Field `CRCBSYTO` reader - CRC Status Token Busy Timeout"]
pub type CRCBSYTO_R = crate::BitReader<CRCBSYTO_A>;
#[doc = "CRC Status Token Busy Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCBSYTO_A {
    #[doc = "0: Not timeout"]
    _0 = 0,
    #[doc = "1: The busy state continues for longer than N-cycle after the CRC status"]
    _1 = 1,
}
impl From<CRCBSYTO_A> for bool {
    #[inline(always)]
    fn from(variant: CRCBSYTO_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCBSYTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCBSYTO_A {
        match self.bits {
            false => CRCBSYTO_A::_0,
            true => CRCBSYTO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRCBSYTO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRCBSYTO_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Response Timeout 0"]
    #[inline(always)]
    pub fn rspto0(&self) -> RSPTO0_R {
        RSPTO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Response Timeout 1"]
    #[inline(always)]
    pub fn rspto1(&self) -> RSPTO1_R {
        RSPTO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Busy Timeout 0"]
    #[inline(always)]
    pub fn bsyto0(&self) -> BSYTO0_R {
        BSYTO0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Busy Timeout 1"]
    #[inline(always)]
    pub fn bsyto1(&self) -> BSYTO1_R {
        BSYTO1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read Data Timeout"]
    #[inline(always)]
    pub fn rdto(&self) -> RDTO_R {
        RDTO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CRC Status Token Timeout"]
    #[inline(always)]
    pub fn crcto(&self) -> CRCTO_R {
        CRCTO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC Status Token Busy Timeout"]
    #[inline(always)]
    pub fn crcbsyto(&self) -> CRCBSYTO_R {
        CRCBSYTO_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "SD Error Status Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_err_sts2](index.html) module"]
pub struct SD_ERR_STS2_SPEC;
impl crate::RegisterSpec for SD_ERR_STS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_err_sts2::R](R) reader structure"]
impl crate::Readable for SD_ERR_STS2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SD_ERR_STS2 to value 0"]
impl crate::Resettable for SD_ERR_STS2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
