#[doc = "Register `GTIOR` reader"]
pub struct R(crate::R<GTIOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTIOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTIOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTIOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTIOR` writer"]
pub struct W(crate::W<GTIOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTIOR_SPEC>;
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
impl From<crate::W<GTIOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTIOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GTIOA` reader - GTIOCnA Pin Function Select"]
pub type GTIOA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GTIOA` writer - GTIOCnA Pin Function Select"]
pub type GTIOA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTIOR_SPEC, u8, u8, 5, O>;
#[doc = "Field `OADFLT` reader - GTIOCnA Pin Output Value Setting at the Count Stop"]
pub type OADFLT_R = crate::BitReader<OADFLT_A>;
#[doc = "GTIOCnA Pin Output Value Setting at the Count Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OADFLT_A {
    #[doc = "0: The GTIOCnA pin outputs low when counting stops"]
    _0 = 0,
    #[doc = "1: The GTIOCnA pin outputs high when counting stops"]
    _1 = 1,
}
impl From<OADFLT_A> for bool {
    #[inline(always)]
    fn from(variant: OADFLT_A) -> Self {
        variant as u8 != 0
    }
}
impl OADFLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OADFLT_A {
        match self.bits {
            false => OADFLT_A::_0,
            true => OADFLT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OADFLT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OADFLT_A::_1
    }
}
#[doc = "Field `OADFLT` writer - GTIOCnA Pin Output Value Setting at the Count Stop"]
pub type OADFLT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTIOR_SPEC, OADFLT_A, O>;
impl<'a, const O: u8> OADFLT_W<'a, O> {
    #[doc = "The GTIOCnA pin outputs low when counting stops"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OADFLT_A::_0)
    }
    #[doc = "The GTIOCnA pin outputs high when counting stops"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OADFLT_A::_1)
    }
}
#[doc = "Field `OAHLD` reader - GTIOCnA Pin Output Setting at the Start/Stop Count"]
pub type OAHLD_R = crate::BitReader<OAHLD_A>;
#[doc = "GTIOCnA Pin Output Setting at the Start/Stop Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OAHLD_A {
    #[doc = "0: The GTIOCnA pin output level at the start or stop of counting depends on the register setting"]
    _0 = 0,
    #[doc = "1: The GTIOCnA pin output level is retained at the start or stop of counting"]
    _1 = 1,
}
impl From<OAHLD_A> for bool {
    #[inline(always)]
    fn from(variant: OAHLD_A) -> Self {
        variant as u8 != 0
    }
}
impl OAHLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OAHLD_A {
        match self.bits {
            false => OAHLD_A::_0,
            true => OAHLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OAHLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OAHLD_A::_1
    }
}
#[doc = "Field `OAHLD` writer - GTIOCnA Pin Output Setting at the Start/Stop Count"]
pub type OAHLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTIOR_SPEC, OAHLD_A, O>;
impl<'a, const O: u8> OAHLD_W<'a, O> {
    #[doc = "The GTIOCnA pin output level at the start or stop of counting depends on the register setting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OAHLD_A::_0)
    }
    #[doc = "The GTIOCnA pin output level is retained at the start or stop of counting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OAHLD_A::_1)
    }
}
#[doc = "Field `OAE` reader - GTIOCnA Pin Output Enable"]
pub type OAE_R = crate::BitReader<OAE_A>;
#[doc = "GTIOCnA Pin Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OAE_A {
    #[doc = "0: Output is disabled"]
    _0 = 0,
    #[doc = "1: Output is enabled"]
    _1 = 1,
}
impl From<OAE_A> for bool {
    #[inline(always)]
    fn from(variant: OAE_A) -> Self {
        variant as u8 != 0
    }
}
impl OAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OAE_A {
        match self.bits {
            false => OAE_A::_0,
            true => OAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OAE_A::_1
    }
}
#[doc = "Field `OAE` writer - GTIOCnA Pin Output Enable"]
pub type OAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTIOR_SPEC, OAE_A, O>;
impl<'a, const O: u8> OAE_W<'a, O> {
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OAE_A::_0)
    }
    #[doc = "Output is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OAE_A::_1)
    }
}
#[doc = "Field `OADF` reader - GTIOCnA Pin Disable Value Setting"]
pub type OADF_R = crate::FieldReader<u8, OADF_A>;
#[doc = "GTIOCnA Pin Disable Value Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OADF_A {
    #[doc = "0: None of the below options are specified"]
    _00 = 0,
    #[doc = "1: GTIOCnA pin is set to Hi-Z in response to controlling the output negation"]
    _01 = 1,
    #[doc = "2: GTIOCnA pin is set to 0 in response to controlling the output negation"]
    _10 = 2,
    #[doc = "3: GTIOCnA pin is set to 1 in response to controlling the output negation"]
    _11 = 3,
}
impl From<OADF_A> for u8 {
    #[inline(always)]
    fn from(variant: OADF_A) -> Self {
        variant as _
    }
}
impl OADF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OADF_A {
        match self.bits {
            0 => OADF_A::_00,
            1 => OADF_A::_01,
            2 => OADF_A::_10,
            3 => OADF_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OADF_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OADF_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OADF_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OADF_A::_11
    }
}
#[doc = "Field `OADF` writer - GTIOCnA Pin Disable Value Setting"]
pub type OADF_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTIOR_SPEC, u8, OADF_A, 2, O>;
impl<'a, const O: u8> OADF_W<'a, O> {
    #[doc = "None of the below options are specified"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(OADF_A::_00)
    }
    #[doc = "GTIOCnA pin is set to Hi-Z in response to controlling the output negation"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(OADF_A::_01)
    }
    #[doc = "GTIOCnA pin is set to 0 in response to controlling the output negation"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(OADF_A::_10)
    }
    #[doc = "GTIOCnA pin is set to 1 in response to controlling the output negation"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(OADF_A::_11)
    }
}
#[doc = "Field `NFAEN` reader - Noise Filter A Enable"]
pub type NFAEN_R = crate::BitReader<NFAEN_A>;
#[doc = "Noise Filter A Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NFAEN_A {
    #[doc = "0: The noise filter for the GTIOCnA pin is disabled"]
    _0 = 0,
    #[doc = "1: The noise filter for the GTIOCnA pin is enabled"]
    _1 = 1,
}
impl From<NFAEN_A> for bool {
    #[inline(always)]
    fn from(variant: NFAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl NFAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NFAEN_A {
        match self.bits {
            false => NFAEN_A::_0,
            true => NFAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NFAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NFAEN_A::_1
    }
}
#[doc = "Field `NFAEN` writer - Noise Filter A Enable"]
pub type NFAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTIOR_SPEC, NFAEN_A, O>;
impl<'a, const O: u8> NFAEN_W<'a, O> {
    #[doc = "The noise filter for the GTIOCnA pin is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NFAEN_A::_0)
    }
    #[doc = "The noise filter for the GTIOCnA pin is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NFAEN_A::_1)
    }
}
#[doc = "Field `NFCSA` reader - Noise Filter A Sampling Clock Select"]
pub type NFCSA_R = crate::FieldReader<u8, NFCSA_A>;
#[doc = "Noise Filter A Sampling Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NFCSA_A {
    #[doc = "0: PCLKD/1"]
    _00 = 0,
    #[doc = "1: PCLKD/4"]
    _01 = 1,
    #[doc = "2: PCLKD/16"]
    _10 = 2,
    #[doc = "3: PCLKD/64"]
    _11 = 3,
}
impl From<NFCSA_A> for u8 {
    #[inline(always)]
    fn from(variant: NFCSA_A) -> Self {
        variant as _
    }
}
impl NFCSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NFCSA_A {
        match self.bits {
            0 => NFCSA_A::_00,
            1 => NFCSA_A::_01,
            2 => NFCSA_A::_10,
            3 => NFCSA_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == NFCSA_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == NFCSA_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == NFCSA_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == NFCSA_A::_11
    }
}
#[doc = "Field `NFCSA` writer - Noise Filter A Sampling Clock Select"]
pub type NFCSA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTIOR_SPEC, u8, NFCSA_A, 2, O>;
impl<'a, const O: u8> NFCSA_W<'a, O> {
    #[doc = "PCLKD/1"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(NFCSA_A::_00)
    }
    #[doc = "PCLKD/4"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(NFCSA_A::_01)
    }
    #[doc = "PCLKD/16"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(NFCSA_A::_10)
    }
    #[doc = "PCLKD/64"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(NFCSA_A::_11)
    }
}
#[doc = "Field `GTIOB` reader - GTIOCnB Pin Function Select"]
pub type GTIOB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GTIOB` writer - GTIOCnB Pin Function Select"]
pub type GTIOB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTIOR_SPEC, u8, u8, 5, O>;
#[doc = "Field `OBDFLT` reader - GTIOCnB Pin Output Value Setting at the Count Stop"]
pub type OBDFLT_R = crate::BitReader<OBDFLT_A>;
#[doc = "GTIOCnB Pin Output Value Setting at the Count Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBDFLT_A {
    #[doc = "0: The GTIOCnB pin outputs low when counting stops"]
    _0 = 0,
    #[doc = "1: The GTIOCnB pin outputs high when counting stops"]
    _1 = 1,
}
impl From<OBDFLT_A> for bool {
    #[inline(always)]
    fn from(variant: OBDFLT_A) -> Self {
        variant as u8 != 0
    }
}
impl OBDFLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OBDFLT_A {
        match self.bits {
            false => OBDFLT_A::_0,
            true => OBDFLT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OBDFLT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OBDFLT_A::_1
    }
}
#[doc = "Field `OBDFLT` writer - GTIOCnB Pin Output Value Setting at the Count Stop"]
pub type OBDFLT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTIOR_SPEC, OBDFLT_A, O>;
impl<'a, const O: u8> OBDFLT_W<'a, O> {
    #[doc = "The GTIOCnB pin outputs low when counting stops"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OBDFLT_A::_0)
    }
    #[doc = "The GTIOCnB pin outputs high when counting stops"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OBDFLT_A::_1)
    }
}
#[doc = "Field `OBHLD` reader - GTIOCnB Pin Output Setting at the Start/Stop Count"]
pub type OBHLD_R = crate::BitReader<OBHLD_A>;
#[doc = "GTIOCnB Pin Output Setting at the Start/Stop Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBHLD_A {
    #[doc = "0: The GTIOCnB pin output level at the start/stop of counting depends on the register setting"]
    _0 = 0,
    #[doc = "1: The GTIOCnB pin output level is retained at the start/stop of counting"]
    _1 = 1,
}
impl From<OBHLD_A> for bool {
    #[inline(always)]
    fn from(variant: OBHLD_A) -> Self {
        variant as u8 != 0
    }
}
impl OBHLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OBHLD_A {
        match self.bits {
            false => OBHLD_A::_0,
            true => OBHLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OBHLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OBHLD_A::_1
    }
}
#[doc = "Field `OBHLD` writer - GTIOCnB Pin Output Setting at the Start/Stop Count"]
pub type OBHLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTIOR_SPEC, OBHLD_A, O>;
impl<'a, const O: u8> OBHLD_W<'a, O> {
    #[doc = "The GTIOCnB pin output level at the start/stop of counting depends on the register setting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OBHLD_A::_0)
    }
    #[doc = "The GTIOCnB pin output level is retained at the start/stop of counting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OBHLD_A::_1)
    }
}
#[doc = "Field `OBE` reader - GTIOCnB Pin Output Enable"]
pub type OBE_R = crate::BitReader<OBE_A>;
#[doc = "GTIOCnB Pin Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBE_A {
    #[doc = "0: Output is disabled"]
    _0 = 0,
    #[doc = "1: Output is enabled"]
    _1 = 1,
}
impl From<OBE_A> for bool {
    #[inline(always)]
    fn from(variant: OBE_A) -> Self {
        variant as u8 != 0
    }
}
impl OBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OBE_A {
        match self.bits {
            false => OBE_A::_0,
            true => OBE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OBE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OBE_A::_1
    }
}
#[doc = "Field `OBE` writer - GTIOCnB Pin Output Enable"]
pub type OBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTIOR_SPEC, OBE_A, O>;
impl<'a, const O: u8> OBE_W<'a, O> {
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OBE_A::_0)
    }
    #[doc = "Output is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OBE_A::_1)
    }
}
#[doc = "Field `OBDF` reader - GTIOCnB Pin Disable Value Setting"]
pub type OBDF_R = crate::FieldReader<u8, OBDF_A>;
#[doc = "GTIOCnB Pin Disable Value Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OBDF_A {
    #[doc = "0: None of the below options are specified"]
    _00 = 0,
    #[doc = "1: GTIOCnB pin is set to Hi-Z in response to controlling the output negation"]
    _01 = 1,
    #[doc = "2: GTIOCnB pin is set to 0 in response to controlling the output negation"]
    _10 = 2,
    #[doc = "3: GTIOCnB pin is set to 1 in response to controlling the output negation"]
    _11 = 3,
}
impl From<OBDF_A> for u8 {
    #[inline(always)]
    fn from(variant: OBDF_A) -> Self {
        variant as _
    }
}
impl OBDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OBDF_A {
        match self.bits {
            0 => OBDF_A::_00,
            1 => OBDF_A::_01,
            2 => OBDF_A::_10,
            3 => OBDF_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OBDF_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OBDF_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OBDF_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OBDF_A::_11
    }
}
#[doc = "Field `OBDF` writer - GTIOCnB Pin Disable Value Setting"]
pub type OBDF_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTIOR_SPEC, u8, OBDF_A, 2, O>;
impl<'a, const O: u8> OBDF_W<'a, O> {
    #[doc = "None of the below options are specified"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(OBDF_A::_00)
    }
    #[doc = "GTIOCnB pin is set to Hi-Z in response to controlling the output negation"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(OBDF_A::_01)
    }
    #[doc = "GTIOCnB pin is set to 0 in response to controlling the output negation"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(OBDF_A::_10)
    }
    #[doc = "GTIOCnB pin is set to 1 in response to controlling the output negation"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(OBDF_A::_11)
    }
}
#[doc = "Field `NFBEN` reader - Noise Filter B Enable"]
pub type NFBEN_R = crate::BitReader<NFBEN_A>;
#[doc = "Noise Filter B Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NFBEN_A {
    #[doc = "0: The noise filter for the GTIOCnB pin is disabled"]
    _0 = 0,
    #[doc = "1: The noise filter for the GTIOCnB pin is enabled"]
    _1 = 1,
}
impl From<NFBEN_A> for bool {
    #[inline(always)]
    fn from(variant: NFBEN_A) -> Self {
        variant as u8 != 0
    }
}
impl NFBEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NFBEN_A {
        match self.bits {
            false => NFBEN_A::_0,
            true => NFBEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NFBEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NFBEN_A::_1
    }
}
#[doc = "Field `NFBEN` writer - Noise Filter B Enable"]
pub type NFBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTIOR_SPEC, NFBEN_A, O>;
impl<'a, const O: u8> NFBEN_W<'a, O> {
    #[doc = "The noise filter for the GTIOCnB pin is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NFBEN_A::_0)
    }
    #[doc = "The noise filter for the GTIOCnB pin is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NFBEN_A::_1)
    }
}
#[doc = "Field `NFCSB` reader - Noise Filter B Sampling Clock Select"]
pub type NFCSB_R = crate::FieldReader<u8, NFCSB_A>;
#[doc = "Noise Filter B Sampling Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NFCSB_A {
    #[doc = "0: PCLKD/1"]
    _00 = 0,
    #[doc = "1: PCLKD/4"]
    _01 = 1,
    #[doc = "2: PCLKD/16"]
    _10 = 2,
    #[doc = "3: PCLKD/64"]
    _11 = 3,
}
impl From<NFCSB_A> for u8 {
    #[inline(always)]
    fn from(variant: NFCSB_A) -> Self {
        variant as _
    }
}
impl NFCSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NFCSB_A {
        match self.bits {
            0 => NFCSB_A::_00,
            1 => NFCSB_A::_01,
            2 => NFCSB_A::_10,
            3 => NFCSB_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == NFCSB_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == NFCSB_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == NFCSB_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == NFCSB_A::_11
    }
}
#[doc = "Field `NFCSB` writer - Noise Filter B Sampling Clock Select"]
pub type NFCSB_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTIOR_SPEC, u8, NFCSB_A, 2, O>;
impl<'a, const O: u8> NFCSB_W<'a, O> {
    #[doc = "PCLKD/1"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(NFCSB_A::_00)
    }
    #[doc = "PCLKD/4"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(NFCSB_A::_01)
    }
    #[doc = "PCLKD/16"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(NFCSB_A::_10)
    }
    #[doc = "PCLKD/64"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(NFCSB_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:4 - GTIOCnA Pin Function Select"]
    #[inline(always)]
    pub fn gtioa(&self) -> GTIOA_R {
        GTIOA_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - GTIOCnA Pin Output Value Setting at the Count Stop"]
    #[inline(always)]
    pub fn oadflt(&self) -> OADFLT_R {
        OADFLT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GTIOCnA Pin Output Setting at the Start/Stop Count"]
    #[inline(always)]
    pub fn oahld(&self) -> OAHLD_R {
        OAHLD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GTIOCnA Pin Output Enable"]
    #[inline(always)]
    pub fn oae(&self) -> OAE_R {
        OAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - GTIOCnA Pin Disable Value Setting"]
    #[inline(always)]
    pub fn oadf(&self) -> OADF_R {
        OADF_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 13 - Noise Filter A Enable"]
    #[inline(always)]
    pub fn nfaen(&self) -> NFAEN_R {
        NFAEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Noise Filter A Sampling Clock Select"]
    #[inline(always)]
    pub fn nfcsa(&self) -> NFCSA_R {
        NFCSA_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:20 - GTIOCnB Pin Function Select"]
    #[inline(always)]
    pub fn gtiob(&self) -> GTIOB_R {
        GTIOB_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - GTIOCnB Pin Output Value Setting at the Count Stop"]
    #[inline(always)]
    pub fn obdflt(&self) -> OBDFLT_R {
        OBDFLT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GTIOCnB Pin Output Setting at the Start/Stop Count"]
    #[inline(always)]
    pub fn obhld(&self) -> OBHLD_R {
        OBHLD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GTIOCnB Pin Output Enable"]
    #[inline(always)]
    pub fn obe(&self) -> OBE_R {
        OBE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - GTIOCnB Pin Disable Value Setting"]
    #[inline(always)]
    pub fn obdf(&self) -> OBDF_R {
        OBDF_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 29 - Noise Filter B Enable"]
    #[inline(always)]
    pub fn nfben(&self) -> NFBEN_R {
        NFBEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Noise Filter B Sampling Clock Select"]
    #[inline(always)]
    pub fn nfcsb(&self) -> NFCSB_R {
        NFCSB_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - GTIOCnA Pin Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn gtioa(&mut self) -> GTIOA_W<0> {
        GTIOA_W::new(self)
    }
    #[doc = "Bit 6 - GTIOCnA Pin Output Value Setting at the Count Stop"]
    #[inline(always)]
    #[must_use]
    pub fn oadflt(&mut self) -> OADFLT_W<6> {
        OADFLT_W::new(self)
    }
    #[doc = "Bit 7 - GTIOCnA Pin Output Setting at the Start/Stop Count"]
    #[inline(always)]
    #[must_use]
    pub fn oahld(&mut self) -> OAHLD_W<7> {
        OAHLD_W::new(self)
    }
    #[doc = "Bit 8 - GTIOCnA Pin Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn oae(&mut self) -> OAE_W<8> {
        OAE_W::new(self)
    }
    #[doc = "Bits 9:10 - GTIOCnA Pin Disable Value Setting"]
    #[inline(always)]
    #[must_use]
    pub fn oadf(&mut self) -> OADF_W<9> {
        OADF_W::new(self)
    }
    #[doc = "Bit 13 - Noise Filter A Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nfaen(&mut self) -> NFAEN_W<13> {
        NFAEN_W::new(self)
    }
    #[doc = "Bits 14:15 - Noise Filter A Sampling Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn nfcsa(&mut self) -> NFCSA_W<14> {
        NFCSA_W::new(self)
    }
    #[doc = "Bits 16:20 - GTIOCnB Pin Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn gtiob(&mut self) -> GTIOB_W<16> {
        GTIOB_W::new(self)
    }
    #[doc = "Bit 22 - GTIOCnB Pin Output Value Setting at the Count Stop"]
    #[inline(always)]
    #[must_use]
    pub fn obdflt(&mut self) -> OBDFLT_W<22> {
        OBDFLT_W::new(self)
    }
    #[doc = "Bit 23 - GTIOCnB Pin Output Setting at the Start/Stop Count"]
    #[inline(always)]
    #[must_use]
    pub fn obhld(&mut self) -> OBHLD_W<23> {
        OBHLD_W::new(self)
    }
    #[doc = "Bit 24 - GTIOCnB Pin Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn obe(&mut self) -> OBE_W<24> {
        OBE_W::new(self)
    }
    #[doc = "Bits 25:26 - GTIOCnB Pin Disable Value Setting"]
    #[inline(always)]
    #[must_use]
    pub fn obdf(&mut self) -> OBDF_W<25> {
        OBDF_W::new(self)
    }
    #[doc = "Bit 29 - Noise Filter B Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nfben(&mut self) -> NFBEN_W<29> {
        NFBEN_W::new(self)
    }
    #[doc = "Bits 30:31 - Noise Filter B Sampling Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn nfcsb(&mut self) -> NFCSB_W<30> {
        NFCSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer I/O Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtior](index.html) module"]
pub struct GTIOR_SPEC;
impl crate::RegisterSpec for GTIOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtior::R](R) reader structure"]
impl crate::Readable for GTIOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtior::W](W) writer structure"]
impl crate::Writable for GTIOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTIOR to value 0"]
impl crate::Resettable for GTIOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
