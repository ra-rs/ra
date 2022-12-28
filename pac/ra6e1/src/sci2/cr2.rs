#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFCS` reader - RXDXn Signal Digital Filter Clock Select"]
pub type DFCS_R = crate::FieldReader<u8, DFCS_A>;
#[doc = "RXDXn Signal Digital Filter Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DFCS_A {
    #[doc = "0: Filter is disabled."]
    _000 = 0,
    #[doc = "1: Filter clock is SCI base clock"]
    _001 = 1,
    #[doc = "2: Filter clock is PCLK/8"]
    _010 = 2,
    #[doc = "3: Filter clock is PCLK/16"]
    _011 = 3,
    #[doc = "4: Filter clock is PCLK/32"]
    _100 = 4,
    #[doc = "5: Filter clock is PCLK/64"]
    _101 = 5,
    #[doc = "6: Filter clock is PCLK/128"]
    _110 = 6,
    #[doc = "7: Setting prohibited"]
    _111 = 7,
}
impl From<DFCS_A> for u8 {
    #[inline(always)]
    fn from(variant: DFCS_A) -> Self {
        variant as _
    }
}
impl DFCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFCS_A {
        match self.bits {
            0 => DFCS_A::_000,
            1 => DFCS_A::_001,
            2 => DFCS_A::_010,
            3 => DFCS_A::_011,
            4 => DFCS_A::_100,
            5 => DFCS_A::_101,
            6 => DFCS_A::_110,
            7 => DFCS_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DFCS_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DFCS_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DFCS_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DFCS_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DFCS_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DFCS_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == DFCS_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == DFCS_A::_111
    }
}
#[doc = "Field `DFCS` writer - RXDXn Signal Digital Filter Clock Select"]
pub type DFCS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CR2_SPEC, u8, DFCS_A, 3, O>;
impl<'a, const O: u8> DFCS_W<'a, O> {
    #[doc = "Filter is disabled."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(DFCS_A::_000)
    }
    #[doc = "Filter clock is SCI base clock"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(DFCS_A::_001)
    }
    #[doc = "Filter clock is PCLK/8"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(DFCS_A::_010)
    }
    #[doc = "Filter clock is PCLK/16"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(DFCS_A::_011)
    }
    #[doc = "Filter clock is PCLK/32"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(DFCS_A::_100)
    }
    #[doc = "Filter clock is PCLK/64"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(DFCS_A::_101)
    }
    #[doc = "Filter clock is PCLK/128"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(DFCS_A::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(DFCS_A::_111)
    }
}
#[doc = "Field `BCCS` reader - Bus Collision Detection Clock Select"]
pub type BCCS_R = crate::FieldReader<u8, BCCS_A>;
#[doc = "Bus Collision Detection Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BCCS_A {
    #[doc = "0: SCI base clock"]
    _00 = 0,
    #[doc = "1: SCI base clock frequency divided by 2"]
    _01 = 1,
    #[doc = "2: SCI base clock frequency divided by 4"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<BCCS_A> for u8 {
    #[inline(always)]
    fn from(variant: BCCS_A) -> Self {
        variant as _
    }
}
impl BCCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCCS_A {
        match self.bits {
            0 => BCCS_A::_00,
            1 => BCCS_A::_01,
            2 => BCCS_A::_10,
            3 => BCCS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == BCCS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == BCCS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == BCCS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == BCCS_A::_11
    }
}
#[doc = "Field `BCCS` writer - Bus Collision Detection Clock Select"]
pub type BCCS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CR2_SPEC, u8, BCCS_A, 2, O>;
impl<'a, const O: u8> BCCS_W<'a, O> {
    #[doc = "SCI base clock"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(BCCS_A::_00)
    }
    #[doc = "SCI base clock frequency divided by 2"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(BCCS_A::_01)
    }
    #[doc = "SCI base clock frequency divided by 4"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(BCCS_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(BCCS_A::_11)
    }
}
#[doc = "Field `RTS` reader - RXDXn Reception Sampling Timing Select"]
pub type RTS_R = crate::FieldReader<u8, RTS_A>;
#[doc = "RXDXn Reception Sampling Timing Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTS_A {
    #[doc = "0: Rising edge of the 8th cycle of SCI base clock"]
    _00 = 0,
    #[doc = "1: Rising edge of the 10th cycle of SCI base clock"]
    _01 = 1,
    #[doc = "2: Rising edge of the 12th cycle of SCI base clock"]
    _10 = 2,
    #[doc = "3: Rising edge of the 14th cycle of SCI base clock"]
    _11 = 3,
}
impl From<RTS_A> for u8 {
    #[inline(always)]
    fn from(variant: RTS_A) -> Self {
        variant as _
    }
}
impl RTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTS_A {
        match self.bits {
            0 => RTS_A::_00,
            1 => RTS_A::_01,
            2 => RTS_A::_10,
            3 => RTS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RTS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RTS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RTS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == RTS_A::_11
    }
}
#[doc = "Field `RTS` writer - RXDXn Reception Sampling Timing Select"]
pub type RTS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CR2_SPEC, u8, RTS_A, 2, O>;
impl<'a, const O: u8> RTS_W<'a, O> {
    #[doc = "Rising edge of the 8th cycle of SCI base clock"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RTS_A::_00)
    }
    #[doc = "Rising edge of the 10th cycle of SCI base clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RTS_A::_01)
    }
    #[doc = "Rising edge of the 12th cycle of SCI base clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RTS_A::_10)
    }
    #[doc = "Rising edge of the 14th cycle of SCI base clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(RTS_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:2 - RXDXn Signal Digital Filter Clock Select"]
    #[inline(always)]
    pub fn dfcs(&self) -> DFCS_R {
        DFCS_R::new(self.bits & 7)
    }
    #[doc = "Bits 4:5 - Bus Collision Detection Clock Select"]
    #[inline(always)]
    pub fn bccs(&self) -> BCCS_R {
        BCCS_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - RXDXn Reception Sampling Timing Select"]
    #[inline(always)]
    pub fn rts(&self) -> RTS_R {
        RTS_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:2 - RXDXn Signal Digital Filter Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn dfcs(&mut self) -> DFCS_W<0> {
        DFCS_W::new(self)
    }
    #[doc = "Bits 4:5 - Bus Collision Detection Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn bccs(&mut self) -> BCCS_W<4> {
        BCCS_W::new(self)
    }
    #[doc = "Bits 6:7 - RXDXn Reception Sampling Timing Select"]
    #[inline(always)]
    #[must_use]
    pub fn rts(&mut self) -> RTS_W<6> {
        RTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
