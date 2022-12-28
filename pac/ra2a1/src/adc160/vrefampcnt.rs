#[doc = "Register `VREFAMPCNT` reader"]
pub struct R(crate::R<VREFAMPCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREFAMPCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREFAMPCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREFAMPCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREFAMPCNT` writer"]
pub struct W(crate::W<VREFAMPCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREFAMPCNT_SPEC>;
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
impl From<crate::W<VREFAMPCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREFAMPCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OLDETEN` reader - OLDET Enable"]
pub type OLDETEN_R = crate::BitReader<OLDETEN_A>;
#[doc = "OLDET Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OLDETEN_A {
    #[doc = "0: Disable the over current detection"]
    _0 = 0,
    #[doc = "1: Enable the over current detection"]
    _1 = 1,
}
impl From<OLDETEN_A> for bool {
    #[inline(always)]
    fn from(variant: OLDETEN_A) -> Self {
        variant as u8 != 0
    }
}
impl OLDETEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OLDETEN_A {
        match self.bits {
            false => OLDETEN_A::_0,
            true => OLDETEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OLDETEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OLDETEN_A::_1
    }
}
#[doc = "Field `OLDETEN` writer - OLDET Enable"]
pub type OLDETEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, VREFAMPCNT_SPEC, OLDETEN_A, O>;
impl<'a, const O: u8> OLDETEN_W<'a, O> {
    #[doc = "Disable the over current detection"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OLDETEN_A::_0)
    }
    #[doc = "Enable the over current detection"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OLDETEN_A::_1)
    }
}
#[doc = "Field `VREFADCG` reader - VREFADC Output Voltage Control"]
pub type VREFADCG_R = crate::FieldReader<u8, VREFADCG_A>;
#[doc = "VREFADC Output Voltage Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VREFADCG_A {
    #[doc = "0: 1.5 V"]
    _00 = 0,
    #[doc = "1: 1.5 V"]
    _01 = 1,
    #[doc = "2: 2.0 V"]
    _10 = 2,
    #[doc = "3: 2.5 V"]
    _11 = 3,
}
impl From<VREFADCG_A> for u8 {
    #[inline(always)]
    fn from(variant: VREFADCG_A) -> Self {
        variant as _
    }
}
impl VREFADCG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFADCG_A {
        match self.bits {
            0 => VREFADCG_A::_00,
            1 => VREFADCG_A::_01,
            2 => VREFADCG_A::_10,
            3 => VREFADCG_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == VREFADCG_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == VREFADCG_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == VREFADCG_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == VREFADCG_A::_11
    }
}
#[doc = "Field `VREFADCG` writer - VREFADC Output Voltage Control"]
pub type VREFADCG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, VREFAMPCNT_SPEC, u8, VREFADCG_A, 2, O>;
impl<'a, const O: u8> VREFADCG_W<'a, O> {
    #[doc = "1.5 V"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(VREFADCG_A::_00)
    }
    #[doc = "1.5 V"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(VREFADCG_A::_01)
    }
    #[doc = "2.0 V"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(VREFADCG_A::_10)
    }
    #[doc = "2.5 V"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(VREFADCG_A::_11)
    }
}
#[doc = "Field `VREFADCEN` reader - VREFADCG Enable"]
pub type VREFADCEN_R = crate::BitReader<VREFADCEN_A>;
#[doc = "VREFADCG Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFADCEN_A {
    #[doc = "0: Disable the VREFADC output"]
    _0 = 0,
    #[doc = "1: Enable the VREFADC output"]
    _1 = 1,
}
impl From<VREFADCEN_A> for bool {
    #[inline(always)]
    fn from(variant: VREFADCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VREFADCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFADCEN_A {
        match self.bits {
            false => VREFADCEN_A::_0,
            true => VREFADCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VREFADCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VREFADCEN_A::_1
    }
}
#[doc = "Field `VREFADCEN` writer - VREFADCG Enable"]
pub type VREFADCEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, VREFAMPCNT_SPEC, VREFADCEN_A, O>;
impl<'a, const O: u8> VREFADCEN_W<'a, O> {
    #[doc = "Disable the VREFADC output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VREFADCEN_A::_0)
    }
    #[doc = "Enable the VREFADC output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VREFADCEN_A::_1)
    }
}
#[doc = "Field `BGREN` reader - Low-Potential Reference Voltage Select"]
pub type BGREN_R = crate::BitReader<BGREN_A>;
#[doc = "Low-Potential Reference Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BGREN_A {
    #[doc = "0: Select AVSS0 as the low-potential reference voltage"]
    _0 = 0,
    #[doc = "1: Select VREFL0 as the low-potential reference voltage."]
    _1 = 1,
}
impl From<BGREN_A> for bool {
    #[inline(always)]
    fn from(variant: BGREN_A) -> Self {
        variant as u8 != 0
    }
}
impl BGREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGREN_A {
        match self.bits {
            false => BGREN_A::_0,
            true => BGREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BGREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BGREN_A::_1
    }
}
#[doc = "Field `BGREN` writer - Low-Potential Reference Voltage Select"]
pub type BGREN_W<'a, const O: u8> = crate::BitWriter<'a, u8, VREFAMPCNT_SPEC, BGREN_A, O>;
impl<'a, const O: u8> BGREN_W<'a, O> {
    #[doc = "Select AVSS0 as the low-potential reference voltage"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BGREN_A::_0)
    }
    #[doc = "Select VREFL0 as the low-potential reference voltage."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BGREN_A::_1)
    }
}
#[doc = "Field `ADSLP` reader - Sleep"]
pub type ADSLP_R = crate::BitReader<ADSLP_A>;
#[doc = "Sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSLP_A {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: Standby state"]
    _1 = 1,
}
impl From<ADSLP_A> for bool {
    #[inline(always)]
    fn from(variant: ADSLP_A) -> Self {
        variant as u8 != 0
    }
}
impl ADSLP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADSLP_A {
        match self.bits {
            false => ADSLP_A::_0,
            true => ADSLP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADSLP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADSLP_A::_1
    }
}
#[doc = "Field `ADSLP` writer - Sleep"]
pub type ADSLP_W<'a, const O: u8> = crate::BitWriter<'a, u8, VREFAMPCNT_SPEC, ADSLP_A, O>;
impl<'a, const O: u8> ADSLP_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADSLP_A::_0)
    }
    #[doc = "Standby state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADSLP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - OLDET Enable"]
    #[inline(always)]
    pub fn oldeten(&self) -> OLDETEN_R {
        OLDETEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - VREFADC Output Voltage Control"]
    #[inline(always)]
    pub fn vrefadcg(&self) -> VREFADCG_R {
        VREFADCG_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 3 - VREFADCG Enable"]
    #[inline(always)]
    pub fn vrefadcen(&self) -> VREFADCEN_R {
        VREFADCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Low-Potential Reference Voltage Select"]
    #[inline(always)]
    pub fn bgren(&self) -> BGREN_R {
        BGREN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Sleep"]
    #[inline(always)]
    pub fn adslp(&self) -> ADSLP_R {
        ADSLP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OLDET Enable"]
    #[inline(always)]
    #[must_use]
    pub fn oldeten(&mut self) -> OLDETEN_W<0> {
        OLDETEN_W::new(self)
    }
    #[doc = "Bits 1:2 - VREFADC Output Voltage Control"]
    #[inline(always)]
    #[must_use]
    pub fn vrefadcg(&mut self) -> VREFADCG_W<1> {
        VREFADCG_W::new(self)
    }
    #[doc = "Bit 3 - VREFADCG Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vrefadcen(&mut self) -> VREFADCEN_W<3> {
        VREFADCEN_W::new(self)
    }
    #[doc = "Bit 4 - Low-Potential Reference Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn bgren(&mut self) -> BGREN_W<4> {
        BGREN_W::new(self)
    }
    #[doc = "Bit 7 - Sleep"]
    #[inline(always)]
    #[must_use]
    pub fn adslp(&mut self) -> ADSLP_W<7> {
        ADSLP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Dedicated Reference Voltage Circuit Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vrefampcnt](index.html) module"]
pub struct VREFAMPCNT_SPEC;
impl crate::RegisterSpec for VREFAMPCNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [vrefampcnt::R](R) reader structure"]
impl crate::Readable for VREFAMPCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vrefampcnt::W](W) writer structure"]
impl crate::Writable for VREFAMPCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VREFAMPCNT to value 0"]
impl crate::Resettable for VREFAMPCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
