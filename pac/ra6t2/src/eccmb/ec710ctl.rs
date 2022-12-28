#[doc = "Register `EC710CTL` reader"]
pub struct R(crate::R<EC710CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EC710CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EC710CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EC710CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EC710CTL` writer"]
pub struct W(crate::W<EC710CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EC710CTL_SPEC>;
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
impl From<crate::W<EC710CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EC710CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECEMF` reader - ECC Error Message Flag"]
pub type ECEMF_R = crate::BitReader<ECEMF_A>;
#[doc = "ECC Error Message Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECEMF_A {
    #[doc = "0: There is no bit error in present RAM output data"]
    _0 = 0,
    #[doc = "1: There is bit error in present RAM output data"]
    _1 = 1,
}
impl From<ECEMF_A> for bool {
    #[inline(always)]
    fn from(variant: ECEMF_A) -> Self {
        variant as u8 != 0
    }
}
impl ECEMF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECEMF_A {
        match self.bits {
            false => ECEMF_A::_0,
            true => ECEMF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECEMF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECEMF_A::_1
    }
}
#[doc = "Field `ECER1F` reader - ECC Error Detection and Correction Flag"]
pub type ECER1F_R = crate::BitReader<ECER1F_A>;
#[doc = "ECC Error Detection and Correction Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECER1F_A {
    #[doc = "0: After clearing this bit, 1-bit error correction has not occurred"]
    _0 = 0,
    #[doc = "1: 1-bit error has occurred"]
    _1 = 1,
}
impl From<ECER1F_A> for bool {
    #[inline(always)]
    fn from(variant: ECER1F_A) -> Self {
        variant as u8 != 0
    }
}
impl ECER1F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECER1F_A {
        match self.bits {
            false => ECER1F_A::_0,
            true => ECER1F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECER1F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECER1F_A::_1
    }
}
#[doc = "Field `ECER2F` reader - 2-bit ECC Error Detection Flag"]
pub type ECER2F_R = crate::BitReader<ECER2F_A>;
#[doc = "2-bit ECC Error Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECER2F_A {
    #[doc = "0: After clearing this bit, 2-bit error has not occurred"]
    _0 = 0,
    #[doc = "1: 2-bit error has occurred"]
    _1 = 1,
}
impl From<ECER2F_A> for bool {
    #[inline(always)]
    fn from(variant: ECER2F_A) -> Self {
        variant as u8 != 0
    }
}
impl ECER2F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECER2F_A {
        match self.bits {
            false => ECER2F_A::_0,
            true => ECER2F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECER2F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECER2F_A::_1
    }
}
#[doc = "Field `EC1EDIC` reader - ECC 1-bit Error Detection Interrupt Control"]
pub type EC1EDIC_R = crate::BitReader<EC1EDIC_A>;
#[doc = "ECC 1-bit Error Detection Interrupt Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EC1EDIC_A {
    #[doc = "0: Disable 1-bit error detection interrupt request"]
    _0 = 0,
    #[doc = "1: Enable 1-bit error detection interrupt request"]
    _1 = 1,
}
impl From<EC1EDIC_A> for bool {
    #[inline(always)]
    fn from(variant: EC1EDIC_A) -> Self {
        variant as u8 != 0
    }
}
impl EC1EDIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EC1EDIC_A {
        match self.bits {
            false => EC1EDIC_A::_0,
            true => EC1EDIC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EC1EDIC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EC1EDIC_A::_1
    }
}
#[doc = "Field `EC1EDIC` writer - ECC 1-bit Error Detection Interrupt Control"]
pub type EC1EDIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC710CTL_SPEC, EC1EDIC_A, O>;
impl<'a, const O: u8> EC1EDIC_W<'a, O> {
    #[doc = "Disable 1-bit error detection interrupt request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EC1EDIC_A::_0)
    }
    #[doc = "Enable 1-bit error detection interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EC1EDIC_A::_1)
    }
}
#[doc = "Field `EC2EDIC` reader - ECC 2-bit Error Detection Interrupt Control"]
pub type EC2EDIC_R = crate::BitReader<EC2EDIC_A>;
#[doc = "ECC 2-bit Error Detection Interrupt Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EC2EDIC_A {
    #[doc = "0: Disable 2-bit error detection interrupt request"]
    _0 = 0,
    #[doc = "1: Enable 2-bit error detection interrupt request"]
    _1 = 1,
}
impl From<EC2EDIC_A> for bool {
    #[inline(always)]
    fn from(variant: EC2EDIC_A) -> Self {
        variant as u8 != 0
    }
}
impl EC2EDIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EC2EDIC_A {
        match self.bits {
            false => EC2EDIC_A::_0,
            true => EC2EDIC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EC2EDIC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EC2EDIC_A::_1
    }
}
#[doc = "Field `EC2EDIC` writer - ECC 2-bit Error Detection Interrupt Control"]
pub type EC2EDIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC710CTL_SPEC, EC2EDIC_A, O>;
impl<'a, const O: u8> EC2EDIC_W<'a, O> {
    #[doc = "Disable 2-bit error detection interrupt request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EC2EDIC_A::_0)
    }
    #[doc = "Enable 2-bit error detection interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EC2EDIC_A::_1)
    }
}
#[doc = "Field `EC1ECP` reader - ECC 1-bit Error Correction Permission"]
pub type EC1ECP_R = crate::BitReader<EC1ECP_A>;
#[doc = "ECC 1-bit Error Correction Permission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EC1ECP_A {
    #[doc = "0: At 1-bit error detection, the error correction is executed"]
    _0 = 0,
    #[doc = "1: At 1-bit error detection, the error correction is not executed"]
    _1 = 1,
}
impl From<EC1ECP_A> for bool {
    #[inline(always)]
    fn from(variant: EC1ECP_A) -> Self {
        variant as u8 != 0
    }
}
impl EC1ECP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EC1ECP_A {
        match self.bits {
            false => EC1ECP_A::_0,
            true => EC1ECP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EC1ECP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EC1ECP_A::_1
    }
}
#[doc = "Field `EC1ECP` writer - ECC 1-bit Error Correction Permission"]
pub type EC1ECP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC710CTL_SPEC, EC1ECP_A, O>;
impl<'a, const O: u8> EC1ECP_W<'a, O> {
    #[doc = "At 1-bit error detection, the error correction is executed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EC1ECP_A::_0)
    }
    #[doc = "At 1-bit error detection, the error correction is not executed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EC1ECP_A::_1)
    }
}
#[doc = "Field `ECERVF` reader - ECC Error Judgment Enable Flag"]
pub type ECERVF_R = crate::BitReader<ECERVF_A>;
#[doc = "ECC Error Judgment Enable Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECERVF_A {
    #[doc = "0: Error judgment disable"]
    _0 = 0,
    #[doc = "1: Error judgment enable"]
    _1 = 1,
}
impl From<ECERVF_A> for bool {
    #[inline(always)]
    fn from(variant: ECERVF_A) -> Self {
        variant as u8 != 0
    }
}
impl ECERVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECERVF_A {
        match self.bits {
            false => ECERVF_A::_0,
            true => ECERVF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECERVF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECERVF_A::_1
    }
}
#[doc = "Field `ECERVF` writer - ECC Error Judgment Enable Flag"]
pub type ECERVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC710CTL_SPEC, ECERVF_A, O>;
impl<'a, const O: u8> ECERVF_W<'a, O> {
    #[doc = "Error judgment disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ECERVF_A::_0)
    }
    #[doc = "Error judgment enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ECERVF_A::_1)
    }
}
#[doc = "Field `ECER1C` reader - Accumulating ECC Error Detection and Correction Flag Clear"]
pub type ECER1C_R = crate::BitReader<ECER1C_A>;
#[doc = "Accumulating ECC Error Detection and Correction Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECER1C_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear accumulating ECC error detection and correction flag"]
    _1 = 1,
}
impl From<ECER1C_A> for bool {
    #[inline(always)]
    fn from(variant: ECER1C_A) -> Self {
        variant as u8 != 0
    }
}
impl ECER1C_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECER1C_A {
        match self.bits {
            false => ECER1C_A::_0,
            true => ECER1C_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECER1C_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECER1C_A::_1
    }
}
#[doc = "Field `ECER1C` writer - Accumulating ECC Error Detection and Correction Flag Clear"]
pub type ECER1C_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC710CTL_SPEC, ECER1C_A, O>;
impl<'a, const O: u8> ECER1C_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ECER1C_A::_0)
    }
    #[doc = "Clear accumulating ECC error detection and correction flag"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ECER1C_A::_1)
    }
}
#[doc = "Field `ECER2C` reader - 2-bit ECC Error Detection Flag Clear"]
pub type ECER2C_R = crate::BitReader<ECER2C_A>;
#[doc = "2-bit ECC Error Detection Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECER2C_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear 2-bit ECC error detection flag"]
    _1 = 1,
}
impl From<ECER2C_A> for bool {
    #[inline(always)]
    fn from(variant: ECER2C_A) -> Self {
        variant as u8 != 0
    }
}
impl ECER2C_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECER2C_A {
        match self.bits {
            false => ECER2C_A::_0,
            true => ECER2C_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECER2C_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECER2C_A::_1
    }
}
#[doc = "Field `ECER2C` writer - 2-bit ECC Error Detection Flag Clear"]
pub type ECER2C_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC710CTL_SPEC, ECER2C_A, O>;
impl<'a, const O: u8> ECER2C_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ECER2C_A::_0)
    }
    #[doc = "Clear 2-bit ECC error detection flag"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ECER2C_A::_1)
    }
}
#[doc = "Field `ECOVFF` reader - ECC Overflow Detection Flag"]
pub type ECOVFF_R = crate::BitReader<ECOVFF_A>;
#[doc = "ECC Overflow Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECOVFF_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ECC overflow detection flag"]
    _1 = 1,
}
impl From<ECOVFF_A> for bool {
    #[inline(always)]
    fn from(variant: ECOVFF_A) -> Self {
        variant as u8 != 0
    }
}
impl ECOVFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECOVFF_A {
        match self.bits {
            false => ECOVFF_A::_0,
            true => ECOVFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECOVFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECOVFF_A::_1
    }
}
#[doc = "Field `EMCA` reader - Access Control to ECC Mode Select bit"]
pub type EMCA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EMCA` writer - Access Control to ECC Mode Select bit"]
pub type EMCA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EC710CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `ECSEDF0` reader - ECC Single bit Error Address Detection Flag"]
pub type ECSEDF0_R = crate::BitReader<ECSEDF0_A>;
#[doc = "ECC Single bit Error Address Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECSEDF0_A {
    #[doc = "0: There is no bit error in EC710EAD0 after reset or clearing ECER1F bit"]
    _0 = 0,
    #[doc = "1: Address captured in EC710EAD0 shows that 1-bit error occurred and captured"]
    _1 = 1,
}
impl From<ECSEDF0_A> for bool {
    #[inline(always)]
    fn from(variant: ECSEDF0_A) -> Self {
        variant as u8 != 0
    }
}
impl ECSEDF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECSEDF0_A {
        match self.bits {
            false => ECSEDF0_A::_0,
            true => ECSEDF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECSEDF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECSEDF0_A::_1
    }
}
#[doc = "Field `ECDEDF0` reader - ECC Dual Bit Error Address Detection Flag"]
pub type ECDEDF0_R = crate::BitReader<ECDEDF0_A>;
#[doc = "ECC Dual Bit Error Address Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECDEDF0_A {
    #[doc = "0: There is no bit error in EC710EAD0 after reset or clearing ECER2F bit"]
    _0 = 0,
    #[doc = "1: Address captured in EC710EAD0 shows that 2-bit error occurred and captured"]
    _1 = 1,
}
impl From<ECDEDF0_A> for bool {
    #[inline(always)]
    fn from(variant: ECDEDF0_A) -> Self {
        variant as u8 != 0
    }
}
impl ECDEDF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECDEDF0_A {
        match self.bits {
            false => ECDEDF0_A::_0,
            true => ECDEDF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECDEDF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECDEDF0_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - ECC Error Message Flag"]
    #[inline(always)]
    pub fn ecemf(&self) -> ECEMF_R {
        ECEMF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECC Error Detection and Correction Flag"]
    #[inline(always)]
    pub fn ecer1f(&self) -> ECER1F_R {
        ECER1F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2-bit ECC Error Detection Flag"]
    #[inline(always)]
    pub fn ecer2f(&self) -> ECER2F_R {
        ECER2F_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECC 1-bit Error Detection Interrupt Control"]
    #[inline(always)]
    pub fn ec1edic(&self) -> EC1EDIC_R {
        EC1EDIC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ECC 2-bit Error Detection Interrupt Control"]
    #[inline(always)]
    pub fn ec2edic(&self) -> EC2EDIC_R {
        EC2EDIC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ECC 1-bit Error Correction Permission"]
    #[inline(always)]
    pub fn ec1ecp(&self) -> EC1ECP_R {
        EC1ECP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ECC Error Judgment Enable Flag"]
    #[inline(always)]
    pub fn ecervf(&self) -> ECERVF_R {
        ECERVF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Accumulating ECC Error Detection and Correction Flag Clear"]
    #[inline(always)]
    pub fn ecer1c(&self) -> ECER1C_R {
        ECER1C_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 2-bit ECC Error Detection Flag Clear"]
    #[inline(always)]
    pub fn ecer2c(&self) -> ECER2C_R {
        ECER2C_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ECC Overflow Detection Flag"]
    #[inline(always)]
    pub fn ecovff(&self) -> ECOVFF_R {
        ECOVFF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Access Control to ECC Mode Select bit"]
    #[inline(always)]
    pub fn emca(&self) -> EMCA_R {
        EMCA_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - ECC Single bit Error Address Detection Flag"]
    #[inline(always)]
    pub fn ecsedf0(&self) -> ECSEDF0_R {
        ECSEDF0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ECC Dual Bit Error Address Detection Flag"]
    #[inline(always)]
    pub fn ecdedf0(&self) -> ECDEDF0_R {
        ECDEDF0_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - ECC 1-bit Error Detection Interrupt Control"]
    #[inline(always)]
    #[must_use]
    pub fn ec1edic(&mut self) -> EC1EDIC_W<3> {
        EC1EDIC_W::new(self)
    }
    #[doc = "Bit 4 - ECC 2-bit Error Detection Interrupt Control"]
    #[inline(always)]
    #[must_use]
    pub fn ec2edic(&mut self) -> EC2EDIC_W<4> {
        EC2EDIC_W::new(self)
    }
    #[doc = "Bit 5 - ECC 1-bit Error Correction Permission"]
    #[inline(always)]
    #[must_use]
    pub fn ec1ecp(&mut self) -> EC1ECP_W<5> {
        EC1ECP_W::new(self)
    }
    #[doc = "Bit 6 - ECC Error Judgment Enable Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ecervf(&mut self) -> ECERVF_W<6> {
        ECERVF_W::new(self)
    }
    #[doc = "Bit 9 - Accumulating ECC Error Detection and Correction Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ecer1c(&mut self) -> ECER1C_W<9> {
        ECER1C_W::new(self)
    }
    #[doc = "Bit 10 - 2-bit ECC Error Detection Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ecer2c(&mut self) -> ECER2C_W<10> {
        ECER2C_W::new(self)
    }
    #[doc = "Bits 14:15 - Access Control to ECC Mode Select bit"]
    #[inline(always)]
    #[must_use]
    pub fn emca(&mut self) -> EMCA_W<14> {
        EMCA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ec710ctl](index.html) module"]
pub struct EC710CTL_SPEC;
impl crate::RegisterSpec for EC710CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ec710ctl::R](R) reader structure"]
impl crate::Readable for EC710CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ec710ctl::W](W) writer structure"]
impl crate::Writable for EC710CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EC710CTL to value 0x10"]
impl crate::Resettable for EC710CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
