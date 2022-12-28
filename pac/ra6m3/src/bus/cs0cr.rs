#[doc = "Register `CS0CR` reader"]
pub struct R(crate::R<CS0CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CS0CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CS0CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CS0CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CS0CR` writer"]
pub struct W(crate::W<CS0CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CS0CR_SPEC>;
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
impl From<crate::W<CS0CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CS0CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXENB` reader - Operation Enable"]
pub type EXENB_R = crate::BitReader<EXENB_A>;
#[doc = "Operation Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXENB_A {
    #[doc = "0: Disable operation"]
    _0 = 0,
    #[doc = "1: Enable operation"]
    _1 = 1,
}
impl From<EXENB_A> for bool {
    #[inline(always)]
    fn from(variant: EXENB_A) -> Self {
        variant as u8 != 0
    }
}
impl EXENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXENB_A {
        match self.bits {
            false => EXENB_A::_0,
            true => EXENB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXENB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXENB_A::_1
    }
}
#[doc = "Field `EXENB` writer - Operation Enable"]
pub type EXENB_W<'a, const O: u8> = crate::BitWriter<'a, u16, CS0CR_SPEC, EXENB_A, O>;
impl<'a, const O: u8> EXENB_W<'a, O> {
    #[doc = "Disable operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXENB_A::_0)
    }
    #[doc = "Enable operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXENB_A::_1)
    }
}
#[doc = "Field `BSIZE` reader - External Bus Width Select"]
pub type BSIZE_R = crate::FieldReader<u8, BSIZE_A>;
#[doc = "External Bus Width Select\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BSIZE_A {
    #[doc = "0: A 16-bit bus space"]
    _00 = 0,
    #[doc = "1: Setting prohibited"]
    _01 = 1,
    #[doc = "2: An 8-bit bus space"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<BSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: BSIZE_A) -> Self {
        variant as _
    }
}
impl BSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSIZE_A {
        match self.bits {
            0 => BSIZE_A::_00,
            1 => BSIZE_A::_01,
            2 => BSIZE_A::_10,
            3 => BSIZE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == BSIZE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == BSIZE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == BSIZE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == BSIZE_A::_11
    }
}
#[doc = "Field `BSIZE` writer - External Bus Width Select"]
pub type BSIZE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, CS0CR_SPEC, u8, BSIZE_A, 2, O>;
impl<'a, const O: u8> BSIZE_W<'a, O> {
    #[doc = "A 16-bit bus space"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(BSIZE_A::_00)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(BSIZE_A::_01)
    }
    #[doc = "An 8-bit bus space"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(BSIZE_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(BSIZE_A::_11)
    }
}
#[doc = "Field `EMODE` reader - Endian Mode"]
pub type EMODE_R = crate::BitReader<EMODE_A>;
#[doc = "Endian Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EMODE_A {
    #[doc = "0: Little Endian"]
    _0 = 0,
    #[doc = "1: Big Endian"]
    _1 = 1,
}
impl From<EMODE_A> for bool {
    #[inline(always)]
    fn from(variant: EMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl EMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMODE_A {
        match self.bits {
            false => EMODE_A::_0,
            true => EMODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EMODE_A::_1
    }
}
#[doc = "Field `EMODE` writer - Endian Mode"]
pub type EMODE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CS0CR_SPEC, EMODE_A, O>;
impl<'a, const O: u8> EMODE_W<'a, O> {
    #[doc = "Little Endian"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EMODE_A::_0)
    }
    #[doc = "Big Endian"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EMODE_A::_1)
    }
}
#[doc = "Field `MPXEN` reader - Address/Data Multiplexed I/O Interface Select"]
pub type MPXEN_R = crate::BitReader<MPXEN_A>;
#[doc = "Address/Data Multiplexed I/O Interface Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPXEN_A {
    #[doc = "0: Separate bus interface is selected for area n"]
    _0 = 0,
    #[doc = "1: Address/data multiplexed I/O interface is selected for area n. (n = 0 to 7)"]
    _1 = 1,
}
impl From<MPXEN_A> for bool {
    #[inline(always)]
    fn from(variant: MPXEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MPXEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPXEN_A {
        match self.bits {
            false => MPXEN_A::_0,
            true => MPXEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPXEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPXEN_A::_1
    }
}
#[doc = "Field `MPXEN` writer - Address/Data Multiplexed I/O Interface Select"]
pub type MPXEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CS0CR_SPEC, MPXEN_A, O>;
impl<'a, const O: u8> MPXEN_W<'a, O> {
    #[doc = "Separate bus interface is selected for area n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPXEN_A::_0)
    }
    #[doc = "Address/data multiplexed I/O interface is selected for area n. (n = 0 to 7)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPXEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Operation Enable"]
    #[inline(always)]
    pub fn exenb(&self) -> EXENB_R {
        EXENB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - External Bus Width Select"]
    #[inline(always)]
    pub fn bsize(&self) -> BSIZE_R {
        BSIZE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Endian Mode"]
    #[inline(always)]
    pub fn emode(&self) -> EMODE_R {
        EMODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Address/Data Multiplexed I/O Interface Select"]
    #[inline(always)]
    pub fn mpxen(&self) -> MPXEN_R {
        MPXEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn exenb(&mut self) -> EXENB_W<0> {
        EXENB_W::new(self)
    }
    #[doc = "Bits 4:5 - External Bus Width Select"]
    #[inline(always)]
    #[must_use]
    pub fn bsize(&mut self) -> BSIZE_W<4> {
        BSIZE_W::new(self)
    }
    #[doc = "Bit 8 - Endian Mode"]
    #[inline(always)]
    #[must_use]
    pub fn emode(&mut self) -> EMODE_W<8> {
        EMODE_W::new(self)
    }
    #[doc = "Bit 12 - Address/Data Multiplexed I/O Interface Select"]
    #[inline(always)]
    #[must_use]
    pub fn mpxen(&mut self) -> MPXEN_W<12> {
        MPXEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CS0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cs0cr](index.html) module"]
pub struct CS0CR_SPEC;
impl crate::RegisterSpec for CS0CR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cs0cr::R](R) reader structure"]
impl crate::Readable for CS0CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cs0cr::W](W) writer structure"]
impl crate::Writable for CS0CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CS0CR to value 0x21"]
impl crate::Resettable for CS0CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x21;
}
