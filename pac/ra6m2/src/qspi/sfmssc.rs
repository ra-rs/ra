#[doc = "Register `SFMSSC` reader"]
pub struct R(crate::R<SFMSSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFMSSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFMSSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFMSSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFMSSC` writer"]
pub struct W(crate::W<SFMSSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFMSSC_SPEC>;
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
impl From<crate::W<SFMSSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFMSSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFMSW` reader - Selection of a minimum high-level width of the QSSL signal"]
pub type SFMSW_R = crate::FieldReader<u8, SFMSW_A>;
#[doc = "Selection of a minimum high-level width of the QSSL signal\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SFMSW_A {
    #[doc = "0: 1 x QSPCLK"]
    _0000 = 0,
    #[doc = "1: 2 x QSPCLK"]
    _0001 = 1,
    #[doc = "2: 3 x QSPCLK"]
    _0010 = 2,
    #[doc = "3: 4 x QSPCLK"]
    _0011 = 3,
    #[doc = "4: 5 x QSPCLK"]
    _0100 = 4,
    #[doc = "5: 6 x QSPCLK"]
    _0101 = 5,
    #[doc = "6: 7 x QSPCLK"]
    _0110 = 6,
    #[doc = "7: 8 x QSPCLK"]
    _0111 = 7,
    #[doc = "8: 9 x QSPCLK"]
    _1000 = 8,
    #[doc = "9: 10 x QSPCLK"]
    _1001 = 9,
    #[doc = "10: 11 x QSPCLK"]
    _1010 = 10,
    #[doc = "11: 12 x QSPCLK"]
    _1011 = 11,
    #[doc = "12: 13 x QSPCLK"]
    _1100 = 12,
    #[doc = "13: 14 x QSPCLK"]
    _1101 = 13,
    #[doc = "14: 15 x QSPCLK"]
    _1110 = 14,
    #[doc = "15: 16 x QSPCLK"]
    _1111 = 15,
}
impl From<SFMSW_A> for u8 {
    #[inline(always)]
    fn from(variant: SFMSW_A) -> Self {
        variant as _
    }
}
impl SFMSW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFMSW_A {
        match self.bits {
            0 => SFMSW_A::_0000,
            1 => SFMSW_A::_0001,
            2 => SFMSW_A::_0010,
            3 => SFMSW_A::_0011,
            4 => SFMSW_A::_0100,
            5 => SFMSW_A::_0101,
            6 => SFMSW_A::_0110,
            7 => SFMSW_A::_0111,
            8 => SFMSW_A::_1000,
            9 => SFMSW_A::_1001,
            10 => SFMSW_A::_1010,
            11 => SFMSW_A::_1011,
            12 => SFMSW_A::_1100,
            13 => SFMSW_A::_1101,
            14 => SFMSW_A::_1110,
            15 => SFMSW_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == SFMSW_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == SFMSW_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == SFMSW_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == SFMSW_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == SFMSW_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == SFMSW_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == SFMSW_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == SFMSW_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == SFMSW_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == SFMSW_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == SFMSW_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == SFMSW_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == SFMSW_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == SFMSW_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == SFMSW_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == SFMSW_A::_1111
    }
}
#[doc = "Field `SFMSW` writer - Selection of a minimum high-level width of the QSSL signal"]
pub type SFMSW_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SFMSSC_SPEC, u8, SFMSW_A, 4, O>;
impl<'a, const O: u8> SFMSW_W<'a, O> {
    #[doc = "1 x QSPCLK"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(SFMSW_A::_0000)
    }
    #[doc = "2 x QSPCLK"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(SFMSW_A::_0001)
    }
    #[doc = "3 x QSPCLK"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(SFMSW_A::_0010)
    }
    #[doc = "4 x QSPCLK"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(SFMSW_A::_0011)
    }
    #[doc = "5 x QSPCLK"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(SFMSW_A::_0100)
    }
    #[doc = "6 x QSPCLK"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(SFMSW_A::_0101)
    }
    #[doc = "7 x QSPCLK"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(SFMSW_A::_0110)
    }
    #[doc = "8 x QSPCLK"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(SFMSW_A::_0111)
    }
    #[doc = "9 x QSPCLK"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(SFMSW_A::_1000)
    }
    #[doc = "10 x QSPCLK"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(SFMSW_A::_1001)
    }
    #[doc = "11 x QSPCLK"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(SFMSW_A::_1010)
    }
    #[doc = "12 x QSPCLK"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(SFMSW_A::_1011)
    }
    #[doc = "13 x QSPCLK"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(SFMSW_A::_1100)
    }
    #[doc = "14 x QSPCLK"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(SFMSW_A::_1101)
    }
    #[doc = "15 x QSPCLK"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(SFMSW_A::_1110)
    }
    #[doc = "16 x QSPCLK"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(SFMSW_A::_1111)
    }
}
#[doc = "Field `SFMSHD` reader - QSSL signal release timing selection"]
pub type SFMSHD_R = crate::BitReader<SFMSHD_A>;
#[doc = "QSSL signal release timing selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMSHD_A {
    #[doc = "0: Releases QSSL 0.5*SCK after the last rising edge of QSPCLK"]
    _0 = 0,
    #[doc = "1: Releases QSSL 1.5*SCK after the last rising edge of QSPCLK"]
    _1 = 1,
}
impl From<SFMSHD_A> for bool {
    #[inline(always)]
    fn from(variant: SFMSHD_A) -> Self {
        variant as u8 != 0
    }
}
impl SFMSHD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFMSHD_A {
        match self.bits {
            false => SFMSHD_A::_0,
            true => SFMSHD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMSHD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMSHD_A::_1
    }
}
#[doc = "Field `SFMSHD` writer - QSSL signal release timing selection"]
pub type SFMSHD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFMSSC_SPEC, SFMSHD_A, O>;
impl<'a, const O: u8> SFMSHD_W<'a, O> {
    #[doc = "Releases QSSL 0.5*SCK after the last rising edge of QSPCLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SFMSHD_A::_0)
    }
    #[doc = "Releases QSSL 1.5*SCK after the last rising edge of QSPCLK"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SFMSHD_A::_1)
    }
}
#[doc = "Field `SFMSLD` reader - QSSL signal output timing selection"]
pub type SFMSLD_R = crate::BitReader<SFMSLD_A>;
#[doc = "QSSL signal output timing selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMSLD_A {
    #[doc = "0: Outputs QSSL 0.5*SCK before the first rising edge of QSPCLK"]
    _0 = 0,
    #[doc = "1: Outputs QSSL 1.5*SCK before the first rising edge of QSPCLK"]
    _1 = 1,
}
impl From<SFMSLD_A> for bool {
    #[inline(always)]
    fn from(variant: SFMSLD_A) -> Self {
        variant as u8 != 0
    }
}
impl SFMSLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFMSLD_A {
        match self.bits {
            false => SFMSLD_A::_0,
            true => SFMSLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMSLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMSLD_A::_1
    }
}
#[doc = "Field `SFMSLD` writer - QSSL signal output timing selection"]
pub type SFMSLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFMSSC_SPEC, SFMSLD_A, O>;
impl<'a, const O: u8> SFMSLD_W<'a, O> {
    #[doc = "Outputs QSSL 0.5*SCK before the first rising edge of QSPCLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SFMSLD_A::_0)
    }
    #[doc = "Outputs QSSL 1.5*SCK before the first rising edge of QSPCLK"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SFMSLD_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Selection of a minimum high-level width of the QSSL signal"]
    #[inline(always)]
    pub fn sfmsw(&self) -> SFMSW_R {
        SFMSW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - QSSL signal release timing selection"]
    #[inline(always)]
    pub fn sfmshd(&self) -> SFMSHD_R {
        SFMSHD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - QSSL signal output timing selection"]
    #[inline(always)]
    pub fn sfmsld(&self) -> SFMSLD_R {
        SFMSLD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selection of a minimum high-level width of the QSSL signal"]
    #[inline(always)]
    #[must_use]
    pub fn sfmsw(&mut self) -> SFMSW_W<0> {
        SFMSW_W::new(self)
    }
    #[doc = "Bit 4 - QSSL signal release timing selection"]
    #[inline(always)]
    #[must_use]
    pub fn sfmshd(&mut self) -> SFMSHD_W<4> {
        SFMSHD_W::new(self)
    }
    #[doc = "Bit 5 - QSSL signal output timing selection"]
    #[inline(always)]
    #[must_use]
    pub fn sfmsld(&mut self) -> SFMSLD_W<5> {
        SFMSLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Chip Selection Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfmssc](index.html) module"]
pub struct SFMSSC_SPEC;
impl crate::RegisterSpec for SFMSSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfmssc::R](R) reader structure"]
impl crate::Readable for SFMSSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfmssc::W](W) writer structure"]
impl crate::Writable for SFMSSC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFMSSC to value 0x37"]
impl crate::Resettable for SFMSSC_SPEC {
    const RESET_VALUE: Self::Ux = 0x37;
}
