#[doc = "Register `DOCR` reader"]
pub struct R(crate::R<DOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOCR` writer"]
pub struct W(crate::W<DOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOCR_SPEC>;
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
impl From<crate::W<DOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OMS` reader - Operating Mode Select"]
pub type OMS_R = crate::FieldReader<u8, OMS_A>;
#[doc = "Operating Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OMS_A {
    #[doc = "0: Data comparison mode"]
    _00 = 0,
    #[doc = "1: Data addition mode"]
    _01 = 1,
    #[doc = "2: Data subtraction mode"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<OMS_A> for u8 {
    #[inline(always)]
    fn from(variant: OMS_A) -> Self {
        variant as _
    }
}
impl OMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OMS_A {
        match self.bits {
            0 => OMS_A::_00,
            1 => OMS_A::_01,
            2 => OMS_A::_10,
            3 => OMS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OMS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OMS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OMS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OMS_A::_11
    }
}
#[doc = "Field `OMS` writer - Operating Mode Select"]
pub type OMS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, DOCR_SPEC, u8, OMS_A, 2, O>;
impl<'a, const O: u8> OMS_W<'a, O> {
    #[doc = "Data comparison mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(OMS_A::_00)
    }
    #[doc = "Data addition mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(OMS_A::_01)
    }
    #[doc = "Data subtraction mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(OMS_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(OMS_A::_11)
    }
}
#[doc = "Field `DOBW` reader - Data Operation Bit Width Select"]
pub type DOBW_R = crate::BitReader<DOBW_A>;
#[doc = "Data Operation Bit Width Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOBW_A {
    #[doc = "0: 16-bit"]
    _0 = 0,
    #[doc = "1: 32-bit"]
    _1 = 1,
}
impl From<DOBW_A> for bool {
    #[inline(always)]
    fn from(variant: DOBW_A) -> Self {
        variant as u8 != 0
    }
}
impl DOBW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOBW_A {
        match self.bits {
            false => DOBW_A::_0,
            true => DOBW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOBW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOBW_A::_1
    }
}
#[doc = "Field `DOBW` writer - Data Operation Bit Width Select"]
pub type DOBW_W<'a, const O: u8> = crate::BitWriter<'a, u8, DOCR_SPEC, DOBW_A, O>;
impl<'a, const O: u8> DOBW_W<'a, O> {
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOBW_A::_0)
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOBW_A::_1)
    }
}
#[doc = "Field `DCSEL` reader - Detection Condition Select"]
pub type DCSEL_R = crate::FieldReader<u8, DCSEL_A>;
#[doc = "Detection Condition Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DCSEL_A {
    #[doc = "0: Mismatch (DODSR0 â\u{89} DODIR)"]
    _000 = 0,
    #[doc = "1: Match (DODSR0 = DODIR)"]
    _001 = 1,
    #[doc = "2: Lower (DODSR0 > DODIR)"]
    _010 = 2,
    #[doc = "3: Upper (DODSR0 < DODIR)"]
    _011 = 3,
    #[doc = "4: Inside window (DODSR0 < DODIR < DODSR1)"]
    _100 = 4,
    #[doc = "5: Outside window (DODIR < DODSR0, DODSR1 < DODIR)"]
    _101 = 5,
}
impl From<DCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DCSEL_A) -> Self {
        variant as _
    }
}
impl DCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DCSEL_A> {
        match self.bits {
            0 => Some(DCSEL_A::_000),
            1 => Some(DCSEL_A::_001),
            2 => Some(DCSEL_A::_010),
            3 => Some(DCSEL_A::_011),
            4 => Some(DCSEL_A::_100),
            5 => Some(DCSEL_A::_101),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DCSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DCSEL_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DCSEL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DCSEL_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DCSEL_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DCSEL_A::_101
    }
}
#[doc = "Field `DCSEL` writer - Detection Condition Select"]
pub type DCSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, DOCR_SPEC, u8, DCSEL_A, 3, O>;
impl<'a, const O: u8> DCSEL_W<'a, O> {
    #[doc = "Mismatch (DODSR0 â\u{89} DODIR)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(DCSEL_A::_000)
    }
    #[doc = "Match (DODSR0 = DODIR)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(DCSEL_A::_001)
    }
    #[doc = "Lower (DODSR0 > DODIR)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(DCSEL_A::_010)
    }
    #[doc = "Upper (DODSR0 < DODIR)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(DCSEL_A::_011)
    }
    #[doc = "Inside window (DODSR0 < DODIR < DODSR1)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(DCSEL_A::_100)
    }
    #[doc = "Outside window (DODIR < DODSR0, DODSR1 < DODIR)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(DCSEL_A::_101)
    }
}
#[doc = "Field `DOPCIE` reader - Data Operation Circuit Interrupt Enable"]
pub type DOPCIE_R = crate::BitReader<DOPCIE_A>;
#[doc = "Data Operation Circuit Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOPCIE_A {
    #[doc = "0: Disables interrupts from the data operation circuit."]
    _0 = 0,
    #[doc = "1: Enables interrupts from the data operation circuit."]
    _1 = 1,
}
impl From<DOPCIE_A> for bool {
    #[inline(always)]
    fn from(variant: DOPCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl DOPCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOPCIE_A {
        match self.bits {
            false => DOPCIE_A::_0,
            true => DOPCIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOPCIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOPCIE_A::_1
    }
}
#[doc = "Field `DOPCIE` writer - Data Operation Circuit Interrupt Enable"]
pub type DOPCIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, DOCR_SPEC, DOPCIE_A, O>;
impl<'a, const O: u8> DOPCIE_W<'a, O> {
    #[doc = "Disables interrupts from the data operation circuit."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOPCIE_A::_0)
    }
    #[doc = "Enables interrupts from the data operation circuit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOPCIE_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Operating Mode Select"]
    #[inline(always)]
    pub fn oms(&self) -> OMS_R {
        OMS_R::new(self.bits & 3)
    }
    #[doc = "Bit 3 - Data Operation Bit Width Select"]
    #[inline(always)]
    pub fn dobw(&self) -> DOBW_R {
        DOBW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Detection Condition Select"]
    #[inline(always)]
    pub fn dcsel(&self) -> DCSEL_R {
        DCSEL_R::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - Data Operation Circuit Interrupt Enable"]
    #[inline(always)]
    pub fn dopcie(&self) -> DOPCIE_R {
        DOPCIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Operating Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn oms(&mut self) -> OMS_W<0> {
        OMS_W::new(self)
    }
    #[doc = "Bit 3 - Data Operation Bit Width Select"]
    #[inline(always)]
    #[must_use]
    pub fn dobw(&mut self) -> DOBW_W<3> {
        DOBW_W::new(self)
    }
    #[doc = "Bits 4:6 - Detection Condition Select"]
    #[inline(always)]
    #[must_use]
    pub fn dcsel(&mut self) -> DCSEL_W<4> {
        DCSEL_W::new(self)
    }
    #[doc = "Bit 7 - Data Operation Circuit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dopcie(&mut self) -> DOPCIE_W<7> {
        DOPCIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DOC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [docr](index.html) module"]
pub struct DOCR_SPEC;
impl crate::RegisterSpec for DOCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [docr::R](R) reader structure"]
impl crate::Readable for DOCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [docr::W](W) writer structure"]
impl crate::Writable for DOCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOCR to value 0"]
impl crate::Resettable for DOCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
