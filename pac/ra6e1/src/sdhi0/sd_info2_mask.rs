#[doc = "Register `SD_INFO2_MASK` reader"]
pub struct R(crate::R<SD_INFO2_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_INFO2_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_INFO2_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_INFO2_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SD_INFO2_MASK` writer"]
pub struct W(crate::W<SD_INFO2_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_INFO2_MASK_SPEC>;
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
impl From<crate::W<SD_INFO2_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_INFO2_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDEM` reader - Command Error Interrupt Request Mask"]
pub type CMDEM_R = crate::BitReader<CMDEM_A>;
#[doc = "Command Error Interrupt Request Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDEM_A {
    #[doc = "0: Do not mask command error interrupt request"]
    _0 = 0,
    #[doc = "1: Mask command error interrupt request"]
    _1 = 1,
}
impl From<CMDEM_A> for bool {
    #[inline(always)]
    fn from(variant: CMDEM_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDEM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDEM_A {
        match self.bits {
            false => CMDEM_A::_0,
            true => CMDEM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMDEM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMDEM_A::_1
    }
}
#[doc = "Field `CMDEM` writer - Command Error Interrupt Request Mask"]
pub type CMDEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_INFO2_MASK_SPEC, CMDEM_A, O>;
impl<'a, const O: u8> CMDEM_W<'a, O> {
    #[doc = "Do not mask command error interrupt request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMDEM_A::_0)
    }
    #[doc = "Mask command error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMDEM_A::_1)
    }
}
#[doc = "Field `CRCEM` reader - CRC Error Interrupt Request Mask"]
pub type CRCEM_R = crate::BitReader<CRCEM_A>;
#[doc = "CRC Error Interrupt Request Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCEM_A {
    #[doc = "0: Do not mask CRC error interrupt request"]
    _0 = 0,
    #[doc = "1: Mask CRC error interrupt request"]
    _1 = 1,
}
impl From<CRCEM_A> for bool {
    #[inline(always)]
    fn from(variant: CRCEM_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCEM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCEM_A {
        match self.bits {
            false => CRCEM_A::_0,
            true => CRCEM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRCEM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRCEM_A::_1
    }
}
#[doc = "Field `CRCEM` writer - CRC Error Interrupt Request Mask"]
pub type CRCEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_INFO2_MASK_SPEC, CRCEM_A, O>;
impl<'a, const O: u8> CRCEM_W<'a, O> {
    #[doc = "Do not mask CRC error interrupt request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRCEM_A::_0)
    }
    #[doc = "Mask CRC error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRCEM_A::_1)
    }
}
#[doc = "Field `ENDEM` reader - End Bit Error Interrupt Request Mask"]
pub type ENDEM_R = crate::BitReader<ENDEM_A>;
#[doc = "End Bit Error Interrupt Request Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDEM_A {
    #[doc = "0: Do not mask end bit detection error interrupt request"]
    _0 = 0,
    #[doc = "1: Mask end bit detection error interrupt request"]
    _1 = 1,
}
impl From<ENDEM_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEM_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDEM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEM_A {
        match self.bits {
            false => ENDEM_A::_0,
            true => ENDEM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENDEM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENDEM_A::_1
    }
}
#[doc = "Field `ENDEM` writer - End Bit Error Interrupt Request Mask"]
pub type ENDEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_INFO2_MASK_SPEC, ENDEM_A, O>;
impl<'a, const O: u8> ENDEM_W<'a, O> {
    #[doc = "Do not mask end bit detection error interrupt request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENDEM_A::_0)
    }
    #[doc = "Mask end bit detection error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENDEM_A::_1)
    }
}
#[doc = "Field `DTOM` reader - Data Timeout Interrupt Request Mask"]
pub type DTOM_R = crate::BitReader<DTOM_A>;
#[doc = "Data Timeout Interrupt Request Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTOM_A {
    #[doc = "0: Do not mask data timeout interrupt request"]
    _0 = 0,
    #[doc = "1: Mask data timeout interrupt request"]
    _1 = 1,
}
impl From<DTOM_A> for bool {
    #[inline(always)]
    fn from(variant: DTOM_A) -> Self {
        variant as u8 != 0
    }
}
impl DTOM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTOM_A {
        match self.bits {
            false => DTOM_A::_0,
            true => DTOM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTOM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTOM_A::_1
    }
}
#[doc = "Field `DTOM` writer - Data Timeout Interrupt Request Mask"]
pub type DTOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_INFO2_MASK_SPEC, DTOM_A, O>;
impl<'a, const O: u8> DTOM_W<'a, O> {
    #[doc = "Do not mask data timeout interrupt request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTOM_A::_0)
    }
    #[doc = "Mask data timeout interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTOM_A::_1)
    }
}
#[doc = "Field `ILWM` reader - SD_BUF0 Register Illegal Write Interrupt Request Mask"]
pub type ILWM_R = crate::BitReader<ILWM_A>;
#[doc = "SD_BUF0 Register Illegal Write Interrupt Request Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILWM_A {
    #[doc = "0: Do not mask illegal write detection interrupt request for the SD_BUF0 register"]
    _0 = 0,
    #[doc = "1: Mask illegal write detection interrupt request for the SD_BUF0 register"]
    _1 = 1,
}
impl From<ILWM_A> for bool {
    #[inline(always)]
    fn from(variant: ILWM_A) -> Self {
        variant as u8 != 0
    }
}
impl ILWM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILWM_A {
        match self.bits {
            false => ILWM_A::_0,
            true => ILWM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILWM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILWM_A::_1
    }
}
#[doc = "Field `ILWM` writer - SD_BUF0 Register Illegal Write Interrupt Request Mask"]
pub type ILWM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_INFO2_MASK_SPEC, ILWM_A, O>;
impl<'a, const O: u8> ILWM_W<'a, O> {
    #[doc = "Do not mask illegal write detection interrupt request for the SD_BUF0 register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ILWM_A::_0)
    }
    #[doc = "Mask illegal write detection interrupt request for the SD_BUF0 register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ILWM_A::_1)
    }
}
#[doc = "Field `ILRM` reader - SD_BUF0 Register Illegal Read Interrupt Request Mask"]
pub type ILRM_R = crate::BitReader<ILRM_A>;
#[doc = "SD_BUF0 Register Illegal Read Interrupt Request Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILRM_A {
    #[doc = "0: Do not mask illegal read detection interrupt request for the SD_BUF0 register"]
    _0 = 0,
    #[doc = "1: Mask illegal read detection interrupt request for the SD_BUF0 register"]
    _1 = 1,
}
impl From<ILRM_A> for bool {
    #[inline(always)]
    fn from(variant: ILRM_A) -> Self {
        variant as u8 != 0
    }
}
impl ILRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILRM_A {
        match self.bits {
            false => ILRM_A::_0,
            true => ILRM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILRM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILRM_A::_1
    }
}
#[doc = "Field `ILRM` writer - SD_BUF0 Register Illegal Read Interrupt Request Mask"]
pub type ILRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_INFO2_MASK_SPEC, ILRM_A, O>;
impl<'a, const O: u8> ILRM_W<'a, O> {
    #[doc = "Do not mask illegal read detection interrupt request for the SD_BUF0 register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ILRM_A::_0)
    }
    #[doc = "Mask illegal read detection interrupt request for the SD_BUF0 register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ILRM_A::_1)
    }
}
#[doc = "Field `RSPTOM` reader - Response Timeout Interrupt Request Mask"]
pub type RSPTOM_R = crate::BitReader<RSPTOM_A>;
#[doc = "Response Timeout Interrupt Request Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPTOM_A {
    #[doc = "0: Do not mask response timeout interrupt request"]
    _0 = 0,
    #[doc = "1: Mask response timeout interrupt request"]
    _1 = 1,
}
impl From<RSPTOM_A> for bool {
    #[inline(always)]
    fn from(variant: RSPTOM_A) -> Self {
        variant as u8 != 0
    }
}
impl RSPTOM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSPTOM_A {
        match self.bits {
            false => RSPTOM_A::_0,
            true => RSPTOM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSPTOM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSPTOM_A::_1
    }
}
#[doc = "Field `RSPTOM` writer - Response Timeout Interrupt Request Mask"]
pub type RSPTOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_INFO2_MASK_SPEC, RSPTOM_A, O>;
impl<'a, const O: u8> RSPTOM_W<'a, O> {
    #[doc = "Do not mask response timeout interrupt request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSPTOM_A::_0)
    }
    #[doc = "Mask response timeout interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSPTOM_A::_1)
    }
}
#[doc = "Field `BREM` reader - BRE Interrupt Request Mask"]
pub type BREM_R = crate::BitReader<BREM_A>;
#[doc = "BRE Interrupt Request Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BREM_A {
    #[doc = "0: Do not mask read enable interrupt request for the SD buffer"]
    _0 = 0,
    #[doc = "1: Mask read enable interrupt request for the SD buffer"]
    _1 = 1,
}
impl From<BREM_A> for bool {
    #[inline(always)]
    fn from(variant: BREM_A) -> Self {
        variant as u8 != 0
    }
}
impl BREM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BREM_A {
        match self.bits {
            false => BREM_A::_0,
            true => BREM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BREM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BREM_A::_1
    }
}
#[doc = "Field `BREM` writer - BRE Interrupt Request Mask"]
pub type BREM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_INFO2_MASK_SPEC, BREM_A, O>;
impl<'a, const O: u8> BREM_W<'a, O> {
    #[doc = "Do not mask read enable interrupt request for the SD buffer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BREM_A::_0)
    }
    #[doc = "Mask read enable interrupt request for the SD buffer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BREM_A::_1)
    }
}
#[doc = "Field `BWEM` reader - BWE Interrupt Request Mask"]
pub type BWEM_R = crate::BitReader<BWEM_A>;
#[doc = "BWE Interrupt Request Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWEM_A {
    #[doc = "0: Do not mask write enable interrupt request for the SD_BUF0 register"]
    _0 = 0,
    #[doc = "1: Mask write enable interrupt request for the SD_BUF0 register"]
    _1 = 1,
}
impl From<BWEM_A> for bool {
    #[inline(always)]
    fn from(variant: BWEM_A) -> Self {
        variant as u8 != 0
    }
}
impl BWEM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWEM_A {
        match self.bits {
            false => BWEM_A::_0,
            true => BWEM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BWEM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BWEM_A::_1
    }
}
#[doc = "Field `BWEM` writer - BWE Interrupt Request Mask"]
pub type BWEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_INFO2_MASK_SPEC, BWEM_A, O>;
impl<'a, const O: u8> BWEM_W<'a, O> {
    #[doc = "Do not mask write enable interrupt request for the SD_BUF0 register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BWEM_A::_0)
    }
    #[doc = "Mask write enable interrupt request for the SD_BUF0 register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BWEM_A::_1)
    }
}
#[doc = "Field `ILAM` reader - Illegal Access Error Interrupt Request Mask"]
pub type ILAM_R = crate::BitReader<ILAM_A>;
#[doc = "Illegal Access Error Interrupt Request Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILAM_A {
    #[doc = "0: Do not mask illegal access error interrupt request"]
    _0 = 0,
    #[doc = "1: Mask illegal access error interrupt request"]
    _1 = 1,
}
impl From<ILAM_A> for bool {
    #[inline(always)]
    fn from(variant: ILAM_A) -> Self {
        variant as u8 != 0
    }
}
impl ILAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILAM_A {
        match self.bits {
            false => ILAM_A::_0,
            true => ILAM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILAM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILAM_A::_1
    }
}
#[doc = "Field `ILAM` writer - Illegal Access Error Interrupt Request Mask"]
pub type ILAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_INFO2_MASK_SPEC, ILAM_A, O>;
impl<'a, const O: u8> ILAM_W<'a, O> {
    #[doc = "Do not mask illegal access error interrupt request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ILAM_A::_0)
    }
    #[doc = "Mask illegal access error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ILAM_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Command Error Interrupt Request Mask"]
    #[inline(always)]
    pub fn cmdem(&self) -> CMDEM_R {
        CMDEM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CRC Error Interrupt Request Mask"]
    #[inline(always)]
    pub fn crcem(&self) -> CRCEM_R {
        CRCEM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End Bit Error Interrupt Request Mask"]
    #[inline(always)]
    pub fn endem(&self) -> ENDEM_R {
        ENDEM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Timeout Interrupt Request Mask"]
    #[inline(always)]
    pub fn dtom(&self) -> DTOM_R {
        DTOM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SD_BUF0 Register Illegal Write Interrupt Request Mask"]
    #[inline(always)]
    pub fn ilwm(&self) -> ILWM_R {
        ILWM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SD_BUF0 Register Illegal Read Interrupt Request Mask"]
    #[inline(always)]
    pub fn ilrm(&self) -> ILRM_R {
        ILRM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Response Timeout Interrupt Request Mask"]
    #[inline(always)]
    pub fn rsptom(&self) -> RSPTOM_R {
        RSPTOM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - BRE Interrupt Request Mask"]
    #[inline(always)]
    pub fn brem(&self) -> BREM_R {
        BREM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BWE Interrupt Request Mask"]
    #[inline(always)]
    pub fn bwem(&self) -> BWEM_R {
        BWEM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - Illegal Access Error Interrupt Request Mask"]
    #[inline(always)]
    pub fn ilam(&self) -> ILAM_R {
        ILAM_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Error Interrupt Request Mask"]
    #[inline(always)]
    #[must_use]
    pub fn cmdem(&mut self) -> CMDEM_W<0> {
        CMDEM_W::new(self)
    }
    #[doc = "Bit 1 - CRC Error Interrupt Request Mask"]
    #[inline(always)]
    #[must_use]
    pub fn crcem(&mut self) -> CRCEM_W<1> {
        CRCEM_W::new(self)
    }
    #[doc = "Bit 2 - End Bit Error Interrupt Request Mask"]
    #[inline(always)]
    #[must_use]
    pub fn endem(&mut self) -> ENDEM_W<2> {
        ENDEM_W::new(self)
    }
    #[doc = "Bit 3 - Data Timeout Interrupt Request Mask"]
    #[inline(always)]
    #[must_use]
    pub fn dtom(&mut self) -> DTOM_W<3> {
        DTOM_W::new(self)
    }
    #[doc = "Bit 4 - SD_BUF0 Register Illegal Write Interrupt Request Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ilwm(&mut self) -> ILWM_W<4> {
        ILWM_W::new(self)
    }
    #[doc = "Bit 5 - SD_BUF0 Register Illegal Read Interrupt Request Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ilrm(&mut self) -> ILRM_W<5> {
        ILRM_W::new(self)
    }
    #[doc = "Bit 6 - Response Timeout Interrupt Request Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rsptom(&mut self) -> RSPTOM_W<6> {
        RSPTOM_W::new(self)
    }
    #[doc = "Bit 8 - BRE Interrupt Request Mask"]
    #[inline(always)]
    #[must_use]
    pub fn brem(&mut self) -> BREM_W<8> {
        BREM_W::new(self)
    }
    #[doc = "Bit 9 - BWE Interrupt Request Mask"]
    #[inline(always)]
    #[must_use]
    pub fn bwem(&mut self) -> BWEM_W<9> {
        BWEM_W::new(self)
    }
    #[doc = "Bit 15 - Illegal Access Error Interrupt Request Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ilam(&mut self) -> ILAM_W<15> {
        ILAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD INFO2 Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_info2_mask](index.html) module"]
pub struct SD_INFO2_MASK_SPEC;
impl crate::RegisterSpec for SD_INFO2_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_info2_mask::R](R) reader structure"]
impl crate::Readable for SD_INFO2_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_info2_mask::W](W) writer structure"]
impl crate::Writable for SD_INFO2_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SD_INFO2_MASK to value 0x8b7f"]
impl crate::Resettable for SD_INFO2_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x8b7f;
}
