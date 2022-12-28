#[doc = "Register `TMR` reader"]
pub struct R(crate::R<TMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMR` writer"]
pub struct W(crate::W<TMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMR_SPEC>;
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
impl From<crate::W<TMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOMS` reader - Timer Operating Mode Select"]
pub type TOMS_R = crate::FieldReader<u8, TOMS_A>;
#[doc = "Timer Operating Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TOMS_A {
    #[doc = "0: Timer mode"]
    _00 = 0,
    #[doc = "1: Break Field low width determination mode"]
    _01 = 1,
    #[doc = "2: Break Field low width output mode"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<TOMS_A> for u8 {
    #[inline(always)]
    fn from(variant: TOMS_A) -> Self {
        variant as _
    }
}
impl TOMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOMS_A {
        match self.bits {
            0 => TOMS_A::_00,
            1 => TOMS_A::_01,
            2 => TOMS_A::_10,
            3 => TOMS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TOMS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TOMS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TOMS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TOMS_A::_11
    }
}
#[doc = "Field `TOMS` writer - Timer Operating Mode Select"]
pub type TOMS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TMR_SPEC, u8, TOMS_A, 2, O>;
impl<'a, const O: u8> TOMS_W<'a, O> {
    #[doc = "Timer mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TOMS_A::_00)
    }
    #[doc = "Break Field low width determination mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TOMS_A::_01)
    }
    #[doc = "Break Field low width output mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TOMS_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TOMS_A::_11)
    }
}
#[doc = "Field `TWRC` reader - Counter Write Control"]
pub type TWRC_R = crate::BitReader<TWRC_A>;
#[doc = "Counter Write Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWRC_A {
    #[doc = "0: Data is written to the reload register and counter"]
    _0 = 0,
    #[doc = "1: Data is written to the reload register only"]
    _1 = 1,
}
impl From<TWRC_A> for bool {
    #[inline(always)]
    fn from(variant: TWRC_A) -> Self {
        variant as u8 != 0
    }
}
impl TWRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWRC_A {
        match self.bits {
            false => TWRC_A::_0,
            true => TWRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWRC_A::_1
    }
}
#[doc = "Field `TWRC` writer - Counter Write Control"]
pub type TWRC_W<'a, const O: u8> = crate::BitWriter<'a, u8, TMR_SPEC, TWRC_A, O>;
impl<'a, const O: u8> TWRC_W<'a, O> {
    #[doc = "Data is written to the reload register and counter"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWRC_A::_0)
    }
    #[doc = "Data is written to the reload register only"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWRC_A::_1)
    }
}
#[doc = "Field `TCSS` reader - Timer Count Clock Source Select"]
pub type TCSS_R = crate::FieldReader<u8, TCSS_A>;
#[doc = "Timer Count Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCSS_A {
    #[doc = "0: PCLK"]
    _000 = 0,
    #[doc = "1: PCLK/2"]
    _001 = 1,
    #[doc = "2: PCLK/4"]
    _010 = 2,
    #[doc = "3: PCLK/8"]
    _011 = 3,
    #[doc = "4: PCLK/16"]
    _100 = 4,
    #[doc = "5: PCLK/32"]
    _101 = 5,
    #[doc = "6: PCLK/64"]
    _110 = 6,
    #[doc = "7: PCLK/128"]
    _111 = 7,
}
impl From<TCSS_A> for u8 {
    #[inline(always)]
    fn from(variant: TCSS_A) -> Self {
        variant as _
    }
}
impl TCSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCSS_A {
        match self.bits {
            0 => TCSS_A::_000,
            1 => TCSS_A::_001,
            2 => TCSS_A::_010,
            3 => TCSS_A::_011,
            4 => TCSS_A::_100,
            5 => TCSS_A::_101,
            6 => TCSS_A::_110,
            7 => TCSS_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == TCSS_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == TCSS_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == TCSS_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == TCSS_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == TCSS_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == TCSS_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == TCSS_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == TCSS_A::_111
    }
}
#[doc = "Field `TCSS` writer - Timer Count Clock Source Select"]
pub type TCSS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TMR_SPEC, u8, TCSS_A, 3, O>;
impl<'a, const O: u8> TCSS_W<'a, O> {
    #[doc = "PCLK"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(TCSS_A::_000)
    }
    #[doc = "PCLK/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(TCSS_A::_001)
    }
    #[doc = "PCLK/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(TCSS_A::_010)
    }
    #[doc = "PCLK/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(TCSS_A::_011)
    }
    #[doc = "PCLK/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(TCSS_A::_100)
    }
    #[doc = "PCLK/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(TCSS_A::_101)
    }
    #[doc = "PCLK/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(TCSS_A::_110)
    }
    #[doc = "PCLK/128"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(TCSS_A::_111)
    }
}
impl R {
    #[doc = "Bits 0:1 - Timer Operating Mode Select"]
    #[inline(always)]
    pub fn toms(&self) -> TOMS_R {
        TOMS_R::new(self.bits & 3)
    }
    #[doc = "Bit 3 - Counter Write Control"]
    #[inline(always)]
    pub fn twrc(&self) -> TWRC_R {
        TWRC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Timer Count Clock Source Select"]
    #[inline(always)]
    pub fn tcss(&self) -> TCSS_R {
        TCSS_R::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer Operating Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn toms(&mut self) -> TOMS_W<0> {
        TOMS_W::new(self)
    }
    #[doc = "Bit 3 - Counter Write Control"]
    #[inline(always)]
    #[must_use]
    pub fn twrc(&mut self) -> TWRC_W<3> {
        TWRC_W::new(self)
    }
    #[doc = "Bits 4:6 - Timer Count Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn tcss(&mut self) -> TCSS_W<4> {
        TCSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr](index.html) module"]
pub struct TMR_SPEC;
impl crate::RegisterSpec for TMR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tmr::R](R) reader structure"]
impl crate::Readable for TMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmr::W](W) writer structure"]
impl crate::Writable for TMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMR to value 0"]
impl crate::Resettable for TMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
