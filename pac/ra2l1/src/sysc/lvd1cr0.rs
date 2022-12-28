#[doc = "Register `LVD1CR0` reader"]
pub struct R(crate::R<LVD1CR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LVD1CR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LVD1CR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LVD1CR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LVD1CR0` writer"]
pub struct W(crate::W<LVD1CR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LVD1CR0_SPEC>;
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
impl From<crate::W<LVD1CR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LVD1CR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RIE` reader - Voltage Monitor 1 Interrupt/Reset Enable"]
pub type RIE_R = crate::BitReader<RIE_A>;
#[doc = "Voltage Monitor 1 Interrupt/Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RIE_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<RIE_A> for bool {
    #[inline(always)]
    fn from(variant: RIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIE_A {
        match self.bits {
            false => RIE_A::_0,
            true => RIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RIE_A::_1
    }
}
#[doc = "Field `RIE` writer - Voltage Monitor 1 Interrupt/Reset Enable"]
pub type RIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, LVD1CR0_SPEC, RIE_A, O>;
impl<'a, const O: u8> RIE_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RIE_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RIE_A::_1)
    }
}
#[doc = "Field `CMPE` reader - Voltage Monitor 1 Circuit Comparison Result Output Enable"]
pub type CMPE_R = crate::BitReader<CMPE_A>;
#[doc = "Voltage Monitor 1 Circuit Comparison Result Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPE_A {
    #[doc = "0: Disable voltage monitor 1 circuit comparison result output"]
    _0 = 0,
    #[doc = "1: Enable voltage monitor 1 circuit comparison result output"]
    _1 = 1,
}
impl From<CMPE_A> for bool {
    #[inline(always)]
    fn from(variant: CMPE_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPE_A {
        match self.bits {
            false => CMPE_A::_0,
            true => CMPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPE_A::_1
    }
}
#[doc = "Field `CMPE` writer - Voltage Monitor 1 Circuit Comparison Result Output Enable"]
pub type CMPE_W<'a, const O: u8> = crate::BitWriter<'a, u8, LVD1CR0_SPEC, CMPE_A, O>;
impl<'a, const O: u8> CMPE_W<'a, O> {
    #[doc = "Disable voltage monitor 1 circuit comparison result output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPE_A::_0)
    }
    #[doc = "Enable voltage monitor 1 circuit comparison result output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPE_A::_1)
    }
}
#[doc = "Field `RI` reader - Voltage Monitor 1 Circuit Mode Select"]
pub type RI_R = crate::BitReader<RI_A>;
#[doc = "Voltage Monitor 1 Circuit Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RI_A {
    #[doc = "0: Generate voltage monitor 1 interrupt on Vdet1 crossing"]
    _0 = 0,
    #[doc = "1: Enable voltage monitor 1 reset when the voltage falls to and below Vdet1"]
    _1 = 1,
}
impl From<RI_A> for bool {
    #[inline(always)]
    fn from(variant: RI_A) -> Self {
        variant as u8 != 0
    }
}
impl RI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RI_A {
        match self.bits {
            false => RI_A::_0,
            true => RI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RI_A::_1
    }
}
#[doc = "Field `RI` writer - Voltage Monitor 1 Circuit Mode Select"]
pub type RI_W<'a, const O: u8> = crate::BitWriter<'a, u8, LVD1CR0_SPEC, RI_A, O>;
impl<'a, const O: u8> RI_W<'a, O> {
    #[doc = "Generate voltage monitor 1 interrupt on Vdet1 crossing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RI_A::_0)
    }
    #[doc = "Enable voltage monitor 1 reset when the voltage falls to and below Vdet1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RI_A::_1)
    }
}
#[doc = "Field `RN` reader - Voltage Monitor 1 Reset Negate Select"]
pub type RN_R = crate::BitReader<RN_A>;
#[doc = "Voltage Monitor 1 Reset Negate Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RN_A {
    #[doc = "0: Negate after a stabilization time (tLVD1) when VCC > Vdet1 is detected"]
    _0 = 0,
    #[doc = "1: Negate after a stabilization time (tLVD1) on assertion of the LVD1 reset"]
    _1 = 1,
}
impl From<RN_A> for bool {
    #[inline(always)]
    fn from(variant: RN_A) -> Self {
        variant as u8 != 0
    }
}
impl RN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RN_A {
        match self.bits {
            false => RN_A::_0,
            true => RN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RN_A::_1
    }
}
#[doc = "Field `RN` writer - Voltage Monitor 1 Reset Negate Select"]
pub type RN_W<'a, const O: u8> = crate::BitWriter<'a, u8, LVD1CR0_SPEC, RN_A, O>;
impl<'a, const O: u8> RN_W<'a, O> {
    #[doc = "Negate after a stabilization time (tLVD1) when VCC > Vdet1 is detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RN_A::_0)
    }
    #[doc = "Negate after a stabilization time (tLVD1) on assertion of the LVD1 reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Voltage Monitor 1 Interrupt/Reset Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage Monitor 1 Circuit Comparison Result Output Enable"]
    #[inline(always)]
    pub fn cmpe(&self) -> CMPE_R {
        CMPE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Voltage Monitor 1 Circuit Mode Select"]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Voltage Monitor 1 Reset Negate Select"]
    #[inline(always)]
    pub fn rn(&self) -> RN_R {
        RN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage Monitor 1 Interrupt/Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<0> {
        RIE_W::new(self)
    }
    #[doc = "Bit 2 - Voltage Monitor 1 Circuit Comparison Result Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpe(&mut self) -> CMPE_W<2> {
        CMPE_W::new(self)
    }
    #[doc = "Bit 6 - Voltage Monitor 1 Circuit Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn ri(&mut self) -> RI_W<6> {
        RI_W::new(self)
    }
    #[doc = "Bit 7 - Voltage Monitor 1 Reset Negate Select"]
    #[inline(always)]
    #[must_use]
    pub fn rn(&mut self) -> RN_W<7> {
        RN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Voltage Monitor 1 Circuit Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lvd1cr0](index.html) module"]
pub struct LVD1CR0_SPEC;
impl crate::RegisterSpec for LVD1CR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lvd1cr0::R](R) reader structure"]
impl crate::Readable for LVD1CR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lvd1cr0::W](W) writer structure"]
impl crate::Writable for LVD1CR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LVD1CR0 to value 0x80"]
impl crate::Resettable for LVD1CR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
