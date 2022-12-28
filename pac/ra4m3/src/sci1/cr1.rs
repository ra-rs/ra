#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BFE` reader - Break Field Enable"]
pub type BFE_R = crate::BitReader<BFE_A>;
#[doc = "Break Field Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFE_A {
    #[doc = "0: Break Field detection is disabled."]
    _0 = 0,
    #[doc = "1: Break Field detection is enabled."]
    _1 = 1,
}
impl From<BFE_A> for bool {
    #[inline(always)]
    fn from(variant: BFE_A) -> Self {
        variant as u8 != 0
    }
}
impl BFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFE_A {
        match self.bits {
            false => BFE_A::_0,
            true => BFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BFE_A::_1
    }
}
#[doc = "Field `BFE` writer - Break Field Enable"]
pub type BFE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR1_SPEC, BFE_A, O>;
impl<'a, const O: u8> BFE_W<'a, O> {
    #[doc = "Break Field detection is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BFE_A::_0)
    }
    #[doc = "Break Field detection is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BFE_A::_1)
    }
}
#[doc = "Field `CF0RE` reader - Control Field 0 Reception Enable"]
pub type CF0RE_R = crate::BitReader<CF0RE_A>;
#[doc = "Control Field 0 Reception Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CF0RE_A {
    #[doc = "0: Reception of Control Field 0 is disabled."]
    _0 = 0,
    #[doc = "1: Reception of Control Field 0 is enabled."]
    _1 = 1,
}
impl From<CF0RE_A> for bool {
    #[inline(always)]
    fn from(variant: CF0RE_A) -> Self {
        variant as u8 != 0
    }
}
impl CF0RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CF0RE_A {
        match self.bits {
            false => CF0RE_A::_0,
            true => CF0RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CF0RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CF0RE_A::_1
    }
}
#[doc = "Field `CF0RE` writer - Control Field 0 Reception Enable"]
pub type CF0RE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR1_SPEC, CF0RE_A, O>;
impl<'a, const O: u8> CF0RE_W<'a, O> {
    #[doc = "Reception of Control Field 0 is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CF0RE_A::_0)
    }
    #[doc = "Reception of Control Field 0 is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CF0RE_A::_1)
    }
}
#[doc = "Field `CF1DS` reader - Control Field 1 Data Register Select"]
pub type CF1DS_R = crate::FieldReader<u8, CF1DS_A>;
#[doc = "Control Field 1 Data Register Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CF1DS_A {
    #[doc = "0: Selects comparison with the value in PCF1DR."]
    _00 = 0,
    #[doc = "1: Selects comparison with the value in SCF1DR."]
    _01 = 1,
    #[doc = "2: Selects comparison with the values in PCF1DR and SCF1DR."]
    _10 = 2,
    #[doc = "3: Setting prohibited."]
    _11 = 3,
}
impl From<CF1DS_A> for u8 {
    #[inline(always)]
    fn from(variant: CF1DS_A) -> Self {
        variant as _
    }
}
impl CF1DS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CF1DS_A {
        match self.bits {
            0 => CF1DS_A::_00,
            1 => CF1DS_A::_01,
            2 => CF1DS_A::_10,
            3 => CF1DS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CF1DS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CF1DS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CF1DS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CF1DS_A::_11
    }
}
#[doc = "Field `CF1DS` writer - Control Field 1 Data Register Select"]
pub type CF1DS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CR1_SPEC, u8, CF1DS_A, 2, O>;
impl<'a, const O: u8> CF1DS_W<'a, O> {
    #[doc = "Selects comparison with the value in PCF1DR."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CF1DS_A::_00)
    }
    #[doc = "Selects comparison with the value in SCF1DR."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CF1DS_A::_01)
    }
    #[doc = "Selects comparison with the values in PCF1DR and SCF1DR."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CF1DS_A::_10)
    }
    #[doc = "Setting prohibited."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CF1DS_A::_11)
    }
}
#[doc = "Field `PIBE` reader - Priority Interrupt Bit Enable"]
pub type PIBE_R = crate::BitReader<PIBE_A>;
#[doc = "Priority Interrupt Bit Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIBE_A {
    #[doc = "0: The priority interrupt bit is disabled."]
    _0 = 0,
    #[doc = "1: The priority interrupt bit is enabled."]
    _1 = 1,
}
impl From<PIBE_A> for bool {
    #[inline(always)]
    fn from(variant: PIBE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIBE_A {
        match self.bits {
            false => PIBE_A::_0,
            true => PIBE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIBE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIBE_A::_1
    }
}
#[doc = "Field `PIBE` writer - Priority Interrupt Bit Enable"]
pub type PIBE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR1_SPEC, PIBE_A, O>;
impl<'a, const O: u8> PIBE_W<'a, O> {
    #[doc = "The priority interrupt bit is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIBE_A::_0)
    }
    #[doc = "The priority interrupt bit is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIBE_A::_1)
    }
}
#[doc = "Field `PIBS` reader - Priority Interrupt Bit Select"]
pub type PIBS_R = crate::FieldReader<u8, PIBS_A>;
#[doc = "Priority Interrupt Bit Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PIBS_A {
    #[doc = "0: 0th bit of Control Field 1"]
    _000 = 0,
    #[doc = "1: 1st bit of Control Field 1"]
    _001 = 1,
    #[doc = "2: 2nd bit of Control Field 1"]
    _010 = 2,
    #[doc = "3: 3rd bit of Control Field 1"]
    _011 = 3,
    #[doc = "4: 4th bit of Control Field 1"]
    _100 = 4,
    #[doc = "5: 5th bit of Control Field 1"]
    _101 = 5,
    #[doc = "6: 6th bit of Control Field 1"]
    _110 = 6,
    #[doc = "7: 7th bit of Control Field 1"]
    _111 = 7,
}
impl From<PIBS_A> for u8 {
    #[inline(always)]
    fn from(variant: PIBS_A) -> Self {
        variant as _
    }
}
impl PIBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIBS_A {
        match self.bits {
            0 => PIBS_A::_000,
            1 => PIBS_A::_001,
            2 => PIBS_A::_010,
            3 => PIBS_A::_011,
            4 => PIBS_A::_100,
            5 => PIBS_A::_101,
            6 => PIBS_A::_110,
            7 => PIBS_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PIBS_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PIBS_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PIBS_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PIBS_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PIBS_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PIBS_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PIBS_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == PIBS_A::_111
    }
}
#[doc = "Field `PIBS` writer - Priority Interrupt Bit Select"]
pub type PIBS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CR1_SPEC, u8, PIBS_A, 3, O>;
impl<'a, const O: u8> PIBS_W<'a, O> {
    #[doc = "0th bit of Control Field 1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PIBS_A::_000)
    }
    #[doc = "1st bit of Control Field 1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PIBS_A::_001)
    }
    #[doc = "2nd bit of Control Field 1"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PIBS_A::_010)
    }
    #[doc = "3rd bit of Control Field 1"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PIBS_A::_011)
    }
    #[doc = "4th bit of Control Field 1"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PIBS_A::_100)
    }
    #[doc = "5th bit of Control Field 1"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PIBS_A::_101)
    }
    #[doc = "6th bit of Control Field 1"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PIBS_A::_110)
    }
    #[doc = "7th bit of Control Field 1"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(PIBS_A::_111)
    }
}
impl R {
    #[doc = "Bit 0 - Break Field Enable"]
    #[inline(always)]
    pub fn bfe(&self) -> BFE_R {
        BFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control Field 0 Reception Enable"]
    #[inline(always)]
    pub fn cf0re(&self) -> CF0RE_R {
        CF0RE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Control Field 1 Data Register Select"]
    #[inline(always)]
    pub fn cf1ds(&self) -> CF1DS_R {
        CF1DS_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - Priority Interrupt Bit Enable"]
    #[inline(always)]
    pub fn pibe(&self) -> PIBE_R {
        PIBE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Priority Interrupt Bit Select"]
    #[inline(always)]
    pub fn pibs(&self) -> PIBS_R {
        PIBS_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bit 0 - Break Field Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bfe(&mut self) -> BFE_W<0> {
        BFE_W::new(self)
    }
    #[doc = "Bit 1 - Control Field 0 Reception Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cf0re(&mut self) -> CF0RE_W<1> {
        CF0RE_W::new(self)
    }
    #[doc = "Bits 2:3 - Control Field 1 Data Register Select"]
    #[inline(always)]
    #[must_use]
    pub fn cf1ds(&mut self) -> CF1DS_W<2> {
        CF1DS_W::new(self)
    }
    #[doc = "Bit 4 - Priority Interrupt Bit Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pibe(&mut self) -> PIBE_W<4> {
        PIBE_W::new(self)
    }
    #[doc = "Bits 5:7 - Priority Interrupt Bit Select"]
    #[inline(always)]
    #[must_use]
    pub fn pibs(&mut self) -> PIBS_W<5> {
        PIBS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
