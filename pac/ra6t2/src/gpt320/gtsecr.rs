#[doc = "Register `GTSECR` reader"]
pub struct R(crate::R<GTSECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTSECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTSECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTSECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTSECR` writer"]
pub struct W(crate::W<GTSECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTSECR_SPEC>;
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
impl From<crate::W<GTSECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTSECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBDCE` reader - GTCCR Register Buffer Operation Simultaneous Enable"]
pub type SBDCE_R = crate::BitReader<SBDCE_A>;
#[doc = "GTCCR Register Buffer Operation Simultaneous Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBDCE_A {
    #[doc = "0: Disable simultaneous enabling GTCCR buffer operations"]
    _0 = 0,
    #[doc = "1: Enable GTCCR register buffer operations simultaneously"]
    _1 = 1,
}
impl From<SBDCE_A> for bool {
    #[inline(always)]
    fn from(variant: SBDCE_A) -> Self {
        variant as u8 != 0
    }
}
impl SBDCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBDCE_A {
        match self.bits {
            false => SBDCE_A::_0,
            true => SBDCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBDCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBDCE_A::_1
    }
}
#[doc = "Field `SBDCE` writer - GTCCR Register Buffer Operation Simultaneous Enable"]
pub type SBDCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECR_SPEC, SBDCE_A, O>;
impl<'a, const O: u8> SBDCE_W<'a, O> {
    #[doc = "Disable simultaneous enabling GTCCR buffer operations"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBDCE_A::_0)
    }
    #[doc = "Enable GTCCR register buffer operations simultaneously"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBDCE_A::_1)
    }
}
#[doc = "Field `SBDPE` reader - GTPR Register Buffer Operation Simultaneous Enable"]
pub type SBDPE_R = crate::BitReader<SBDPE_A>;
#[doc = "GTPR Register Buffer Operation Simultaneous Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBDPE_A {
    #[doc = "0: Disable simultaneous enabling GTPR buffer operations"]
    _0 = 0,
    #[doc = "1: Enable GTPR register buffer operations simultaneously"]
    _1 = 1,
}
impl From<SBDPE_A> for bool {
    #[inline(always)]
    fn from(variant: SBDPE_A) -> Self {
        variant as u8 != 0
    }
}
impl SBDPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBDPE_A {
        match self.bits {
            false => SBDPE_A::_0,
            true => SBDPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBDPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBDPE_A::_1
    }
}
#[doc = "Field `SBDPE` writer - GTPR Register Buffer Operation Simultaneous Enable"]
pub type SBDPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECR_SPEC, SBDPE_A, O>;
impl<'a, const O: u8> SBDPE_W<'a, O> {
    #[doc = "Disable simultaneous enabling GTPR buffer operations"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBDPE_A::_0)
    }
    #[doc = "Enable GTPR register buffer operations simultaneously"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBDPE_A::_1)
    }
}
#[doc = "Field `SBDAE` reader - GTADTR Register Buffer Operation Simultaneous Enable"]
pub type SBDAE_R = crate::BitReader<SBDAE_A>;
#[doc = "GTADTR Register Buffer Operation Simultaneous Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBDAE_A {
    #[doc = "0: Disable simultaneous enabling GTADTR buffer operations"]
    _0 = 0,
    #[doc = "1: Enable GTADTR register buffer operations simultaneously"]
    _1 = 1,
}
impl From<SBDAE_A> for bool {
    #[inline(always)]
    fn from(variant: SBDAE_A) -> Self {
        variant as u8 != 0
    }
}
impl SBDAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBDAE_A {
        match self.bits {
            false => SBDAE_A::_0,
            true => SBDAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBDAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBDAE_A::_1
    }
}
#[doc = "Field `SBDAE` writer - GTADTR Register Buffer Operation Simultaneous Enable"]
pub type SBDAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECR_SPEC, SBDAE_A, O>;
impl<'a, const O: u8> SBDAE_W<'a, O> {
    #[doc = "Disable simultaneous enabling GTADTR buffer operations"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBDAE_A::_0)
    }
    #[doc = "Enable GTADTR register buffer operations simultaneously"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBDAE_A::_1)
    }
}
#[doc = "Field `SBDDE` reader - GTDV Register Buffer Operation Simultaneous Enable"]
pub type SBDDE_R = crate::BitReader<SBDDE_A>;
#[doc = "GTDV Register Buffer Operation Simultaneous Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBDDE_A {
    #[doc = "0: Disable simultaneous enabling GTDV buffer operations"]
    _0 = 0,
    #[doc = "1: Enable GTDV register buffer operations simultaneously"]
    _1 = 1,
}
impl From<SBDDE_A> for bool {
    #[inline(always)]
    fn from(variant: SBDDE_A) -> Self {
        variant as u8 != 0
    }
}
impl SBDDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBDDE_A {
        match self.bits {
            false => SBDDE_A::_0,
            true => SBDDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBDDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBDDE_A::_1
    }
}
#[doc = "Field `SBDDE` writer - GTDV Register Buffer Operation Simultaneous Enable"]
pub type SBDDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECR_SPEC, SBDDE_A, O>;
impl<'a, const O: u8> SBDDE_W<'a, O> {
    #[doc = "Disable simultaneous enabling GTDV buffer operations"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBDDE_A::_0)
    }
    #[doc = "Enable GTDV register buffer operations simultaneously"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBDDE_A::_1)
    }
}
#[doc = "Field `SBDCD` reader - GTCCR Register Buffer Operation Simultaneous Disable"]
pub type SBDCD_R = crate::BitReader<SBDCD_A>;
#[doc = "GTCCR Register Buffer Operation Simultaneous Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBDCD_A {
    #[doc = "0: Disable simultaneous disabling GTCCR buffer operations"]
    _0 = 0,
    #[doc = "1: Disable GTCCR register buffer operations simultaneously"]
    _1 = 1,
}
impl From<SBDCD_A> for bool {
    #[inline(always)]
    fn from(variant: SBDCD_A) -> Self {
        variant as u8 != 0
    }
}
impl SBDCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBDCD_A {
        match self.bits {
            false => SBDCD_A::_0,
            true => SBDCD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBDCD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBDCD_A::_1
    }
}
#[doc = "Field `SBDCD` writer - GTCCR Register Buffer Operation Simultaneous Disable"]
pub type SBDCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECR_SPEC, SBDCD_A, O>;
impl<'a, const O: u8> SBDCD_W<'a, O> {
    #[doc = "Disable simultaneous disabling GTCCR buffer operations"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBDCD_A::_0)
    }
    #[doc = "Disable GTCCR register buffer operations simultaneously"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBDCD_A::_1)
    }
}
#[doc = "Field `SBDPD` reader - GTPR Register Buffer Operation Simultaneous Disable"]
pub type SBDPD_R = crate::BitReader<SBDPD_A>;
#[doc = "GTPR Register Buffer Operation Simultaneous Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBDPD_A {
    #[doc = "0: Disable simultaneous disabling GTPR buffer operations"]
    _0 = 0,
    #[doc = "1: Disable GTPR register buffer operations simultaneously"]
    _1 = 1,
}
impl From<SBDPD_A> for bool {
    #[inline(always)]
    fn from(variant: SBDPD_A) -> Self {
        variant as u8 != 0
    }
}
impl SBDPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBDPD_A {
        match self.bits {
            false => SBDPD_A::_0,
            true => SBDPD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBDPD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBDPD_A::_1
    }
}
#[doc = "Field `SBDPD` writer - GTPR Register Buffer Operation Simultaneous Disable"]
pub type SBDPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECR_SPEC, SBDPD_A, O>;
impl<'a, const O: u8> SBDPD_W<'a, O> {
    #[doc = "Disable simultaneous disabling GTPR buffer operations"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBDPD_A::_0)
    }
    #[doc = "Disable GTPR register buffer operations simultaneously"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBDPD_A::_1)
    }
}
#[doc = "Field `SBDAD` reader - GTADTR Register Buffer Operation Simultaneous Disable"]
pub type SBDAD_R = crate::BitReader<SBDAD_A>;
#[doc = "GTADTR Register Buffer Operation Simultaneous Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBDAD_A {
    #[doc = "0: Disable simultaneous disabling GTADTR buffer operations"]
    _0 = 0,
    #[doc = "1: Disable GTADTR register buffer operations simultaneously"]
    _1 = 1,
}
impl From<SBDAD_A> for bool {
    #[inline(always)]
    fn from(variant: SBDAD_A) -> Self {
        variant as u8 != 0
    }
}
impl SBDAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBDAD_A {
        match self.bits {
            false => SBDAD_A::_0,
            true => SBDAD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBDAD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBDAD_A::_1
    }
}
#[doc = "Field `SBDAD` writer - GTADTR Register Buffer Operation Simultaneous Disable"]
pub type SBDAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECR_SPEC, SBDAD_A, O>;
impl<'a, const O: u8> SBDAD_W<'a, O> {
    #[doc = "Disable simultaneous disabling GTADTR buffer operations"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBDAD_A::_0)
    }
    #[doc = "Disable GTADTR register buffer operations simultaneously"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBDAD_A::_1)
    }
}
#[doc = "Field `SBDDD` reader - GTDV Register Buffer Operation Simultaneous Disable"]
pub type SBDDD_R = crate::BitReader<SBDDD_A>;
#[doc = "GTDV Register Buffer Operation Simultaneous Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBDDD_A {
    #[doc = "0: Disable simultaneous disabling GTDV buffer operations"]
    _0 = 0,
    #[doc = "1: Disable GTDV register buffer operations simultaneously"]
    _1 = 1,
}
impl From<SBDDD_A> for bool {
    #[inline(always)]
    fn from(variant: SBDDD_A) -> Self {
        variant as u8 != 0
    }
}
impl SBDDD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBDDD_A {
        match self.bits {
            false => SBDDD_A::_0,
            true => SBDDD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBDDD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBDDD_A::_1
    }
}
#[doc = "Field `SBDDD` writer - GTDV Register Buffer Operation Simultaneous Disable"]
pub type SBDDD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECR_SPEC, SBDDD_A, O>;
impl<'a, const O: u8> SBDDD_W<'a, O> {
    #[doc = "Disable simultaneous disabling GTDV buffer operations"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBDDD_A::_0)
    }
    #[doc = "Disable GTDV register buffer operations simultaneously"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBDDD_A::_1)
    }
}
#[doc = "Field `SPCE` reader - Period Count Function Simultaneous Enable"]
pub type SPCE_R = crate::BitReader<SPCE_A>;
#[doc = "Period Count Function Simultaneous Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPCE_A {
    #[doc = "0: Disable simultaneous enabling period count function"]
    _0 = 0,
    #[doc = "1: Enable period count function simultaneously"]
    _1 = 1,
}
impl From<SPCE_A> for bool {
    #[inline(always)]
    fn from(variant: SPCE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPCE_A {
        match self.bits {
            false => SPCE_A::_0,
            true => SPCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPCE_A::_1
    }
}
#[doc = "Field `SPCE` writer - Period Count Function Simultaneous Enable"]
pub type SPCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECR_SPEC, SPCE_A, O>;
impl<'a, const O: u8> SPCE_W<'a, O> {
    #[doc = "Disable simultaneous enabling period count function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPCE_A::_0)
    }
    #[doc = "Enable period count function simultaneously"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPCE_A::_1)
    }
}
#[doc = "Field `SSCE` reader - Synchronous Set/Clear Simultaneous Enable"]
pub type SSCE_R = crate::BitReader<SSCE_A>;
#[doc = "Synchronous Set/Clear Simultaneous Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSCE_A {
    #[doc = "0: Disable simultaneous enabling synchronous set/clear"]
    _0 = 0,
    #[doc = "1: Enable synchronous set/clear simultaneously"]
    _1 = 1,
}
impl From<SSCE_A> for bool {
    #[inline(always)]
    fn from(variant: SSCE_A) -> Self {
        variant as u8 != 0
    }
}
impl SSCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSCE_A {
        match self.bits {
            false => SSCE_A::_0,
            true => SSCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSCE_A::_1
    }
}
#[doc = "Field `SSCE` writer - Synchronous Set/Clear Simultaneous Enable"]
pub type SSCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECR_SPEC, SSCE_A, O>;
impl<'a, const O: u8> SSCE_W<'a, O> {
    #[doc = "Disable simultaneous enabling synchronous set/clear"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSCE_A::_0)
    }
    #[doc = "Enable synchronous set/clear simultaneously"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSCE_A::_1)
    }
}
#[doc = "Field `SPCD` reader - Period Count Function Simultaneous Disable"]
pub type SPCD_R = crate::BitReader<SPCD_A>;
#[doc = "Period Count Function Simultaneous Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPCD_A {
    #[doc = "0: Disable simultaneous disabling period count function"]
    _0 = 0,
    #[doc = "1: Disable period count function simultaneously"]
    _1 = 1,
}
impl From<SPCD_A> for bool {
    #[inline(always)]
    fn from(variant: SPCD_A) -> Self {
        variant as u8 != 0
    }
}
impl SPCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPCD_A {
        match self.bits {
            false => SPCD_A::_0,
            true => SPCD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPCD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPCD_A::_1
    }
}
#[doc = "Field `SPCD` writer - Period Count Function Simultaneous Disable"]
pub type SPCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECR_SPEC, SPCD_A, O>;
impl<'a, const O: u8> SPCD_W<'a, O> {
    #[doc = "Disable simultaneous disabling period count function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPCD_A::_0)
    }
    #[doc = "Disable period count function simultaneously"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPCD_A::_1)
    }
}
#[doc = "Field `SSCD` reader - Synchronous Set/Clear Simultaneous Disable"]
pub type SSCD_R = crate::BitReader<SSCD_A>;
#[doc = "Synchronous Set/Clear Simultaneous Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSCD_A {
    #[doc = "0: Disable simultaneous disabling synchronous set/clear"]
    _0 = 0,
    #[doc = "1: Disable synchronous set/clear simultaneously"]
    _1 = 1,
}
impl From<SSCD_A> for bool {
    #[inline(always)]
    fn from(variant: SSCD_A) -> Self {
        variant as u8 != 0
    }
}
impl SSCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSCD_A {
        match self.bits {
            false => SSCD_A::_0,
            true => SSCD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSCD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSCD_A::_1
    }
}
#[doc = "Field `SSCD` writer - Synchronous Set/Clear Simultaneous Disable"]
pub type SSCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECR_SPEC, SSCD_A, O>;
impl<'a, const O: u8> SSCD_W<'a, O> {
    #[doc = "Disable simultaneous disabling synchronous set/clear"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSCD_A::_0)
    }
    #[doc = "Disable synchronous set/clear simultaneously"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSCD_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - GTCCR Register Buffer Operation Simultaneous Enable"]
    #[inline(always)]
    pub fn sbdce(&self) -> SBDCE_R {
        SBDCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GTPR Register Buffer Operation Simultaneous Enable"]
    #[inline(always)]
    pub fn sbdpe(&self) -> SBDPE_R {
        SBDPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GTADTR Register Buffer Operation Simultaneous Enable"]
    #[inline(always)]
    pub fn sbdae(&self) -> SBDAE_R {
        SBDAE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GTDV Register Buffer Operation Simultaneous Enable"]
    #[inline(always)]
    pub fn sbdde(&self) -> SBDDE_R {
        SBDDE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - GTCCR Register Buffer Operation Simultaneous Disable"]
    #[inline(always)]
    pub fn sbdcd(&self) -> SBDCD_R {
        SBDCD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GTPR Register Buffer Operation Simultaneous Disable"]
    #[inline(always)]
    pub fn sbdpd(&self) -> SBDPD_R {
        SBDPD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GTADTR Register Buffer Operation Simultaneous Disable"]
    #[inline(always)]
    pub fn sbdad(&self) -> SBDAD_R {
        SBDAD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GTDV Register Buffer Operation Simultaneous Disable"]
    #[inline(always)]
    pub fn sbddd(&self) -> SBDDD_R {
        SBDDD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Period Count Function Simultaneous Enable"]
    #[inline(always)]
    pub fn spce(&self) -> SPCE_R {
        SPCE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Synchronous Set/Clear Simultaneous Enable"]
    #[inline(always)]
    pub fn ssce(&self) -> SSCE_R {
        SSCE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - Period Count Function Simultaneous Disable"]
    #[inline(always)]
    pub fn spcd(&self) -> SPCD_R {
        SPCD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Synchronous Set/Clear Simultaneous Disable"]
    #[inline(always)]
    pub fn sscd(&self) -> SSCD_R {
        SSCD_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GTCCR Register Buffer Operation Simultaneous Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sbdce(&mut self) -> SBDCE_W<0> {
        SBDCE_W::new(self)
    }
    #[doc = "Bit 1 - GTPR Register Buffer Operation Simultaneous Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sbdpe(&mut self) -> SBDPE_W<1> {
        SBDPE_W::new(self)
    }
    #[doc = "Bit 2 - GTADTR Register Buffer Operation Simultaneous Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sbdae(&mut self) -> SBDAE_W<2> {
        SBDAE_W::new(self)
    }
    #[doc = "Bit 3 - GTDV Register Buffer Operation Simultaneous Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sbdde(&mut self) -> SBDDE_W<3> {
        SBDDE_W::new(self)
    }
    #[doc = "Bit 8 - GTCCR Register Buffer Operation Simultaneous Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sbdcd(&mut self) -> SBDCD_W<8> {
        SBDCD_W::new(self)
    }
    #[doc = "Bit 9 - GTPR Register Buffer Operation Simultaneous Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sbdpd(&mut self) -> SBDPD_W<9> {
        SBDPD_W::new(self)
    }
    #[doc = "Bit 10 - GTADTR Register Buffer Operation Simultaneous Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sbdad(&mut self) -> SBDAD_W<10> {
        SBDAD_W::new(self)
    }
    #[doc = "Bit 11 - GTDV Register Buffer Operation Simultaneous Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sbddd(&mut self) -> SBDDD_W<11> {
        SBDDD_W::new(self)
    }
    #[doc = "Bit 16 - Period Count Function Simultaneous Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spce(&mut self) -> SPCE_W<16> {
        SPCE_W::new(self)
    }
    #[doc = "Bit 17 - Synchronous Set/Clear Simultaneous Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssce(&mut self) -> SSCE_W<17> {
        SSCE_W::new(self)
    }
    #[doc = "Bit 24 - Period Count Function Simultaneous Disable"]
    #[inline(always)]
    #[must_use]
    pub fn spcd(&mut self) -> SPCD_W<24> {
        SPCD_W::new(self)
    }
    #[doc = "Bit 25 - Synchronous Set/Clear Simultaneous Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sscd(&mut self) -> SSCD_W<25> {
        SSCD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Operation Enable Bit Simultaneous Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtsecr](index.html) module"]
pub struct GTSECR_SPEC;
impl crate::RegisterSpec for GTSECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtsecr::R](R) reader structure"]
impl crate::Readable for GTSECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtsecr::W](W) writer structure"]
impl crate::Writable for GTSECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTSECR to value 0"]
impl crate::Resettable for GTSECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
