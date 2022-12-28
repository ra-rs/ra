#[doc = "Register `CCR0` reader"]
pub struct R(crate::R<CCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR0` writer"]
pub struct W(crate::W<CCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR0_SPEC>;
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
impl From<crate::W<CCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RE` reader - Receive Enable"]
pub type RE_R = crate::BitReader<RE_A>;
#[doc = "Receive Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RE_A {
    #[doc = "0: Serial reception is disabled"]
    _0 = 0,
    #[doc = "1: Serial reception is enabled"]
    _1 = 1,
}
impl From<RE_A> for bool {
    #[inline(always)]
    fn from(variant: RE_A) -> Self {
        variant as u8 != 0
    }
}
impl RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RE_A {
        match self.bits {
            false => RE_A::_0,
            true => RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RE_A::_1
    }
}
#[doc = "Field `RE` writer - Receive Enable"]
pub type RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR0_SPEC, RE_A, O>;
impl<'a, const O: u8> RE_W<'a, O> {
    #[doc = "Serial reception is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RE_A::_0)
    }
    #[doc = "Serial reception is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RE_A::_1)
    }
}
#[doc = "Field `TE` reader - Transmit Enable"]
pub type TE_R = crate::BitReader<TE_A>;
#[doc = "Transmit Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TE_A {
    #[doc = "0: Serial transmission is disabled"]
    _0 = 0,
    #[doc = "1: Serial transmission is enabled"]
    _1 = 1,
}
impl From<TE_A> for bool {
    #[inline(always)]
    fn from(variant: TE_A) -> Self {
        variant as u8 != 0
    }
}
impl TE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TE_A {
        match self.bits {
            false => TE_A::_0,
            true => TE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TE_A::_1
    }
}
#[doc = "Field `TE` writer - Transmit Enable"]
pub type TE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR0_SPEC, TE_A, O>;
impl<'a, const O: u8> TE_W<'a, O> {
    #[doc = "Serial transmission is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TE_A::_0)
    }
    #[doc = "Serial transmission is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TE_A::_1)
    }
}
#[doc = "Field `MPIE` reader - Multi-Processor Interrupt Enable"]
pub type MPIE_R = crate::BitReader<MPIE_A>;
#[doc = "Multi-Processor Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPIE_A {
    #[doc = "0: Non-Multi-Processor reception"]
    _0 = 0,
    #[doc = "1: Multi -Processor reception When the data with the multi-processor bit set to 0 is received, the data is not read, and setting the status flags to 1 is disabled. When the data with the multiprocessor bit set to 1 is received, the MPIE bit is automatically cleared to 0, and non-multi-processor reception is resumed. If you want to continue receiving operation using the multiprocessor function, set this bit to 1 sufficiently earlier than receiving the STOP bit of the next received frame. (Consider the synchronization delay time.)"]
    _1 = 1,
}
impl From<MPIE_A> for bool {
    #[inline(always)]
    fn from(variant: MPIE_A) -> Self {
        variant as u8 != 0
    }
}
impl MPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPIE_A {
        match self.bits {
            false => MPIE_A::_0,
            true => MPIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPIE_A::_1
    }
}
#[doc = "Field `MPIE` writer - Multi-Processor Interrupt Enable"]
pub type MPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR0_SPEC, MPIE_A, O>;
impl<'a, const O: u8> MPIE_W<'a, O> {
    #[doc = "Non-Multi-Processor reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPIE_A::_0)
    }
    #[doc = "Multi -Processor reception When the data with the multi-processor bit set to 0 is received, the data is not read, and setting the status flags to 1 is disabled. When the data with the multiprocessor bit set to 1 is received, the MPIE bit is automatically cleared to 0, and non-multi-processor reception is resumed. If you want to continue receiving operation using the multiprocessor function, set this bit to 1 sufficiently earlier than receiving the STOP bit of the next received frame. (Consider the synchronization delay time.)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPIE_A::_1)
    }
}
#[doc = "Field `DCME` reader - Data Compare Match Enable"]
pub type DCME_R = crate::BitReader<DCME_A>;
#[doc = "Data Compare Match Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCME_A {
    #[doc = "0: Address match function is disabled"]
    _0 = 0,
    #[doc = "1: Address match function is enabled"]
    _1 = 1,
}
impl From<DCME_A> for bool {
    #[inline(always)]
    fn from(variant: DCME_A) -> Self {
        variant as u8 != 0
    }
}
impl DCME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCME_A {
        match self.bits {
            false => DCME_A::_0,
            true => DCME_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCME_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCME_A::_1
    }
}
#[doc = "Field `DCME` writer - Data Compare Match Enable"]
pub type DCME_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR0_SPEC, DCME_A, O>;
impl<'a, const O: u8> DCME_W<'a, O> {
    #[doc = "Address match function is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCME_A::_0)
    }
    #[doc = "Address match function is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCME_A::_1)
    }
}
#[doc = "Field `IDSEL` reader - ID frame select"]
pub type IDSEL_R = crate::BitReader<IDSEL_A>;
#[doc = "ID frame select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDSEL_A {
    #[doc = "0: It's always compared data in spite of the value of the MPB bit."]
    _0 = 0,
    #[doc = "1: It's compared data when the MPB bit is 1 (ID frame) only."]
    _1 = 1,
}
impl From<IDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: IDSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl IDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDSEL_A {
        match self.bits {
            false => IDSEL_A::_0,
            true => IDSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDSEL_A::_1
    }
}
#[doc = "Field `IDSEL` writer - ID frame select"]
pub type IDSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR0_SPEC, IDSEL_A, O>;
impl<'a, const O: u8> IDSEL_W<'a, O> {
    #[doc = "It's always compared data in spite of the value of the MPB bit."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IDSEL_A::_0)
    }
    #[doc = "It's compared data when the MPB bit is 1 (ID frame) only."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IDSEL_A::_1)
    }
}
#[doc = "Field `RIE` reader - Receive Interrupt Enable"]
pub type RIE_R = crate::BitReader<RIE_A>;
#[doc = "Receive Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RIE_A {
    #[doc = "0: SCIn_RXI and SCIn_ERI interrupt requests are disabled"]
    _0 = 0,
    #[doc = "1: SCIn_RXI and SCIn_ERI interrupt requests are enabled"]
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
#[doc = "Field `RIE` writer - Receive Interrupt Enable"]
pub type RIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR0_SPEC, RIE_A, O>;
impl<'a, const O: u8> RIE_W<'a, O> {
    #[doc = "SCIn_RXI and SCIn_ERI interrupt requests are disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RIE_A::_0)
    }
    #[doc = "SCIn_RXI and SCIn_ERI interrupt requests are enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RIE_A::_1)
    }
}
#[doc = "Field `TIE` reader - Transmit Interrupt Enable"]
pub type TIE_R = crate::BitReader<TIE_A>;
#[doc = "Transmit Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIE_A {
    #[doc = "0: SCIn_TXI interrupt request is disabled"]
    _0 = 0,
    #[doc = "1: SCIn_TXI interrupt request is enabled"]
    _1 = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::_0,
            true => TIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIE_A::_1
    }
}
#[doc = "Field `TIE` writer - Transmit Interrupt Enable"]
pub type TIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR0_SPEC, TIE_A, O>;
impl<'a, const O: u8> TIE_W<'a, O> {
    #[doc = "SCIn_TXI interrupt request is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIE_A::_0)
    }
    #[doc = "SCIn_TXI interrupt request is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIE_A::_1)
    }
}
#[doc = "Field `TEIE` reader - Transmit End Interrupt Enable"]
pub type TEIE_R = crate::BitReader<TEIE_A>;
#[doc = "Transmit End Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIE_A {
    #[doc = "0: SCIn_TEI interrupt request is disabled"]
    _0 = 0,
    #[doc = "1: SCIn_TEI interrupt request is enabled"]
    _1 = 1,
}
impl From<TEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIE_A {
        match self.bits {
            false => TEIE_A::_0,
            true => TEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEIE_A::_1
    }
}
#[doc = "Field `TEIE` writer - Transmit End Interrupt Enable"]
pub type TEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR0_SPEC, TEIE_A, O>;
impl<'a, const O: u8> TEIE_W<'a, O> {
    #[doc = "SCIn_TEI interrupt request is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEIE_A::_0)
    }
    #[doc = "SCIn_TEI interrupt request is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEIE_A::_1)
    }
}
#[doc = "Field `SSE` reader - SSn Pin Function Enable"]
pub type SSE_R = crate::BitReader<SSE_A>;
#[doc = "SSn Pin Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSE_A {
    #[doc = "0: SSn pin function is disabled."]
    _0 = 0,
    #[doc = "1: SSn pin function is enabled."]
    _1 = 1,
}
impl From<SSE_A> for bool {
    #[inline(always)]
    fn from(variant: SSE_A) -> Self {
        variant as u8 != 0
    }
}
impl SSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSE_A {
        match self.bits {
            false => SSE_A::_0,
            true => SSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSE_A::_1
    }
}
#[doc = "Field `SSE` writer - SSn Pin Function Enable"]
pub type SSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR0_SPEC, SSE_A, O>;
impl<'a, const O: u8> SSE_W<'a, O> {
    #[doc = "SSn pin function is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSE_A::_0)
    }
    #[doc = "SSn pin function is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Receive Enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Multi-Processor Interrupt Enable"]
    #[inline(always)]
    pub fn mpie(&self) -> MPIE_R {
        MPIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data Compare Match Enable"]
    #[inline(always)]
    pub fn dcme(&self) -> DCME_R {
        DCME_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ID frame select"]
    #[inline(always)]
    pub fn idsel(&self) -> IDSEL_R {
        IDSEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit End Interrupt Enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - SSn Pin Function Enable"]
    #[inline(always)]
    pub fn sse(&self) -> SSE_R {
        SSE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Enable"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<0> {
        RE_W::new(self)
    }
    #[doc = "Bit 4 - Transmit Enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<4> {
        TE_W::new(self)
    }
    #[doc = "Bit 8 - Multi-Processor Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mpie(&mut self) -> MPIE_W<8> {
        MPIE_W::new(self)
    }
    #[doc = "Bit 9 - Data Compare Match Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcme(&mut self) -> DCME_W<9> {
        DCME_W::new(self)
    }
    #[doc = "Bit 10 - ID frame select"]
    #[inline(always)]
    #[must_use]
    pub fn idsel(&mut self) -> IDSEL_W<10> {
        IDSEL_W::new(self)
    }
    #[doc = "Bit 16 - Receive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<16> {
        RIE_W::new(self)
    }
    #[doc = "Bit 20 - Transmit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<20> {
        TIE_W::new(self)
    }
    #[doc = "Bit 21 - Transmit End Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<21> {
        TEIE_W::new(self)
    }
    #[doc = "Bit 24 - SSn Pin Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sse(&mut self) -> SSE_W<24> {
        SSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr0](index.html) module"]
pub struct CCR0_SPEC;
impl crate::RegisterSpec for CCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr0::R](R) reader structure"]
impl crate::Readable for CCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr0::W](W) writer structure"]
impl crate::Writable for CCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR0 to value 0"]
impl crate::Resettable for CCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
