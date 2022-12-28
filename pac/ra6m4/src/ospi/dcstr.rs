#[doc = "Register `DCSTR` reader"]
pub struct R(crate::R<DCSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCSTR` writer"]
pub struct W(crate::W<DCSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCSTR_SPEC>;
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
impl From<crate::W<DCSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DVSELCMD` reader - Device Command execution interval setting"]
pub type DVSELCMD_R = crate::FieldReader<u8, DVSELCMD_A>;
#[doc = "Device Command execution interval setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DVSELCMD_A {
    #[doc = "0: 2 clock cycles"]
    _000 = 0,
    #[doc = "1: 5 clock cycles"]
    _001 = 1,
    #[doc = "2: 7 clock cycles"]
    _010 = 2,
    #[doc = "3: 9 clock cycles"]
    _011 = 3,
    #[doc = "4: 11 clock cycles"]
    _100 = 4,
    #[doc = "5: 13 clock cycles"]
    _101 = 5,
    #[doc = "6: 15 clock cycles"]
    _110 = 6,
    #[doc = "7: 17 clock cycles"]
    _111 = 7,
}
impl From<DVSELCMD_A> for u8 {
    #[inline(always)]
    fn from(variant: DVSELCMD_A) -> Self {
        variant as _
    }
}
impl DVSELCMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVSELCMD_A {
        match self.bits {
            0 => DVSELCMD_A::_000,
            1 => DVSELCMD_A::_001,
            2 => DVSELCMD_A::_010,
            3 => DVSELCMD_A::_011,
            4 => DVSELCMD_A::_100,
            5 => DVSELCMD_A::_101,
            6 => DVSELCMD_A::_110,
            7 => DVSELCMD_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DVSELCMD_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DVSELCMD_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DVSELCMD_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DVSELCMD_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DVSELCMD_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DVSELCMD_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == DVSELCMD_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == DVSELCMD_A::_111
    }
}
#[doc = "Field `DVSELCMD` writer - Device Command execution interval setting"]
pub type DVSELCMD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DCSTR_SPEC, u8, DVSELCMD_A, 3, O>;
impl<'a, const O: u8> DVSELCMD_W<'a, O> {
    #[doc = "2 clock cycles"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(DVSELCMD_A::_000)
    }
    #[doc = "5 clock cycles"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(DVSELCMD_A::_001)
    }
    #[doc = "7 clock cycles"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(DVSELCMD_A::_010)
    }
    #[doc = "9 clock cycles"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(DVSELCMD_A::_011)
    }
    #[doc = "11 clock cycles"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(DVSELCMD_A::_100)
    }
    #[doc = "13 clock cycles"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(DVSELCMD_A::_101)
    }
    #[doc = "15 clock cycles"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(DVSELCMD_A::_110)
    }
    #[doc = "17 clock cycles"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(DVSELCMD_A::_111)
    }
}
#[doc = "Field `DVSELHI` reader - Device select signal pull-up timing setting"]
pub type DVSELHI_R = crate::FieldReader<u8, DVSELHI_A>;
#[doc = "Device select signal pull-up timing setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DVSELHI_A {
    #[doc = "0: Setting prohibited"]
    _000 = 0,
    #[doc = "1: Setting prohibited"]
    _001 = 1,
    #[doc = "2: Setting prohibited"]
    _010 = 2,
    #[doc = "3: Setting prohibited (DOPI mode) 5 clock cycles (Other mode)"]
    _011 = 3,
    #[doc = "4: Setting prohibited (DOPI mode) 6 clock cycles (Other mode)"]
    _100 = 4,
    #[doc = "5: 6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)"]
    _101 = 5,
    #[doc = "6: 7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)"]
    _110 = 6,
    #[doc = "7: 8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)"]
    _111 = 7,
}
impl From<DVSELHI_A> for u8 {
    #[inline(always)]
    fn from(variant: DVSELHI_A) -> Self {
        variant as _
    }
}
impl DVSELHI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVSELHI_A {
        match self.bits {
            0 => DVSELHI_A::_000,
            1 => DVSELHI_A::_001,
            2 => DVSELHI_A::_010,
            3 => DVSELHI_A::_011,
            4 => DVSELHI_A::_100,
            5 => DVSELHI_A::_101,
            6 => DVSELHI_A::_110,
            7 => DVSELHI_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DVSELHI_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DVSELHI_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DVSELHI_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DVSELHI_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DVSELHI_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DVSELHI_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == DVSELHI_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == DVSELHI_A::_111
    }
}
#[doc = "Field `DVSELHI` writer - Device select signal pull-up timing setting"]
pub type DVSELHI_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DCSTR_SPEC, u8, DVSELHI_A, 3, O>;
impl<'a, const O: u8> DVSELHI_W<'a, O> {
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(DVSELHI_A::_000)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(DVSELHI_A::_001)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(DVSELHI_A::_010)
    }
    #[doc = "Setting prohibited (DOPI mode) 5 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(DVSELHI_A::_011)
    }
    #[doc = "Setting prohibited (DOPI mode) 6 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(DVSELHI_A::_100)
    }
    #[doc = "6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(DVSELHI_A::_101)
    }
    #[doc = "7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(DVSELHI_A::_110)
    }
    #[doc = "8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(DVSELHI_A::_111)
    }
}
#[doc = "Field `DVSELLO` reader - Device select signal pull-down timing setting"]
pub type DVSELLO_R = crate::FieldReader<u8, DVSELLO_A>;
#[doc = "Device select signal pull-down timing setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DVSELLO_A {
    #[doc = "0: Setting prohibit"]
    _00 = 0,
    #[doc = "1: 2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)"]
    _01 = 1,
    #[doc = "2: 3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)"]
    _10 = 2,
    #[doc = "3: 4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)"]
    _11 = 3,
}
impl From<DVSELLO_A> for u8 {
    #[inline(always)]
    fn from(variant: DVSELLO_A) -> Self {
        variant as _
    }
}
impl DVSELLO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVSELLO_A {
        match self.bits {
            0 => DVSELLO_A::_00,
            1 => DVSELLO_A::_01,
            2 => DVSELLO_A::_10,
            3 => DVSELLO_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DVSELLO_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DVSELLO_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DVSELLO_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DVSELLO_A::_11
    }
}
#[doc = "Field `DVSELLO` writer - Device select signal pull-down timing setting"]
pub type DVSELLO_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DCSTR_SPEC, u8, DVSELLO_A, 2, O>;
impl<'a, const O: u8> DVSELLO_W<'a, O> {
    #[doc = "Setting prohibit"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DVSELLO_A::_00)
    }
    #[doc = "2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DVSELLO_A::_01)
    }
    #[doc = "3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DVSELLO_A::_10)
    }
    #[doc = "4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DVSELLO_A::_11)
    }
}
impl R {
    #[doc = "Bits 8:10 - Device Command execution interval setting"]
    #[inline(always)]
    pub fn dvselcmd(&self) -> DVSELCMD_R {
        DVSELCMD_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Device select signal pull-up timing setting"]
    #[inline(always)]
    pub fn dvselhi(&self) -> DVSELHI_R {
        DVSELHI_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:15 - Device select signal pull-down timing setting"]
    #[inline(always)]
    pub fn dvsello(&self) -> DVSELLO_R {
        DVSELLO_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - Device Command execution interval setting"]
    #[inline(always)]
    #[must_use]
    pub fn dvselcmd(&mut self) -> DVSELCMD_W<8> {
        DVSELCMD_W::new(self)
    }
    #[doc = "Bits 11:13 - Device select signal pull-up timing setting"]
    #[inline(always)]
    #[must_use]
    pub fn dvselhi(&mut self) -> DVSELHI_W<11> {
        DVSELHI_W::new(self)
    }
    #[doc = "Bits 14:15 - Device select signal pull-down timing setting"]
    #[inline(always)]
    #[must_use]
    pub fn dvsello(&mut self) -> DVSELLO_W<14> {
        DVSELLO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Chip Select Timing Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcstr](index.html) module"]
pub struct DCSTR_SPEC;
impl crate::RegisterSpec for DCSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcstr::R](R) reader structure"]
impl crate::Readable for DCSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcstr::W](W) writer structure"]
impl crate::Writable for DCSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCSTR to value 0"]
impl crate::Resettable for DCSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
