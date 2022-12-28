#[doc = "Register `SD_ERR_STS1` reader"]
pub struct R(crate::R<SD_ERR_STS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_ERR_STS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_ERR_STS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_ERR_STS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMDE0` reader - Command Error Flag 0"]
pub type CMDE0_R = crate::BitReader<CMDE0_A>;
#[doc = "Command Error Flag 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDE0_A {
    #[doc = "0: No error exists in command index field value of a command response"]
    _0 = 0,
    #[doc = "1: Error exists in command index field value of a command response"]
    _1 = 1,
}
impl From<CMDE0_A> for bool {
    #[inline(always)]
    fn from(variant: CMDE0_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDE0_A {
        match self.bits {
            false => CMDE0_A::_0,
            true => CMDE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMDE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMDE0_A::_1
    }
}
#[doc = "Field `CMDE1` reader - Command Error Flag 1"]
pub type CMDE1_R = crate::BitReader<CMDE1_A>;
#[doc = "Command Error Flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDE1_A {
    #[doc = "0: No error exists in command index field value of a command response"]
    _0 = 0,
    #[doc = "1: Error exists in command index field value of a command response (with SD_CMD.CMDIDX\\[5:0\\]
setting, an error that occurs with CMD12 issue is indicated in the CMDE0 flag)"]
    _1 = 1,
}
impl From<CMDE1_A> for bool {
    #[inline(always)]
    fn from(variant: CMDE1_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDE1_A {
        match self.bits {
            false => CMDE1_A::_0,
            true => CMDE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMDE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMDE1_A::_1
    }
}
#[doc = "Field `RSPLENE0` reader - Response Length Error Flag 0"]
pub type RSPLENE0_R = crate::BitReader<RSPLENE0_A>;
#[doc = "Response Length Error Flag 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPLENE0_A {
    #[doc = "0: No error exists in command response length"]
    _0 = 0,
    #[doc = "1: Error exists in command response length"]
    _1 = 1,
}
impl From<RSPLENE0_A> for bool {
    #[inline(always)]
    fn from(variant: RSPLENE0_A) -> Self {
        variant as u8 != 0
    }
}
impl RSPLENE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSPLENE0_A {
        match self.bits {
            false => RSPLENE0_A::_0,
            true => RSPLENE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSPLENE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSPLENE0_A::_1
    }
}
#[doc = "Field `RSPLENE1` reader - Response Length Error Flag 1"]
pub type RSPLENE1_R = crate::BitReader<RSPLENE1_A>;
#[doc = "Response Length Error Flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPLENE1_A {
    #[doc = "0: No error exists in command response length"]
    _0 = 0,
    #[doc = "1: Error exists in command response length (with SD_CMD.CMDIDX\\[5:0\\]
setting, an error that occurs with CMD12 issue is indicated in the RSPLENE0 flag)"]
    _1 = 1,
}
impl From<RSPLENE1_A> for bool {
    #[inline(always)]
    fn from(variant: RSPLENE1_A) -> Self {
        variant as u8 != 0
    }
}
impl RSPLENE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSPLENE1_A {
        match self.bits {
            false => RSPLENE1_A::_0,
            true => RSPLENE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSPLENE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSPLENE1_A::_1
    }
}
#[doc = "Field `RDLENE` reader - Read Data Length Error Flag"]
pub type RDLENE_R = crate::BitReader<RDLENE_A>;
#[doc = "Read Data Length Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDLENE_A {
    #[doc = "0: No read data length error occurred"]
    _0 = 0,
    #[doc = "1: Read data length error occurred"]
    _1 = 1,
}
impl From<RDLENE_A> for bool {
    #[inline(always)]
    fn from(variant: RDLENE_A) -> Self {
        variant as u8 != 0
    }
}
impl RDLENE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDLENE_A {
        match self.bits {
            false => RDLENE_A::_0,
            true => RDLENE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDLENE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDLENE_A::_1
    }
}
#[doc = "Field `CRCLENE` reader - CRC Status Token Length Error Flag"]
pub type CRCLENE_R = crate::BitReader<CRCLENE_A>;
#[doc = "CRC Status Token Length Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCLENE_A {
    #[doc = "0: No CRC status token length error occurred"]
    _0 = 0,
    #[doc = "1: CRC status token length error occurred"]
    _1 = 1,
}
impl From<CRCLENE_A> for bool {
    #[inline(always)]
    fn from(variant: CRCLENE_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCLENE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCLENE_A {
        match self.bits {
            false => CRCLENE_A::_0,
            true => CRCLENE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRCLENE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRCLENE_A::_1
    }
}
#[doc = "Field `RSPCRCE0` reader - Response CRC Error Flag 0"]
pub type RSPCRCE0_R = crate::BitReader<RSPCRCE0_A>;
#[doc = "Response CRC Error Flag 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPCRCE0_A {
    #[doc = "0: No CRC error detected in command response"]
    _0 = 0,
    #[doc = "1: CRC error detected in command response"]
    _1 = 1,
}
impl From<RSPCRCE0_A> for bool {
    #[inline(always)]
    fn from(variant: RSPCRCE0_A) -> Self {
        variant as u8 != 0
    }
}
impl RSPCRCE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSPCRCE0_A {
        match self.bits {
            false => RSPCRCE0_A::_0,
            true => RSPCRCE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSPCRCE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSPCRCE0_A::_1
    }
}
#[doc = "Field `RSPCRCE1` reader - Response CRC Error Flag 1"]
pub type RSPCRCE1_R = crate::BitReader<RSPCRCE1_A>;
#[doc = "Response CRC Error Flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPCRCE1_A {
    #[doc = "0: No CRC error detected in command response (with SD_CMD.CMDIDX\\[5:0\\]
setting, an error that occurs with CMD12 issue is indicated in the RSPCRCE0 flag)"]
    _0 = 0,
    #[doc = "1: CRC error detected in command response"]
    _1 = 1,
}
impl From<RSPCRCE1_A> for bool {
    #[inline(always)]
    fn from(variant: RSPCRCE1_A) -> Self {
        variant as u8 != 0
    }
}
impl RSPCRCE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSPCRCE1_A {
        match self.bits {
            false => RSPCRCE1_A::_0,
            true => RSPCRCE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSPCRCE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSPCRCE1_A::_1
    }
}
#[doc = "Field `RDCRCE` reader - Read Data CRC Error Flag"]
pub type RDCRCE_R = crate::BitReader<RDCRCE_A>;
#[doc = "Read Data CRC Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDCRCE_A {
    #[doc = "0: No CRC error detected in read data"]
    _0 = 0,
    #[doc = "1: CRC error detected in read data"]
    _1 = 1,
}
impl From<RDCRCE_A> for bool {
    #[inline(always)]
    fn from(variant: RDCRCE_A) -> Self {
        variant as u8 != 0
    }
}
impl RDCRCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDCRCE_A {
        match self.bits {
            false => RDCRCE_A::_0,
            true => RDCRCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDCRCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDCRCE_A::_1
    }
}
#[doc = "Field `CRCTKE` reader - CRC Status Token Error Flag"]
pub type CRCTKE_R = crate::BitReader<CRCTKE_A>;
#[doc = "CRC Status Token Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCTKE_A {
    #[doc = "0: No error detected in CRC status token"]
    _0 = 0,
    #[doc = "1: Error detected in CRC status token"]
    _1 = 1,
}
impl From<CRCTKE_A> for bool {
    #[inline(always)]
    fn from(variant: CRCTKE_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCTKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCTKE_A {
        match self.bits {
            false => CRCTKE_A::_0,
            true => CRCTKE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRCTKE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRCTKE_A::_1
    }
}
#[doc = "Field `CRCTK` reader - CRC Status Token"]
pub type CRCTK_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Command Error Flag 0"]
    #[inline(always)]
    pub fn cmde0(&self) -> CMDE0_R {
        CMDE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command Error Flag 1"]
    #[inline(always)]
    pub fn cmde1(&self) -> CMDE1_R {
        CMDE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Response Length Error Flag 0"]
    #[inline(always)]
    pub fn rsplene0(&self) -> RSPLENE0_R {
        RSPLENE0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Response Length Error Flag 1"]
    #[inline(always)]
    pub fn rsplene1(&self) -> RSPLENE1_R {
        RSPLENE1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read Data Length Error Flag"]
    #[inline(always)]
    pub fn rdlene(&self) -> RDLENE_R {
        RDLENE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CRC Status Token Length Error Flag"]
    #[inline(always)]
    pub fn crclene(&self) -> CRCLENE_R {
        CRCLENE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Response CRC Error Flag 0"]
    #[inline(always)]
    pub fn rspcrce0(&self) -> RSPCRCE0_R {
        RSPCRCE0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Response CRC Error Flag 1"]
    #[inline(always)]
    pub fn rspcrce1(&self) -> RSPCRCE1_R {
        RSPCRCE1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Read Data CRC Error Flag"]
    #[inline(always)]
    pub fn rdcrce(&self) -> RDCRCE_R {
        RDCRCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CRC Status Token Error Flag"]
    #[inline(always)]
    pub fn crctke(&self) -> CRCTKE_R {
        CRCTKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - CRC Status Token"]
    #[inline(always)]
    pub fn crctk(&self) -> CRCTK_R {
        CRCTK_R::new(((self.bits >> 12) & 7) as u8)
    }
}
#[doc = "SD Error Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_err_sts1](index.html) module"]
pub struct SD_ERR_STS1_SPEC;
impl crate::RegisterSpec for SD_ERR_STS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_err_sts1::R](R) reader structure"]
impl crate::Readable for SD_ERR_STS1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SD_ERR_STS1 to value 0x2000"]
impl crate::Resettable for SD_ERR_STS1_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000;
}
