#[doc = "Register `LCDM0` reader"]
pub struct R(crate::R<LCDM0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDM0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDM0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDM0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDM0` writer"]
pub struct W(crate::W<LCDM0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDM0_SPEC>;
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
impl From<crate::W<LCDM0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDM0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LBAS` reader - LCD Display Bias Method Select"]
pub type LBAS_R = crate::FieldReader<u8, LBAS_A>;
#[doc = "LCD Display Bias Method Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LBAS_A {
    #[doc = "0: 1/2 bias method"]
    _00 = 0,
    #[doc = "1: 1/3 bias method"]
    _01 = 1,
    #[doc = "2: 1/4 bias method"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<LBAS_A> for u8 {
    #[inline(always)]
    fn from(variant: LBAS_A) -> Self {
        variant as _
    }
}
impl LBAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBAS_A {
        match self.bits {
            0 => LBAS_A::_00,
            1 => LBAS_A::_01,
            2 => LBAS_A::_10,
            3 => LBAS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LBAS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LBAS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == LBAS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == LBAS_A::_11
    }
}
#[doc = "Field `LBAS` writer - LCD Display Bias Method Select"]
pub type LBAS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, LCDM0_SPEC, u8, LBAS_A, 2, O>;
impl<'a, const O: u8> LBAS_W<'a, O> {
    #[doc = "1/2 bias method"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LBAS_A::_00)
    }
    #[doc = "1/3 bias method"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LBAS_A::_01)
    }
    #[doc = "1/4 bias method"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(LBAS_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(LBAS_A::_11)
    }
}
#[doc = "Field `LDTY` reader - Time Slice of LCD Display Select"]
pub type LDTY_R = crate::FieldReader<u8, LDTY_A>;
#[doc = "Time Slice of LCD Display Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LDTY_A {
    #[doc = "0: Static"]
    _000 = 0,
    #[doc = "1: 2-time slice"]
    _001 = 1,
    #[doc = "2: 3-time slice"]
    _010 = 2,
    #[doc = "3: 4-time slice"]
    _011 = 3,
    #[doc = "5: 8-time slice"]
    _101 = 5,
}
impl From<LDTY_A> for u8 {
    #[inline(always)]
    fn from(variant: LDTY_A) -> Self {
        variant as _
    }
}
impl LDTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LDTY_A> {
        match self.bits {
            0 => Some(LDTY_A::_000),
            1 => Some(LDTY_A::_001),
            2 => Some(LDTY_A::_010),
            3 => Some(LDTY_A::_011),
            5 => Some(LDTY_A::_101),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == LDTY_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == LDTY_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == LDTY_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == LDTY_A::_011
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == LDTY_A::_101
    }
}
#[doc = "Field `LDTY` writer - Time Slice of LCD Display Select"]
pub type LDTY_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LCDM0_SPEC, u8, LDTY_A, 3, O>;
impl<'a, const O: u8> LDTY_W<'a, O> {
    #[doc = "Static"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(LDTY_A::_000)
    }
    #[doc = "2-time slice"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(LDTY_A::_001)
    }
    #[doc = "3-time slice"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(LDTY_A::_010)
    }
    #[doc = "4-time slice"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(LDTY_A::_011)
    }
    #[doc = "8-time slice"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(LDTY_A::_101)
    }
}
#[doc = "Field `LWAVE` reader - LCD display waveform selection"]
pub type LWAVE_R = crate::BitReader<LWAVE_A>;
#[doc = "LCD display waveform selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LWAVE_A {
    #[doc = "0: Waveform A"]
    _0 = 0,
    #[doc = "1: Waveform B"]
    _1 = 1,
}
impl From<LWAVE_A> for bool {
    #[inline(always)]
    fn from(variant: LWAVE_A) -> Self {
        variant as u8 != 0
    }
}
impl LWAVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LWAVE_A {
        match self.bits {
            false => LWAVE_A::_0,
            true => LWAVE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LWAVE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LWAVE_A::_1
    }
}
#[doc = "Field `LWAVE` writer - LCD display waveform selection"]
pub type LWAVE_W<'a, const O: u8> = crate::BitWriter<'a, u8, LCDM0_SPEC, LWAVE_A, O>;
impl<'a, const O: u8> LWAVE_W<'a, O> {
    #[doc = "Waveform A"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LWAVE_A::_0)
    }
    #[doc = "Waveform B"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LWAVE_A::_1)
    }
}
#[doc = "Field `MDSET` reader - LCD drive voltage generator selection"]
pub type MDSET_R = crate::FieldReader<u8, MDSET_A>;
#[doc = "LCD drive voltage generator selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MDSET_A {
    #[doc = "0: External resistance division method"]
    _00 = 0,
    #[doc = "1: Internal voltage boosting method"]
    _01 = 1,
    #[doc = "2: Capacitor split method"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<MDSET_A> for u8 {
    #[inline(always)]
    fn from(variant: MDSET_A) -> Self {
        variant as _
    }
}
impl MDSET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDSET_A {
        match self.bits {
            0 => MDSET_A::_00,
            1 => MDSET_A::_01,
            2 => MDSET_A::_10,
            3 => MDSET_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == MDSET_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == MDSET_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MDSET_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == MDSET_A::_11
    }
}
#[doc = "Field `MDSET` writer - LCD drive voltage generator selection"]
pub type MDSET_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, LCDM0_SPEC, u8, MDSET_A, 2, O>;
impl<'a, const O: u8> MDSET_W<'a, O> {
    #[doc = "External resistance division method"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(MDSET_A::_00)
    }
    #[doc = "Internal voltage boosting method"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(MDSET_A::_01)
    }
    #[doc = "Capacitor split method"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(MDSET_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(MDSET_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - LCD Display Bias Method Select"]
    #[inline(always)]
    pub fn lbas(&self) -> LBAS_R {
        LBAS_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:4 - Time Slice of LCD Display Select"]
    #[inline(always)]
    pub fn ldty(&self) -> LDTY_R {
        LDTY_R::new((self.bits >> 2) & 7)
    }
    #[doc = "Bit 5 - LCD display waveform selection"]
    #[inline(always)]
    pub fn lwave(&self) -> LWAVE_R {
        LWAVE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - LCD drive voltage generator selection"]
    #[inline(always)]
    pub fn mdset(&self) -> MDSET_R {
        MDSET_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - LCD Display Bias Method Select"]
    #[inline(always)]
    #[must_use]
    pub fn lbas(&mut self) -> LBAS_W<0> {
        LBAS_W::new(self)
    }
    #[doc = "Bits 2:4 - Time Slice of LCD Display Select"]
    #[inline(always)]
    #[must_use]
    pub fn ldty(&mut self) -> LDTY_W<2> {
        LDTY_W::new(self)
    }
    #[doc = "Bit 5 - LCD display waveform selection"]
    #[inline(always)]
    #[must_use]
    pub fn lwave(&mut self) -> LWAVE_W<5> {
        LWAVE_W::new(self)
    }
    #[doc = "Bits 6:7 - LCD drive voltage generator selection"]
    #[inline(always)]
    #[must_use]
    pub fn mdset(&mut self) -> MDSET_W<6> {
        MDSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Mode Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm0](index.html) module"]
pub struct LCDM0_SPEC;
impl crate::RegisterSpec for LCDM0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lcdm0::R](R) reader structure"]
impl crate::Readable for LCDM0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdm0::W](W) writer structure"]
impl crate::Writable for LCDM0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCDM0 to value 0"]
impl crate::Resettable for LCDM0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
