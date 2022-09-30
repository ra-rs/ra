#[doc = "Register `RCR1` reader"]
pub struct R(crate::R<RCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCR1` writer"]
pub struct W(crate::W<RCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCR1_SPEC>;
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
impl From<crate::W<RCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AIE` reader - Alarm Interrupt Enable"]
pub type AIE_R = crate::BitReader<AIE_A>;
#[doc = "Alarm Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIE_A {
    #[doc = "0: Disable alarm interrupt requests"]
    _0 = 0,
    #[doc = "1: Enable alarm interrupt requests"]
    _1 = 1,
}
impl From<AIE_A> for bool {
    #[inline(always)]
    fn from(variant: AIE_A) -> Self {
        variant as u8 != 0
    }
}
impl AIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AIE_A {
        match self.bits {
            false => AIE_A::_0,
            true => AIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AIE_A::_1
    }
}
#[doc = "Field `AIE` writer - Alarm Interrupt Enable"]
pub type AIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, RCR1_SPEC, AIE_A, O>;
impl<'a, const O: u8> AIE_W<'a, O> {
    #[doc = "Disable alarm interrupt requests"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AIE_A::_0)
    }
    #[doc = "Enable alarm interrupt requests"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AIE_A::_1)
    }
}
#[doc = "Field `CIE` reader - Carry Interrupt Enable"]
pub type CIE_R = crate::BitReader<CIE_A>;
#[doc = "Carry Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIE_A {
    #[doc = "0: Disable carry interrupt requests"]
    _0 = 0,
    #[doc = "1: Enable carry interrupt requests"]
    _1 = 1,
}
impl From<CIE_A> for bool {
    #[inline(always)]
    fn from(variant: CIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIE_A {
        match self.bits {
            false => CIE_A::_0,
            true => CIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CIE_A::_1
    }
}
#[doc = "Field `CIE` writer - Carry Interrupt Enable"]
pub type CIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, RCR1_SPEC, CIE_A, O>;
impl<'a, const O: u8> CIE_W<'a, O> {
    #[doc = "Disable carry interrupt requests"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CIE_A::_0)
    }
    #[doc = "Enable carry interrupt requests"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CIE_A::_1)
    }
}
#[doc = "Field `PIE` reader - Periodic Interrupt Enable"]
pub type PIE_R = crate::BitReader<PIE_A>;
#[doc = "Periodic Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIE_A {
    #[doc = "0: Disable periodic interrupt requests"]
    _0 = 0,
    #[doc = "1: Enable periodic interrupt requests"]
    _1 = 1,
}
impl From<PIE_A> for bool {
    #[inline(always)]
    fn from(variant: PIE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIE_A {
        match self.bits {
            false => PIE_A::_0,
            true => PIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIE_A::_1
    }
}
#[doc = "Field `PIE` writer - Periodic Interrupt Enable"]
pub type PIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, RCR1_SPEC, PIE_A, O>;
impl<'a, const O: u8> PIE_W<'a, O> {
    #[doc = "Disable periodic interrupt requests"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIE_A::_0)
    }
    #[doc = "Enable periodic interrupt requests"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIE_A::_1)
    }
}
#[doc = "Field `RTCOS` reader - RTCOUT Output Select"]
pub type RTCOS_R = crate::BitReader<RTCOS_A>;
#[doc = "RTCOUT Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCOS_A {
    #[doc = "0: Outputs 1 Hz on RTCOUT"]
    _0 = 0,
    #[doc = "1: Outputs 64 Hz RTCOUT"]
    _1 = 1,
}
impl From<RTCOS_A> for bool {
    #[inline(always)]
    fn from(variant: RTCOS_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCOS_A {
        match self.bits {
            false => RTCOS_A::_0,
            true => RTCOS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTCOS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTCOS_A::_1
    }
}
#[doc = "Field `RTCOS` writer - RTCOUT Output Select"]
pub type RTCOS_W<'a, const O: u8> = crate::BitWriter<'a, u8, RCR1_SPEC, RTCOS_A, O>;
impl<'a, const O: u8> RTCOS_W<'a, O> {
    #[doc = "Outputs 1 Hz on RTCOUT"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTCOS_A::_0)
    }
    #[doc = "Outputs 64 Hz RTCOUT"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTCOS_A::_1)
    }
}
#[doc = "Field `PES` reader - Periodic Interrupt Select"]
pub type PES_R = crate::FieldReader<u8, PES_A>;
#[doc = "Periodic Interrupt Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PES_A {
    #[doc = "6: Generate periodic interrupt every 1/256 second"]
    _0X6 = 6,
    #[doc = "7: Generate periodic interrupt every 1/128 second"]
    _0X7 = 7,
    #[doc = "8: Generate periodic interrupt every 1/64 second"]
    _0X8 = 8,
    #[doc = "9: Generate periodic interrupt every 1/32 second"]
    _0X9 = 9,
    #[doc = "10: Generate periodic interrupt every 1/16 second"]
    _0X_A = 10,
    #[doc = "11: Generate periodic interrupt every 1/8 second"]
    _0X_B = 11,
    #[doc = "12: Generate periodic interrupt every 1/4 second"]
    _0X_C = 12,
    #[doc = "13: Generate periodic interrupt every 1/2 second"]
    _0X_D = 13,
    #[doc = "14: Generate periodic interrupt every 1 second"]
    _0X_E = 14,
    #[doc = "15: Generate periodic interrupt every 2 seconds"]
    _0X_F = 15,
}
impl From<PES_A> for u8 {
    #[inline(always)]
    fn from(variant: PES_A) -> Self {
        variant as _
    }
}
impl PES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PES_A> {
        match self.bits {
            6 => Some(PES_A::_0X6),
            7 => Some(PES_A::_0X7),
            8 => Some(PES_A::_0X8),
            9 => Some(PES_A::_0X9),
            10 => Some(PES_A::_0X_A),
            11 => Some(PES_A::_0X_B),
            12 => Some(PES_A::_0X_C),
            13 => Some(PES_A::_0X_D),
            14 => Some(PES_A::_0X_E),
            15 => Some(PES_A::_0X_F),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X6`"]
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == PES_A::_0X6
    }
    #[doc = "Checks if the value of the field is `_0X7`"]
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == PES_A::_0X7
    }
    #[doc = "Checks if the value of the field is `_0X8`"]
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == PES_A::_0X8
    }
    #[doc = "Checks if the value of the field is `_0X9`"]
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == PES_A::_0X9
    }
    #[doc = "Checks if the value of the field is `_0X_A`"]
    #[inline(always)]
    pub fn is_0x_a(&self) -> bool {
        *self == PES_A::_0X_A
    }
    #[doc = "Checks if the value of the field is `_0X_B`"]
    #[inline(always)]
    pub fn is_0x_b(&self) -> bool {
        *self == PES_A::_0X_B
    }
    #[doc = "Checks if the value of the field is `_0X_C`"]
    #[inline(always)]
    pub fn is_0x_c(&self) -> bool {
        *self == PES_A::_0X_C
    }
    #[doc = "Checks if the value of the field is `_0X_D`"]
    #[inline(always)]
    pub fn is_0x_d(&self) -> bool {
        *self == PES_A::_0X_D
    }
    #[doc = "Checks if the value of the field is `_0X_E`"]
    #[inline(always)]
    pub fn is_0x_e(&self) -> bool {
        *self == PES_A::_0X_E
    }
    #[doc = "Checks if the value of the field is `_0X_F`"]
    #[inline(always)]
    pub fn is_0x_f(&self) -> bool {
        *self == PES_A::_0X_F
    }
}
#[doc = "Field `PES` writer - Periodic Interrupt Select"]
pub type PES_W<'a, const O: u8> = crate::FieldWriter<'a, u8, RCR1_SPEC, u8, PES_A, 4, O>;
impl<'a, const O: u8> PES_W<'a, O> {
    #[doc = "Generate periodic interrupt every 1/256 second"]
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut W {
        self.variant(PES_A::_0X6)
    }
    #[doc = "Generate periodic interrupt every 1/128 second"]
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut W {
        self.variant(PES_A::_0X7)
    }
    #[doc = "Generate periodic interrupt every 1/64 second"]
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut W {
        self.variant(PES_A::_0X8)
    }
    #[doc = "Generate periodic interrupt every 1/32 second"]
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut W {
        self.variant(PES_A::_0X9)
    }
    #[doc = "Generate periodic interrupt every 1/16 second"]
    #[inline(always)]
    pub fn _0x_a(self) -> &'a mut W {
        self.variant(PES_A::_0X_A)
    }
    #[doc = "Generate periodic interrupt every 1/8 second"]
    #[inline(always)]
    pub fn _0x_b(self) -> &'a mut W {
        self.variant(PES_A::_0X_B)
    }
    #[doc = "Generate periodic interrupt every 1/4 second"]
    #[inline(always)]
    pub fn _0x_c(self) -> &'a mut W {
        self.variant(PES_A::_0X_C)
    }
    #[doc = "Generate periodic interrupt every 1/2 second"]
    #[inline(always)]
    pub fn _0x_d(self) -> &'a mut W {
        self.variant(PES_A::_0X_D)
    }
    #[doc = "Generate periodic interrupt every 1 second"]
    #[inline(always)]
    pub fn _0x_e(self) -> &'a mut W {
        self.variant(PES_A::_0X_E)
    }
    #[doc = "Generate periodic interrupt every 2 seconds"]
    #[inline(always)]
    pub fn _0x_f(self) -> &'a mut W {
        self.variant(PES_A::_0X_F)
    }
}
impl R {
    #[doc = "Bit 0 - Alarm Interrupt Enable"]
    #[inline(always)]
    pub fn aie(&self) -> AIE_R {
        AIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Carry Interrupt Enable"]
    #[inline(always)]
    pub fn cie(&self) -> CIE_R {
        CIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Periodic Interrupt Enable"]
    #[inline(always)]
    pub fn pie(&self) -> PIE_R {
        PIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTCOUT Output Select"]
    #[inline(always)]
    pub fn rtcos(&self) -> RTCOS_R {
        RTCOS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Periodic Interrupt Select"]
    #[inline(always)]
    pub fn pes(&self) -> PES_R {
        PES_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Alarm Interrupt Enable"]
    #[inline(always)]
    pub fn aie(&mut self) -> AIE_W<0> {
        AIE_W::new(self)
    }
    #[doc = "Bit 1 - Carry Interrupt Enable"]
    #[inline(always)]
    pub fn cie(&mut self) -> CIE_W<1> {
        CIE_W::new(self)
    }
    #[doc = "Bit 2 - Periodic Interrupt Enable"]
    #[inline(always)]
    pub fn pie(&mut self) -> PIE_W<2> {
        PIE_W::new(self)
    }
    #[doc = "Bit 3 - RTCOUT Output Select"]
    #[inline(always)]
    pub fn rtcos(&mut self) -> RTCOS_W<3> {
        RTCOS_W::new(self)
    }
    #[doc = "Bits 4:7 - Periodic Interrupt Select"]
    #[inline(always)]
    pub fn pes(&mut self) -> PES_W<4> {
        PES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcr1](index.html) module"]
pub struct RCR1_SPEC;
impl crate::RegisterSpec for RCR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rcr1::R](R) reader structure"]
impl crate::Readable for RCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcr1::W](W) writer structure"]
impl crate::Writable for RCR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCR1 to value 0"]
impl crate::Resettable for RCR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
