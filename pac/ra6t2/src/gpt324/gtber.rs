#[doc = "Register `GTBER` reader"]
pub struct R(crate::R<GTBER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTBER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTBER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTBER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTBER` writer"]
pub struct W(crate::W<GTBER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTBER_SPEC>;
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
impl From<crate::W<GTBER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTBER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BD0` reader - GTCCR Buffer Operation Disable"]
pub type BD0_R = crate::BitReader<BD0_A>;
#[doc = "GTCCR Buffer Operation Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BD0_A {
    #[doc = "0: Buffer operation is enabled"]
    _0 = 0,
    #[doc = "1: Buffer operation is disabled"]
    _1 = 1,
}
impl From<BD0_A> for bool {
    #[inline(always)]
    fn from(variant: BD0_A) -> Self {
        variant as u8 != 0
    }
}
impl BD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BD0_A {
        match self.bits {
            false => BD0_A::_0,
            true => BD0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BD0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BD0_A::_1
    }
}
#[doc = "Field `BD0` writer - GTCCR Buffer Operation Disable"]
pub type BD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTBER_SPEC, BD0_A, O>;
impl<'a, const O: u8> BD0_W<'a, O> {
    #[doc = "Buffer operation is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BD0_A::_0)
    }
    #[doc = "Buffer operation is disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BD0_A::_1)
    }
}
#[doc = "Field `BD1` reader - GTPR Buffer Operation Disable"]
pub type BD1_R = crate::BitReader<BD1_A>;
#[doc = "GTPR Buffer Operation Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BD1_A {
    #[doc = "0: Buffer operation is enabled"]
    _0 = 0,
    #[doc = "1: Buffer operation is disabled"]
    _1 = 1,
}
impl From<BD1_A> for bool {
    #[inline(always)]
    fn from(variant: BD1_A) -> Self {
        variant as u8 != 0
    }
}
impl BD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BD1_A {
        match self.bits {
            false => BD1_A::_0,
            true => BD1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BD1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BD1_A::_1
    }
}
#[doc = "Field `BD1` writer - GTPR Buffer Operation Disable"]
pub type BD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTBER_SPEC, BD1_A, O>;
impl<'a, const O: u8> BD1_W<'a, O> {
    #[doc = "Buffer operation is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BD1_A::_0)
    }
    #[doc = "Buffer operation is disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BD1_A::_1)
    }
}
#[doc = "Field `BD2` reader - GTADTRA/GTADTRB Registers Buffer Operation Disable"]
pub type BD2_R = crate::BitReader<BD2_A>;
#[doc = "GTADTRA/GTADTRB Registers Buffer Operation Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BD2_A {
    #[doc = "0: Buffer operation is enabled"]
    _0 = 0,
    #[doc = "1: Buffer operation is disabled"]
    _1 = 1,
}
impl From<BD2_A> for bool {
    #[inline(always)]
    fn from(variant: BD2_A) -> Self {
        variant as u8 != 0
    }
}
impl BD2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BD2_A {
        match self.bits {
            false => BD2_A::_0,
            true => BD2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BD2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BD2_A::_1
    }
}
#[doc = "Field `BD2` writer - GTADTRA/GTADTRB Registers Buffer Operation Disable"]
pub type BD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTBER_SPEC, BD2_A, O>;
impl<'a, const O: u8> BD2_W<'a, O> {
    #[doc = "Buffer operation is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BD2_A::_0)
    }
    #[doc = "Buffer operation is disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BD2_A::_1)
    }
}
#[doc = "Field `BD3` reader - GTDVU/GTDVD Registers Buffer Operation Disable"]
pub type BD3_R = crate::BitReader<BD3_A>;
#[doc = "GTDVU/GTDVD Registers Buffer Operation Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BD3_A {
    #[doc = "0: Buffer operation is enabled"]
    _0 = 0,
    #[doc = "1: Buffer operation is disabled"]
    _1 = 1,
}
impl From<BD3_A> for bool {
    #[inline(always)]
    fn from(variant: BD3_A) -> Self {
        variant as u8 != 0
    }
}
impl BD3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BD3_A {
        match self.bits {
            false => BD3_A::_0,
            true => BD3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BD3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BD3_A::_1
    }
}
#[doc = "Field `BD3` writer - GTDVU/GTDVD Registers Buffer Operation Disable"]
pub type BD3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTBER_SPEC, BD3_A, O>;
impl<'a, const O: u8> BD3_W<'a, O> {
    #[doc = "Buffer operation is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BD3_A::_0)
    }
    #[doc = "Buffer operation is disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BD3_A::_1)
    }
}
#[doc = "Field `DBRTECA` reader - GTCCRA Register Double Buffer Repeat Operation Enable"]
pub type DBRTECA_R = crate::BitReader<DBRTECA_A>;
#[doc = "GTCCRA Register Double Buffer Repeat Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBRTECA_A {
    #[doc = "0: GTCCRA register double buffer repeat operation is disabled"]
    _0 = 0,
    #[doc = "1: GTCCRA register double buffer repeat operation is enabled"]
    _1 = 1,
}
impl From<DBRTECA_A> for bool {
    #[inline(always)]
    fn from(variant: DBRTECA_A) -> Self {
        variant as u8 != 0
    }
}
impl DBRTECA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBRTECA_A {
        match self.bits {
            false => DBRTECA_A::_0,
            true => DBRTECA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBRTECA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBRTECA_A::_1
    }
}
#[doc = "Field `DBRTECA` writer - GTCCRA Register Double Buffer Repeat Operation Enable"]
pub type DBRTECA_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTBER_SPEC, DBRTECA_A, O>;
impl<'a, const O: u8> DBRTECA_W<'a, O> {
    #[doc = "GTCCRA register double buffer repeat operation is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBRTECA_A::_0)
    }
    #[doc = "GTCCRA register double buffer repeat operation is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBRTECA_A::_1)
    }
}
#[doc = "Field `DBRTECB` reader - GTCCRB Register Double Buffer Repeat Operation Enable"]
pub type DBRTECB_R = crate::BitReader<DBRTECB_A>;
#[doc = "GTCCRB Register Double Buffer Repeat Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBRTECB_A {
    #[doc = "0: GTCCRB register double buffer repeat operation is disabled"]
    _0 = 0,
    #[doc = "1: GTCCRB register double buffer repeat operation is enabled"]
    _1 = 1,
}
impl From<DBRTECB_A> for bool {
    #[inline(always)]
    fn from(variant: DBRTECB_A) -> Self {
        variant as u8 != 0
    }
}
impl DBRTECB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBRTECB_A {
        match self.bits {
            false => DBRTECB_A::_0,
            true => DBRTECB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBRTECB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBRTECB_A::_1
    }
}
#[doc = "Field `DBRTECB` writer - GTCCRB Register Double Buffer Repeat Operation Enable"]
pub type DBRTECB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTBER_SPEC, DBRTECB_A, O>;
impl<'a, const O: u8> DBRTECB_W<'a, O> {
    #[doc = "GTCCRB register double buffer repeat operation is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBRTECB_A::_0)
    }
    #[doc = "GTCCRB register double buffer repeat operation is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBRTECB_A::_1)
    }
}
#[doc = "Field `CCRA` reader - GTCCRA Buffer Operation"]
pub type CCRA_R = crate::FieldReader<u8, CCRA_A>;
#[doc = "GTCCRA Buffer Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CCRA_A {
    #[doc = "0: No buffer operation"]
    _00 = 0,
    #[doc = "1: Single buffer operation (GTCCRA <---->GTCCRC)"]
    _01 = 1,
}
impl From<CCRA_A> for u8 {
    #[inline(always)]
    fn from(variant: CCRA_A) -> Self {
        variant as _
    }
}
impl CCRA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CCRA_A> {
        match self.bits {
            0 => Some(CCRA_A::_00),
            1 => Some(CCRA_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CCRA_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CCRA_A::_01
    }
}
#[doc = "Field `CCRA` writer - GTCCRA Buffer Operation"]
pub type CCRA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTBER_SPEC, u8, CCRA_A, 2, O>;
impl<'a, const O: u8> CCRA_W<'a, O> {
    #[doc = "No buffer operation"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CCRA_A::_00)
    }
    #[doc = "Single buffer operation (GTCCRA <---->GTCCRC)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CCRA_A::_01)
    }
}
#[doc = "Field `CCRB` reader - GTCCRB Buffer Operation"]
pub type CCRB_R = crate::FieldReader<u8, CCRB_A>;
#[doc = "GTCCRB Buffer Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CCRB_A {
    #[doc = "0: No buffer operation"]
    _00 = 0,
    #[doc = "1: Single buffer operation (GTCCRB <----> GTCCRE)"]
    _01 = 1,
}
impl From<CCRB_A> for u8 {
    #[inline(always)]
    fn from(variant: CCRB_A) -> Self {
        variant as _
    }
}
impl CCRB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CCRB_A> {
        match self.bits {
            0 => Some(CCRB_A::_00),
            1 => Some(CCRB_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CCRB_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CCRB_A::_01
    }
}
#[doc = "Field `CCRB` writer - GTCCRB Buffer Operation"]
pub type CCRB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTBER_SPEC, u8, CCRB_A, 2, O>;
impl<'a, const O: u8> CCRB_W<'a, O> {
    #[doc = "No buffer operation"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CCRB_A::_00)
    }
    #[doc = "Single buffer operation (GTCCRB <----> GTCCRE)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CCRB_A::_01)
    }
}
#[doc = "Field `PR` reader - GTPR Buffer Operation"]
pub type PR_R = crate::FieldReader<u8, PR_A>;
#[doc = "GTPR Buffer Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PR_A {
    #[doc = "0: No buffer operation"]
    _00 = 0,
    #[doc = "1: Single buffer operation (GTPBR --> GTPR)"]
    _01 = 1,
}
impl From<PR_A> for u8 {
    #[inline(always)]
    fn from(variant: PR_A) -> Self {
        variant as _
    }
}
impl PR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PR_A> {
        match self.bits {
            0 => Some(PR_A::_00),
            1 => Some(PR_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PR_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PR_A::_01
    }
}
#[doc = "Field `PR` writer - GTPR Buffer Operation"]
pub type PR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTBER_SPEC, u8, PR_A, 2, O>;
impl<'a, const O: u8> PR_W<'a, O> {
    #[doc = "No buffer operation"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PR_A::_00)
    }
    #[doc = "Single buffer operation (GTPBR --> GTPR)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PR_A::_01)
    }
}
#[doc = "Field `CCRSWT` writer - GTCCRA and GTCCRB Forcible Buffer Operation"]
pub type CCRSWT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTBER_SPEC, bool, O>;
#[doc = "Field `ADTTA` reader - GTADTRA Register Buffer Transfer Timing Select"]
pub type ADTTA_R = crate::FieldReader<u8, ADTTA_A>;
#[doc = "GTADTRA Register Buffer Transfer Timing Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADTTA_A {
    #[doc = "0: In triangle wave or complementary PWM mode, no transfer. In saw-wave mode, no transfer."]
    _00 = 0,
    #[doc = "1: In triangle wave or complementary PWM mode, transfer at crest. In saw-wave mode, transfer at underflow (in down-counting), overflow (in up-counting), or counter clearing."]
    _01 = 1,
    #[doc = "2: In triangle wave or complementary PWM mode, transfer at trough. In saw-wave mode, transfer at underflow (in down-counting), overflow (in up-counting), or counter clearing."]
    _10 = 2,
    #[doc = "3: In triangle wave or complementary PWM mode, transfer at both crest and trough. In saw-wave mode, transfer at underflow (in down-counting), overflow (in up-counting), or counter clearing."]
    _11 = 3,
}
impl From<ADTTA_A> for u8 {
    #[inline(always)]
    fn from(variant: ADTTA_A) -> Self {
        variant as _
    }
}
impl ADTTA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADTTA_A {
        match self.bits {
            0 => ADTTA_A::_00,
            1 => ADTTA_A::_01,
            2 => ADTTA_A::_10,
            3 => ADTTA_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ADTTA_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ADTTA_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ADTTA_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ADTTA_A::_11
    }
}
#[doc = "Field `ADTTA` writer - GTADTRA Register Buffer Transfer Timing Select"]
pub type ADTTA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTBER_SPEC, u8, ADTTA_A, 2, O>;
impl<'a, const O: u8> ADTTA_W<'a, O> {
    #[doc = "In triangle wave or complementary PWM mode, no transfer. In saw-wave mode, no transfer."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ADTTA_A::_00)
    }
    #[doc = "In triangle wave or complementary PWM mode, transfer at crest. In saw-wave mode, transfer at underflow (in down-counting), overflow (in up-counting), or counter clearing."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ADTTA_A::_01)
    }
    #[doc = "In triangle wave or complementary PWM mode, transfer at trough. In saw-wave mode, transfer at underflow (in down-counting), overflow (in up-counting), or counter clearing."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ADTTA_A::_10)
    }
    #[doc = "In triangle wave or complementary PWM mode, transfer at both crest and trough. In saw-wave mode, transfer at underflow (in down-counting), overflow (in up-counting), or counter clearing."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ADTTA_A::_11)
    }
}
#[doc = "Field `ADTDA` reader - GTADTRA Register Double Buffer Operation"]
pub type ADTDA_R = crate::BitReader<ADTDA_A>;
#[doc = "GTADTRA Register Double Buffer Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADTDA_A {
    #[doc = "0: Single buffer operation (GTADTBRA --> GTADTRA)"]
    _0 = 0,
    #[doc = "1: Double buffer operation (GTADTDBRA --> GTADTBRA --> GTADTRA)"]
    _1 = 1,
}
impl From<ADTDA_A> for bool {
    #[inline(always)]
    fn from(variant: ADTDA_A) -> Self {
        variant as u8 != 0
    }
}
impl ADTDA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADTDA_A {
        match self.bits {
            false => ADTDA_A::_0,
            true => ADTDA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADTDA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADTDA_A::_1
    }
}
#[doc = "Field `ADTDA` writer - GTADTRA Register Double Buffer Operation"]
pub type ADTDA_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTBER_SPEC, ADTDA_A, O>;
impl<'a, const O: u8> ADTDA_W<'a, O> {
    #[doc = "Single buffer operation (GTADTBRA --> GTADTRA)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADTDA_A::_0)
    }
    #[doc = "Double buffer operation (GTADTDBRA --> GTADTBRA --> GTADTRA)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADTDA_A::_1)
    }
}
#[doc = "Field `ADTTB` reader - GTADTRB Register Buffer Transfer Timing Select"]
pub type ADTTB_R = crate::FieldReader<u8, ADTTB_A>;
#[doc = "GTADTRB Register Buffer Transfer Timing Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADTTB_A {
    #[doc = "0: In triangle wave or complementary PWM mode, no transfer. In saw-wave mode, no transfer."]
    _00 = 0,
    #[doc = "1: In triangle wave or complementary PWM mode, transfer at crest. In saw-wave mode, transfer at underflow (in down-counting), overflow (in up-counting), or counter clearing."]
    _01 = 1,
    #[doc = "2: In triangle wave or complementary PWM mode, transfer at trough. In saw-wave mode, transfer at underflow (in down-counting), overflow (in up-counting), or counter clearing."]
    _10 = 2,
    #[doc = "3: In triangle wave or complementary PWM mode, transfer at both crest and trough. In saw-wave mode, transfer at underflow (in down-counting), overflow (in up-counting), or counter clearing."]
    _11 = 3,
}
impl From<ADTTB_A> for u8 {
    #[inline(always)]
    fn from(variant: ADTTB_A) -> Self {
        variant as _
    }
}
impl ADTTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADTTB_A {
        match self.bits {
            0 => ADTTB_A::_00,
            1 => ADTTB_A::_01,
            2 => ADTTB_A::_10,
            3 => ADTTB_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ADTTB_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ADTTB_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ADTTB_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ADTTB_A::_11
    }
}
#[doc = "Field `ADTTB` writer - GTADTRB Register Buffer Transfer Timing Select"]
pub type ADTTB_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTBER_SPEC, u8, ADTTB_A, 2, O>;
impl<'a, const O: u8> ADTTB_W<'a, O> {
    #[doc = "In triangle wave or complementary PWM mode, no transfer. In saw-wave mode, no transfer."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ADTTB_A::_00)
    }
    #[doc = "In triangle wave or complementary PWM mode, transfer at crest. In saw-wave mode, transfer at underflow (in down-counting), overflow (in up-counting), or counter clearing."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ADTTB_A::_01)
    }
    #[doc = "In triangle wave or complementary PWM mode, transfer at trough. In saw-wave mode, transfer at underflow (in down-counting), overflow (in up-counting), or counter clearing."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ADTTB_A::_10)
    }
    #[doc = "In triangle wave or complementary PWM mode, transfer at both crest and trough. In saw-wave mode, transfer at underflow (in down-counting), overflow (in up-counting), or counter clearing."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ADTTB_A::_11)
    }
}
#[doc = "Field `ADTDB` reader - GTADTRB Register Double Buffer Operation"]
pub type ADTDB_R = crate::BitReader<ADTDB_A>;
#[doc = "GTADTRB Register Double Buffer Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADTDB_A {
    #[doc = "0: Single buffer operation (GTADTBRB --> GTADTRB)"]
    _0 = 0,
    #[doc = "1: Double buffer operation (GTADTDBRB --> GTADTBRB --> GTADTRB)"]
    _1 = 1,
}
impl From<ADTDB_A> for bool {
    #[inline(always)]
    fn from(variant: ADTDB_A) -> Self {
        variant as u8 != 0
    }
}
impl ADTDB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADTDB_A {
        match self.bits {
            false => ADTDB_A::_0,
            true => ADTDB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADTDB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADTDB_A::_1
    }
}
#[doc = "Field `ADTDB` writer - GTADTRB Register Double Buffer Operation"]
pub type ADTDB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTBER_SPEC, ADTDB_A, O>;
impl<'a, const O: u8> ADTDB_W<'a, O> {
    #[doc = "Single buffer operation (GTADTBRB --> GTADTRB)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADTDB_A::_0)
    }
    #[doc = "Double buffer operation (GTADTDBRB --> GTADTBRB --> GTADTRB)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADTDB_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - GTCCR Buffer Operation Disable"]
    #[inline(always)]
    pub fn bd0(&self) -> BD0_R {
        BD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GTPR Buffer Operation Disable"]
    #[inline(always)]
    pub fn bd1(&self) -> BD1_R {
        BD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GTADTRA/GTADTRB Registers Buffer Operation Disable"]
    #[inline(always)]
    pub fn bd2(&self) -> BD2_R {
        BD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GTDVU/GTDVD Registers Buffer Operation Disable"]
    #[inline(always)]
    pub fn bd3(&self) -> BD3_R {
        BD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - GTCCRA Register Double Buffer Repeat Operation Enable"]
    #[inline(always)]
    pub fn dbrteca(&self) -> DBRTECA_R {
        DBRTECA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - GTCCRB Register Double Buffer Repeat Operation Enable"]
    #[inline(always)]
    pub fn dbrtecb(&self) -> DBRTECB_R {
        DBRTECB_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:17 - GTCCRA Buffer Operation"]
    #[inline(always)]
    pub fn ccra(&self) -> CCRA_R {
        CCRA_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - GTCCRB Buffer Operation"]
    #[inline(always)]
    pub fn ccrb(&self) -> CCRB_R {
        CCRB_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - GTPR Buffer Operation"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - GTADTRA Register Buffer Transfer Timing Select"]
    #[inline(always)]
    pub fn adtta(&self) -> ADTTA_R {
        ADTTA_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - GTADTRA Register Double Buffer Operation"]
    #[inline(always)]
    pub fn adtda(&self) -> ADTDA_R {
        ADTDA_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 28:29 - GTADTRB Register Buffer Transfer Timing Select"]
    #[inline(always)]
    pub fn adttb(&self) -> ADTTB_R {
        ADTTB_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - GTADTRB Register Double Buffer Operation"]
    #[inline(always)]
    pub fn adtdb(&self) -> ADTDB_R {
        ADTDB_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GTCCR Buffer Operation Disable"]
    #[inline(always)]
    #[must_use]
    pub fn bd0(&mut self) -> BD0_W<0> {
        BD0_W::new(self)
    }
    #[doc = "Bit 1 - GTPR Buffer Operation Disable"]
    #[inline(always)]
    #[must_use]
    pub fn bd1(&mut self) -> BD1_W<1> {
        BD1_W::new(self)
    }
    #[doc = "Bit 2 - GTADTRA/GTADTRB Registers Buffer Operation Disable"]
    #[inline(always)]
    #[must_use]
    pub fn bd2(&mut self) -> BD2_W<2> {
        BD2_W::new(self)
    }
    #[doc = "Bit 3 - GTDVU/GTDVD Registers Buffer Operation Disable"]
    #[inline(always)]
    #[must_use]
    pub fn bd3(&mut self) -> BD3_W<3> {
        BD3_W::new(self)
    }
    #[doc = "Bit 8 - GTCCRA Register Double Buffer Repeat Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbrteca(&mut self) -> DBRTECA_W<8> {
        DBRTECA_W::new(self)
    }
    #[doc = "Bit 10 - GTCCRB Register Double Buffer Repeat Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbrtecb(&mut self) -> DBRTECB_W<10> {
        DBRTECB_W::new(self)
    }
    #[doc = "Bits 16:17 - GTCCRA Buffer Operation"]
    #[inline(always)]
    #[must_use]
    pub fn ccra(&mut self) -> CCRA_W<16> {
        CCRA_W::new(self)
    }
    #[doc = "Bits 18:19 - GTCCRB Buffer Operation"]
    #[inline(always)]
    #[must_use]
    pub fn ccrb(&mut self) -> CCRB_W<18> {
        CCRB_W::new(self)
    }
    #[doc = "Bits 20:21 - GTPR Buffer Operation"]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<20> {
        PR_W::new(self)
    }
    #[doc = "Bit 22 - GTCCRA and GTCCRB Forcible Buffer Operation"]
    #[inline(always)]
    #[must_use]
    pub fn ccrswt(&mut self) -> CCRSWT_W<22> {
        CCRSWT_W::new(self)
    }
    #[doc = "Bits 24:25 - GTADTRA Register Buffer Transfer Timing Select"]
    #[inline(always)]
    #[must_use]
    pub fn adtta(&mut self) -> ADTTA_W<24> {
        ADTTA_W::new(self)
    }
    #[doc = "Bit 26 - GTADTRA Register Double Buffer Operation"]
    #[inline(always)]
    #[must_use]
    pub fn adtda(&mut self) -> ADTDA_W<26> {
        ADTDA_W::new(self)
    }
    #[doc = "Bits 28:29 - GTADTRB Register Buffer Transfer Timing Select"]
    #[inline(always)]
    #[must_use]
    pub fn adttb(&mut self) -> ADTTB_W<28> {
        ADTTB_W::new(self)
    }
    #[doc = "Bit 30 - GTADTRB Register Double Buffer Operation"]
    #[inline(always)]
    #[must_use]
    pub fn adtdb(&mut self) -> ADTDB_W<30> {
        ADTDB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Buffer Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtber](index.html) module"]
pub struct GTBER_SPEC;
impl crate::RegisterSpec for GTBER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtber::R](R) reader structure"]
impl crate::Readable for GTBER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtber::W](W) writer structure"]
impl crate::Writable for GTBER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTBER to value 0"]
impl crate::Resettable for GTBER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
